// SPDX-License-Identifier: GPL-2.0

/*
 * Hyper-V nested virtualization code.
 *
 * Copyright (C) 2018, Microsoft, Inc.
 *
 * Author : Lan Tianyu <Tianyu.Lan@microsoft.com>
 */

use core::ffi::c_void;

// Definitions supplied by the Linux Hyper-V and x86 dependencies.
// The build configuration supplies the corresponding types, constants, and helpers.

extern "C" {
    static mut hv_hypercall_pg: *mut c_void;
    static mut hyperv_pcpu_input_arg: *mut *mut hv_guest_mapping_flush;

    fn hv_do_hypercall(code: u64, input: *mut c_void, output: *mut c_void) -> u64;
    fn hv_do_rep_hypercall(
        code: u64,
        rep_count: i32,
        varhead_size: u16,
        input: *mut c_void,
        output: *mut c_void,
    ) -> u64;
    fn hv_result_success(status: u64) -> bool;
    fn hv_result(status: u64) -> i32;
    fn trace_hyperv_nested_flush_guest_mapping(as_: u64, ret: i32);
    fn trace_hyperv_nested_flush_guest_mapping_range(as_: u64, ret: i32);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

#[repr(C)]
pub struct hv_guest_mapping_flush {
    pub address_space: u64,
    pub flags: u64,
    pub gpa_list: [hv_guest_mapping_flush_entry; HV_MAX_FLUSH_REP_COUNT as usize],
}

#[repr(C)]
pub struct hv_guest_mapping_flush_list {
    pub address_space: u64,
    pub flags: u64,
    pub gpa_list: [hv_guest_mapping_flush_entry; HV_MAX_FLUSH_REP_COUNT as usize],
}

#[repr(C)]
pub union hv_guest_mapping_flush_entry {
    pub page: hv_guest_mapping_flush_page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_guest_mapping_flush_page {
    pub additional_pages: u64,
    pub largepage: bool,
    pub basepfn: u64,
}

pub type hyperv_fill_flush_list_func = unsafe extern "C" fn(
    flush: *mut hv_guest_mapping_flush_list,
    data: *mut c_void,
) -> i32;

extern "C" {
    static HV_MAX_FLUSH_REP_COUNT: u32;
    static HV_MAX_FLUSH_PAGES: u64;
    static HVCALL_FLUSH_GUEST_PHYSICAL_ADDRESS_SPACE: u64;
    static HVCALL_FLUSH_GUEST_PHYSICAL_ADDRESS_LIST: u64;
}

pub unsafe extern "C" fn hyperv_flush_guest_mapping(as_: u64) -> i32 {
    let mut flush: *mut hv_guest_mapping_flush;
    let mut status: u64;
    let mut flags: usize = 0;
    let mut ret: i32 = -95; // -ENOTSUPP

    if hv_hypercall_pg.is_null() {
        trace_hyperv_nested_flush_guest_mapping(as_, ret);
        return ret;
    }

    local_irq_save(&mut flags);

    flush = hyperv_pcpu_input_arg;

    if flush.is_null() {
        local_irq_restore(flags);
        goto_fault;
        trace_hyperv_nested_flush_guest_mapping(as_, ret);
        return ret;
    }

    (*flush).address_space = as_;
    (*flush).flags = 0;

    status = hv_do_hypercall(
        HVCALL_FLUSH_GUEST_PHYSICAL_ADDRESS_SPACE,
        flush.cast(),
        core::ptr::null_mut(),
    );
    local_irq_restore(flags);

    if hv_result_success(status) {
        ret = 0;
    }

    trace_hyperv_nested_flush_guest_mapping(as_, ret);
    ret
}

pub unsafe extern "C" fn hyperv_fill_flush_guest_mapping_list(
    flush: *mut hv_guest_mapping_flush_list,
    start_gfn: u64,
    mut pages: u64,
) -> i32 {
    let mut cur = start_gfn;
    let mut additional_pages: u64;
    let mut gpa_n: i32 = 0;

    loop {
        if gpa_n as u32 >= HV_MAX_FLUSH_REP_COUNT {
            return -28; // -ENOSPC
        }

        additional_pages = core::cmp::min(pages, HV_MAX_FLUSH_PAGES) - 1;

        (*flush).gpa_list[gpa_n as usize].page.additional_pages = additional_pages;
        (*flush).gpa_list[gpa_n as usize].page.largepage = false;
        (*flush).gpa_list[gpa_n as usize].page.basepfn = cur;

        pages -= additional_pages + 1;
        cur += additional_pages + 1;
        gpa_n += 1;

        if pages <= 0 {
            break;
        }
    }

    gpa_n
}

pub unsafe extern "C" fn hyperv_flush_guest_mapping_range(
    as_: u64,
    fill_flush_list_func: Option<hyperv_fill_flush_list_func>,
    data: *mut c_void,
) -> i32 {
    let mut flush: *mut hv_guest_mapping_flush_list;
    let mut status: u64;
    let mut flags: usize = 0;
    let mut ret: i32 = -95; // -ENOTSUPP
    let mut gpa_n: i32 = 0;

    if hv_hypercall_pg.is_null() || fill_flush_list_func.is_none() {
        trace_hyperv_nested_flush_guest_mapping_range(as_, ret);
        return ret;
    }

    local_irq_save(&mut flags);
    flush = hyperv_pcpu_input_arg.cast();

    if flush.is_null() {
        local_irq_restore(flags);
        trace_hyperv_nested_flush_guest_mapping_range(as_, ret);
        return ret;
    }

    (*flush).address_space = as_;
    (*flush).flags = 0;

    gpa_n = fill_flush_list_func.unwrap()(flush, data);
    if gpa_n < 0 {
        local_irq_restore(flags);
        trace_hyperv_nested_flush_guest_mapping_range(as_, ret);
        return ret;
    }

    status = hv_do_rep_hypercall(
        HVCALL_FLUSH_GUEST_PHYSICAL_ADDRESS_LIST,
        gpa_n,
        0,
        flush.cast(),
        core::ptr::null_mut(),
    );

    local_irq_restore(flags);

    if hv_result_success(status) {
        ret = 0;
    } else {
        ret = hv_result(status);
    }

    trace_hyperv_nested_flush_guest_mapping_range(as_, ret);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
