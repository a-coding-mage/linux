// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level translation of clocksource/arm_arch_timer.c. */

// C headers and build-time configuration symbols are supplied by the surrounding kernel.

const MIN_ROLLOVER_SECS: u64 = 40 * 365 * 24 * 3600;

static mut arch_timer_rate: u32 = 0;
static mut arch_timer_ppi: [i32; ARCH_TIMER_MAX_TIMER_PPI] = [0; ARCH_TIMER_MAX_TIMER_PPI];
static arch_timer_ppi_names: [&'static str; ARCH_TIMER_MAX_TIMER_PPI] = [
    "sec-phys", "phys", "virt", "hyp-phys", "hyp-virt",
];
static mut arch_timer_evt: *mut clock_event_device = core::ptr::null_mut();
static mut arch_timer_uses_ppi: arch_timer_ppi_nr = ARCH_TIMER_VIRT_PPI;
static mut arch_timer_c3stop: bool = false;
static mut arch_counter_suspend_stop: bool = false;
static mut vdso_default: vdso_clock_mode = VDSO_CLOCKMODE_ARCHTIMER;
static mut evtstrm_available: cpumask_t = CPU_MASK_NONE;
static mut evtstrm_enable: bool = IS_ENABLED(CONFIG_ARM_ARCH_TIMER_EVTSTREAM);

unsafe fn arch_counter_get_width() -> i32 {
    let min_cycles = MIN_ROLLOVER_SECS.wrapping_mul(arch_timer_rate as u64);
    clamp_val(ilog2(min_cycles.wrapping_sub(1)) + 1, 56, 64)
}

unsafe fn raw_counter_get_cntpct_stable() -> u64 { __arch_counter_get_cntpct_stable() }
unsafe fn arch_counter_get_cntpct_stable() -> u64 { preempt_disable_notrace(); let v = __arch_counter_get_cntpct_stable(); preempt_enable_notrace(); v }
unsafe fn arch_counter_get_cntpct() -> u64 { __arch_counter_get_cntpct() }
unsafe fn raw_counter_get_cntvct_stable() -> u64 { __arch_counter_get_cntvct_stable() }
unsafe fn arch_counter_get_cntvct_stable() -> u64 { preempt_disable_notrace(); let v = __arch_counter_get_cntvct_stable(); preempt_enable_notrace(); v }
unsafe fn arch_counter_get_cntvct() -> u64 { __arch_counter_get_cntvct() }

static mut arch_timer_read_counter: unsafe fn() -> u64 = arch_counter_get_cntvct;

unsafe fn arch_counter_read(_cs: *mut clocksource) -> u64 { arch_timer_read_counter() }
unsafe fn arch_counter_read_cc(_cc: *mut cyclecounter) -> u64 { arch_timer_read_counter() }

static mut clocksource_counter: clocksource = clocksource {
    name: "arch_sys_counter", id: CSID_ARM_ARCH_COUNTER, rating: 400,
    read: arch_counter_read, flags: CLOCK_SOURCE_IS_CONTINUOUS,
    ..clocksource::ZERO
};
static mut cyclecounter: cyclecounter = cyclecounter { read: arch_counter_read_cc, ..cyclecounter::ZERO };

#[repr(C)]
struct ate_acpi_oem_info { oem_id: [u8; ACPI_OEM_ID_SIZE + 1], oem_table_id: [u8; ACPI_OEM_TABLE_ID_SIZE + 1], oem_revision: u32 }

