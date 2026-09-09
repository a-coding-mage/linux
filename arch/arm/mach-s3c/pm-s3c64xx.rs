// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// S3C64XX CPU PM support.

// C header dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
struct S3c64xxPmDomain {
    name: *mut core::ffi::c_char,
    ena: u32,
    pwr_stat: u32,
    pd: GenericPmDomain,
}

#[repr(C)]
struct GenericPmDomain {
    power_off: Option<unsafe extern "C" fn(*mut GenericPmDomain) -> i32>,
    power_on: Option<unsafe extern "C" fn(*mut GenericPmDomain) -> i32>,
}

unsafe fn s3c64xx_pd_off(domain: *mut GenericPmDomain) -> i32 {
    let pd = (domain as *mut u8).sub(core::mem::offset_of!(S3c64xxPmDomain, pd))
        as *mut S3c64xxPmDomain;
    let mut val: u32 = __raw_readl(S3C64XX_NORMAL_CFG);
    val &= !(*pd).ena;
    __raw_writel(val, S3C64XX_NORMAL_CFG);
    0
}

unsafe fn s3c64xx_pd_on(domain: *mut GenericPmDomain) -> i32 {
    let pd = (domain as *mut u8).sub(core::mem::offset_of!(S3c64xxPmDomain, pd))
        as *mut S3c64xxPmDomain;
    let mut val: u32 = __raw_readl(S3C64XX_NORMAL_CFG);
    val |= (*pd).ena;
    __raw_writel(val, S3C64XX_NORMAL_CFG);

    // Not all domains provide power status readback
    if (*pd).pwr_stat != 0 {
        let mut retry: i64 = 1_000_000;
        loop {
            cpu_relax();
            if (__raw_readl(S3C64XX_BLK_PWR_STAT) & (*pd).pwr_stat) != 0 {
                break;
            }
            retry -= 1;
            if retry < 0 {
                break;
            }
        }
        if retry == 0 {
            pr_err!("Failed to start domain %s\n", (*pd).name);
            return -EBUSY;
        }
    }
    0
}

macro_rules! pm_domain {
    ($name:expr, $ena:expr) => {
        S3c64xxPmDomain { name: $name as *mut _, ena: $ena, pwr_stat: 0,
            pd: GenericPmDomain { power_off: Some(s3c64xx_pd_off), power_on: Some(s3c64xx_pd_on) } }
    };
    ($name:expr, $ena:expr, $stat:expr) => {
        S3c64xxPmDomain { name: $name as *mut _, ena: $ena, pwr_stat: $stat,
            pd: GenericPmDomain { power_off: Some(s3c64xx_pd_off), power_on: Some(s3c64xx_pd_on) } }
    };
}

static mut S3C64XX_PM_IROM: S3c64xxPmDomain = pm_domain!("IROM", S3C64XX_NORMALCFG_IROM_ON);
static mut S3C64XX_PM_ETM: S3c64xxPmDomain = pm_domain!("ETM", S3C64XX_NORMALCFG_DOMAIN_ETM_ON, S3C64XX_BLKPWRSTAT_ETM);
static mut S3C64XX_PM_S: S3c64xxPmDomain = pm_domain!("S", S3C64XX_NORMALCFG_DOMAIN_S_ON, S3C64XX_BLKPWRSTAT_S);
static mut S3C64XX_PM_F: S3c64xxPmDomain = pm_domain!("F", S3C64XX_NORMALCFG_DOMAIN_F_ON, S3C64XX_BLKPWRSTAT_F);
static mut S3C64XX_PM_P: S3c64xxPmDomain = pm_domain!("P", S3C64XX_NORMALCFG_DOMAIN_P_ON, S3C64XX_BLKPWRSTAT_P);
static mut S3C64XX_PM_I: S3c64xxPmDomain = pm_domain!("I", S3C64XX_NORMALCFG_DOMAIN_I_ON, S3C64XX_BLKPWRSTAT_I);
static mut S3C64XX_PM_G: S3c64xxPmDomain = pm_domain!("G", S3C64XX_NORMALCFG_DOMAIN_G_ON);
static mut S3C64XX_PM_V: S3c64xxPmDomain = pm_domain!("V", S3C64XX_NORMALCFG_DOMAIN_V_ON, S3C64XX_BLKPWRSTAT_V);

static mut S3C64XX_ALWAYS_ON_PM_DOMAINS: [*mut S3c64xxPmDomain; 1] = [&raw mut S3C64XX_PM_IROM];
static mut S3C64XX_PM_DOMAINS: [*mut S3c64xxPmDomain; 7] = [
    &raw mut S3C64XX_PM_ETM, &raw mut S3C64XX_PM_G, &raw mut S3C64XX_PM_V,
    &raw mut S3C64XX_PM_I, &raw mut S3C64XX_PM_P, &raw mut S3C64XX_PM_S,
    &raw mut S3C64XX_PM_F,
];

#[cfg(feature = "CONFIG_PM_SLEEP")]
static mut CORE_SAVE: [SleepSave; 2] = [SAVE_ITEM!(S3C64XX_MEM0DRVCON), SAVE_ITEM!(S3C64XX_MEM1DRVCON)];

