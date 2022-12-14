
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {

}
///
/// server traint for common server.example is httpserver or mqttserver
#[async_trait::async_trait]
pub trait Server {
    fn start(&mut self) -> Result<()>;

    async fn stop(&mut self);
}