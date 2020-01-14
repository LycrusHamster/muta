use bytes::Bytes;
use protocol::types::{Address, Hash, ServiceContext};
use protocol::{ProtocolError, ProtocolErrorKind, ProtocolResult};

pub trait ChainInterface {
    fn get_storage(&self, key: &Bytes) -> ProtocolResult<Bytes>;

    fn set_storage(&mut self, key: Bytes, val: Bytes) -> ProtocolResult<()>;

    fn contract_call(
        &mut self,
        address: Address,
        args: Bytes,
        current_cycle: u64,
    ) -> ProtocolResult<(String, u64)>;
}
