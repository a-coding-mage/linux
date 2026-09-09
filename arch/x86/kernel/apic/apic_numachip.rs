/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Numascale NumaConnect-Specific APIC Code
 *
 * Copyright (C) 2011 Numascale AS. All rights reserved.
 *
 * Send feedback to <support@numascale.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut numachip_system: u8 = 0;
static mut numachip_apic_icr_write: Option<unsafe fn(i32, u32)> = None;

unsafe fn numachip1_get_apic_id(x: u32) -> u32 {
    let mut value: u64 = 0;
    let mut id = (x >> 24) & 0xff;

    if cpu_feature_enabled(X86_FEATURE_NODEID_MSR) {
        rdmsrq(MSR_FAM10H_NODE_ID, &mut value);
        id |= ((value << 2) & 0xff00) as u32;
    }

    id
}

unsafe fn numachip2_get_apic_id(x: u32) -> u32 {
    let mut mcfg: u64 = 0;

    rdmsrq(MSR_FAM10H_MMIO_CONF_BASE, &mut mcfg);
    (((mcfg >> (28 - 8)) & 0xfff00) as u32) | (x >> 24)
}

unsafe fn numachip1_apic_icr_write(apicid: i32, val: u32) {
    write_lcsr(CSR_G3_EXT_IRQ_GEN, ((apicid << 16) as u32) | val);
}

unsafe fn numachip2_apic_icr_write(apicid: i32, val: u32) {
    numachip2_write32_lcsr(NUMACHIP2_APIC_ICR, ((apicid << 12) as u32) | val);
}

unsafe fn numachip_wakeup_secondary(
    phys_apicid: u32,
    start_rip: usize,
    _cpu: u32,
) -> i32 {
    if let Some(write) = numachip_apic_icr_write {
        write(phys_apicid as i32, APIC_DM_INIT);
        write(phys_apicid as i32, APIC_DM_STARTUP | (start_rip >> 12) as u32);
    }

    0
}

unsafe fn numachip_send_IPI_one(cpu: i32, vector: i32) {
    let mut local_apicid: i32;
    let apicid = per_cpu(x86_cpu_to_apicid, cpu);
    let dmode: u32;

    preempt_disable();
    local_apicid = __this_cpu_read(x86_cpu_to_apicid);

    /* Send via local APIC where non-local part matches */
    if ((apicid ^ local_apicid) >> NUMACHIP_LAPIC_BITS) == 0 {
        let mut flags: usize = 0;

        local_irq_save(&mut flags);
        __default_send_IPI_dest_field(apicid, vector, APIC_DEST_PHYSICAL);
        local_irq_restore(flags);
        preempt_enable();
        return;
    }
    preempt_enable();

    dmode = if vector == NMI_VECTOR { APIC_DM_NMI } else { APIC_DM_FIXED };
    if let Some(write) = numachip_apic_icr_write {
        write(apicid, dmode | vector as u32);
    }
}

unsafe fn numachip_send_IPI_mask(mask: *const cpumask, vector: i32) {
    for_each_cpu(|cpu| numachip_send_IPI_one(cpu, vector), mask);
}

unsafe fn numachip_send_IPI_mask_allbutself(mask: *const cpumask, vector: i32) {
    let this_cpu = smp_processor_id();

    for_each_cpu(|cpu| {
        if cpu != this_cpu {
            numachip_send_IPI_one(cpu, vector);
        }
    }, mask);
}

unsafe fn numachip_send_IPI_allbutself(vector: i32) {
    let this_cpu = smp_processor_id();

    for_each_online_cpu(|cpu| {
        if cpu != this_cpu {
            numachip_send_IPI_one(cpu, vector);
        }
    });
}

unsafe fn numachip_send_IPI_all(vector: i32) {
    numachip_send_IPI_mask(cpu_online_mask, vector);
}

unsafe fn numachip_send_IPI_self(vector: i32) {
    apic_write(APIC_SELF_IPI, vector);
}

unsafe fn numachip1_probe() -> i32 {
    (apic == &apic_numachip1) as i32
}

unsafe fn numachip2_probe() -> i32 {
    (apic == &apic_numachip2) as i32
}

unsafe fn fixup_cpu_id(c: *mut cpuinfo_x86, node: i32) {
    let mut val: u64 = 0;
    let mut nodes: u32 = 1;

    (*c).topo.llc_id = node;

    /* Account for nodes per socket in multi-core-module processors */
    if boot_cpu_has(X86_FEATURE_NODEID_MSR) {
        rdmsrq(MSR_FAM10H_NODE_ID, &mut val);
        nodes = (((val >> 3) & 7) + 1) as u32;
    }

    (*c).topo.pkg_id = node / nodes as i32;
}

