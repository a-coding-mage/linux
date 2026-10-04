// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2009 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
 *
 * For licencing details see kernel-base/COPYING
 */
// Rust owner of x86_init.c. All ABI layouts come from the configured headers.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]

// Clang's __init suppresses sanitizer coverage on four functions only. Rust
// 1.85 cannot express no_sanitize(coverage); do not silently instrument those
// functions or turn off coverage for this object's runtime callbacks.
#[cfg(all(CONFIG_CC_IS_CLANG, CONFIG_KCOV))]
compile_error!(
    "x86_init Rust ownership requires per-init Clang KCOV suppression; Rust 1.85 cannot express it"
);
#[cfg(CONFIG_GCC_PLUGIN_LATENT_ENTROPY)]
compile_error!(
    "x86_init Rust ownership does not implement the GCC __init latent-entropy instrumentation"
);
#[cfg(CONFIG_KSTACK_ERASE)]
compile_error!(
    "x86_init Rust ownership requires matching per-init and runtime KSTACK_ERASE instrumentation"
);

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/x86_platform_callbacks_generated.rs"
    ));
}
use bindings as b;
use kernel::ffi::{c_int, c_uint, c_ulong};

#[path = "../../../rust/ffi_export.rs"]
mod ffi_export;

// Preserve the header aliases rather than introducing forwarding callbacks:
// these aliases must retain exactly the original function-pointer identities.
#[cfg(not(CONFIG_X86_MPPARSE))]
use self::{
    x86_init_noop as mpparse_find_mptable, x86_init_noop as mpparse_parse_early_smp_config,
    x86_init_noop as mpparse_parse_smp_config,
};
#[cfg(CONFIG_X86_MPPARSE)]
use b::{mpparse_find_mptable, mpparse_parse_early_smp_config, mpparse_parse_smp_config};

#[cfg(not(CONFIG_X86_LOCAL_APIC))]
use self::{x86_init_noop as setup_boot_APIC_clock, x86_init_noop as setup_secondary_APIC_clock};
#[cfg(CONFIG_X86_LOCAL_APIC)]
use b::{
    apic_intr_mode_init, apic_intr_mode_select, setup_boot_APIC_clock, setup_secondary_APIC_clock,
};
// Exact !CONFIG_X86_LOCAL_APIC static-inline bodies from asm/apic.h.
#[cfg(not(CONFIG_X86_LOCAL_APIC))]
unsafe extern "C" fn apic_intr_mode_select() {}
#[cfg(not(CONFIG_X86_LOCAL_APIC))]
unsafe extern "C" fn apic_intr_mode_init() {}

#[cfg(not(CONFIG_PARAVIRT))]
use self::x86_init_noop as default_banner;
#[cfg(CONFIG_PARAVIRT)]
use b::default_banner;

#[cfg(CONFIG_X86_32)]
use b::native_pagetable_init;
#[cfg(not(CONFIG_X86_32))]
use b::paging_init as native_pagetable_init;

#[cfg(CONFIG_DMI)]
use b::dmi_setup;
// Exact !CONFIG_DMI static-inline body from linux/dmi.h.
#[cfg(not(CONFIG_DMI))]
unsafe extern "C" fn dmi_setup() {}

#[cfg(CONFIG_ACPI)]
use b::{acpi_generic_reduced_hw_init, x86_default_get_root_pointer, x86_default_set_root_pointer};
// Exact !CONFIG_ACPI static-inline bodies from asm/acpi.h.
#[cfg(not(CONFIG_ACPI))]
unsafe extern "C" fn acpi_generic_reduced_hw_init() {}
#[cfg(not(CONFIG_ACPI))]
unsafe extern "C" fn x86_default_set_root_pointer(_addr: u64) {}
#[cfg(not(CONFIG_ACPI))]
unsafe extern "C" fn x86_default_get_root_pointer() -> u64 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn x86_init_noop() {}

#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn x86_init_uint_noop(_unused: c_uint) {}

#[cold]
#[link_section = ".init.text"]
unsafe extern "C" fn iommu_init_noop() -> c_int {
    0
}

unsafe extern "C" fn iommu_shutdown_noop() {}

#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn bool_x86_init_noop() -> bool {
    false
}