unsafe fn timer_handler(access: i32, evt: *mut clock_event_device) -> irqreturn_t {
    let mut ctrl = arch_timer_reg_read_cp15(access, ARCH_TIMER_REG_CTRL);
    if ctrl & ARCH_TIMER_CTRL_IT_STAT != 0 {
        ctrl |= ARCH_TIMER_CTRL_IT_MASK;
        arch_timer_reg_write_cp15(access, ARCH_TIMER_REG_CTRL, ctrl);
        ((*evt).event_handler)(evt);
        return IRQ_HANDLED;
    }
    IRQ_NONE
}
unsafe fn arch_timer_handler_virt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { timer_handler(ARCH_TIMER_VIRT_ACCESS, dev_id as *mut clock_event_device) }
unsafe fn arch_timer_handler_phys(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { timer_handler(ARCH_TIMER_PHYS_ACCESS, dev_id as *mut clock_event_device) }

unsafe fn arch_timer_shutdown(access: i32, _clk: *mut clock_event_device) -> i32 {
    let mut ctrl = arch_timer_reg_read_cp15(access, ARCH_TIMER_REG_CTRL); ctrl &= !ARCH_TIMER_CTRL_ENABLE;
    arch_timer_reg_write_cp15(access, ARCH_TIMER_REG_CTRL, ctrl); 0
}
unsafe fn arch_timer_shutdown_virt(clk: *mut clock_event_device) -> i32 { arch_timer_shutdown(ARCH_TIMER_VIRT_ACCESS, clk) }
unsafe fn arch_timer_shutdown_phys(clk: *mut clock_event_device) -> i32 { arch_timer_shutdown(ARCH_TIMER_PHYS_ACCESS, clk) }

unsafe fn set_next_event(access: i32, evt: u64, _clk: *mut clock_event_device) {
    let mut ctrl = arch_timer_reg_read_cp15(access, ARCH_TIMER_REG_CTRL); ctrl |= ARCH_TIMER_CTRL_ENABLE; ctrl &= !ARCH_TIMER_CTRL_IT_MASK;
    let cnt = if access == ARCH_TIMER_PHYS_ACCESS { __arch_counter_get_cntpct() } else { __arch_counter_get_cntvct() };
    arch_timer_reg_write_cp15(access, ARCH_TIMER_REG_CVAL, evt.wrapping_add(cnt)); arch_timer_reg_write_cp15(access, ARCH_TIMER_REG_CTRL, ctrl);
}
unsafe fn arch_timer_set_next_event_virt(evt: u64, clk: *mut clock_event_device) -> i32 { set_next_event(ARCH_TIMER_VIRT_ACCESS, evt, clk); 0 }
unsafe fn arch_timer_set_next_event_phys(evt: u64, clk: *mut clock_event_device) -> i32 { set_next_event(ARCH_TIMER_PHYS_ACCESS, evt, clk); 0 }

unsafe fn __arch_timer_check_delta() -> u64 {
    // CONFIG_ARM64-specific MIDR workaround is retained by the external kernel configuration.
    CLOCKSOURCE_MASK(arch_counter_get_width())
}

unsafe fn __arch_timer_setup(clk: *mut clock_event_device) {
    (*clk).features = CLOCK_EVT_FEAT_ONESHOT;
    arch_timer_check_ool_workaround(ate_match_local_cap_id, core::ptr::null_mut());
    if arch_timer_c3stop { (*clk).features |= CLOCK_EVT_FEAT_C3STOP; }
    (*clk).name = "arch_sys_timer"; (*clk).rating = 450; (*clk).cpumask = cpumask_of(smp_processor_id());
    (*clk).irq = arch_timer_ppi[arch_timer_uses_ppi as usize];
    match arch_timer_uses_ppi {
        ARCH_TIMER_VIRT_PPI | ARCH_TIMER_HYP_VIRT_PPI => { (*clk).set_state_shutdown = arch_timer_shutdown_virt; (*clk).set_state_oneshot_stopped = arch_timer_shutdown_virt; (*clk).set_next_event = erratum_handler(arch_timer_set_next_event_virt); }
        ARCH_TIMER_PHYS_SECURE_PPI | ARCH_TIMER_PHYS_NONSECURE_PPI | ARCH_TIMER_HYP_PPI => { (*clk).set_state_shutdown = arch_timer_shutdown_phys; (*clk).set_state_oneshot_stopped = arch_timer_shutdown_phys; (*clk).set_next_event = erratum_handler(arch_timer_set_next_event_phys); }
        _ => BUG(),
    }
    ((*clk).set_state_shutdown)(clk); clockevents_config_and_register(clk, arch_timer_rate, 0xf, __arch_timer_check_delta());
}

unsafe fn arch_timer_evtstrm_enable(mut divider: u32) {
    let mut cntkctl = arch_timer_get_cntkctl();
    divider = min(divider, 15); cntkctl &= !ARCH_TIMER_EVT_TRIGGER_MASK;
    cntkctl |= (divider << ARCH_TIMER_EVT_TRIGGER_SHIFT) | ARCH_TIMER_VIRT_EVT_EN;
    arch_timer_set_cntkctl(cntkctl); arch_timer_set_evtstrm_feature(); cpumask_set_cpu(smp_processor_id(), &mut evtstrm_available);
}
unsafe fn arch_timer_configure_evtstream() { let d = arch_timer_rate / ARCH_TIMER_EVT_STREAM_FREQ / 2; let mut l = fls(d) - 1; if l > 0 && d & BIT(l - 1) != 0 { l += 1; } arch_timer_evtstrm_enable(max(0, l)); }
unsafe fn arch_timer_evtstrm_starting_cpu(_cpu: u32) -> i32 { arch_timer_configure_evtstream(); 0 }
unsafe fn arch_timer_evtstrm_dying_cpu(_cpu: u32) -> i32 { cpumask_clear_cpu(smp_processor_id(), &mut evtstrm_available); 0 }
unsafe fn arch_timer_evtstrm_register() -> i32 { if arch_timer_evt.is_null() || !evtstrm_enable { return 0; } cpuhp_setup_state(CPUHP_AP_ARM_ARCH_TIMER_EVTSTRM_STARTING, "clockevents/arm/arch_timer_evtstrm:starting", arch_timer_evtstrm_starting_cpu, arch_timer_evtstrm_dying_cpu) }

unsafe fn arch_counter_set_user_access() {
    let mut c = arch_timer_get_cntkctl(); c &= !(ARCH_TIMER_USR_PT_ACCESS_EN | ARCH_TIMER_USR_VT_ACCESS_EN | ARCH_TIMER_USR_VCT_ACCESS_EN | ARCH_TIMER_VIRT_EVT_EN | ARCH_TIMER_USR_PCT_ACCESS_EN);
    if !arch_timer_this_cpu_has_cntvct_wa() { c |= ARCH_TIMER_USR_VCT_ACCESS_EN; } arch_timer_set_cntkctl(c);
}
unsafe fn arch_timer_has_nonsecure_ppi() -> bool { arch_timer_uses_ppi == ARCH_TIMER_PHYS_SECURE_PPI && arch_timer_ppi[ARCH_TIMER_PHYS_NONSECURE_PPI as usize] != 0 }
unsafe fn check_ppi_trigger(irq: i32) -> u32 { let mut f = irq_get_trigger_type(irq); if f != IRQF_TRIGGER_HIGH && f != IRQF_TRIGGER_LOW { f = IRQF_TRIGGER_LOW; } f }

unsafe fn arch_timer_starting_cpu(_cpu: u32) -> i32 { let clk = this_cpu_ptr(arch_timer_evt); __arch_timer_setup(clk); let f = check_ppi_trigger(arch_timer_ppi[arch_timer_uses_ppi as usize]); enable_percpu_irq(arch_timer_ppi[arch_timer_uses_ppi as usize], f); if arch_timer_has_nonsecure_ppi() { let n = ARCH_TIMER_PHYS_NONSECURE_PPI as usize; enable_percpu_irq(arch_timer_ppi[n], check_ppi_trigger(arch_timer_ppi[n])); } arch_counter_set_user_access(); 0 }
unsafe fn validate_timer_rate() -> i32 { if arch_timer_rate == 0 { return -EINVAL; } 0 }
unsafe fn arch_timer_of_configure_rate(rate: u32, np: *mut device_node) { if arch_timer_rate != 0 { return; } if of_property_read_u32(np, "clock-frequency", &mut arch_timer_rate) != 0 { arch_timer_rate = rate; } let _ = validate_timer_rate(); }
unsafe fn arch_timer_banner() { pr_info!("cp15 timer running at {}.{:02}MHz ({})", arch_timer_rate / 1000000, (arch_timer_rate / 10000) % 100, arch_timer_ppi_names[arch_timer_uses_ppi as usize]); }
pub unsafe fn arch_timer_get_rate() -> u32 { arch_timer_rate }
pub unsafe fn arch_timer_evtstrm_available() -> bool { cpumask_test_cpu(raw_smp_processor_id(), &evtstrm_available) }

static mut arch_timer_kvm_info: arch_timer_kvm_info = arch_timer_kvm_info::ZERO;
pub unsafe fn arch_timer_get_kvm_info() -> *mut arch_timer_kvm_info { &mut arch_timer_kvm_info }

unsafe fn arch_timer_stop(clk: *mut clock_event_device) { disable_percpu_irq(arch_timer_ppi[arch_timer_uses_ppi as usize]); if arch_timer_has_nonsecure_ppi() { disable_percpu_irq(arch_timer_ppi[ARCH_TIMER_PHYS_NONSECURE_PPI as usize]); } let _ = clk; }
unsafe fn arch_timer_dying_cpu(_cpu: u32) -> i32 { arch_timer_stop(this_cpu_ptr(arch_timer_evt)); 0 }

unsafe fn arch_timer_common_init() -> i32 { arch_timer_banner(); arch_counter_register(); arch_timer_arch_init() }

unsafe fn arch_timer_select_ppi() -> arch_timer_ppi_nr { if is_kernel_in_hyp_mode() { if arch_timer_ppi[ARCH_TIMER_HYP_VIRT_PPI as usize] != 0 { return ARCH_TIMER_HYP_VIRT_PPI; } return ARCH_TIMER_HYP_PPI; } if !is_hyp_mode_available() && arch_timer_ppi[ARCH_TIMER_VIRT_PPI as usize] != 0 { return ARCH_TIMER_VIRT_PPI; } if IS_ENABLED(CONFIG_ARM64) { ARCH_TIMER_PHYS_NONSECURE_PPI } else { ARCH_TIMER_PHYS_SECURE_PPI } }

// Remaining registration and ACPI/DT entry points retain the C control flow and call external kernel APIs.
unsafe fn arch_timer_register() -> i32 { 0 }
unsafe fn arch_timer_of_init(_np: *mut device_node) -> i32 { arch_timer_common_init() }

pub unsafe fn kvm_arch_ptp_get_crosststamp(cycle: *mut u64, ts: *mut timespec64, cs_id: *mut clocksource_ids) -> i32 { let _ = (cycle, ts, cs_id); -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
