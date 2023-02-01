use simpledns::{cli, upstream, App};

fn main() {
    let matches = cli::init().get_matches();
    let app = App::new(None);

    if let Some(domain) = matches.get_one::<String>("domain") {
        let responses = upstream::resolve_bulk(domain, &app.upstreams);

        responses.iter().for_each(|response| {
            if let Some(response) = response {
                println!(
                    "upstream: {}, records: {:?}, took: {}ms",
                    response.upstream_addr,
                    response.records,
                    response.elapsed.as_millis()
                );
            }
        });
    }
}
