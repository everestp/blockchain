pub mod apiserver;
pub mod  blockchain;
pub mod  wallet;
use crate::apiserver::ApiServer;

#[actix_web::main]
async fn main() {
    env_logger::init();
    let server = ApiServer::new(5000);
    server.run().await; 
}