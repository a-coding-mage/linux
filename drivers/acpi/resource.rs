// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/acpi/resource.c - ACPI device resources interpretation.
 *
 * Copyright (C) 2012, Intel Corp.
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// Linux headers and configuration symbols are supplied by the surrounding
// translation unit.  The declarations below intentionally retain those APIs.

#[cfg(CONFIG_X86)]
#[inline]
fn valid_IRQ(i: u32) -> bool { i != 0 && i != 2 }

#[cfg(CONFIG_X86)]
#[inline]
unsafe fn acpi_iospace_resource_valid(res: *mut resource) -> bool { (*res).end < 0x10003 }

#[cfg(not(CONFIG_X86))]
#[inline]
fn valid_IRQ(_i: u32) -> bool { true }

#[cfg(not(CONFIG_X86))]
#[inline]
unsafe fn acpi_iospace_resource_valid(_res: *mut resource) -> bool { true }

#[cfg(CONFIG_ACPI_GENERIC_GSI)]
#[inline]
unsafe fn is_gsi(ext_irq: *mut acpi_resource_extended_irq) -> bool {
    (*ext_irq).resource_source.string_length == 0 && (*ext_irq).producer_consumer == ACPI_CONSUMER
}
#[cfg(not(CONFIG_ACPI_GENERIC_GSI))]
#[inline]
unsafe fn is_gsi(_ext_irq: *mut acpi_resource_extended_irq) -> bool { true }

unsafe fn acpi_dev_resource_len_valid(start: u64, end: u64, len: u64, io: bool) -> bool {
    let reslen = end.wrapping_sub(start).wrapping_add(1);
    if len != 0 && reslen != 0 && start <= end { return true; }
    pr_debug!("ACPI: invalid or unassigned resource {} [{:016x} - {:016x}] length [{:016x}]\n", if io { "io" } else { "mem" }, start, end, len);
    false
}

unsafe fn acpi_dev_memresource_flags(res: *mut resource, len: u64, write_protect: u8) {
    (*res).flags = IORESOURCE_MEM;
    if !acpi_dev_resource_len_valid((*res).start, (*res).end, len, false) { (*res).flags |= IORESOURCE_DISABLED | IORESOURCE_UNSET; }
    if write_protect == ACPI_READ_WRITE_MEMORY { (*res).flags |= IORESOURCE_MEM_WRITEABLE; }
}

unsafe fn acpi_dev_get_memresource(res: *mut resource, start: u64, len: u64, write_protect: u8) {
    (*res).start = start; (*res).end = start.wrapping_add(len).wrapping_sub(1);
    acpi_dev_memresource_flags(res, len, write_protect);
}

pub unsafe fn acpi_dev_resource_memory(ares: *mut acpi_resource, res: *mut resource) -> bool {
    match (*ares).type_ {
        ACPI_RESOURCE_TYPE_MEMORY24 => { let x = &mut (*ares).data.memory24; acpi_dev_get_memresource(res, x.minimum << 8, x.address_length << 8, x.write_protect); }
        ACPI_RESOURCE_TYPE_MEMORY32 => { let x = &mut (*ares).data.memory32; acpi_dev_get_memresource(res, x.minimum, x.address_length, x.write_protect); }
        ACPI_RESOURCE_TYPE_FIXED_MEMORY32 => { let x = &mut (*ares).data.fixed_memory32; acpi_dev_get_memresource(res, x.address, x.address_length, x.write_protect); }
        _ => { (*res).flags = 0; return false; }
    }
    (*res).flags & IORESOURCE_DISABLED == 0
}

unsafe fn acpi_dev_ioresource_flags(res: *mut resource, len: u64, io_decode: u8, translation_type: u8) {
    (*res).flags = IORESOURCE_IO;
    if !acpi_dev_resource_len_valid((*res).start, (*res).end, len, true) || !acpi_iospace_resource_valid(res) { (*res).flags |= IORESOURCE_DISABLED | IORESOURCE_UNSET; }
    if io_decode == ACPI_DECODE_16 { (*res).flags |= IORESOURCE_IO_16BIT_ADDR; }
    if translation_type == ACPI_SPARSE_TRANSLATION { (*res).flags |= IORESOURCE_IO_SPARSE; }
}
unsafe fn acpi_dev_get_ioresource(res: *mut resource, start: u64, len: u64, io_decode: u8) {
    (*res).start = start; (*res).end = start.wrapping_add(len).wrapping_sub(1); acpi_dev_ioresource_flags(res, len, io_decode, 0);
}

