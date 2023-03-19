pub mod model;

use crate::model::{init_db, BizActivity};
use rbs::to_value;
use serde_json::json;

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    //query
    let table: Option<BizActivity> = rb
        .query_decode("select * from biz_activity limit ?", vec![to_value!(1)])
        .await
        .unwrap();
    //exec
    let result = rb
        .exec("update biz_activity set status = 0 where id > 0", vec![])
        .await
        .unwrap();
    log::logger().flush();
    println!(">>>>> table={}", json!(table));
    println!(">>>>> exec={}", result);
}
