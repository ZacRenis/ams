use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct App {
    pub host: String,
    pub port: usize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Database {
    pub database_type: String,
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: usize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OSS {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub region: String,
    pub end_point: String,
    pub bucket: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Setting {
    pub app: App,
    pub database: Database,
    pub oss: OSS,
}

/// 從toml文件載入系統配置（儅沒有setting.toml用戶配置文件時自動加載默認default.setting.toml文件）
macro_rules! get_setting_from_toml {
    ($struct: ident) => ({
        let result = $struct::default();    // 設置默認返回參數
        let current_dir = if let Ok(v) = env::current_dir() { v } else { return result; };
        let current_path = if let Some(v) = current_dir.to_str() { v } else { return result; };
        let toml_file = format!("{}/setting.toml", current_path);   //自定義配置文件
        let default_toml_file = format!("{}/default.setting.toml", current_path);   //默認人配置文件
        match File::open(&toml_file) {  //嘗試加載自定義配置文件
            Ok(mut v) => {
                let mut content = String::new();
                if let Ok(_) = v.read_to_string(&mut content) {
                    if let Ok(t) = toml::from_str::<$struct>(&content) { t } else { result }
                } else { result }
            },
            Err(err) => {
                println!("读取文件失败：{:?}", err);
                match File::open(&default_toml_file) {  //嘗試加載默認配置文件
                    Ok(mut v) => {
                        let mut content = String::new();
                        if let Ok(_) = v.read_to_string(&mut content) {
                            if let Ok(t) = toml::from_str::<$struct>(&content) { t } else { result }
                        } else { result }
                    },
                    Err(err) => {
                        println!("读取文件失败: {}", err);
                        result
                    }
                }
            }
        }
    })
}

lazy_static! {
    pub static ref SETTING: Setting = { get_setting_from_toml!(Setting) };
    //pub static ref DB_INFO: Database = { dbg!(get_setting_from_toml!(Database)) };
    //pub static ref APP_INFO: App = { get_setting_from_toml!(App) };
    //pub static ref OSS_INFO: OSS = { get_setting_from_toml!(OSS) };
}

/// 獲取數據庫鏈接字符串
pub fn get_conn_string() -> String {
    let setting = &*SETTING;
    let db = &setting.database;
    format!(
        "{}://{}:{}@{}:{}/{}",
        db.database_type, db.user, db.password, db.host, db.port, db.name
    )
}

#[cfg(test)]
fn test_get_conn_string() {
    assert_eq!(
        get_conn_string(),
        "postgres://postgres:renzhichao@localhost:5432/ams"
    );
}
