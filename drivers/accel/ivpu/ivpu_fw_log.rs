// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// Linux kernel and local header dependencies are supplied by the surrounding translation unit.

const IVPU_FW_LOG_LINE_LENGTH: usize = 256;

pub static mut ivpu_fw_log_level: u32 = IVPU_FW_LOG_ERROR;

unsafe fn fw_log_from_bo(
    vdev: *mut ivpu_device,
    bo: *mut ivpu_bo,
    offset: *mut u32,
    out_log: *mut *mut vpu_tracing_buffer_header,
) -> i32 {
    let log: *mut vpu_tracing_buffer_header;

    if (*offset as usize + core::mem::size_of::<vpu_tracing_buffer_header>()) > ivpu_bo_size(bo) {
        return -EINVAL;
    }

    log = (ivpu_bo_vaddr(bo) as *mut u8)
        .add(*offset as usize) as *mut vpu_tracing_buffer_header;

    if (*log).vpu_canary_start != VPU_TRACING_BUFFER_CANARY {
        return -EINVAL;
    }

    if (*log).header_size < core::mem::size_of::<vpu_tracing_buffer_header>() as _
        || (*log).header_size > 1024
    {
        ivpu_dbg(vdev, FW_BOOT, "Invalid header size 0x%x\n", (*log).header_size);
        return -EINVAL;
    }
    if (*log).size < (*log).header_size {
        ivpu_dbg(vdev, FW_BOOT, "Invalid log size 0x%x\n", (*log).size);
        return -EINVAL;
    }
    if (log as *mut u8).add((*log).size as usize)
        > (ivpu_bo_vaddr(bo) as *mut u8).add(ivpu_bo_size(bo))
    {
        ivpu_dbg(vdev, FW_BOOT, "Invalid log size 0x%x\n", (*log).size);
        return -EINVAL;
    }

    *out_log = log;
    *offset += (*log).size;

    ivpu_dbg(
        vdev,
        FW_BOOT,
        "FW log name \"%s\", write offset 0x%x size 0x%x, wrap count %d, hdr version %d size %d format %d, alignment %d",
        (*log).name,
        (*log).write_index,
        (*log).size,
        (*log).wrap_count,
        (*log).header_version,
        (*log).header_size,
        (*log).format,
        (*log).alignment,
    );

    0
}

unsafe fn fw_log_print_lines(mut buffer: *mut i8, mut size: u32, p: *mut drm_printer) {
    let mut line = [0i8; IVPU_FW_LOG_LINE_LENGTH];
    let mut index: usize = 0;

    if size == 0 || buffer.is_null() {
        return;
    }

    while size != 0 {
        size -= 1;
        if *buffer == b'\n' as i8 || *buffer == 0 {
            line[index] = 0;
            if index != 0 {
                drm_printf(p, "%s\n", line.as_mut_ptr());
            }
            index = 0;
            buffer = buffer.add(1);
            continue;
        }
        if index == IVPU_FW_LOG_LINE_LENGTH - 1 {
            line[index] = 0;
            index = 0;
            drm_printf(p, "%s\n", line.as_mut_ptr());
        }
        if *buffer != b'\r' as i8 && (isprint(*buffer) || iscntrl(*buffer)) {
            line[index] = *buffer;
            index += 1;
        }
        buffer = buffer.add(1);
    }
    line[index] = 0;
    if index != 0 {
        drm_printf(p, "%s", line.as_mut_ptr());
    }
}

unsafe fn fw_log_print_buffer(
    log: *mut vpu_tracing_buffer_header,
    prefix: *const i8,
    only_new_msgs: bool,
    p: *mut drm_printer,
) {
    let log_data = (log as *mut u8).add((*log).header_size as usize) as *mut i8;
    let data_size = (*log).size - (*log).header_size;
    let mut log_start = if only_new_msgs { READ_ONCE((*log).read_index) } else { 0 };
    let mut log_end = READ_ONCE((*log).write_index);

    if log_start >= data_size { log_start = 0; }
    if log_end > data_size { log_end = data_size; }

    if (*log).wrap_count == (*log).read_wrap_count {
        if log_end <= log_start {
            drm_printf(p, "==== %s \"%s\" log empty ====\n", prefix, (*log).name);
            return;
        }
    } else if (*log).wrap_count == (*log).read_wrap_count + 1 {
        if log_end > log_start { log_start = log_end; }
    } else { log_start = log_end; }

    drm_printf(p, "==== %s \"%s\" log start ====\n", prefix, (*log).name);
    if log_end > log_start {
        fw_log_print_lines(log_data.add(log_start as usize), log_end - log_start, p);
    } else {
        fw_log_print_lines(log_data.add(log_start as usize), data_size - log_start, p);
        fw_log_print_lines(log_data, log_end, p);
    }
    drm_printf(p, "\n\x1b[0m");
    drm_printf(p, "==== %s \"%s\" log end   ====\n", prefix, (*log).name);
}

unsafe fn fw_log_print_all_in_bo(
    vdev: *mut ivpu_device, name: *const i8, bo: *mut ivpu_bo,
    only_new_msgs: bool, p: *mut drm_printer,
) {
    let mut log: *mut vpu_tracing_buffer_header = core::ptr::null_mut();
    let mut next: u32 = 0;
    while fw_log_from_bo(vdev, bo, &mut next, &mut log) == 0 {
        fw_log_print_buffer(log, name, only_new_msgs, p);
    }
}

pub unsafe fn ivpu_fw_log_print(vdev: *mut ivpu_device, only_new_msgs: bool, p: *mut drm_printer) {
    fw_log_print_all_in_bo(vdev, b"NPU critical\0".as_ptr() as *const i8, (*vdev).fw.mem_log_crit, only_new_msgs, p);
    fw_log_print_all_in_bo(vdev, b"NPU verbose\0".as_ptr() as *const i8, (*vdev).fw.mem_log_verb, only_new_msgs, p);
}

pub unsafe fn ivpu_fw_log_mark_read(vdev: *mut ivpu_device) {
    let mut log = core::ptr::null_mut();
    let mut next = 0;
    while fw_log_from_bo(vdev, (*vdev).fw.mem_log_crit, &mut next, &mut log) == 0 {
        (*log).read_index = READ_ONCE((*log).write_index);
        (*log).read_wrap_count = READ_ONCE((*log).wrap_count);
    }
    next = 0;
    while fw_log_from_bo(vdev, (*vdev).fw.mem_log_verb, &mut next, &mut log) == 0 {
        (*log).read_index = READ_ONCE((*log).write_index);
        (*log).read_wrap_count = READ_ONCE((*log).wrap_count);
    }
}

pub unsafe fn ivpu_fw_log_reset(vdev: *mut ivpu_device) {
    let mut log = core::ptr::null_mut();
    let mut next = 0;
    while fw_log_from_bo(vdev, (*vdev).fw.mem_log_crit, &mut next, &mut log) == 0 {
        (*log).read_index = 0; (*log).read_wrap_count = 0;
    }
    next = 0;
    while fw_log_from_bo(vdev, (*vdev).fw.mem_log_verb, &mut next, &mut log) == 0 {
        (*log).read_index = 0; (*log).read_wrap_count = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
