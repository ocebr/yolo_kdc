mod lib;
mod db;
mod errors;
mod config;
mod models;
use db::user::UserRepository;
use models::user::{Identity,NameOf};
use actix_web::{web,web::Json,post, App, HttpServer,HttpRequest,middleware::Logger, Responder,HttpResponse,HttpMessage};
use crate::config::Config;
use serde::{Deserialize,Serialize};
use sqlx::{PgPool, postgres::PgQueryAs};
use lib::*;
use p256::{
    ecdsa::{ Signature}};


#[derive(Debug)]
pub struct Bundle {
    identity_key : IdentityKey,
    signed_pre_key: SignedPreKey,
    signature : Signature,
    one_time_pre_key : OneTimePreKey,

}


async fn stock_bundle(repository: UserRepository,req: HttpRequest,info : web::Json<Identity>) -> impl Responder{
    
    println!("{:?}", req);
    println!("allo ff");
    println!("{:?}", info);

    let bundle = Identity {
        name_ : info.name_.clone(),
        identity_key : info.identity_key.clone(),
        signed_pre_key : info.signed_pre_key.clone(),
        signature : info.signature.clone(),
        one_time_pre_key : info.one_time_pre_key.clone(),
        ephemeral_key : info.ephemeral_key.clone()

    };
    let store_bundle : std::result::Result<models::user::Identity, color_eyre::Report>;
    match repository.get_stored_bundle_of(bundle.name_.clone()).await{
        Ok(o) => store_bundle = repository.store_bundle(bundle.clone(), true).await,
        Err(e) =>  store_bundle = repository.store_bundle(bundle.clone(), false).await,

    }
    //let store_bundle = repository.store_bundle(bundle).await;
    
    HttpResponse::Ok().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","*").body("ok")
}

async fn get_bundle_of(repository : UserRepository , req : HttpRequest, info : Json<NameOf>) -> impl Responder{
    println!("dans get bundle of");
    println!("{}", info.name_);
  
    let bundle_to_return = repository.get_stored_bundle_of(info.name_.clone()).await;

    HttpResponse::Ok().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","*").json(bundle_to_return.unwrap())
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body

async fn index(info: web::Json<Info>) -> String {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config : Config = Config::from_env()
            .expect("error while server configuration");

    //pool (allow connection to be reuse for futures requests)

    let pool = config.db_pool().await.expect("pool error");

    HttpServer::new(move || { // move = move ownership !
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(
                web::resource("/test")
                    .route(web::get().to(|| HttpResponse::Ok())))
            .service(
                web::resource("/stock_bundle")
                .data(
                    web::JsonConfig::default())
                    .route(web::post().to(stock_bundle))   
            )
            .service(
                web::resource("/get_bundle_of")
                    .route(web::post().to(get_bundle_of))   
            )
            .service(web::resource("/json")
            .data(
                web::JsonConfig::default())
                        .route(web::post().to(index)))
            
            
            
            // .route("/stock_bundle", web::post().to(stock_bundle))
    })
    .bind(format!("{}:{}",config.host,config.port))?
    .run()
    .await
}