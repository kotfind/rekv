use minicbor::{Decode, Encode};

use crate::{Id, TableId};

pub trait Entity: Sized + Encode<()> + Decode<'static, ()> {
    const RAW_TABLE_ID: u8;
    const TABLE_ID: TableId = TableId::new(Self::RAW_TABLE_ID);

    const CBOR_MAX_LEN: usize;

    const DEBUG_NAME: &str;

    fn id(&self) -> Id<Self>;
}
