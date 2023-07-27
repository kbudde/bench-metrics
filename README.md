[![build](https://github.com/kbudde/bench-metrics/actions/workflows/build.yml/badge.svg)](https://github.com/kbudde/bench-metrics/actions/workflows/build.yml) 

# bench-metrics

`bench-metrics` is a Prometheus exporter that exports histograms for the duration of queries send to a Prometheus-compatible API (Thanos, VictoriaMetrics, ...).

## Installation

To install `bench-metrics` on your Kubernetes cluster, apply the Kubernetes manifests located in the `manifests` folder:

```
kubectl apply -f manifests/
```

Alternatively, the application can be compiled with `cargo build`.
Configuration is done using environment variables (envrc.example) or `config.yaml`.

Start with:

```
./bench-metrics
```

The application is listening on port 9898.

## Supported APIs

`bench-metrics` supports the following Prometheus-compatible APIs. Tested with:

- Thanos
- VictoriaMetrics

## example

```
# HELP benchem_prometheus_request_duration_seconds The prometheus request latencies in seconds.
# TYPE benchem_prometheus_request_duration_seconds histogram
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.05"} 1
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.1"} 2
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.25"} 2
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="1"} 2
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="2.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="10"} 2
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="+Inf"} 2
benchem_prometheus_request_duration_seconds_sum{query="count_up"} 0.094481327
benchem_prometheus_request_duration_seconds_count{query="count_up"} 2
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.05"} 0
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.1"} 1
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.25"} 2
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="example",le="1"} 2
benchem_prometheus_request_duration_seconds_bucket{query="example",le="2.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="example",le="5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="example",le="10"} 2
benchem_prometheus_request_duration_seconds_bucket{query="example",le="+Inf"} 2
benchem_prometheus_request_duration_seconds_sum{query="example"} 0.157936939
benchem_prometheus_request_duration_seconds_count{query="example"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.05"} 0
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.1"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.25"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="1"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="2.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="10"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="+Inf"} 2
benchem_prometheus_request_duration_seconds_sum{query="memory_usage"} 0.12306187700000001
benchem_prometheus_request_duration_seconds_count{query="memory_usage"} 2
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.05"} 0
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.1"} 0
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.25"} 1
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.5"} 1
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="1"} 1
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="2.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="10"} 2
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="+Inf"} 2
benchem_prometheus_request_duration_seconds_sum{query="min_over_time__up"} 2.348106937
benchem_prometheus_request_duration_seconds_count{query="min_over_time__up"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.05"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.1"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.25"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="1"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="2.5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="5"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="10"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="+Inf"} 2
benchem_prometheus_request_duration_seconds_sum{query="unavailable_apiservice"} 0.08635997100000001
benchem_prometheus_request_duration_seconds_count{query="unavailable_apiservice"} 2
# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 0
# HELP process_max_fds Maximum number of open file descriptors.
# TYPE process_max_fds gauge
process_max_fds 1024
# HELP process_open_fds Number of open file descriptors.
# TYPE process_open_fds gauge
process_open_fds 16
# HELP process_resident_memory_bytes Resident memory size in bytes.
# TYPE process_resident_memory_bytes gauge
process_resident_memory_bytes 16941056
# HELP process_start_time_seconds Start time of the process since unix epoch in seconds.
# TYPE process_start_time_seconds gauge
process_start_time_seconds 1690483337
# HELP process_threads Number of OS threads in the process.
# TYPE process_threads gauge
process_threads 5
# HELP process_virtual_memory_bytes Virtual memory size in bytes.
# TYPE process_virtual_memory_bytes gauge
process_virtual_memory_bytes 369057792
```


## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on GitHub. If you want to contribute code, please fork the repository and submit a pull request.

## License

`bench-metrics` is licensed under the MIT License. See the LICENSE file for details.
