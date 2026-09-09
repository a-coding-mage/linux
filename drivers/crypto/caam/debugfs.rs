// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/* Copyright 2019, 2023 NXP */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/debugfs.h, compat.h, debugfs.h, regs.h, and intern.h.

unsafe fn caam_debugfs_u64_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    *val = caam64_to_cpu(*(data as *const u64));
    0
}

unsafe fn caam_debugfs_u32_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    *val = caam32_to_cpu(*(data as *const u32)) as u64;
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE(caam_fops_u32_ro, caam_debugfs_u32_get, NULL, "%llu\\n");
// DEFINE_DEBUGFS_ATTRIBUTE(caam_fops_u64_ro, caam_debugfs_u64_get, NULL, "%llu\\n");

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI)]
/*
 * This is a counter for the number of times the congestion group (where all
 * the request and response queueus are) reached congestion. Incremented
 * each time the congestion callback is called with congested == true.
 */
static mut times_congested: u64 = 0;

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI)]
pub unsafe fn caam_debugfs_qi_congested() {
    times_congested = times_congested.wrapping_add(1);
}

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI)]
pub unsafe fn caam_debugfs_qi_init(ctrlpriv: *mut caam_drv_private) {
    debugfs_create_file(
        b"qi_congested\\0".as_ptr() as *const core::ffi::c_char,
        0o444,
        (*ctrlpriv).ctl,
        &raw mut times_congested as *mut core::ffi::c_void,
        &caam_fops_u64_ro,
    );
}

pub unsafe fn caam_debugfs_init(
    ctrlpriv: *mut caam_drv_private,
    perfmon: *mut caam_perfmon,
    root: *mut dentry,
) {
    /*
     * FIXME: needs better naming distinction, as some amalgamation of
     * "caam" and nprop->full_name. The OF name isn't distinctive,
     * but does separate instances
     */

    (*ctrlpriv).ctl = debugfs_create_dir(b"ctl\\0".as_ptr() as *const core::ffi::c_char, root);

    debugfs_create_file(b"rq_dequeued\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).req_dequeued as *mut _ as *mut core::ffi::c_void, &caam_fops_u64_ro);
    debugfs_create_file(b"ob_rq_encrypted\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).ob_enc_req as *mut _ as *mut core::ffi::c_void, &caam_fops_u64_ro);
    debugfs_create_file(b"ib_rq_decrypted\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).ib_dec_req as *mut _ as *mut core::ffi::c_void, &caam_fops_u64_ro);
    debugfs_create_file(b"ob_bytes_encrypted\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).ob_enc_bytes as *mut _ as *mut core::ffi::c_void, &caam_fops_u64_ro);
    debugfs_create_file(b"ob_bytes_protected\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).ob_prot_bytes as *mut _ as *mut core::ffi::c_void, &caam_fops_u64_ro);
    debugfs_create_file(b"ib_bytes_decrypted\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).ib_dec_bytes as *mut _ as *mut core::ffi::c_void, &caam_fops_u64_ro);
    debugfs_create_file(b"ib_bytes_validated\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).ib_valid_bytes as *mut _ as *mut core::ffi::c_void, &caam_fops_u64_ro);

    /* Controller level - global status values */
    debugfs_create_file(b"fault_addr\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).faultaddr as *mut _ as *mut core::ffi::c_void, &caam_fops_u32_ro);
    debugfs_create_file(b"fault_detail\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).faultdetail as *mut _ as *mut core::ffi::c_void, &caam_fops_u32_ro);
    debugfs_create_file(b"fault_status\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*perfmon).status as *mut _ as *mut core::ffi::c_void, &caam_fops_u32_ro);

    if (*ctrlpriv).optee_en {
        return;
    }

    /* Internal covering keys (useful in non-secure mode only) */
    (*ctrlpriv).ctl_kek_wrap.data = (*ctrlpriv).ctrl.kek.as_mut_ptr() as *mut core::ffi::c_void;
    (*ctrlpriv).ctl_kek_wrap.size = KEK_KEY_SIZE * core::mem::size_of::<u32>();
    debugfs_create_blob(b"kek\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*ctrlpriv).ctl_kek_wrap);

    (*ctrlpriv).ctl_tkek_wrap.data = (*ctrlpriv).ctrl.tkek.as_mut_ptr() as *mut core::ffi::c_void;
    (*ctrlpriv).ctl_tkek_wrap.size = KEK_KEY_SIZE * core::mem::size_of::<u32>();
    debugfs_create_blob(b"tkek\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*ctrlpriv).ctl_tkek_wrap);

    (*ctrlpriv).ctl_tdsk_wrap.data = (*ctrlpriv).ctrl.tdsk.as_mut_ptr() as *mut core::ffi::c_void;
    (*ctrlpriv).ctl_tdsk_wrap.size = KEK_KEY_SIZE * core::mem::size_of::<u32>();
    debugfs_create_blob(b"tdsk\\0".as_ptr() as *const core::ffi::c_char, 0o444, (*ctrlpriv).ctl, &mut (*ctrlpriv).ctl_tdsk_wrap);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
