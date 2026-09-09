// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Linux/debugfs and QAT header dependencies are supplied by the surrounding
// translation unit.

const HB_OK: i32 = 0;
const HB_ERROR: i32 = -1;
const HB_STATUS_MAX_STRLEN: usize = 4;
const HB_STATS_MAX_STRLEN: usize = 16;

unsafe fn adf_hb_stats_read(
    file: *mut file,
    user_buffer: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    let mut buf = [0 as core::ffi::c_char; HB_STATS_MAX_STRLEN];
    let value: *mut u32;
    let len: i32;

    if *ppos > 0 {
        return 0;
    }

    value = (*file).private_data as *mut u32;
    len = scnprintf(buf.as_mut_ptr(), buf.len(), c"%u\n".as_ptr(), *value);

    simple_read_from_buffer(user_buffer, count, ppos, buf.as_ptr() as *const _, (len + 1) as usize)
}

static adf_hb_stats_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(simple_open),
    read: Some(adf_hb_stats_read),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn adf_hb_status_read(
    file: *mut file,
    user_buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    let mut hb_status: adf_device_heartbeat_status;
    let mut ret_str = [0 as core::ffi::c_char; HB_STATUS_MAX_STRLEN];
    let accel_dev: *mut adf_accel_dev;
    let mut ret_code: i32;
    let len: usize;

    if *ppos > 0 {
        return 0;
    }

    accel_dev = (*file).private_data as *mut adf_accel_dev;
    ret_code = HB_OK;
    adf_heartbeat_status(accel_dev, &mut hb_status);

    if hb_status != HB_DEV_ALIVE {
        ret_code = HB_ERROR;
    }

    len = scnprintf(ret_str.as_mut_ptr(), ret_str.len(), c"%d\n".as_ptr(), ret_code) as usize;
    simple_read_from_buffer(user_buf, count, ppos, ret_str.as_ptr() as *const _, len + 1)
}

