use common::setup;
use simpledns::upstream;
mod common;

#[test]
fn test_resolve() {
    let domain = "example.com";
    let app = setup();

    for upstream in app.upstreams {
        if let Some(upstream) = upstream {
            let response = upstream.resolve(domain);

            for result in response {
                assert!(result.is_ipv4() || result.is_ipv6());
            }
        }
    }
}

#[test]
fn test_resolve_bulk() {
    let domain = "example.com";
    let app = setup();

    let responses = upstream::resolve_bulk(domain, &app.upstreams);

    assert!(responses.iter().all(|response| {
        if let Some(response) = response {
            response
                .records
                .iter()
                .all(|record| record.is_ipv4() || record.is_ipv6())
        } else {
            false
        }
    }));
}
