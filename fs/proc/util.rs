// Dependency intent from <linux/dcache.h> and "internal.h" is supplied by
// the surrounding translation unit.

use core::ffi::{c_char, c_uint};

use crate::qstr;

pub unsafe fn name_to_int(qstr: *const qstr) -> c_uint {
    let mut name: *const c_char = (*qstr).name;
    let mut len = (*qstr).len;
    let mut n: c_uint = 0;

    if len > 1 && *name == b'0' as c_char {
        return !0_u32 as c_uint;
    }

    loop {
        let c = (*name as u8).wrapping_sub(b'0') as c_uint;
        name = name.add(1);
        if c > 9 {
            return !0_u32 as c_uint;
        }
        if n >= (!0_u32 as c_uint).wrapping_sub(9) / 10 {
            return !0_u32 as c_uint;
        }
        n = n.wrapping_mul(10);
        n = n.wrapping_add(c);
        len = len.wrapping_sub(1);
        if len == 0 {
            break;
        }
    }
    n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
