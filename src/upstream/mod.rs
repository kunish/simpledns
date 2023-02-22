use rayon::prelude::*;
use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, SystemTime};
use trust_dns_resolver::{
    config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts},
    Resolver,
};

pub struct Upstream {
    pub addr: SocketAddr,
    pub resolver: Resolver,
}

impl Debug for Upstream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Upstream")
            .field("addr", &self.addr)
            .field("resolver", &format_args!("resolver"))
            .finish()
    }
}

impl Upstream {
    pub fn new(addr: &str) -> Option<Self> {
        let mut resolver_config = ResolverConfig::new();
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.cache_size = 0;

        if let Some(addr) = addr.parse::<SocketAddr>().ok() {
            resolver_config.add_name_server(NameServerConfig::new(addr, Protocol::Udp));

            let resolver = Resolver::new(resolver_config, resolver_opts).ok();

            if let Some(resolver) = resolver {
                return Some(Self { addr, resolver });
            }
        }

        None
    }

    pub fn resolve(&self, domain: &str) -> Vec<IpAddr> {
        let mut records = vec![];
        let response = self.resolver.lookup_ip(domain).ok();

        if let Some(lookup) = response {
            lookup.iter().for_each(|record| {
                records.push(record);
            });
        }

        records
    }
}

pub struct BulkResolveResult {
    pub upstream_addr: SocketAddr,
    pub records: Vec<IpAddr>,
    pub elapsed: Duration,
}

pub fn resolve_bulk(
    domain: &str,
    upstreams: &Vec<Option<Upstream>>,
) -> Vec<Option<BulkResolveResult>> {
    upstreams
        .par_iter()
        .map(|upstream| {
            if let Some(upstream) = upstream {
                let start = SystemTime::now();
                let records = upstream.resolve(domain);
                let end = SystemTime::now();
                let elapsed = end.duration_since(start).unwrap_or_default();

                Some(BulkResolveResult {
                    upstream_addr: upstream.addr,
                    records,
                    elapsed,
                })
            } else {
                None
            }
        })
        .collect()
}
