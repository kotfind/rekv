use core::marker::PhantomData;

use ekv::{Cursor, ReadError, ReadTransaction, flash::Flash};
use embassy_sync::blocking_mutex::raw::RawMutex;
use generic_array::GenericArray;
use typenum::U;

use crate::{Db, Entity, Error, Id, Rslt, key, util::GVec};

// -------------------- Rtx --------------------

pub struct Rtx<'a, F: Flash + 'a, M: RawMutex + 'a> {
    rtx: ReadTransaction<'a, F, M>,
}

impl<'a, F: Flash + 'a, M: RawMutex + 'a> Rtx<'a, F, M> {
    pub(crate) async fn new(db: &'a Db<F, M>) -> Self {
        let rtx = db.db.read_transaction().await;

        Self { rtx }
    }

    pub async fn read<E: Entity>(&self, id: Id<E>) -> Rslt<Option<E>, F> {
        let key = key::entity::to_bytes(id);

        let val = {
            let mut val_buf = GenericArray::<u8, E::CBOR_MAX_LEN>::default();

            match self.rtx.read(&key, &mut val_buf).await {
                Ok(val_len) => GVec::<u8, E::CBOR_MAX_LEN>::from_garr(&val_buf, val_len),
                Err(ReadError::KeyNotFound) => return Ok(None),
                Err(e) => Err(e)?,
            }
        };

        let entity: E = minicbor::decode(&val)?;

        if entity.id() != id {
            return Err(Error::BadFs {
                msg: "key's id doesn't match entity's",
            });
        }

        Ok(Some(entity))
    }

    pub async fn read_all<'b: 'a, E: Entity + 'b>(&'b self) -> Rslt<Batch<'a, F, M, E>, F> {
        Batch::new(self).await
    }
}

// -------------------- Batch --------------------

pub struct Batch<'a, F: Flash + 'a, M: RawMutex + 'a, E: Entity> {
    cursor: Cursor<'a, F, M>,
    marker: PhantomData<&'a E>,
}

impl<'a, F: Flash + 'a, M: RawMutex + 'a, E: Entity> Batch<'a, F, M, E> {
    async fn new<'b: 'a>(rtx: &'a Rtx<'b, F, M>) -> Rslt<Self, F> {
        let key_range = key::entity::full_range::<E>();

        let cursor = rtx.rtx.read_range(key_range).await?;

        Ok(Self {
            cursor,
            marker: PhantomData,
        })
    }

    pub async fn next(&mut self) -> Rslt<Option<E>, F> {
        let mut key_buf = GenericArray::<u8, U<{ key::MAX_CBOR_LEN }>>::default();
        let mut val_buf = GenericArray::<u8, E::CBOR_MAX_LEN>::default();

        let (key, val) = match self.cursor.next(&mut key_buf, &mut val_buf).await {
            Ok(Some((key_len, val_len))) => (
                GVec::<u8, U<{ key::MAX_CBOR_LEN }>>::from_garr(&key_buf, key_len),
                GVec::<u8, E::CBOR_MAX_LEN>::from_garr(&val_buf, val_len),
            ),
            Ok(None) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let entity: E = minicbor::decode(&val)?;

        let Ok(key_bytes) = (&key as &[u8]).try_into() else {
            return Err(Error::BadFs {
                msg: "entity key has a wrong length",
            });
        };

        match key::entity::from_bytes(key_bytes) {
            None => {
                return Err(Error::BadFs {
                    msg: "failed to parse entity key",
                });
            }
            Some(id) if id != entity.id() => {
                return Err(Error::BadFs {
                    msg: "key's id doesn't match entity's",
                });
            }
            _ => (),
        }

        Ok(Some(entity))
    }
}
