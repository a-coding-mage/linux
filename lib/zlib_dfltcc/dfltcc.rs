// SPDX-License-Identifier: Zlib
/* dfltcc.c - SystemZ DEFLATE CONVERSION CALL support. */

// Dependencies supplied by the surrounding translation unit:
// dfltcc_util.h, dfltcc.h, and the Linux export/module facilities.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn is_dfltcc_enabled() -> c_int;
    fn dfltcc(
        function_code: c_int,
        param: *mut dfltcc_param,
        next_in: *mut c_void,
        avail_in: *mut c_void,
        next_out: *mut c_void,
        avail_out: *mut c_void,
        gs: *mut c_void,
    );
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
}

// These types and constants are declared by dfltcc.h.
#[allow(non_camel_case_types)]
pub struct dfltcc_param {
    pub nt: u8,
    pub ribm: u8,
    // Remaining fields are supplied by the native parameter-block definition.
}

#[allow(non_camel_case_types)]
pub struct dfltcc_state {
    pub param: dfltcc_param,
    pub af: dfltcc_af,
}

#[allow(non_camel_case_types)]
pub struct dfltcc_af;

extern "C" {
    static DFLTCC_QAF: c_int;
    static DFLTCC_RIBM: u8;
}

#[no_mangle]
pub unsafe extern "C" fn oesc_msg(buf: *mut c_char, oesc: c_int) -> *mut c_char {
    if oesc == 0x00 {
        core::ptr::null_mut() /* Successful completion */
    } else {
        // #ifdef STATIC: return NULL; /* Ignore for pre-boot decompressor */
        #[cfg(not(feature = "STATIC"))]
        {
            let format = b"Operation-Ending-Supplemental Code is 0x%.2X\0";
            sprintf(buf, format.as_ptr() as *const c_char, oesc);
            buf
        }
        #[cfg(feature = "STATIC")]
        {
            core::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dfltcc_reset_state(state: *mut dfltcc_state) {
    /* Initialize available functions */
    if is_dfltcc_enabled() != 0 {
        dfltcc(
            DFLTCC_QAF,
            &mut (*state).param,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        memmove(
            &mut (*state).af as *mut dfltcc_af as *mut c_void,
            &(*state).param as *const dfltcc_param as *const c_void,
            core::mem::size_of::<dfltcc_af>(),
        );
    } else {
        memset(
            &mut (*state).af as *mut dfltcc_af as *mut c_void,
            0,
            core::mem::size_of::<dfltcc_af>(),
        );
    }

    /* Initialize parameter block */
    memset(
        &mut (*state).param as *mut dfltcc_param as *mut c_void,
        0,
        core::mem::size_of::<dfltcc_param>(),
    );
    (*state).param.nt = 1;
    (*state).param.ribm = DFLTCC_RIBM;
}

// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
