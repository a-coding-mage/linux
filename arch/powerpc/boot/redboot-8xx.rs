// SPDX-License-Identifier: GPL-2.0-only
/*
 * RedBoot firmware support
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// C dependencies supplied by the surrounding platform.

#[repr(C)]
struct bd_t {
    bi_tag: u32,
    bi_memstart: usize,
    bi_memsize: usize,
    bi_enetaddr: [u8; 6],
    bi_intfreq: u32,
    bi_busfreq: u32,
    bi_cmdline: usize,
}

#[repr(C)]
struct PlatformOps {
    fixups: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct LoaderInfo {
    cmdline: *mut u8,
    cmdline_len: usize,
}

extern "C" {
    static mut _end: u8;
    static _dtb_start: u8;
    static mut platform_ops: PlatformOps;
    static mut loader_info: LoaderInfo;

    fn dt_fixup_memory(start: usize, size: usize);
    fn dt_fixup_mac_addresses(address: *const u8);
    fn dt_fixup_cpu_clocks(intfreq: u32, busfreq_div: u32, busfreq: u32);
    fn finddevice(path: *const u8) -> *mut core::ffi::c_void;
    fn printf(format: *const u8, ...);
    fn setprop(node: *mut core::ffi::c_void, name: *const u8, value: *const u8, length: u32);
    fn memcpy(destination: *mut core::ffi::c_void, source: *const core::ffi::c_void, length: usize);
    fn simple_alloc_init(start: *mut u8, size: usize, align: usize, boundary: usize);
    fn fdt_init(dtb: *const u8);
    fn serial_console_init();
    fn strlen(string: *const u8) -> usize;
}

static mut bd: bd_t = bd_t {
    bi_tag: 0,
    bi_memstart: 0,
    bi_memsize: 0,
    bi_enetaddr: [0; 6],
    bi_intfreq: 0,
    bi_busfreq: 0,
    bi_cmdline: 0,
};

// BSS_STACK(4096);

#[inline]
const fn mhz(x: u32) -> u32 {
    (x + 500_000) / 1_000_000
}

unsafe extern "C" fn platform_fixups() {
    let mut node: *mut core::ffi::c_void;

    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_addresses(bd.bi_enetaddr.as_ptr());
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 16, bd.bi_busfreq);

    node = finddevice(b"/soc/cpm/brg\0".as_ptr());
    if !node.is_null() {
        printf(
            b"BRG clock-frequency <- 0x%x (%dMHz)\r\n\0".as_ptr(),
            bd.bi_busfreq,
            mhz(bd.bi_busfreq),
        );
        setprop(
            node,
            b"clock-frequency\0".as_ptr(),
            (&bd.bi_busfreq as *const u32).cast::<u8>(),
            4,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn platform_init(
    r3: usize,
    _r4: usize,
    _r5: usize,
    _r6: usize,
    _r7: usize,
) {
    memcpy(
        (&raw mut bd).cast::<core::ffi::c_void>(),
        (r3 as *const u8).cast::<core::ffi::c_void>(),
        core::mem::size_of::<bd_t>(),
    );

    if bd.bi_tag != 0x4244_4944 {
        return;
    }

    simple_alloc_init(
        (&raw mut _end),
        bd.bi_memstart + bd.bi_memsize - (&raw const _end as usize),
        32,
        64,
    );

    fdt_init(&raw const _dtb_start);
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);

    loader_info.cmdline = bd.bi_cmdline as *mut u8;
    loader_info.cmdline_len = strlen(bd.bi_cmdline as *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
