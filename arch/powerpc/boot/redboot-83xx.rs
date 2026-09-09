// SPDX-License-Identifier: GPL-2.0-only
/*
 * RedBoot firmware support
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 * Copyright (c) 2008 Codehermit
 */

// C dependencies: ops.h, stdio.h, redboot.h, fsl-soc.h, io.h

static mut bd: bd_t = unsafe { core::mem::zeroed() };
// BSS_STACK(4096);

const fn mhz(x: u32) -> u32 {
    (x + 500_000) / 1_000_000
}

unsafe fn platform_fixups() {
    let mut node: *mut core::ffi::c_void;

    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_addresses(bd.bi_enetaddr);
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 16, bd.bi_busfreq);

    node = finddevice(c"/soc/cpm/brg".as_ptr() as *const core::ffi::c_char);
    if !node.is_null() {
        printf(
            c"BRG clock-frequency <- 0x%x (%dMHz)\r\n".as_ptr(),
            bd.bi_busfreq,
            mhz(bd.bi_busfreq),
        );
        setprop(
            node,
            c"clock-frequency".as_ptr() as *const core::ffi::c_char,
            &bd.bi_busfreq as *const _ as *const core::ffi::c_void,
            4,
        );
    }
}

pub unsafe fn platform_init(
    r3: u64,
    r4: u64,
    r5: u64,
    r6: u64,
    r7: u64,
) {
    let _ = (r4, r5, r6, r7);
    memcpy(
        &mut bd as *mut bd_t as *mut core::ffi::c_void,
        r3 as *const core::ffi::c_char as *const core::ffi::c_void,
        core::mem::size_of::<bd_t>(),
    );

    if bd.bi_tag != 0x42444944 {
        return;
    }

    simple_alloc_init(
        _end,
        bd.bi_memstart + bd.bi_memsize - _end as u64,
        32,
        64,
    );

    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);

    loader_info.cmdline = bd.bi_cmdline as *mut core::ffi::c_char;
    loader_info.cmdline_len = strlen(bd.bi_cmdline as *const core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
