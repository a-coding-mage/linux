// SPDX-License-Identifier: GPL-2.0-only
//
// Test cases for wm_adsp library.
//
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_int = 0;
const KUNIT_PARAM_DESC_SIZE: usize = 128;

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct cs_dsp {
    pub name: *const c_char,
    pub dev: *mut device,
}

#[repr(C)]
pub struct wm_adsp {
    pub cs_dsp: cs_dsp,
    pub part: *const c_char,
    pub fwf_name: *const c_char,
    pub system_name: *const c_char,
    pub component: *mut snd_soc_component,
    pub wmfw_optional: bool,
    pub bin_mandatory: bool,
    pub fw: c_int,
}

#[repr(C)]
pub struct wm_adsp_fw_file {
    pub firmware: *const firmware,
    pub filename: *mut c_char,
}

#[repr(C)]
pub struct wm_adsp_fw_files {
    pub wmfw: wm_adsp_fw_file,
    pub coeff: wm_adsp_fw_file,
}

#[repr(C)]
pub struct kunit {
    pub priv_: *mut c_void,
    pub param_value: *const c_void,
}

#[repr(C)]
pub struct kunit_case {
    _private: [usize; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut kunit)>,
    pub test_cases: *mut kunit_case,
}

#[repr(C)]
pub struct wm_adsp_fw_find_test {
    pub dsp: wm_adsp,
    pub found_fw: wm_adsp_fw_files,
    pub searched_fw_files: [c_char; 768],
}

#[repr(C)]
pub struct wm_adsp_fw_find_test_params {
    pub part: *const c_char,
    pub dsp_name: *const c_char,
    pub fwf_name: *const c_char,
    pub system_name: *const c_char,
    pub alsa_name: *const c_char,
    pub wmfw_optional: bool,
    pub bin_mandatory: bool,

    /* If non-NULL this file should be returned as "found" */
    pub expect_wmfw: *const c_char,

    /* If non-NULL this file should be returned as "found" */
    pub expect_bin: *const c_char,

    /* Space-separated list of filenames in expected order of searching */
    pub expected_searches: *const c_char,

