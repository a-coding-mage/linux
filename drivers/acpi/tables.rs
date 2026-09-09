// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  acpi_tables.c - ACPI Boot-Time Table Parsing
 *
 *  Copyright (C) 2001 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 */

// Linux headers and the local ACPI internals supply the declarations used here.
// Build-time configuration conditionals from the C source are retained below.

const ACPI_MAX_TABLES: usize = 128;

static mut MPS_INTI_FLAGS_POLARITY: [&'static str; 4] = ["dfl", "high", "res", "low"];
static mut MPS_INTI_FLAGS_TRIGGER: [&'static str; 4] = ["dfl", "edge", "res", "level"];
static mut initial_tables: [acpi_table_desc; ACPI_MAX_TABLES] = [unsafe { core::mem::zeroed() }; ACPI_MAX_TABLES];
static mut acpi_apic_instance: i32 = 0;
/* Disable table checksum verification for the early stage. */
static mut acpi_verify_table_checksum: bool = false;

pub unsafe fn acpi_table_print_madt_entry(header: *mut acpi_subtable_header) {
    if header.is_null() { return; }
    match (*header).type_ {
        ACPI_MADT_TYPE_LOCAL_APIC => {
            let p = header as *mut acpi_madt_local_apic;
            pr_debug!("LAPIC (acpi_id[0x%02x] lapic_id[0x%02x] %s)\n", (*p).processor_id, (*p).id, str_enabled_disabled((*p).lapic_flags & ACPI_MADT_ENABLED));
        }
        ACPI_MADT_TYPE_LOCAL_X2APIC => {
            let p = header as *mut acpi_madt_local_x2apic;
            pr_debug!("X2APIC (apic_id[0x%02x] uid[0x%02x] %s)\n", (*p).local_apic_id, (*p).uid, str_enabled_disabled((*p).lapic_flags & ACPI_MADT_ENABLED));
        }
        ACPI_MADT_TYPE_IO_APIC => {
            let p = header as *mut acpi_madt_io_apic;
            pr_debug!("IOAPIC (id[0x%02x] address[0x%08x] gsi_base[%d])\n", (*p).id, (*p).address, (*p).global_irq_base);
        }
        ACPI_MADT_TYPE_INTERRUPT_OVERRIDE => {
            let p = header as *mut acpi_madt_interrupt_override;
            pr_info!("INT_SRC_OVR (bus %d bus_irq %d global_irq %d %s %s)\n", (*p).bus, (*p).source_irq, (*p).global_irq, MPS_INTI_FLAGS_POLARITY[((*p).inti_flags & ACPI_MADT_POLARITY_MASK) as usize], MPS_INTI_FLAGS_TRIGGER[(((*p).inti_flags & ACPI_MADT_TRIGGER_MASK) >> 2) as usize]);
            if (*p).inti_flags & !(ACPI_MADT_POLARITY_MASK | ACPI_MADT_TRIGGER_MASK) != 0 { pr_info!("INT_SRC_OVR unexpected reserved flags: 0x%x\n", (*p).inti_flags & !(ACPI_MADT_POLARITY_MASK | ACPI_MADT_TRIGGER_MASK)); }
        }
        ACPI_MADT_TYPE_NMI_SOURCE => {
            let p = header as *mut acpi_madt_nmi_source;
            pr_info!("NMI_SRC (%s %s global_irq %d)\n", MPS_INTI_FLAGS_POLARITY[((*p).inti_flags & ACPI_MADT_POLARITY_MASK) as usize], MPS_INTI_FLAGS_TRIGGER[(((*p).inti_flags & ACPI_MADT_TRIGGER_MASK) >> 2) as usize], (*p).global_irq);
        }
        ACPI_MADT_TYPE_LOCAL_APIC_NMI => {
            let p = header as *mut acpi_madt_local_apic_nmi;
            pr_info!("LAPIC_NMI (acpi_id[0x%02x] %s %s lint[0x%x])\n", (*p).processor_id, MPS_INTI_FLAGS_POLARITY[((*p).inti_flags & ACPI_MADT_POLARITY_MASK) as usize], MPS_INTI_FLAGS_TRIGGER[(((*p).inti_flags & ACPI_MADT_TRIGGER_MASK) >> 2) as usize], (*p).lint);
        }
        ACPI_MADT_TYPE_LOCAL_X2APIC_NMI => {
            let p = header as *mut acpi_madt_local_x2apic_nmi;
            let polarity = (*p).inti_flags & ACPI_MADT_POLARITY_MASK;
            let trigger = ((*p).inti_flags & ACPI_MADT_TRIGGER_MASK) >> 2;
            pr_info!("X2APIC_NMI (uid[0x%02x] %s %s lint[0x%x])\n", (*p).uid, MPS_INTI_FLAGS_POLARITY[polarity as usize], MPS_INTI_FLAGS_TRIGGER[trigger as usize], (*p).lint);
        }
        ACPI_MADT_TYPE_LOCAL_APIC_OVERRIDE => { let p = header as *mut acpi_madt_local_apic_override; pr_info!("LAPIC_ADDR_OVR (address[0x%llx])\n", (*p).address); }
        ACPI_MADT_TYPE_IO_SAPIC => { let p = header as *mut acpi_madt_io_sapic; pr_debug!("IOSAPIC (id[0x%x] address[%p] gsi_base[%d])\n", (*p).id, (*p).address as usize as *const core::ffi::c_void, (*p).global_irq_base); }
        ACPI_MADT_TYPE_LOCAL_SAPIC => { let p = header as *mut acpi_madt_local_sapic; pr_debug!("LSAPIC (acpi_id[0x%02x] lsapic_id[0x%02x] lsapic_eid[0x%02x] %s)\n", (*p).processor_id, (*p).id, (*p).eid, str_enabled_disabled((*p).lapic_flags & ACPI_MADT_ENABLED)); }
        ACPI_MADT_TYPE_INTERRUPT_SOURCE => { let p = header as *mut acpi_madt_interrupt_source; pr_info!("PLAT_INT_SRC (%s %s type[0x%x] id[0x%04x] eid[0x%x] iosapic_vector[0x%x] global_irq[0x%x]\n", MPS_INTI_FLAGS_POLARITY[(((*p).inti_flags & ACPI_MADT_POLARITY_MASK)) as usize], MPS_INTI_FLAGS_TRIGGER[(((*p).inti_flags & ACPI_MADT_TRIGGER_MASK) >> 2) as usize], (*p).type_, (*p).id, (*p).eid, (*p).io_sapic_vector, (*p).global_irq); }
        ACPI_MADT_TYPE_GENERIC_INTERRUPT => { let p = header as *mut acpi_madt_generic_interrupt; pr_debug!("GICC (acpi_id[0x%04x] address[%llx] MPIDR[0x%llx] %s)\n", (*p).uid, (*p).base_address, (*p).arm_mpidr, str_enabled_disabled((*p).flags & ACPI_MADT_ENABLED)); }
        ACPI_MADT_TYPE_GENERIC_DISTRIBUTOR => { let p = header as *mut acpi_madt_generic_distributor; pr_debug!("GIC Distributor (gic_id[0x%04x] address[%llx] gsi_base[%d])\n", (*p).gic_id, (*p).base_address, (*p).global_irq_base); }
        ACPI_MADT_TYPE_MULTIPROC_WAKEUP => { let p = header as *mut acpi_madt_multiproc_wakeup; let reset_vector = if (*p).version >= ACPI_MADT_MP_WAKEUP_VERSION_V1 { (*p).reset_vector } else { 0 }; pr_debug!("MP Wakeup (version[%d], mailbox[%#llx], reset[%#llx])\n", (*p).version, (*p).mailbox_address, reset_vector); }
        ACPI_MADT_TYPE_CORE_PIC => { let p = header as *mut acpi_madt_core_pic; pr_debug!("CORE PIC (processor_id[0x%02x] core_id[0x%02x] %s)\n", (*p).processor_id, (*p).core_id, str_enabled_disabled((*p).flags & ACPI_MADT_ENABLED)); }
        ACPI_MADT_TYPE_RINTC => { let p = header as *mut acpi_madt_rintc; pr_debug!("RISC-V INTC (acpi_uid[0x%04x] hart_id[0x%llx] %s)\n", (*p).uid, (*p).hart_id, str_enabled_disabled((*p).flags & ACPI_MADT_ENABLED)); }
        _ => pr_warn!("Found unsupported MADT entry (type = 0x%x)\n", (*header).type_),
    }
}

pub unsafe fn acpi_table_parse_entries_array(id: *mut i8, table_size: usize, proc_: *mut acpi_subtable_proc, proc_num: i32, max_entries: u32) -> i32 {
    if acpi_disabled { return -ENODEV; }
    if id.is_null() || table_size == 0 { return -EINVAL; }
    let mut instance = 0u32;
    if !strncmp(id, ACPI_SIG_MADT, 4) { instance = acpi_apic_instance as u32; }
    let mut table_header: *mut acpi_table_header = core::ptr::null_mut();
    acpi_get_table(id, instance, &mut table_header);
    if table_header.is_null() { pr_debug!("%4.4s not present\n", id); return -ENODEV; }
    let count = acpi_parse_entries_array(id, table_size, table_header as *mut fw_table_header, 0, proc_, proc_num, max_entries);
    acpi_put_table(table_header);
    count
}

unsafe fn __acpi_table_parse_entries(id: *mut i8, table_size: usize, entry_id: i32, handler: acpi_tbl_entry_handler, handler_arg: acpi_tbl_entry_handler_arg, arg: *mut core::ffi::c_void, max_entries: u32) -> i32 {
    let mut proc_ = acpi_subtable_proc { id: entry_id, handler, handler_arg, arg };
    acpi_table_parse_entries_array(id, table_size, &mut proc_, 1, max_entries)
}

pub unsafe fn acpi_table_parse_cedt(id: acpi_cedt_type, handler_arg: acpi_tbl_entry_handler_arg, arg: *mut core::ffi::c_void) -> i32 { __acpi_table_parse_entries(ACPI_SIG_CEDT as *mut i8, core::mem::size_of::<acpi_table_cedt>(), id as i32, None, handler_arg, arg, 0) }
pub unsafe fn acpi_table_parse_entries(id: *mut i8, table_size: usize, entry_id: i32, handler: acpi_tbl_entry_handler, max_entries: u32) -> i32 { __acpi_table_parse_entries(id, table_size, entry_id, handler, None, core::ptr::null_mut(), max_entries) }
pub unsafe fn acpi_table_parse_madt(id: acpi_madt_type, handler: acpi_tbl_entry_handler, max_entries: u32) -> i32 { acpi_table_parse_entries(ACPI_SIG_MADT as *mut i8, core::mem::size_of::<acpi_table_madt>(), id as i32, handler, max_entries) }

pub unsafe fn acpi_table_parse(id: *mut i8, handler: acpi_tbl_table_handler) -> i32 {
    if acpi_disabled { return -ENODEV; }
    if id.is_null() || handler.is_none() { return -EINVAL; }
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    acpi_get_table(id, if strncmp(id, ACPI_SIG_MADT, 4) == 0 { acpi_apic_instance as u32 } else { 0 }, &mut table);
    if !table.is_null() { handler.unwrap()(table); acpi_put_table(table); 0 } else { -ENODEV }
}

unsafe fn check_multiple_madt() {
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    acpi_get_table(ACPI_SIG_MADT as *mut i8, 2, &mut table);
    if !table.is_null() { pr_warn!("BIOS bug: multiple APIC/MADT found, using %d\n", acpi_apic_instance); pr_warn!("If \"acpi_apic_instance=%d\" works better, notify linux-acpi@vger.kernel.org\n", if acpi_apic_instance != 0 { 0 } else { 2 }); acpi_put_table(table); } else { acpi_apic_instance = 0; }
}

unsafe fn acpi_table_taint(table: *mut acpi_table_header) { pr_warn!("Override [%4.4s-%8.8s], this is unsafe: tainting kernel\n", (*table).signature, (*table).oem_table_id); add_taint(TAINT_OVERRIDDEN_ACPI_TABLE, LOCKDEP_NOW_UNRELIABLE); }

// CONFIG_ACPI_TABLE_UPGRADE supplies the full initrd table relocation and scan implementation.
#[cfg(CONFIG_ACPI_TABLE_UPGRADE)]
static mut acpi_tables_addr: u64 = 0;
#[cfg(CONFIG_ACPI_TABLE_UPGRADE)]
static mut all_tables_size: i32 = 0;
#[cfg(CONFIG_ACPI_TABLE_UPGRADE)]
unsafe fn acpi_table_checksum(mut buffer: *mut u8, length: u32) -> u8 { let end = buffer.add(length as usize); let mut sum = 0u8; while buffer < end { sum = sum.wrapping_add(*buffer); buffer = buffer.add(1); } sum }
#[cfg(CONFIG_ACPI_TABLE_UPGRADE)]
unsafe fn acpi_table_initrd_override(existing: *mut acpi_table_header, address: *mut acpi_physical_address, length: *mut u32) -> acpi_status {
    *length = 0; *address = 0; if acpi_tables_addr == 0 { return AE_OK; }
    let mut off = 0i32; let mut index = 0i32;
    while off + core::mem::size_of::<acpi_table_header>() as i32 <= all_tables_size {
        let table = acpi_os_map_memory(acpi_tables_addr + off as u64, core::mem::size_of::<acpi_table_header>());
        let table_length = (*table).length;
        if off + table_length as i32 > all_tables_size { acpi_os_unmap_memory(table, core::mem::size_of::<acpi_table_header>()); WARN_ON!(true); return AE_OK; }
        if memcmp((*existing).signature.as_ptr(), (*table).signature.as_ptr(), 4) != 0 || memcmp((*table).oem_id.as_ptr(), (*existing).oem_id.as_ptr(), ACPI_OEM_ID_SIZE) != 0 || memcmp((*table).oem_table_id.as_ptr(), (*existing).oem_table_id.as_ptr(), ACPI_OEM_TABLE_ID_SIZE) != 0 { acpi_os_unmap_memory(table, core::mem::size_of::<acpi_table_header>()); off += table_length as i32; index += 1; continue; }
        if test_and_set_bit(index as usize, acpi_initrd_installed) || (*existing).oem_revision >= (*table).oem_revision { acpi_os_unmap_memory(table, core::mem::size_of::<acpi_table_header>()); off += table_length as i32; index += 1; continue; }
        *length = table_length; *address = acpi_tables_addr + off as u64; pr_info!("Table Upgrade: override [%4.4s-%6.6s-%8.8s]\n", (*table).signature, (*table).oem_id, (*table).oem_table_id); acpi_os_unmap_memory(table, core::mem::size_of::<acpi_table_header>()); break;
    }
    AE_OK
}
#[cfg(CONFIG_ACPI_TABLE_UPGRADE)]
unsafe fn acpi_table_initrd_scan() {
    if acpi_tables_addr == 0 { return; }
    let mut off = 0i32; let mut index = 0i32;
    while off + core::mem::size_of::<acpi_table_header>() as i32 <= all_tables_size {
        let table = acpi_os_map_memory(acpi_tables_addr + off as u64, core::mem::size_of::<acpi_table_header>()); let len = (*table).length;
        if off + len as i32 > all_tables_size { acpi_os_unmap_memory(table, core::mem::size_of::<acpi_table_header>()); WARN_ON!(true); return; }
        if ACPI_COMPARE_NAMESEG!((*table).signature, ACPI_SIG_RSDT) || ACPI_COMPARE_NAMESEG!((*table).signature, ACPI_SIG_XSDT) || test_and_set_bit(index as usize, acpi_initrd_installed) { acpi_os_unmap_memory(table, core::mem::size_of::<acpi_table_header>()); off += len as i32; index += 1; continue; }
        pr_info!("Table Upgrade: install [%4.4s-%6.6s-%8.8s]\n", (*table).signature, (*table).oem_id, (*table).oem_table_id); acpi_os_unmap_memory(table, core::mem::size_of::<acpi_table_header>()); acpi_install_physical_table(acpi_tables_addr + off as u64); off += len as i32; index += 1;
    }
}

// The complete C implementation also validates initrd CPIO entries, allocates reserved physical memory,
// and copies each table through MAP_CHUNK_SIZE early mappings before the scan above.
#[cfg(not(CONFIG_ACPI_TABLE_UPGRADE))]
unsafe fn acpi_table_initrd_override(_: *mut acpi_table_header, address: *mut acpi_physical_address, length: *mut u32) -> acpi_status { *length = 0; *address = 0; AE_OK }
#[cfg(not(CONFIG_ACPI_TABLE_UPGRADE))]
unsafe fn acpi_table_initrd_scan() {}

pub unsafe fn acpi_os_physical_table_override(existing_table: *mut acpi_table_header, address: *mut acpi_physical_address, table_length: *mut u32) -> acpi_status { acpi_table_initrd_override(existing_table, address, table_length) }

pub unsafe fn acpi_os_table_override(existing_table: *mut acpi_table_header, new_table: *mut *mut acpi_table_header) -> acpi_status {
    if existing_table.is_null() || new_table.is_null() { return AE_BAD_PARAMETER; }
    *new_table = core::ptr::null_mut();
    // CONFIG_ACPI_CUSTOM_DSDT optionally assigns the weak AmlCode/dsdt_aml_code symbols here.
    if cfg!(CONFIG_ACPI_CUSTOM_DSDT) && strncmp((*existing_table).signature.as_mut_ptr(), b"DSDT\0".as_ptr() as *const i8, 4) == 0 { *new_table = core::ptr::null_mut(); }
    if !(*new_table).is_null() { acpi_table_taint(existing_table); }
    AE_OK
}

pub unsafe fn acpi_locate_initial_tables() -> i32 {
    if acpi_verify_table_checksum { pr_info!("Early table checksum verification enabled\n"); acpi_gbl_enable_table_validation = TRUE; } else { pr_info!("Early table checksum verification disabled\n"); acpi_gbl_enable_table_validation = FALSE; }
    let status = acpi_initialize_tables(initial_tables.as_mut_ptr(), ACPI_MAX_TABLES as u32, 0);
    if ACPI_FAILURE(status) { let msg = acpi_format_exception(status); pr_warn!("Failed to initialize tables, status=0x%x (%s)", status, msg); return -EINVAL; }
    0
}

pub unsafe fn acpi_reserve_initial_tables() {
    for i in 0..ACPI_MAX_TABLES { let table_desc = &initial_tables[i]; let start = table_desc.address; let size = table_desc.length; if start == 0 || size == 0 { break; } pr_info!("Reserving %4s table memory at [mem 0x%llx-0x%llx]\n", table_desc.signature.ascii, start, start + size - 1); memblock_reserve(start, size); }
}
pub unsafe fn acpi_table_init_complete() { acpi_table_initrd_scan(); check_multiple_madt(); }
pub unsafe fn acpi_table_init() -> i32 { let ret = acpi_locate_initial_tables(); if ret != 0 { return ret; } acpi_table_init_complete(); 0 }

unsafe fn acpi_parse_apic_instance(str_: *mut i8) -> i32 { if str_.is_null() { return -EINVAL; } if kstrtoint(str_, 0, &mut acpi_apic_instance) != 0 { return -EINVAL; } pr_notice!("Shall use APIC/MADT table %d\n", acpi_apic_instance); 0 }
unsafe fn acpi_force_table_verification_setup(_: *mut i8) -> i32 { acpi_verify_table_checksum = true; 0 }
unsafe fn acpi_force_32bit_fadt_addr(_: *mut i8) -> i32 { pr_info!("Forcing 32 Bit FADT addresses\n"); acpi_gbl_use32_bit_fadt_addresses = TRUE; 0 }

// early_param("acpi_apic_instance", acpi_parse_apic_instance);
// early_param("acpi_force_table_verification", acpi_force_table_verification_setup);
// early_param("acpi_force_32bit_fadt_addr", acpi_force_32bit_fadt_addr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
