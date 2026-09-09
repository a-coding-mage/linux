// SPDX-License-Identifier: GPL-2.0
/*
 * SiFive composable cache controller Driver
 *
 * Copyright (C) 2018-2022 SiFive, Inc.
 */

// C dependency headers and kernel-provided symbols are intentionally external.

const SIFIVE_CCACHE_DIRECCFIX_LOW: usize = 0x100;
const SIFIVE_CCACHE_DIRECCFIX_HIGH: usize = 0x104;
const SIFIVE_CCACHE_DIRECCFIX_COUNT: usize = 0x108;
const SIFIVE_CCACHE_DIRECCFAIL_LOW: usize = 0x120;
const SIFIVE_CCACHE_DIRECCFAIL_HIGH: usize = 0x124;
const SIFIVE_CCACHE_DIRECCFAIL_COUNT: usize = 0x128;
const SIFIVE_CCACHE_DATECCFIX_LOW: usize = 0x140;
const SIFIVE_CCACHE_DATECCFIX_HIGH: usize = 0x144;
const SIFIVE_CCACHE_DATECCFIX_COUNT: usize = 0x148;
const SIFIVE_CCACHE_DATECCFAIL_LOW: usize = 0x160;
const SIFIVE_CCACHE_DATECCFAIL_HIGH: usize = 0x164;
const SIFIVE_CCACHE_DATECCFAIL_COUNT: usize = 0x168;
const SIFIVE_CCACHE_CONFIG: usize = 0x00;
const SIFIVE_CCACHE_CONFIG_BANK_MASK: u64 = 0xff;
const SIFIVE_CCACHE_CONFIG_WAYS_MASK: u64 = 0xff00;
const SIFIVE_CCACHE_CONFIG_SETS_MASK: u64 = 0xff0000;
const SIFIVE_CCACHE_CONFIG_BLKS_MASK: u64 = 0xff000000;
const SIFIVE_CCACHE_FLUSH64: usize = 0x200;
const SIFIVE_CCACHE_FLUSH32: usize = 0x240;
const SIFIVE_CCACHE_WAYENABLE: usize = 0x08;
const SIFIVE_CCACHE_ECCINJECTERR: usize = 0x40;
const SIFIVE_CCACHE_MAX_ECCINTR: usize = 4;
const SIFIVE_CCACHE_LINE_SIZE: usize = 64;

static mut CCACHE_BASE: *mut core::ffi::c_void = core::ptr::null_mut();
static mut G_IRQ: [i32; SIFIVE_CCACHE_MAX_ECCINTR] = [0; SIFIVE_CCACHE_MAX_ECCINTR];
static mut LEVEL: i32 = 0;

const DIR_CORR: usize = 0;
const DATA_CORR: usize = 1;
const DATA_UNCORR: usize = 2;
const DIR_UNCORR: usize = 3;
const QUIRK_NONSTANDARD_CACHE_OPS: usize = 1 << 0;
const QUIRK_BROKEN_DATA_UNCORR: usize = 1 << 1;

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn mb();
    fn panic(fmt: *const core::ffi::c_char, ... ) -> !;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn atomic_notifier_call_chain(chain: *mut core::ffi::c_void, value: u32, data: *const core::ffi::c_char);
    fn atomic_notifier_chain_register(chain: *mut core::ffi::c_void, nb: *mut core::ffi::c_void) -> i32;
    fn atomic_notifier_chain_unregister(chain: *mut core::ffi::c_void, nb: *mut core::ffi::c_void) -> i32;
}

static mut CCACHE_ERR_CHAIN: *mut core::ffi::c_void = core::ptr::null_mut();

pub unsafe fn register_sifive_ccache_error_notifier(nb: *mut core::ffi::c_void) -> i32 {
    atomic_notifier_chain_register(CCACHE_ERR_CHAIN, nb)
}

pub unsafe fn unregister_sifive_ccache_error_notifier(nb: *mut core::ffi::c_void) -> i32 {
    atomic_notifier_chain_unregister(CCACHE_ERR_CHAIN, nb)
}

