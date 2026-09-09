// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Clocksource using the Low Power Timer found in the Low Power Controller (LPC)
 *
 * Copyright (C) 2015 STMicroelectronics – All Rights Reserved
 *
 * Author(s): Francesco Virlinzi <francesco.virlinzi@st.com>
 *\t      Ajit Pal Singh <ajitpal.singh@st.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

/* Low Power Timer */
const LPC_LPT_LSB_OFF: usize = 0x400;
const LPC_LPT_MSB_OFF: usize = 0x404;
const LPC_LPT_START_OFF: usize = 0x408;

#[repr(C)]
struct StClksrcDdata {
    clk: *mut Clk,
    base: *mut core::ffi::c_void,
}

static mut ddata: StClksrcDdata = StClksrcDdata {
    clk: core::ptr::null_mut(),
    base: core::ptr::null_mut(),
};

unsafe fn st_clksrc_reset() {
    writel_relaxed(0, (ddata.base as *mut u8).add(LPC_LPT_START_OFF));
    writel_relaxed(0, (ddata.base as *mut u8).add(LPC_LPT_MSB_OFF));
    writel_relaxed(0, (ddata.base as *mut u8).add(LPC_LPT_LSB_OFF));
    writel_relaxed(1, (ddata.base as *mut u8).add(LPC_LPT_START_OFF));
}

unsafe fn st_clksrc_sched_clock_read() -> u64 {
    readl_relaxed((ddata.base as *mut u8).add(LPC_LPT_LSB_OFF)) as u64
}

unsafe fn st_clksrc_init() -> i32 {
    let rate: usize;
    let ret: i32;

    st_clksrc_reset();

    rate = clk_get_rate(ddata.clk);

    sched_clock_register(st_clksrc_sched_clock_read, 32, rate);

    ret = clocksource_mmio_init(
        (ddata.base as *mut u8).add(LPC_LPT_LSB_OFF),
        "clksrc-st-lpc",
        rate,
        300,
        32,
        clocksource_mmio_readl_up,
    );
    if ret != 0 {
        pr_err("clksrc-st-lpc: Failed to register clocksource\n");
        return ret;
    }

    0
}

unsafe fn st_clksrc_setup_clk(np: *mut DeviceNode) -> i32 {
    let clk: *mut Clk;

    clk = of_clk_get(np, 0);
    if is_err(clk) {
        pr_err("clksrc-st-lpc: Failed to get LPC clock\n");
        return ptr_err(clk);
    }

    if clk_prepare_enable(clk) != 0 {
        pr_err("clksrc-st-lpc: Failed to enable LPC clock\n");
        return -22;
    }

    if clk_get_rate(clk) == 0 {
        pr_err("clksrc-st-lpc: Failed to get LPC clock rate\n");
        clk_disable_unprepare(clk);
        return -22;
    }

    ddata.clk = clk;

    0
}

unsafe fn st_clksrc_of_register(np: *mut DeviceNode) -> i32 {
    let ret: i32;
    let mut mode: u32 = 0;

    ret = of_property_read_u32(np, "st,lpc-mode", &mut mode);
    if ret != 0 {
        pr_err("clksrc-st-lpc: An LPC mode must be provided\n");
        return ret;
    }

    /* LPC can either run as a Clocksource or in RTC or WDT mode */
    if mode != ST_LPC_MODE_CLKSRC {
        return 0;
    }

    ddata.base = of_iomap(np, 0);
    if ddata.base.is_null() {
        pr_err("clksrc-st-lpc: Unable to map iomem\n");
        return -6;
    }

    ret = st_clksrc_setup_clk(np);
    if ret != 0 {
        iounmap(ddata.base);
        return ret;
    }

    ret = st_clksrc_init();
    if ret != 0 {
        clk_disable_unprepare(ddata.clk);
        clk_put(ddata.clk);
        iounmap(ddata.base);
        return ret;
    }

    pr_info(
        "clksrc-st-lpc: clocksource initialised - running @ %luHz\n",
        clk_get_rate(ddata.clk),
    );

    ret
}

// Equivalent of TIMER_OF_DECLARE(ddata, "st,stih407-lpc", st_clksrc_of_register).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
