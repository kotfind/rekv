use crate::util::array_max;

pub mod entity;
pub mod id_counter;

type TypeValue = u8;

pub(crate) const CBOR_MAX_LEN: usize = array_max([entity::CBOR_LEN, id_counter::CBOR_LEN]);
