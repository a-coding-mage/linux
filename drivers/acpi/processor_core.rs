// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005 Intel Corporation
 * Copyright (C) 2009 Hewlett-Packard Development Company, L.P.
 *
 *	Alex Chiang <achiang@hp.com>
 *	- Unified x86/ia64 implementations
 *
 * I/O APIC hotplug support
 *	Yinghai Lu <yinghai@kernel.org>
 *	Jiang Liu <jiang.liu@intel.com>
 */

unsafe fn get_madt_table() -> *mut acpi_table_madt {
    static mut MADT: *mut acpi_table_madt = core::ptr::null_mut();
    static mut READ_MADT: i32 = 0;

    if READ_MADT == 0 {
        if acpi_failure(acpi_get_table(ACPI_SIG_MADT, 0,
            &mut MADT as *mut _ as *mut *mut acpi_table_header)) {
            MADT = core::ptr::null_mut();
        }
        READ_MADT += 1;
    }
    MADT
}

unsafe fn map_lapic_id(entry: *mut acpi_subtable_header, acpi_id: u32,
                       apic_id: *mut phys_cpuid_t) -> i32 {
    let lapic = (entry as *mut u8).sub(offset_of!(acpi_madt_local_apic, header))
        as *mut acpi_madt_local_apic;
    if (*lapic).lapic_flags & ACPI_MADT_ENABLED == 0 { return -ENODEV; }
    if (*lapic).processor_id != acpi_id { return -EINVAL; }
    *apic_id = (*lapic).id as phys_cpuid_t;
    0
}

unsafe fn map_x2apic_id(entry: *mut acpi_subtable_header, device_declaration: i32,
                        acpi_id: u32, apic_id: *mut phys_cpuid_t) -> i32 {
    let apic = (entry as *mut u8).sub(offset_of!(acpi_madt_local_x2apic, header))
        as *mut acpi_madt_local_x2apic;
    if (*apic).lapic_flags & ACPI_MADT_ENABLED == 0 { return -ENODEV; }
    if (*apic).uid == acpi_id && (device_declaration != 0 || acpi_id < 255) {
        *apic_id = (*apic).local_apic_id as phys_cpuid_t;
        return 0;
    }
    -EINVAL
}

unsafe fn map_lsapic_id(entry: *mut acpi_subtable_header, device_declaration: i32,
                        acpi_id: u32, apic_id: *mut phys_cpuid_t) -> i32 {
    let lsapic = (entry as *mut u8).sub(offset_of!(acpi_madt_local_sapic, header))
        as *mut acpi_madt_local_sapic;
    if (*lsapic).lapic_flags & ACPI_MADT_ENABLED == 0 { return -ENODEV; }
    if device_declaration != 0 {
        if (*entry).length < 16 || (*lsapic).uid != acpi_id { return -EINVAL; }
    } else if (*lsapic).processor_id != acpi_id { return -EINVAL; }
    *apic_id = (((*lsapic).id as phys_cpuid_t) << 8) | (*lsapic).eid as phys_cpuid_t;
    0
}

/* Retrieve the ARM CPU physical identifier (MPIDR). */
unsafe fn map_gicc_mpidr(entry: *mut acpi_subtable_header, device_declaration: i32,
                         acpi_id: u32, mpidr: *mut phys_cpuid_t) -> i32 {
    let gicc = (entry as *mut u8).sub(offset_of!(acpi_madt_generic_interrupt, header))
        as *mut acpi_madt_generic_interrupt;
    if (*gicc).flags & (ACPI_MADT_ENABLED | ACPI_MADT_GICC_ONLINE_CAPABLE) == 0 {
        return -ENODEV;
    }
    if device_declaration != 0 && (*gicc).uid == acpi_id {
        *mpidr = (*gicc).arm_mpidr as phys_cpuid_t;
        return 0;
    }
    -EINVAL
}

/* Retrieve the RISC-V hartid for the processor. */
unsafe fn map_rintc_hartid(entry: *mut acpi_subtable_header, device_declaration: i32,
                           acpi_id: u32, hartid: *mut phys_cpuid_t) -> i32 {
    let rintc = (entry as *mut u8).sub(offset_of!(acpi_madt_rintc, header))
        as *mut acpi_madt_rintc;
    if (*rintc).flags & ACPI_MADT_ENABLED == 0 { return -ENODEV; }
    if device_declaration != 0 && (*rintc).uid == acpi_id {
        *hartid = (*rintc).hart_id as phys_cpuid_t;
        return 0;
    }
    -EINVAL
}

