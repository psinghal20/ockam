use ockam::{Context, LocalEntity, Result, TcpTransport};
use ockam_get_started::Echoer;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    let cloud_address = "40.78.99.34:4000";
    let secure_channel_address = "secure_channel_listener";

    let tcp = TcpTransport::create(&ctx).await?;
    tcp.connect(cloud_address).await?;

    let mut entity = LocalEntity::create_with_worker(&ctx, "echoer", Echoer).await?;

    entity
        .secure_channel_listen_on_address(secure_channel_address)
        .await?;

    let forwarder = entity
        .forward(cloud_address, secure_channel_address)
        .await?;

    println!("Forwarding address: {}", forwarder.remote_address());

    Ok(())
}
