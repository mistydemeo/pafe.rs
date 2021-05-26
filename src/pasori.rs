use crate::felica::FelicaTag;
use pafe_sys;

/// Specifies the timeslot to use when polling.
/// This affects the number of cards which can be read at once.
/// For more information, see [Sony's documentation](https://www.sony.net/Products/felica/business/tech-support/data/card_usersmanual_2.11e.pdf).
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
    /// Converts a Timeslot variant into a u8 suitable for use with
    /// the `pafe_sys::felica_polling` function.
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::N0 => 0,
            Self::N1 => 1,
            Self::N3 => 3,
            Self::N7 => 7,
            Self::NF => 0xf,
        }
    }
}

/// Specifies the type of card to interact with.
/// Primarily used by functions that read or write data.
pub enum CardType {
    /// Any card
    Any,
    /// Rakuten Edy
    Edy,
    /// JR East Suica
    Suica,
}

impl CardType {
    /// Converts a CardType variant into constants suitable for use with
    /// `pafe_sys` functions.
    pub fn to_sys(&self) -> u16 {
        match self {
            Self::Any => pafe_sys::FELICA_POLLING_ANY,
            Self::Edy => pafe_sys::FELICA_POLLING_EDY,
            Self::Suica => pafe_sys::FELICA_POLLING_SUICA,
        }
    }
}

/// Represents an individual PaSoRi reader.
/// This can be created by calling Pasori::create().
/// Methods on this struct will interact with the same PaSoRi reader it was created from.
pub struct Pasori {
    pointer: *mut pafe_sys::Pasori,
}

impl Pasori {
    /// Creates a handle representing a specific PaSoRi reader.
    /// Due to a limitation in the underlying library, it's not possible
    /// to choose a specific reader if more than one is attached to the
    /// computer.
    /// The reader won't yet be initialized after calling this; make sure
    /// to call `.init()` before calling any methods.
    pub fn create() -> Pasori {
        let pasori;
        unsafe {
            pasori = pafe_sys::pasori_open();
        }

        Pasori { pointer: pasori }
    }

    /// Attempts to initialize the PaSoRi represented by this struct.
    /// Returns Err if it couldn't be initialized; otherwise it returns
    /// self, which is suitable for chaining.
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

    /// Attempts to read a card within NFC distance. If successful, returns
    /// a [FelicaTag](FelicaTag) containing the card's data.
    /// This method attempts to read the card immediately, then returns;
    /// it won't poll continuously. Most likely, you'll want to call this
    /// method in a loop.
    ///
    /// `card_type` allows you to specify what kinds of card to read.
    /// For example, if `CardType::Suica` is passed, an Edy will be ignored
    /// if it's tapped.
    /// The `timeslot` parameter affects the number of cards that can be
    /// read at once; for more information, see [Timeslot](Timeslot) and
    /// the FeliCa specification.
    pub fn poll(&self, card_type: CardType, timeslot: Timeslot) -> Option<FelicaTag> {
        let card_type_raw = card_type.to_sys();
        let pointer;
        let tag;
        unsafe {
            // According to libpafe, RFU, the third parameter, is always 0.
            // It's probably safe to just hardcode it here.
            // npasoriv does the same.
            pointer = pafe_sys::felica_polling(self.pointer, card_type_raw, 0, timeslot.as_u8());
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
