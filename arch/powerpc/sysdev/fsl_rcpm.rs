// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RCPM(Run Control/Power Management) support
 *
 * Copyright 2012-2015 Freescale Semiconductor Inc.
 *
 * Author: Chenhui Zhao <chenhui.zhao@freescale.com>
 */

// Linux headers and build-time definitions are supplied by the surrounding tree.

static mut rcpm_v1_regs: *mut ccsr_rcpm_v1 = core::ptr::null_mut();
static mut rcpm_v2_regs: *mut ccsr_rcpm_v2 = core::ptr::null_mut();
static mut fsl_supported_pm_modes: u32 = 0;

unsafe fn rcpm_v1_irq_mask(cpu: i32) {
    let hw_cpu = get_hard_smp_processor_id(cpu);
    let mask: u32 = 1u32 << hw_cpu;
    setbits32(&mut (*rcpm_v1_regs).cpmimr, mask);
    setbits32(&mut (*rcpm_v1_regs).cpmcimr, mask);
    setbits32(&mut (*rcpm_v1_regs).cpmmcmr, mask);
    setbits32(&mut (*rcpm_v1_regs).cpmnmimr, mask);
}

unsafe fn rcpm_v2_irq_mask(cpu: i32) {
    let hw_cpu = get_hard_smp_processor_id(cpu);
    let mask: u32 = 1u32 << hw_cpu;
    setbits32(&mut (*rcpm_v2_regs).tpmimr0, mask);
    setbits32(&mut (*rcpm_v2_regs).tpmcimr0, mask);
    setbits32(&mut (*rcpm_v2_regs).tpmmcmr0, mask);
    setbits32(&mut (*rcpm_v2_regs).tpmnmimr0, mask);
}

unsafe fn rcpm_v1_irq_unmask(cpu: i32) {
    let hw_cpu = get_hard_smp_processor_id(cpu);
    let mask: u32 = 1u32 << hw_cpu;
    clrbits32(&mut (*rcpm_v1_regs).cpmimr, mask);
    clrbits32(&mut (*rcpm_v1_regs).cpmcimr, mask);
    clrbits32(&mut (*rcpm_v1_regs).cpmmcmr, mask);
    clrbits32(&mut (*rcpm_v1_regs).cpmnmimr, mask);
}

unsafe fn rcpm_v2_irq_unmask(cpu: i32) {
    let hw_cpu = get_hard_smp_processor_id(cpu);
    let mask: u32 = 1u32 << hw_cpu;
    clrbits32(&mut (*rcpm_v2_regs).tpmimr0, mask);
    clrbits32(&mut (*rcpm_v2_regs).tpmcimr0, mask);
    clrbits32(&mut (*rcpm_v2_regs).tpmmcmr0, mask);
    clrbits32(&mut (*rcpm_v2_regs).tpmnmimr0, mask);
}

unsafe fn rcpm_v1_set_ip_power(enable: bool, mask: u32) {
    if enable { setbits32(&mut (*rcpm_v1_regs).ippdexpcr, mask); }
    else { clrbits32(&mut (*rcpm_v1_regs).ippdexpcr, mask); }
}

unsafe fn rcpm_v2_set_ip_power(enable: bool, mask: u32) {
    if enable { setbits32(&mut (*rcpm_v2_regs).ippdexpcr[0], mask); }
    else { clrbits32(&mut (*rcpm_v2_regs).ippdexpcr[0], mask); }
}

unsafe fn rcpm_v1_cpu_enter_state(cpu: i32, state: i32) {
    let hw_cpu = get_hard_smp_processor_id(cpu);
    let mask: u32 = 1u32 << hw_cpu;
    match state {
        E500_PM_PH10 => setbits32(&mut (*rcpm_v1_regs).cdozcr, mask),
        E500_PM_PH15 => setbits32(&mut (*rcpm_v1_regs).cnapcr, mask),
        _ => pr_warn!("Unknown cpu PM state ({})\n", state),
    }
}

