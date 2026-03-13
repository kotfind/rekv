use ekv::{Database, flash::Flash};
use embassy_sync::blocking_mutex::raw::RawMutex;

use crate::{Rslt, Wtx};

pub struct Db<F: Flash, M: RawMutex> {
    pub(crate) db: Database<F, M>,
}

impl<F: Flash, M: RawMutex> Db<F, M> {
    pub fn new(db: Database<F, M>) -> Self {
        Self { db }
    }

    pub async fn wtx<'a>(&'a self) -> Rslt<Wtx<'a, F, M>, F> {
        Wtx::new(self).await
    }
}
