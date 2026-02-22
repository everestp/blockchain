use std::{collections::HashMap, sync::Arc};

use actix_web::{App, HttpResponse, HttpServer, web};
use log::{debug, info};
use p256::elliptic_curve::rand_core::block;
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

   async fn get_wallet() -> HttpResponse {
        HttpResponse::Ok().content_type("text/html").body(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
               <meta charset="UTF-8"/>
               <title>Wallet</title>
               <script src="https://ajax.googleapis.com/ajax/libs/jquery/3.7.1/jquery.min.js"></script>
               <script>
                  $(function(){
                    $.ajax({
                        url: "/get-wallet",
                        method: "GET",
                        success: function(response) {
                            console.log(response);
                            $("\#public_key").val(response['public_key'])
                            $("\#private_key").val(response['private_key'])
                            $("\#blockchain_address").val(response['blockchain_address'])
                        },
                        error: function(error) {
                            console.log(error)
                        }
                    })

                    $("\#send_money").click(function(){
                        let confirm_text = 'Are you ready to send the given amount?'
                        let confirm_result = confirm(confirm_text)
                        if (!confirm_result) {
                            alert('Transaction cancelled')
                            return
                        }

                        let transaction = {
                            'private_key': $("\#private_key").val(),
                            'blockchain_address': $("\#blockchain_address").val(),
                            'public_key': $("\#public_key").val(),
                            'recipient_address': $("\#recipient_address").val(),
                            'amount': $("\#send_amount").val(),
                        }

                        $.ajax({
                            url: "/transaction",
                            method: "POST",
                            contentType: "application/json",
                            data: JSON.stringify(transaction),
                            success: function(response) {
                                console.log(response)
                                alert('success')
                            },
                            error: function(error) {
                                console.error(error)
                                alert('error')
                            }
                        })
                    })

                    function reload_amount() {
                        const address = $("\#blockchain_address").val()
                        console.log("get amount for address:", address)
                        const url = `/amount/${address}`
                        console.log("query amount url: ", url)
                        $.ajax({
                            url: url,
                            type: "GET",
                            success: function(response) {
                                let amount = response["amount"]
                                $("\#input_amount").text(amount)
                                console.log(amount)
                            },
                            error: function(error) {
                                console.error(error)
                            }
                        })
                    }

                    $("\#refresh_wallet").click(function(){
                        reload_amount()
                    })

                    setInterval(reload_amount, 3000)
                  })
               </script>
            </head>
            <body>
                <div>
                  <h1>Wallet</h1>
                   <div id="input_amount">0</div>
                   <button id="refresh_wallet">Refresh Wallet</button>
                   <p>Publick Key</p>
                   <textarea id="public_key" row="2" cols="100">
                   </textarea>

                   <p>Private Key</p>
                   <textarea id="private_key" row="1" cols="100">
                   </textarea>

                    <p>Blockchain address</p>
                   <textarea id="blockchain_address" row="1" cols="100">
                   </textarea>
                </div>

                <div>
                    <h1>Send Money</h1>
                    <div>
                        Address: <input id="recipient_address" size="100" type="text"></input>
                        <br>
                        Amount: <input id="send_amount" type="text"/>
                        <br>
                        <button id="send_money">Send</button>
                    </div>
                </div>

            </body>
            </html>
            "#
        )
    }
       
    async fn get_wallet_handler() -> HttpResponse{ 
        let wallet_user = Wallet::new();
        let wallet_data = wallet_user.get_wallet_data();
         HttpResponse::Ok().json(wallet_data)
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
                .route("/", web::get().to(ApiServer::get_index_handler))
                .route("/wallet", web::get().to(Self::get_wallet))
                .route("/get-wallet", web::get().to(Self::get_wallet_handler))
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