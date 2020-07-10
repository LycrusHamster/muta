use derive_more::Display;
use protocol::{ProtocolError, ProtocolErrorKind, ProtocolResult};

#[derive(Debug, Display)]
pub enum ExecutorError {
    #[display(fmt = "service {:?} was not found", service)]
    NotFoundService { service: String },
    #[display(fmt = "service {:?} method {:?} was not found", service, method)]
    NotFoundMethod { service: String, method: String },
    #[display(fmt = "Parsing payload to json failed {:?}", _0)]
    JsonParse(serde_json::Error),

    #[display(fmt = "Init service genesis failed: {:?}", _0)]
    InitService(String),
    #[display(fmt = "Query service failed: {:?}", _0)]
    QueryService(String),
    #[display(fmt = "Call service failed: {:?}", _0)]
    CallService(String),

    #[display(fmt = "Service {} canceled {:?}", service, reason)]
    Canceled {
        service: String,
        reason:  Option<String>,
    },

    #[display(fmt = "Tx run fails in Tx Before Hook")]
    TxBefore,

    #[display(fmt = "Tx run fails in Tx After Hook")]
    TxAfter,

    #[display(fmt = "Block run fails in Block Before Hook")]
    BlockBefore,

    #[display(fmt = "Block run fails in Block After Hook")]
    BlockAfter,
}

impl std::error::Error for ExecutorError {}

impl From<ExecutorError> for ProtocolError {
    fn from(err: ExecutorError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::Executor, Box::new(err))
    }
}
