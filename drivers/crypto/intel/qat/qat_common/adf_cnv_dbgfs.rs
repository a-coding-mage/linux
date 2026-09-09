// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Dependencies supplied by the surrounding kernel/QAT implementation are
// intentionally referenced but not defined here.

const CNV_DEBUGFS_FILENAME: &str = "cnv_errors";
const CNV_MIN_PADDING: usize = 16;

const CNV_ERR_INFO_MASK: u16 = 0x0fff;
const CNV_ERR_TYPE_MASK: u16 = 0xf000;
const CNV_SLICE_ERR_SIGN_BIT_INDEX: u32 = 7;
const CNV_DELTA_ERR_SIGN_BIT_INDEX: u32 = 11;

#[repr(u8)]
enum cnv_error_type {
    CNV_ERR_TYPE_NONE,
    CNV_ERR_TYPE_CHECKSUM,
    CNV_ERR_TYPE_DECOMP_PRODUCED_LENGTH,
    CNV_ERR_TYPE_DECOMPRESSION,
    CNV_ERR_TYPE_TRANSLATION,
    CNV_ERR_TYPE_DECOMP_CONSUMED_LENGTH,
    CNV_ERR_TYPE_UNKNOWN,
    CNV_ERR_TYPES_COUNT,
}

const CNV_ERR_TYPES_COUNT_USIZE: usize = 7;
const CNV_FIELDS_COUNT: usize = 2;

#[repr(u8)]
enum cnv_fields {
    CNV_ERR_COUNT,
    CNV_LATEST_ERR,
    CNV_FIELDS_COUNT_ENUM,
}

static CNV_FIELD_NAMES: [&str; CNV_FIELDS_COUNT] = ["Total Errors", "Last Error"];
static CNV_ERROR_NAMES: [&str; CNV_ERR_TYPES_COUNT_USIZE] = [
    "No Error", "Checksum Error", "Length Error-P", "Decomp Error",
    "Xlat Error", "Length Error-C", "Unknown Error",
];

#[repr(C)]
struct ae_cnv_errors {
    ae: u16,
    err_cnt: u16,
    latest_err: u16,
    is_comp_ae: bool,
}

#[repr(C)]
struct cnv_err_stats {
    ae_count: u16,
    ae_cnv_errors: [ae_cnv_errors; 0],
}

#[inline]
fn cnv_error_type_get(latest_err: u16) -> u8 {
    core::cmp::min(((latest_err & CNV_ERR_TYPE_MASK) >> 12) as u8, 6)
}

#[inline]
fn sign_extend(value: u16, bit: u32) -> i16 {
    ((value << (15 - bit)) as i16) >> (15 - bit)
}

#[inline]
fn get_err_info(error_type: u8, latest: u16) -> i16 {
    match error_type {
        2 | 5 => sign_extend(latest, CNV_DELTA_ERR_SIGN_BIT_INDEX),
        3 | 4 => sign_extend(latest, CNV_SLICE_ERR_SIGN_BIT_INDEX),
        _ => (latest & CNV_ERR_INFO_MASK) as i16,
    }
}

unsafe fn qat_cnv_errors_seq_start(sfile: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let err_stats = (*sfile).private as *mut cnv_err_stats;
    if *pos == 0 { return SEQ_START_TOKEN; }
    if *pos > (*err_stats).ae_count as loff_t { return core::ptr::null_mut(); }
    (*err_stats).ae_cnv_errors.as_mut_ptr().add((*pos - 1) as usize) as *mut _
}

unsafe fn qat_cnv_errors_seq_next(sfile: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let err_stats = (*sfile).private as *mut cnv_err_stats;
    *pos += 1;
    if *pos > (*err_stats).ae_count as loff_t { return core::ptr::null_mut(); }
    (*err_stats).ae_cnv_errors.as_mut_ptr().add((*pos - 1) as usize) as *mut _
}

unsafe fn qat_cnv_errors_seq_stop(_sfile: *mut seq_file, _v: *mut core::ffi::c_void) {}

unsafe fn qat_cnv_errors_seq_show(sfile: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == SEQ_START_TOKEN {
        seq_puts(sfile, "AE ");
        for i in 0..CNV_FIELDS_COUNT { seq_printf(sfile, " %*s", CNV_MIN_PADDING, CNV_FIELD_NAMES[i]); }
    } else {
        let ae_errors = v as *mut ae_cnv_errors;
        if !(*ae_errors).is_comp_ae { return 0; }
        let err_type = cnv_error_type_get((*ae_errors).latest_err);
        let err_info = get_err_info(err_type, (*ae_errors).latest_err);
        seq_printf(sfile, "%d:", (*ae_errors).ae);
        seq_printf(sfile, " %*d", CNV_MIN_PADDING, (*ae_errors).err_cnt);
        seq_printf(sfile, "%*s [%d]", CNV_MIN_PADDING, CNV_ERROR_NAMES[err_type as usize], err_info);
    }
    seq_putc(sfile, '\n');
    0
}

