// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit.

pub static mut x2apic_phys: i32 = 0;

pub static mut x2apic_max_apicid: u32 = u32::MAX;

pub unsafe fn x2apic_set_max_apicid(apicid: u32) {
    x2apic_max_apicid = apicid;
    if (*apic).x2apic_set_max_apicid {
        (*apic).max_apic_id = apicid;
    }
}

unsafe fn set_x2apic_phys_mode(_arg: *mut core::ffi::c_char) -> i32 {
    x2apic_phys = 1;
    0
}

// C registration: early_param("x2apic_phys", set_x2apic_phys_mode);

unsafe fn x2apic_fadt_phys() -> bool {
    // CONFIG_ACPI conditional preserved from the source.
    #[cfg(CONFIG_ACPI)]
    {
        if ((*core::ptr::addr_of!(acpi_gbl_FADT.header).cast::<FadtHeader>()).revision
            >= FADT2_REVISION_ID)
            && ((*core::ptr::addr_of!(acpi_gbl_FADT.flags)) & ACPI_FADT_APIC_PHYSICAL != 0)
        {
            printk(KERN_DEBUG, "System requires x2apic physical mode\n");
            return true;
        }
    }
    false
}

unsafe fn x2apic_acpi_madt_oem_check(
    _oem_id: *mut core::ffi::c_char,
    _oem_table_id: *mut core::ffi::c_char,
) -> i32 {
    (x2apic_enabled() && (x2apic_phys != 0 || x2apic_fadt_phys())) as i32
}

unsafe fn x2apic_send_IPI(cpu: i32, vector: i32) {
    let dest: u32 = per_cpu(x86_cpu_to_apicid, cpu);
    // x2apic MSRs are special and need a special fence:
    weak_wrmsr_fence();
    __x2apic_send_IPI_dest(dest, vector, APIC_DEST_PHYSICAL);
}

unsafe fn __x2apic_send_IPI_mask(
    mask: *const cpumask,
    vector: i32,
    apic_dest: i32,
) {
    let mut query_cpu: usize;
    let this_cpu: usize;
    let mut flags: usize = 0;

    // x2apic MSRs are special and need a special fence:
    weak_wrmsr_fence();
    local_irq_save(&mut flags);

    this_cpu = smp_processor_id();
    for_each_cpu!(query_cpu, mask, {
        if apic_dest == APIC_DEST_ALLBUT && this_cpu == query_cpu {
            continue;
        }
        __x2apic_send_IPI_dest(
            per_cpu(x86_cpu_to_apicid, query_cpu),
            vector,
            APIC_DEST_PHYSICAL,
        );
    });
    local_irq_restore(flags);
}

unsafe fn x2apic_send_IPI_mask(mask: *const cpumask, vector: i32) {
    __x2apic_send_IPI_mask(mask, vector, APIC_DEST_ALLINC);
}

unsafe fn x2apic_send_IPI_mask_allbutself(mask: *const cpumask, vector: i32) {
    __x2apic_send_IPI_mask(mask, vector, APIC_DEST_ALLBUT);
}

unsafe fn __x2apic_send_IPI_shorthand(vector: i32, which: u32) {
    // x2apic MSRs are special and need a special fence:
    weak_wrmsr_fence();
    native_x2apic_icr_write(__prepare_ICR(which, vector, 0), 0);
}

pub unsafe fn x2apic_send_IPI_allbutself(vector: i32) {
    __x2apic_send_IPI_shorthand(vector, APIC_DEST_ALLBUT);
}

pub unsafe fn x2apic_send_IPI_all(vector: i32) {
    __x2apic_send_IPI_shorthand(vector, APIC_DEST_ALLINC);
}

pub unsafe fn x2apic_send_IPI_self(vector: i32) {
    apic_write(APIC_SELF_IPI, vector);
}

unsafe fn x2apic_phys_probe() -> i32 {
    if !x2apic_mode {
        return 0;
    }
    if x2apic_phys != 0 || x2apic_fadt_phys() {
        return 1;
    }
    (core::ptr::eq(apic, core::ptr::addr_of_mut!(apic_x2apic_phys))).into()
}

pub unsafe fn x2apic_get_apic_id(id: u32) -> u32 {
    id
}

static mut apic_x2apic_phys: apic = apic {
    name: "physical x2apic\0".as_ptr() as *const core::ffi::c_char,
    probe: Some(x2apic_phys_probe),
    acpi_madt_oem_check: Some(x2apic_acpi_madt_oem_check),
    dest_mode_logical: false,
    disable_esr: 0,
    cpu_present_to_apicid: Some(default_cpu_present_to_apicid),
    max_apic_id: u32::MAX,
    x2apic_set_max_apicid: true,
    get_apic_id: Some(x2apic_get_apic_id),
    calc_dest_apicid: Some(apic_default_calc_apicid),
    send_IPI: Some(x2apic_send_IPI),
    send_IPI_mask: Some(x2apic_send_IPI_mask),
    send_IPI_mask_allbutself: Some(x2apic_send_IPI_mask_allbutself),
    send_IPI_allbutself: Some(x2apic_send_IPI_allbutself),
    send_IPI_all: Some(x2apic_send_IPI_all),
    send_IPI_self: Some(x2apic_send_IPI_self),
    nmi_to_offline_cpu: true,
    read: Some(native_apic_msr_read),
    write: Some(native_apic_msr_write),
    eoi: Some(native_apic_msr_eoi),
    icr_read: Some(native_x2apic_icr_read),
    icr_write: Some(native_x2apic_icr_write),
};

// C registration: apic_driver(apic_x2apic_phys);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
