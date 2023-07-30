use simpledns::{cli, upstream, App};

fn main() {
    let matches = cli::init().get_matches();
    let app = App::new(Some("docs/config.yaml"));

    if let Some(domain) = matches.get_one::<String>("domain") {
        let responses = upstream::resolve_bulk(domain, &app.upstreams);

        responses.iter().for_each(|response| {
            println!(
                "records: {:?}, took: {}ms",
                response.records,
                response.elapsed.as_millis()
            );
        });
    }
}
