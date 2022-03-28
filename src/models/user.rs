use serde::*;

#[derive(Debug,Deserialize,Serialize,sqlx::FromRow)]
pub struct Identity {
    pub name_ : String,
    #[serde(with = "serde_bytes")]
    pub identity_key : Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub signed_pre_key: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub signature : Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub one_time_pre_key : Vec<u8>,
}

#[derive(Debug,Deserialize,Serialize,sqlx::FromRow)]
pub struct NameOf {
    pub name_ : String,
   
}


#[derive(Debug,Deserialize,Serialize,sqlx::FromRow)]
pub struct IdentityStringify {
    pub name_ : String,
    pub identity_key : String,

    pub signed_pre_key: String,
  
    pub signature_ : String,

    pub one_time_pre_key : String
}


