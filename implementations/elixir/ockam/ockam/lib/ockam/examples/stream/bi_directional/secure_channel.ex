defmodule Ockam.Example.Stream.BiDirectional.SecureChannel do
  @moduledoc """

  Ping-pong example for bi-directional stream communication using local subsctiption

  Use-case: integrate ockam nodes which implement stream protocol consumer and publisher

  Pre-requisites:

  Ockam hub running with stream service and TCP listener

  Two ockam nodes "ping" and "pong"

  Expected behaviour:

  Two nodes "ping" and "pong" send messages to each other using two streams:
  "sc_listener_topic" to send messages to "pong" node
  "sc_initiator_topic" to send messages to "ping" node

  Implementation:

  Stream service is running on the hub node

  Ping and pong nodes create local consumers and publishers to exchange messages
  """
  alias Ockam.SecureChannel
  alias Ockam.Vault
  alias Ockam.Vault.Software, as: SoftwareVault

  alias Ockam.Example.Stream.Ping
  alias Ockam.Example.Stream.Pong

  alias Ockam.Stream.Client.BiDirectional
  alias Ockam.Stream.Client.BiDirectional.PublisherRegistry

  def config() do
    %{
      hub_ip: "13.64.73.230",
      hub_port: 4000,
      service_address: "stream_fedotov_danil_gmail_com_service",
      index_address: "stream_fedotov_danil_gmail_com_index"
    }
  end

  def secure_channel_listener() do
    ensure_tcp(5000)
    ## PONG worker
    {:ok, "pong"} = Pong.create(address: "pong")

    create_secure_channel_listener()

    ## Create a local subscription to forward pong_topic messages to local node
    subscribe("sc_listener_topic", "pong")
  end

  def secure_channel() do
    ensure_tcp(3000)

    ## PING worker
    Ping.create(address: "ping")

    ## Subscribe to response topic
    subscribe("sc_initiator_topic", "ping")

    ## Create local publisher worker to forward to pong_topic and add metadata to
    ## messages to send responses to ping_topic
    {:ok, publisher} = init_publisher("sc_listener_topic", "sc_initiator_topic")

    {:ok, channel} = create_secure_channel([publisher, "SC_listener"])

    ## Send a message THROUGH the local publisher to the remote worker
    send_message([channel, "pong"], ["ping"], "0")
  end

  defp create_secure_channel_listener() do
    {:ok, vault} = SoftwareVault.init()
    {:ok, identity} = Vault.secret_generate(vault, type: :curve25519)

    SecureChannel.create_listener(
      vault: vault,
      identity_keypair: identity,
      address: "SC_listener"
    )
  end

  defp create_secure_channel(route_to_listener) do
    {:ok, vault} = SoftwareVault.init()
    {:ok, identity} = Vault.secret_generate(vault, type: :curve25519)

    {:ok, c} =
      SecureChannel.create(route: route_to_listener, vault: vault, identity_keypair: identity)

    wait(fn -> SecureChannel.established?(c) end)
    {:ok, c}
  end

  defp wait(fun) do
    case fun.() do
      true ->
        :ok

      false ->
        :timer.sleep(100)
        wait(fun)
    end
  end

  def init_publisher(publisher_stream, consumer_stream) do
    BiDirectional.ensure_publisher(
      consumer_stream,
      publisher_stream,
      stream_options()
    )
  end

  def send_message(onward_route, return_route, payload) do
    msg = %{
      onward_route: onward_route,
      return_route: return_route,
      payload: payload
    }

    Ockam.Router.route(msg)
  end

  def ensure_tcp(port) do
    Ockam.Transport.TCP.create_listener(port: port, route_outgoing: true)
  end

  def subscribe(stream, subscription_id) do
    ## Local subscribe
    ## Create bidirectional subscription on local node
    ## using stream service configuration from stream_options
    BiDirectional.subscribe(stream, subscription_id, stream_options())

    ## This is necessary to make sure we don't spawn publisher for each message
    PublisherRegistry.start_link([])
  end

  def stream_options() do
    config = config()

    {:ok, hub_ip_n} = :inet.parse_address(to_charlist(config.hub_ip))
    tcp_address = %Ockam.Transport.TCPAddress{ip: hub_ip_n, port: config.hub_port}

    [
      service_route: [tcp_address, config.service_address],
      index_route: [tcp_address, config.index_address],
      partitions: 1
    ]
  end
end