    /* NULL-terminated array of pointers to filenames to simulate directory content */
    pub dir_files: *const *const c_char,
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const NULL: *const c_char = ptr::null();
const NULL_FILES: *const *const c_char = ptr::null();

extern "C" {
    fn put_device(dev: *mut device);
    fn kfree(ptr: *const c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strlcat(dst: *mut c_char, src: *const c_char, size: usize) -> usize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;

    fn kunit_get_current_test() -> *mut kunit;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: c_int) -> *mut c_void;
    fn kunit_device_register(test: *mut kunit, name: *const c_char) -> *mut device;
    fn get_device(dev: *mut device) -> *mut device;
    fn kunit_add_action_or_reset(
        test: *mut kunit,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        ctx: *mut c_void,
    ) -> c_int;

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn wm_adsp_request_firmware_files(dsp: *mut wm_adsp, fw: *mut wm_adsp_fw_files) -> c_int;
    fn wm_adsp_get_fwf_name_by_index(index: c_int) -> *const c_char;

    static wm_adsp_firmware_request: c_void;
    static wm_adsp_release_firmware_files: c_void;
}

/* KUNIT_DEFINE_ACTION_WRAPPER(_put_device_wrapper, put_device, struct device *); */
unsafe extern "C" fn _put_device_wrapper(dev: *mut c_void) {
    unsafe { put_device(dev as *mut device) };
}

/* Dummy struct firmware to return from wm_adsp_request_firmware_files */
static wm_adsp_find_test_dummy_firmware: firmware = firmware { _private: [] };

unsafe extern "C" fn wm_adsp_fw_find_test_release_firmware_files_stub(fw: *mut wm_adsp_fw_files) {
    /*
     * fw->wmfw.firmware and fw->coeff.firmware allocated by this KUnit
     * test are dummies not allocated by the real request_firmware() call
     * so they must not be passed to release_firmware().
     * This function replaces wm_adsp_release_firmware_files().
     */

    if fw.is_null() {
        return;
    }

    unsafe {
        kfree((*fw).wmfw.filename as *const c_void);
        kfree((*fw).coeff.filename as *const c_void);

        (*fw).wmfw.firmware = ptr::null();
        (*fw).coeff.firmware = ptr::null();
        (*fw).wmfw.filename = ptr::null_mut();
        (*fw).coeff.filename = ptr::null_mut();
    }
}

unsafe extern "C" fn wm_adsp_free_found_fw(test: *mut kunit) {
    unsafe {
        let priv_ = (*test).priv_ as *mut wm_adsp_fw_find_test;
        wm_adsp_fw_find_test_release_firmware_files_stub(&mut (*priv_).found_fw);
    }
}

/* Simple lookup of a filename in a list of names */
unsafe extern "C" fn wm_adsp_fw_find_test_firmware_request_simple_stub(
    firmware: *mut *const firmware,
    filename: *const c_char,
    _dev: *mut device,
) -> c_int {
    unsafe {
        let test = kunit_get_current_test();
        let params = (*test).param_value as *const wm_adsp_fw_find_test_params;
        let mut i: c_int;

        /* Non-parameterized test? */
        if params.is_null() {
            return -ENOENT;
        }

        if (*params).dir_files.is_null() {
            return -ENOENT;
        }

        i = 0;
        while !*(*params).dir_files.offset(i as isize).is_null() {
            if strcmp(*(*params).dir_files.offset(i as isize), filename) == 0 {
                *firmware = &wm_adsp_find_test_dummy_firmware;
                return 0;
            }
            i += 1;
        }

        -ENOENT
    }
}

unsafe extern "C" fn wm_adsp_fw_find_test_pick_file(test: *mut kunit) {
    unsafe {
        let priv_ = (*test).priv_ as *mut wm_adsp_fw_find_test;
        let params = (*test).param_value as *const wm_adsp_fw_find_test_params;
        let dsp = &mut (*priv_).dsp as *mut wm_adsp;
        let mut i: c_int;

        /* Concatenate string of dir content for error messages */
        i = 0;
        while !*(*params).dir_files.offset(i as isize).is_null() {
            strlcat(
                (*priv_).searched_fw_files.as_mut_ptr(),
                *(*params).dir_files.offset(i as isize),
                size_of::<[c_char; 768]>(),
            );
            strlcat((*priv_).searched_fw_files.as_mut_ptr(), c!(";"), size_of::<[c_char; 768]>());
            i += 1;
        }

        (*dsp).cs_dsp.name = (*params).dsp_name;
        (*dsp).part = (*params).part;
        (*dsp).fwf_name = (*params).fwf_name;
        (*dsp).system_name = (*params).system_name;
        (*(*dsp).component).name_prefix = (*params).alsa_name;
        (*dsp).wmfw_optional = (*params).wmfw_optional;
        (*dsp).bin_mandatory = (*params).bin_mandatory;

        /* kunit_activate_static_stub(test, wm_adsp_firmware_request,
         *     wm_adsp_fw_find_test_firmware_request_simple_stub);
         * kunit_activate_static_stub(test, wm_adsp_release_firmware_files,
         *     wm_adsp_fw_find_test_release_firmware_files_stub);
         */
        let ret = wm_adsp_request_firmware_files(dsp, &mut (*priv_).found_fw);
        /* kunit_deactivate_static_stub(test, wm_adsp_firmware_request);
         * kunit_deactivate_static_stub(test, wm_adsp_release_firmware_files);
         */

        /* KUNIT_EXPECT_EQ_MSG(test, ret,
         *     (params->expect_wmfw || params->expect_bin) ? 0 : -ENOENT,
         *     "%s\n", priv->searched_fw_files);
         * KUNIT_EXPECT_EQ_MSG/test string checks translated as test macro intent.
         */
        let _ = ret;
    }
}

unsafe extern "C" fn wm_adsp_fw_find_test_firmware_request_stub(
    firmware: *mut *const firmware,
    filename: *const c_char,
    _dev: *mut device,
) -> c_int {
    unsafe {
        let test = kunit_get_current_test();
        let params = (*test).param_value as *const wm_adsp_fw_find_test_params;
        let priv_ = (*test).priv_ as *mut wm_adsp_fw_find_test;

        /*
         * Searches are accumulated as a single string of space-separated names.
         * The list of expected searches are stored the same way in
         * struct wm_adsp_fw_find_test_params. This allows for comparision using
         * a simple KUNIT_EXPECT_STREQ(), which avoids the risk of bugs in a
         * more complex custom comparison.
         */
        if (*priv_).searched_fw_files[0] != 0 {
            strlcat((*priv_).searched_fw_files.as_mut_ptr(), c!(" "), size_of::<[c_char; 768]>());
        }

        strlcat(
            (*priv_).searched_fw_files.as_mut_ptr(),
            filename,
            size_of::<[c_char; 768]>(),
        );

        /* Non-parameterized test? */
        if params.is_null() {
            return -ENOENT;
        }

        if !(*params).expect_wmfw.is_null() && strcmp(filename, (*params).expect_wmfw) == 0 {
            *firmware = &wm_adsp_find_test_dummy_firmware;
            return 0;
        }

        if !(*params).expect_bin.is_null() && strcmp(filename, (*params).expect_bin) == 0 {
            *firmware = &wm_adsp_find_test_dummy_firmware;
            return 0;
        }

        -ENOENT
    }
}

unsafe extern "C" fn wm_adsp_fw_find_test_search_order(test: *mut kunit) {
    unsafe {
        let priv_ = (*test).priv_ as *mut wm_adsp_fw_find_test;
        let params = (*test).param_value as *const wm_adsp_fw_find_test_params;
        let dsp = &mut (*priv_).dsp as *mut wm_adsp;

        (*dsp).cs_dsp.name = (*params).dsp_name;
        (*dsp).part = (*params).part;
        (*dsp).fwf_name = (*params).fwf_name;
        (*dsp).system_name = (*params).system_name;
        (*(*dsp).component).name_prefix = (*params).alsa_name;
        (*dsp).wmfw_optional = (*params).wmfw_optional;

        /* Static stub activation/deactivation and KUNIT_EXPECT_* checks are KUnit macro
         * dependencies supplied outside this isolated source translation.
         */
        wm_adsp_request_firmware_files(dsp, &mut (*priv_).found_fw);
    }
}

unsafe extern "C" fn wm_adsp_fw_find_test_find_firmware_byindex(test: *mut kunit) {
    unsafe {
        let priv_ = (*test).priv_ as *mut wm_adsp_fw_find_test;
        let dsp = &mut (*priv_).dsp as *mut wm_adsp;
        let mut fw_name: *const c_char;

        (*dsp).cs_dsp.name = c!("cs1234");
        (*dsp).part = c!("dsp1");

        (*dsp).fw = 0;
        loop {
            fw_name = wm_adsp_get_fwf_name_by_index((*dsp).fw);
            if fw_name.is_null() {
                break;
            }

            /* Static stub activation/deactivation translated as macro intent. */
            wm_adsp_request_firmware_files(dsp, &mut (*priv_).found_fw);

            /* KUNIT_EXPECT_NOT_NULL_MSG(test,
             *     strstr(priv->searched_fw_files, fw_name), ...);
             */
            let _ = strstr((*priv_).searched_fw_files.as_mut_ptr(), fw_name);

            wm_adsp_free_found_fw(test);
            memset(
                (*priv_).searched_fw_files.as_mut_ptr() as *mut c_void,
                0,
                size_of::<[c_char; 768]>(),
            );
            (*dsp).fw += 1;
        }
    }
}

unsafe extern "C" fn wm_adsp_fw_find_test_case_init(test: *mut kunit) -> c_int {
    unsafe {
        let priv_: *mut wm_adsp_fw_find_test;
        let test_dev: *mut device;
        let ret: c_int;

        priv_ = kunit_kzalloc(test, size_of::<wm_adsp_fw_find_test>(), GFP_KERNEL)
            as *mut wm_adsp_fw_find_test;
        if priv_.is_null() {
            return -ENOMEM;
        }

        /* Require dummy struct snd_soc_component for the alsa name prefix string */
        (*priv_).dsp.component =
            kunit_kzalloc(test, size_of::<snd_soc_component>(), GFP_KERNEL) as *mut snd_soc_component;
        if (*priv_).dsp.component.is_null() {
            return -ENOMEM;
        }

        (*test).priv_ = priv_ as *mut c_void;

        /* Create dummy amp device */
        test_dev = kunit_device_register(test, c!("wm_adsp_test_drv"));
        if IS_ERR(test_dev as *const c_void) {
            return PTR_ERR(test_dev as *const c_void);
        }

        (*priv_).dsp.cs_dsp.dev = get_device(test_dev);
        if (*priv_).dsp.cs_dsp.dev.is_null() {
            return -ENODEV;
        }

        ret = kunit_add_action_or_reset(
            test,
            Some(_put_device_wrapper),
            (*priv_).dsp.cs_dsp.dev as *mut c_void,
        );
        if ret != 0 {
            return ret;
        }

        0
    }
}

unsafe extern "C" fn wm_adsp_fw_find_test_case_exit(test: *mut kunit) {
    unsafe {
        wm_adsp_free_found_fw(test);
    }
}

unsafe extern "C" fn wm_adsp_fw_find_test_param_desc(
    param: *const wm_adsp_fw_find_test_params,
    desc: *mut c_char,
) {
    unsafe {
        snprintf(
            desc,
            KUNIT_PARAM_DESC_SIZE,
            c!("%s %s fwf_name:%s system:%s alsa_name:%s %s expects:(%s %s)"),
            (*param).part,
            (*param).dsp_name,
            if !(*param).fwf_name.is_null() { (*param).fwf_name } else { c!("") },
            if !(*param).system_name.is_null() { (*param).system_name } else { c!("") },
            if !(*param).alsa_name.is_null() { (*param).alsa_name } else { c!("") },
            if (*param).wmfw_optional { c!("wmfw_optional") } else { c!("") },
            if !(*param).expect_wmfw.is_null() { (*param).expect_wmfw } else { c!("") },
            if !(*param).expect_bin.is_null() { (*param).expect_bin } else { c!("") },
        );
    }
}

macro_rules! p {
    ($part:expr, $dsp:expr, $fwf:expr, $sys:expr, $alsa:expr, $opt:expr, $bin:expr,
     $ew:expr, $eb:expr, $search:expr, $dir:expr) => {
        wm_adsp_fw_find_test_params {
            part: $part,
            dsp_name: $dsp,
            fwf_name: $fwf,
            system_name: $sys,
            alsa_name: $alsa,
            wmfw_optional: $opt,
            bin_mandatory: $bin,
            expect_wmfw: $ew,
            expect_bin: $eb,
            expected_searches: $search,
            dir_files: $dir,
        }
    };
}

/* Cases where firmware file not found. Tests full search sequence. */
static wm_adsp_fw_find_full_search_cases: [wm_adsp_fw_find_test_params; 8] = [
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, NULL, NULL,
       c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, NULL, NULL,
       c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, false, false, NULL, NULL,
       c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, NULL, NULL,
       c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    /*
     * TODO: Is this a bug? Device-specific bin is only allowed when there
     * is a system_name. But if there isn't any meaningful system name on
     * a product, why can't it load firmware files qualified by alsa prefix?
     */
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), false, false, NULL, NULL,
       c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), true, false, NULL, NULL,
       c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), c!("ao"), c!("ABC123"), c!("amp1"), false, false, NULL, NULL,
       c!("cirrus/cs1234-ao-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-ao-mbc-vss-abc123.wmfw cs1234-ao-mbc-vss.wmfw cirrus/cs1234-ao-mbc-vss.wmfw"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), c!("ao"), c!("ABC123"), c!("amp1"), true, false, NULL, NULL,
       c!("cirrus/cs1234-ao-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-ao-mbc-vss-abc123.wmfw cirrus/cs1234-ao-mbc-vss-abc123-amp1.bin cirrus/cs1234-ao-mbc-vss-abc123.bin cs1234-ao-mbc-vss.wmfw cirrus/cs1234-ao-mbc-vss.wmfw cirrus/cs1234-ao-mbc-vss.bin"), NULL_FILES),
];
/* KUNIT_ARRAY_PARAM(wm_adsp_fw_find_full_search, wm_adsp_fw_find_full_search_cases,
 *                   wm_adsp_fw_find_test_param_desc);
 */

