use ekv::{WriteTransaction, flash::Flash};
use embassy_sync::blocking_mutex::raw::RawMutex;

use crate::{Db, Entity, Id, IdCounterList, Rslt, key, util::GVec};

pub struct Wtx<'a, F: Flash + 'a, M: RawMutex + 'a> {
    pub(crate) wtx: WriteTransaction<'a, F, M>,
    pub(crate) id_counters: IdCounterList,
}

impl<'a, F: Flash + 'a, M: RawMutex + 'a> Wtx<'a, F, M> {
    pub(crate) async fn new(db: &'a Db<F, M>) -> Rslt<Self, F> {
        let wtx = db.db.write_transaction().await;
        let rtx = db.db.read_transaction().await;

        let id_counters = IdCounterList::read(&rtx).await?;

        Ok(Self { wtx, id_counters })
    }

    pub fn new_id<E: Entity>(&mut self) -> Rslt<Id<E>, F> {
        self.id_counters.new_id()
    }

    pub async fn write<E: Entity>(&mut self, e: &E) -> Rslt<(), F> {
        let key = key::entity::to_bytes(e.id());
        let val = {
            let mut buf = GVec::<u8, E::CBOR_MAX_LEN>::new();
            minicbor::encode(e, &mut buf)?;

            buf
        };

        self.wtx.write(&key, &val).await?;

        Ok(())
    }

    pub async fn commit(mut self) -> Rslt<(), F> {
        self.id_counters.write(&mut self.wtx).await?;
        self.wtx.commit().await?;

        Ok(())
    }
}
