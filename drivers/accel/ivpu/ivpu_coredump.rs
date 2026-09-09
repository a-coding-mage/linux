// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// C dependencies supplied by the surrounding repository:
// linux/devcoredump.h, linux/firmware.h, ivpu_coredump.h, ivpu_fw.h,
// ivpu_gem.h, and vpu_boot_api.h.

const CRASH_DUMP_HEADER: &str = "Intel NPU crash dump";
const CRASH_DUMP_HEADERS_SIZE: usize = 4096; // SZ_4K

pub unsafe fn ivpu_dev_coredump(vdev: *mut ivpu_device) {
    let mut pi: drm_print_iterator = core::mem::zeroed();
    let mut p: drm_printer;
    let coredump_size: usize;
    let coredump: *mut core::ffi::c_char;

    coredump_size = CRASH_DUMP_HEADERS_SIZE
        + FW_VERSION_HEADER_SIZE
        + ivpu_bo_size((*(*vdev).fw).mem_log_crit)
        + ivpu_bo_size((*(*vdev).fw).mem_log_verb);
    coredump = vmalloc(coredump_size);
    if coredump.is_null() {
        return;
    }

    pi.data = coredump;
    pi.remain = coredump_size;
    p = drm_coredump_printer(&mut pi);

    drm_printf(&mut p, format_args!("{}\n", CRASH_DUMP_HEADER));
    drm_printf(
        &mut p,
        format_args!("FW version: {}\n", (*(*vdev).fw).version),
    );
    ivpu_fw_log_print(vdev, false, &mut p);

    dev_coredumpv((*vdev).drm.dev, coredump, pi.offset, GFP_KERNEL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
