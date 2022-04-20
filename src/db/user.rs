use crate::{
    errors::AppError,
    models::user::{Identity,IdentityStringify},
    };
use actix_web::{web::Data, FromRequest ,HttpResponse, web::Json};
use sqlx::{PgPool, postgres::PgQueryAs,query_as};
use std::sync::Arc;
use std::ops::Deref;
use color_eyre::Result;
use futures::future::{ready,Ready};
use tracing::instrument;
use serde_json::{Value,json};
use serde::*;

pub struct UserRepository {
    pool: Arc<PgPool>
}
pub fn parse_bundle_arguments(s:String)-> Vec<u8> {
    //let mut buf = String::from("[0, 0, 0, 0, 0, 178, 0, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 70, 107, 119, 69, 119, 89, 72, 75, 111, 90, 73, 122, 106, 48, 67, 65, 81, 89, 73, 75, 111, 90, 73, 122, 106, 48, 68, 65, 81, 99, 68, 81, 103, 65, 69, 51, 110, 105, 67, 104, 85, 102, 57, 47, 53, 85, 116, 104, 118, 83, 105, 52, 68, 119, 47, 72, 48, 66, 113, 83, 86, 105, 103, 10, 56, 97, 122, 113, 77, 111, 113, 75, 76, 114, 122, 53, 116, 102, 55, 81, 101, 79, 114, 111, 113, 105, 74, 118, 83, 86, 52, 90, 118, 117, 78, 108, 90, 76, 110, 119, 83, 106, 85, 118, 79, 119, 122, 122, 49, 55, 72, 116, 99, 113, 75, 68, 104, 48, 99, 88, 56, 65, 61, 61, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10]");
            
         let mut s = s.replace("[", "");
         let mut s = s.replace("]", "");
         let t = s.replace(" ",""); //enlever les spaces
         let a : Vec<String> = t.split(",").map(str::to_string).collect();
         //let int = s.parse::<u8>().unwrap();
         //println!("{:?}",a);
         let mut vec : Vec<u8> = Vec::new();
         for i in 0..a.len() {
             vec.push( a[i].parse::<u8>().unwrap())
         }
        // println!("{:?}", vec);
         vec
}



impl UserRepository {
    pub fn new(pool:Arc<PgPool>) -> Self {
        Self {pool}
    }

