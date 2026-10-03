//! Minimal bindings to libopenmpt (vendored, see `build.rs`): the MO3 module player that
//! stands in for the original's BASS music library.

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

#[repr(C)]
struct OpenmptModule {
    _p: [u8; 0],
}

#[cfg_attr(windows, link(name = "libopenmpt"))]
unsafe extern "C" {
    fn openmpt_module_create_from_memory2(
        filedata: *const c_void,
        filesize: usize,
        logfunc: *const c_void,
        loguser: *mut c_void,
        errfunc: *const c_void,
        erruser: *mut c_void,
        error: *mut c_int,
        error_message: *mut *const c_char,
        ctls: *const c_void,
    ) -> *mut OpenmptModule;
    fn openmpt_module_destroy(m: *mut OpenmptModule);
    fn openmpt_module_set_repeat_count(m: *mut OpenmptModule, repeat_count: i32) -> c_int;
    fn openmpt_module_set_position_order_row(m: *mut OpenmptModule, order: i32, row: i32) -> f64;
    fn openmpt_module_read_interleaved_float_stereo(m: *mut OpenmptModule, samplerate: i32, count: usize, out: *mut f32) -> usize;
}

/// One loaded module. libopenmpt modules may be used from any one thread at a time.
pub struct Module(*mut OpenmptModule);

// The pointer is only ever used behind the music mixer's mutex.
unsafe impl Send for Module {}

impl Module {
    /// Loads a module (MO3, IT, XM, ...) from memory; `None` when libopenmpt rejects it.
    pub fn load(bytes: &[u8]) -> Option<Module> {
        let mut err: c_int = 0;
        let m = unsafe {
            openmpt_module_create_from_memory2(
                bytes.as_ptr() as *const c_void,
                bytes.len(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut err,
                std::ptr::null_mut(),
                std::ptr::null(),
            )
        };
        if m.is_null() { None } else { Some(Module(m)) }
    }

    /// -1 loops forever (pattern jumps and the module's end restart), 0 plays once.
    pub fn set_repeat_count(&mut self, n: i32) {
        unsafe { openmpt_module_set_repeat_count(self.0, n) };
    }

    /// Jumps to row `row` of order `order`.
    pub fn set_position(&mut self, order: i32, row: i32) {
        unsafe { openmpt_module_set_position_order_row(self.0, order, row) };
    }

    /// Renders up to `out.len() / 2` stereo frames; the number rendered (0 at the end).
    pub fn read_stereo(&mut self, rate: i32, out: &mut [f32]) -> usize {
        unsafe { openmpt_module_read_interleaved_float_stereo(self.0, rate, out.len() / 2, out.as_mut_ptr()) }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { openmpt_module_destroy(self.0) };
    }
}