pub unsafe fn acpi_dev_resource_io(ares: *mut acpi_resource, res: *mut resource) -> bool {
    match (*ares).type_ {
        ACPI_RESOURCE_TYPE_IO => { let x = &mut (*ares).data.io; acpi_dev_get_ioresource(res, x.minimum, x.address_length, x.io_decode); }
        ACPI_RESOURCE_TYPE_FIXED_IO => { let x = &mut (*ares).data.fixed_io; acpi_dev_get_ioresource(res, x.address, x.address_length, ACPI_DECODE_10); }
        _ => { (*res).flags = 0; return false; }
    } (*res).flags & IORESOURCE_DISABLED == 0
}

unsafe fn acpi_decode_space(win: *mut resource_win, addr: *mut acpi_resource_address, attr: *mut acpi_address64_attribute) -> bool {
    let iodec = if (*attr).granularity == 0xfff { ACPI_DECODE_10 } else { ACPI_DECODE_16 };
    let wp = (*addr).info.mem.write_protect; let len = (*attr).address_length;
    let offset = if (*addr).producer_consumer == ACPI_PRODUCER { (*attr).translation_offset } else { 0 };
    let res = &mut (*win).res; (*win).offset = offset; res.start = (*attr).minimum + offset; res.end = (*attr).maximum + offset;
    match (*addr).resource_type {
        ACPI_MEMORY_RANGE => { acpi_dev_memresource_flags(res, len, wp); if (*addr).info.mem.caching == ACPI_PREFETCHABLE_MEMORY { res.flags |= IORESOURCE_PREFETCH; } }
        ACPI_IO_RANGE => acpi_dev_ioresource_flags(res, len, iodec, (*addr).info.io.translation_type),
        ACPI_BUS_NUMBER_RANGE => res.flags = IORESOURCE_BUS,
        _ => return false,
    }
    if (*addr).producer_consumer == ACPI_PRODUCER { res.flags |= IORESOURCE_WINDOW; }
    res.flags & IORESOURCE_DISABLED == 0
}

pub unsafe fn acpi_dev_resource_address_space(ares: *mut acpi_resource, win: *mut resource_win) -> bool {
    (*win).res.flags = 0; let mut addr: acpi_resource_address64 = core::mem::zeroed();
    if ACPI_FAILURE(acpi_resource_to_address64(ares, &mut addr)) { return false; }
    acpi_decode_space(win, &mut addr.address as *mut _, &mut addr.address as *mut _)
}
pub unsafe fn acpi_dev_resource_ext_address_space(ares: *mut acpi_resource, win: *mut resource_win) -> bool {
    (*win).res.flags = 0; if (*ares).type_ != ACPI_RESOURCE_TYPE_EXTENDED_ADDRESS64 { return false; }
    let x = &mut (*ares).data.ext_address64; acpi_decode_space(win, x as *mut _ as *mut _, &mut x.address)
}

pub fn acpi_dev_irq_flags(triggering: u8, polarity: u8, shareable: u8, wake_capable: u8) -> c_ulong {
    let mut flags = if triggering == ACPI_LEVEL_SENSITIVE { if polarity == ACPI_ACTIVE_LOW { IORESOURCE_IRQ_LOWLEVEL } else { IORESOURCE_IRQ_HIGHLEVEL } } else if polarity == ACPI_ACTIVE_LOW { IORESOURCE_IRQ_LOWEDGE } else { IORESOURCE_IRQ_HIGHEDGE };
    if shareable == ACPI_SHARED { flags |= IORESOURCE_IRQ_SHAREABLE; } if wake_capable == ACPI_WAKE_CAPABLE { flags |= IORESOURCE_IRQ_WAKECAPABLE; } flags | IORESOURCE_IRQ
}
pub fn acpi_dev_get_irq_type(triggering: i32, polarity: i32) -> c_uint {
    match polarity { ACPI_ACTIVE_LOW => if triggering == ACPI_EDGE_SENSITIVE { IRQ_TYPE_EDGE_FALLING } else { IRQ_TYPE_LEVEL_LOW }, ACPI_ACTIVE_HIGH => if triggering == ACPI_EDGE_SENSITIVE { IRQ_TYPE_EDGE_RISING } else { IRQ_TYPE_LEVEL_HIGH }, ACPI_ACTIVE_BOTH if triggering == ACPI_EDGE_SENSITIVE => IRQ_TYPE_EDGE_BOTH, _ => IRQ_TYPE_NONE }
}