    #[instrument(skip(self))]
    pub async fn store_bundle(&self, bundle: Identity, replace_or_not : bool) -> Result<Identity> {
        //println!("{:?}",format!("{:?}",bundle.identity_key));
    if replace_or_not == false {
        //println!("dans le if");
        let bundle1 = sqlx::query_as::<_, Identity>("insert into yolo_bundle (name_ ,identity_key,signed_pre_key,signature_,one_time_pre_key,ephemeral_key) values ($1,$2,$3,$4,$5,$6) returning *")
            .bind(format!("{:?}",bundle.name_))
            .bind(format!("{:?}",bundle.identity_key))
            .bind(format!("{:?}",bundle.signed_pre_key))
            .bind(format!("{:?}",bundle.signature))
            .bind(format!("{:?}",bundle.one_time_pre_key))
            .bind(format!("{:?}",bundle.ephemeral_key))
            .fetch_one(&*self.pool)
            .await?;

            //println!("SQL :     {:?}",bundle1);
            Ok(bundle1)
    }else {
        println!("dans le else");
        let bundle1 = sqlx::query_as::<_, Identity>("update yolo_bundle set identity_key = $2 , signed_pre_key = $3, signature_ = $4, one_time_pre_key = $5, ephemeral_key = $6 where name_ = $1 returning *")
        .bind(format!("{:?}",bundle.name_))
        .bind(format!("{:?}",bundle.identity_key))
        .bind(format!("{:?}",bundle.signed_pre_key))
        .bind(format!("{:?}",bundle.signature))
        .bind(format!("{:?}",bundle.one_time_pre_key))
        .bind(format!("{:?}",bundle.ephemeral_key))
        .fetch_one(&*self.pool)
        .await?;
        Ok(bundle1)
    }
            // insert into yolo_bundle (identity_key,signed_pre_key,signature,one_time_pre_key) values (ARRAY [0, 0, 0, 0, 0, 178, 0, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 70, 107, 119, 69, 119, 89, 72, 75, 111, 90, 73, 122, 106, 48, 67, 65, 81, 89, 73, 75, 111, 90, 73, 122, 106, 48, 68, 65, 81, 99, 68, 81, 103, 65, 69, 49, 98, 68, 78, 110, 112, 89, 85, 73, 118, 122, 116, 74, 83, 43, 114, 50, 54, 119, 101, 66, 87, 98, 117, 73, 79, 84, 105, 10, 116, 75, 120, 54, 79, 114, 90, 105, 49, 113, 50, 122, 109, 65, 102, 100, 99, 77, 90, 87, 114, 99, 76, 88, 85, 118, 106, 122, 115, 104, 97, 88, 118, 84, 47, 113, 85, 106, 43, 73, 69, 98, 87, 81, 112, 86, 81, 103, 81, 81, 117, 81, 112, 110, 76, 103, 55, 81, 61, 61, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10], ARRAY [2, 0, 0, 0, 0, 178, 0, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 70, 107, 119, 69, 119, 89, 72, 75, 111, 90, 73, 122, 106, 48, 67, 65, 81, 89, 73, 75, 111, 90, 73, 122, 106, 48, 68, 65, 81, 99, 68, 81, 103, 65, 69, 69, 114, 121, 86, 85, 75, 80, 79, 81, 97, 77, 105, 122, 83, 82, 52, 106, 71, 51, 80, 103, 50, 107, 84, 105, 101, 120, 116, 10, 81, 118, 75, 119, 77, 67, 114, 117, 118, 51, 106, 43, 101, 49, 85, 75, 113, 43, 80, 85, 79, 55, 89, 50, 83, 98, 115, 51, 107, 49, 102, 97, 114, 101, 102, 105, 90, 122, 97, 66, 89, 80, 79, 53, 55, 90, 51, 114, 111, 80, 69, 81, 115, 71, 84, 57, 112, 119, 61, 61, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10], ARRAY [57, 152, 134, 35, 172, 141, 188, 214, 208, 150, 23, 28, 65, 106, 169, 215, 14, 179, 187, 113, 21, 245, 59, 66, 48, 39, 21, 77, 174, 48, 35, 71, 96, 187, 83, 79, 221, 245, 34, 227, 77, 129, 160, 64, 30, 85, 106, 79, 27, 123, 26, 8, 99, 174, 76, 220, 47, 97, 142, 153, 187, 255, 185, 158], ARRAY  [3, 0, 0, 0, 0, 178, 0, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 70, 107, 119, 69, 119, 89, 72, 75, 111, 90, 73, 122, 106, 48, 67, 65, 81, 89, 73, 75, 111, 90, 73, 122, 106, 48, 68, 65, 81, 99, 68, 81, 103, 65, 69, 112, 74, 107, 49, 104, 118, 76, 69, 87, 75, 102, 120, 53, 50, 115, 80, 79, 55, 67, 43, 88, 50, 114, 57, 102, 77, 113, 76, 10, 117, 53, 70, 112, 67, 48, 56, 56, 48, 98, 122, 110, 83, 104, 77, 116, 103, 84, 110, 85, 90, 76, 104, 99, 98, 80, 80, 120, 73, 47, 115, 57, 115, 119, 84, 83, 84, 55, 81, 99, 101, 90, 120, 88, 51, 99, 81, 71, 117, 69, 71, 108, 121, 106, 74, 100, 111, 81, 61, 61, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10]) returning *;

    }
    pub async fn get_stored_bundle_of(&self , name_ : String) -> Result<Identity,&str>
    {
    //println!("dans db --> get bundle of ");
    //println!("{:?}",name_.clone());
        //TODO :
            //  - store un nom dans la db
            // get le bundle du nom
            // formater le bundle du nom en json 
            // le renvoyer dans une response HTTP
        let bundle_to_return = sqlx::query_as::<_, IdentityStringify>("select * from yolo_bundle where name_ = $1 ;")//"select * from yolo_bundle where name_ = '$1';")
            .bind(format!("{:?}",name_))
            .fetch_optional(&*self.pool)
            .await;
        let bundle_to_return_stringify = bundle_to_return.unwrap();

        if bundle_to_return_stringify.is_some(){
            let bundle_to_return_stringify2 = bundle_to_return_stringify.clone().unwrap();
            println!("{}", bundle_to_return_stringify2.name_);
                    let bundle = Identity {
                        name_ : bundle_to_return_stringify2.name_,
                        identity_key: parse_bundle_arguments(bundle_to_return_stringify2.identity_key),
                        signed_pre_key : parse_bundle_arguments(bundle_to_return_stringify2.signed_pre_key),
                        signature : parse_bundle_arguments(bundle_to_return_stringify2.signature_),
                        one_time_pre_key: parse_bundle_arguments(bundle_to_return_stringify2.one_time_pre_key),
                        ephemeral_key : parse_bundle_arguments(bundle_to_return_stringify2.ephemeral_key)
                    };
                   // println!("{:?}", bundle);
                    Ok(bundle)
            
        }else{
            Err("")
        }
     
        // println!("bundle_to_return :    {:?}", bundle_to_return.name_);
        // println!("bundle_to_return :    {:?}", bundle_to_return.identity_key);
        // println!("bundle_to_return :    {:?}", bundle_to_return.signed_pre_key);
        // println!("bundle_to_return :    {:?}", bundle_to_return.signature_);
        // println!("bundle_to_return :    {:?}", bundle_to_return.one_time_pre_key);
        //println!("bundle a parser : {:?}", bundle_to_return_stringify.identity_key);
        //let vec :Vec<u8> = parse_bundle_arguments(bundle_to_return_stringify.identity_key);
       // println!("{:?}", vec);

       

    }
}

impl FromRequest for UserRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req, payload))]
    fn from_request(    
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PgPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(UserRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}