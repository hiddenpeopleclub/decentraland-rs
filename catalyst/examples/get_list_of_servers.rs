extern crate catalyst;

use catalyst::*;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::production();
    let servers = LambdaClient::servers(&server).await?;

    for server in servers {
        println!(" - {}", server.base_url);
    }
    Ok(())
}
