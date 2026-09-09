// SPDX-License-Identifier: GPL-2.0
/*
 * SDK7786 FPGA Support.
 *
 * Copyright (C) 2010  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/io.h, linux/bcd.h, mach/fpga.h, linux/sizes.h

const FPGA_REGS_OFFSET: usize = 0x03fff800;
const FPGA_REGS_SIZE: usize = 0x490;

/*
 * The FPGA can be mapped in any of the generally available areas,
 * so we attempt to scan for it using the fixed SRSTR read magic.
 *
 * Once the FPGA is located, the rest of the mapping data for the other
 * components can be determined dynamically from its section mapping
 * registers.
 */
unsafe fn sdk7786_fpga_probe() -> *mut core::ffi::c_void {
    let mut area: usize;
    let mut base: *mut core::ffi::c_void;

    /*
     * Iterate over all of the areas where the FPGA could be mapped.
     * The possible range is anywhere from area 0 through 6, area 7
     * is reserved.
     */
    area = PA_AREA0 as usize;
    while area < PA_AREA7 as usize {
        base = ioremap(
            area.wrapping_add(FPGA_REGS_OFFSET),
            FPGA_REGS_SIZE,
        );
        if base.is_null() {
            /* Failed to remap this area, move along. */
            area = area.wrapping_add(SZ_64M as usize);
            continue;
        }

        if ioread16((base as *mut u8).add(SRSTR as usize)) == SRSTR_MAGIC {
            return base; /* Found it! */
        }

        iounmap(base);
        area = area.wrapping_add(SZ_64M as usize);
    }

    core::ptr::null_mut()
}

pub static mut sdk7786_fpga_base: *mut core::ffi::c_void = core::ptr::null_mut();

pub unsafe fn sdk7786_fpga_init() {
    let version: u16;
    let date: u16;

    sdk7786_fpga_base = sdk7786_fpga_probe();
    if sdk7786_fpga_base.is_null() {
        panic("FPGA detection failed.\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    version = fpga_read_reg(FPGAVR);
    date = fpga_read_reg(FPGADR);

    pr_info(
        "\tFPGA version:\t%d.%d (built on %d/%d/%d)\n\0".as_ptr()
            as *const core::ffi::c_char,
        bcd2bin(version >> 8) & 0xf,
        bcd2bin(version & 0xf),
        ((date >> 12) & 0xf) + 2000,
        (date >> 8) & 0xf,
        bcd2bin(date & 0xff),
    );
}

extern "C" {
    static PA_AREA0: usize;
    static PA_AREA7: usize;
    static SZ_64M: usize;
    static SRSTR: usize;
    static SRSTR_MAGIC: u16;
    static FPGAVR: usize;
    static FPGADR: usize;

    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn ioread16(addr: *mut u8) -> u16;
    fn fpga_read_reg(reg: usize) -> u16;
    fn bcd2bin(value: u16) -> u16;
    fn panic(message: *const core::ffi::c_char) -> !;
    fn pr_info(format: *const core::ffi::c_char, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
