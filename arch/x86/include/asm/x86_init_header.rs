/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations supplied by other translation units.
pub struct ghcb;
pub struct mpc_bus;
pub struct mpc_cpu;
pub struct pt_regs;
pub struct mpc_table;
pub struct cpuinfo_x86;
pub struct irq_domain;
pub struct x86_msi_ops;
pub struct timespec64;

/**
 * struct x86_init_mpparse - platform specific mpparse ops
 */
#[repr(C)]
pub struct x86_init_mpparse {
    pub setup_ioapic_ids: Option<unsafe extern "C" fn()>,
    pub find_mptable: Option<unsafe extern "C" fn()>,
    pub early_parse_smp_cfg: Option<unsafe extern "C" fn()>,
    pub parse_smp_cfg: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct x86_init_resources {
    pub probe_roms: Option<unsafe extern "C" fn()>,
    pub reserve_resources: Option<unsafe extern "C" fn()>,
    pub memory_setup: Option<unsafe extern "C" fn() -> *mut ::core::ffi::c_char>,
    pub dmi_setup: Option<unsafe extern "C" fn()>,
    pub realmode_limit: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct x86_init_irqs {
    pub pre_vector_init: Option<unsafe extern "C" fn()>,
    pub intr_init: Option<unsafe extern "C" fn()>,
    pub intr_mode_select: Option<unsafe extern "C" fn()>,
    pub intr_mode_init: Option<unsafe extern "C" fn()>,
    pub create_pci_msi_domain: Option<unsafe extern "C" fn() -> *mut irq_domain>,
}

#[repr(C)]
pub struct x86_init_oem {
    pub arch_setup: Option<unsafe extern "C" fn()>,
    pub banner: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct x86_init_paging {
    pub pagetable_init: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct x86_init_timers {
    pub setup_percpu_clockev: Option<unsafe extern "C" fn()>,
    pub timer_init: Option<unsafe extern "C" fn()>,
    pub wallclock_init: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct x86_init_iommu {
    pub iommu_init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct x86_init_pci {
    pub arch_init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub fixup_irqs: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct x86_hyper_init {
    pub init_platform: Option<unsafe extern "C" fn()>,
    pub guest_late_init: Option<unsafe extern "C" fn()>,
    pub x2apic_available: Option<unsafe extern "C" fn() -> bool>,
    pub msi_ext_dest_id: Option<unsafe extern "C" fn() -> bool>,
    pub init_mem_mapping: Option<unsafe extern "C" fn()>,
    pub init_after_bootmem: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct x86_init_acpi {
    pub set_root_pointer: Option<unsafe extern "C" fn(addr: u64)>,
    pub get_root_pointer: Option<unsafe extern "C" fn() -> u64>,
    pub reduced_hw_early_init: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct x86_guest {
    pub enc_status_change_prepare: Option<unsafe extern "C" fn(vaddr: ::core::ffi::c_ulong, npages: ::core::ffi::c_int, enc: bool) -> ::core::ffi::c_int>,
    pub enc_status_change_finish: Option<unsafe extern "C" fn(vaddr: ::core::ffi::c_ulong, npages: ::core::ffi::c_int, enc: bool) -> ::core::ffi::c_int>,
    pub enc_tlb_flush_required: Option<unsafe extern "C" fn(enc: bool) -> bool>,
    pub enc_cache_flush_required: Option<unsafe extern "C" fn() -> bool>,
    pub enc_kexec_begin: Option<unsafe extern "C" fn()>,
    pub enc_kexec_finish: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct x86_init_ops {
    pub resources: x86_init_resources,
    pub mpparse: x86_init_mpparse,
    pub irqs: x86_init_irqs,
    pub oem: x86_init_oem,
    pub paging: x86_init_paging,
    pub timers: x86_init_timers,
    pub iommu: x86_init_iommu,
    pub pci: x86_init_pci,
    pub hyper: x86_hyper_init,
    pub acpi: x86_init_acpi,
}

#[repr(C)]
pub struct x86_cpuinit_ops {
    pub setup_percpu_clockev: Option<unsafe extern "C" fn()>,
    pub early_percpu_clock_init: Option<unsafe extern "C" fn()>,
    pub fixup_cpu_id: Option<unsafe extern "C" fn(c: *mut cpuinfo_x86, node: ::core::ffi::c_int)>,
    pub parallel_bringup: bool,
}

#[repr(C)]
pub struct x86_legacy_devices {
    pub pnpbios: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum x86_legacy_i8042_state {
    X86_LEGACY_I8042_PLATFORM_ABSENT,
    X86_LEGACY_I8042_FIRMWARE_ABSENT,
    X86_LEGACY_I8042_EXPECTED_PRESENT,
}

#[repr(C)]
pub struct x86_legacy_features {
    pub i8042: x86_legacy_i8042_state,
    pub rtc: ::core::ffi::c_int,
    pub warm_reset: ::core::ffi::c_int,
    pub no_vga: ::core::ffi::c_int,
    pub reserve_bios_regions: ::core::ffi::c_int,
    pub devices: x86_legacy_devices,
}

#[repr(C)]
pub struct x86_hyper_runtime {
    pub pin_vcpu: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_int)>,
    pub sev_es_hcall_prepare: Option<unsafe extern "C" fn(ghcb: *mut ghcb, regs: *mut pt_regs)>,
    pub sev_es_hcall_finish: Option<unsafe extern "C" fn(ghcb: *mut ghcb, regs: *mut pt_regs) -> bool>,
    pub is_private_mmio: Option<unsafe extern "C" fn(addr: u64) -> bool>,
}

#[repr(C)]
pub struct x86_platform_ops {
    pub calibrate_cpu: Option<unsafe extern "C" fn() -> ::core::ffi::c_ulong>,
    pub calibrate_tsc: Option<unsafe extern "C" fn() -> ::core::ffi::c_ulong>,
    pub get_wallclock: Option<unsafe extern "C" fn(ts: *mut timespec64)>,
    pub set_wallclock: Option<unsafe extern "C" fn(ts: *const timespec64) -> ::core::ffi::c_int>,
    pub iommu_shutdown: Option<unsafe extern "C" fn()>,
    pub is_untracked_pat_range: Option<unsafe extern "C" fn(start: u64, end: u64) -> bool>,
    pub nmi_init: Option<unsafe extern "C" fn()>,
    pub get_nmi_reason: Option<unsafe extern "C" fn() -> u8>,
    pub save_sched_clock_state: Option<unsafe extern "C" fn()>,
    pub restore_sched_clock_state: Option<unsafe extern "C" fn()>,
    pub apic_post_init: Option<unsafe extern "C" fn()>,
    pub legacy: x86_legacy_features,
    pub set_legacy_features: Option<unsafe extern "C" fn()>,
    pub realmode_reserve: Option<unsafe extern "C" fn()>,
    pub realmode_init: Option<unsafe extern "C" fn()>,
    pub hyper: x86_hyper_runtime,
    pub guest: x86_guest,
}

#[repr(C)]
pub struct x86_apic_ops {
    pub io_apic_read: Option<unsafe extern "C" fn(apic: u32, reg: u32) -> u32>,
    pub restore: Option<unsafe extern "C" fn()>,
}

extern "C" {
    pub static mut x86_init: x86_init_ops;
    pub static mut x86_cpuinit: x86_cpuinit_ops;
    pub static mut x86_platform: x86_platform_ops;
    pub static mut x86_msi: x86_msi_ops;
    pub static mut x86_apic_ops: x86_apic_ops;

    pub fn x86_early_init_platform_quirks();
    pub fn x86_init_noop();
    pub fn x86_init_uint_noop(unused: u32);
    pub fn bool_x86_init_noop() -> bool;
    pub fn x86_op_int_noop(cpu: ::core::ffi::c_int);
    pub fn x86_pnpbios_disabled() -> bool;
    pub fn set_rtc_noop(now: *const timespec64) -> ::core::ffi::c_int;
    pub fn get_rtc_noop(now: *mut timespec64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
