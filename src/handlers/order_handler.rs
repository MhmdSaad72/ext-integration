// use actix_web::{
//     post,
//     web::{Bytes, Data, Path},
//     HttpRequest, HttpResponse,
// };
// use serde_json::json;

// use crate::DbPool;
// #[post("/create-awb/{platform}/{shop_id}")]
// async fn _recive_orders(
//     conn: Data<DbPool>,
//     payload: Bytes,
//     path: Path<(String, u64)>,
// ) -> HttpResponse {
//     let (platform, shop_id) = path.into_inner();

//     println!("Platform 1: {}, Shop ID {}", platform, shop_id);
//     HttpResponse::Ok().json("\"body\":\"Hello World\"")
// }

// #[post("/create-awb/{platform}")]
// async fn recive_orders(
//     conn: Data<DbPool>,
//     req: HttpRequest,
//     platform: Path<String>,
// ) -> HttpResponse {
//     let platform = platform.into_inner();
//     // let request = req.headers();
//     // println!("req: {:#?}", platform);
//     // println!("Headers : {:#?}", request);
//     HttpResponse::Ok().json(json!({"Success":true, "Platform":platform}))
// }
