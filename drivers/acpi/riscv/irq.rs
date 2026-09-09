// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023-2024, Ventana Micro Systems Inc
 *	Author: Sunil V L <sunilvl@ventanamicro.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const RISCV_ACPI_INTC_FLAG_PENDING: u32 = 1 << 0;

#[repr(C)]
pub struct riscv_ext_intc_list {
    pub handle: acpi_handle,
    pub gsi_base: u32,
    pub nr_irqs: u32,
    pub nr_idcs: u32,
    pub id: u32,
    pub type_: u32,
    pub flag: u32,
    pub list: list_head,
}

pub static mut ext_intc_list: list_head = LIST_HEAD_INIT;

unsafe fn irqchip_cmp_func(in0: *const core::ffi::c_void, in1: *const core::ffi::c_void) -> i32 {
    let elem0 = in0 as *const acpi_probe_entry;
    let elem1 = in1 as *const acpi_probe_entry;

    ((*elem0).type_ > (*elem1).type_) as i32 - ((*elem0).type_ < (*elem1).type_) as i32
}

/*
 * On RISC-V, RINTC structures in MADT should be probed before any other
 * interrupt controller structures and IMSIC before APLIC. The interrupt
 * controller subtypes in MADT of ACPI spec for RISC-V are defined in the
 * incremental order like RINTC(24)->IMSIC(25)->APLIC(26)->PLIC(27).
 * Hence, simply sorting the subtypes in incremental order will establish the
 * required order.
 */
pub unsafe fn arch_sort_irqchip_probe(ap_head: *mut acpi_probe_entry, nr: i32) {
    let ape = ap_head;

    if nr == 1 || !ACPI_COMPARE_NAMESEG(ACPI_SIG_MADT, (*ape).id) {
        return;
    }
    sort(ape as *mut core::ffi::c_void, nr as usize, core::mem::size_of::<acpi_probe_entry>(), irqchip_cmp_func, core::ptr::null_mut());
}

unsafe fn riscv_acpi_update_gsi_handle(gsi_base: u32, handle: acpi_handle) -> acpi_status {
    let mut i: *mut list_head;
    let mut tmp: *mut list_head;
    list_for_each_safe!(i, tmp, &raw mut ext_intc_list);
    while !i.is_null() {
        let ext_intc_element = list_entry!(i, riscv_ext_intc_list, list);
        if gsi_base == (*ext_intc_element).gsi_base {
            (*ext_intc_element).handle = handle;
            return AE_OK;
        }
        i = (*i).next;
    }
    AE_NOT_FOUND
}

pub unsafe fn riscv_acpi_update_gsi_range(gsi_base: u32, nr_irqs: u32) -> i32 {
    let mut ext_intc_element: *mut riscv_ext_intc_list;
    list_for_each_entry!(ext_intc_element, &raw mut ext_intc_list, list);
    while !ext_intc_element.is_null() {
        if gsi_base == (*ext_intc_element).gsi_base
            && ((*ext_intc_element).flag & RISCV_ACPI_INTC_FLAG_PENDING) != 0
        {
            (*ext_intc_element).nr_irqs = nr_irqs;
            (*ext_intc_element).flag &= !RISCV_ACPI_INTC_FLAG_PENDING;
            return 0;
        }
        ext_intc_element = list_next_entry!(ext_intc_element, list);
    }
    -ENODEV
}

pub unsafe fn riscv_acpi_get_gsi_info(
    fwnode: *mut fwnode_handle,
    gsi_base: *mut u32,
    id: *mut u32,
    nr_irqs: *mut u32,
    nr_idcs: *mut u32,
) -> i32 {
    let mut i: *mut list_head;
    list_for_each!(i, &raw mut ext_intc_list);
    while !i.is_null() {
        let ext_intc_element = list_entry!(i, riscv_ext_intc_list, list);
        if (*ext_intc_element).handle == ACPI_HANDLE_FWNODE(fwnode) {
            *gsi_base = (*ext_intc_element).gsi_base;
            *id = (*ext_intc_element).id;
            *nr_irqs = (*ext_intc_element).nr_irqs;
            if !nr_idcs.is_null() {
                *nr_idcs = (*ext_intc_element).nr_idcs;
            }
            return 0;
        }
        i = (*i).next;
    }
    -ENODEV
}

pub unsafe fn riscv_acpi_get_gsi_domain_id(gsi: u32) -> *mut fwnode_handle {
    let mut i: *mut list_head;
    list_for_each!(i, &raw mut ext_intc_list);
    while !i.is_null() {
        let ext_intc_element = list_entry!(i, riscv_ext_intc_list, list);
        if gsi >= (*ext_intc_element).gsi_base
            && gsi < (*ext_intc_element).gsi_base.wrapping_add((*ext_intc_element).nr_irqs)
        {
            let adev = acpi_fetch_acpi_dev((*ext_intc_element).handle);
            if adev.is_null() { return core::ptr::null_mut(); }
            return acpi_fwnode_handle(adev);
        }
        i = (*i).next;
    }
    core::ptr::null_mut()
}

