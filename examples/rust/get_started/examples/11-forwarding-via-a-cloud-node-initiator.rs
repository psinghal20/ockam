// This node routes a message, to a different node, using a forwarding address on the cloud node.

use ockam::{Context, Result, Route, TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Create a cloud node by going to https://hub.ockam.network
    let cloud_node_tcp_address = "Paste the tcp address of your cloud node here.";

    // Run 11-forwarding-via-a-cloud-node-responder,
    // it will print the forwarding address of echoer on your cloud node
    let echoer_forwarding_address = "Paste the forwarding address of the echoer here.";

    // Initialize the TCP Transport.
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a TCP connection to your cloud node.
    tcp.connect(cloud_node_tcp_address).await?;

    // Send a message to the echoer worker, on a different node,
    // using a forwarding address on your cloud node
    ctx.send(
        Route::new()
            .append_t(TCP, cloud_node_tcp_address)
            .append(echoer_forwarding_address),
        "Hello Ockam!".to_string(),
    )
    .await?;

    // Wait to receive a reply and print it.
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // Stop all workers, stop the node, cleanup and return.
    ctx.stop().await
}
