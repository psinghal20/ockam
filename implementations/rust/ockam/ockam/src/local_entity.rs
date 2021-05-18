use crate::{
    Entity, Profile, ProfileSync, RemoteForwarder, RemoteForwarderInfo, SecureChannelTrait,
};
use ockam_core::{Address, AddressSet, Message, Worker};
use ockam_node::Context;
use ockam_vault_sync_core::Vault;

pub struct LocalEntity<'c> {
    ctx: &'c Context,
    vault: Address,
    entity: Entity<ProfileSync>,
}

impl<'c> LocalEntity<'c> {
    /// Create
    pub async fn create<A, M, W>(
        ctx: &'c Context,
        worker_address: A,
        worker: W,
    ) -> ockam_core::Result<LocalEntity<'c>>
    where
        A: Into<AddressSet>,
        M: Message + Send + 'static,
        W: Worker<Context = Context, Message = M>,
    {
        let vault = Vault::create(&ctx)?;
        let default_profile = Profile::create(ctx, &vault).await?;
        let entity = Entity::new(default_profile);

        ctx.start_worker(worker_address, worker).await?;

        let local = LocalEntity { ctx, vault, entity };
        Ok(local)
    }

    pub async fn secure_channel_listen<A: Into<Address>>(
        &mut self,
        address: A,
    ) -> ockam_core::Result<()> {
        self.entity
            .create_secure_channel_listener(&self.ctx, address.into(), &self.vault)
            .await
    }

    pub async fn forward<A: Into<Address>, S: Into<String>>(
        &mut self,
        hub_address: S,
        service_address: A,
    ) -> ockam_core::Result<RemoteForwarderInfo> {
        RemoteForwarder::create(&self.ctx, hub_address.into(), service_address.into()).await
    }
}
