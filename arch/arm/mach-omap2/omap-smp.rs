// SPDX-License-Identifier: GPL-2.0-only
/* OMAP4 SMP source file; direct low-level translation of omap-smp.c. */

// C headers and build-time configuration supplied by the surrounding kernel.

const CPU_MASK: u32 = 0xff0ffff0;
const CPU_CORTEX_A9: u32 = 0x410fc090;
const CPU_CORTEX_A15: u32 = 0x410fc0f0;
const OMAP5_CORE_COUNT: u32 = 0x2;
const AUX_CORE_BOOT0_GP_RELEASE: u32 = 0x020;
const AUX_CORE_BOOT0_HS_RELEASE: u32 = 0x200;

#[repr(C)]
pub struct OmapSmpConfig {
    cpu1_rstctrl_pa: usize,
    cpu1_rstctrl_va: *mut core::ffi::c_void,
    scu_base: *mut core::ffi::c_void,
    wakeupgen_base: *mut core::ffi::c_void,
    startup_addr: *mut core::ffi::c_void,
}

static mut CFG: OmapSmpConfig = OmapSmpConfig {
    cpu1_rstctrl_pa: 0,
    cpu1_rstctrl_va: core::ptr::null_mut(),
    scu_base: core::ptr::null_mut(),
    wakeupgen_base: core::ptr::null_mut(),
    startup_addr: core::ptr::null_mut(),
};

extern "C" {
    fn omap4_secondary_startup();
    fn omap4460_secondary_startup();
    fn omap5_secondary_startup();
    fn omap5_secondary_hyp_startup();
    fn omap4_cpu_die();
    fn omap4_cpu_kill();
}
static OMAP443X_CFG: OmapSmpConfig = OmapSmpConfig { cpu1_rstctrl_pa: 0x4824380c, cpu1_rstctrl_va: core::ptr::null_mut(), scu_base: core::ptr::null_mut(), wakeupgen_base: core::ptr::null_mut(), startup_addr: omap4_secondary_startup as *mut _ };
static OMAP446X_CFG: OmapSmpConfig = OmapSmpConfig { cpu1_rstctrl_pa: 0x4824380c, cpu1_rstctrl_va: core::ptr::null_mut(), scu_base: core::ptr::null_mut(), wakeupgen_base: core::ptr::null_mut(), startup_addr: omap4460_secondary_startup as *mut _ };
static OMAP5_CFG: OmapSmpConfig = OmapSmpConfig { cpu1_rstctrl_pa: 0x48243810, cpu1_rstctrl_va: core::ptr::null_mut(), scu_base: core::ptr::null_mut(), wakeupgen_base: core::ptr::null_mut(), startup_addr: omap5_secondary_startup as *mut _ };

pub unsafe extern "C" fn omap4_get_scu_base() -> *mut core::ffi::c_void { CFG.scu_base }

#[cfg(feature = "CONFIG_OMAP5_ERRATA_801819")]
unsafe fn omap5_erratum_workaround_801819() {
    let mut acr: u32;
    let revidr: u32;
    core::arch::asm!("mrc p15, 0, {0}, c0, c0, 6", out(reg) revidr);
    if revidr & (1 << 3) != 0 { return; }
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 1", out(reg) acr);
    let acr_mask = (3 << 25) | (3 << 27);
    if acr & acr_mask == acr_mask { return; }
    acr |= acr_mask;
    omap_smc1(OMAP5_DRA7_MON_SET_ACR_INDEX, acr);
    pr_debug("%s: ARM erratum workaround 801819 applied on CPU%d\n", "omap5_erratum_workaround_801819", smp_processor_id());
}
#[cfg(not(feature = "CONFIG_OMAP5_ERRATA_801819"))]
unsafe fn omap5_erratum_workaround_801819() {}

#[cfg(feature = "CONFIG_HARDEN_BRANCH_PREDICTOR")]
unsafe fn omap5_secondary_harden_predictor() {
    let acr: u32;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 1", out(reg) acr);
    let acr_mask = 1u32;
    if acr & acr_mask == acr_mask { return; }
    omap_smc1(OMAP5_DRA7_MON_SET_ACR_INDEX, acr | acr_mask);
    pr_debug("%s: ARM ACR setup for CVE_2017_5715 applied on CPU%d\n", "omap5_secondary_harden_predictor", smp_processor_id());
}
#[cfg(not(feature = "CONFIG_HARDEN_BRANCH_PREDICTOR"))]
unsafe fn omap5_secondary_harden_predictor() {}

unsafe fn omap4_secondary_init(_cpu: u32) {
    if soc_is_omap443x() && omap_type() != OMAP2_DEVICE_TYPE_GP {
        omap_secure_dispatcher(OMAP4_PPA_CPU_ACTRL_SMP_INDEX, 4, 0, 0, 0, 0, 0);
    }
    if soc_is_omap54xx() || soc_is_dra7xx() {
        set_cntfreq();
        omap5_erratum_workaround_801819();
        omap5_secondary_harden_predictor();
    }
}

