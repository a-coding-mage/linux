// SPDX-License-Identifier: GPL-2.0
/*
 * PeeCeeI.c: The emerging standard...
 *
 * Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
 */

unsafe extern "C" {
    fn __raw_writeb(value: u8, addr: *mut u8);
    fn __raw_writew(value: u16, addr: *mut u8);
    fn __raw_writel(value: u32, addr: *mut u8);
    fn __raw_readb(addr: *mut u8) -> u8;
    fn __raw_readw(addr: *mut u8) -> u16;
    fn __raw_readl(addr: *mut u8) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn outsb(__addr: usize, src: *const core::ffi::c_void, mut count: usize) {
    let addr = __addr as *mut u8;
    let mut p = src as *const u8;

    while count != 0 {
        count -= 1;
        __raw_writeb(*p, addr);
        p = p.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn outsw(__addr: usize, mut src: *const core::ffi::c_void, mut count: usize) {
    let addr = __addr as *mut u8;

    while count != 0 {
        count -= 1;
        __raw_writew(*(src as *const u16), addr);
        src = (src as *const u8).add(core::mem::size_of::<u16>()) as *const core::ffi::c_void;
    }
}

#[no_mangle]
pub unsafe extern "C" fn outsl(__addr: usize, mut src: *const core::ffi::c_void, mut count: usize) {
    let addr = __addr as *mut u8;
    let mut l: u32;
    let mut l2: u32;

    if count == 0 { return; }

    match (src as usize) & 0x3 {
        0x0 => {
            while count != 0 {
                count -= 1;
                __raw_writel(*(src as *const u32), addr);
                src = (src as *const u8).add(core::mem::size_of::<u32>()) as *const core::ffi::c_void;
            }
        }
        0x2 => {
            while count != 0 {
                count -= 1;
                l = (*(src as *const u16) as u32) << 16;
                l |= *((src as *const u8).add(core::mem::size_of::<u16>()) as *const u16) as u32;
                __raw_writel(l, addr);
                src = (src as *const u8).add(core::mem::size_of::<u32>()) as *const core::ffi::c_void;
            }
        }
        0x1 => {
            l = (*(src as *const u8) as u32) << 24;
            l |= (*((src as *const u8).add(1) as *const u16) as u32) << 8;
            src = (src as *const u8).add(3) as *const core::ffi::c_void;
            while count != 0 {
                count -= 1;
                l2 = *(src as *const u32);
                l |= l2 >> 24;
                __raw_writel(l, addr);
                l = l2 << 8;
                src = (src as *const u8).add(core::mem::size_of::<u32>()) as *const core::ffi::c_void;
            }
        }
        0x3 => {
            l = (*(src as *const u8) as u32) << 24;
            src = (src as *const u8).add(1) as *const core::ffi::c_void;
            while count != 0 {
                count -= 1;
                l2 = *(src as *const u32);
                l |= l2 >> 8;
                __raw_writel(l, addr);
                l = l2 << 24;
                src = (src as *const u8).add(core::mem::size_of::<u32>()) as *const core::ffi::c_void;
            }
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn insb(__addr: usize, dst: *mut core::ffi::c_void, mut count: usize) {
    let addr = __addr as *mut u8;
    if count != 0 {
        let mut pb = dst as *mut u8;
        while (pb as usize) & 0x3 != 0 && count != 0 {
            count -= 1; *pb = __raw_readb(addr); pb = pb.add(1);
        }
        let mut pi = pb as *mut u32;
        while count >= 4 {
            let w = ((__raw_readb(addr) as u32) << 24) |
                ((__raw_readb(addr) as u32) << 16) |
                ((__raw_readb(addr) as u32) << 8) | (__raw_readb(addr) as u32);
            *pi = w; pi = pi.add(1); count -= 4;
        }
        pb = pi as *mut u8;
        while count != 0 { count -= 1; *pb = __raw_readb(addr); pb = pb.add(1); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn insw(__addr: usize, dst: *mut core::ffi::c_void, mut count: usize) {
    let addr = __addr as *mut u8;
    if count != 0 {
        let mut ps = dst as *mut u16;
        if (ps as usize) & 0x2 != 0 { *ps = __raw_readw(addr); ps = ps.add(1); count -= 1; }
        let mut pi = ps as *mut u32;
        while count >= 2 {
            *pi = ((__raw_readw(addr) as u32) << 16) | (__raw_readw(addr) as u32);
            pi = pi.add(1); count -= 2;
        }
        ps = pi as *mut u16;
        if count != 0 { *ps = __raw_readw(addr); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn insl(__addr: usize, dst: *mut core::ffi::c_void, mut count: usize) {
    let addr = __addr as *mut u8;
    if count != 0 {
        if (dst as usize) & 0x3 == 0 {
            let mut pi = dst as *mut u32;
            while count != 0 { *pi = __raw_readl(addr); pi = pi.add(1); count -= 1; }
        } else {
            let mut l: u32 = 0;
            let mut l2: u32;
            match (dst as usize) & 3 {
                0x2 => {
                    let mut ps = dst as *mut u16; count -= 1; l = __raw_readl(addr); *ps = l as u16; ps = ps.add(1);
                    let mut pi = ps as *mut u32;
                    while count != 0 { l2 = __raw_readl(addr); *pi = (l << 16) | (l2 >> 16); pi = pi.add(1); l = l2; count -= 1; }
                    *(pi as *mut u16) = l as u16;
                }
                0x1 => {
                    let mut pb = dst as *mut u8; count -= 1; l = __raw_readl(addr); *pb = (l >> 24) as u8; pb = pb.add(1);
                    let mut ps = pb as *mut u16; *ps = ((l >> 8) & 0xffff) as u16; ps = ps.add(1);
                    let mut pi = ps as *mut u32;
                    while count != 0 { l2 = __raw_readl(addr); *pi = (l << 24) | (l2 >> 8); pi = pi.add(1); l = l2; count -= 1; }
                    *(pi as *mut u8) = l as u8;
                }
                0x3 => {
                    let mut pb = dst as *mut u8; count -= 1; l = __raw_readl(addr); *pb = (l >> 24) as u8; pb = pb.add(1);
                    let mut pi = pb as *mut u32;
                    while count != 0 { l2 = __raw_readl(addr); *pi = (l << 8) | (l2 >> 24); pi = pi.add(1); l = l2; count -= 1; }
                    let mut ps = pi as *mut u16; *ps = ((l >> 8) & 0xffff) as u16; ps = ps.add(1); *(ps as *mut u8) = l as u8;
                }
                _ => {}
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