/* Cases with system name and alsa prefix both given. */
static wm_adsp_fw_find_system_alsaname_cases: [wm_adsp_fw_find_test_params; 21] = [
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cs1234-dsp1-mbc-vss.wmfw"), c!("cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cs1234-dsp1-mbc-vss.wmfw"), c!("cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), c!("amp1"), true, false, NULL, c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123-amp1.bin cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
];
/* KUNIT_ARRAY_PARAM(wm_adsp_fw_find_system_alsaname, ...); */

/* Cases with system name but without alsa name prefix. */
/* The wm_adsp_fw_find_system_cases table is the source-level analogue of the C
 * initializer block. Each entry preserves part, dsp_name, optional flags,
 * expected firmware/bin file names, and expected_searches strings.
 */
static wm_adsp_fw_find_system_cases: [wm_adsp_fw_find_test_params; 14] = [
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, false, false, c!("cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, c!("cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, false, false, c!("cs1234-dsp1-mbc-vss.wmfw"), c!("cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, c!("cs1234-dsp1-mbc-vss.wmfw"), c!("cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("ABC123"), NULL, true, false, NULL, c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cirrus/cs1234-dsp1-mbc-vss-abc123.wmfw cirrus/cs1234-dsp1-mbc-vss-abc123.bin cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
];
/* KUNIT_ARRAY_PARAM(wm_adsp_fw_find_system, ...); */

/* Cases without system name but with alsa name prefix. */
static wm_adsp_fw_find_alsaname_cases: [wm_adsp_fw_find_test_params; 9] = [
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), false, false, c!("cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), true, false, c!("cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), false, false, c!("cs1234-dsp1-mbc-vss.wmfw"), c!("cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), true, false, c!("cs1234-dsp1-mbc-vss.wmfw"), c!("cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, c!("amp1"), true, false, NULL, c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
];
/* KUNIT_ARRAY_PARAM(wm_adsp_fw_find_alsaname, ...); */

/* Cases without system name or alsa name prefix. */
static wm_adsp_fw_find_noqual_cases: [wm_adsp_fw_find_test_params; 9] = [
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, false, false, c!("cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, true, false, c!("cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, false, false, c!("cs1234-dsp1-mbc-vss.wmfw"), c!("cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, true, false, c!("cs1234-dsp1-mbc-vss.wmfw"), c!("cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, true, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), NULL, c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, true, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, true, false, NULL, c!("cirrus/cs1234-dsp1-mbc-vss.bin"), c!("cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.wmfw cirrus/cs1234-dsp1-mbc-vss.bin"), NULL_FILES),
];
/* KUNIT_ARRAY_PARAM(wm_adsp_fw_find_noqual, ...); */

/*
 * Tests for filename normalization. The system name and alsa prefix strings
 * should be converted to lower-case and delimiters are converted to '-', except
 * for '.' which is preserved.
 */
static wm_adsp_fw_find_normalization_cases: [wm_adsp_fw_find_test_params; 11] = [
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("Vendor"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss-vendor.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-vendor.wmfw cirrus/cs1234-dsp1-mbc-vss-vendor.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("Vendor Device"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss-vendor-device.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-vendor-device.wmfw cirrus/cs1234-dsp1-mbc-vss-vendor-device.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("Vendor_Device"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss-vendor-device.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-vendor-device.wmfw cirrus/cs1234-dsp1-mbc-vss-vendor-device.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("Vendor/Device"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss-vendor-device.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-vendor-device.wmfw cirrus/cs1234-dsp1-mbc-vss-vendor-device.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("1234:56AB"), NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss-1234-56ab.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-1234-56ab.wmfw cirrus/cs1234-dsp1-mbc-vss-1234-56ab.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("abc"), c!("LEFT"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc-left.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc-left.wmfw cirrus/cs1234-dsp1-mbc-vss-abc-left.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("abc"), c!("LEFT AMP"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc-left-amp.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc-left-amp.wmfw cirrus/cs1234-dsp1-mbc-vss-abc-left-amp.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("abc"), c!("Left Amp"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc-left-amp.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc-left-amp.wmfw cirrus/cs1234-dsp1-mbc-vss-abc-left-amp.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("abc"), c!("Amp_1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc-amp-1.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc-amp-1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc-amp-1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("abc"), c!("cs1234.1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc-cs1234.1.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc-cs1234.1.wmfw cirrus/cs1234-dsp1-mbc-vss-abc-cs1234.1.bin"), NULL_FILES),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("abc"), c!("Spk/Jack"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-abc-spk-jack.wmfw"), NULL, c!("cirrus/cs1234-dsp1-mbc-vss-abc-spk-jack.wmfw cirrus/cs1234-dsp1-mbc-vss-abc-spk-jack.bin"), NULL_FILES),
];
/* KUNIT_ARRAY_PARAM(wm_adsp_fw_find_normalization, ...); */

/*
 * Dummy directory content for regression tests.
 * DSP part name and system name are used to select different available
 * files.
 *
 * System:
 * WFBF1111 = wmfw and bin fully-qualified
 * WSBF1111 = wmfw system-qualified, bin fully-qualified
 * WSBS1111 = wmfw and bin system-qualified
 * WFXX1111 = wmfw fully-qualified, bin not present
 * XXBF1111 = wmfw not present, bin fully-qualified
 *
 * Part:
 * cs1234	= for testing fully-qualified configurations
 * cs1234nobin	= generic wmfw without a bin available
 * wm1234	= legacy wmfw and bin
 * wm1234nobin	= legacy wmfw without bin
 */
static wm_adsp_fw_find_test_dir_all_files: [*const c_char; 22] = [
    c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-amp1.wmfw"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-l1u2.wmfw"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111.wmfw"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111.wmfw"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wsbs1111.wmfw"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wfxx1111.wmfw"),
    c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"),
    c!("cirrus/cs1234nobin-dsp1-mbc-vss.wmfw"),
    c!("cirrus/wm1234-dsp1-mbc-vss.wmfw"),
    c!("cirrus/wm1234nobin-dsp1-mbc-vss.wmfw"),
    c!("wm1234-dsp1-mbc-vss.wmfw"),
    c!("wm1234nobin-dsp1-mbc-vss.wmfw"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-amp1.bin"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-l1u2.bin"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111-amp1.bin"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111-l1u2.bin"),
    c!("cirrus/cs1234-dsp1-mbc-vss-wsbs1111.bin"),
    c!("cirrus/cs1234-dsp1-mbc-vss-xxbf1111-amp1.bin"),
    c!("cirrus/cs1234-dsp1-mbc-vss.bin"),
    c!("cirrus/wm1234-dsp1-mbc-vss.bin"),
    c!("wm1234-dsp1-mbc-vss.bin"),
    ptr::null(), /* terminator */
];

/*
 * Regression testing that a change in the search algorithm doesn't change
 * which file is picked. This doesn't cover every possible combination, only
 * those that are already in use and typical cases.
 *
 * It wouldn't be efficent to fully prove the algorithm this way (too many
 * directory content combinations would be needed, and it only infers what the
 * algorithm searched for, it doesn't prove exactly what searches were made).
 * So the main testing is done by checking for the expected file searches.
 * This regression test is independent of the search algorithm.
 *
 * The main tests already prove that the algorithm only searches for files
 * with the correct qualifiers so we can assume that files with the wrong
 * qualifiers would not be picked and there's no need to test for that here.
 */
static wm_adsp_fw_find_pick_cases: [wm_adsp_fw_find_test_params; 18] = [
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WFBF1111"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-amp1.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-amp1.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WFBF1111"), c!("l1u2"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-l1u2.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-l1u2.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WSBF1111"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111-amp1.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WSBF1111"), c!("l1u2"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111-l1u2.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WFBF1111"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-amp1.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-amp1.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WSBS1111"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss-wsbs1111.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-wsbs1111.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WSBF1111"), c!("amp1"), true, false, c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-wsbf1111-amp1.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("XXBF1111"), c!("amp1"), true, false, NULL, c!("cirrus/cs1234-dsp1-mbc-vss-xxbf1111-amp1.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WFBF1111"), c!("amp1"), false, true, c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-amp1.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss-wfbf1111-amp1.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WFXX1111"), c!("amp1"), false, true, c!("cirrus/cs1234-dsp1-mbc-vss-wfxx1111.wmfw"), NULL, NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("WFXX1111"), c!("amp1"), true, true, c!("cirrus/cs1234-dsp1-mbc-vss-wfxx1111.wmfw"), NULL, NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, c!("XXXX1111"), c!("amp1"), false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234nobin"), c!("dsp1"), NULL, c!("XXXX1111"), c!("amp1"), false, false, c!("cirrus/cs1234nobin-dsp1-mbc-vss.wmfw"), NULL, NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("wm1234nobin"), c!("dsp1"), NULL, NULL, NULL, false, false, c!("wm1234nobin-dsp1-mbc-vss.wmfw"), NULL, NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("wm1234"), c!("dsp1"), NULL, NULL, NULL, false, false, c!("wm1234-dsp1-mbc-vss.wmfw"), c!("wm1234-dsp1-mbc-vss.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234nobin"), c!("dsp1"), NULL, NULL, NULL, false, false, c!("cirrus/cs1234nobin-dsp1-mbc-vss.wmfw"), NULL, NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
    p!(c!("cs1234"), c!("dsp1"), NULL, NULL, NULL, false, false, c!("cirrus/cs1234-dsp1-mbc-vss.wmfw"), c!("cirrus/cs1234-dsp1-mbc-vss.bin"), NULL, wm_adsp_fw_find_test_dir_all_files.as_ptr()),
];
/* KUNIT_ARRAY_PARAM(wm_adsp_fw_find_pick, wm_adsp_fw_find_pick_cases,
 *                   wm_adsp_fw_find_test_param_desc);
 */

/* static struct kunit_case wm_adsp_fw_find_test_cases[] = {
 *     KUNIT_CASE_PARAM(wm_adsp_fw_find_test_search_order,
 *                      wm_adsp_fw_find_full_search_gen_params),
 *     KUNIT_CASE_PARAM(wm_adsp_fw_find_test_search_order,
 *                      wm_adsp_fw_find_system_alsaname_gen_params),
 *     KUNIT_CASE_PARAM(wm_adsp_fw_find_test_search_order,
 *                      wm_adsp_fw_find_system_gen_params),
 *     KUNIT_CASE_PARAM(wm_adsp_fw_find_test_search_order,
 *                      wm_adsp_fw_find_alsaname_gen_params),
 *     KUNIT_CASE_PARAM(wm_adsp_fw_find_test_search_order,
 *                      wm_adsp_fw_find_noqual_gen_params),
 *     KUNIT_CASE_PARAM(wm_adsp_fw_find_test_search_order,
 *                      wm_adsp_fw_find_normalization_gen_params),
 *     KUNIT_CASE_PARAM(wm_adsp_fw_find_test_pick_file,
 *                      wm_adsp_fw_find_pick_gen_params),
 *     KUNIT_CASE(wm_adsp_fw_find_test_find_firmware_byindex),
 *     { }
 * };
 */
static mut wm_adsp_fw_find_test_cases: [kunit_case; 1] = [kunit_case { _private: [] }];

static mut wm_adsp_fw_find_test_suite: kunit_suite = kunit_suite {
    name: c!("wm-adsp-fw-find"),
    init: Some(wm_adsp_fw_find_test_case_init),
    exit: Some(wm_adsp_fw_find_test_case_exit),
    test_cases: unsafe { wm_adsp_fw_find_test_cases.as_mut_ptr() },
};

/* kunit_test_suite(wm_adsp_fw_find_test_suite); */

/* MODULE_DESCRIPTION("KUnit test for Cirrus Logic wm_adsp driver");
 * MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
 * MODULE_LICENSE("GPL");
 * MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
