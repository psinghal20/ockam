import Config

influx_token =
  with true <- File.exists?("/mnt/secrets/influx/token"),
       {:ok, contents} <- File.read("/mnt/secrets/influx/token"),
       client_secret <- String.trim(contents) do
    client_secret
  else
    false ->
      System.get_env("INFLUXDB_TOKEN")

    {:error, :enoent} ->
      System.get_env("INFLUXDB_TOKEN")
  end

config :telemetry_influxdb,
  host: System.get_env("INFLUXDB_HOST"),
  port: System.get_env("INFLUXDB_PORT"),
  bucket: System.get_env("INFLUXDB_BUCKET"),
  org: System.get_env("INFLUXDB_ORG"),
  token: influx_token

config :ockam_hub, :influxdb,
  host: System.get_env("INFLUXDB_HOST"),
  port: System.get_env("INFLUXDB_PORT"),
  bucket: System.get_env("INFLUXDB_BUCKET"),
  org: System.get_env("INFLUXDB_ORG"),
  token: influx_token

ui_auth_message =
  with true <- File.exists?("/mnt/secrets/auth/message"),
       {:ok, contents} <- File.read("/mnt/secrets/auth/message"),
       client_secret <- String.trim(contents) do
    client_secret
  else
    false ->
      System.get_env("AUTH_MESSAGE") || "devsecret"

    {:error, :enoent} ->
      System.get_env("AUTH_MESSAGE") || "devsecret"
  end

ui_auth_host =
  with true <- File.exists?("/mnt/secrets/auth/host"),
       {:ok, contents} <- File.read("/mnt/secrets/auth/host"),
       client_secret <- String.trim(contents) do
    client_secret
  else
    false ->
      System.get_env("AUTH_HOST") || "http://localhost:4001"

    {:error, :enoent} ->
      System.get_env("AUTH_HOST") || "http://localhost:4001"
  end

node_fqdn =
  case System.get_env("NODE_FQDN") do
    fqdn when is_binary(fqdn) and length(fqdn) > 0 ->
      fqdn

    _ ->
      case config_env() do
        :dev -> "localhost"
        :test -> "localhost"
        _ -> "1.node.ockam.network"
      end
  end

node_ip =
  case config_env() do
    :prod ->
      {:ok, {:hostent, _, [_ | _], :inet, 4, [node_ip]}} =
        :inet.gethostbyname(to_charlist(node_fqdn))

      node_ip

    :test ->
      {127, 0, 0, 1}

    :dev ->
      {127, 0, 0, 1}
  end

config :ockam_hub,
  auth_message: ui_auth_message,
  auth_host: ui_auth_host,
  node_ip: node_ip,
  node_fqdn: node_fqdn