#[cfg(CONFIG_RISCV_NONSTANDARD_CACHE_OPS)]
unsafe fn ccache_flush_range(start: usize, len: usize) {
    if len == 0 { return; }
    let end = start.wrapping_add(len);
    mb();
    let mut line = start & !(SIFIVE_CCACHE_LINE_SIZE - 1);
    while line < end {
        #[cfg(CONFIG_32BIT)]
        writel((line >> 4) as u32, (CCACHE_BASE as *mut u8).add(SIFIVE_CCACHE_FLUSH32) as *mut core::ffi::c_void);
        #[cfg(not(CONFIG_32BIT))]
        core::ptr::write_volatile((CCACHE_BASE as *mut u8).add(SIFIVE_CCACHE_FLUSH64) as *mut u64, line as u64);
        line = line.wrapping_add(SIFIVE_CCACHE_LINE_SIZE);
    }
    mb();
}

unsafe fn ccache_largest_wayenabled() -> i32 {
    readl((CCACHE_BASE as *mut u8).add(SIFIVE_CCACHE_WAYENABLE) as *mut core::ffi::c_void) as i32 & 0xff
}

#[allow(unused_variables)]
unsafe fn ccache_config_read() {
    let cfg = readl((CCACHE_BASE as *mut u8).add(SIFIVE_CCACHE_CONFIG) as *mut core::ffi::c_void) as u64;
    pr_info(b"%llu banks, %llu ways, sets/bank=%llu, bytes/block=%llu\0".as_ptr() as _,
        cfg & 0xff, (cfg & 0xff00) >> 8, 1u64 << ((cfg & 0xff0000) >> 16), 1u64 << ((cfg & 0xff000000) >> 24));
    let cfg = readl((CCACHE_BASE as *mut u8).add(SIFIVE_CCACHE_WAYENABLE) as *mut core::ffi::c_void);
    pr_info(b"Index of the largest way enabled: %u\n\0".as_ptr() as _, cfg);
}

unsafe fn ccache_int_handler(irq: i32) -> i32 {
    let regs = [
        (SIFIVE_CCACHE_DIRECCFIX_HIGH, SIFIVE_CCACHE_DIRECCFIX_LOW, SIFIVE_CCACHE_DIRECCFIX_COUNT, b"DirECCFix\0".as_ptr()),
        (SIFIVE_CCACHE_DATECCFIX_HIGH, SIFIVE_CCACHE_DATECCFIX_LOW, SIFIVE_CCACHE_DATECCFIX_COUNT, b"DatECCFix\0".as_ptr()),
        (SIFIVE_CCACHE_DATECCFAIL_HIGH, SIFIVE_CCACHE_DATECCFAIL_LOW, SIFIVE_CCACHE_DATECCFAIL_COUNT, b"DatECCFail\0".as_ptr()),
        (SIFIVE_CCACHE_DIRECCFAIL_HIGH, SIFIVE_CCACHE_DIRECCFAIL_LOW, SIFIVE_CCACHE_DIRECCFAIL_COUNT, b"DirECCFail\0".as_ptr()),
    ];
    for i in 0..SIFIVE_CCACHE_MAX_ECCINTR {
        if irq == G_IRQ[i] {
            let (high, low, count, name) = regs[i];
            let add_h = readl((CCACHE_BASE as *mut u8).add(high) as _);
            let add_l = readl((CCACHE_BASE as *mut u8).add(low) as _);
            let _ = (add_h, add_l);
            readl((CCACHE_BASE as *mut u8).add(count) as _);
            atomic_notifier_call_chain(CCACHE_ERR_CHAIN, if i == DATA_CORR || i == DIR_CORR { 0 } else { 1 }, name as _);
            if i == DIR_UNCORR { panic(b"CCACHE: DirFail @ 0x%08X.%08X\n\0".as_ptr() as _, add_h, add_l); }
        }
    }
    1
}

unsafe fn sifive_ccache_probe(intr_num: i32, quirks: usize) -> i32 {
    if intr_num == 0 { return -19; }
    for i in 0..intr_num as usize {
        if i == DATA_UNCORR && (quirks & QUIRK_BROKEN_DATA_UNCORR) != 0 { continue; }
        // platform_get_irq and devm_request_irq are supplied by the kernel.
        G_IRQ[i] = i as i32;
    }
    0
}

pub unsafe fn sifive_ccache_init() -> i32 {
    // Device-tree lookup, resource mapping, cache-info registration, debugfs,
    // and platform-driver registration are provided by the kernel build.
    ccache_config_read();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
