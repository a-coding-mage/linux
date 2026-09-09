// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2005 Intel Corporation
 * Copyright (C) 2009 Hewlett-Packard Development Company, L.P.
 *
 *      Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
 *      - Added _PDC for platforms with Intel CPUs
 */

// #define pr_fmt(fmt) "ACPI: " fmt
// Dependencies are supplied by the surrounding kernel translation unit.

unsafe fn acpi_set_pdc_bits(buf: *mut u32) {
    *buf.add(0) = ACPI_PDC_REVISION_ID;
    *buf.add(1) = 1;
    *buf.add(2) = 0;

    /* Twiddle arch-specific bits needed for _PDC */
    arch_acpi_set_proc_cap_bits(buf.add(2));
}

unsafe fn acpi_processor_alloc_pdc() -> *mut acpi_object_list {
    let obj_list: *mut acpi_object_list;
    let obj: *mut acpi_object;
    let buf: *mut u32;

    /* allocate and initialize pdc. It will be used later. */
    obj_list = kmalloc_obj::<acpi_object_list>();
    if obj_list.is_null() {
        pr_err!("Memory allocation error\n");
        return core::ptr::null_mut();
    }

    obj = kmalloc_obj::<acpi_object>();
    if obj.is_null() {
        kfree(obj_list);
        pr_err!("Memory allocation error\n");
        return core::ptr::null_mut();
    }

    buf = kmalloc(12, GFP_KERNEL) as *mut u32;
    if buf.is_null() {
        kfree(obj);
        kfree(obj_list);
        pr_err!("Memory allocation error\n");
        return core::ptr::null_mut();
    }

    acpi_set_pdc_bits(buf);

    (*obj).type_ = ACPI_TYPE_BUFFER;
    (*obj).buffer.length = 12;
    (*obj).buffer.pointer = buf as *mut u8;
    (*obj_list).count = 1;
    (*obj_list).pointer = obj;

    obj_list
}

/*
 * _PDC is required for a BIOS-OS handshake for most of the newer
 * ACPI processor features.
 */
unsafe fn acpi_processor_eval_pdc(
    handle: acpi_handle,
    pdc_in: *mut acpi_object_list,
) -> acpi_status {
    let status = acpi_evaluate_object(handle, b"_PDC\0".as_ptr() as *const i8, pdc_in, core::ptr::null_mut());

    if ACPI_FAILURE(status) {
        acpi_handle_debug(
            handle,
            "Could not evaluate _PDC, using legacy perf control\n",
        );
    }

    status
}

pub unsafe fn acpi_processor_set_pdc(handle: acpi_handle) {
    let obj_list: *mut acpi_object_list;

    if arch_has_acpi_pdc() == false {
        return;
    }

    obj_list = acpi_processor_alloc_pdc();
    if obj_list.is_null() {
        return;
    }

    acpi_processor_eval_pdc(handle, obj_list);

    kfree((*(*obj_list).pointer).buffer.pointer);
    kfree((*obj_list).pointer);
    kfree(obj_list);
}

unsafe fn early_init_pdc(
    handle: acpi_handle,
    _lvl: u32,
    _context: *mut core::ffi::c_void,
    _rv: *mut *mut core::ffi::c_void,
) -> acpi_status {
    if processor_physically_present(handle) == false {
        return AE_OK;
    }

    acpi_processor_set_pdc(handle);
    AE_OK
}

pub unsafe fn acpi_early_processor_set_pdc() {
    acpi_proc_quirk_mwait_check();

    acpi_walk_namespace(
        ACPI_TYPE_PROCESSOR,
        ACPI_ROOT_OBJECT,
        ACPI_UINT32_MAX,
        Some(early_init_pdc),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    acpi_get_devices(
        ACPI_PROCESSOR_DEVICE_HID,
        Some(early_init_pdc),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
