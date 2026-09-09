// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// C dependencies:
// linux/memblock.h, asm/bootinfo.h, asm/traps.h, asm/smp-ops.h,
// asm/cacheflush.h, asm/fw/fw.h, and loongson.h

use core::ffi::c_void;

extern "C" {
    static except_vec_nmi: *const u8;
    static mut board_nmi_handler_setup: Option<unsafe extern "C" fn()>;

    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn flush_icache_range(start: usize, end: usize);
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn fw_init_cmdline();
    fn prom_init_machtype();
    fn prom_init_env();
    fn set_io_port_base(base: usize);
    fn prom_init_memory();
    fn prom_init_uart_base();
}

// Loongson CPU address windows config space base address
#[no_mangle]
pub static mut _loongson_addrwincfg_base: usize = 0;

unsafe extern "C" fn mips_nmi_setup() {
    let mut base: *mut c_void;

    base = (CAC_BASE + 0x380) as *mut c_void;
    memcpy(base, except_vec_nmi as *const c_void, 0x80);
    flush_icache_range(base as usize, (base as usize).wrapping_add(0x80));
}

#[no_mangle]
pub unsafe extern "C" fn prom_init() {
    // CONFIG_CPU_SUPPORTS_ADDRWINCFG controls this build-time conditional.
    #[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
    {
        _loongson_addrwincfg_base = ioremap(
            LOONGSON_ADDRWINCFG_BASE,
            LOONGSON_ADDRWINCFG_SIZE,
        ) as usize;
    }

    fw_init_cmdline();
    prom_init_machtype();
    prom_init_env();

    /* init base address of io space */
    set_io_port_base(ioremap(LOONGSON_PCIIO_BASE, LOONGSON_PCIIO_SIZE) as usize);
    prom_init_memory();

    /*init the uart base address */
    prom_init_uart_base();
    board_nmi_handler_setup = Some(mips_nmi_setup);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
