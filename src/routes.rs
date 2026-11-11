use actix_web::HttpResponse;
use actix_web::{Responder, delete, get, post, web::Json};

use crate::inputs::{CreateOrderInput, DeleteOrder};
use crate::outputs::{CreateOrderInputRepose, DeleteOrderResponse, Depth};

#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;
    return HttpResponse::Ok().json(CreateOrderInputRepose {
        order_id: String::from("ads"),
    });
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>) -> impl Responder {
    let order_id = body.order_id;
    HttpResponse::Ok().json(DeleteOrderResponse {
        filled_qty: 0,
        average_price: 100,
    })
}

#[get("/depth")]
pub async fn get_depth() -> impl Responder {
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        ask: vec![],
        last_update_id: String::from("adsa"),
    })
}
