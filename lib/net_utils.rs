// SPDX-License-Identifier: GPL-2.0

// Dependencies: linux/string.h, linux/if_ether.h, linux/ctype.h, linux/hex.h.

use kernel::bindings::{_ctype, strnlen, ETH_ALEN, MAC_ADDR_STR_LEN};
use kernel::ffi::{c_char, c_int};

extern "C" {
    fn hex_to_bin(ch: u8) -> c_int;
}

/// `_D` and `_X` from linux/ctype.h; lib/net_utils_helpers.c asserts them.
const CTYPE_D: u8 = 0x04;
const CTYPE_X: u8 = 0x40;

/// isxdigit(): `(__ismask(c) & (_D | _X)) != 0` on the shared _ctype table.
fn isxdigit(c: c_char) -> bool {
    // SAFETY: the C declaration is an incomplete array; the definition has
    // 256 entries indexed by the unsigned char value.
    unsafe { *core::ptr::addr_of!(_ctype).cast::<u8>().add(usize::from(c)) & (CTYPE_D | CTYPE_X) != 0 }
}

/// Parses a "xx:xx:xx:xx:xx:xx" MAC address into `mac`.
///
/// # Safety
/// `s` is a NUL-terminated string or holds at least MAC_ADDR_STR_LEN
/// characters; `mac` has room for ETH_ALEN bytes.
#[no_mangle]
pub unsafe extern "C" fn mac_pton(s: *const c_char, mac: *mut u8) -> bool {
    let alen = ETH_ALEN as usize;

    // SAFETY: strnlen() bounds every later index below MAC_ADDR_STR_LEN.
    unsafe {
        if strnlen(s, MAC_ADDR_STR_LEN as usize) < MAC_ADDR_STR_LEN as usize {
            return false;
        }

        /* Don't dirty result unless string is valid MAC. */
        for i in 0..alen {
            if !isxdigit(*s.add(i * 3)) || !isxdigit(*s.add(i * 3 + 1)) {
                return false;
            }
            if i != alen - 1 && *s.add(i * 3 + 2) != b':' {
                return false;
            }
        }
        for i in 0..alen {
            *mac.add(i) = ((hex_to_bin(*s.add(i * 3)) << 4) | hex_to_bin(*s.add(i * 3 + 1))) as u8;
        }
    }
    true
}
// EXPORT_SYMBOL(mac_pton);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
