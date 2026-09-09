// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/boot/wii.c
 *
 * Nintendo Wii bootwrapper support
 * Copyright (C) 2008-2009 The GameCube Linux Team
 * Copyright (C) 2008,2009 Albert Herranz
 */

use core::ffi::{c_char, c_int, c_void};

// BSS_STACK(8192);

const EXI_CTRL: *mut c_void = 0x0d800070usize as *mut c_void;
const EXI_CTRL_ENABLE: u32 = 1 << 0;

const MEM2_TOP: u32 = 0x10000000 + 64 * 1024 * 1024;
const FIRMWARE_DEFAULT_SIZE: u32 = 12 * 1024 * 1024;

#[repr(C)]
struct mipc_infohdr {
    magic: [c_char; 3],
    version: u8,
    mem2_boundary: u32,
    ipc_in: u32,
    ipc_in_size: usize,
    ipc_out: u32,
    ipc_out_size: usize,
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn finddevice(path: *const c_char) -> *mut c_void;
    fn fatal(format: *const c_char, ...);
    fn getprop(node: *mut c_void, name: *const c_char, buf: *mut c_void, buflen: usize) -> c_int;
    fn setprop(node: *mut c_void, name: *const c_char, buf: *const c_void, buflen: usize) -> c_int;
    fn in_be32(addr: *const c_void) -> u32;
    fn out_be32(addr: *mut c_void, value: u32);
    fn simple_alloc_init(start: *mut u8, size: u32, align: u32, boundary: u32);
    fn fdt_init(dtb: *mut u8);
    fn ug_probe() -> c_int;
    fn ug_console_write();

    static mut _end: u8;
    static mut _dtb_start: u8;
    static mut console_ops: ConsoleOps;
    static mut platform_ops: PlatformOps;
}

#[repr(C)]
struct ConsoleOps {
    write: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct PlatformOps {
    fixups: Option<unsafe fn()>,
}

const EINVAL: c_int = 22;

unsafe fn mipc_check_address(pa: u32) -> c_int {
    /* only MEM2 addresses */
    if pa < 0x10000000 || pa > 0x14000000 {
        return -EINVAL;
    }
    0
}

unsafe fn mipc_get_infohdr() -> *mut mipc_infohdr {
    let hdrp = 0x13fffffcusize as *mut *mut mipc_infohdr;
    let hdr: *mut mipc_infohdr;

    if mipc_check_address(hdrp as u32) != 0 {
        printf(b"mini: invalid hdrp %08X\0".as_ptr() as *const c_char, hdrp as u32);
        hdr = core::ptr::null_mut();
        return hdr;
    }

    hdr = *hdrp;
    if mipc_check_address(hdr as u32) != 0 {
        printf(b"mini: invalid hdr %08X\0".as_ptr() as *const c_char, hdr as u32);
        return core::ptr::null_mut();
    }
    if memcmp((*hdr).magic.as_ptr() as *const c_void, b"IPC", 3) != 0 {
        printf(b"mini: invalid magic\n\0".as_ptr() as *const c_char);
        return core::ptr::null_mut();
    }

    hdr
}

unsafe fn mipc_get_mem2_boundary(mem2_boundary: *mut u32) -> c_int {
    let hdr = mipc_get_infohdr();
    if hdr.is_null() {
        return -1;
    }

    if mipc_check_address((*hdr).mem2_boundary) != 0 {
        printf(
            b"mini: invalid mem2_boundary %08X\n\0".as_ptr() as *const c_char,
            (*hdr).mem2_boundary,
        );
        return -EINVAL;
    }
    *mem2_boundary = (*hdr).mem2_boundary;
    0
}

unsafe fn platform_fixups() {
    let mem = finddevice(b"/memory\0".as_ptr() as *const c_char);
    if mem.is_null() {
        fatal(b"Can't find memory node\n\0".as_ptr() as *const c_char);
    }

    /* two ranges of (address, size) words */
    let mut reg = [0u32; 4];
    let len = getprop(
        mem,
        b"reg\0".as_ptr() as *const c_char,
        reg.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&reg),
    );
    if len != core::mem::size_of_val(&reg) as c_int {
        return;
    }

    /* retrieve MEM2 boundary from 'mini' */
    let mut mem2_boundary: u32 = 0;
    if mipc_get_mem2_boundary(&mut mem2_boundary) != 0 {
        /* if that fails use a sane value */
        mem2_boundary = MEM2_TOP - FIRMWARE_DEFAULT_SIZE;
    }

    if mem2_boundary > reg[2] && mem2_boundary < reg[2].wrapping_add(reg[3]) {
        reg[3] = mem2_boundary.wrapping_sub(reg[2]);
        printf(
            b"top of MEM2 @ %08X\n\0".as_ptr() as *const c_char,
            reg[2].wrapping_add(reg[3]),
        );
        setprop(
            mem,
            b"reg\0".as_ptr() as *const c_char,
            reg.as_ptr() as *const c_void,
            core::mem::size_of_val(&reg),
        );
    }
}

pub unsafe extern "C" fn platform_init(_r3: usize, _r4: usize, _r5: usize) {
    let heapsize = 24 * 1024 * 1024u32 - (&raw mut _end as *mut u8 as u32);

    simple_alloc_init(&raw mut _end, heapsize, 32, 64);
    fdt_init(&raw mut _dtb_start);

    /*
     * 'mini' boots the Broadway processor with EXI disabled.
     * We need it enabled before probing for the USB Gecko.
     */
    out_be32(EXI_CTRL, in_be32(EXI_CTRL) | EXI_CTRL_ENABLE);

    if ug_probe() != 0 {
        console_ops.write = Some(ug_console_write);
    }

    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
