use core::fmt::Debug;

use ekv::flash::Flash;
use thiserror::Error;

use crate::util::CapacityError;

pub(crate) type Rslt<T, F> = Result<T, Error<F>>;

#[derive(Error)]
pub enum Error<F: Flash> {
    #[error("ekv mount error")]
    EkvMount(#[from] ekv::MountError<F::Error>),

    #[error("ekv format error")]
    EkvFormat(#[from] ekv::FormatError<F::Error>),

    #[error("ekv write error")]
    EkvWrite(#[from] ekv::WriteError<F::Error>),

    #[error("ekv batch read error")]
    EkvBatchRead(#[from] ekv::Error<F::Error>),

    #[error("ekv cursor error")]
    EkvCursorError(#[from] ekv::CursorError<F::Error>),

    #[error("ekv commit error")]
    EkvCommit(#[from] ekv::CommitError<F::Error>),

    #[error("failed to encode as cbor")]
    CborEncode(#[from] minicbor::encode::Error<CapacityError>),

    #[error("maximum id value reached for {entity_name}")]
    OutOfIds { entity_name: &'static str },

    #[error("wrong filesystem state")]
    BadFs { msg: &'static str },
}

impl<F: Flash> Debug for Error<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EkvMount(e) => f.debug_tuple("EkvMount").field(e).finish(),
            Self::EkvFormat(e) => f.debug_tuple("EkvFormat").field(e).finish(),
            Self::EkvWrite(e) => f.debug_tuple("EkvWrite").field(e).finish(),
            Self::EkvBatchRead(e) => f.debug_tuple("EkvBatchRead").field(e).finish(),
            Self::EkvCursorError(e) => f.debug_tuple("EkvCursorError").field(e).finish(),
            Self::EkvCommit(e) => f.debug_tuple("EkvCommit").field(e).finish(),
            Self::CborEncode(e) => f.debug_tuple("CborEncode").field(e).finish(),
            Self::OutOfIds { entity_name } => f
                .debug_struct("OutOfIds")
                .field("entity_name", entity_name)
                .finish(),
            Self::BadFs { msg } => f.debug_struct("BadFs").field("msg", msg).finish(),
        }
    }
}