unsafe fn omap4_boot_secondary(cpu: u32, _idle: *mut core::ffi::c_void) -> i32 {
    static mut CPU1_CLKDM: *mut core::ffi::c_void = core::ptr::null_mut();
    static mut CPU1_PWRDM: *mut core::ffi::c_void = core::ptr::null_mut();
    static mut BOOTED: bool = false;
    if omap_secure_apis_support() { omap_modify_auxcoreboot0(AUX_CORE_BOOT0_HS_RELEASE, 0xfffffdff); }
    else { writel_relaxed(AUX_CORE_BOOT0_GP_RELEASE, CFG.wakeupgen_base.add(OMAP_AUX_CORE_BOOT_0 as usize)); }
    if CPU1_CLKDM.is_null() && CPU1_PWRDM.is_null() {
        CPU1_CLKDM = clkdm_lookup("mpu1_clkdm\0".as_ptr() as *const _);
        CPU1_PWRDM = pwrdm_lookup("cpu1_pwrdm\0".as_ptr() as *const _);
    }
    if BOOTED && !CPU1_PWRDM.is_null() && !CPU1_CLKDM.is_null() {
        if IS_PM44XX_ERRATUM(PM_OMAP4_ROM_SMP_BOOT_ERRATUM_GICD) { local_irq_disable(); gic_dist_disable(); }
        clkdm_deny_idle_nolock(CPU1_CLKDM); pwrdm_set_next_pwrst(CPU1_PWRDM, PWRDM_POWER_ON); clkdm_allow_idle_nolock(CPU1_CLKDM);
        if IS_PM44XX_ERRATUM(PM_OMAP4_ROM_SMP_BOOT_ERRATUM_GICD) {
            while gic_dist_disabled() { udelay(1); cpu_relax(); }
            gic_timer_retrigger(); local_irq_enable();
        }
    } else { dsb_sev(); BOOTED = true; }
    arch_send_wakeup_ipi_mask(cpumask_of(cpu)); 0
}

unsafe fn omap4_smp_init_cpus() {
    let mut ncores = 1u32;
    let cpu_id = read_cpuid_id() & CPU_MASK;
    if cpu_id == CPU_CORTEX_A9 { CFG.scu_base = OMAP2_L4_IO_ADDRESS(scu_a9_get_base()); BUG_ON(CFG.scu_base.is_null()); ncores = scu_get_core_count(CFG.scu_base); }
    else if cpu_id == CPU_CORTEX_A15 { ncores = OMAP5_CORE_COUNT; }
    if ncores > nr_cpu_ids { pr_warn("SMP: %u cores greater than maximum (%u), clipping\n", ncores, nr_cpu_ids); ncores = nr_cpu_ids; }
    for i in 0..ncores { set_cpu_possible(i, true); }
}

unsafe fn omap4_smp_cpu1_startup_valid(addr: usize) -> bool { !(addr >= __pa(PAGE_OFFSET) && addr <= __pa(__bss_start)) }

unsafe fn omap4_smp_maybe_reset_cpu1(c: *mut OmapSmpConfig) {
    let released = if omap_secure_apis_support() { omap_read_auxcoreboot0() & AUX_CORE_BOOT0_HS_RELEASE } else { readl_relaxed(CFG.wakeupgen_base.add(OMAP_AUX_CORE_BOOT_0 as usize)) & AUX_CORE_BOOT0_GP_RELEASE };
    if released != 0 { pr_warn("smp: CPU1 not parked?\n"); return; }
    let startup = readl_relaxed(CFG.wakeupgen_base.add(OMAP_AUX_CORE_BOOT_1 as usize));
    let mut needs_reset = !omap4_smp_cpu1_startup_valid(startup as usize);
    let ns = if soc_is_omap44xx() || soc_is_omap54xx() { let x = omap4_get_cpu1_ns_pa_addr(); if !omap4_smp_cpu1_startup_valid(x as usize) { needs_reset = true; } x } else { 0 };
    if !needs_reset || (*c).cpu1_rstctrl_va.is_null() { return; }
    pr_info("smp: CPU1 parked within kernel, needs reset (0x%lx 0x%lx)\n", startup, ns);
    writel_relaxed(1, (*c).cpu1_rstctrl_va); readl_relaxed((*c).cpu1_rstctrl_va); writel_relaxed(0, (*c).cpu1_rstctrl_va);
}

unsafe fn omap4_smp_prepare_cpus(_max_cpus: u32) {
    let c = if soc_is_omap443x() { &OMAP443X_CFG } else if soc_is_omap446x() { &OMAP446X_CFG } else if soc_is_dra74x() || soc_is_omap54xx() || soc_is_dra76x() { &OMAP5_CFG } else { pr_err!("%s Unknown SMP SoC?\n", "omap4_smp_prepare_cpus"); return; };
    CFG.cpu1_rstctrl_pa = c.cpu1_rstctrl_pa; CFG.startup_addr = c.startup_addr; CFG.wakeupgen_base = omap_get_wakeupgen_base();
    if soc_is_dra74x() || soc_is_omap54xx() || soc_is_dra76x() { if (__boot_cpu_mode & MODE_MASK) == HYP_MODE { CFG.startup_addr = omap5_secondary_hyp_startup as *mut _; } omap5_erratum_workaround_801819(); }
    CFG.cpu1_rstctrl_va = ioremap(CFG.cpu1_rstctrl_pa, 4); if CFG.cpu1_rstctrl_va.is_null() { return; }
    if !CFG.scu_base.is_null() { scu_enable(CFG.scu_base); }
    omap4_smp_maybe_reset_cpu1(&mut CFG);
    if omap_secure_apis_support() { omap_auxcoreboot_addr(__pa_symbol(CFG.startup_addr)); } else { writel_relaxed(__pa_symbol(CFG.startup_addr), CFG.wakeupgen_base.add(OMAP_AUX_CORE_BOOT_1 as usize)); }
}

#[repr(C)]
pub struct SmpOperations { pub smp_init_cpus: unsafe fn(), pub smp_prepare_cpus: unsafe fn(u32), pub smp_secondary_init: unsafe fn(u32), pub smp_boot_secondary: unsafe fn(u32, *mut core::ffi::c_void) -> i32 }
pub static OMAP4_SMP_OPS: SmpOperations = SmpOperations { smp_init_cpus: omap4_smp_init_cpus, smp_prepare_cpus: omap4_smp_prepare_cpus, smp_secondary_init: omap4_secondary_init, smp_boot_secondary: omap4_boot_secondary };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
