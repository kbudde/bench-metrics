#!/bin/env bash

docker network create local

docker run -d -p 9090:9090 --network local --name prom prom/prometheus

docker run -d -p 9898:9898 --network local --name benchem \
  -e BENCH_URL="http://prom:9090" \
  -e BENCH_QUERIES_up="count(up)" \
  kbudde/bench-metrics:latest


while ! curl --connect-timeout 2 -s localhost:9090 2>/dev/null ; do
  echo wait for prometheus... 
  sleep 1
done  

sleep 5
curl http://localhost:9898 > metrics.txt

cat metrics.txt

docker rm -f prom
docker rm -f benchem
docker network rm local

grep 'benchem_prom_timeseries{query="up"} 1' metrics.txt || exit 1
grep 'benchem_prometheus_request_duration_seconds_bucket{query="up",le="5"}' metrics.txt || exit 2
grep 'process_cpu_seconds_total' metrics.txt || exit 3

