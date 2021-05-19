use ockam::{Context, LocalEntity, RemoteEntity, Result, Route, TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    let mut local = LocalEntity::create(&ctx).await?;

    let cloud_address = "40.78.99.34:4000";

    let cloud_node =
        RemoteEntity::create(Route::new().append_t(TCP, cloud_address).append("dc1c226a"));

    let tcp = TcpTransport::create(&ctx).await?;
    tcp.connect(cloud_address).await?;

    let channel = local.secure_channel_to(cloud_node).await?;

    let route = Route::create(vec![channel, "echoer".into()]);

    local.send(route, "Hello world!".to_string()).await?;

    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply);

    // Stop all workers, stop the node, cleanup and return.
    ctx.stop().await
}
