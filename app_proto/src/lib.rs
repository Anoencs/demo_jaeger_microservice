use opentelemetry::{
    global,
    propagation::{Extractor, Injector},
    sdk::propagation::TraceContextPropagator,
};

use tonic::Request;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::prelude::*;
#[cfg(feature = "add")]
pub mod add;

#[cfg(feature = "mul")]
pub mod mul;

#[cfg(feature = "maths")]
pub mod maths;

pub fn otel_jaeger_init(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .install_batch(opentelemetry::runtime::Tokio)?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    Ok(())
}

//inject trace context to req and return new request
pub fn trace_req<T>(req: T) -> Request<T> {
    let mut req = Request::new(req);

    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(
            &tracing::Span::current().context(),
            &mut MutMetadataMap(req.metadata_mut()),
        )
    });

    req
}

pub fn set_trace_ctx<T>(req: &Request<T>) {
    let parent_cx =
        global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(req.metadata())));
    tracing::Span::current().set_parent(parent_cx);
}

struct MetadataMap<'a>(pub &'a tonic::metadata::MetadataMap);
struct MutMetadataMap<'a>(pub &'a mut tonic::metadata::MetadataMap);

impl<'a> Injector for MutMetadataMap<'a> {
    /// Set a key and value in the MetadataMap.  Does nothing if the key or value are not valid inputs
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(val) = tonic::metadata::MetadataValue::from_str(&value) {
                self.0.insert(key, val);
            }
        }
    }
}

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}
