use anyhow::Result;
use psy_core::ipc::{Request, Response};
use serde_json::json;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const SOCK_PATH: &str = "/run/psychy/netd.sock";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    if Path::new(SOCK_PATH).exists() { std::fs::remove_file(SOCK_PATH)?; }
    let listener = UnixListener::bind(SOCK_PATH)?;
    std::fs::set_permissions(SOCK_PATH, std::fs::Permissions::from_mode(0o666))?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle(stream));
    }
}

async fn handle(mut stream: UnixStream) -> Result<()> {
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let req: Request = serde_json::from_slice(&buf[..n])?;
    let resp = match req {
        Request::NewTab { url } => {
            tracing::info!("new tab: {:?}", url);
            Response::Ok(json!({"proxy":"socks5h://127.0.0.1:9050"}))
        }
        Request::NewIdentity => {
            tracing::info!("newnym sent");
            Response::Ok(json!({}))
        }
        Request::RoutingIntent { mode } => {
            tracing::info!("routing: {}", mode);
            Response::Ok(json!({"mode": mode}))
        }
    };
    stream.write_all(&serde_json::to_vec(&resp)?).await?;
    Ok(())
}