unsafe fn rcpm_v2_cpu_enter_state(cpu: i32, state: i32) {
    let hw_cpu = get_hard_smp_processor_id(cpu);
    let mask: u32 = 1u32 << cpu_core_index_of_thread(cpu);
    match state {
        E500_PM_PH10 => setbits32(&mut (*rcpm_v2_regs).tph10setr0, 1u32 << hw_cpu),
        E500_PM_PH15 => setbits32(&mut (*rcpm_v2_regs).pcph15setr, mask),
        E500_PM_PH20 => setbits32(&mut (*rcpm_v2_regs).pcph20setr, mask),
        E500_PM_PH30 => setbits32(&mut (*rcpm_v2_regs).pcph30setr, mask),
        _ => pr_warn!("Unknown cpu PM state ({})\n", state),
    }
}

unsafe fn rcpm_v1_cpu_die(cpu: i32) { rcpm_v1_cpu_enter_state(cpu, E500_PM_PH15); }

#[cfg(CONFIG_PPC64)]
unsafe fn qoriq_disable_thread(cpu: i32) {
    let thread = cpu_thread_in_core(cpu);
    book3e_stop_thread(thread);
}

unsafe fn rcpm_v2_cpu_die(cpu: i32) {
    #[cfg(CONFIG_PPC64)]
    if threads_per_core == 2 {
        let primary = cpu_first_thread_sibling(cpu);
        if cpu_is_offline(primary) && cpu_is_offline(primary + 1) {
            rcpm_v2_cpu_enter_state(cpu, E500_PM_PH20);
        } else { qoriq_disable_thread(cpu); }
    }
    if threads_per_core == 1 { rcpm_v2_cpu_enter_state(cpu, E500_PM_PH20); }
}

unsafe fn rcpm_v1_cpu_exit_state(cpu: i32, state: i32) {
    let hw_cpu = get_hard_smp_processor_id(cpu);
    let mask: u32 = 1u32 << hw_cpu;
    match state {
        E500_PM_PH10 => clrbits32(&mut (*rcpm_v1_regs).cdozcr, mask),
        E500_PM_PH15 => clrbits32(&mut (*rcpm_v1_regs).cnapcr, mask),
        _ => pr_warn!("Unknown cpu PM state ({})\n", state),
    }
}

unsafe fn rcpm_v1_cpu_up_prepare(cpu: i32) {
    rcpm_v1_cpu_exit_state(cpu, E500_PM_PH15); rcpm_v1_irq_unmask(cpu);
}

unsafe fn rcpm_v2_cpu_exit_state(cpu: i32, state: i32) {
    let hw_cpu = get_hard_smp_processor_id(cpu);
    let mask: u32 = 1u32 << cpu_core_index_of_thread(cpu);
    match state {
        E500_PM_PH10 => setbits32(&mut (*rcpm_v2_regs).tph10clrr0, 1u32 << hw_cpu),
        E500_PM_PH15 => setbits32(&mut (*rcpm_v2_regs).pcph15clrr, mask),
        E500_PM_PH20 => setbits32(&mut (*rcpm_v2_regs).pcph20clrr, mask),
        E500_PM_PH30 => setbits32(&mut (*rcpm_v2_regs).pcph30clrr, mask),
        _ => pr_warn!("Unknown cpu PM state ({})\n", state),
    }
}

unsafe fn rcpm_v2_cpu_up_prepare(cpu: i32) {
    rcpm_v2_cpu_exit_state(cpu, E500_PM_PH20); rcpm_v2_irq_unmask(cpu);
}

unsafe fn rcpm_v1_plat_enter_state(state: i32) -> i32 {
    let pmcsr_reg = &mut (*rcpm_v1_regs).powmgtcsr; let mut ret = 0;
    match state {
        PLAT_PM_SLEEP => { setbits32(pmcsr_reg, RCPM_POWMGTCSR_SLP); let result = spin_event_timeout(in_be32(pmcsr_reg) & RCPM_POWMGTCSR_SLP == 0, 10000, 10); if result == 0 { pr_err!("timeout waiting for SLP bit to be cleared\n"); ret = -ETIMEDOUT; } }
        _ => { pr_warn!("Unknown platform PM state ({})", state); ret = -EINVAL; }
    } ret
}

unsafe fn rcpm_v2_plat_enter_state(state: i32) -> i32 {
    let pmcsr_reg = &mut (*rcpm_v2_regs).powmgtcsr; let mut ret = 0;
    match state {
        PLAT_PM_LPM20 => { setbits32(pmcsr_reg, RCPM_POWMGTCSR_P_LPM20_ST); setbits32(pmcsr_reg, RCPM_POWMGTCSR_LPM20_RQ); let result = spin_event_timeout(in_be32(pmcsr_reg) & RCPM_POWMGTCSR_LPM20_ST == 0, 10000, 10); if result == 0 { pr_err!("timeout waiting for LPM20 bit to be cleared\n"); ret = -ETIMEDOUT; } }
        _ => { pr_warn!("Unknown platform PM state ({})", state); ret = -EINVAL; }
    } ret
}