#[no_mangle]
pub unsafe extern "C" fn x86_op_int_noop(_cpu: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn set_rtc_noop(_now: *const b::timespec64) -> c_int {
    -(b::RUST_X86_PLATFORM_EINVAL as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn get_rtc_noop(_now: *mut b::timespec64) {}

// The source table contains fixed arrays (name[32], type[32], compatible[128])
// and a data pointer, not a pointer to a compatible string. Construct it through
// the canonical type so its sizes, offsets and alignment remain header-owned.
#[cfg(CONFIG_OF)]
const fn of_cmos_match() -> [b::of_device_id; 2] {
    let mut table: [b::of_device_id; 2] = unsafe { core::mem::zeroed() };
    let compatible = b"motorola,mc146818\0";
    let mut i = 0;
    while i < compatible.len() {
        table[0].compatible[i] = compatible[i] as kernel::ffi::c_char;
        i += 1;
    }
    table
}

#[cfg(CONFIG_OF)]
#[repr(transparent)]
struct OfCmosMatch([b::of_device_id; 2]);
// SAFETY: the immutable table has no mutable pointers; both data fields are NULL.
#[cfg(CONFIG_OF)]
unsafe impl Sync for OfCmosMatch {}

#[cfg(CONFIG_OF)]
#[link_section = ".init.rodata"]
static OF_CMOS_MATCH: OfCmosMatch = OfCmosMatch(of_cmos_match());

// linux/of.h's of_find_matching_node is an inline call of
// of_find_matching_node_and_match(from, matches, NULL). With CONFIG_OF=n the
// latter returns NULL, so the original optimized wallclock body is empty.
#[cold]
#[link_section = ".init.text"]
unsafe extern "C" fn x86_wallclock_init() {
    #[cfg(CONFIG_OF)]
    {
        let node = b::of_find_matching_node_and_match(
            core::ptr::null_mut(),
            core::ptr::addr_of!(OF_CMOS_MATCH.0).cast::<b::of_device_id>(),
            core::ptr::null_mut(),
        );
        if !node.is_null() && !b::of_device_is_available(node) {
            // The source retains its matching-node reference; do not add a put.
            // Raw writes avoid forming references to a mutable exported static.
            core::ptr::addr_of_mut!(x86_platform.get_wallclock).write(Some(get_rtc_noop));
            core::ptr::addr_of_mut!(x86_platform.set_wallclock).write(Some(set_rtc_noop));
        }
    }
}

// The platform setup functions are preset for standard PC hardware. Explicit
// None fields below are precisely the C static initializer's omitted fields or
// the headers' CONFIG-disabled NULL macros, never replacement implementations.
#[no_mangle]
#[link_section = ".init.data"]
pub static mut x86_init: b::x86_init_ops = b::x86_init_ops {
    resources: b::x86_init_resources {
        probe_roms: Some(b::probe_roms),
        reserve_resources: Some(b::reserve_standard_io_resources),
        memory_setup: Some(b::e820__memory_setup_default),
        dmi_setup: Some(dmi_setup),
        // Has to be under 1M so we can execute real-mode AP code.
        realmode_limit: b::RUST_X86_PLATFORM_SZ_1M as c_ulong,
    },
    mpparse: b::x86_init_mpparse {
        setup_ioapic_ids: Some(x86_init_noop),
        find_mptable: Some(mpparse_find_mptable),
        early_parse_smp_cfg: Some(mpparse_parse_early_smp_config),
        parse_smp_cfg: Some(mpparse_parse_smp_config),
    },
    irqs: b::x86_init_irqs {
        pre_vector_init: Some(b::init_ISA_irqs),
        intr_init: Some(b::native_init_IRQ),
        intr_mode_select: Some(apic_intr_mode_select),
        intr_mode_init: Some(apic_intr_mode_init),
        #[cfg(CONFIG_PCI_MSI)]
        create_pci_msi_domain: Some(b::native_create_pci_msi_domain),
        #[cfg(not(CONFIG_PCI_MSI))]
        create_pci_msi_domain: None,
    },
    oem: b::x86_init_oem {
        arch_setup: Some(x86_init_noop),
        banner: Some(default_banner),
    },
    paging: b::x86_init_paging {
        pagetable_init: Some(native_pagetable_init),
    },
    timers: b::x86_init_timers {
        setup_percpu_clockev: Some(setup_boot_APIC_clock),
        timer_init: Some(b::hpet_time_init),
        wallclock_init: Some(x86_wallclock_init),
    },
    iommu: b::x86_init_iommu {
        iommu_init: Some(iommu_init_noop),
    },
    pci: b::x86_init_pci {
        arch_init: None,
        #[cfg(all(CONFIG_PCI, CONFIG_ACPI))]
        init: Some(b::pci_acpi_init),
        #[cfg(all(CONFIG_PCI, not(CONFIG_ACPI)))]
        init: Some(b::pci_legacy_init),
        #[cfg(not(CONFIG_PCI))]
        init: None,
        #[cfg(CONFIG_PCI)]
        init_irq: Some(b::pcibios_irq_init),
        #[cfg(not(CONFIG_PCI))]
        init_irq: None,
        #[cfg(CONFIG_PCI)]
        fixup_irqs: Some(b::pcibios_fixup_irqs),
        #[cfg(not(CONFIG_PCI))]
        fixup_irqs: None,
    },
    hyper: b::x86_hyper_init {
        init_platform: Some(x86_init_noop),
        guest_late_init: Some(x86_init_noop),
        x2apic_available: Some(bool_x86_init_noop),
        msi_ext_dest_id: Some(bool_x86_init_noop),
        init_mem_mapping: Some(x86_init_noop),
        init_after_bootmem: Some(x86_init_noop),
    },
    acpi: b::x86_init_acpi {
        set_root_pointer: Some(x86_default_set_root_pointer),
        get_root_pointer: Some(x86_default_get_root_pointer),
        reduced_hw_early_init: Some(acpi_generic_reduced_hw_init),
    },
};

#[no_mangle]
pub static mut x86_cpuinit: b::x86_cpuinit_ops = b::x86_cpuinit_ops {
    early_percpu_clock_init: Some(x86_init_noop),
    setup_percpu_clockev: Some(setup_secondary_APIC_clock),
    fixup_cpu_id: None,
    parallel_bringup: true,
};

unsafe extern "C" fn default_nmi_init() {}

unsafe extern "C" fn enc_status_change_prepare_noop(
    _vaddr: c_ulong,
    _npages: c_int,
    _enc: bool,
) -> c_int {
    0
}

unsafe extern "C" fn enc_status_change_finish_noop(
    _vaddr: c_ulong,
    _npages: c_int,
    _enc: bool,
) -> c_int {
    0
}

unsafe extern "C" fn enc_tlb_flush_required_noop(_enc: bool) -> bool {
    false
}

unsafe extern "C" fn enc_cache_flush_required_noop() -> bool {
    false
}

unsafe extern "C" fn enc_kexec_begin_noop() {}
unsafe extern "C" fn enc_kexec_finish_noop() {}

unsafe extern "C" fn is_private_mmio_noop(_addr: u64) -> bool {
    false
}

// Exact inline body from asm/e820/api.h, with header-evaluated constants.
unsafe extern "C" fn is_ISA_range(start: u64, end: u64) -> bool {
    start >= b::RUST_X86_PLATFORM_ISA_START_ADDRESS as u64
        && end <= b::RUST_X86_PLATFORM_ISA_END_ADDRESS as u64
}

// asm/mach_traps.h -> asm/shared/io.h's volatile inb(NMI_REASON_PORT).
// No C body or indirect I/O wrapper is required for this instruction boundary.
unsafe extern "C" fn default_get_nmi_reason() -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, {port}",
        port = const b::RUST_X86_PLATFORM_NMI_REASON_PORT,
        lateout("al") value,
        options(nomem, nostack, preserves_flags),
    );
    value
}

#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut x86_platform: b::x86_platform_ops = b::x86_platform_ops {
    calibrate_cpu: Some(b::native_calibrate_cpu_early),
    calibrate_tsc: Some(b::native_calibrate_tsc),
    get_wallclock: Some(b::mach_get_cmos_time),
    set_wallclock: Some(b::mach_set_cmos_time),
    iommu_shutdown: Some(iommu_shutdown_noop),
    is_untracked_pat_range: Some(is_ISA_range),
    nmi_init: Some(default_nmi_init),
    get_nmi_reason: Some(default_get_nmi_reason),
    save_sched_clock_state: Some(b::tsc_save_sched_clock_state),
    restore_sched_clock_state: Some(b::tsc_restore_sched_clock_state),
    apic_post_init: None,
    legacy: b::x86_legacy_features {
        i8042: b::X86_LEGACY_I8042_PLATFORM_ABSENT,
        rtc: 0,
        warm_reset: 0,
        no_vga: 0,
        reserve_bios_regions: 0,
        devices: b::x86_legacy_devices { pnpbios: 0 },
    },
    set_legacy_features: None,
    realmode_reserve: Some(b::reserve_real_mode),
    realmode_init: Some(b::init_real_mode),
    hyper: b::x86_hyper_runtime {
        pin_vcpu: Some(x86_op_int_noop),
        sev_es_hcall_prepare: None,
        sev_es_hcall_finish: None,
        is_private_mmio: Some(is_private_mmio_noop),
    },
    guest: b::x86_guest {
        enc_status_change_prepare: Some(enc_status_change_prepare_noop),
        enc_status_change_finish: Some(enc_status_change_finish_noop),
        enc_tlb_flush_required: Some(enc_tlb_flush_required_noop),
        enc_cache_flush_required: Some(enc_cache_flush_required_noop),
        enc_kexec_begin: Some(enc_kexec_begin_noop),
        enc_kexec_finish: Some(enc_kexec_finish_noop),
    },
};

ffi_export::export_symbol!(x86_platform, x86_platform, "GPL", "");

#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut x86_apic_ops: b::x86_apic_ops = b::x86_apic_ops {
    #[cfg(CONFIG_X86_IO_APIC)]
    io_apic_read: Some(b::native_io_apic_read),
    #[cfg(not(CONFIG_X86_IO_APIC))]
    io_apic_read: None,
    #[cfg(CONFIG_X86_IO_APIC)]
    restore: Some(b::native_restore_boot_irq_mode),
    #[cfg(not(CONFIG_X86_IO_APIC))]
    restore: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
