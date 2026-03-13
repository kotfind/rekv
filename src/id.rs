use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem,
};

use ekv::{ReadTransaction, WriteTransaction, flash::Flash};
use embassy_sync::blocking_mutex::raw::RawMutex;
use minicbor::{Decode, Encode};

use crate::{Entity, Error, Rslt, key};

// -------------------- Table Id --------------------

pub(crate) type TableIdInner = u8;

pub const TABLE_ID_COUNT: usize = TableIdInner::MAX as usize + 1;

#[derive(Debug, Clone, Copy)]
pub struct TableId(TableIdInner);

impl TableId {
    pub(crate) const fn new(id: TableIdInner) -> Self {
        Self(id)
    }

    pub(crate) fn to_usize(self) -> usize {
        self.0 as usize
    }

    pub(crate) fn to_inner(self) -> TableIdInner {
        self.0
    }
}

// -------------------- Entity Id --------------------

pub(crate) type IdInner = u16;

pub const ID_COUNT: usize = IdInner::MAX as usize + 1;

pub struct Id<E: Entity>(pub(crate) u16, pub(crate) PhantomData<E>);

impl<E: Entity> fmt::Debug for Id<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id<{}>({})", E::DEBUG_NAME, self.0)
    }
}

impl<E: Entity> Clone for Id<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E: Entity> Copy for Id<E> {}

impl<'a, E: Entity, C> Decode<'a, C> for Id<E> {
    fn decode(d: &mut minicbor::Decoder<'a>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        IdInner::decode(d, ctx).map(Id::new)
    }
}

impl<E: Entity, C> Encode<C> for Id<E> {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        self.0.encode(e, ctx)
    }
}

impl<E: Entity> Id<E> {
    pub(crate) const fn new(id: u16) -> Self {
        Self(id, PhantomData)
    }

    pub(crate) fn to_inner(self) -> IdInner {
        self.0
    }
}

// -------------------- IdCounter --------------------

#[derive(Clone, Copy)]
struct IdCounter(IdInner);

impl IdCounter {
    fn new_id(&mut self) -> Option<IdInner> {
        let ans = self.0;
        self.0 = self.0.checked_add(1)?;
        Some(ans)
    }
}

pub(crate) struct IdCounterList([IdCounter; TABLE_ID_COUNT]);

impl IdCounterList {
    pub(crate) fn new_id<E: Entity, F: Flash>(&mut self) -> Rslt<Id<E>, F> {
        let inner = self.0[E::TABLE_ID.to_usize()]
            .new_id()
            .ok_or_else(|| Error::OutOfIds {
                entity_name: E::DEBUG_NAME,
            })?;

        Ok(Id::new(inner))
    }

    pub(crate) async fn read<'a, F: Flash, M: RawMutex>(
        rtx: &'a ReadTransaction<'a, F, M>,
    ) -> Rslt<Self, F> {
        let mut ans = Self([IdCounter(0); _]);

        let mut cursor = rtx.read_range(key::id_counter::full_range()).await?;

        loop {
            let mut key_buf = [0u8; key::id_counter::LEN];
            let mut val_buf = [0u8; mem::size_of::<IdInner>()];

            let Some((key_n, val_n)) = cursor.next(&mut key_buf, &mut val_buf).await? else {
                break;
            };

            if key_n != key_buf.len() {
                return Err(Error::BadFs {
                    msg: "wrong id_counter key length",
                });
            }

            if val_n != val_buf.len() {
                return Err(Error::BadFs {
                    msg: "wrong id_counter value length",
                });
            }

            let table_id = key::id_counter::from_bytes(key_buf).ok_or(Error::BadFs {
                msg: "failed to parse id_counter key",
            })?;
            let id_counter = IdCounter(IdInner::from_be_bytes(val_buf));

            ans.0[table_id.to_usize()] = id_counter;
        }

        Ok(ans)
    }

    pub(crate) async fn write<'a, 'b, F: Flash, M: RawMutex>(
        &'a self,
        wtx: &'a mut WriteTransaction<'b, F, M>,
    ) -> Rslt<(), F> {
        for (table_id_inner, id_counter) in self.0.iter().enumerate() {
            if id_counter.0 == 0 {
                continue;
            }

            let table_id = TableId(table_id_inner as u8);

            let key = key::id_counter::to_bytes(table_id);
            let val = id_counter.0.to_be_bytes();

            wtx.write(&key, &val).await?;
        }

        Ok(())
    }
}
