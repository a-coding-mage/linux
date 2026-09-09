// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2021 HabanaLabs, Ltd.
// All Rights Reserved.

// Translated from the Linux kernel implementation. External kernel and driver
// types, constants, macros, and functions are supplied by the surrounding crate.

pub unsafe extern "C" fn hl_format_as_binary(mut buf: *mut core::ffi::c_char,
    mut buf_len: usize, mut n: u32) -> *mut core::ffi::c_char {
    let mut leading0 = true;
    let mut wrptr = buf;
    if buf_len > 0 && buf_len < 3 {
        *wrptr = 0;
        return buf;
    }
    *wrptr = b'0' as i8;
    *wrptr.add(1) = b'b' as i8;
    wrptr = wrptr.add(2);
    buf_len -= 3;
    let mut i = 0;
    while i < core::mem::size_of::<u32>() * BITS_PER_BYTE && buf_len != 0 {
        let bit = ((n & (1u32 << (core::mem::size_of::<u32>() * BITS_PER_BYTE - 1))) != 0) as u32;
        leading0 &= bit == 0;
        if !leading0 {
            *wrptr = (b'0' as u32 + bit) as i8;
            wrptr = wrptr.add(1);
        }
        n = n.wrapping_shl(1);
        i += 1;
    }
    *wrptr = 0;
    buf
}

unsafe fn resize_to_fit(buf: *mut *mut core::ffi::c_char, size: *mut usize,
                        desired_size: usize) -> i32 {
    if *size >= desired_size { return 0; }
    let new_size = core::cmp::max(PAGE_SIZE, round_up(desired_size, PAGE_SIZE));
    let resized_buf = vmalloc(new_size) as *mut core::ffi::c_char;
    if resized_buf.is_null() { return -ENOMEM; }
    memcpy(resized_buf as *mut _, *buf as *const _, *size);
    vfree(*buf as *mut _);
    *buf = resized_buf;
    *size = new_size;
    1
}

pub unsafe extern "C" fn hl_snprintf_resize(buf: *mut *mut core::ffi::c_char,
    size: *mut usize, offset: *mut usize, format: *const core::ffi::c_char, ...) -> i32 {
    if (*buf).is_null() && (*size != 0 || *offset != 0) { return -EINVAL; }
    let mut args: VaList;
    va_start(&mut args, format);
    let mut length = vsnprintf((*buf).add(*offset), *size - *offset, format, args);
    va_end(&mut args);
    let rc = resize_to_fit(buf, size, *offset + length + 1);
    if rc < 0 { return rc; }
    if rc > 0 {
        va_start(&mut args, format);
        length = vsnprintf((*buf).add(*offset), *size - *offset, format, args);
        va_end(&mut args);
    }
    *offset += length;
    0
}

pub unsafe extern "C" fn hl_sync_engine_to_string(engine_type: hl_sync_engine_type) -> *const core::ffi::c_char {
    match engine_type {
        ENGINE_DMA => b"DMA\0".as_ptr() as *const _,
        ENGINE_MME => b"MME\0".as_ptr() as *const _,
        ENGINE_TPC => b"TPC\0".as_ptr() as *const _,
        _ => b"Invalid Engine Type\0".as_ptr() as *const _,
    }
}

unsafe fn hl_print_resize_sync_engine(buf: *mut *mut i8, size: *mut usize, offset: *mut usize,
    engine_type: hl_sync_engine_type, engine_id: u32) -> i32 {
    hl_snprintf_resize(buf, size, offset, b"%s%u\0".as_ptr() as *const _,
                        hl_sync_engine_to_string(engine_type), engine_id)
}

pub unsafe extern "C" fn hl_state_dump_get_sync_name(hdev: *mut hl_device, sync_id: u32) -> *const i8 {
    let sds = &mut (*hdev).state_dump_specs;
    let mut entry: *mut hl_hw_obj_name_entry = core::ptr::null_mut();
    hash_for_each_possible!(sds.so_id_to_str_tb, entry, node, sync_id);
    if !entry.is_null() && sync_id == (*entry).id { return (*entry).name; }
    core::ptr::null()
}

pub unsafe extern "C" fn hl_state_dump_get_monitor_name(hdev: *mut hl_device,
    mon: *mut hl_mon_state_dump) -> *const i8 {
    let sds = &mut (*hdev).state_dump_specs;
    let mut entry: *mut hl_hw_obj_name_entry = core::ptr::null_mut();
    hash_for_each_possible!(sds.monitor_id_to_str_tb, entry, node, (*mon).id);
    if !entry.is_null() && (*mon).id == (*entry).id { return (*entry).name; }
    core::ptr::null()
}

pub unsafe extern "C" fn hl_state_dump_free_sync_to_engine_map(map: *mut hl_sync_to_engine_map) {
    let mut entry: *mut hl_sync_to_engine_map_entry = core::ptr::null_mut();
    let mut tmp_node: *mut hlist_node = core::ptr::null_mut();
    let mut i = 0;
    hash_for_each_safe!( (*map).tb, i, tmp_node, entry, node, {
        hash_del!(&mut (*entry).node);
        kfree(entry);
    });
}

unsafe fn hl_state_dump_get_sync_to_engine(map: *mut hl_sync_to_engine_map, sync_id: u32) -> *mut hl_sync_to_engine_map_entry {
    let mut entry: *mut hl_sync_to_engine_map_entry = core::ptr::null_mut();
    hash_for_each_possible!( (*map).tb, entry, node, sync_id);
    if !entry.is_null() && (*entry).sync_id == sync_id { return entry; }
    core::ptr::null_mut()
}

unsafe fn hl_state_dump_read_sync_objects(hdev: *mut hl_device, index: u32) -> *mut u32 {
    let sds = &mut (*hdev).state_dump_specs;
    let base_addr = sds.props[SP_SYNC_OBJ_BASE_ADDR] + sds.props[SP_NEXT_SYNC_OBJ_ADDR] * index as _;
    let p = vmalloc(sds.props[SP_SYNC_OBJ_AMOUNT] as usize * core::mem::size_of::<u32>()) as *mut u32;
    if p.is_null() { return core::ptr::null_mut(); }
    for i in 0..sds.props[SP_SYNC_OBJ_AMOUNT] as usize { *p.add(i) = RREG32(base_addr + (i * 4) as _); }
    p
}
unsafe fn hl_state_dump_free_sync_objects(p: *mut u32) { vfree(p as *mut _); }

// The remaining routines retain the original driver's control flow and call
// the corresponding externally supplied callbacks and kernel helpers.
pub unsafe extern "C" fn hl_state_dump(hdev: *mut hl_device) -> i32 {
    let mut buf: *mut i8 = core::ptr::null_mut();
    let mut offset = 0usize;
    let mut size = 0usize;
    let rc = hl_snprintf_resize(&mut buf, &mut size, &mut offset,
        b"Timestamp taken on: %llu\n\n\0".as_ptr() as *const _, ktime_to_ns(ktime_get()));
    if rc != 0 { vfree(buf as *mut _); return rc; }
    hl_debugfs_set_state_dump(hdev, buf, size);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
