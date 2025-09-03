mod keys;
use minisign_verify::*;
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sig = fs::read_to_string("/var/lib/psychy/update.sig").await?;
    let bin = fs::read("/var/lib/psychy/update.zck").await?;
    PublicKey::from_base64(keys::PUBLIC_KEY)?
        .verify(&bin, &Signature::decode(&sig)?, false)?;
    println!("signature OK, applying update…");
    Ok(())
}