#[cfg(feature = "CONFIG_PM_SLEEP")]
static mut MISC_SAVE: [SleepSave; 12] = [
    SAVE_ITEM!(S3C64XX_AHB_CON0), SAVE_ITEM!(S3C64XX_AHB_CON1), SAVE_ITEM!(S3C64XX_AHB_CON2),
    SAVE_ITEM!(S3C64XX_SPCON), SAVE_ITEM!(S3C64XX_MEM0CONSTOP), SAVE_ITEM!(S3C64XX_MEM1CONSTOP),
    SAVE_ITEM!(S3C64XX_MEM0CONSLP0), SAVE_ITEM!(S3C64XX_MEM0CONSLP1), SAVE_ITEM!(S3C64XX_MEM1CONSLP),
    SAVE_ITEM!(S3C64XX_SDMA_SEL), SAVE_ITEM!(S3C64XX_MODEM_MIFPCON), SAVE_ITEM!(S3C64XX_NORMAL_CFG),
];

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe extern "C" fn s3c_pm_configure_extint() { __raw_writel(s3c_irqwake_eintmask, S3C64XX_EINT_MASK); }

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe extern "C" fn s3c_pm_restore_core() {
    __raw_writel(0, S3C64XX_EINT_MASK);
    s3c_pm_do_restore_core(CORE_SAVE.as_ptr(), CORE_SAVE.len());
    s3c_pm_do_restore(MISC_SAVE.as_ptr(), MISC_SAVE.len());
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe extern "C" fn s3c_pm_save_core() {
    s3c_pm_do_save(MISC_SAVE.as_ptr(), MISC_SAVE.len());
    s3c_pm_do_save(CORE_SAVE.as_ptr(), CORE_SAVE.len());
}

unsafe extern "C" fn s3c64xx_cpu_suspend(_arg: usize) -> i32 {
    let mut tmp: usize = __raw_readl(S3C64XX_PWR_CFG) as usize;
    tmp &= !(S3C64XX_PWRCFG_CFG_WFI_MASK as usize);
    tmp |= S3C64XX_PWRCFG_CFG_WFI_SLEEP as usize;
    __raw_writel(tmp as u32, S3C64XX_PWR_CFG);
    __raw_writel(__raw_readl(S3C64XX_WAKEUP_STAT), S3C64XX_WAKEUP_STAT);
    tmp = 0;
    core::arch::asm!("b 1f", ".align 5", "1:", "mcr p15, 0, {0}, c7, c10, 5", "mcr p15, 0, {0}, c7, c10, 4", "mcr p15, 0, {0}, c7, c0, 4", in(reg) tmp);
    pr_info!("Failed to suspend the system\n");
    1
}

static WAKE_IRQS: [SamsungWakeupMask; 10] = [
    SamsungWakeupMask { irq: IRQ_RTC_ALARM, bit: S3C64XX_PWRCFG_RTC_ALARM_DISABLE },
    SamsungWakeupMask { irq: IRQ_RTC_TIC, bit: S3C64XX_PWRCFG_RTC_TICK_DISABLE },
    SamsungWakeupMask { irq: IRQ_PENDN, bit: S3C64XX_PWRCFG_TS_DISABLE },
    SamsungWakeupMask { irq: IRQ_HSMMC0, bit: S3C64XX_PWRCFG_MMC0_DISABLE },
    SamsungWakeupMask { irq: IRQ_HSMMC1, bit: S3C64XX_PWRCFG_MMC1_DISABLE },
    SamsungWakeupMask { irq: IRQ_HSMMC2, bit: S3C64XX_PWRCFG_MMC2_DISABLE },
    SamsungWakeupMask { irq: NO_WAKEUP_IRQ, bit: S3C64XX_PWRCFG_BATF_DISABLE },
    SamsungWakeupMask { irq: NO_WAKEUP_IRQ, bit: S3C64XX_PWRCFG_MSM_DISABLE },
    SamsungWakeupMask { irq: NO_WAKEUP_IRQ, bit: S3C64XX_PWRCFG_HSI_DISABLE },
    SamsungWakeupMask { irq: NO_WAKEUP_IRQ, bit: S3C64XX_PWRCFG_MSM_DISABLE },
];

unsafe extern "C" fn s3c64xx_pm_prepare() {
    samsung_sync_wakemask(S3C64XX_PWR_CFG, WAKE_IRQS.as_ptr(), WAKE_IRQS.len());
    __raw_writel(__pa_symbol(s3c_cpu_resume), S3C64XX_INFORM0);
    __raw_writel(__raw_readl(S3C64XX_WAKEUP_STAT), S3C64XX_WAKEUP_STAT);
}

pub unsafe extern "C" fn s3c64xx_pm_init() -> i32 {
    s3c_pm_init();
    for pd in S3C64XX_ALWAYS_ON_PM_DOMAINS.iter() { pm_genpd_init(&mut (**pd).pd, &pm_domain_always_on_gov, false); }
    for pd in S3C64XX_PM_DOMAINS.iter() { pm_genpd_init(&mut (**pd).pd, core::ptr::null(), false); }
    0
}

unsafe extern "C" fn s3c64xx_pm_initcall() -> i32 {
    if !soc_is_s3c64xx() { return 0; }
    pm_cpu_prep = Some(s3c64xx_pm_prepare);
    pm_cpu_sleep = Some(s3c64xx_cpu_suspend);
    0
}

// arch_initcall(s3c64xx_pm_initcall);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
