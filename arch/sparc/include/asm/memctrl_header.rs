/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _SPARC_MEMCTRL_H

use core::ffi::{c_char, c_int, c_ulong};

pub type dimm_printer_t = Option<unsafe extern "C" fn(
    synd_code: c_int,
    paddr: c_ulong,
    buf: *mut c_char,
    buflen: c_int,
) -> c_int>;

unsafe extern "C" {
    pub fn register_dimm_printer(func: dimm_printer_t) -> c_int;
    pub fn unregister_dimm_printer(func: dimm_printer_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
