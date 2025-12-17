use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use z2p::run;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run().await
}

#[cfg(test)]
mod tests {
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
