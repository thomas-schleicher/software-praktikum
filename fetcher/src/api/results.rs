use core::f32;

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
    pub src_addr: Option<String>,
    pub rt: Option<f32>,
    pub res: Option<u32>,
    pub ver: Option<String>,
    pub hsize: Option<u32>,
    pub bsize: Option<u32>,
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
            src_addr: http_result
                .src_addr
                .clone()
                .unwrap_or_else(|| "timeout".to_string()),
            rt: http_result.rt.unwrap_or_else(|| 0.0),
            res: http_result.res.unwrap_or_else(|| 408),
            ver: http_result
                .ver
                .clone()
                .unwrap_or_else(|| "timeout".to_string()),
            hsize: http_result.hsize.unwrap_or_else(|| 0),
            bsize: http_result.bsize.unwrap_or_else(|| 0),
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
    pub prb_id: u32,
    pub msm_id: u32,
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
    pub msm_id: u32,
    pub src_addr: String,
    pub dst_addr: String,
    pub hop: u32,
    pub sent: u32,
    pub received: u32,
    pub min: f32,
    pub avg: f32,
    pub max: f32,
    pub from: String,
    pub timed_out: bool,
    pub timestamp: usize,
    pub prb_id: u32,
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
            .map(|hop_result| {
                let rtts: Vec<f32> = hop_result.result.iter().filter_map(|hr| hr.rtt).collect();

                let sent = hop_result.result.len() as u32;
                let received = rtts.len() as u32;
                let timed_out = received == 0;

                let (min, max, avg) = if received > 0 {
                    let min = rtts.iter().cloned().fold(f32::INFINITY, f32::min);
                    let max = rtts.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                    let avg = rtts.iter().sum::<f32>() / received as f32;
                    (min, max, avg)
                } else {
                    (0.0, 0.0, 0.0)
                };

                let from = hop_result
                        .result
                        .iter()
                        .find_map(|hr| hr.from.clone())
                        .unwrap_or_else(|| "unknown".to_string());

                FlattenedTraceRouteMeasurement {
                    msm_id: traceroute_measurement.msm_id,
                    src_addr: traceroute_measurement.src_addr.clone(),
                    dst_addr: traceroute_measurement.dst_addr.clone(),
                    hop: hop_result.hop,
                    sent,
                    received,
                    min,
                    avg,
                    max,
                    from,
                    timed_out,
                    timestamp: traceroute_measurement.timestamp,
                    prb_id: traceroute_measurement.prb_id,
                    endtime: traceroute_measurement.endtime,
                    proto: traceroute_measurement.proto.clone(),
                    paris_id: traceroute_measurement.paris_id,
                    destination_ip_responded: traceroute_measurement.destination_ip_responded,
                }
            })
            .collect()
    }
}
