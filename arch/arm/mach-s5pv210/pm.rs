// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2010-2014 Samsung Electronics Co., Ltd.
//	http://www.samsung.com
//
// S5PV210 - Power Management support
//
// Based on arch/arm/mach-s3c2410/pm.c
// Copyright (c) 2006 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>

// Linux/kernel and platform dependencies are supplied by the surrounding tree.

#[repr(C)]
struct sleep_save {
    reg: *mut core::ffi::c_void,
    val: usize,
}

unsafe fn s3c_pm_do_save(mut ptr: *mut sleep_save, mut count: i32) {
    while count > 0 {
        (*ptr).val = readl_relaxed((*ptr).reg) as usize;
        s3c_pmdbg_saved((*ptr).reg, (*ptr).val);
        count -= 1;
        ptr = ptr.add(1);
    }
}

unsafe fn s3c_pm_do_restore_core(mut ptr: *const sleep_save, mut count: i32) {
    while count > 0 {
        writel_relaxed((*ptr).val as u32, (*ptr).reg);
        count -= 1;
        ptr = ptr.add(1);
    }
}

static mut s5pv210_core_save: [sleep_save; 1] = [
    sleep_save { reg: S5P_MDNIE_SEL, val: 0 },
];

static mut s5pv210_irqwake_intmask: u32 = 0xffff_ffff;

unsafe fn s5pv210_read_eint_wakeup_mask() -> u32 {
    __raw_readl(S5P_EINT_WAKEUP_MASK)
}

unsafe fn s5pv210_cpu_suspend(_arg: usize) -> i32 {
    let tmp: usize = 0;
    core::arch::asm!(
        "b 1f",
        ".align 5",
        "1:",
        "mcr p15, 0, {0}, c7, c10, 5",
        "mcr p15, 0, {0}, c7, c10, 4",
        "wfi",
        in(reg) tmp,
        options(nostack)
    );
    pr_info("Failed to suspend the system\n");
    1
}

unsafe fn s5pv210_pm_prepare() {
    __raw_writel(s5pv210_irqwake_intmask, S5P_WAKEUP_MASK);
    __raw_writel(__pa_symbol(s5pv210_cpu_resume), S5P_INFORM0);

    let mut tmp = __raw_readl(S5P_SLEEP_CFG);
    tmp &= !(S5P_SLEEP_CFG_OSC_EN | S5P_SLEEP_CFG_USBOSC_EN);
    __raw_writel(tmp, S5P_SLEEP_CFG);

    tmp = __raw_readl(S5P_PWR_CFG);
    tmp &= S5P_CFG_WFI_CLEAN;
    tmp |= S5P_CFG_WFI_SLEEP;
    __raw_writel(tmp, S5P_PWR_CFG);

    tmp = __raw_readl(S5P_OTHERS);
    tmp |= S5P_OTHER_SYSC_INTOFF;
    __raw_writel(tmp, S5P_OTHERS);

    s3c_pm_do_save(s5pv210_core_save.as_mut_ptr(), 1);
}

unsafe fn s5pv210_suspend_enter(_state: suspend_state_t) -> i32 {
    let eint_wakeup_mask = s5pv210_read_eint_wakeup_mask();
    let mut ret: i32;

    s3c_pmdbg("%s: suspending the system...\n", "s5pv210_suspend_enter");
    s3c_pmdbg("%s: wakeup masks: %08x,%08x\n", "s5pv210_suspend_enter", s5pv210_irqwake_intmask, eint_wakeup_mask);

    if s5pv210_irqwake_intmask == u32::MAX && eint_wakeup_mask == u32::MAX {
        pr_err("%s: No wake-up sources!\n", "s5pv210_suspend_enter");
        pr_err("%s: Aborting sleep\n", "s5pv210_suspend_enter");
        return -22;
    }

    s3c_pm_save_uarts(false);
    s5pv210_pm_prepare();
    flush_cache_all();
    s3c_pm_check_store();
    ret = cpu_suspend(0, s5pv210_cpu_suspend);
    if ret != 0 { return ret; }
    s3c_pm_restore_uarts(false);
    s3c_pmdbg("%s: wakeup stat: %08x\n", "s5pv210_suspend_enter", __raw_readl(S5P_WAKEUP_STAT));
    s3c_pm_check_restore();
    s3c_pmdbg("%s: resuming the system...\n", "s5pv210_suspend_enter");
    0
}

unsafe fn s5pv210_suspend_prepare() -> i32 { s3c_pm_check_prepare(); 0 }
unsafe fn s5pv210_suspend_finish() { s3c_pm_check_cleanup(); }

static s5pv210_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    enter: Some(s5pv210_suspend_enter),
    prepare: Some(s5pv210_suspend_prepare),
    finish: Some(s5pv210_suspend_finish),
    valid: Some(suspend_valid_only_mem),
};

unsafe fn s5pv210_pm_resume(_data: *mut core::ffi::c_void) {
    s3c_pm_do_restore_core(s5pv210_core_save.as_ptr(), 1);
}

static s5pv210_pm_syscore_ops: syscore_ops = syscore_ops { resume: Some(s5pv210_pm_resume) };
static mut s5pv210_pm_syscore: syscore = syscore { ops: &s5pv210_pm_syscore_ops };

pub unsafe fn s5pv210_pm_init() {
    register_syscore(&mut s5pv210_pm_syscore);
    suspend_set_ops(&s5pv210_suspend_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
