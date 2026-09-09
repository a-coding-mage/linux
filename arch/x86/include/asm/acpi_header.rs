/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from x86/include/asm/acpi.h. */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(feature = "CONFIG_ACPI")]
extern "C" {
    pub static mut acpi_lapic: ::core::ffi::c_int;
    pub static mut acpi_ioapic: ::core::ffi::c_int;
    pub static mut acpi_noirq: ::core::ffi::c_int;
    pub static mut acpi_strict: ::core::ffi::c_int;
    pub static mut acpi_disabled: ::core::ffi::c_int;
    pub static mut acpi_pci_disabled: ::core::ffi::c_int;
    pub static mut acpi_skip_timer_override: ::core::ffi::c_int;
    pub static mut acpi_use_timer_override: ::core::ffi::c_int;
    pub static mut acpi_fix_pin2_polarity: ::core::ffi::c_int;
    pub static mut acpi_disable_cmcff: ::core::ffi::c_int;
    pub static mut acpi_int_src_ovr: [bool; NR_IRQS_LEGACY];
    pub static mut acpi_sci_flags: u8;
    pub static mut acpi_sci_override_gsi: u32;
    pub fn acpi_pic_sci_set_trigger(trigger: ::core::ffi::c_uint, polarity: u16);
    pub static mut __acpi_register_gsi:
        Option<unsafe extern "C" fn(*mut device, u32, ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>;
    pub static mut __acpi_unregister_gsi: Option<unsafe extern "C" fn(u32)>;
    pub fn acpi_gsi_to_irq(gsi: u32, irq: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn acpi_blacklisted() -> ::core::ffi::c_int;
    pub static mut acpi_suspend_lowlevel: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
    pub fn acpi_get_wakeup_address() -> ::core::ffi::c_ulong;
    pub fn acpi_parse_mp_wake(header: *mut acpi_subtable_headers, end: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn asm_acpi_mp_play_dead(reset_vector: u64, pgd_pa: u64);
    pub fn acpi_generic_reduced_hw_init();
    pub fn x86_default_set_root_pointer(addr: u64);
    pub fn x86_default_get_root_pointer() -> u64;
    pub fn acpi_setup_mp_wakeup_mailbox(addr: u64);
    pub fn acpi_get_mp_wakeup_mailbox() -> *mut acpi_madt_multiproc_wakeup_mailbox;
    pub fn acpi_get_mp_wakeup_mailbox_paddr() -> u64;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_subtable_headers {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn disable_acpi() {
    acpi_disabled = 1;
    acpi_pci_disabled = 1;
    acpi_noirq = 1;
}

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn acpi_noirq_set() { acpi_noirq = 1; }

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn acpi_disable_pci() {
    acpi_pci_disabled = 1;
    acpi_noirq_set();
}

#[inline]
pub fn acpi_skip_set_wakeup_address() -> bool {
    unsafe { cpu_feature_enabled(X86_FEATURE_XENPV) }
}

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn acpi_processor_cstate_check(mut max_cstate: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    if boot_cpu_data.x86 == 0x0f && boot_cpu_data.x86_vendor == X86_VENDOR_AMD
        && boot_cpu_data.x86_model <= 0x05 && boot_cpu_data.x86_stepping < 0x0a {
        1
    } else if boot_cpu_has(X86_BUG_AMD_APIC_C1E) { 1 } else { max_cstate }
}

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn arch_has_acpi_pdc() -> bool {
    let c = &cpu_data(0);
    c.x86_vendor == X86_VENDOR_INTEL || c.x86_vendor == X86_VENDOR_CENTAUR
}

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn arch_acpi_set_proc_cap_bits(cap: *mut u32) {
    let c = &cpu_data(0);
    *cap |= ACPI_PROC_CAP_C_CAPABILITY_SMP;
    *cap |= ACPI_PROC_CAP_SMP_T_SWCOORD;
    if cpu_has(c, X86_FEATURE_EST) { *cap |= ACPI_PROC_CAP_EST_CAPABILITY_SWSMP; }
    if cpu_has(c, X86_FEATURE_ACPI) { *cap |= ACPI_PROC_CAP_T_FFH; }
    if cpu_has(c, X86_FEATURE_HWP) { *cap |= ACPI_PROC_CAP_COLLAB_PROC_PERF; }
    if !cpu_has(c, X86_FEATURE_MWAIT) || boot_option_idle_override == IDLE_NOMWAIT {
        *cap &= !(ACPI_PROC_CAP_C_C1_FFH | ACPI_PROC_CAP_C_C2C3_FFH);
    }
    if xen_initial_domain() { xen_sanitize_proc_cap_bits(cap); }
}

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn acpi_has_cpu_in_madt() -> bool { acpi_lapic != 0 }

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn acpi_arch_set_root_pointer(addr: u64) { x86_init.acpi.set_root_pointer(addr); }

#[cfg(feature = "CONFIG_ACPI")]
pub unsafe fn acpi_arch_get_root_pointer() -> u64 { x86_init.acpi.get_root_pointer() }

#[cfg(not(feature = "CONFIG_ACPI"))]
pub const acpi_lapic: i32 = 0;
#[cfg(not(feature = "CONFIG_ACPI"))]
pub const acpi_ioapic: i32 = 0;
#[cfg(not(feature = "CONFIG_ACPI"))]
pub const acpi_disable_cmcff: i32 = 0;
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn acpi_noirq_set() {}
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn acpi_disable_pci() {}
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn disable_acpi() {}
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn acpi_generic_reduced_hw_init() {}
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn x86_default_set_root_pointer(_addr: u64) {}
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn x86_default_get_root_pointer() -> u64 { 0 }
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn acpi_setup_mp_wakeup_mailbox(_addr: u64) {}
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn acpi_get_mp_wakeup_mailbox() -> *mut acpi_madt_multiproc_wakeup_mailbox { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_ACPI"))]
pub fn acpi_get_mp_wakeup_mailbox_paddr() -> u64 { 0 }

pub const ARCH_HAS_POWER_INIT: i32 = 1;

#[cfg(feature = "CONFIG_ACPI_NUMA")]
extern "C" { pub fn x86_acpi_numa_init() -> ::core::ffi::c_int; }

#[repr(C)]
pub struct cper_ia_proc_ctx { _private: [u8; 0] }

#[cfg(feature = "CONFIG_ACPI_APEI")]
pub unsafe fn arch_apei_get_mem_attribute(_addr: phys_addr_t) -> pgprot_t { PAGE_KERNEL_NOENC }

#[cfg(feature = "CONFIG_ACPI_APEI")]
extern "C" { pub fn arch_apei_report_x86_error(ctx_info: *mut cper_ia_proc_ctx, lapic_id: u64) -> ::core::ffi::c_int; }
#[cfg(not(feature = "CONFIG_ACPI_APEI"))]
pub unsafe fn arch_apei_report_x86_error(_ctx_info: *mut cper_ia_proc_ctx, _lapic_id: u64) -> ::core::ffi::c_int { -EINVAL }

pub const ACPI_TABLE_UPGRADE_MAX_PHYS: usize = max_low_pfn_mapped << PAGE_SHIFT;

/* CONFIG_XEN_PV: acpi_os_ioremap has a Xen PV-specific declaration in the C header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
