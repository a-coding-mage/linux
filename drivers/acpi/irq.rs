// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI GSI IRQ layer
 *
 * Copyright (C) 2015 ARM Ltd.
 * Author: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut ACPI_IRQ_MODEL: acpi_irq_model_id = 0 as acpi_irq_model_id;

static mut ACPI_GET_GSI_DOMAIN_ID: Option<acpi_gsi_domain_disp_fn> = None;
static mut ACPI_GET_GSI_HANDLE: Option<acpi_gsi_handle_disp_fn> = None;
static mut ACPI_GSI_TO_IRQ_FALLBACK: Option<unsafe extern "C" fn(u32) -> u32> = None;

pub unsafe extern "C" fn acpi_gsi_to_irq(gsi: u32, irq: *mut c_uint) -> c_int {
    let d = irq_find_matching_fwnode(
        (ACPI_GET_GSI_DOMAIN_ID.unwrap())(gsi),
        DOMAIN_BUS_ANY,
    );
    *irq = irq_find_mapping(d, gsi);
    /*
     * *irq == 0 means no mapping, that should be reported as a
     * failure, unless there is an arch-specific fallback handler.
     */
    if *irq == 0 {
        if let Some(fallback) = ACPI_GSI_TO_IRQ_FALLBACK {
            *irq = fallback(gsi);
        }
    }

    if *irq > 0 { 0 } else { -EINVAL }
}

pub unsafe extern "C" fn acpi_register_gsi(
    dev: *mut device,
    gsi: u32,
    trigger: c_int,
    polarity: c_int,
) -> c_int {
    let mut fwspec: irq_fwspec = core::mem::zeroed();
    let fwnode = (ACPI_GET_GSI_DOMAIN_ID.unwrap())(gsi);
    if WARN_ON(!fwnode.is_null()) {
        pr_warn!("GSI: No registered irqchip, giving up\n");
        return -EINVAL;
    }

    fwspec.fwnode = fwnode;
    fwspec.param[0] = gsi;
    fwspec.param[1] = acpi_dev_get_irq_type(trigger, polarity);
    fwspec.param_count = 2;

    let irq = irq_create_fwspec_mapping(&mut fwspec);
    if irq == 0 { return -EINVAL; }
    irq as c_int
}

pub unsafe extern "C" fn acpi_unregister_gsi(gsi: u32) {
    if WARN_ON(ACPI_IRQ_MODEL == ACPI_IRQ_MODEL_GIC && gsi < 16) { return; }
    let d = irq_find_matching_fwnode((ACPI_GET_GSI_DOMAIN_ID.unwrap())(gsi), DOMAIN_BUS_ANY);
    let irq = irq_find_mapping(d, gsi);
    irq_dispose_mapping(irq);
}

unsafe fn acpi_get_irq_source_fwhandle(
    source: *const acpi_resource_source,
    gsi: u32,
) -> *mut fwnode_handle {
    if (*source).string_length == 0 { return (ACPI_GET_GSI_DOMAIN_ID.unwrap())(gsi); }
    let mut handle: acpi_handle = core::ptr::null_mut();
    let status = acpi_get_handle(core::ptr::null_mut(), (*source).string_ptr, &mut handle);
    if WARN_ON(ACPI_FAILURE(status)) { return core::ptr::null_mut(); }
    let device = acpi_get_acpi_dev(handle);
    if WARN_ON(device.is_null()) { return core::ptr::null_mut(); }
    let result = &mut (*device).fwnode as *mut fwnode_handle;
    acpi_put_acpi_dev(device);
    result
}

#[repr(C)]
struct acpi_irq_parse_one_ctx {
    rc: c_int,
    index: c_uint,
    res_flags: *mut c_ulong,
    fwspec: *mut irq_fwspec,
}

unsafe fn acpi_irq_parse_one_match(
    fwnode: *mut fwnode_handle, hwirq: u32, triggering: u8,
    polarity: u8, shareable: u8, wake_capable: u8,
    ctx: *mut acpi_irq_parse_one_ctx,
) {
    if fwnode.is_null() { return; }
    (*ctx).rc = 0;
    *(*ctx).res_flags = acpi_dev_irq_flags(triggering, polarity, shareable, wake_capable);
    (*(*ctx).fwspec).fwnode = fwnode;
    (*(*ctx).fwspec).param[0] = hwirq;
    (*(*ctx).fwspec).param[1] = acpi_dev_get_irq_type(triggering as c_int, polarity as c_int);
    (*(*ctx).fwspec).param_count = 2;
}

