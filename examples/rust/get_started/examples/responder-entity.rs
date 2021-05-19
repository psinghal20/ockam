use ockam::{Context, LocalEntity, RemoteEntity, Result, TcpTransport};
use ockam_get_started::Echoer;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    let mut local = LocalEntity::create_with_worker(&ctx, "echoer", Echoer).await?;

    let cloud_address = "40.78.99.34:4000";
    let cloud = RemoteEntity::create(cloud_address);

    let tcp = TcpTransport::create(&ctx).await?;
    tcp.connect(cloud_address).await?;

    local.secure_channel_listen().await?;

    let forwarder = local.forward(cloud, local.secure_channel_address()).await?;

    println!("Forwarding address: {}", forwarder.remote_address());

    Ok(())
}