unsafe fn riscv_acpi_register_ext_intc(gsi_base: u32, nr_irqs: u32, nr_idcs: u32, id: u32, type_: u32) -> i32 {
    let ext_intc_element = kzalloc_obj!(riscv_ext_intc_list);
    if ext_intc_element.is_null() { return -ENOMEM; }
    (*ext_intc_element).gsi_base = gsi_base;
    if nr_irqs != 0 {
        (*ext_intc_element).nr_irqs = nr_irqs;
    } else {
        (*ext_intc_element).flag |= RISCV_ACPI_INTC_FLAG_PENDING;
        (*ext_intc_element).nr_irqs = u32::MAX.wrapping_sub(gsi_base);
    }
    (*ext_intc_element).nr_idcs = nr_idcs;
    (*ext_intc_element).id = id;
    let mut node: *mut riscv_ext_intc_list;
    list_for_each_entry!(node, &raw mut ext_intc_list, list);
    while !node.is_null() {
        if (*node).gsi_base < (*ext_intc_element).gsi_base { break; }
        node = list_next_entry!(node, list);
    }
    let prev = list_prev_entry!(node, list);
    if !list_entry_is_head!(prev, &raw mut ext_intc_list, list)
        && ((*prev).flag & RISCV_ACPI_INTC_FLAG_PENDING) != 0
    { (*prev).nr_irqs = (*ext_intc_element).gsi_base - (*prev).gsi_base; }
    list_add_tail!(&raw mut (*ext_intc_element).list, &raw mut (*node).list);
    0
}

// The remaining ACPI callbacks retain the C callback ABI and external ACPI operations.
unsafe fn riscv_acpi_create_gsi_map_smsi(handle: acpi_handle, _level: u32, _context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    if !acpi_has_method(handle, "_GSB") { acpi_handle_err(handle, "_GSB method not found\n"); return AE_ERROR; }
    let mut gbase = 0u64;
    let status = acpi_evaluate_integer(handle, "_GSB", core::ptr::null_mut(), &mut gbase);
    if ACPI_FAILURE(status) { acpi_handle_err(handle, "failed to evaluate _GSB method\n"); return status; }
    riscv_acpi_register_ext_intc(gbase as u32, 0, 0, 0, ACPI_RISCV_IRQCHIP_SMSI);
    let status = riscv_acpi_update_gsi_handle(gbase as u32, handle);
    if ACPI_FAILURE(status) { acpi_handle_err(handle, "failed to find the GSI mapping entry\n"); return status; }
    AE_OK
}

unsafe fn riscv_acpi_create_gsi_map(handle: acpi_handle, _level: u32, _context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    if !acpi_has_method(handle, "_GSB") { acpi_handle_err(handle, "_GSB method not found\n"); return AE_ERROR; }
    let mut gbase = 0u64;
    let status = acpi_evaluate_integer(handle, "_GSB", core::ptr::null_mut(), &mut gbase);
    if ACPI_FAILURE(status) { acpi_handle_err(handle, "failed to evaluate _GSB method\n"); return status; }
    let status = riscv_acpi_update_gsi_handle(gbase as u32, handle);
    if ACPI_FAILURE(status) { acpi_handle_err(handle, "failed to find the GSI mapping entry\n"); return status; }
    AE_OK
}

unsafe fn riscv_acpi_aplic_parse_madt(header: *mut acpi_subtable_headers, _end: usize) -> i32 { let aplic = header as *mut acpi_madt_aplic; riscv_acpi_register_ext_intc((*aplic).gsi_base, (*aplic).num_sources, (*aplic).num_idcs, (*aplic).id, ACPI_RISCV_IRQCHIP_APLIC) }
unsafe fn riscv_acpi_plic_parse_madt(header: *mut acpi_subtable_headers, _end: usize) -> i32 { let plic = header as *mut acpi_madt_plic; riscv_acpi_register_ext_intc((*plic).gsi_base, (*plic).num_irqs, 0, (*plic).id, ACPI_RISCV_IRQCHIP_PLIC) }

pub unsafe fn riscv_acpi_init_gsi_mapping() {
    if acpi_table_parse_madt(ACPI_MADT_TYPE_PLIC, riscv_acpi_plic_parse_madt, 0) > 0 { acpi_get_devices("RSCV0001", riscv_acpi_create_gsi_map, core::ptr::null_mut(), core::ptr::null_mut()); return; }
    if acpi_table_parse_madt(ACPI_MADT_TYPE_APLIC, riscv_acpi_aplic_parse_madt, 0) > 0 { acpi_get_devices("RSCV0002", riscv_acpi_create_gsi_map, core::ptr::null_mut(), core::ptr::null_mut()); }
    acpi_get_devices("RSCV0006", riscv_acpi_create_gsi_map_smsi, core::ptr::null_mut(), core::ptr::null_mut());
}

pub unsafe fn acpi_get_riscv_gsi_handle(gsi: u32) -> acpi_handle {
    let mut i: *mut list_head;
    list_for_each!(i, &raw mut ext_intc_list);
    while !i.is_null() { let e = list_entry!(i, riscv_ext_intc_list, list); if gsi >= (*e).gsi_base && gsi < (*e).gsi_base.wrapping_add((*e).nr_irqs) { return (*e).handle; } i = (*i).next; }
    core::ptr::null_mut()
}

pub unsafe fn arch_acpi_add_auto_dep(handle: acpi_handle) -> u32 { acpi_irq_add_auto_dep(handle) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
