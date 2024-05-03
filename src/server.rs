
use canzero_server::Server;

use crate::errors::Result;

pub async fn command_server() -> Result<()> {
    let server = Server::create().await?;

    server.start();
    loop {
        tokio::task::yield_now().await;
    }
}