unsafe fn numachip_system_init() -> i32 {
    /* Map the LCSR area and set up the apic_icr_write function */
    match numachip_system {
        1 => {
            init_extra_mapping_uc(NUMACHIP_LCSR_BASE, NUMACHIP_LCSR_SIZE);
            numachip_apic_icr_write = Some(numachip1_apic_icr_write);
        }
        2 => {
            init_extra_mapping_uc(NUMACHIP2_LCSR_BASE, NUMACHIP2_LCSR_SIZE);
            numachip_apic_icr_write = Some(numachip2_apic_icr_write);
        }
        _ => return 0,
    }

    x86_cpuinit.fixup_cpu_id = Some(fixup_cpu_id);
    x86_init.pci.arch_init = Some(pci_numachip_init);

    0
}
early_initcall!(numachip_system_init);

unsafe fn numachip1_acpi_madt_oem_check(oem_id: *mut i8, oem_table_id: *mut i8) -> i32 {
    if strncmp(oem_id, b"NUMASC\0".as_ptr() as *const i8, 6) != 0
        || strncmp(oem_table_id, b"NCONNECT\0".as_ptr() as *const i8, 8) != 0
    {
        return 0;
    }

    numachip_system = 1;
    1
}

unsafe fn numachip2_acpi_madt_oem_check(oem_id: *mut i8, oem_table_id: *mut i8) -> i32 {
    if strncmp(oem_id, b"NUMASC\0".as_ptr() as *const i8, 6) != 0
        || strncmp(oem_table_id, b"NCONECT2\0".as_ptr() as *const i8, 8) != 0
    {
        return 0;
    }

    numachip_system = 2;
    1
}

static apic_numachip1: apic = apic {
    name: "NumaConnect system",
    probe: Some(numachip1_probe),
    acpi_madt_oem_check: Some(numachip1_acpi_madt_oem_check),
    dest_mode_logical: false,
    disable_esr: 0,
    cpu_present_to_apicid: Some(default_cpu_present_to_apicid),
    max_apic_id: UINT_MAX,
    get_apic_id: Some(numachip1_get_apic_id),
    calc_dest_apicid: Some(apic_default_calc_apicid),
    send_IPI: Some(numachip_send_IPI_one),
    send_IPI_mask: Some(numachip_send_IPI_mask),
    send_IPI_mask_allbutself: Some(numachip_send_IPI_mask_allbutself),
    send_IPI_allbutself: Some(numachip_send_IPI_allbutself),
    send_IPI_all: Some(numachip_send_IPI_all),
    send_IPI_self: Some(numachip_send_IPI_self),
    wakeup_secondary_cpu: Some(numachip_wakeup_secondary),
    read: Some(native_apic_mem_read),
    write: Some(native_apic_mem_write),
    eoi: Some(native_apic_mem_eoi),
    icr_read: Some(native_apic_icr_read),
    icr_write: Some(native_apic_icr_write),
    ..unsafe { core::mem::zeroed() }
};
apic_driver!(apic_numachip1);

static apic_numachip2: apic = apic {
    name: "NumaConnect2 system",
    probe: Some(numachip2_probe),
    acpi_madt_oem_check: Some(numachip2_acpi_madt_oem_check),
    dest_mode_logical: false,
    disable_esr: 0,
    cpu_present_to_apicid: Some(default_cpu_present_to_apicid),
    max_apic_id: UINT_MAX,
    get_apic_id: Some(numachip2_get_apic_id),
    calc_dest_apicid: Some(apic_default_calc_apicid),
    send_IPI: Some(numachip_send_IPI_one),
    send_IPI_mask: Some(numachip_send_IPI_mask),
    send_IPI_mask_allbutself: Some(numachip_send_IPI_mask_allbutself),
    send_IPI_allbutself: Some(numachip_send_IPI_allbutself),
    send_IPI_all: Some(numachip_send_IPI_all),
    send_IPI_self: Some(numachip_send_IPI_self),
    wakeup_secondary_cpu: Some(numachip_wakeup_secondary),
    read: Some(native_apic_mem_read),
    write: Some(native_apic_mem_write),
    eoi: Some(native_apic_mem_eoi),
    icr_read: Some(native_apic_icr_read),
    icr_write: Some(native_apic_icr_write),
    ..unsafe { core::mem::zeroed() }
};
apic_driver!(apic_numachip2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
