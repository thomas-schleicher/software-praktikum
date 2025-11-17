use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AggregatedMeasurement {
    #[serde(rename = "http")]
    Http(HttpMeasurement),
    #[serde(rename = "ping")]
    Ping(PingMeasurement),
    #[serde(rename = "traceroute")]
    TraceRoute(TraceRouteMeasurement),
}

impl AggregatedMeasurement {
    pub fn kind(&self) -> &str {
        match self {
            AggregatedMeasurement::Http(_) => "http",
            AggregatedMeasurement::Ping(_) => "ping",
            AggregatedMeasurement::TraceRoute(_) => "traceroute",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingMeasurement {
    pub dst_addr: String,
    pub src_addr: String,
    pub proto: String,
    pub rcvd: u32,
    pub sent: u32,
    pub min: f32,
    pub max: f32,
    pub avg: f32,
    pub msm_id: u32,
    pub timestamp: usize,
    pub prb_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpMeasurement {
    pub ttr: f32,
    pub result: Vec<HttpResult>,
    pub msm_id: u32,
    pub timestamp: usize,
    pub prb_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResult {
    pub method: String,
    pub dst_addr: String,
    pub src_addr: String,
    pub rt: f32,
    pub res: u32,
    pub ver: String,
    pub hsize: u32,
    pub bsize: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlattenedHttpMeasurement {
    pub ttr: f32,
    pub method: String,
    pub dst_addr: String,
    pub src_addr: String,
    pub rt: f32,
    pub res: u32,
    pub ver: String,
    pub hsize: u32,
    pub bsize: u32,
    pub msm_id: u32,
    pub timestamp: usize,
    pub prb_id: u32,
}

impl FlattenedHttpMeasurement {
    pub fn from_http_measurement(measurement: &HttpMeasurement) -> Self {
        let http_result = measurement
            .result
            .first()
            .expect("Http measurement should contain exactly one result, first() is none");
        Self {
            ttr: measurement.ttr,
            method: http_result.method.clone(),
            dst_addr: http_result.dst_addr.clone(),
            src_addr: http_result.src_addr.clone(),
            rt: http_result.rt,
            res: http_result.res,
            ver: http_result.ver.clone(),
            hsize: http_result.hsize,
            bsize: http_result.bsize,
            msm_id: measurement.msm_id,
            timestamp: measurement.timestamp,
            prb_id: measurement.prb_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceRouteMeasurement {
    pub endtime: usize,
    pub dst_addr: String,
    pub src_addr: String,
    pub proto: String,
    pub paris_id: u32,
    pub result: Vec<TraceRouteResult>,
    pub destination_ip_responded: bool,
    pub probe_id: String,
    pub msm_id: String,
    pub timestamp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceRouteResult {
    pub hop: u32,
    pub result: Vec<HopResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HopResult {
    pub from: Option<String>,
    pub ttl: Option<u32>,
    pub size: Option<u32>,
    pub rtt: Option<f32>,
    pub x: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlattenedTraceRouteMeasurement {
    pub msm_id: String,
    pub hop: u32,
    pub sent: u32,
    pub recived: u32,
    pub min: f32,
    pub avg: f32,
    pub max: f32,
    pub from: String,
    pub timed_out: bool,
    pub timestamp: usize,
    pub probe_id: String,
    pub endtime: usize,
    pub proto: String,
    pub paris_id: u32,
    pub destination_ip_responded: bool,
}

impl FlattenedTraceRouteMeasurement {
    pub fn from_traceroute_measurement(
        traceroute_measurement: &TraceRouteMeasurement,
    ) -> Vec<Self> {
        traceroute_measurement
            .result
            .iter()
            .map(|result| FlattenedTraceRouteMeasurement {
                msm_id: todo!(),
                hop: todo!(),
                sent: todo!(),
                recived: todo!(),
                min: todo!(),
                avg: todo!(),
                max: todo!(),
                from: todo!(),
                timed_out: todo!(),
                timestamp: todo!(),
                probe_id: todo!(),
                endtime: todo!(),
                proto: todo!(),
                paris_id: todo!(),
                destination_ip_responded: todo!(),
            })
            .collect()
    }
}
