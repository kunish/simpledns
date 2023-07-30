use common::setup;
use simpledns::upstream;
mod common;

#[test]
fn test_resolve() {
    let domain = "example.com";
    let app = setup();

    for upstream in app.upstreams {
        let response = upstream.resolve(domain);

        for result in response {
            assert!(!result.is_private());
        }
    }
}

#[test]
fn test_resolve_bulk() {
    let domain = "example.com";
    let app = setup();

    let responses = upstream::resolve_bulk(domain, &app.upstreams);

    assert!(responses
        .iter()
        .all(|response| { response.records.iter().all(|record| !record.is_private()) }));
}
