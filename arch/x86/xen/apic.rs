// SPDX-License-Identifier: GPL-2.0
//
// C includes translated as external dependencies supplied by other files:
// linux/init.h, linux/thread_info.h, asm/x86_init.h, asm/apic.h,
// asm/io_apic.h, asm/xen/hypercall.h, xen/xen.h,
// xen/interface/physdev.h, and xen-ops.h.

unsafe fn xen_io_apic_read(apic: ::core::ffi::c_uint, reg: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut apic_op: physdev_apic = ::core::mem::zeroed();
    apic_op.apic_physbase = mpc_ioapic_addr(apic);
    apic_op.reg = reg;
    let ret = HYPERVISOR_physdev_op(PHYSDEVOP_apic_read, &mut apic_op);
    if ret == 0 {
        return apic_op.value;
    }

    // fallback to return an emulated IO_APIC values
    if reg == 0x1 {
        return 0x00170020;
    } else if reg == 0x0 {
        return apic << 24;
    }

    0xfd
}

unsafe fn xen_get_apic_id(x: u32) -> u32 {
    (x >> 24) & 0xFFu32
}

unsafe fn xen_apic_read(reg: u32) -> u32 {
    let mut op: xen_platform_op = xen_platform_op {
        cmd: XENPF_get_cpuinfo,
        interface_version: XENPF_INTERFACE_VERSION,
        ..::core::mem::zeroed()
    };

    if reg == APIC_LVR {
        return 0x14;
    }
    if reg != APIC_ID {
        return 0;
    }

    let cpu = smp_processor_id();
    if !xen_initial_domain() {
        return if cpu != 0 { cpuid_to_apicid[cpu as usize] << 24 } else { 0 };
    }

    op.u.pcpu_info.xen_cpuid = cpu;

    let ret = HYPERVISOR_platform_op(&mut op);
    if ret != 0 {
        op.u.pcpu_info.apic_id = BAD_APICID;
    }

    op.u.pcpu_info.apic_id << 24
}

unsafe fn xen_apic_write(reg: u32, val: u32) {
    if reg == APIC_LVTPC {
        let _ = pmu_apic_update(reg);
        return;
    }

    // Warn to see if there's any stray references
    WARN(1, "register: %x, value: %x\n", reg, val);
}

unsafe fn xen_apic_eoi() {
    WARN_ON_ONCE(1);
}

unsafe fn xen_apic_icr_read() -> u64 {
    0
}

unsafe fn xen_apic_icr_write(_low: u32, _id: u32) {
    // Warn to see if there's any stray references
    WARN_ON(1);
}

unsafe fn xen_apic_probe_pv() -> ::core::ffi::c_int {
    if xen_pv_domain() {
        return 1;
    }
    0
}

unsafe fn xen_madt_oem_check(_oem_id: *mut ::core::ffi::c_char, _oem_table_id: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    xen_pv_domain() as ::core::ffi::c_int
}

unsafe fn xen_cpu_present_to_apicid(cpu: ::core::ffi::c_int) -> u32 {
    if cpu_present(cpu) {
        cpu_data(cpu).topo.apicid
    } else {
        BAD_APICID
    }
}

static mut xen_pv_apic: apic = apic {
    name: "Xen PV\0".as_ptr() as *const ::core::ffi::c_char,
    probe: Some(xen_apic_probe_pv),
    acpi_madt_oem_check: Some(xen_madt_oem_check),

    // .delivery_mode and .dest_mode_logical not used by XENPV

    disable_esr: 0,
    cpu_present_to_apicid: Some(xen_cpu_present_to_apicid),
    max_apic_id: UINT_MAX,
    get_apic_id: Some(xen_get_apic_id),
    calc_dest_apicid: Some(apic_flat_calc_apicid),

    #[cfg(feature = "CONFIG_SMP")]
    send_IPI_mask: Some(xen_send_IPI_mask),
    #[cfg(feature = "CONFIG_SMP")]
    send_IPI_mask_allbutself: Some(xen_send_IPI_mask_allbutself),
    #[cfg(feature = "CONFIG_SMP")]
    send_IPI_allbutself: Some(xen_send_IPI_allbutself),
    #[cfg(feature = "CONFIG_SMP")]
    send_IPI_all: Some(xen_send_IPI_all),
    #[cfg(feature = "CONFIG_SMP")]
    send_IPI_self: Some(xen_send_IPI_self),
    read: Some(xen_apic_read),
    write: Some(xen_apic_write),
    eoi: Some(xen_apic_eoi),
    icr_read: Some(xen_apic_icr_read),
    icr_write: Some(xen_apic_icr_write),
    ..::core::mem::zeroed()
};

// apic_driver(xen_pv_apic);

unsafe fn xen_init_apic() {
    x86_apic_ops.io_apic_read = Some(xen_io_apic_read);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
