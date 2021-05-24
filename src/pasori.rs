use crate::felica::FelicaTag;
use pafe_sys;

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

    pub fn poll(&self, card_type: CardType) -> Result<FelicaTag, String> {
        let card_type_raw = card_type.to_sys();
        let pointer;
        unsafe {
            // According to libpafe, RFU, the third parameter, is always 0.
            // It's probably safe to just hardcode it here.
            // npasoriv does the same.
            pointer = pafe_sys::felica_polling(self.pointer, card_type_raw, 0, 0);
        }

        let tag = FelicaTag { pointer };

        return Ok(tag)
    }
}

impl Drop for Pasori {
    fn drop(&mut self) {
        unsafe {
            pafe_sys::pasori_close(self.pointer)
        }
    }
}
