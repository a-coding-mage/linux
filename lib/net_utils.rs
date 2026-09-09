// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the included Linux headers are supplied externally.

use core::ffi::{c_char, c_int};

const ETH_ALEN: usize = 6;
const MAC_ADDR_STR_LEN: usize = 17;

unsafe extern "C" {
    fn strnlen(s: *const c_char, maxlen: usize) -> usize;
    fn isxdigit(c: c_int) -> c_int;
    fn hex_to_bin(c: u8) -> c_int;
}

pub unsafe fn mac_pton(s: *const c_char, mac: *mut u8) -> bool {
    let mut i: c_int;

    if strnlen(s, MAC_ADDR_STR_LEN) < MAC_ADDR_STR_LEN {
        return false;
    }

    /* Don't dirty result unless string is valid MAC. */
    i = 0;
    while i < ETH_ALEN as c_int {
        if isxdigit(*s.add((i * 3) as usize) as c_int) == 0
            || isxdigit(*s.add((i * 3 + 1) as usize) as c_int) == 0
        {
            return false;
        }
        if i != ETH_ALEN as c_int - 1
            && *s.add((i * 3 + 2) as usize) as u8 != b':'
        {
            return false;
        }
        i += 1;
    }
    i = 0;
    while i < ETH_ALEN as c_int {
        *mac.add(i as usize) = ((hex_to_bin(*s.add((i * 3) as usize) as u8) << 4)
            | hex_to_bin(*s.add((i * 3 + 1) as usize) as u8)) as u8;
        i += 1;
    }
    true
}

// EXPORT_SYMBOL(mac_pton);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