/* Retrieve LoongArch CPU physical id. */
unsafe fn map_core_pic_id(entry: *mut acpi_subtable_header, device_declaration: i32,
                          acpi_id: u32, phys_id: *mut phys_cpuid_t) -> i32 {
    let core_pic = (entry as *mut u8).sub(offset_of!(acpi_madt_core_pic, header))
        as *mut acpi_madt_core_pic;
    if (*core_pic).flags & ACPI_MADT_ENABLED == 0 { return -ENODEV; }
    if device_declaration != 0 && (*core_pic).processor_id == acpi_id {
        *phys_id = (*core_pic).core_id as phys_cpuid_t;
        return 0;
    }
    -EINVAL
}

unsafe fn map_madt_entry(madt: *mut acpi_table_madt, typ: i32, acpi_id: u32) -> phys_cpuid_t {
    let mut phys_id = PHYS_CPUID_INVALID;
    if madt.is_null() { return phys_id; }
    let entry = madt as usize;
    let madt_end = entry + (*madt).header.length as usize;
    let mut entry = entry + core::mem::size_of::<acpi_table_madt>();
    while entry + core::mem::size_of::<acpi_subtable_header>() < madt_end {
        let header = entry as *mut acpi_subtable_header;
        let ret = match (*header).type_ {
            ACPI_MADT_TYPE_LOCAL_APIC => map_lapic_id(header, acpi_id, &mut phys_id),
            ACPI_MADT_TYPE_LOCAL_X2APIC => map_x2apic_id(header, typ, acpi_id, &mut phys_id),
            ACPI_MADT_TYPE_LOCAL_SAPIC => map_lsapic_id(header, typ, acpi_id, &mut phys_id),
            ACPI_MADT_TYPE_GENERIC_INTERRUPT => map_gicc_mpidr(header, typ, acpi_id, &mut phys_id),
            ACPI_MADT_TYPE_RINTC => map_rintc_hartid(header, typ, acpi_id, &mut phys_id),
            ACPI_MADT_TYPE_CORE_PIC => map_core_pic_id(header, typ, acpi_id, &mut phys_id),
            _ => -EINVAL,
        };
        if ret == 0 { break; }
        entry += (*header).length as usize;
    }
    phys_id
}

pub unsafe fn acpi_map_madt_entry(acpi_id: u32) -> phys_cpuid_t {
    let mut madt: *mut acpi_table_madt = core::ptr::null_mut();
    acpi_get_table(ACPI_SIG_MADT, 0, &mut madt as *mut _ as *mut *mut acpi_table_header);
    if madt.is_null() { return PHYS_CPUID_INVALID; }
    let rv = map_madt_entry(madt, 1, acpi_id);
    acpi_put_table(madt as *mut acpi_table_header);
    rv
}

pub unsafe fn acpi_get_madt_revision() -> i32 {
    let mut madt: *mut acpi_table_header = core::ptr::null_mut();
    if acpi_failure(acpi_get_table(ACPI_SIG_MADT, 0, &mut madt)) { return -EINVAL; }
    let revision = (*madt).revision as i32;
    acpi_put_table(madt);
    revision
}

unsafe fn map_mat_entry(handle: acpi_handle, typ: i32, acpi_id: u32) -> phys_cpuid_t {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let mut phys_id = PHYS_CPUID_INVALID;
    if acpi_failure(acpi_evaluate_object(handle, b"_MAT\0".as_ptr() as _, core::ptr::null_mut(), &mut buffer)) { return phys_id; }
    if buffer.length == 0 || buffer.pointer.is_null() { return phys_id; }
    let obj = buffer.pointer as *mut acpi_object;
    if (*obj).type_ != ACPI_TYPE_BUFFER || (*obj).buffer.length < core::mem::size_of::<acpi_subtable_header>() { kfree(buffer.pointer); return phys_id; }
    let header = (*obj).buffer.pointer as *mut acpi_subtable_header;
    match (*header).type_ {
        ACPI_MADT_TYPE_LOCAL_APIC => { map_lapic_id(header, acpi_id, &mut phys_id); },
        ACPI_MADT_TYPE_LOCAL_SAPIC => { map_lsapic_id(header, typ, acpi_id, &mut phys_id); },
        ACPI_MADT_TYPE_LOCAL_X2APIC => { map_x2apic_id(header, typ, acpi_id, &mut phys_id); },
        ACPI_MADT_TYPE_GENERIC_INTERRUPT => { map_gicc_mpidr(header, typ, acpi_id, &mut phys_id); },
        ACPI_MADT_TYPE_CORE_PIC => { map_core_pic_id(header, typ, acpi_id, &mut phys_id); },
        _ => {}
    }
    kfree(buffer.pointer);
    phys_id
}

pub unsafe fn acpi_get_phys_id(handle: acpi_handle, typ: i32, acpi_id: u32) -> phys_cpuid_t {
    let mut phys_id = map_mat_entry(handle, typ, acpi_id);
    if invalid_phys_cpuid(phys_id) { phys_id = map_madt_entry(get_madt_table(), typ, acpi_id); }
    phys_id
}

