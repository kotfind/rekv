use generic_array::GenericArray;
use typenum::U0;

use crate::{Entity, Id, IdInner, TableIdInner, key::TypeValue, util::GenericArrayExt};

pub const TYPE_VALUE: TypeValue = 0x00;

pub const LEN: usize = 4;

pub fn to_bytes<E: Entity>(id: Id<E>) -> [u8; LEN] {
    let buf = GenericArray::<u8, U0>::default();

    let buf = buf.join_arr(&TYPE_VALUE.to_be_bytes());
    let buf = buf.join_arr(&E::TABLE_ID.to_inner().to_be_bytes());
    let buf = buf.join_arr(&id.to_inner().to_be_bytes());

    buf.into_array()
}

pub(crate) fn from_bytes<E: Entity>(data: [u8; LEN]) -> Option<Id<E>> {
    let buf = GenericArray::from_array(data);

    let (buf, arr) = buf.split_arr();
    let id = IdInner::from_be_bytes(arr);

    let (buf, arr) = buf.split_arr();
    let table_id = TableIdInner::from_be_bytes(arr);

    let (buf, arr) = buf.split_arr();
    let type_value = TypeValue::from_be_bytes(arr);

    buf.assert_empty();

    (table_id == E::TABLE_ID.to_inner() && type_value == TYPE_VALUE).then_some(Id::new(id))
}
