#[macro_use]
extern crate lazy_static;

mod config;
mod db;
mod utils;
mod validations;

use actix_rt;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use config::SETTING;

async fn setting_json() -> impl Responder {
    let setting = &*SETTING; // 載入配置數據
    let conn_string = config::get_conn_string(); // 載入數據庫鏈接字符串
    println!("{:#?}", setting);
    HttpResponse::Ok().json(setting)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let setting = &*SETTING; // 載入配置數據
    let conn_string = config::get_conn_string(); // 載入數據庫鏈接字符串
    println!("{:#?}", &setting.app);
    db::init_connection(&setting.database.database_type,conn_string);
    HttpServer::new(||
        App::new().service(
            web::scope("/setting").route("", web::get().to(setting_json))
        )
    )
        .bind(format!("{}:{}", &setting.app.host, &setting.app.port))?
        .run()
        .await
}
