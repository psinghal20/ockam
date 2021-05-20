use ockam::{Address, Context, LocalEntity, RemoteEntity, Result, Route, TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    let cloud_address = "40.78.99.34:4000";
    let forwarding_address: Address = "adfe2b29".into();

    let tcp = TcpTransport::create(&ctx).await?;
    tcp.connect(cloud_address).await?;

    let cloud_address: Address = (TCP, cloud_address).into();
    let cloud_route: Route = vec![cloud_address, forwarding_address].into();

    let mut local = LocalEntity::create(&ctx, "initiator").await?;

    let channel = local.create_secure_channel(cloud_route).await?;

    if let Ok(channels) = local.list_secure_channels() {
        for c in channels {
            println!("Secure Channel: {} to {}", c.0, c.1);
        }
    }
    let route: Route = vec![channel, "echoer".into()].into();

    local.send(route, "Hello world!".to_string()).await?;

    let reply = local.receive::<String>().await?;
    println!("App Received: {}", reply);

    // Stop all workers, stop the node, cleanup and return.
    ctx.stop().await?;
    Ok(())
}
