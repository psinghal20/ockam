defmodule Ockam.TokenLeaseManager.Lease do
  @moduledoc false
  defstruct id: "",
            issued: nil,
            renewable: false,
            tags: [],
            ttl: 0,
            value: ""
end
