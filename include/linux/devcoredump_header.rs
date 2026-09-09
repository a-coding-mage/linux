/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2015 Intel Deutschland GmbH */

// Translated from devcoredump.h. The Linux header dependencies are supplied externally.

pub const DEVCD_TIMEOUT: c_ulong = HZ.wrapping_mul(60).wrapping_mul(5);

/// Free all memory belonging to the given scatterlist table.
pub unsafe fn _devcd_free_sgtable(mut table: *mut scatterlist) {
    let mut i: c_int = 0;
    let mut page: *mut page;
    let mut iter: *mut scatterlist = table;
    let mut delete_iter: *mut scatterlist;

    // The C for_each_sg/sg_nents traversal is retained through the corresponding
    // Linux scatterlist operations supplied by the including environment.
    for_each_sg!(table, iter, sg_nents(table), i, {
        page = sg_page(iter);
        if !page.is_null() {
            __free_page(page);
        }
    });

    iter = table;
    delete_iter = table;
    while !sg_is_last(iter) {
        iter = iter.add(1);
        if sg_is_chain(iter) {
            iter = sg_chain_ptr(iter);
            kfree(delete_iter as *mut c_void);
            delete_iter = iter;
        }
    }
    kfree(delete_iter as *mut c_void);
}

#[cfg(CONFIG_DEV_COREDUMP)]
extern "C" {
    pub fn dev_coredumpv(dev: *mut device, data: *mut c_void, datalen: size_t, gfp: gfp_t);
    pub fn dev_coredumpm_timeout(
        dev: *mut device,
        owner: *mut module,
        data: *mut c_void,
        datalen: size_t,
        gfp: gfp_t,
        read: Option<unsafe extern "C" fn(*mut c_char, loff_t, size_t, *mut c_void, size_t) -> ssize_t>,
        free: Option<unsafe extern "C" fn(*mut c_void)>,
        timeout: c_ulong,
    );
    pub fn dev_coredumpsg(dev: *mut device, table: *mut scatterlist, datalen: size_t, gfp: gfp_t);
    pub fn dev_coredump_put(dev: *mut device);
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn dev_coredumpv(_dev: *mut device, data: *mut c_void, _datalen: size_t, _gfp: gfp_t) {
    vfree(data);
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn dev_coredumpm_timeout(
    _dev: *mut device,
    _owner: *mut module,
    data: *mut c_void,
    _datalen: size_t,
    _gfp: gfp_t,
    _read: Option<unsafe extern "C" fn(*mut c_char, loff_t, size_t, *mut c_void, size_t) -> ssize_t>,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
    _timeout: c_ulong,
) {
    if let Some(f) = free {
        f(data);
    }
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn dev_coredumpsg(_dev: *mut device, table: *mut scatterlist, _datalen: size_t, _gfp: gfp_t) {
    _devcd_free_sgtable(table);
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn dev_coredump_put(_dev: *mut device) {}

pub unsafe fn dev_coredumpm(
    dev: *mut device,
    owner: *mut module,
    data: *mut c_void,
    datalen: size_t,
    gfp: gfp_t,
    read: Option<unsafe extern "C" fn(*mut c_char, loff_t, size_t, *mut c_void, size_t) -> ssize_t>,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    dev_coredumpm_timeout(dev, owner, data, datalen, gfp, read, free, DEVCD_TIMEOUT);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
