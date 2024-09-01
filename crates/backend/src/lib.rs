use connector::Connector;

pub mod connector;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Connection {
    pub name: String,
    pub connector: Box<dyn Connector>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Account {
    name: String,
    pnl: i32,
}
