use std::os::raw::{c_ulong, c_void};

pub struct CallbackData<'a>(pub &'a mut dyn FnMut(usize));

impl CallbackData<'_> {
    pub unsafe extern "C" fn callback(size: c_ulong, data: *mut c_void) {
        let data = &mut *(data as *mut CallbackData);
        data.0(size as usize);
    }
    pub fn as_mut_void(&mut self) -> *mut c_void {
        self as *mut CallbackData as *mut c_void
    }
}
