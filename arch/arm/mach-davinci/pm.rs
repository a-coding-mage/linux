// SPDX-License-Identifier: GPL-2.0-only
/*
 * DaVinci Power Management Routines
 *
 * Copyright (C) 2009 Texas Instruments, Inc. https://www.ti.com/
 */

// Kernel, architecture, and local header dependencies are supplied externally.

const DA850_PLL1_BASE: usize = 0x01e1a000;
const DEEPSLEEP_SLEEPCOUNT_MASK: u32 = 0xFFFF;
const DEEPSLEEP_SLEEPCOUNT: u32 = 128;

static mut davinci_sram_suspend: Option<unsafe extern "C" fn(*mut davinci_pm_config)> = None;
static mut pm_config: davinci_pm_config = davinci_pm_config {
    sleepcount: DEEPSLEEP_SLEEPCOUNT,
    ddrpsc_num: DA8XX_LPSC1_EMIF3C,
    ..davinci_pm_config::default()
};

unsafe fn davinci_sram_push(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: u32) {
    memcpy(dest, src, size as usize);
    flush_icache_range(dest as usize, dest.add(size as usize) as usize);
}

unsafe fn davinci_pm_suspend() {
    let mut val: u32;

    if pm_config.cpupll_reg_base != pm_config.ddrpll_reg_base {
        /* Switch CPU PLL to bypass mode */
        val = __raw_readl(pm_config.cpupll_reg_base.add(PLLCTL));
        val &= !(PLLCTL_PLLENSRC | PLLCTL_PLLEN);
        __raw_writel(val, pm_config.cpupll_reg_base.add(PLLCTL));

        udelay(PLL_BYPASS_TIME);

        /* Powerdown CPU PLL */
        val = __raw_readl(pm_config.cpupll_reg_base.add(PLLCTL));
        val |= PLLCTL_PLLPWRDN;
        __raw_writel(val, pm_config.cpupll_reg_base.add(PLLCTL));
    }

    /* Configure sleep count in deep sleep register */
    val = __raw_readl(pm_config.deepsleep_reg);
    val &= !DEEPSLEEP_SLEEPCOUNT_MASK;
    val |= pm_config.sleepcount;
    __raw_writel(val, pm_config.deepsleep_reg);

    /* System goes to sleep in this call */
    (davinci_sram_suspend.unwrap())(&mut pm_config);

    if pm_config.cpupll_reg_base != pm_config.ddrpll_reg_base {
        /* put CPU PLL in reset */
        val = __raw_readl(pm_config.cpupll_reg_base.add(PLLCTL));
        val &= !PLLCTL_PLLRST;
        __raw_writel(val, pm_config.cpupll_reg_base.add(PLLCTL));

        /* put CPU PLL in power down */
        val = __raw_readl(pm_config.cpupll_reg_base.add(PLLCTL));
        val &= !PLLCTL_PLLPWRDN;
        __raw_writel(val, pm_config.cpupll_reg_base.add(PLLCTL));

        /* wait for CPU PLL reset */
        udelay(PLL_RESET_TIME);

        /* bring CPU PLL out of reset */
        val = __raw_readl(pm_config.cpupll_reg_base.add(PLLCTL));
        val |= PLLCTL_PLLRST;
        __raw_writel(val, pm_config.cpupll_reg_base.add(PLLCTL));

        /* Wait for CPU PLL to lock */
        udelay(PLL_LOCK_TIME);

        /* Remove CPU PLL from bypass mode */
        val = __raw_readl(pm_config.cpupll_reg_base.add(PLLCTL));
        val &= !PLLCTL_PLLENSRC;
        val |= PLLCTL_PLLEN;
        __raw_writel(val, pm_config.cpupll_reg_base.add(PLLCTL));
    }
}

unsafe fn davinci_pm_enter(state: suspend_state_t) -> i32 {
    let mut ret: i32 = 0;

    match state {
        PM_SUSPEND_MEM => davinci_pm_suspend(),
        _ => ret = -EINVAL,
    }

    ret
}

static davinci_pm_ops: platform_suspend_ops = platform_suspend_ops {
    enter: Some(davinci_pm_enter),
    valid: Some(suspend_valid_only_mem),
};

pub unsafe fn davinci_pm_init() -> i32 {
    let mut ret: i32;

    ret = davinci_cfg_reg(DA850_RTC_ALARM);
    if ret != 0 {
        return ret;
    }

    pm_config.ddr2_ctlr_base = da8xx_get_mem_ctlr();
    pm_config.deepsleep_reg = DA8XX_SYSCFG1_VIRT(DA8XX_DEEPSLEEP_REG);

    pm_config.cpupll_reg_base = ioremap(DA8XX_PLL0_BASE, SZ_4K);
    if pm_config.cpupll_reg_base.is_null() {
        return -ENOMEM;
    }

    pm_config.ddrpll_reg_base = ioremap(DA850_PLL1_BASE, SZ_4K);
    if pm_config.ddrpll_reg_base.is_null() {
        ret = -ENOMEM;
        goto no_ddrpll_mem;
    }

    pm_config.ddrpsc_reg_base = ioremap(DA8XX_PSC1_BASE, SZ_4K);
    if pm_config.ddrpsc_reg_base.is_null() {
        ret = -ENOMEM;
        goto no_ddrpsc_mem;
    }

    davinci_sram_suspend = sram_alloc(davinci_cpu_suspend_sz, core::ptr::null_mut());
    if davinci_sram_suspend.is_none() {
        pr_err!("PM: cannot allocate SRAM memory\n");
        ret = -ENOMEM;
        goto no_sram_mem;
    }

    davinci_sram_push(
        davinci_sram_suspend.unwrap() as *mut core::ffi::c_void,
        davinci_cpu_suspend,
        davinci_cpu_suspend_sz,
    );

    suspend_set_ops(&davinci_pm_ops);

    return 0;

no_sram_mem:
    iounmap(pm_config.ddrpsc_reg_base);
no_ddrpsc_mem:
    iounmap(pm_config.ddrpll_reg_base);
no_ddrpll_mem:
    iounmap(pm_config.cpupll_reg_base);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
