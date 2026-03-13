use core::ops::Range;

use ekv::flash::Flash;
use heapless::Vec;

use crate::{Entity, Id, IdInner, TableId, TableIdInner, util::VecExt};

// -------------------- Entity --------------------

pub mod entity {
    use super::*;

    pub const TYPE_VALUE: TypeValue = 0x00;

    pub const LEN: usize = 4;

    pub fn to_bytes<E: Entity>(id: Id<E>) -> [u8; LEN] {
        let mut ans = Vec::<u8, LEN>::new();

        ans.extend(TYPE_VALUE.to_be_bytes());
        ans.extend(E::TABLE_ID.to_inner().to_be_bytes());
        ans.extend(id.to_inner().to_be_bytes());

        ans.into_array().expect("right size")
    }

    pub(crate) fn from_bytes<E: Entity, F: Flash>(data: [u8; LEN]) -> Option<Id<E>> {
        let mut ans = Vec::<u8, LEN>::from_array(data);

        let id = IdInner::from_be_bytes(ans.pop_slice());
        let table_id = TableIdInner::from_be_bytes(ans.pop_slice());
        let type_value = TypeValue::from_be_bytes(ans.pop_slice());

        ans.assert_empty();

        (table_id == E::TABLE_ID.to_inner() && type_value == TYPE_VALUE).then_some(Id::new(id))
    }
}

// -------------------- Id Counter --------------------

pub mod id_counter {
    use super::*;

    pub const TYPE_VALUE: TypeValue = 0x01;

    pub const LEN: usize = 2;

    pub fn to_bytes(table_id: TableId) -> [u8; LEN] {
        let mut ans = Vec::<u8, LEN>::new();

        ans.extend(TYPE_VALUE.to_be_bytes());
        ans.extend(table_id.to_inner().to_be_bytes());

        ans.into_array().expect("right size")
    }

    pub fn from_bytes(data: [u8; LEN]) -> Option<TableId> {
        let mut ans = Vec::<u8, LEN>::from_array(data);

        let table_id = TableIdInner::from_be_bytes(ans.pop_slice());
        let type_value = TypeValue::from_be_bytes(ans.pop_slice());

        ans.assert_empty();

        (type_value == TYPE_VALUE).then_some(TableId::new(table_id))
    }

    pub fn full_range() -> Range<&'static [u8]> {
        const START: &[u8] = &TYPE_VALUE.to_be_bytes();
        const END: &[u8] = &(TYPE_VALUE + 1).to_be_bytes();
        START..END
    }
}

// -------------------- General --------------------

pub type TypeValue = u8;
