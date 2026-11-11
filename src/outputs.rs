use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderInputRepose {
    pub order_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteOrderResponse {
    pub filled_qty: u32,
    pub average_price: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Depth {
    pub bids: Vec<[u32; 2]>,
    pub ask: Vec<[u32; 2]>,
    pub last_update_id: String,
}
