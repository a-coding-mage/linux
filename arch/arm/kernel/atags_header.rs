/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

#[repr(C)]
pub struct tag {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine_desc {
    _private: [u8; 0],
}

extern "C" {
    pub fn convert_to_tag_list(tags: *mut tag);
}

#[cfg(CONFIG_ATAGS)]
extern "C" {
    pub fn setup_machine_tags(
        __atags_vaddr: *mut c_void,
        machine_nr: u32,
    ) -> *const machine_desc;
}

#[cfg(not(CONFIG_ATAGS))]
pub unsafe fn setup_machine_tags(
    __atags_vaddr: *mut c_void,
    machine_nr: u32,
) -> *const machine_desc {
    let _ = (__atags_vaddr, machine_nr);
    early_print(b"no ATAGS support: can't continue\n\0".as_ptr() as *const i8);
    loop {}
    // C source marks this point unreachable after the infinite loop.
    #[allow(unreachable_code)]
    {
        core::hint::unreachable_unchecked()
    }
}

extern "C" {
    fn early_print(format: *const i8, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
