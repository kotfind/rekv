use core::{array, ops::Range};

use embassy_sync::lazy_lock::LazyLock;
use generic_array::GenericArray;
use typenum::U0;

use crate::{
    Entity, Id, IdInner, TABLE_ID_COUNT, TableId, TableIdInner, key::TypeValue, util::GArrExt,
};

pub const TYPE_VALUE: TypeValue = 0x00;

pub const CBOR_LEN: usize = 4;

pub fn to_bytes<E: Entity>(id: Id<E>) -> [u8; CBOR_LEN] {
    let buf = GenericArray::<u8, U0>::default();

    let buf = buf.join_arr(&TYPE_VALUE.to_be_bytes());
    let buf = buf.join_arr(&E::TABLE_ID.to_inner().to_be_bytes());
    let buf = buf.join_arr(&id.to_inner().to_be_bytes());

    buf.into_array()
}

pub(crate) fn from_bytes<E: Entity>(data: [u8; CBOR_LEN]) -> Option<Id<E>> {
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

pub(crate) fn full_range<E: Entity>() -> Range<&'static [u8]> {
    fn start(table_id: TableId) -> [u8; 2] {
        let buf = GenericArray::<u8, U0>::default();

        let buf = buf.join_arr(&TYPE_VALUE.to_be_bytes());
        let buf = buf.join_arr(&table_id.to_inner().to_be_bytes());

        buf.into_array()
    }

    fn end(table_id: TableId) -> [u8; 2] {
        let buf = GenericArray::<u8, U0>::default();

        let buf = buf.join_arr(&TYPE_VALUE.to_be_bytes());
        let buf = buf.join_arr(&(table_id.to_inner() + 1).to_be_bytes());

        buf.into_array()
    }

    static RANGES: LazyLock<[([u8; 2], [u8; 2]); TABLE_ID_COUNT]> = LazyLock::new(|| {
        array::from_fn(|table_id_inner| {
            let table_id = TableId::new(table_id_inner as TableIdInner);
            (start(table_id), end(table_id))
        })
    });

    let (ref start, ref end) = RANGES.get()[E::TABLE_ID.to_usize()];

    (start as &[u8])..(end as &[u8])
}
