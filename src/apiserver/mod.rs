use std::{collections::HashMap, sync::Arc};

use actix_web::{App, HttpResponse, HttpServer, web};
use log::{debug, info};
use p256::elliptic_curve::rand_core::block;
use serde::de;
use crate::{blockchain::BlockChain, wallet::Wallet};
use serde_json;

#[derive(Clone, Debug)]
pub struct ApiServer {
    port: u16,
    cache: HashMap<String, BlockChain>, // wallet_address -> blockchain
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        let wallet_miner = Wallet::new();
        let miner_address = wallet_miner.get_adress();

        // Initialize blockchain for this miner
        let blockchain = BlockChain::new(miner_address.clone());

        // Initialize cache
        let mut cache = HashMap::new();
        cache.insert(miner_address, blockchain);

       let mut api_server =  ApiServer {
            port,
            cache,
        };

        return  api_server;
    }

    async  fn get_wallet() -> HttpResponse {
        HttpResponse::Ok().content_type("text/html").
    }

    // Instance method to get blockchain info
    async fn get_index(&self) -> HttpResponse {
         let blockchain = self.cache.get("blockchain").unwrap();
         let first_block = blockchain[0].clone();
         let block_json = serde_json::to_string(&first_block).unwrap();
         debug!("block json {:?}",block_json);
        return   HttpResponse::Ok().json(block_json);


    
    }

    // Handler for Actix
    pub async fn get_index_handler(data: web::Data<Arc<ApiServer>>) -> HttpResponse {
        info!("Receiving request at '/' endpoint");
        debug!("Handler received ApiServer data: {:?}", data);
        data.get_ref().get_index().await
    }

    pub async fn run(&self) {
        let api = Arc::new(self.clone());
        let port = self.port;

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(api.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .route("/", web::get().to(ApiServer::get_index_handler)) // remove ()
        });

        println!("Server is running on port: {}", port);

        server
            .bind(format!("0.0.0.0:{}", port))
            .expect("Failed to bind server")
            .run()
            .await
            .expect("Error running the server");
    }
}