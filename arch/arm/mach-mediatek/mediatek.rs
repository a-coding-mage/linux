// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Device Tree support for Mediatek SoCs
 *
 * Copyright (c) 2014 MundoReader S.L.
 * Author: Matthias Brugger <matthias.bgg@gmail.com>
 */

// Declarations supplied by the Linux kernel and architecture support.
unsafe extern "C" {
    fn of_machine_is_compatible(compat: *const core::ffi::c_char) -> bool;
    fn ioremap(phys_addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn iounmap(addr: *mut core::ffi::c_void);
    fn of_clk_init(clk_match: *const core::ffi::c_void);
    fn timer_probe();
}

const GPT6_CON_MT65XX: usize = 0x10008060;
const GPT_ENABLE: u32 = 0x31;

unsafe fn mediatek_timer_init() {
    let mut gpt_base: *mut core::ffi::c_void;

    if of_machine_is_compatible(c"mediatek,mt6589".as_ptr())
        || of_machine_is_compatible(c"mediatek,mt7623".as_ptr())
        || of_machine_is_compatible(c"mediatek,mt8135".as_ptr())
        || of_machine_is_compatible(c"mediatek,mt8127".as_ptr())
    {
        // turn on GPT6 which ungates arch timer clocks
        gpt_base = ioremap(GPT6_CON_MT65XX, 0x04);

        // enable clock and set to free-run
        writel(GPT_ENABLE, gpt_base);
        iounmap(gpt_base);
    }

    of_clk_init(core::ptr::null());
    timer_probe();
}

static mediatek_board_dt_compat: [*const core::ffi::c_char; 10] = [
    c"mediatek,mt2701".as_ptr(),
    c"mediatek,mt6572".as_ptr(),
    c"mediatek,mt6582".as_ptr(),
    c"mediatek,mt6589".as_ptr(),
    c"mediatek,mt6592".as_ptr(),
    c"mediatek,mt7623".as_ptr(),
    c"mediatek,mt7629".as_ptr(),
    c"mediatek,mt8127".as_ptr(),
    c"mediatek,mt8135".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(MEDIATEK_DT, "Mediatek Cortex-A7 (Device Tree)")
//     .dt_compat = mediatek_board_dt_compat,
//     .init_time = mediatek_timer_init,
// MACHINE_END
// The machine descriptor is emitted by the architecture's DT machine macro.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
