use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::net::TcpStream;
use std::sync::Arc;

use enumflags2::{make_bitflags, BitFlags};
use prost::Message;
use rustls::pki_types::ServerName;
use ws_tool::{codec::BytesCodec, ClientBuilder};

use backend::connector::{Connector, Features};
use vendor::RequestRithmicSystemInfo;
use vendor::ResponseRithmicSystemInfo;

mod vendor;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Rithmic {}

#[typetag::serde]
impl Connector for Rithmic {
    fn features(self: &Self) -> BitFlags<Features> {
        make_bitflags!(Features::{OrderRouting | MarketData})
    }

    fn name(self: &Self) -> &str {
        "Rithmic"
    }
}

impl Rithmic {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_systems_info() -> Vec<String> {
        let uri = "wss://rprotocol-mobile.rithmic.com:443/"
            .parse::<http::Uri>()
            .unwrap();
        let hostname = uri.host().unwrap();
        let port = uri.port().unwrap();
        let mut roots = rustls::RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs().unwrap() {
            roots.add(cert).unwrap();
        }

        let mut stream;

        /*fn read_proxy_from_env() -> Option<Uri> {
            const ENV_VARS: &[&str] = &[
                "ALL_PROXY",
                "all_proxy",
                "HTTPS_PROXY",
                "https_proxy",
                "HTTP_PROXY",
                "http_proxy",
            ];

            for var in ENV_VARS {
                if let Ok(env) = std::env::var(var) {
                    return env.parse::<Uri>().ok();
                }
            }

            None
        } */



        if std::env::var("HTTPS_PROXY").is_ok() {
            let http_proxy = std::env::var("HTTPS_PROXY")
                .unwrap()
                .parse::<http::Uri>()
                .unwrap();
            //let proxy_addr = std::env::var("HTTPS_PROXY").expect("env HTTPS_PROXY not set");
            let config = hproxy::ProxyConfig {
                host: http_proxy.host().unwrap().to_string(),
                port: http_proxy.port().unwrap().as_u16(),
                auth: hproxy::AuthCredential::None,
                /*auth: hproxy::AuthCredential::Basic {
                    user: "cortep".into(),
                    passwd: "xxxxxxxxx".into(),
                },*/
                keep_alive: true,
            };
            stream = hproxy::create_conn(&config, "rprotocol-mobile.rithmic.com:443").unwrap();
        } else {
            stream = TcpStream::connect("rprotocol-mobile.rithmic.com:443").unwrap();
        }
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let mut conn = rustls::ClientConnection::new(
            Arc::new(config),
            ServerName::try_from(hostname.to_string()).unwrap(),
        )
        .unwrap();
        let tls = rustls::Stream::new(&mut conn, &mut stream);

        let mut client = ClientBuilder::new()
            .with_stream(uri, tls, BytesCodec::check_fn)
            .unwrap();

        let mut rq = RequestRithmicSystemInfo::default();
        rq.template_id = 16;
        rq.user_msg.push("Hello".into());
        let mut buf = Vec::new();
        buf.reserve(rq.encoded_len());
        rq.encode(&mut buf).unwrap();
        //let (mut read, mut write) = client.split();
        let _ = client.send(buf.to_vec().as_slice());
        //let _ = client.send(&buf.to_vec());
        let data = client.receive().unwrap().data;
        //let data = client.receive().unwrap();
        let response = ResponseRithmicSystemInfo::decode(&mut Cursor::new(data.to_vec())).unwrap();
        response.system_name.to_vec()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_systems_info() -> Vec<String> {
        let uri = "wss://rprotocol-mobile.rithmic.com:443/"
            .parse::<http::Uri>()
            .unwrap();
        let hostname = uri.host().unwrap();
        let port = uri.port().unwrap();

        let mut client = ClientBuilder::new()
            .with_stream(uri, tls, BytesCodec::check_fn)
            .unwrap();

        let mut rq = RequestRithmicSystemInfo::default();
        rq.template_id = 16;
        rq.user_msg.push("Hello".into());
        let mut buf = Vec::new();
        buf.reserve(rq.encoded_len());
        rq.encode(&mut buf).unwrap();
        //let (mut read, mut write) = client.split();
        let _ = client.send(buf.to_vec().as_slice());
        //let _ = client.send(&buf.to_vec());
        let data = client.receive().unwrap().data;
        //let data = client.receive().unwrap();
        let response = ResponseRithmicSystemInfo::decode(&mut Cursor::new(data.to_vec())).unwrap();
        response.system_name.to_vec()
    }
}
