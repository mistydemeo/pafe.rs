use pafe_sys;

pub struct FelicaTag {
    pub tag: pafe_sys::Felica,
}

impl FelicaTag {
    pub fn manufacturer_code(&self) -> [u8; 2] {
        [self.tag.IDm[0], self.tag.IDm[1]].clone()
    }

    pub fn card_id(&self) -> &[u8] {
        &self.tag.IDm[2..7]
    }
}