static qat_cnv_errors_sops: seq_operations = seq_operations {
    start: Some(qat_cnv_errors_seq_start), next: Some(qat_cnv_errors_seq_next),
    stop: Some(qat_cnv_errors_seq_stop), show: Some(qat_cnv_errors_seq_show),
};

unsafe fn cnv_err_stats_alloc(accel_dev: *mut adf_accel_dev) -> *mut cnv_err_stats {
    let hw_data = GET_HW_DATA(accel_dev);
    if !adf_dev_started(accel_dev) { dev_err(&GET_DEV(accel_dev), "QAT Device not started\n"); return ERR_PTR(-EBUSY); }
    let ae_mask = (*hw_data).ae_mask & !(*hw_data).admin_ae_mask;
    let ae_count = hweight_long(ae_mask);
    if ae_count == 0 { return ERR_PTR(-EINVAL); }
    let size = core::mem::size_of::<cnv_err_stats>() + ae_count as usize * core::mem::size_of::<ae_cnv_errors>();
    let err_stats = kmalloc(size, GFP_KERNEL) as *mut cnv_err_stats;
    if err_stats.is_null() { return ERR_PTR(-ENOMEM); }
    (*err_stats).ae_count = ae_count as u16;
    let entries = (*err_stats).ae_cnv_errors.as_mut_ptr();
    let mut i = 0usize;
    for ae in for_each_set_bit(ae_mask, GET_MAX_ACCELENGINES(accel_dev)) {
        let mut err_cnt = 0u16; let mut latest_err = 0u16;
        if adf_get_cnv_stats(accel_dev, ae, &mut err_cnt, &mut latest_err) != 0 {
            (*entries.add(i)).is_comp_ae = false;
        } else {
            (*entries.add(i)) = ae_cnv_errors { ae: ae as u16, err_cnt, latest_err, is_comp_ae: true };
        }
        i += 1;
    }
    err_stats
}

unsafe fn qat_cnv_errors_file_open(inode: *mut inode, file: *mut file) -> i32 {
    let stats = cnv_err_stats_alloc((*inode).i_private);
    if IS_ERR(stats) { return PTR_ERR(stats); }
    let ret = seq_open(file, &qat_cnv_errors_sops);
    if ret != 0 { kfree(stats as *mut core::ffi::c_void); return ret; }
    (*((*file).private_data as *mut seq_file)).private = stats as *mut core::ffi::c_void; ret
}

unsafe fn qat_cnv_errors_file_release(inode: *mut inode, file: *mut file) -> i32 {
    let seq = (*file).private_data as *mut seq_file;
    kfree((*seq).private); (*seq).private = core::ptr::null_mut(); seq_release(inode, file)
}

static qat_cnv_fops: file_operations = file_operations { owner: THIS_MODULE, open: Some(qat_cnv_errors_file_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(qat_cnv_errors_file_release) };

unsafe fn no_comp_file_read(f: *mut file, buf: *mut core::ffi::c_void, count: usize, pos: *mut loff_t) -> isize {
    let msg = "No engine configured for comp\n"; simple_read_from_buffer(buf, count, pos, msg.as_ptr() as *const _, msg.len())
}

static qat_cnv_no_comp_fops: file_operations = file_operations { owner: THIS_MODULE, read: Some(no_comp_file_read) };

pub unsafe fn adf_cnv_dbgfs_add(accel_dev: *mut adf_accel_dev) {
    let (fops, data) = if adf_hw_dev_has_compression(accel_dev) { (&qat_cnv_fops, accel_dev as *mut _) } else { (&qat_cnv_no_comp_fops, core::ptr::null_mut()) };
    (*accel_dev).cnv_dbgfile = debugfs_create_file(CNV_DEBUGFS_FILENAME, 0o400, (*accel_dev).debugfs_dir, data, fops);
}

pub unsafe fn adf_cnv_dbgfs_rm(accel_dev: *mut adf_accel_dev) {
    debugfs_remove((*accel_dev).cnv_dbgfile); (*accel_dev).cnv_dbgfile = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
