use core::ops::Range;

use generic_array::GenericArray;
use typenum::U0;

use crate::{TableId, TableIdInner, key::TypeValue, util::GenericArrayExt};

pub const TYPE_VALUE: TypeValue = 0x01;

pub const LEN: usize = 2;

pub fn to_bytes(table_id: TableId) -> [u8; LEN] {
    let buf = GenericArray::<u8, U0>::default();

    let buf = buf.join_arr(&TYPE_VALUE.to_be_bytes());
    let buf = buf.join_arr(&table_id.to_inner().to_be_bytes());

    buf.into_array()
}

pub fn from_bytes(data: [u8; LEN]) -> Option<TableId> {
    let buf = GenericArray::from_array(data);

    let (buf, arr) = buf.split_arr();
    let table_id = TableIdInner::from_be_bytes(arr);

    let (buf, arr) = buf.split_arr();
    let type_value = TypeValue::from_be_bytes(arr);

    buf.assert_empty();

    (type_value == TYPE_VALUE).then_some(TableId::new(table_id))
}

pub fn full_range() -> Range<&'static [u8]> {
    const START: &[u8] = &TYPE_VALUE.to_be_bytes();
    const END: &[u8] = &(TYPE_VALUE + 1).to_be_bytes();
    START..END
}
