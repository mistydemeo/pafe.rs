use pafe_sys;

/// Represents data read from a FeliCa card.
/// While this includes a few helper methods, most consumers will
/// want to read the data directly from the underlying [pafe_sys::tag_felica](pafe_sys::tag_felica) struct.
pub struct FelicaTag {
    /// The raw pafe_sys::tag_felica](pafe_sys::tag_felica) struct
    /// returned by libpafe. Its members are exactly the same as in
    /// the C library. For more details, please consult [libpafe](https://github.com/rfujita/libpafe).
    pub tag: pafe_sys::Felica,
}

impl FelicaTag {
    /// Extracts just the manufacturer code from the card's IDm.
    /// The IDm is eight bytes; the manufacturer code is the first two bytes.
    pub fn manufacturer_code(&self) -> [u8; 2] {
        [self.tag.IDm[0], self.tag.IDm[1]].clone()
    }

    /// Extracts just the card ID from the card's IDm.
    /// The IDm is eight bytes; the card ID is the last six bytes.
    pub fn card_id(&self) -> &[u8] {
        &self.tag.IDm[2..7]
    }
}
