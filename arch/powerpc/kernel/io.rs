// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * I/O string operations
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *    Copyright (C) 2006 IBM Corporation
 *
 * Largely rewritten by Cort Dougan (cort@cs.nmt.edu)
 * and Paul Mackerras.
 *
 * Adapted for iSeries by Mike Corrigan (mikejc@us.ibm.com)
 * PPC64 updates by Dave Engebretsen (engebret@us.ibm.com)
 *
 * Rewritten in C by Stephen Rothwell.
 */

/* See definition in io.h */
pub static mut isa_io_special: bool = false;

pub unsafe fn _insb(port: *const u8, buf: *mut u8, mut count: isize) {
    if count <= 0 { return; }
    mb();
    let mut tbuf = buf;
    let mut tmp: u8;
    loop {
        tmp = core::ptr::read_volatile(port as *const u8);
        eieio();
        core::ptr::write(tbuf, tmp);
        tbuf = tbuf.add(1);
        count -= 1;
        if count == 0 { break; }
    }
    data_barrier(tmp);
}

pub unsafe fn _outsb(port: *mut u8, buf: *const u8, mut count: isize) {
    if count <= 0 { return; }
    mb();
    let mut tbuf = buf;
    loop {
        core::ptr::write_volatile(port as *mut u8, core::ptr::read(tbuf));
        tbuf = tbuf.add(1);
        count -= 1;
        if count == 0 { break; }
    }
    mb();
}

pub unsafe fn _insw(port: *const u16, buf: *mut u16, mut count: isize) {
    if count <= 0 { return; }
    mb();
    let mut tbuf = buf;
    let mut tmp: u16;
    loop {
        tmp = core::ptr::read_volatile(port as *const u16);
        eieio();
        core::ptr::write(tbuf, tmp);
        tbuf = tbuf.add(1);
        count -= 1;
        if count == 0 { break; }
    }
    data_barrier(tmp);
}

pub unsafe fn _outsw(port: *mut u16, buf: *const u16, mut count: isize) {
    if count <= 0 { return; }
    mb();
    let mut tbuf = buf;
    loop {
        core::ptr::write_volatile(port as *mut u16, core::ptr::read(tbuf));
        tbuf = tbuf.add(1);
        count -= 1;
        if count == 0 { break; }
    }
    mb();
}

pub unsafe fn _insl(port: *const u32, buf: *mut u32, mut count: isize) {
    if count <= 0 { return; }
    mb();
    let mut tbuf = buf;
    let mut tmp: u32;
    loop {
        tmp = core::ptr::read_volatile(port as *const u32);
        eieio();
        core::ptr::write(tbuf, tmp);
        tbuf = tbuf.add(1);
        count -= 1;
        if count == 0 { break; }
    }
    data_barrier(tmp);
}

pub unsafe fn _outsl(port: *mut u32, buf: *const u32, mut count: isize) {
    if count <= 0 { return; }
    mb();
    let mut tbuf = buf;
    loop {
        core::ptr::write_volatile(port as *mut u32, core::ptr::read(tbuf));
        tbuf = tbuf.add(1);
        count -= 1;
        if count == 0 { break; }
    }
    mb();
}

#[inline]
unsafe fn io_check_align(v: *const u8, a: usize) -> bool {
    (v as usize & (a - 1)) == 0
}

pub unsafe fn _memset_io(addr: *mut u8, c: i32, mut n: usize) {
    let mut p = addr as *mut u8;
    let mut lc = c as u32;
    lc |= lc << 8;
    lc |= lc << 16;
    mb();
    while n != 0 && !io_check_align(p, 4) {
        core::ptr::write_volatile(p, c as u8);
        p = p.add(1); n -= 1;
    }
    while n >= 4 {
        core::ptr::write_volatile(p as *mut u32, lc);
        p = p.add(4); n -= 4;
    }
    while n != 0 {
        core::ptr::write_volatile(p, c as u8);
        p = p.add(1); n -= 1;
    }
    mb();
}

pub unsafe fn _memcpy_fromio(mut dest: *mut u8, src: *const u8, mut n: usize) {
    let mut vsrc = src as *const u8;
    mb();
    while n != 0 && (!io_check_align(vsrc, 4) || !io_check_align(dest, 4)) {
        *dest = core::ptr::read_volatile(vsrc); eieio();
        vsrc = vsrc.add(1); dest = dest.add(1); n -= 1;
    }
    while n >= 4 {
        core::ptr::write(dest as *mut u32, core::ptr::read_volatile(vsrc as *const u32)); eieio();
        vsrc = vsrc.add(4); dest = dest.add(4); n -= 4;
    }
    while n != 0 {
        *dest = core::ptr::read_volatile(vsrc); eieio();
        vsrc = vsrc.add(1); dest = dest.add(1); n -= 1;
    }
    mb();
}

pub unsafe fn _memcpy_toio(mut dest: *mut u8, mut src: *const u8, mut n: usize) {
    let mut vdest = dest as *mut u8;
    mb();
    while n != 0 && (!io_check_align(vdest, 4) || !io_check_align(src, 4)) {
        core::ptr::write_volatile(vdest, *src);
        src = src.add(1); vdest = vdest.add(1); n -= 1;
    }
    while n >= 4 {
        core::ptr::write_volatile(vdest as *mut u32, core::ptr::read(src as *const u32));
        src = src.add(4); vdest = vdest.add(4); n -= 4;
    }
    while n != 0 {
        core::ptr::write_volatile(vdest, *src);
        src = src.add(1); vdest = vdest.add(1); n -= 1;
    }
    mb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
