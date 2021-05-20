use ockam::{Address, Context, LocalEntity, RemoteEntity, Result, TcpTransport};
use ockam_get_started::Echoer;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    let cloud_hub = "40.78.99.34:4000";
    let cloud_address: Address = cloud_hub.into();

    let cloud = RemoteEntity::create(cloud_address);

    let tcp = TcpTransport::create(&ctx).await?;
    tcp.connect(cloud_hub).await?;

    let mut local = LocalEntity::create_with_worker(&ctx, "echoer", Echoer).await?;

    local
        .create_secure_channel_listener("secure_channel_listener")
        .await?;

    let forwarder = local.forward(cloud, local.secure_channel_address()).await?;

    println!("Forwarding address: {}", forwarder.remote_address());

    Ok(())
}
