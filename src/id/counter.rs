use core::mem;

use ekv::{ReadTransaction, WriteTransaction, flash::Flash};
use embassy_sync::blocking_mutex::raw::RawMutex;

use crate::{
    Entity, Error, Rslt,
    id::{
        entity::{Id, IdInner},
        table::{TABLE_ID_COUNT, TableId},
    },
    key,
};

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
            let mut key_buf = [0u8; key::id_counter::CBOR_LEN];
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

            let table_id = TableId::new(table_id_inner as u8);

            let key = key::id_counter::to_bytes(table_id);
            let val = id_counter.0.to_be_bytes();

            wtx.write(&key, &val).await?;
        }

        Ok(())
    }
}
