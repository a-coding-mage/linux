// SPDX-License-Identifier: GPL-2.0
/*
 * SDK7786 FPGA SRAM Support.
 *
 * Copyright (C) 2010  Paul Mundt
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/kernel.h, linux/types.h, linux/io.h, linux/string.h,
// mach/fpga.h, asm/sram.h, and linux/sizes.h.

use core::ffi::{c_int, c_ulong, c_void};

extern "C" {
    static mut sram_pool: *mut gen_pool;

    fn fpga_read_reg(reg: c_int) -> u16;
    fn fpga_write_reg(data: u16, reg: c_int);
    fn ioremap(phys: c_ulong, size: c_ulong) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn gen_pool_add(
        pool: *mut gen_pool,
        addr: c_ulong,
        size: c_ulong,
        nid: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct gen_pool {
    _private: [u8; 0],
}

// These register fields and size constants are supplied by the included
// platform headers and retain their original C names here.

unsafe fn fpga_sram_init() -> c_int {
    let mut phys: c_ulong;
    let area: u32;
    let vaddr: *mut c_void;
    let ret: c_int;
    let mut data: u16;

    /* Enable FPGA SRAM */
    data = fpga_read_reg(LCLASR);
    data |= LCLASR_FRAMEN as u16;
    fpga_write_reg(data, LCLASR);

    /*
     * FPGA_SEL determines the area mapping
     */
    area = ((data as u32 & LCLASR_FPGA_SEL_MASK as u32)
        >> LCLASR_FPGA_SEL_SHIFT) as u32;
    if area == LCLASR_AREA_MASK as u32 {
        pr_err!("FPGA memory unmapped.\n");
        return -ENXIO;
    }

    /*
     * The memory itself occupies a 2KiB range at the top of the area
     * immediately below the system registers.
     */
    phys = ((area as c_ulong) << 26) + SZ_64M as c_ulong - SZ_4K as c_ulong;

    /*
     * The FPGA SRAM resides in translatable physical space, so set
     * up a mapping prior to inserting it in to the pool.
     */
    vaddr = ioremap(phys, SZ_2K as c_ulong);
    if vaddr.is_null() {
        pr_err!("Failed remapping FPGA memory.\n");
        return -ENXIO;
    }

    pr_info!(
        "Adding {}KiB of FPGA memory at 0x{:08x}-0x{:08x} (area {}) to pool.\n",
        SZ_2K >> 10,
        phys,
        phys + SZ_2K as c_ulong - 1,
        area
    );

    ret = gen_pool_add(
        sram_pool,
        vaddr as c_ulong,
        SZ_2K as c_ulong,
        -1,
    );
    if ret < 0 {
        pr_err!("Failed adding memory\n");
        iounmap(vaddr);
        return ret;
    }

    0
}

// Equivalent of: postcore_initcall(fpga_sram_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