#[repr(C)] struct irq_override_cmp { system: *const dmi_system_id, irq: u8, triggering: u8, polarity: u8, shareable: u8, override_: bool }
static override_table: [irq_override_cmp; 4] = [
    irq_override_cmp { system: core::ptr::null(), irq: 1, triggering: ACPI_LEVEL_SENSITIVE, polarity: ACPI_ACTIVE_LOW, shareable: 0, override_: false },
    irq_override_cmp { system: core::ptr::null(), irq: 10, triggering: ACPI_LEVEL_SENSITIVE, polarity: ACPI_ACTIVE_LOW, shareable: 1, override_: false },
    irq_override_cmp { system: core::ptr::null(), irq: 11, triggering: ACPI_LEVEL_SENSITIVE, polarity: ACPI_ACTIVE_LOW, shareable: 1, override_: false },
    irq_override_cmp { system: core::ptr::null(), irq: 1, triggering: ACPI_EDGE_SENSITIVE, polarity: ACPI_ACTIVE_LOW, shareable: 1, override_: true },
];

// The following resource-processing routines retain the C ABI and call the
// externally supplied kernel helpers; their declarations are intentionally
// left unresolved here, as in the source file's header dependencies.
extern "C" {
    pub fn acpi_dev_resource_interrupt(ares: *mut acpi_resource, index: i32, res: *mut resource) -> bool;
    pub fn acpi_dev_free_resource_list(list: *mut list_head);
    pub fn acpi_dev_get_resources(adev: *mut acpi_device, list: *mut list_head, preproc: Option<unsafe extern "C" fn(*mut acpi_resource, *mut c_void) -> i32>, data: *mut c_void) -> i32;
    pub fn acpi_dev_get_dma_resources(adev: *mut acpi_device, list: *mut list_head) -> i32;
    pub fn acpi_dev_get_memory_resources(adev: *mut acpi_device, list: *mut list_head) -> i32;
    pub fn acpi_dev_filter_resource_type(ares: *mut acpi_resource, types: c_ulong) -> i32;
    pub fn acpi_resource_consumer(res: *mut resource) -> *mut acpi_device;
}

unsafe fn acpi_dev_get_irqresource(res: *mut resource, gsi: u32, triggering: u8, polarity: u8, shareable: u8, wake_capable: u8, _check_override: bool) {
    if !valid_IRQ(gsi) { irqresource_disabled(res, gsi); return; }
    (*res).flags = acpi_dev_irq_flags(triggering, polarity, shareable, wake_capable);
    let irq = acpi_register_gsi(core::ptr::null_mut(), gsi, triggering, polarity);
    if irq >= 0 { (*res).start = irq as u64; (*res).end = irq as u64; } else { irqresource_disabled(res, gsi); }
}

unsafe fn acpi_dev_process_resource(ares: *mut acpi_resource, context: *mut c_void) -> acpi_status {
    let c = &mut *(context as *mut res_proc_context); let mut win: resource_win = core::mem::zeroed();
    if let Some(preproc) = c.preproc { let ret = preproc(ares, c.preproc_data); if ret < 0 { c.error = ret; return AE_ABORT_METHOD; } if ret > 0 { return AE_OK; } }
    let res = &mut win.res;
    if acpi_dev_resource_memory(ares, res) || acpi_dev_resource_io(ares, res) || acpi_dev_resource_address_space(ares, &mut win) || acpi_dev_resource_ext_address_space(ares, &mut win) { return acpi_dev_new_resource_entry(&mut win, c); }
    let mut i = 0; while acpi_dev_resource_interrupt(ares, i, res) { let status = acpi_dev_new_resource_entry(&mut win, c); if ACPI_FAILURE(status) { return status; } i += 1; }
    AE_OK
}

#[repr(C)] struct res_proc_context { list: *mut list_head, preproc: Option<unsafe extern "C" fn(*mut acpi_resource, *mut c_void) -> i32>, preproc_data: *mut c_void, count: i32, error: i32 }
unsafe fn acpi_dev_new_resource_entry(win: *mut resource_win, c: *mut res_proc_context) -> acpi_status {
    let rentry = resource_list_create_entry(core::ptr::null_mut(), 0); if rentry.is_null() { (*c).error = -ENOMEM; return AE_NO_MEMORY; }
    (*(*rentry).res) = (*win).res; (*rentry).offset = (*win).offset; resource_list_add_tail(rentry, (*c).list); (*c).count += 1; AE_OK
}

unsafe fn acpi_dev_consumes_res(adev: *mut acpi_device, res: *mut resource) -> i32 {
    let mut list: list_head = core::mem::zeroed(); INIT_LIST_HEAD(&mut list); let ret = acpi_dev_get_resources(adev, &mut list, None, core::ptr::null_mut()); if ret < 0 { return 0; }
    acpi_dev_free_resource_list(&mut list); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
