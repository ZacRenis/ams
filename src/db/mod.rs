use crate::config::{Database, SETTING};
use mysql::{self, Pool, PooledConn};
use std::sync::Mutex;

lazy_static! {
    static ref POOLS: Mutex<Vec<Pool>> = Mutex::new(vec![]);
}

//初始化數據庫
pub fn init_connection(database_type: &String, conn_string: String) {
    println!("Initing connection: {:?} ...", conn_string);
    match database_type.as_str() {
        "mysql" => {
            let pool = mysql::Pool::new(conn_string).unwrap();
            let mut pools = POOLS.lock().unwrap();
            (*pools).push(pool);
        }
        "postgres" => {
            let pool = mysql::Pool::new(conn_string).unwrap();
            let mut pools = POOLS.lock().unwrap();
            (*pools).push(pool);
        }
        _ => {}
    }
}
