// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux firmware, property, security, vmalloc,
// fallback, and firmware interfaces are intentionally not implemented here.

pub unsafe extern "C" fn firmware_fallback_platform(fw_priv: *mut fw_priv) -> i32 {
    let mut data: *const u8;
    let mut size: usize = 0;
    let mut rc: i32;

    if ((*fw_priv).opt_flags & FW_OPT_FALLBACK_PLATFORM) == 0 {
        return -ENOENT;
    }

    rc = security_kernel_load_data(LOADING_FIRMWARE, true);
    if rc != 0 {
        return rc;
    }

    rc = efi_get_embedded_fw((*fw_priv).fw_name, &mut data, &mut size);
    if rc != 0 {
        return rc; // rc == -ENOENT when the fw was not found
    }

    if !(*fw_priv).data.is_null() && size > (*fw_priv).allocated_size {
        return -ENOMEM;
    }

    rc = security_kernel_post_load_data(
        data as *mut u8,
        size,
        LOADING_FIRMWARE,
        b"platform\0".as_ptr() as *const i8,
    );
    if rc != 0 {
        return rc;
    }

    if (*fw_priv).data.is_null() {
        (*fw_priv).data = vmalloc(size);
    }
    if (*fw_priv).data.is_null() {
        return -ENOMEM;
    }

    core::ptr::copy_nonoverlapping(data, (*fw_priv).data, size);
    (*fw_priv).size = size;
    fw_state_done(fw_priv);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
