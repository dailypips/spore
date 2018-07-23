pub enum Message {
    // Incoming Messages
    //TickPrice(TickPrice),


    // Outgoing Messages
}

pub struct Source {
    //timestamp: Time,
    provider_id: i32,
}

pub struct TickPriceMsg {
    source: Source,
    price: f64,
}

// -> Queue  -> socket
// ->
