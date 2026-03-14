use core::fmt::Debug;

pub(crate) type TableIdInner = u8;

pub const TABLE_ID_COUNT: usize = TableIdInner::MAX as usize + 1;

#[derive(Debug, Clone, Copy)]
pub struct TableId(TableIdInner);

impl TableId {
    pub(crate) const fn new(id: TableIdInner) -> Self {
        Self(id)
    }

    pub(crate) const fn to_usize(self) -> usize {
        self.0 as usize
    }

    pub(crate) const fn to_inner(self) -> TableIdInner {
        self.0
    }
}
