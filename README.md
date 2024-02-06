## Run
# Run Jaeger 
``` Dockerfile
docker run -d --name jaeger -e COLLECTOR_JAEGER_HTTP_PORT=14268   -p 5775:5775/udp   -p 6831:6831/udp  -p 6832:6832/udp   -p 5778:5778  -p 16686:16686  -p 14268:14268 -p 9411:9411 jaegertracing/all-in-one:1.6\n
```
# Run test
cargo run --bin add-service
cargo run --bin mul-service

cargo run --bin app_cli add 3 5
cargo run --bin app_cli mul 3 5

## Notice
You have your tracing UI in port 16686 
