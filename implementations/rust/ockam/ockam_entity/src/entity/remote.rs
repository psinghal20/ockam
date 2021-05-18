use ockam_core::Address;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct RemoteEntity {
    pub address: Option<Address>,
}

impl RemoteEntity {
    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    pub fn from_address<A: Into<Address>>(address: A) -> Self {
        RemoteEntity {
            address: Some(address.into()),
        }
    }

    pub fn with_address<A: Into<Address>>(&mut self, address: A) -> &mut Self {
        self.address = Some(address.into());
        self
    }
}

#[test]
fn test_remote_entity() {
    let mut re: RemoteEntity = Default::default();
    assert!(!re.has_address());

    let re = re.with_address("127.0.0.1");
    assert!(re.has_address());

    let mut local = RemoteEntity::from_address("127.0.0.1");

    assert_eq!(re, &mut local);
}
