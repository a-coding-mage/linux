// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn validate_of_partition(np: *mut device_node, _slot: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut offset: u64;
    let mut size: u64;
    let mut len: ::core::ffi::c_int = 0;

    let reg: *const __be32 = of_get_property(np, b"reg\0".as_ptr() as *const ::core::ffi::c_char, &mut len);
    let a_cells: ::core::ffi::c_int = of_n_addr_cells(np);
    let s_cells: ::core::ffi::c_int = of_n_size_cells(np);

    /* Make sure reg len match the expected addr and size cells */
    if len / ::core::mem::size_of::<__be32>() as ::core::ffi::c_int != a_cells + s_cells {
        return -EINVAL;
    }

    /* Validate offset conversion from bytes to sectors */
    offset = of_read_number(reg, a_cells);
    if offset % SECTOR_SIZE != 0 {
        return -EINVAL;
    }

    /* Validate size conversion from bytes to sectors */
    size = of_read_number(reg.add(a_cells as usize), s_cells);
    if size == 0 || size % SECTOR_SIZE != 0 {
        return -EINVAL;
    }

    0
}

unsafe fn add_of_partition(
    state: *mut parsed_partitions,
    slot: ::core::ffi::c_int,
    np: *mut device_node,
) {
    let info: *mut partition_meta_info;
    let mut partname: *const ::core::ffi::c_char;
    let mut len: ::core::ffi::c_int = 0;

    let reg: *const __be32 = of_get_property(np, b"reg\0".as_ptr() as *const ::core::ffi::c_char, &mut len);
    let a_cells: ::core::ffi::c_int = of_n_addr_cells(np);
    let s_cells: ::core::ffi::c_int = of_n_size_cells(np);

    /* Convert bytes to sector size */
    let offset: u64 = of_read_number(reg, a_cells) / SECTOR_SIZE;
    let size: u64 = of_read_number(reg.add(a_cells as usize), s_cells) / SECTOR_SIZE;

    put_partition(state, slot, offset, size);

    if of_property_read_bool(np, b"read-only\0".as_ptr() as *const ::core::ffi::c_char) {
        (*state).parts[slot as usize].flags |= ADDPART_FLAG_READONLY;
    }

    /*
     * Follow MTD label logic, search for label property,
     * fallback to node name if not found.
     */
    info = &mut (*state).parts[slot as usize].info;
    partname = of_get_property(np, b"label\0".as_ptr() as *const ::core::ffi::c_char, &mut len);
    if partname.is_null() {
        partname = of_get_property(np, b"name\0".as_ptr() as *const ::core::ffi::c_char, &mut len);
    }
    strscpy((*info).volname.as_mut_ptr(), partname, ::core::mem::size_of_val(&(*info).volname));

    seq_buf_printf(&mut (*state).pp_buf, b"(%s)\0".as_ptr() as *const ::core::ffi::c_char, (*info).volname.as_ptr());
}

pub unsafe fn of_partition(state: *mut parsed_partitions) -> ::core::ffi::c_int {
    let ddev: *mut device = disk_to_dev((*state).disk);
    let mut np: *mut device_node;
    let mut slot: ::core::ffi::c_int;

    let partitions_np: *mut device_node = of_node_get((*ddev).of_node);

    if partitions_np.is_null()
        || !of_device_is_compatible(
            partitions_np,
            b"fixed-partitions\0".as_ptr() as *const ::core::ffi::c_char,
        )
    {
        of_node_put(partitions_np);
        return 0;
    }

    slot = 1;
    /* Validate parition offset and size */
    // Equivalent expansion of for_each_child_of_node(partitions_np, np).
    np = of_get_next_child(partitions_np, ::core::ptr::null_mut());
    while !np.is_null() {
        if validate_of_partition(np, slot) != 0 {
            of_node_put(np);
            of_node_put(partitions_np);
            return -1;
        }
        slot += 1;
        np = of_get_next_child(partitions_np, np);
    }

    slot = 1;
    np = of_get_next_child(partitions_np, ::core::ptr::null_mut());
    while !np.is_null() {
        if slot >= (*state).limit {
            of_node_put(np);
            break;
        }

        add_of_partition(state, slot, np);
        slot += 1;
        np = of_get_next_child(partitions_np, np);
    }

    seq_buf_puts(&mut (*state).pp_buf, b"\n\0".as_ptr() as *const ::core::ffi::c_char);

    of_node_put(partitions_np);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
