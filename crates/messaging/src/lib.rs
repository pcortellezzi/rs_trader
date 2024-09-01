#[derive(Debug)]
pub enum ToBackend {
    GetConnections,
    ConnectionCreate(String),
    ConnectionUpdate(String),
    ConnectionDelete(String),
    ConnectionConnect(u32),
    ConnectionDisconnect(u32),
    ConnectionStatus(u32),
}

#[derive(Debug)]
pub enum ToFrontend {
    DataList(Vec<String>),
}