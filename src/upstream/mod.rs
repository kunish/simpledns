use dnsclient::sync::DNSClient;
use rayon::prelude::*;
use std::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::{Duration, SystemTime};

pub struct Upstream {
    pub resolver: DNSClient,
}

impl Debug for Upstream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Upstream")
            .field("resolver", &format_args!("resolver"))
            .finish()
    }
}

impl Upstream {
    pub fn new(addr: &str) -> Self {
        Self {
            resolver: DNSClient::new(vec![dnsclient::UpstreamServer::new(
                addr.parse::<SocketAddr>().unwrap(),
            )]),
        }
    }

    pub fn resolve(&self, domain: &str) -> Vec<Ipv4Addr> {
        self.resolver.query_a(domain).unwrap()
    }
}

pub struct BulkResolveResult {
    pub records: Vec<Ipv4Addr>,
    pub elapsed: Duration,
}

pub fn resolve_bulk(domain: &str, upstreams: &Vec<Upstream>) -> Vec<BulkResolveResult> {
    upstreams
        .par_iter()
        .map(|upstream| {
            let start = SystemTime::now();
            let records = upstream.resolve(domain);
            let end = SystemTime::now();
            let elapsed = end.duration_since(start).unwrap_or_default();
            BulkResolveResult { records, elapsed }
        })
        .collect()
}
