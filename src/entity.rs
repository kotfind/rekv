use generic_array::ArrayLength;
use minicbor::{Decode, Encode};

use crate::{Id, TableId};

pub trait Entity: Sized + Encode<()> + Decode<'static, ()> {
    #[allow(non_camel_case_types)]
    type CBOR_MAX_LEN: ArrayLength;

    const RAW_TABLE_ID: u8;
    const TABLE_ID: TableId = TableId::new(Self::RAW_TABLE_ID);

    const DEBUG_NAME: &str;

    fn id(&self) -> Id<Self>;
}