pub unsafe fn acpi_map_cpuid(phys_id: phys_cpuid_t, acpi_id: u32) -> i32 {
    if invalid_phys_cpuid(phys_id) {
        if nr_cpu_ids <= 1 && acpi_id == 0 { return acpi_id as i32; }
        return -EINVAL;
    }
    // CONFIG_SMP: for_each_possible_cpu(i) and cpu_physical_id(i) are supplied externally.
    #[cfg(CONFIG_SMP)]
    for i in 0..nr_cpu_ids {
        if cpu_physical_id(i) == phys_id { return i as i32; }
    }
    #[cfg(not(CONFIG_SMP))]
    if phys_id == 0 { return phys_id as i32; }
    -ENODEV
}

pub unsafe fn acpi_get_cpuid(handle: acpi_handle, typ: i32, acpi_id: u32) -> i32 {
    acpi_map_cpuid(acpi_get_phys_id(handle, typ, acpi_id), acpi_id)
}

// CONFIG_ACPI_HOTPLUG_IOAPIC contains the IOAPIC parsing helpers and public entry point.
#[cfg(CONFIG_ACPI_HOTPLUG_IOAPIC)]
unsafe fn madt_entry_is_valid(entry: *mut acpi_subtable_header, end: usize) -> bool {
    let start = entry as usize;
    start < end && end - start >= core::mem::size_of::<acpi_subtable_header>() &&
        (*entry).length as usize >= core::mem::size_of::<acpi_subtable_header>() &&
        (*entry).length as usize <= end - start
}

#[cfg(CONFIG_ACPI_HOTPLUG_IOAPIC)]
unsafe fn get_ioapic_id(entry: *mut acpi_subtable_header, end: usize, gsi_base: u32,
                        phys_addr: *mut u64, ioapic_id: *mut i32) -> i32 {
    let ioapic = entry as *mut acpi_madt_io_apic;
    if !madt_entry_is_valid(entry, end) || bad_madt_entry(ioapic, end) { return 0; }
    if (*ioapic).global_irq_base != gsi_base { return 0; }
    *phys_addr = (*ioapic).address as u64;
    *ioapic_id = (*ioapic).id as i32;
    1
}

#[cfg(CONFIG_ACPI_HOTPLUG_IOAPIC)]
unsafe fn parse_madt_ioapic_entry(gsi_base: u32, phys_addr: *mut u64) -> i32 {
    let madt = get_madt_table();
    let mut apic_id = -1;
    if madt.is_null() || (*madt).header.length as usize < core::mem::size_of::<acpi_table_madt>() { return apic_id; }
    let start = madt as usize;
    let end = start + (*madt).header.length as usize;
    let mut entry = start + core::mem::size_of::<acpi_table_madt>();
    while madt_entry_is_valid(entry as *mut acpi_subtable_header, end) {
        let hdr = entry as *mut acpi_subtable_header;
        if (*hdr).type_ == ACPI_MADT_TYPE_IO_APIC && get_ioapic_id(hdr, end, gsi_base, phys_addr, &mut apic_id) != 0 { break; }
        entry += (*hdr).length as usize;
    }
    apic_id
}

#[cfg(CONFIG_ACPI_HOTPLUG_IOAPIC)]
unsafe fn parse_mat_ioapic_entry(handle: acpi_handle, gsi_base: u32, phys_addr: *mut u64) -> i32 {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let mut apic_id = -1;
    if !acpi_failure(acpi_evaluate_object(handle, b"_MAT\0".as_ptr() as _, core::ptr::null_mut(), &mut buffer)) &&
       buffer.length != 0 && !buffer.pointer.is_null() {
        let obj = buffer.pointer as *mut acpi_object;
        if (*obj).type_ == ACPI_TYPE_BUFFER && (*obj).buffer.length >= core::mem::size_of::<acpi_subtable_header>() {
            let header = (*obj).buffer.pointer as *mut acpi_subtable_header;
            if (*header).type_ == ACPI_MADT_TYPE_IO_APIC {
                get_ioapic_id(header, header as usize + (*obj).buffer.length as usize, gsi_base, phys_addr, &mut apic_id);
            }
        }
    }
    kfree(buffer.pointer);
    apic_id
}

pub unsafe fn acpi_get_ioapic_id(handle: acpi_handle, gsi_base: u32, phys_addr: *mut u64) -> i32 {
    let mut apic_id = parse_mat_ioapic_entry(handle, gsi_base, phys_addr);
    if apic_id == -1 { apic_id = parse_madt_ioapic_entry(gsi_base, phys_addr); }
    apic_id
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
