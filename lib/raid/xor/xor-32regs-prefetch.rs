// SPDX-License-Identifier: GPL-2.0-or-later
// C dependencies: <linux/prefetch.h> and "xor_impl.h"

use core::ffi::{c_long, c_ulong};

unsafe extern "C" {
    fn prefetchw(address: *mut c_ulong);
    fn prefetch(address: *const c_ulong);
}

unsafe fn xor_32regs_p_2(mut bytes: c_ulong, mut p1: *mut c_ulong, mut p2: *const c_ulong) {
    let mut lines = (bytes / (core::mem::size_of::<c_long>() as c_ulong) / 8).wrapping_sub(1) as c_long;
    unsafe { prefetchw(p1); prefetch(p2); }
    loop {
        unsafe { prefetchw(p1.add(8)); prefetch(p2.add(8)); }
        'once_more: loop {
            let (mut d0, mut d1, mut d2, mut d3, mut d4, mut d5, mut d6, mut d7);
            unsafe {
                d0 = *p1.add(0); d1 = *p1.add(1); d2 = *p1.add(2); d3 = *p1.add(3);
                d4 = *p1.add(4); d5 = *p1.add(5); d6 = *p1.add(6); d7 = *p1.add(7);
                d0 ^= *p2.add(0); d1 ^= *p2.add(1); d2 ^= *p2.add(2); d3 ^= *p2.add(3);
                d4 ^= *p2.add(4); d5 ^= *p2.add(5); d6 ^= *p2.add(6); d7 ^= *p2.add(7);
                *p1.add(0)=d0; *p1.add(1)=d1; *p1.add(2)=d2; *p1.add(3)=d3;
                *p1.add(4)=d4; *p1.add(5)=d5; *p1.add(6)=d6; *p1.add(7)=d7;
            }
            p1 = unsafe { p1.add(8) }; p2 = unsafe { p2.add(8) };
            lines -= 1;
            if lines > 0 { break; }
            if lines == 0 { continue 'once_more; }
            return;
        }
    }
}

macro_rules! xor_fn {
    ($name:ident, $( $src:ident ),+) => {
        unsafe fn $name(mut bytes: c_ulong, mut p1: *mut c_ulong, $(mut $src: *const c_ulong),+) {
            let mut lines = (bytes / (core::mem::size_of::<c_long>() as c_ulong) / 8).wrapping_sub(1) as c_long;
            unsafe { prefetchw(p1); $(prefetch($src);)+ }
            loop {
                unsafe { prefetchw(p1.add(8)); $(prefetch($src.add(8));)+ }
                'once_more: loop {
                    let (mut d0, mut d1, mut d2, mut d3, mut d4, mut d5, mut d6, mut d7);
                    unsafe {
                        d0=*p1.add(0); d1=*p1.add(1); d2=*p1.add(2); d3=*p1.add(3);
                        d4=*p1.add(4); d5=*p1.add(5); d6=*p1.add(6); d7=*p1.add(7);
                        $(d0^=* $src.add(0); d1^=* $src.add(1); d2^=* $src.add(2); d3^=* $src.add(3);
                          d4^=* $src.add(4); d5^=* $src.add(5); d6^=* $src.add(6); d7^=* $src.add(7);)+
                        *p1.add(0)=d0; *p1.add(1)=d1; *p1.add(2)=d2; *p1.add(3)=d3;
                        *p1.add(4)=d4; *p1.add(5)=d5; *p1.add(6)=d6; *p1.add(7)=d7;
                    }
                    p1=unsafe{p1.add(8)}; $( $src=unsafe{$src.add(8)}; )+
                    lines-=1; if lines>0 { break; } if lines==0 { continue 'once_more; } return;
                }
            }
        }
    }
}

xor_fn!(xor_32regs_p_3, p2, p3);
xor_fn!(xor_32regs_p_4, p2, p3, p4);
xor_fn!(xor_32regs_p_5, p2, p3, p4, p5);

// DO_XOR_BLOCKS(32regs_p, xor_32regs_p_2, xor_32regs_p_3, xor_32regs_p_4, xor_32regs_p_5);
// This external macro supplies xor_gen_32regs_p and xor_block_template.
unsafe extern "C" {
    fn xor_gen_32regs_p();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
