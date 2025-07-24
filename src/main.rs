use link_shortener::{
    adapters::{axum::build_app, generator::RandomGenerator, in_memory::InMemoryRepository},
    application::link_shortener::LinkShortener,
};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use tracing::{debug, info};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Creating database...");
    let database = InMemoryRepository::default();
    info!("Creating generator...");
    let generator = RandomGenerator::new_with(ChaChaRng::from_os_rng(), 10);

    info!("Setting up service...");
    let service = LinkShortener::new(database, generator);
    let app = build_app(service);

    info!("Application running at http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TcpListener.");
    debug!("test");
    axum::serve(listener, app)
        .await
        .expect("Failed to serve app.");
}
