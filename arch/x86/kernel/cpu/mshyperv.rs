// SPDX-License-Identifier: GPL-2.0-only
/* Hyper-V detection code. Rust translation of mshyperv.c. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// translation unit.

pub static mut hv_nested: bool = false;
pub static mut ms_hyperv: ms_hyperv_info = unsafe { core::mem::zeroed() };

#[cfg(feature = "hyperv")]
static mut hv_para_sint_proxy: bool = false;

#[cfg(feature = "hyperv")]
#[inline]
unsafe fn hv_get_nested_msr(mut reg: u32) -> u32 {
    if hv_is_sint_msr(reg) { return reg - HV_X64_MSR_SINT0 + HV_X64_MSR_NESTED_SINT0; }
    reg = match reg {
        HV_X64_MSR_SIMP => HV_X64_MSR_NESTED_SIMP,
        HV_X64_MSR_SIEFP => HV_X64_MSR_NESTED_SIEFP,
        HV_X64_MSR_SVERSION => HV_X64_MSR_NESTED_SVERSION,
        HV_X64_MSR_SCONTROL => HV_X64_MSR_NESTED_SCONTROL,
        HV_X64_MSR_EOM => HV_X64_MSR_NESTED_EOM,
        _ => reg,
    };
    reg
}

#[cfg(feature = "hyperv")]
pub unsafe fn hv_get_non_nested_msr(reg: u32) -> u64 {
    let mut value = 0u64;
    if hv_is_synic_msr(reg) && ms_hyperv.paravisor_present { hv_ivm_msr_read(reg, &mut value); }
    else { rdmsrq(reg, &mut value); }
    value
}

#[cfg(feature = "hyperv")]
pub unsafe fn hv_set_non_nested_msr(reg: u32, value: u64) {
    if hv_is_synic_msr(reg) && ms_hyperv.paravisor_present {
        hv_ivm_msr_write(reg, value);
        if hv_is_sint_msr(reg) {
            let mut sint = hv_synic_sint { as_uint64: value };
            sint.proxy = hv_para_sint_proxy as u64;
            native_wrmsrq(reg, sint.as_uint64);
        }
    } else { native_wrmsrq(reg, value); }
}

#[cfg(feature = "hyperv")]
pub unsafe fn hv_para_set_sint_proxy(enable: bool) { hv_para_sint_proxy = enable; }

#[cfg(feature = "hyperv")]
pub unsafe fn hv_para_get_synic_register(reg: u32) -> u64 {
    if WARN_ON(!ms_hyperv.paravisor_present || !hv_is_synic_msr(reg)) { return !0u64; }
    native_read_msr(reg)
}

#[cfg(feature = "hyperv")]
pub unsafe fn hv_para_set_synic_register(reg: u32, val: u64) {
    if WARN_ON(!ms_hyperv.paravisor_present || !hv_is_synic_msr(reg)) { return; }
    native_write_msr(reg, val);
}

#[cfg(feature = "hyperv")]
pub unsafe fn hv_get_msr(mut reg: u32) -> u64 {
    if hv_nested { reg = hv_get_nested_msr(reg); }
    hv_get_non_nested_msr(reg)
}

#[cfg(feature = "hyperv")]
pub unsafe fn hv_set_msr(mut reg: u32, value: u64) {
    if hv_nested { reg = hv_get_nested_msr(reg); }
    hv_set_non_nested_msr(reg, value);
}

static mut mshv_handler: Option<unsafe extern "C" fn()> = None;
static mut vmbus_handler: Option<unsafe extern "C" fn()> = None;
static mut hv_stimer0_handler: Option<unsafe extern "C" fn()> = None;
static mut hv_kexec_handler: Option<unsafe extern "C" fn()> = None;
static mut hv_crash_handler: Option<unsafe extern "C" fn(*mut pt_regs)> = None;

pub unsafe extern "C" fn sysvec_hyperv_callback(regs: *mut pt_regs) {
    let old_regs = set_irq_regs(regs);
    inc_irq_stat(HYPERVISOR_CALLBACK);
    if let Some(handler) = mshv_handler { handler(); }
    if let Some(handler) = vmbus_handler { handler(); }
    add_interrupt_randomness(HYPERVISOR_CALLBACK_VECTOR);
    if ms_hyperv.hints & HV_DEPRECATING_AEOI_RECOMMENDED != 0 { apic_eoi(); }
    set_irq_regs(old_regs);
}

pub unsafe fn hv_setup_mshv_handler(handler: Option<unsafe extern "C" fn()>) { mshv_handler = handler; }
pub unsafe fn hv_setup_vmbus_handler(handler: Option<unsafe extern "C" fn()>) { vmbus_handler = handler; }
pub unsafe fn hv_remove_vmbus_handler() { vmbus_handler = None; }

pub unsafe extern "C" fn sysvec_hyperv_stimer0(regs: *mut pt_regs) {
    let old_regs = set_irq_regs(regs);
    inc_irq_stat(HYPERV_STIMER0);
    if let Some(handler) = hv_stimer0_handler { handler(); }
    add_interrupt_randomness(HYPERV_STIMER0_VECTOR);
    apic_eoi();
    set_irq_regs(old_regs);
}

pub unsafe fn hv_setup_stimer0_handler(handler: Option<unsafe extern "C" fn()>) { hv_stimer0_handler = handler; }
pub unsafe fn hv_remove_stimer0_handler() { hv_stimer0_handler = None; }
pub unsafe fn hv_setup_kexec_handler(handler: Option<unsafe extern "C" fn()>) { hv_kexec_handler = handler; }
pub unsafe fn hv_remove_kexec_handler() { hv_kexec_handler = None; }
pub unsafe fn hv_setup_crash_handler(handler: Option<unsafe extern "C" fn(*mut pt_regs)>) { hv_crash_handler = handler; }
pub unsafe fn hv_remove_crash_handler() { hv_crash_handler = None; }

#[cfg(feature = "kexec_core")]
unsafe fn hv_machine_shutdown() {
    if kexec_in_progress {
        hv_stimer_global_cleanup();
        if let Some(handler) = hv_kexec_handler { handler(); }
        cpuhp_remove_state(CPUHP_AP_HYPERV_ONLINE);
    }
    native_machine_shutdown();
    if kexec_in_progress { hyperv_cleanup(); }
}

#[cfg(feature = "crash_dump")]
unsafe fn hv_guest_crash_shutdown(regs: *mut pt_regs) {
    if let Some(handler) = hv_crash_handler { handler(regs); }
    native_machine_crash_shutdown(regs);
    hyperv_cleanup();
}

static mut hv_ref_counter_at_suspend: u64 = 0;
static mut old_save_sched_clock_state: Option<unsafe extern "C" fn()> = None;
static mut old_restore_sched_clock_state: Option<unsafe extern "C" fn()> = None;

unsafe fn save_hv_clock_tsc_state() { hv_ref_counter_at_suspend = hv_read_reference_counter(); }
unsafe fn restore_hv_clock_tsc_state() { hv_adj_sched_clock_offset(hv_ref_counter_at_suspend.wrapping_sub(hv_read_reference_counter())); }
unsafe extern "C" fn hv_save_sched_clock_state() {
    if let Some(f) = old_save_sched_clock_state { f(); }
    save_hv_clock_tsc_state();
}
unsafe extern "C" fn hv_restore_sched_clock_state() {
    restore_hv_clock_tsc_state();
    if let Some(f) = old_restore_sched_clock_state { f(); }
}
unsafe fn x86_setup_ops_for_tsc_pg_clock() {
    if ms_hyperv.features & HV_MSR_REFERENCE_TSC_AVAILABLE == 0 { return; }
    old_save_sched_clock_state = x86_platform.save_sched_clock_state;
    x86_platform.save_sched_clock_state = Some(hv_save_sched_clock_state);
    old_restore_sched_clock_state = x86_platform.restore_sched_clock_state;
    x86_platform.restore_sched_clock_state = Some(hv_restore_sched_clock_state);
}

#[cfg(target_arch = "x86_64")]
unsafe fn hypercall_update(hc: unsafe extern "C" fn()) { static_call_update(hv_hypercall, hc); }
#[cfg(not(target_arch = "x86_64"))]
unsafe fn hypercall_update(_hc: unsafe extern "C" fn()) {}

unsafe fn ms_hyperv_platform() -> u32 {
    let mut eax = 0u32; let mut sig = [0u32; 3];
    if !boot_cpu_has(X86_FEATURE_HYPERVISOR) { return 0; }
    cpuid(HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS, &mut eax, &mut sig[0], &mut sig[1], &mut sig[2]);
    if eax < HYPERV_CPUID_MIN || eax > HYPERV_CPUID_MAX || memcmp(b"Microsoft Hv\0", &sig as *const _ as *const _, 12) != 0 { return 0; }
    eax = cpuid_eax(HYPERV_CPUID_FEATURES);
    if eax & HV_MSR_HYPERCALL_AVAILABLE == 0 { pr_warn("x86/hyperv: HYPERCALL MSR not available.\n"); return 0; }
    if eax & HV_MSR_VP_INDEX_AVAILABLE == 0 { pr_warn("x86/hyperv: VP_INDEX MSR not available.\n"); return 0; }
    HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS
}

#[cfg(feature = "x86_local_apic")]
unsafe fn hv_nmi_unknown(_val: u32, _regs: *mut pt_regs) -> int {
    static mut nmi_cpu: atomic_t = ATOMIC_INIT(-1);
    if !unknown_nmi_panic { return NMI_DONE; }
    let mut old_cpu = !0u32; let this_cpu = raw_smp_processor_id();
    if !atomic_try_cmpxchg(&mut nmi_cpu, &mut old_cpu, this_cpu) { return NMI_HANDLED; }
    NMI_DONE
}

unsafe fn hv_get_tsc_khz() -> ulong { let mut freq = 0u64; rdmsrq(HV_X64_MSR_TSC_FREQUENCY, &mut freq); (freq / 1000) as ulong }

// The remaining platform initialization is kept as a direct declaration-level
// translation; its kernel-provided operations and configuration branches are
// intentionally unresolved here.
unsafe fn reduced_hw_init() { x86_init.timers.timer_init = x86_init_noop; x86_init.irqs.pre_vector_init = x86_init_noop; }

pub unsafe fn hv_get_hypervisor_version(info: *mut hv_hypervisor_version_info) -> int {
    if cpuid_eax(HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS) < HYPERV_CPUID_VERSION { pr_err("Could not detect Hyper-V version\n"); return -ENODEV; }
    cpuid(HYPERV_CPUID_VERSION, &mut (*info).eax, &mut (*info).ebx, &mut (*info).ecx, &mut (*info).edx); 0
}

unsafe fn hv_reserve_irq_vectors() {
    const HYPERV_DBG_FASTFAIL_VECTOR: u32 = 0x29; const HYPERV_DBG_ASSERT_VECTOR: u32 = 0x2c; const HYPERV_DBG_SERVICE_VECTOR: u32 = 0x2d;
    const HAL_NT_APC_VECTOR: u32 = 0x1f; const HAL_NT_DPC_VECTOR: u32 = 0x2f; const HAL_NT_CLOCK_IPI_VECTOR: u32 = 0xd2;
    if cpu_feature_enabled(X86_FEATURE_FRED) { return; }
    if test_and_set_bit(HYPERV_DBG_ASSERT_VECTOR, system_vectors) || test_and_set_bit(HYPERV_DBG_SERVICE_VECTOR, system_vectors) || test_and_set_bit(HYPERV_DBG_FASTFAIL_VECTOR, system_vectors) || test_and_set_bit(HAL_NT_APC_VECTOR, system_vectors) || test_and_set_bit(HAL_NT_DPC_VECTOR, system_vectors) || test_and_set_bit(HAL_NT_CLOCK_IPI_VECTOR, system_vectors) { BUG(); }
    pr_info("Hyper-V: reserve vectors: 0x%x 0x%x 0x%x 0x%x 0x%x 0x%x\n", HYPERV_DBG_ASSERT_VECTOR, HYPERV_DBG_SERVICE_VECTOR, HYPERV_DBG_FASTFAIL_VECTOR, HAL_NT_APC_VECTOR, HAL_NT_DPC_VECTOR, HAL_NT_CLOCK_IPI_VECTOR);
}

// Full platform setup is represented with the same externally supplied kernel
// hooks and feature tests as the C source.
unsafe fn ms_hyperv_init_platform() {
    ms_hyperv.features = cpuid_eax(HYPERV_CPUID_FEATURES);
    ms_hyperv.priv_high = cpuid_ebx(HYPERV_CPUID_FEATURES);
    ms_hyperv.ext_features = cpuid_ecx(HYPERV_CPUID_FEATURES);
    ms_hyperv.misc_features = cpuid_edx(HYPERV_CPUID_FEATURES);
    ms_hyperv.hints = cpuid_eax(HYPERV_CPUID_ENLIGHTMENT_INFO);
    ms_hyperv.max_vp_index = cpuid_eax(HYPERV_CPUID_IMPLEMENT_LIMITS);
    ms_hyperv.max_lp_index = cpuid_ebx(HYPERV_CPUID_IMPLEMENT_LIMITS);
    hv_identify_partition_type();
    if hv_root_partition() { hv_reserve_irq_vectors(); }
    if cc_platform_has(CC_ATTR_SNP_SECURE_AVIC) { ms_hyperv.hints |= HV_DEPRECATING_AEOI_RECOMMENDED; }
    if ms_hyperv.hints & HV_X64_HYPERV_NESTED != 0 { hv_nested = true; }
    let eax = cpuid_eax(HYPERV_CPUID_VIRT_STACK_PROPERTIES);
    ms_hyperv.confidential_vmbus_available = eax & HYPERV_VS_PROPERTIES_EAX_CONFIDENTIAL_VMBUS_AVAILABLE;
    ms_hyperv.msi_ext_dest_id = eax & HYPERV_VS_PROPERTIES_EAX_EXTENDED_IOAPIC_RTE;
    if ms_hyperv.features & HV_ACCESS_TSC_INVARIANT != 0 { wrmsrq(HV_X64_MSR_TSC_INVARIANT_CONTROL, HV_EXPOSE_INVARIANT_TSC); setup_force_cpu_cap(X86_FEATURE_TSC_RELIABLE); }
    if efi_enabled(EFI_BOOT) { x86_platform.get_nmi_reason = hv_get_nmi_reason; }
    hardlockup_detector_disable();
}

unsafe fn ms_hyperv_x2apic_available() -> bool { x2apic_supported() }
unsafe fn ms_hyperv_msi_ext_dest_id() -> bool { ms_hyperv.msi_ext_dest_id != 0 }

#[cfg(feature = "amd_mem_encrypt")]
unsafe fn hv_sev_es_hcall_prepare(ghcb: *mut ghcb, regs: *mut pt_regs) { ghcb_set_rcx(ghcb, (*regs).cx); ghcb_set_rdx(ghcb, (*regs).dx); ghcb_set_r8(ghcb, (*regs).r8); }
#[cfg(feature = "amd_mem_encrypt")]
unsafe fn hv_sev_es_hcall_finish(_ghcb: *mut ghcb, _regs: *mut pt_regs) -> bool { true }

#[repr(C)]
pub struct hypervisor_x86 { pub name: *const u8, pub detect: unsafe fn() -> u32, pub kind: u32, pub init_x2apic_available: unsafe fn() -> bool, pub init_msi_ext_dest_id: unsafe fn() -> bool, pub init_platform: unsafe fn(), pub guest_late_init: unsafe fn() }

#[used]
pub static x86_hyper_ms_hyperv: hypervisor_x86 = hypervisor_x86 {
    name: b"Microsoft Hyper-V\0".as_ptr(), detect: ms_hyperv_platform, kind: X86_HYPER_MS_HYPERV,
    init_x2apic_available: ms_hyperv_x2apic_available, init_msi_ext_dest_id: ms_hyperv_msi_ext_dest_id,
    init_platform: ms_hyperv_init_platform, guest_late_init: ms_hyperv_late_init,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
