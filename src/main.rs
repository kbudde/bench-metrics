use prometheus::register_counter_vec;
use prometheus::CounterVec;
use prometheus_http_query::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
use std::sync::Arc;
use structured_logger::{json::new_writer, Builder};

use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use prometheus::{Encoder, HistogramVec, TextEncoder};

use lazy_static::lazy_static;
use prometheus::register_histogram_vec;

lazy_static! {
    static ref PROM_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "benchem_prometheus_request_duration_seconds",
        "The prometheus request latencies in seconds.",
        &["query"]
    )
    .unwrap();
    static ref PROM_FAILED_COUNTER: CounterVec = register_counter_vec!(
        "benchem_prom_requests_failed_total",
        "Number of prometheus requests made.",
        &["query"]
    )
    .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();

    let config = match envy::prefixed("BENCH_").from_env::<Config>() {
        Ok(cfg) => Ok(cfg),
        Err(error) => {
            log::info!(error= error.to_string();"Failed to read configuration from environment. Failover to 'config.yaml'");
            Config::from_yaml("config.yaml")
        }
    }?;
    log::debug!(config=log::as_serde!(config); "Reading config was sucessfull");

    let addr = ([0, 0, 0, 0], 9898).into();

    let client = Arc::new(PrometheusClient::from_config(&config)?);

    let metrics = make_service_fn(move |_| {
        let client = client.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let client = client.clone();
                handler(req, client)
            }))
        }
    });

    let server = Server::bind(&addr).serve(metrics);
    log::info!(addr=format!("http://{}", addr);"Started and listening");
    server.await?;
    Ok(())
}

fn init_logger() {
    Builder::new()
        .with_target_writer("*", new_writer(stdout()))
        .init();
}

// --- config
// https://github.com/softprops/envy/issues/56
serde_with::with_prefix!(queries "queries_");

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    url: String,
    // PREFIX_QUERIES_X=y will map to Hashmap {"x": "y"}
    #[serde(flatten, with = "queries")]
    queries: HashMap<String, String>,
}

impl Config {
    fn from_yaml(file: &str) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}

// --- prometheus ---
struct PrometheusClient {
    client: Client,
    queries: HashMap<String, String>,
}

impl Clone for PrometheusClient {
    fn clone(&self) -> Self {
        PrometheusClient {
            client: self.client.clone(),
            queries: self.queries.clone(),
        }
    }
}

impl PrometheusClient {
    fn from_config(config: &Config) -> Result<PrometheusClient, Box<dyn Error>> {
        let c = reqwest::Client::builder().no_proxy().build()?;
        let c = Client::from(c, &config.url)?;

        let pc = PrometheusClient {
            client: c,
            queries: config.queries.to_owned(),
        };
        Ok(pc)
    }
}

async fn handler(
    _req: Request<Body>,
    pc: Arc<PrometheusClient>,
) -> Result<Response<Body>, hyper::Error> {
    let client = &pc.client;
    let encoder = TextEncoder::new();

    for (name, query) in pc.queries.clone() {
        let timer = PROM_REQ_HISTOGRAM.with_label_values(&[&name]).start_timer();
        let result = client.query(&query).get().await;
        if let Err(error)=result{
                log::warn!("running up query {}", error);
                PROM_FAILED_COUNTER.with_label_values(&[&query]).inc();
        }
        timer.observe_duration();
    }

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialization() {
        let config = Config::from_yaml("config.yaml").unwrap();

        // Assert the values in the config struct
        assert_eq!(config.url, "http://localhost:10902/");
        assert_eq!(config.queries.len(), 5);
    }

    #[test]
    fn test_prometheus_client_creation() {
        let config = Config {
            url: "http://localhost:10902/".to_string(),
            queries: HashMap::new(),
        };

        let client = PrometheusClient::from_config(&config).unwrap();

        // Assert the values in the client struct
        assert_eq!(client.queries.len(), 0);
    }
}
