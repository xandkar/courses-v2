pub mod get;
pub mod serve;

#[derive(clap::Parser, Debug)]
pub struct Srv {
    #[clap(default_value = "127.0.0.1")]
    addr: std::net::IpAddr,

    #[clap(default_value = "3000")]
    port: u16,
}
