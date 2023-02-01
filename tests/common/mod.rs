use simpledns::App;

pub fn setup() -> App {
    let app = App::new(Some("docs/config.yaml"));

    app
}
