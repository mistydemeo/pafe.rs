use crate::felica::FelicaTag;
use pafe_sys;

pub enum Timeslot {
    N0,
    N1,
    N3,
    N7,
    NF,
}

// Taken from this document; these are the only supported values.
// https://www.sony.net/Products/felica/business/tech-support/data/card_usersmanual_2.11e.pdf
impl Timeslot {
    pub fn to_i(&self) -> u8 {
        match self {
            Self::N0 => 0,
            Self::N1 => 1,
            Self::N3 => 3,
            Self::N7 => 7,
            Self::NF => 0xf,
        }
    }
}

pub enum CardType {
    Any,
    Edy,
    Suica,
}

impl CardType {
    pub fn to_sys(&self) -> u16 {
        match self {
            Self::Any => pafe_sys::FELICA_POLLING_ANY,
            Self::Edy => pafe_sys::FELICA_POLLING_EDY,
            Self::Suica => pafe_sys::FELICA_POLLING_SUICA,
        }
    }
}

pub struct Pasori {
    pointer: *mut pafe_sys::Pasori,
}

impl Pasori {
    pub fn create() -> Pasori {
        let pasori;
        unsafe {
            pasori = pafe_sys::pasori_open();
        }

        Pasori { pointer: pasori }
    }

    pub fn init(&self) -> Result<&Self, String> {
        let result;
        unsafe {
            result = pafe_sys::pasori_init(self.pointer)
        }

        if result == 1 {
            return Err(format!("Unable to initialize PaSoRi; is it attached?"))
        } else {
            return Ok(self)
        }
    }

    pub fn poll(&self, card_type: CardType, timeslot: Timeslot) -> Option<FelicaTag> {
        let card_type_raw = card_type.to_sys();
        let pointer;
        let tag;
        unsafe {
            // According to libpafe, RFU, the third parameter, is always 0.
            // It's probably safe to just hardcode it here.
            // npasoriv does the same.
            pointer = pafe_sys::felica_polling(self.pointer, card_type_raw, 0, timeslot.to_i());
            if pointer.is_null() {
                return None;
            }
            tag = FelicaTag { tag: *pointer };
        }

        return Some(tag)
    }
}

impl Drop for Pasori {
    fn drop(&mut self) {
        unsafe {
            pafe_sys::pasori_close(self.pointer)
        }
    }
}
