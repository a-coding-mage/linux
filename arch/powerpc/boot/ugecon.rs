// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/boot/ugecon.c
 *
 * USB Gecko bootwrapper console.
 * Copyright (C) 2008-2009 The GameCube Linux Team
 * Copyright (C) 2008,2009 Albert Herranz
 */

// Dependencies supplied by the surrounding bootwrapper implementation:
// stddef.h, stdio.h, types.h, io.h, and ops.h.

const EXI_CLK_32MHZ: u32 = 5;

const EXI_CSR: usize = 0x00;
const EXI_CSR_CLKMASK: u32 = 0x7 << 4;
const EXI_CSR_CLK_32MHZ: u32 = EXI_CLK_32MHZ << 4;
const EXI_CSR_CSMASK: u32 = 0x7 << 7;
const EXI_CSR_CS_0: u32 = 0x1 << 7; // Chip Select 001

const EXI_CR: usize = 0x0c;
const EXI_CR_TSTART: u32 = 1 << 0;
const EXI_CR_WRITE: u32 = 1 << 2;
const EXI_CR_READ_WRITE: u32 = 2 << 2;
const EXI_DATA: usize = 0x10;

extern "C" {
    fn out_be32(addr: *mut u32, value: u32);
    fn in_be32(addr: *const u32) -> u32;
    fn barrier();
    fn find_node_by_compatible(parent: *mut core::ffi::c_void, compatible: *const i8) -> *mut core::ffi::c_void;
    fn getprop(node: *mut core::ffi::c_void, name: *const i8, value: *mut u32, len: usize) -> isize;
}

/* virtual address base for input/output, retrieved from device tree */
static mut ug_io_base: *mut u8 = core::ptr::null_mut();

#[inline]
unsafe fn ug_io_transaction(input: u32) -> u32 {
    let csr_reg = ug_io_base.add(EXI_CSR) as *mut u32;
    let data_reg = ug_io_base.add(EXI_DATA) as *mut u32;
    let cr_reg = ug_io_base.add(EXI_CR) as *mut u32;
    let csr: u32;
    let data: u32;
    let cr: u32;

    /* select */
    csr = EXI_CSR_CLK_32MHZ | EXI_CSR_CS_0;
    out_be32(csr_reg, csr);

    /* read/write */
    data = input;
    out_be32(data_reg, data);
    cr = ((2 - 1) << 4) | EXI_CR_READ_WRITE | EXI_CR_TSTART;
    out_be32(cr_reg, cr);

    while (in_be32(cr_reg) & EXI_CR_TSTART) != 0 {
        barrier();
    }

    /* deselect */
    out_be32(csr_reg, 0);

    data = in_be32(data_reg);
    data
}

#[inline]
unsafe fn ug_is_txfifo_ready() -> i32 {
    (ug_io_transaction(0xc0000000) & 0x04000000) as i32
}

#[inline]
unsafe fn ug_raw_putc(ch: i8) {
    ug_io_transaction(0xb0000000 | ((ch as u32) << 20));
}

#[inline]
unsafe fn ug_putc(ch: i8) {
    let mut count = 16;

    if ug_io_base.is_null() {
        return;
    }

    while ug_is_txfifo_ready() == 0 && count != 0 {
        count -= 1;
        barrier();
    }
    if count >= 0 {
        ug_raw_putc(ch);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ug_console_write(buf: *const i8, mut len: i32) {
    let mut b = buf;

    while len != 0 {
        if *b == b'\n' as i8 {
            ug_putc(b'\r' as i8);
        }
        ug_putc(*b);
        b = b.add(1);
        len -= 1;
    }
}

#[inline]
unsafe fn ug_is_adapter_present() -> i32 {
    if ug_io_base.is_null() {
        return 0;
    }
    (ug_io_transaction(0x90000000) == 0x04700000) as i32
}

unsafe fn ug_grab_exi_io_base() -> *mut u8 {
    let mut v: u32 = 0;
    let devp = find_node_by_compatible(core::ptr::null_mut(), b"nintendo,flipper-exi\0".as_ptr() as *const i8);
    if devp.is_null() {
        return core::ptr::null_mut();
    }
    if getprop(devp, b"virtual-reg\0".as_ptr() as *const i8, &mut v, core::mem::size_of::<u32>())
        != core::mem::size_of::<u32>() as isize
    {
        return core::ptr::null_mut();
    }

    v as usize as *mut u8
}

#[no_mangle]
pub unsafe extern "C" fn ug_probe() -> *mut u8 {
    let exi_io_base = ug_grab_exi_io_base();
    if exi_io_base.is_null() {
        return core::ptr::null_mut();
    }

    /* look for a usbgecko on memcard slots A and B */
    let mut i = 0;
    while i < 2 {
        ug_io_base = exi_io_base.add(0x14 * i);
        if ug_is_adapter_present() != 0 {
            break;
        }
        i += 1;
    }
    if i == 2 {
        ug_io_base = core::ptr::null_mut();
    }
    ug_io_base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
