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

```python
# HELP benchem_prom_timeseries Number of timeseries return by prometheus.
# TYPE benchem_prom_timeseries gauge
benchem_prom_timeseries{query="count_up"} 1
benchem_prom_timeseries{query="example"} 7
benchem_prom_timeseries{query="memory_usage"} 33
benchem_prom_timeseries{query="min_over_time__up"} 1
benchem_prom_timeseries{query="unavailable_apiservice"} 1
# HELP benchem_prometheus_request_duration_seconds The prometheus request latencies in seconds.
# TYPE benchem_prometheus_request_duration_seconds histogram
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.05"} 3
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.1"} 3
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.25"} 4
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="0.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="1"} 4
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="2.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="10"} 4
benchem_prometheus_request_duration_seconds_bucket{query="count_up",le="+Inf"} 4
benchem_prometheus_request_duration_seconds_sum{query="count_up"} 0.22450714
benchem_prometheus_request_duration_seconds_count{query="count_up"} 4
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.05"} 0
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.1"} 1
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.25"} 4
benchem_prometheus_request_duration_seconds_bucket{query="example",le="0.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="example",le="1"} 4
benchem_prometheus_request_duration_seconds_bucket{query="example",le="2.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="example",le="5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="example",le="10"} 4
benchem_prometheus_request_duration_seconds_bucket{query="example",le="+Inf"} 4
benchem_prometheus_request_duration_seconds_sum{query="example"} 0.5261183490000001
benchem_prometheus_request_duration_seconds_count{query="example"} 4
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.05"} 0
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.1"} 2
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.25"} 4
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="0.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="1"} 4
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="2.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="10"} 4
benchem_prometheus_request_duration_seconds_bucket{query="memory_usage",le="+Inf"} 4
benchem_prometheus_request_duration_seconds_sum{query="memory_usage"} 0.465249465
benchem_prometheus_request_duration_seconds_count{query="memory_usage"} 4
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.05"} 3
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.1"} 4
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.25"} 4
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="0.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="1"} 4
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="2.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="10"} 4
benchem_prometheus_request_duration_seconds_bucket{query="min_over_time__up",le="+Inf"} 4
benchem_prometheus_request_duration_seconds_sum{query="min_over_time__up"} 0.16513146899999998
benchem_prometheus_request_duration_seconds_count{query="min_over_time__up"} 4
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.005"} 0
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.01"} 0
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.025"} 0
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.05"} 2
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.1"} 4
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.25"} 4
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="0.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="1"} 4
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="2.5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="5"} 4
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="10"} 4
benchem_prometheus_request_duration_seconds_bucket{query="unavailable_apiservice",le="+Inf"} 4
benchem_prometheus_request_duration_seconds_sum{query="unavailable_apiservice"} 0.203665646
benchem_prometheus_request_duration_seconds_count{query="unavailable_apiservice"} 4
# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 0
# HELP process_max_fds Maximum number of open file descriptors.
# TYPE process_max_fds gauge
process_max_fds 1024
# HELP process_open_fds Number of open file descriptors.
# TYPE process_open_fds gauge
process_open_fds 17
# HELP process_resident_memory_bytes Resident memory size in bytes.
# TYPE process_resident_memory_bytes gauge
process_resident_memory_bytes 23588864
# HELP process_start_time_seconds Start time of the process since unix epoch in seconds.
# TYPE process_start_time_seconds gauge
process_start_time_seconds 1690617314
# HELP process_threads Number of OS threads in the process.
# TYPE process_threads gauge
process_threads 6
# HELP process_virtual_memory_bytes Virtual memory size in bytes.
# TYPE process_virtual_memory_bytes gauge
process_virtual_memory_bytes 371343360
```


## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on GitHub. If you want to contribute code, please fork the repository and submit a pull request.

Use [lefhook](https://github.com/evilmartians/lefthook) to lint pre-commit. After installation it can be enabled with `lefthook install -f`.

## License

`bench-metrics` is licensed under the MIT License. See the LICENSE file for details.
