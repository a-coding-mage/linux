// SPDX-License-Identifier: GPL-2.0
/*
 * MIPS-specific debug support for pre-boot environment
 *
 * NOTE: putc() is board specific, if your board have a 16550 compatible uart,
 * please select SYS_SUPPORTS_ZBOOT_UART16550 for your machine. otherwise, you
 * need to implement your own putc().
 */

// Dependencies supplied by the original Linux sources:
// #include <linux/compiler.h>
// #include <linux/types.h>
// #include "decompress.h"

/// Weak board-specific character output hook.
#[no_mangle]
pub extern "C" fn putc(_c: u8) {
}

#[no_mangle]
pub unsafe extern "C" fn puts(mut s: *const u8) {
    let mut c: u8;
    loop {
        c = *s;
        s = s.add(1);
        if c == 0 {
            break;
        }
        putc(c);
        if c == b'\n' {
            putc(b'\r');
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn puthex(mut val: u64) {
    let mut buf = [0u8; 10];
    let digits = b"0123456789ABCDEF";
    let mut i: i32 = 7;
    while i >= 0 {
        buf[i as usize] = digits[(val & 0x0F) as usize];
        val >>= 4;
        i -= 1;
    }
    buf[8] = 0;
    puts(buf.as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
