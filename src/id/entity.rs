use core::{fmt, marker::PhantomData};

use minicbor::{Decode, Encode};

use crate::Entity;

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

impl<E: Entity> PartialEq for Id<E> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<E: Entity> Eq for Id<E> {}

impl<E: Entity> Id<E> {
    pub const fn new_unsafe(id: IdInner) -> Self {
        Self::new(id)
    }

    pub(crate) const fn new(id: IdInner) -> Self {
        Self(id, PhantomData)
    }

    pub fn to_inner(self) -> IdInner {
        self.0
    }
}
