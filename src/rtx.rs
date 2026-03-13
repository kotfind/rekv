use ekv::{ReadTransaction, flash::Flash};
use embassy_sync::blocking_mutex::raw::RawMutex;

use crate::Db;

pub struct Rtx<'a, F: Flash + 'a, M: RawMutex + 'a> {
    rtx: ReadTransaction<'a, F, M>,
}

impl<'a, F: Flash + 'a, M: RawMutex + 'a> Rtx<'a, F, M> {
    pub(crate) async fn new(db: &'a Db<F, M>) -> Self {
        let rtx = db.db.read_transaction().await;

        Self { rtx }
    }
}
