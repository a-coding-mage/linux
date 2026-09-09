// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test of proc sysctl.
 */

// Dependencies supplied by the Linux kernel/KUnit environment.

const KUNIT_PROC_READ: i32 = 0;
const KUNIT_PROC_WRITE: i32 = 1;

unsafe extern "C" {
    fn proc_dointvec(
        table: *mut ctl_table,
        write: i32,
        buffer: *mut core::ffi::c_void,
        len: *mut usize,
        pos: *mut loff_t,
    ) -> i32;
    fn kunit_kzalloc(
        test: *mut kunit,
        size: usize,
        flags: u32,
    ) -> *mut core::ffi::c_void;
}

type loff_t = i64;

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ctl_table {
    pub procname: *const core::ffi::c_char,
    pub data: *mut core::ffi::c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn(
        *mut ctl_table,
        i32,
        *mut core::ffi::c_void,
        *mut usize,
        *mut loff_t,
    ) -> i32>,
    pub extra1: *const core::ffi::c_void,
    pub extra2: *const core::ffi::c_void,
}

const GFP_USER: u32 = 0;
const SYSCTL_ZERO: *const core::ffi::c_void = core::ptr::null();
const SYSCTL_ONE_HUNDRED: *const core::ffi::c_void = core::ptr::null();
const EINVAL: i32 = 22;

unsafe fn sysctl_test_api_dointvec_null_tbl_data(test: *mut kunit) {
    let mut null_data_table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _,
        data: core::ptr::null_mut(),
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO,
        extra2: SYSCTL_ONE_HUNDRED,
    };
    let buffer = kunit_kzalloc(test, core::mem::size_of::<i32>(), GFP_USER);
    let mut len: usize;
    let mut pos: loff_t = 0;

    len = 1234;
    let _ = proc_dointvec(&mut null_data_table, KUNIT_PROC_READ, buffer, &mut len, &mut pos);
    len = 1234;
    let _ = proc_dointvec(&mut null_data_table, KUNIT_PROC_WRITE, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_api_dointvec_table_maxlen_unset(test: *mut kunit) {
    let mut data: i32 = 0;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _,
        data: &mut data as *mut _ as *mut _,
        maxlen: 0,
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO,
        extra2: SYSCTL_ONE_HUNDRED,
    };
    let buffer = kunit_kzalloc(test, core::mem::size_of::<i32>(), GFP_USER);
    let mut len: usize;
    let mut pos: loff_t = 0;
    len = 1234;
    let _ = proc_dointvec(&mut table, KUNIT_PROC_READ, buffer, &mut len, &mut pos);
    len = 1234;
    let _ = proc_dointvec(&mut table, KUNIT_PROC_WRITE, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_api_dointvec_table_len_is_zero(test: *mut kunit) {
    let mut data: i32 = 0;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _,
        data: &mut data as *mut _ as *mut _,
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO,
        extra2: SYSCTL_ONE_HUNDRED,
    };
    let buffer = kunit_kzalloc(test, core::mem::size_of::<i32>(), GFP_USER);
    let mut len = 0usize;
    let mut pos: loff_t = 0;
    let _ = proc_dointvec(&mut table, KUNIT_PROC_READ, buffer, &mut len, &mut pos);
    let _ = proc_dointvec(&mut table, KUNIT_PROC_WRITE, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_api_dointvec_table_read_but_position_set(test: *mut kunit) {
    let mut data: i32 = 0;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _, data: &mut data as *mut _ as *mut _,
        maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE_HUNDRED,
    };
    let buffer = kunit_kzalloc(test, core::mem::size_of::<i32>(), GFP_USER);
    let mut len = 1234usize;
    let mut pos: loff_t = 1;
    let _ = proc_dointvec(&mut table, KUNIT_PROC_READ, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_dointvec_read_happy_single_positive(test: *mut kunit) {
    let mut data: i32 = 13;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _, data: &mut data as *mut _ as *mut _,
        maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE_HUNDRED,
    };
    let mut len = 4usize; let mut pos: loff_t = 0;
    let buffer = kunit_kzalloc(test, len, GFP_USER);
    let _ = proc_dointvec(&mut table, KUNIT_PROC_READ, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_dointvec_read_happy_single_negative(test: *mut kunit) {
    let mut data: i32 = -16;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _, data: &mut data as *mut _ as *mut _,
        maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE_HUNDRED,
    };
    let mut len = 5usize; let mut pos: loff_t = 0;
    let buffer = kunit_kzalloc(test, len, GFP_USER);
    let _ = proc_dointvec(&mut table, KUNIT_PROC_READ, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_dointvec_write_happy_single_positive(test: *mut kunit) {
    let mut data: i32 = 0;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _, data: &mut data as *mut _ as *mut _,
        maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE_HUNDRED,
    };
    let input = b"9"; let mut len = input.len(); let mut pos: loff_t = 0;
    let buffer = kunit_kzalloc(test, len, GFP_USER);
    core::ptr::copy_nonoverlapping(input.as_ptr(), buffer as *mut u8, len);
    let _ = proc_dointvec(&mut table, KUNIT_PROC_WRITE, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_dointvec_write_happy_single_negative(test: *mut kunit) {
    let mut data: i32 = 0;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _, data: &mut data as *mut _ as *mut _,
        maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE_HUNDRED,
    };
    let input = b"-9"; let mut len = input.len(); let mut pos: loff_t = 0;
    let buffer = kunit_kzalloc(test, len, GFP_USER);
    core::ptr::copy_nonoverlapping(input.as_ptr(), buffer as *mut u8, len);
    let _ = proc_dointvec(&mut table, KUNIT_PROC_WRITE, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_api_dointvec_write_single_less_int_min(test: *mut kunit) {
    let mut data: i32 = 0;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _, data: &mut data as *mut _ as *mut _,
        maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE_HUNDRED,
    };
    let max_len = 32usize; let mut len = max_len; let mut pos: loff_t = 0;
    let buffer = kunit_kzalloc(test, max_len, GFP_USER);
    let abs_of_less_than_min = (i32::MAX as u64 - (i32::MAX as i64 + i32::MIN as i64) as u64 + 1) as usize;
    let text = format!("-{abs_of_less_than_min}");
    core::ptr::copy_nonoverlapping(text.as_ptr(), buffer as *mut u8, text.len());
    let _ = proc_dointvec(&mut table, KUNIT_PROC_WRITE, buffer, &mut len, &mut pos);
}

unsafe fn sysctl_test_api_dointvec_write_single_greater_int_max(test: *mut kunit) {
    let mut data: i32 = 0;
    let mut table = ctl_table {
        procname: b"foo\0".as_ptr() as *const _, data: &mut data as *mut _ as *mut _,
        maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec),
        extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE_HUNDRED,
    };
    let max_len = 32usize; let mut len = max_len; let mut pos: loff_t = 0;
    let buffer = kunit_kzalloc(test, max_len, GFP_USER);
    let greater_than_max = i32::MAX as u64 + 1;
    let text = format!("{greater_than_max}");
    core::ptr::copy_nonoverlapping(text.as_ptr(), buffer as *mut u8, text.len());
    let _ = proc_dointvec(&mut table, KUNIT_PROC_WRITE, buffer, &mut len, &mut pos);
}

// Original KUnit registration: sysctl_test_cases and sysctl_test_suite contain
// the ten functions above, and kunit_test_suites registers the suite.
// MODULE_DESCRIPTION("KUnit test of proc sysctl");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