unsafe fn rcpm_v1_plat_enter_sleep() -> i32 { rcpm_v1_plat_enter_state(PLAT_PM_SLEEP) }
unsafe fn rcpm_v2_plat_enter_sleep() -> i32 { rcpm_v2_plat_enter_state(PLAT_PM_LPM20) }

unsafe fn rcpm_common_freeze_time_base(tben_reg: *mut u32, freeze: bool) {
    static mut MASK: u32 = 0;
    if freeze { MASK = in_be32(tben_reg); clrbits32(tben_reg, MASK); } else { setbits32(tben_reg, MASK); }
    in_be32(tben_reg);
}
unsafe fn rcpm_v1_freeze_time_base(freeze: bool) { rcpm_common_freeze_time_base(&mut (*rcpm_v1_regs).ctbenr, freeze); }
unsafe fn rcpm_v2_freeze_time_base(freeze: bool) { rcpm_common_freeze_time_base(&mut (*rcpm_v2_regs).pctbenr, freeze); }
fn rcpm_get_pm_modes() -> u32 { unsafe { fsl_supported_pm_modes } }

// Operation tables and device matching data retain the external kernel types and symbols.
static qoriq_rcpm_v1_ops: fsl_pm_ops = fsl_pm_ops { irq_mask: Some(rcpm_v1_irq_mask), irq_unmask: Some(rcpm_v1_irq_unmask), cpu_enter_state: Some(rcpm_v1_cpu_enter_state), cpu_exit_state: Some(rcpm_v1_cpu_exit_state), cpu_up_prepare: Some(rcpm_v1_cpu_up_prepare), cpu_die: Some(rcpm_v1_cpu_die), plat_enter_sleep: Some(rcpm_v1_plat_enter_sleep), set_ip_power: Some(rcpm_v1_set_ip_power), freeze_time_base: Some(rcpm_v1_freeze_time_base), get_pm_modes: Some(rcpm_get_pm_modes) };
static qoriq_rcpm_v2_ops: fsl_pm_ops = fsl_pm_ops { irq_mask: Some(rcpm_v2_irq_mask), irq_unmask: Some(rcpm_v2_irq_unmask), cpu_enter_state: Some(rcpm_v2_cpu_enter_state), cpu_exit_state: Some(rcpm_v2_cpu_exit_state), cpu_up_prepare: Some(rcpm_v2_cpu_up_prepare), cpu_die: Some(rcpm_v2_cpu_die), plat_enter_sleep: Some(rcpm_v2_plat_enter_sleep), set_ip_power: Some(rcpm_v2_set_ip_power), freeze_time_base: Some(rcpm_v2_freeze_time_base), get_pm_modes: Some(rcpm_get_pm_modes) };

static rcpm_matches: [of_device_id; 4] = [
    of_device_id { compatible: "fsl,qoriq-rcpm-1.0", data: &qoriq_rcpm_v1_ops },
    of_device_id { compatible: "fsl,qoriq-rcpm-2.0", data: &qoriq_rcpm_v2_ops },
    of_device_id { compatible: "fsl,qoriq-rcpm-2.1", data: &qoriq_rcpm_v2_ops },
    of_device_id::default(),
];

unsafe fn fsl_rcpm_init() -> i32 {
    let mut match_ptr: *const of_device_id = core::ptr::null();
    let np = of_find_matching_node_and_match(core::ptr::null_mut(), rcpm_matches.as_ptr(), &mut match_ptr);
    if np.is_null() { return 0; }
    let base = of_iomap(np, 0); of_node_put(np);
    if base.is_null() { pr_err!("of_iomap() error.\n"); return -ENOMEM; }
    rcpm_v1_regs = base as *mut ccsr_rcpm_v1; rcpm_v2_regs = base as *mut ccsr_rcpm_v2;
    fsl_supported_pm_modes = FSL_PM_SLEEP;
    qoriq_pm_ops = (*match_ptr).data;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
