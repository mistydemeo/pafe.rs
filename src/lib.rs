use pafe_sys;

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
}

impl Drop for Pasori {
    fn drop(&mut self) {
        unsafe {
            pafe_sys::pasori_close(self.pointer)
        }
    }
}