unsafe extern "C" fn acpi_irq_parse_one_cb(ares: *mut acpi_resource, context: *mut c_void) -> acpi_status {
    let ctx = context as *mut acpi_irq_parse_one_ctx;
    match (*ares).type_ {
        ACPI_RESOURCE_TYPE_IRQ => {
            let irq = &mut (*ares).data.irq;
            if (*ctx).index >= irq.interrupt_count { (*ctx).index -= irq.interrupt_count; return AE_OK; }
            let n = irq.interrupts[(*ctx).index as usize];
            let fwnode = (ACPI_GET_GSI_DOMAIN_ID.unwrap())(n);
            acpi_irq_parse_one_match(fwnode, n, irq.triggering, irq.polarity, irq.shareable, irq.wake_capable, ctx);
            AE_CTRL_TERMINATE
        }
        ACPI_RESOURCE_TYPE_EXTENDED_IRQ => {
            let eirq = &mut (*ares).data.extended_irq;
            if eirq.producer_consumer == ACPI_PRODUCER { return AE_OK; }
            if (*ctx).index >= eirq.interrupt_count { (*ctx).index -= eirq.interrupt_count; return AE_OK; }
            let n = eirq.interrupts[(*ctx).index as usize];
            let fwnode = acpi_get_irq_source_fwhandle(&eirq.resource_source, n);
            acpi_irq_parse_one_match(fwnode, n, eirq.triggering, eirq.polarity, eirq.shareable, eirq.wake_capable, ctx);
            AE_CTRL_TERMINATE
        }
        _ => AE_OK,
    }
}

unsafe fn acpi_irq_parse_one(handle: acpi_handle, index: c_uint, fwspec: *mut irq_fwspec, flags: *mut c_ulong) -> c_int {
    let mut ctx = acpi_irq_parse_one_ctx { rc: -EINVAL, index, res_flags: flags, fwspec };
    acpi_walk_resources(handle, METHOD_NAME__CRS, Some(acpi_irq_parse_one_cb), &mut ctx as *mut _ as *mut c_void);
    ctx.rc
}

pub unsafe extern "C" fn acpi_irq_get(handle: acpi_handle, index: c_uint, res: *mut resource) -> c_int {
    let mut fwspec: irq_fwspec = core::mem::zeroed();
    let mut flags: c_ulong = 0;
    let rc = acpi_irq_parse_one(handle, index, &mut fwspec, &mut flags);
    if rc != 0 { return rc; }
    let domain = irq_find_matching_fwnode(fwspec.fwnode, DOMAIN_BUS_ANY);
    if domain.is_null() { return -EPROBE_DEFER; }
    let rc = irq_create_fwspec_mapping(&mut fwspec) as c_int;
    if rc <= 0 { return -EINVAL; }
    (*res).start = rc as _; (*res).end = rc as _; (*res).flags = flags; 0
}

pub unsafe extern "C" fn acpi_irq_get_affinity(handle: acpi_handle, index: c_uint) -> *const cpumask {
    let mut info: irq_fwspec_info = core::mem::zeroed();
    let mut fwspec: irq_fwspec = core::mem::zeroed();
    let mut flags: c_ulong = 0;
    if acpi_irq_parse_one(handle, index, &mut fwspec, &mut flags) != 0 { return core::ptr::null(); }
    if irq_populate_fwspec_info(&mut fwspec, &mut info) != 0 { return core::ptr::null(); }
    if info.flags & IRQ_FWSPEC_INFO_AFFINITY_VALID == 0 { return core::ptr::null(); }
    info.affinity
}

pub unsafe extern "C" fn acpi_set_irq_model(model: acpi_irq_model_id, f: acpi_gsi_domain_disp_fn, gsi_dep_fn: acpi_gsi_handle_disp_fn) {
    ACPI_IRQ_MODEL = model; ACPI_GET_GSI_DOMAIN_ID = Some(f); ACPI_GET_GSI_HANDLE = Some(gsi_dep_fn);
}

pub unsafe extern "C" fn acpi_get_gsi_dispatcher() -> acpi_gsi_domain_disp_fn { ACPI_GET_GSI_DOMAIN_ID.unwrap() }

pub unsafe extern "C" fn acpi_set_gsi_to_irq_fallback(f: unsafe extern "C" fn(u32) -> u32) { ACPI_GSI_TO_IRQ_FALLBACK = Some(f); }

pub unsafe extern "C" fn acpi_irq_create_hierarchy(flags: c_uint, size: c_uint, fwnode: *mut fwnode_handle, ops: *const irq_domain_ops, host_data: *mut c_void) -> *mut irq_domain {
    if ACPI_IRQ_MODEL != ACPI_IRQ_MODEL_GIC { return core::ptr::null_mut(); }
    let d = irq_find_matching_fwnode((ACPI_GET_GSI_DOMAIN_ID.unwrap())(0), DOMAIN_BUS_ANY);
    if d.is_null() { return core::ptr::null_mut(); }
    irq_domain_create_hierarchy(d, flags, size, fwnode, ops, host_data)
}

#[repr(C)]
struct acpi_irq_dep_ctx { rc: c_int, index: c_uint, handle: acpi_handle }

