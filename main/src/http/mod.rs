use misc::server::{Server};

pub struct HttpServer{

}

impl HttpServer {
    fn new()->HttpServer{
        let server =  HttpServer{};

        server
    }
}
#[async_trait::async_trait]
impl Server for HttpServer {
    fn start(&mut self) -> misc::server::Result<()> {

        tokio::spawn(async {
            println!("AAAA");
        });
        Ok(())
    }

    async fn stop(&mut self) {
        todo!()
    }
}

mod test{
    use crate::http::HttpServer;
    use misc::server::Server;

    #[test]
    fn test(){
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.spawn(async{
            let mut server = HttpServer::new();
            server.start();
        });

        println!("AAA")
    }
}