// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// C dependencies supplied by the surrounding kernel/QAT translation unit.

const ADF_FW_COUNTERS_MAX_PADDING: usize = 16;

#[repr(usize)]
enum AdfFwCountersTypes {
    ADF_FW_REQUESTS,
    ADF_FW_RESPONSES,
    ADF_FW_COUNTERS_COUNT,
}

static ADF_FW_COUNTER_NAMES: [&'static [u8]; AdfFwCountersTypes::ADF_FW_COUNTERS_COUNT as usize] = [
    b"Requests\0",
    b"Responses\0",
];

const _: () = assert!(ADF_FW_COUNTER_NAMES.len() == AdfFwCountersTypes::ADF_FW_COUNTERS_COUNT as usize);

#[repr(C)]
struct AdfAeCounters {
    ae: u16,
    values: [u64; AdfFwCountersTypes::ADF_FW_COUNTERS_COUNT as usize],
}

#[repr(C)]
struct AdfFwCounters {
    ae_count: u16,
    ae_counters: [AdfAeCounters; 0],
}

extern "C" {
    static mut SEQ_START_TOKEN: *mut core::ffi::c_void;
    fn adf_get_ae_fw_counters(
        accel_dev: *mut adf_accel_dev,
        ae: usize,
        req_count: *mut u64,
        resp_count: *mut u64,
    ) -> i32;
    fn adf_dev_started(accel_dev: *mut adf_accel_dev) -> bool;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn kmalloc_flex(size: usize, ae_count: usize) -> *mut AdfFwCounters;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn seq_puts(sfile: *mut seq_file, text: *const u8);
    fn seq_printf(sfile: *mut seq_file, fmt: *const u8, ...);
    fn seq_putc(sfile: *mut seq_file, c: i32);
    fn seq_open(file: *mut file, ops: *const seq_operations) -> i32;
    fn seq_release(inode: *mut inode, file: *mut file) -> i32;
    fn seq_read(file: *mut file, data: *mut core::ffi::c_void, size: usize, offset: *mut i64) -> isize;
    fn seq_lseek(file: *mut file, offset: i64, whence: i32) -> i64;
    fn debugfs_create_file(
        name: *const u8,
        mode: u32,
        parent: *mut dentry,
        data: *mut core::ffi::c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
}

#[repr(C)]
struct adf_accel_dev {
    fw_cntr_dbgfile: *mut dentry,
    debugfs_dir: *mut dentry,
}
#[repr(C)] struct adf_hw_device_data { ae_mask: usize, admin_ae_mask: usize }
#[repr(C)] struct seq_file { private: *mut core::ffi::c_void }
#[repr(C)] struct inode { i_private: *mut core::ffi::c_void }
#[repr(C)] struct file { private_data: *mut core::ffi::c_void }
#[repr(C)] struct dentry;
#[repr(C)] struct file_operations;
#[repr(C)] struct seq_operations;

// GET_HW_DATA, GET_MAX_ACCELENGINES, GET_DEV, THIS_MODULE, ERR_PTR, IS_ERR,
// PTR_ERR, hweight_long, and for_each_set_bit are supplied by the kernel layer.

unsafe fn adf_fw_counters_parse_ae_values(
    ae_counters: *mut AdfAeCounters,
    ae: u32,
    req_count: u64,
    resp_count: u64,
) {
    (*ae_counters).ae = ae as u16;
    (*ae_counters).values[AdfFwCountersTypes::ADF_FW_REQUESTS as usize] = req_count;
    (*ae_counters).values[AdfFwCountersTypes::ADF_FW_RESPONSES as usize] = resp_count;
}

unsafe fn adf_fw_counters_load_from_device(
    accel_dev: *mut adf_accel_dev,
    fw_counters: *mut AdfFwCounters,
) -> i32 {
    let hw_data = GET_HW_DATA(accel_dev);
    let ae_mask = (*hw_data).ae_mask & !(*hw_data).admin_ae_mask;
    if ae_mask.count_ones() as u16 > (*fw_counters).ae_count { return -22; }
    let mut i = 0usize;
    for ae in 0..GET_MAX_ACCELENGINES(accel_dev) {
        if (ae_mask & (1usize << ae)) == 0 { continue; }
        let mut req_count = 0u64;
        let mut resp_count = 0u64;
        let ret = adf_get_ae_fw_counters(accel_dev, ae, &mut req_count, &mut resp_count);
        if ret != 0 { return ret; }
        let counters = (*fw_counters).ae_counters.as_mut_ptr().add(i);
        adf_fw_counters_parse_ae_values(counters, ae as u32, req_count, resp_count);
        i += 1;
    }
    0
}

unsafe fn adf_fw_counters_allocate(ae_count: usize) -> *mut AdfFwCounters {
    if ae_count == 0 { return ERR_PTR(-22); }
    let fw_counters = kmalloc_flex(core::mem::size_of::<AdfFwCounters>() + ae_count * core::mem::size_of::<AdfAeCounters>(), ae_count);
    if fw_counters.is_null() { return ERR_PTR(-12); }
    (*fw_counters).ae_count = ae_count as u16;
    fw_counters
}

/** adf_fw_counters_get() - Return FW counters for the provided device. */
unsafe fn adf_fw_counters_get(accel_dev: *mut adf_accel_dev) -> *mut AdfFwCounters {
    let hw_data = GET_HW_DATA(accel_dev);
    if !adf_dev_started(accel_dev) { return ERR_PTR(-14); }
    let ae_count = ((*hw_data).ae_mask & !(*hw_data).admin_ae_mask).count_ones() as usize;
    let fw_counters = adf_fw_counters_allocate(ae_count);
    if IS_ERR(fw_counters) { return fw_counters; }
    let ret = adf_fw_counters_load_from_device(accel_dev, fw_counters);
    if ret != 0 { kfree(fw_counters.cast()); return ERR_PTR(ret); }
    fw_counters
}

// The remaining seq_file/debugfs callbacks are represented with the same
// externally visible signatures; kernel-provided sequence operations dispatch them.
pub unsafe fn adf_fw_counters_dbgfs_add(accel_dev: *mut adf_accel_dev) {
    (*accel_dev).fw_cntr_dbgfile = debugfs_create_file(b"fw_counters\0".as_ptr(), 0o400, (*accel_dev).debugfs_dir, accel_dev.cast(), &qat_fw_counters_fops);
}

pub unsafe fn adf_fw_counters_dbgfs_rm(accel_dev: *mut adf_accel_dev) {
    debugfs_remove((*accel_dev).fw_cntr_dbgfile);
    (*accel_dev).fw_cntr_dbgfile = core::ptr::null_mut();
}

unsafe fn qat_fw_counters_seq_start(sfile: *mut seq_file, pos: *mut i64) -> *mut core::ffi::c_void {
    let fw_counters = (*sfile).private.cast::<AdfFwCounters>();
    if *pos == 0 { return SEQ_START_TOKEN; }
    if *pos > (*fw_counters).ae_count as i64 { return core::ptr::null_mut(); }
    (*fw_counters).ae_counters.as_mut_ptr().add((*pos - 1) as usize).cast()
}
unsafe fn qat_fw_counters_seq_next(sfile: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void {
    let fw_counters = (*sfile).private.cast::<AdfFwCounters>(); *pos += 1;
    if *pos > (*fw_counters).ae_count as i64 { return core::ptr::null_mut(); }
    (*fw_counters).ae_counters.as_mut_ptr().add((*pos - 1) as usize).cast()
}
unsafe fn qat_fw_counters_seq_stop(_sfile: *mut seq_file, _v: *mut core::ffi::c_void) {}
unsafe fn qat_fw_counters_seq_show(sfile: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == SEQ_START_TOKEN {
        seq_puts(sfile, b"AE \0".as_ptr());
        for name in ADF_FW_COUNTER_NAMES.iter() { seq_printf(sfile, b" %*s\0".as_ptr(), ADF_FW_COUNTERS_MAX_PADDING, name.as_ptr()); }
    } else {
        let ae_counters = v.cast::<AdfAeCounters>();
        seq_printf(sfile, b"%2d:\0".as_ptr(), (*ae_counters).ae);
        for value in (*ae_counters).values.iter() { seq_printf(sfile, b" %*llu\0".as_ptr(), ADF_FW_COUNTERS_MAX_PADDING, *value); }
    }
    seq_putc(sfile, b'\n' as i32); 0
}
static qat_fw_counters_sops: seq_operations = seq_operations { };
unsafe fn qat_fw_counters_file_open(inode: *mut inode, file: *mut file) -> i32 {
    let fw_counters = adf_fw_counters_get((*inode).i_private.cast());
    if IS_ERR(fw_counters) { return PTR_ERR(fw_counters); }
    let ret = seq_open(file, &qat_fw_counters_sops);
    if ret != 0 { kfree(fw_counters.cast()); return ret; }
    (*(*file).private_data.cast::<seq_file>()).private = fw_counters.cast(); ret
}
unsafe fn qat_fw_counters_file_release(inode: *mut inode, file: *mut file) -> i32 {
    let seq = (*file).private_data.cast::<seq_file>(); kfree((*seq).private);
    (*seq).private = core::ptr::null_mut(); seq_release(inode, file)
}
static qat_fw_counters_fops: file_operations = file_operations { };

extern "C" {
    fn GET_HW_DATA(accel_dev: *mut adf_accel_dev) -> *mut adf_hw_device_data;
    fn GET_MAX_ACCELENGINES(accel_dev: *mut adf_accel_dev) -> usize;
    fn ERR_PTR(error: i32) -> *mut AdfFwCounters;
    fn IS_ERR(ptr: *mut AdfFwCounters) -> bool;
    fn PTR_ERR(ptr: *mut AdfFwCounters) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