unsafe extern "C" fn acpi_irq_get_parent(ares: *mut acpi_resource, context: *mut c_void) -> acpi_status {
    let ctx = context as *mut acpi_irq_dep_ctx;
    match (*ares).type_ {
        ACPI_RESOURCE_TYPE_IRQ => {
            let irq = &mut (*ares).data.irq;
            if (*ctx).index >= irq.interrupt_count { (*ctx).index -= irq.interrupt_count; return AE_OK; }
            (*ctx).handle = (ACPI_GET_GSI_HANDLE.unwrap())(irq.interrupts[(*ctx).index as usize]); (*ctx).rc = 0; AE_CTRL_TERMINATE
        }
        ACPI_RESOURCE_TYPE_EXTENDED_IRQ => {
            let eirq = &mut (*ares).data.extended_irq;
            if eirq.producer_consumer == ACPI_PRODUCER { return AE_OK; }
            if (*ctx).index >= eirq.interrupt_count { (*ctx).index -= eirq.interrupt_count; return AE_OK; }
            if eirq.resource_source.string_length != 0 { return AE_OK; }
            (*ctx).handle = (ACPI_GET_GSI_HANDLE.unwrap())(eirq.interrupts[(*ctx).index as usize]); (*ctx).rc = 0; AE_CTRL_TERMINATE
        }
        _ => AE_OK,
    }
}

unsafe fn acpi_irq_get_dep(handle: acpi_handle, index: c_uint, gsi_handle: *mut acpi_handle) -> c_int {
    let mut ctx = acpi_irq_dep_ctx { rc: -EINVAL, index, handle: core::ptr::null_mut() };
    if gsi_handle.is_null() { return -EINVAL; }
    acpi_walk_resources(handle, METHOD_NAME__CRS, Some(acpi_irq_get_parent), &mut ctx as *mut _ as *mut c_void);
    *gsi_handle = ctx.handle; ctx.rc
}

unsafe fn acpi_prt_entry_valid(prt_entry: *mut c_void) -> bool {
    !prt_entry.is_null() && (*(prt_entry as *mut acpi_pci_routing_table)).length > 0
}

unsafe fn acpi_prt_next_entry(prt_entry: *mut c_void) -> *mut c_void {
    prt_entry.add((*(prt_entry as *mut acpi_pci_routing_table)).length as usize)
}

unsafe fn acpi_add_prt_dep(handle: acpi_handle) -> u32 {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let status = acpi_get_irq_routing_table(handle, &mut buffer);
    if ACPI_FAILURE(status) { acpi_handle_err(handle, "failed to get IRQ routing table\n"); kfree(buffer.pointer); return 0; }
    let mut entry = buffer.pointer as *mut c_void; let mut count = 0;
    while acpi_prt_entry_valid(entry) {
        let e = entry as *mut acpi_pci_routing_table;
        let mut deps = acpi_handle_list { count: 1, handles: core::ptr::null_mut() };
        let dep = if (*e).source[0] != 0 { let mut lh = core::ptr::null_mut(); if ACPI_FAILURE(acpi_get_handle(handle, (*e).source.as_ptr(), &mut lh)) { entry = acpi_prt_next_entry(entry); continue; } lh } else { let gh = (ACPI_GET_GSI_HANDLE.unwrap())((*e).source_index); if gh.is_null() { entry = acpi_prt_next_entry(entry); continue; } gh };
        deps.handles = kcalloc(1, core::mem::size_of::<acpi_handle>(), GFP_KERNEL) as *mut acpi_handle;
        if deps.handles.is_null() { acpi_handle_err(handle, "failed to allocate memory\n"); entry = acpi_prt_next_entry(entry); continue; }
        *deps.handles = dep; count += acpi_scan_add_dep(handle, &mut deps); entry = acpi_prt_next_entry(entry);
    }
    kfree(buffer.pointer); count
}

unsafe fn acpi_add_irq_dep(handle: acpi_handle) -> u32 {
    let mut count = 0; let mut i = 0; let mut gsi_handle = core::ptr::null_mut();
    while acpi_irq_get_dep(handle, i, &mut gsi_handle) == 0 { i += 1; if gsi_handle.is_null() { continue; } let mut deps = acpi_handle_list { count: 1, handles: kcalloc(1, core::mem::size_of::<acpi_handle>(), GFP_KERNEL) as *mut acpi_handle }; if deps.handles.is_null() { acpi_handle_err(handle, "failed to allocate memory\n"); continue; } *deps.handles = gsi_handle; count += acpi_scan_add_dep(handle, &mut deps); }
    count
}

pub unsafe extern "C" fn acpi_irq_add_auto_dep(handle: acpi_handle) -> u32 {
    if ACPI_GET_GSI_HANDLE.is_none() { return 0; }
    if acpi_has_method(handle, b"_PRT\0".as_ptr() as *const c_char) { acpi_add_prt_dep(handle) } else { acpi_add_irq_dep(handle) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
