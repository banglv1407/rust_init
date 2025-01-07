use lazy_static::lazy_static;
use std::env;
lazy_static!{
    pub static ref ADDRESS: String=  set_address();
    pub static ref PORT: u16=  set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref SECRET: String = set_secret();
    pub static ref MAX_FILE_SIZE: u64 = set_max_file_size();

}

fn set_address() -> String{
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn set_port() -> u16{
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse::<u16>().unwrap()
}

fn set_database_url() -> String{
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

fn set_secret() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap_or("SECRET".to_string())
}

fn set_max_file_size() -> u64 {
    dotenv::dotenv().ok();
    env::var("MAX_FILE_SIZE")
    .unwrap_or("10485760".to_owned())
    .parse::<u64>()
    .expect("Can't parse the port")
}