static adf_hb_status_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(simple_open),
    read: Some(adf_hb_status_read),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn adf_hb_cfg_read(
    file: *mut file,
    user_buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    let mut timer_str = [0 as core::ffi::c_char; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let accel_dev: *mut adf_accel_dev;
    let timer_ms: u32;
    let len: i32;

    if *ppos > 0 {
        return 0;
    }

    accel_dev = (*file).private_data as *mut adf_accel_dev;
    timer_ms = (*(*accel_dev).heartbeat).hb_timer;
    len = scnprintf(timer_str.as_mut_ptr(), timer_str.len(), c"%u\n".as_ptr(), timer_ms);
    simple_read_from_buffer(user_buf, count, ppos, timer_str.as_ptr() as *const _, (len + 1) as usize)
}

unsafe fn adf_hb_cfg_write(
    file: *mut file,
    user_buf: *const core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    let mut input_str = [0 as core::ffi::c_char; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let accel_dev: *mut adf_accel_dev;
    let mut ret: i32;
    let written_chars: i32;
    let mut timer_ms: u32;
    let mut ticks: u32 = 0;

    accel_dev = (*file).private_data as *mut adf_accel_dev;
    timer_ms = ADF_CFG_HB_TIMER_DEFAULT_MS;

    if count > input_str.len() - 1 {
        return -EINVAL;
    }

    written_chars = simple_write_to_buffer(input_str.as_mut_ptr() as *mut _, input_str.len() - 1, ppos, user_buf as *const _, count) as i32;
    if written_chars > 0 {
        ret = kstrtouint(input_str.as_ptr(), 10, &mut timer_ms);
        if ret != 0 {
            dev_err(&GET_DEV(accel_dev), c"heartbeat_cfg: Invalid value\n".as_ptr());
            return ret as isize;
        }
        if timer_ms < ADF_CFG_HB_TIMER_MIN_MS {
            dev_err(&GET_DEV(accel_dev), c"heartbeat_cfg: Invalid value\n".as_ptr());
            return -EINVAL;
        }
        /* On 4xxx devices adf_timer is responsible for HB updates and its period is fixed to 200ms. */
        if !(*accel_dev).timer.is_null() {
            timer_ms = ADF_CFG_HB_TIMER_MIN_MS;
        }
        ret = adf_heartbeat_save_cfg_param(accel_dev, timer_ms);
        if ret != 0 { return ret as isize; }
        ret = adf_heartbeat_ms_to_ticks(accel_dev, timer_ms, &mut ticks);
        if ret != 0 { return ret as isize; }
        ret = adf_send_admin_hb_timer(accel_dev, ticks);
        if ret != 0 { return ret as isize; }
        (*(*accel_dev).heartbeat).hb_timer = timer_ms;
    }
    written_chars as isize
}

static adf_hb_cfg_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(simple_open),
    read: Some(adf_hb_cfg_read),
    write: Some(adf_hb_cfg_write),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn adf_hb_error_inject_write(file: *mut file, user_buf: *const core::ffi::c_char, count: usize, ppos: *mut loff_t) -> isize {
    let accel_dev = (*file).private_data as *mut adf_accel_dev;
    let mut buf = [0 as core::ffi::c_char; 3];
    if *ppos != 0 || count != 2 { return -EINVAL; }
    if copy_from_user(buf.as_mut_ptr() as *mut _, user_buf as *const _, count) != 0 { return -EFAULT; }
    buf[count] = 0;
    if buf[0] != b'1' as i8 { return -EINVAL; }
    let ret = adf_heartbeat_inject_error(accel_dev);
    if ret != 0 {
        dev_err(&GET_DEV(accel_dev), c"Heartbeat error injection failed with status %d\n".as_ptr(), ret);
        return ret as isize;
    }
    dev_info(&GET_DEV(accel_dev), c"Heartbeat error injection enabled\n".as_ptr());
    count as isize
}

static adf_hb_error_inject_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(simple_open),
    write: Some(adf_hb_error_inject_write),
    ..unsafe { core::mem::zeroed() }
};

pub unsafe fn adf_heartbeat_dbgfs_add(accel_dev: *mut adf_accel_dev) {
    let hb = (*accel_dev).heartbeat;
    if hb.is_null() { return; }
    (*hb).dbgfs.base_dir = debugfs_create_dir(c"heartbeat".as_ptr(), (*accel_dev).debugfs_dir);
    (*hb).dbgfs.status = debugfs_create_file(c"status".as_ptr(), 0o400, (*hb).dbgfs.base_dir, accel_dev as *mut _, &adf_hb_status_fops);
    (*hb).dbgfs.sent = debugfs_create_file(c"queries_sent".as_ptr(), 0o400, (*hb).dbgfs.base_dir, &mut (*hb).hb_sent_counter as *mut _ as *mut _, &adf_hb_stats_fops);
    (*hb).dbgfs.failed = debugfs_create_file(c"queries_failed".as_ptr(), 0o400, (*hb).dbgfs.base_dir, &mut (*hb).hb_failed_counter as *mut _ as *mut _, &adf_hb_stats_fops);
    (*hb).dbgfs.cfg = debugfs_create_file(c"config".as_ptr(), 0o600, (*hb).dbgfs.base_dir, accel_dev as *mut _, &adf_hb_cfg_fops);
    // CONFIG_CRYPTO_DEV_QAT_ERROR_INJECTION controls this block in the kernel build.
    if IS_ENABLED(CONFIG_CRYPTO_DEV_QAT_ERROR_INJECTION) {
        let inject_error = debugfs_create_file(c"inject_error".as_ptr(), 0o200, (*hb).dbgfs.base_dir, accel_dev as *mut _, &adf_hb_error_inject_fops);
        (*hb).dbgfs.inject_error = inject_error;
    }
}

pub unsafe fn adf_heartbeat_dbgfs_rm(accel_dev: *mut adf_accel_dev) {
    let hb = (*accel_dev).heartbeat;
    if hb.is_null() { return; }
    debugfs_remove((*hb).dbgfs.status); (*hb).dbgfs.status = core::ptr::null_mut();
    debugfs_remove((*hb).dbgfs.sent); (*hb).dbgfs.sent = core::ptr::null_mut();
    debugfs_remove((*hb).dbgfs.failed); (*hb).dbgfs.failed = core::ptr::null_mut();
    debugfs_remove((*hb).dbgfs.cfg); (*hb).dbgfs.cfg = core::ptr::null_mut();
    // CONFIG_CRYPTO_DEV_QAT_ERROR_INJECTION controls this block in the kernel build.
    debugfs_remove((*hb).dbgfs.inject_error); (*hb).dbgfs.inject_error = core::ptr::null_mut();
    debugfs_remove((*hb).dbgfs.base_dir); (*hb).dbgfs.base_dir = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
