use ockam::{Context, Profile, Result, Route, SecureChannelTrait, TcpTransport, Vault, TCP, Entity};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Create a cloud node by going to https://hub.ockam.network
    let cloud_node_tcp_address = "40.78.99.34:4000";

    let secure_channel_listener_forwarding_address =
        "2f7af426";

    // Initialize the TCP Transport.
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a TCP connection to your cloud node.
    tcp.connect(cloud_node_tcp_address).await?;

    let vault = Vault::create(&ctx)?;

    let mut alice = Entity::new(Profile::create(&ctx, &vault).await?);

    let channel = alice
        .create_secure_channel(
            &ctx,
            Route::new()
                .append_t(TCP, cloud_node_tcp_address)
                .append(secure_channel_listener_forwarding_address).into(),
            &vault,
        )
        .await?;

    ctx.send(
        Route::new().append(channel).append("echoer"),
        "Hello world!".to_string(),
    )
        .await?;

    // Wait to receive a reply and print it.
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // Stop all workers, stop the node, cleanup and return.
    ctx.stop().await
}
