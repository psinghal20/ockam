use crate::{
    Entity, Profile, ProfileSync, RemoteEntity, RemoteForwarder, RemoteForwarderInfo, Route,
    SecureChannelTrait,
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
    pub async fn create(ctx: &'c Context) -> ockam_core::Result<LocalEntity<'c>> {
        let vault = Vault::create(&ctx)?;
        let default_profile = Profile::create(ctx, &vault).await?;
        let entity = Entity::new(default_profile);
        Ok(LocalEntity { ctx, vault, entity })
    }

    /// Create with a given worker
    pub async fn create_with_worker<A, M, W>(
        ctx: &'c Context,
        worker_address: A,
        worker: W,
    ) -> ockam_core::Result<LocalEntity<'c>>
    where
        A: Into<AddressSet>,
        M: Message + Send + 'static,
        W: Worker<Context = Context, Message = M>,
    {
        let local = LocalEntity::create(ctx).await?;

        ctx.start_worker(worker_address, worker).await?;

        Ok(local)
    }

    pub async fn secure_channel_listen_on_address<A: Into<Address>>(
        &mut self,
        address: A,
    ) -> ockam_core::Result<()> {
        self.entity
            .create_secure_channel_listener(&self.ctx, address.into(), &self.vault)
            .await
    }

    pub fn secure_channel_address(&self) -> String {
        "secure_channel_listener".to_string()
    }

    pub async fn secure_channel_listen(&mut self) -> ockam_core::Result<()> {
        let address = self.secure_channel_address();
        self.secure_channel_listen_on_address(address).await
    }

    pub async fn secure_channel_to(
        &mut self,
        remote_entity: RemoteEntity,
    ) -> ockam_core::Result<Address> {
        self.entity
            .create_secure_channel(self.ctx, remote_entity.route, &self.vault)
            .await
    }

    pub async fn forward<A: Into<Address>>(
        &mut self,
        remote_entity: RemoteEntity,
        service_address: A,
    ) -> ockam_core::Result<RemoteForwarderInfo> {
        let address = remote_entity.route.next().unwrap().to_string();
        let address = address.strip_prefix("0#").unwrap(); // TODO how can we clean this up?
        RemoteForwarder::create(&self.ctx, address, service_address.into()).await
    }

    pub async fn send<R, M>(&self, route: R, msg: M) -> ockam_core::Result<()>
    where
        R: Into<Route>,
        M: Message + Send + 'static,
    {
        self.ctx.send(route, msg).await
    }
}
