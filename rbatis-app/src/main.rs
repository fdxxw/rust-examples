use axum::extract::Extension;
use axum::routing::get;
use axum::AddExtensionLayer;
use axum::{http::StatusCode, response::IntoResponse, Json, Router};
use rbatis::crud::CRUD;
use rbatis::crud_table;
use rbatis::rbatis::Rbatis;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
//mysql driver url
pub const MYSQL_URL: &'static str = "postgres://accu:123456@192.168.13.105:5434/ecs_dev_cloud";

//handler
pub async fn handler(rb: Extension<Arc<Rbatis>>) -> Json<Value> {
    let v = rb.fetch_list::<EnergyUnit>().await.unwrap_or_default();
    Json(serde_json::json!(v))
}

#[tokio::main]
async fn main() {
    //log
    fast_log::init(fast_log::config::Config::new().console()).unwrap();

    log::info!("linking database...");
    let rb = Rbatis::new();
    rb.link(MYSQL_URL).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);
    log::info!("linking database successful!");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .layer(AddExtensionLayer::new(rb));
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[crud_table(table_name:"acu_client")]
#[derive(Clone, Debug)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}

#[crud_table(table_name:"acu_client")]
#[derive(Clone, Debug)]
pub struct Client {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}
#[crud_table(table_name:"acu_energy_unit")]
#[derive(Clone, Debug)]
pub struct EnergyUnit {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub customer_id: Option<String>,
    pub name: Option<String>,
    pub inner_code: Option<String>,
    pub code: Option<String>,
}
