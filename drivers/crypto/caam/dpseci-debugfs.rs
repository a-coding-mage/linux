// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/* Copyright 2019 NXP */

// Dependencies supplied by the surrounding kernel/debugfs implementation:
// linux/module.h, linux/device.h, linux/debugfs.h, and dpseci-debugfs.h.

#[allow(non_camel_case_types)]
struct dpaa2_caam_priv {
    dev: *mut device,
    num_pairs: i32,
    rx_queue_attr: *mut dpaa2_queue_attr,
    tx_queue_attr: *mut dpaa2_queue_attr,
    dfs_root: *mut dentry,
}

#[allow(non_camel_case_types)]
struct device;
#[allow(non_camel_case_types)]
struct dentry;
#[allow(non_camel_case_types)]
struct seq_file {
    private: *mut core::ffi::c_void,
}
#[allow(non_camel_case_types)]
struct dpaa2_queue_attr {
    fqid: u32,
}
#[allow(non_camel_case_types)]
struct file_operations;

extern "C" {
    fn dev_name(dev: *const device) -> *const core::ffi::c_char;
    fn seq_printf(file: *mut seq_file, format: *const core::ffi::c_char, ...);
    fn dpaa2_io_query_fq_count(
        portal: *mut core::ffi::c_void,
        fqid: u32,
        fcnt: *mut u32,
        bcnt: *mut u32,
    ) -> i32;
    fn debugfs_create_dir(
        name: *const core::ffi::c_char,
        parent: *mut dentry,
    ) -> *mut dentry;
    fn debugfs_create_file(
        name: *const core::ffi::c_char,
        mode: u32,
        parent: *mut dentry,
        data: *mut core::ffi::c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
}

// DEFINE_SHOW_ATTRIBUTE(dpseci_dbg_fqs);
extern "C" {
    static dpseci_dbg_fqs_fops: file_operations;
}

unsafe extern "C" fn dpseci_dbg_fqs_show(
    file: *mut seq_file,
    _offset: *mut core::ffi::c_void,
) -> i32 {
    let priv_ = (*file).private as *mut dpaa2_caam_priv;
    let mut fqid: u32;
    let mut fcnt: u32 = 0;
    let mut bcnt: u32 = 0;
    let mut i: i32;
    let mut err: i32;

    seq_printf(file, c"FQ stats for %s:\n".as_ptr(), dev_name((*priv_).dev));
    seq_printf(
        file,
        c"%s%16s%16s\n".as_ptr(),
        c"Rx-VFQID".as_ptr(),
        c"Pending frames".as_ptr(),
        c"Pending bytes".as_ptr(),
    );

    i = 0;
    while i < (*priv_).num_pairs {
        fqid = (*(*priv_).rx_queue_attr.add(i as usize)).fqid;
        err = dpaa2_io_query_fq_count(core::ptr::null_mut(), fqid, &mut fcnt, &mut bcnt);
        if err != 0 {
            i += 1;
            continue;
        }

        seq_printf(file, c"%5d%16u%16u\n".as_ptr(), fqid, fcnt, bcnt);
        i += 1;
    }

    seq_printf(
        file,
        c"%s%16s%16s\n".as_ptr(),
        c"Tx-VFQID".as_ptr(),
        c"Pending frames".as_ptr(),
        c"Pending bytes".as_ptr(),
    );

    i = 0;
    while i < (*priv_).num_pairs {
        fqid = (*(*priv_).tx_queue_attr.add(i as usize)).fqid;
        err = dpaa2_io_query_fq_count(core::ptr::null_mut(), fqid, &mut fcnt, &mut bcnt);
        if err != 0 {
            i += 1;
            continue;
        }

        seq_printf(file, c"%5d%16u%16u\n".as_ptr(), fqid, fcnt, bcnt);
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn dpaa2_dpseci_debugfs_init(priv_: *mut dpaa2_caam_priv) {
    (*priv_).dfs_root = debugfs_create_dir(dev_name((*priv_).dev), core::ptr::null_mut());

    debugfs_create_file(
        c"fq_stats".as_ptr(),
        0o444,
        (*priv_).dfs_root,
        priv_ as *mut core::ffi::c_void,
        &dpseci_dbg_fqs_fops,
    );
}

#[no_mangle]
pub unsafe extern "C" fn dpaa2_dpseci_debugfs_exit(priv_: *mut dpaa2_caam_priv) {
    debugfs_remove_recursive((*priv_).dfs_root);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
