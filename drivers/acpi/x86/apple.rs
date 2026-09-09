// SPDX-License-Identifier: GPL-2.0-only
/*
 * apple.c - Apple ACPI quirks
 * Copyright (C) 2017 Lukas Wunner <lukas@wunner.de>
 */

// Apple _DSM device properties GUID
static apple_prp_guid: guid_t = GUID_INIT!(
    0xa0b5b7c6, 0x1318, 0x441c,
    0xb0, 0xc9, 0xfe, 0x69, 0x5e, 0xaf, 0x94, 0x9b
);

/**
 * acpi_extract_apple_properties - retrieve and convert Apple _DSM properties
 * @adev: ACPI device for which to retrieve the properties
 *
 * Invoke Apple's custom _DSM once to check the protocol version and once more
 * to retrieve the properties.  They are marshalled up in a single package as
 * alternating key/value elements, unlike _DSD which stores them as a package
 * of 2-element packages.  Convert to _DSD format and make them available under
 * the primary fwnode.
 */
pub unsafe fn acpi_extract_apple_properties(adev: *mut acpi_device) {
    let mut i: c_uint;
    let mut j: c_uint = 0;
    let mut newsize: c_uint = 0;
    let numprops: c_uint;
    let numvalid: c_uint;
    let mut props: *mut acpi_object;
    let mut newprops: *mut acpi_object;
    let mut valid: *mut c_ulong = core::ptr::null_mut();
    let mut free_space: *mut c_void;

    macro_rules! goto_out_free {
        ($p:expr, $v:expr) => {{
            ACPI_FREE!($p);
            bitmap_free($v);
            return;
        }};
    }

    if !x86_apple_machine {
        return;
    }

    props = acpi_evaluate_dsm_typed(
        (*adev).handle, &apple_prp_guid, 1, 0,
        core::ptr::null_mut(), ACPI_TYPE_BUFFER,
    );
    if props.is_null() {
        return;
    }

    if (*props).buffer.length == 0 {
        goto_out_free!(props, valid);
    }

    if (*props).buffer.pointer.read() != 3 {
        acpi_handle_info!(
            (*adev).handle, FW_INFO,
            "unsupported properties version %*ph\\n",
            (*props).buffer.length, (*props).buffer.pointer,
        );
        goto_out_free!(props, valid);
    }

    ACPI_FREE!(props);
    props = acpi_evaluate_dsm_typed(
        (*adev).handle, &apple_prp_guid, 1, 1,
        core::ptr::null_mut(), ACPI_TYPE_PACKAGE,
    );
    if props.is_null() {
        return;
    }

    numprops = (*props).package.count / 2;
    if numprops == 0 {
        goto_out_free!(props, valid);
    }

    valid = bitmap_zalloc(numprops, GFP_KERNEL);
    if valid.is_null() {
        goto_out_free!(props, valid);
    }

    /* newsize = key length + value length of each tuple */
    i = 0;
    while i < numprops {
        let key = (*props).package.elements.add((i * 2) as usize);
        let val = (*props).package.elements.add((i * 2 + 1) as usize);

        if (*key).type_ != ACPI_TYPE_STRING ||
            ((*val).type_ != ACPI_TYPE_INTEGER &&
             (*val).type_ != ACPI_TYPE_BUFFER &&
             (*val).type_ != ACPI_TYPE_STRING) {
            i += 1;
            continue; /* skip invalid properties */
        }

        __set_bit(i, valid);
        newsize += (*key).string.length + 1;
        if (*val).type_ == ACPI_TYPE_BUFFER {
            newsize += (*val).buffer.length;
        } else if (*val).type_ == ACPI_TYPE_STRING {
            newsize += (*val).string.length + 1;
        }
        i += 1;
    }

    numvalid = bitmap_weight(valid, numprops);
    if numprops > numvalid {
        acpi_handle_info!(
            (*adev).handle, FW_INFO,
            "skipped %u properties: wrong type\\n", numprops - numvalid,
        );
    }
    if numvalid == 0 {
        goto_out_free!(props, valid);
    }

    /* newsize += top-level package + 3 objects for each key/value tuple */
    newsize += (1 + 3 * numvalid) * core::mem::size_of::<acpi_object>() as c_uint;
    newprops = ACPI_ALLOCATE_ZEROED!(newsize) as *mut acpi_object;
    if newprops.is_null() {
        goto_out_free!(props, valid);
    }

    /* layout: top-level package | packages | key/value tuples | strings */
    (*newprops).type_ = ACPI_TYPE_PACKAGE;
    (*newprops).package.count = numvalid;
    (*newprops).package.elements = newprops.add(1);
    free_space = newprops.add((1 + 3 * numvalid) as usize) as *mut c_void;

    for_each_set_bit!(i, valid, numprops) {
        let key = (*props).package.elements.add((i * 2) as usize);
        let val = (*props).package.elements.add((i * 2 + 1) as usize);
        let k: usize = (1 + numvalid + j * 2) as usize;
        let v = k + 1;

        (*newprops.add(1 + j as usize)).type_ = ACPI_TYPE_PACKAGE;
        (*newprops.add(1 + j as usize)).package.count = 2;
        (*newprops.add(1 + j as usize)).package.elements = newprops.add(k);

        (*newprops.add(k)).type_ = ACPI_TYPE_STRING;
        (*newprops.add(k)).string.length = (*key).string.length;
        (*newprops.add(k)).string.pointer = free_space as *mut u8;
        core::ptr::copy_nonoverlapping((*key).string.pointer, free_space as *mut u8, (*key).string.length as usize);
        free_space = (free_space as *mut u8).add((*key).string.length as usize + 1) as *mut c_void;

        (*newprops.add(v)).type_ = (*val).type_;
        if (*val).type_ == ACPI_TYPE_INTEGER {
            (*newprops.add(v)).integer.value = (*val).integer.value;
        } else if (*val).type_ == ACPI_TYPE_STRING {
            (*newprops.add(v)).string.length = (*val).string.length;
            (*newprops.add(v)).string.pointer = free_space as *mut u8;
            core::ptr::copy_nonoverlapping((*val).string.pointer, free_space as *mut u8, (*val).string.length as usize);
            free_space = (free_space as *mut u8).add((*val).string.length as usize + 1) as *mut c_void;
        } else {
            (*newprops.add(v)).buffer.length = (*val).buffer.length;
            (*newprops.add(v)).buffer.pointer = free_space as *mut u8;
            core::ptr::copy_nonoverlapping((*val).buffer.pointer, free_space as *mut u8, (*val).buffer.length as usize);
            free_space = (free_space as *mut u8).add((*val).buffer.length as usize) as *mut c_void;
        }
        j += 1; /* count valid properties */
    }
    WARN_ON!(free_space != (newprops as *mut u8).add(newsize as usize) as *mut c_void);

    (*adev).data.pointer = newprops as *mut c_void;
    acpi_data_add_props(&mut (*adev).data, &apple_prp_guid, newprops);

    ACPI_FREE!(props);
    bitmap_free(valid);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
