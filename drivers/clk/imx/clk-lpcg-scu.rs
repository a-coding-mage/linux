// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018 NXP
 *\tDong Aisheng <aisheng.dong@nxp.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut IMX_LPCG_SCU_LOCK: Spinlock = DEFINE_SPINLOCK!();

const CLK_GATE_SCU_LPCG_MASK: u32 = 0x3;
const CLK_GATE_SCU_LPCG_HW_SEL: u32 = BIT(0);
const CLK_GATE_SCU_LPCG_SW_SEL: u32 = BIT(1);

/*
 * struct clk_lpcg_scu - Description of LPCG clock
 *
 * @hw: clk_hw of this LPCG
 * @reg: register of this LPCG clock
 * @bit_idx: bit index of this LPCG clock
 * @hw_gate: HW auto gate enable
 *
 * This structure describes one LPCG clock
 */
#[repr(C)]
struct ClkLpcgScu {
    hw: ClkHw,
    reg: *mut core::ffi::c_void,
    bit_idx: u8,
    hw_gate: bool,

    /* for state save&restore */
    state: u32,
}

unsafe fn to_clk_lpcg_scu(hw: *mut ClkHw) -> *mut ClkLpcgScu {
    container_of!(hw, ClkLpcgScu, hw)
}

/* e10858 -LPCG clock gating register synchronization errata */
unsafe fn lpcg_e10858_writel(rate: usize, reg: *mut core::ffi::c_void, val: u32) {
    writel(val, reg);

    if rate >= 24 * HZ_PER_MHZ || rate == 0 {
        /*
         * The time taken to access the LPCG registers from the AP core
         * through the interconnect is longer than the minimum delay
         * of 4 clock cycles required by the errata.
         * Adding a readl will provide sufficient delay to prevent
         * back-to-back writes.
         */
        readl(reg);
    } else {
        /*
         * For clocks running below 24MHz, wait a minimum of
         * 4 clock cycles.
         */
        ndelay(4 * div_round_up(1000 * HZ_PER_MHZ, rate));
    }
}

unsafe extern "C" fn clk_lpcg_scu_enable(hw: *mut ClkHw) -> i32 {
    let clk = &mut *to_clk_lpcg_scu(hw);
    let mut flags: C_ulong = 0;
    let mut reg: u32;
    let mut val: u32;

    spin_lock_irqsave(&raw mut IMX_LPCG_SCU_LOCK, &mut flags);

    reg = readl_relaxed(clk.reg);
    reg &= !(CLK_GATE_SCU_LPCG_MASK << clk.bit_idx);

    val = CLK_GATE_SCU_LPCG_SW_SEL;
    if clk.hw_gate {
        val |= CLK_GATE_SCU_LPCG_HW_SEL;
    }

    reg |= val << clk.bit_idx;

    lpcg_e10858_writel(clk_hw_get_rate(hw), clk.reg, reg);

    spin_unlock_irqrestore(&raw mut IMX_LPCG_SCU_LOCK, flags);

    0
}

unsafe extern "C" fn clk_lpcg_scu_disable(hw: *mut ClkHw) {
    let clk = &mut *to_clk_lpcg_scu(hw);
    let mut flags: C_ulong = 0;
    let mut reg: u32;

    spin_lock_irqsave(&raw mut IMX_LPCG_SCU_LOCK, &mut flags);

    reg = readl_relaxed(clk.reg);
    reg &= !(CLK_GATE_SCU_LPCG_MASK << clk.bit_idx);
    lpcg_e10858_writel(clk_hw_get_rate(hw), clk.reg, reg);

    spin_unlock_irqrestore(&raw mut IMX_LPCG_SCU_LOCK, flags);
}

static CLK_LPCG_SCU_OPS: ClkOps = ClkOps {
    enable: Some(clk_lpcg_scu_enable),
    disable: Some(clk_lpcg_scu_disable),
};

unsafe extern "C" fn __imx_clk_lpcg_scu(
    dev: *mut Device,
    name: *const C_char,
    parent_name: *const C_char,
    flags: C_ulong,
    reg: *mut core::ffi::c_void,
    bit_idx: u8,
    hw_gate: bool,
) -> *mut ClkHw {
    let clk: *mut ClkLpcgScu;
    let mut init: ClkInitData;
    let mut hw: *mut ClkHw;
    let ret: i32;

    clk = kzalloc_obj!(*clk);
    if clk.is_null() {
        return err_ptr!(-ENOMEM);
    }

    (*clk).reg = reg;
    (*clk).bit_idx = bit_idx;
    (*clk).hw_gate = hw_gate;

    init.name = name;
    init.ops = &raw const CLK_LPCG_SCU_OPS;
    init.flags = CLK_SET_RATE_PARENT | flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };

    (*clk).hw.init = &raw mut init;

    hw = &mut (*clk).hw;
    ret = clk_hw_register(dev, hw);
    if ret != 0 {
        kfree(clk.cast());
        hw = err_ptr!(ret);
        return hw;
    }

    if !dev.is_null() {
        dev_set_drvdata(dev, clk.cast());
    }

    hw
}

unsafe extern "C" fn imx_clk_lpcg_scu_unregister(hw: *mut ClkHw) {
    let clk = to_clk_lpcg_scu(hw);

    clk_hw_unregister(&mut (*clk).hw);
    kfree(clk.cast());
}

unsafe extern "C" fn imx_clk_lpcg_scu_suspend(dev: *mut Device) -> i32 {
    let clk = dev_get_drvdata(dev) as *mut ClkLpcgScu;

    if !strncmp(c"hdmi_lpcg".as_ptr(), clk_hw_get_name(&mut (*clk).hw), strlen(c"hdmi_lpcg".as_ptr())) {
        return 0;
    }

    (*clk).state = readl_relaxed((*clk).reg);
    dev_dbg!(dev, "save lpcg state 0x%x\n", (*clk).state);

    0
}

unsafe extern "C" fn imx_clk_lpcg_scu_resume(dev: *mut Device) -> i32 {
    let clk = dev_get_drvdata(dev) as *mut ClkLpcgScu;

    if !strncmp(c"hdmi_lpcg".as_ptr(), clk_hw_get_name(&mut (*clk).hw), strlen(c"hdmi_lpcg".as_ptr())) {
        return 0;
    }

    writel((*clk).state, (*clk).reg);
    lpcg_e10858_writel(0, (*clk).reg, (*clk).state);
    dev_dbg!(dev, "restore lpcg state 0x%x\n", (*clk).state);

    0
}

static IMX_CLK_LPCG_SCU_PM_OPS: DevPmOps = SET_NOIRQ_SYSTEM_SLEEP_PM_OPS!(
    imx_clk_lpcg_scu_suspend,
    imx_clk_lpcg_scu_resume,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
