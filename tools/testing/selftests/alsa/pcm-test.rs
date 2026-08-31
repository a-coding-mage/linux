// SPDX-License-Identifier: GPL-2.0
//
// kselftest for the ALSA PCM API
//
// Original author: Jaroslav Kysela <perex@perex.cz>
// Copyright (c) 2022 Red Hat Inc.

// This test will iterate over all cards detected in the system, exercising
// every PCM device it can find.  This may conflict with other system
// software if there is audio activity so is best run on a system with a
// minimal active userspace.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

type timestamp_t = timespec;

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

pub enum snd_ctl_card_info_t {}
pub enum snd_pcm_t {}
pub enum snd_config_t {}
pub enum snd_ctl_t {}
pub enum snd_pcm_info_t {}
pub enum snd_pcm_hw_params_t {}
pub enum snd_pcm_sw_params_t {}
pub enum card_cfg_data {}

pub type pthread_t = usize;
pub type pthread_mutex_t = c_int;
pub type snd_config_iterator_t = *mut c_void;
pub type snd_pcm_stream_t = c_int;
pub type snd_pcm_access_t = c_int;
pub type snd_pcm_format_t = c_int;
pub type snd_pcm_sframes_t = c_long;
pub type snd_pcm_uframes_t = c_ulong;
pub type c_ulong = u64;

const CLOCK_MONOTONIC_RAW: c_int = 4;
const ENOENT: c_int = 2;
const SND_CONFIG_TYPE_COMPOUND: c_int = 102;
const SND_PCM_STREAM_PLAYBACK: snd_pcm_stream_t = 0;
const SND_PCM_STREAM_CAPTURE: snd_pcm_stream_t = 1;
const SND_PCM_ACCESS_RW_INTERLEAVED: snd_pcm_access_t = 3;
const SND_PCM_FORMAT_UNKNOWN: snd_pcm_format_t = -1;
const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = 0;

#[repr(C)]
pub struct card_data {
    pub card: c_int,
    pub info: *mut snd_ctl_card_info_t,
    pub name: *const c_char,
    pub thread: pthread_t,
    pub next: *mut card_data,
}

static mut card_list: *mut card_data = ptr::null_mut();

#[repr(C)]
pub struct pcm_data {
    pub handle: *mut snd_pcm_t,
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
    pub card_name: *const c_char,
    pub stream: snd_pcm_stream_t,
    pub pcm_config: *mut snd_config_t,
    pub next: *mut pcm_data,
}

static mut pcm_list: *mut pcm_data = ptr::null_mut();

static mut num_missing: c_int = 0;
static mut pcm_missing: *mut pcm_data = ptr::null_mut();

static mut default_pcm_config: *mut snd_config_t = ptr::null_mut();

/* Lock while reporting results since kselftest doesn't */
static mut results_lock: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum test_class {
    TEST_CLASS_DEFAULT,
    TEST_CLASS_SYSTEM,
}

unsafe extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    static mut errno: c_int;

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn ksft_exit_fail_msg(format: *const c_char, ...) -> !;
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result(pass: bool, format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_test_result_fail(format: *const c_char, ...);
    fn ksft_exit_pass() -> !;

    fn get_alsalib_config() -> *mut snd_config_t;
    fn conf_load_from_file(filename: *const c_char) -> *mut snd_config_t;
    fn conf_load();
    fn conf_free();
    fn conf_by_card(card: c_int) -> *mut snd_config_t;
    fn conf_get_subtree(
        cfg: *mut snd_config_t,
        key: *const c_char,
        default_value: *mut snd_config_t,
    ) -> *mut snd_config_t;
    fn conf_get_bool(
        cfg: *mut snd_config_t,
        key: *const c_char,
        name: *const c_char,
        default_value: bool,
    ) -> bool;
    fn conf_get_string(
        cfg: *mut snd_config_t,
        key: *const c_char,
        name: *const c_char,
        default_value: *const c_char,
    ) -> *const c_char;
    fn conf_get_string_array(
        cfg: *mut snd_config_t,
        key: *const c_char,
        name: *const c_char,
        array: *mut *const c_char,
        array_size: usize,
        default_value: *const c_char,
    );
    fn conf_get_long(
        cfg: *mut snd_config_t,
        key: *const c_char,
        name: *const c_char,
        default_value: c_long,
    ) -> c_long;
    fn conf_get_count(cfg: *mut snd_config_t, key: *const c_char, name: *const c_char) -> c_int;
    static mut conf_cards: *mut card_cfg_data;

    fn snd_config_get_id(config: *mut snd_config_t, value: *mut *const c_char) -> c_int;
    fn snd_config_get_type(config: *mut snd_config_t) -> c_int;
    fn snd_config_iterator_entry(iterator: snd_config_iterator_t) -> *mut snd_config_t;
    fn snd_config_delete(config: *mut snd_config_t) -> c_int;
    fn snd_config_iterator_first(node: *mut snd_config_t) -> snd_config_iterator_t;
    fn snd_config_iterator_next(iterator: snd_config_iterator_t) -> snd_config_iterator_t;
    fn snd_config_iterator_end(node: *mut snd_config_t) -> snd_config_iterator_t;

    fn snd_card_next(card: *mut c_int) -> c_int;
    fn snd_card_get_name(card: c_int, name: *mut *mut c_char) -> c_int;
    fn snd_card_get_longname(card: c_int, name: *mut *mut c_char) -> c_int;
    fn snd_ctl_open_lconf(
        ctl: *mut *mut snd_ctl_t,
        name: *const c_char,
        mode: c_int,
        lconf: *mut snd_config_t,
    ) -> c_int;
    fn snd_ctl_close(ctl: *mut snd_ctl_t) -> c_int;
    fn snd_ctl_card_info_malloc(ptr: *mut *mut snd_ctl_card_info_t) -> c_int;
    fn snd_ctl_card_info(ctl: *mut snd_ctl_t, info: *mut snd_ctl_card_info_t) -> c_int;
    fn snd_ctl_card_info_get_id(obj: *const snd_ctl_card_info_t) -> *const c_char;
    fn snd_ctl_pcm_next_device(ctl: *mut snd_ctl_t, device: *mut c_int) -> c_int;
    fn snd_ctl_pcm_info(ctl: *mut snd_ctl_t, info: *mut snd_pcm_info_t) -> c_int;

    fn snd_pcm_info_alloca(ptr: *mut *mut snd_pcm_info_t);
    fn snd_pcm_info_set_device(obj: *mut snd_pcm_info_t, val: c_uint);
    fn snd_pcm_info_set_subdevice(obj: *mut snd_pcm_info_t, val: c_uint);
    fn snd_pcm_info_set_stream(obj: *mut snd_pcm_info_t, val: snd_pcm_stream_t);
    fn snd_pcm_info_get_id(obj: *const snd_pcm_info_t) -> *const c_char;
    fn snd_pcm_info_get_subdevices_count(obj: *const snd_pcm_info_t) -> c_uint;

    fn snd_pcm_stream_name(stream: snd_pcm_stream_t) -> *const c_char;
    fn snd_pcm_access_name(access: snd_pcm_access_t) -> *const c_char;
    fn snd_pcm_format_name(format: snd_pcm_format_t) -> *const c_char;
    fn snd_pcm_format_value(name: *const c_char) -> snd_pcm_format_t;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_set_silence(format: snd_pcm_format_t, data: *mut c_void, samples: c_uint) -> c_int;
    fn snd_pcm_open(
        pcmp: *mut *mut snd_pcm_t,
        name: *const c_char,
        stream: snd_pcm_stream_t,
        mode: c_int,
    ) -> c_int;
    fn snd_pcm_close(pcm: *mut snd_pcm_t) -> c_int;
    fn snd_pcm_hw_params_alloca(ptr: *mut *mut snd_pcm_hw_params_t);
    fn snd_pcm_sw_params_alloca(ptr: *mut *mut snd_pcm_sw_params_t);
    fn snd_pcm_hw_params_any(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int;
    fn snd_pcm_hw_params_set_rate_resample(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_params_set_access(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        access: snd_pcm_access_t,
    ) -> c_int;
    fn snd_pcm_hw_params_set_format(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        format: snd_pcm_format_t,
    ) -> c_int;
    fn snd_pcm_hw_params_set_channels(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_params_set_rate_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut c_uint,
        dir: *mut c_int,
    ) -> c_int;
    fn snd_pcm_hw_params_set_period_size_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut snd_pcm_uframes_t,
        dir: *mut c_int,
    ) -> c_int;
    fn snd_pcm_hw_params_set_buffer_size_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut snd_pcm_uframes_t,
    ) -> c_int;
    fn snd_pcm_hw_params(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int;
    fn snd_pcm_sw_params_current(pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t) -> c_int;
    fn snd_pcm_sw_params_set_start_threshold(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_sw_params_t,
        val: snd_pcm_uframes_t,
    ) -> c_int;
    fn snd_pcm_sw_params_set_avail_min(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_sw_params_t,
        val: snd_pcm_uframes_t,
    ) -> c_int;
    fn snd_pcm_sw_params(pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t) -> c_int;
    fn snd_pcm_writei(pcm: *mut snd_pcm_t, buffer: *const c_void, size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_readi(pcm: *mut snd_pcm_t, buffer: *mut c_void, size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_drain(pcm: *mut snd_pcm_t) -> c_int;
    fn snd_strerror(errnum: c_int) -> *const c_char;
}

#[repr(C)]
pub struct card_cfg_data_fields {
    pub card: c_int,
    pub filename: *const c_char,
    pub config_id: *const c_char,
    pub next: *mut card_cfg_data,
}

unsafe fn card_cfg_card(conf: *mut card_cfg_data) -> c_int {
    (*(conf as *mut card_cfg_data_fields)).card
}

unsafe fn card_cfg_filename(conf: *mut card_cfg_data) -> *const c_char {
    (*(conf as *mut card_cfg_data_fields)).filename
}

unsafe fn card_cfg_config_id(conf: *mut card_cfg_data) -> *const c_char {
    (*(conf as *mut card_cfg_data_fields)).config_id
}

unsafe fn card_cfg_next(conf: *mut card_cfg_data) -> *mut card_cfg_data {
    (*(conf as *mut card_cfg_data_fields)).next
}

unsafe fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe fn snd_config_for_each<F: FnMut(snd_config_iterator_t)>(node: *mut snd_config_t, mut f: F) {
    let mut i = snd_config_iterator_first(node);
    let end = snd_config_iterator_end(node);
    while i != end {
        let next = snd_config_iterator_next(i);
        f(i);
        i = next;
    }
}

unsafe extern "C" fn timestamp_now(tstamp: *mut timestamp_t) {
    if clock_gettime(CLOCK_MONOTONIC_RAW, tstamp) != 0 {
        ksft_exit_fail_msg(c"clock_get_time\n".as_ptr());
    }
}

unsafe extern "C" fn timestamp_diff_ms(tstamp: *mut timestamp_t) -> i64 {
    let mut now: timestamp_t = mem::zeroed();
    let mut diff: timestamp_t = mem::zeroed();
    timestamp_now(&mut now);
    if (*tstamp).tv_nsec > now.tv_nsec {
        diff.tv_sec = now.tv_sec - (*tstamp).tv_sec - 1;
        diff.tv_nsec = (now.tv_nsec + 1000000000_i64) - (*tstamp).tv_nsec;
    } else {
        diff.tv_sec = now.tv_sec - (*tstamp).tv_sec;
        diff.tv_nsec = now.tv_nsec - (*tstamp).tv_nsec;
    }
    ((diff.tv_sec * 1000) + ((diff.tv_nsec + 500000_i64) / 1000000_i64)) as i64
}

unsafe fn device_from_id(node: *mut snd_config_t) -> c_long {
    let mut id: *const c_char = ptr::null();
    let mut end: *mut c_char = ptr::null_mut();
    let v: c_long;

    if snd_config_get_id(node, &mut id) != 0 {
        ksft_exit_fail_msg(c"snd_config_get_id\n".as_ptr());
    }
    errno = 0;
    v = strtol(id, &mut end, 10);
    if errno != 0 || *end != 0 {
        return -1;
    }
    v
}

unsafe fn missing_device(card: c_int, device: c_int, subdevice: c_int, stream: snd_pcm_stream_t) {
    let mut pcm_data_p: *mut pcm_data;

    pcm_data_p = pcm_list;
    while !pcm_data_p.is_null() {
        if (*pcm_data_p).card != card {
            pcm_data_p = (*pcm_data_p).next;
            continue;
        }
        if (*pcm_data_p).device != device {
            pcm_data_p = (*pcm_data_p).next;
            continue;
        }
        if (*pcm_data_p).subdevice != subdevice {
            pcm_data_p = (*pcm_data_p).next;
            continue;
        }
        if (*pcm_data_p).stream != stream {
            pcm_data_p = (*pcm_data_p).next;
            continue;
        }
        return;
    }
    pcm_data_p = calloc(1, mem::size_of::<pcm_data>()) as *mut pcm_data;
    if pcm_data_p.is_null() {
        ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
    }
    (*pcm_data_p).card = card;
    (*pcm_data_p).device = device;
    (*pcm_data_p).subdevice = subdevice;
    (*pcm_data_p).stream = stream;
    (*pcm_data_p).next = pcm_missing;
    pcm_missing = pcm_data_p;
    num_missing += 1;
}

unsafe fn missing_devices(card: c_int, card_config: *mut snd_config_t) {
    let mut pcm_config: *mut snd_config_t;

    pcm_config = conf_get_subtree(card_config, c"pcm".as_ptr(), ptr::null_mut());
    if pcm_config.is_null() {
        return;
    }
    snd_config_for_each(pcm_config, |i1| unsafe {
        let node1 = snd_config_iterator_entry(i1);
        let device = device_from_id(node1) as c_int;
        if device < 0 {
            return;
        }
        if snd_config_get_type(node1) != SND_CONFIG_TYPE_COMPOUND {
            return;
        }
        snd_config_for_each(node1, |i2| unsafe {
            let node2 = snd_config_iterator_entry(i2);
            let subdevice = device_from_id(node2) as c_int;
            if subdevice < 0 {
                return;
            }
            if !conf_get_subtree(node2, c"PLAYBACK".as_ptr(), ptr::null_mut()).is_null() {
                missing_device(card, device, subdevice, SND_PCM_STREAM_PLAYBACK);
            }
            if !conf_get_subtree(node2, c"CAPTURE".as_ptr(), ptr::null_mut()).is_null() {
                missing_device(card, device, subdevice, SND_PCM_STREAM_CAPTURE);
            }
        });
    });
}

unsafe fn find_pcms() {
    let mut name = [0 as c_char; 32];
    let mut key = [0 as c_char; 64];
    let mut card_name: *mut c_char = ptr::null_mut();
    let mut card_longname: *mut c_char = ptr::null_mut();
    let mut card: c_int;
    let mut dev: c_int;
    let mut subdev: c_int;
    let mut count: c_int;
    let mut direction: c_int;
    let mut err: c_int;
    let mut stream: snd_pcm_stream_t;
    let mut pcm_data_p: *mut pcm_data;
    let mut handle: *mut snd_ctl_t = ptr::null_mut();
    let mut pcm_info: *mut snd_pcm_info_t = ptr::null_mut();
    let mut config: *mut snd_config_t;
    let mut card_config: *mut snd_config_t;
    let mut pcm_config: *mut snd_config_t;
    let mut card_data_p: *mut card_data;

    snd_pcm_info_alloca(&mut pcm_info);

    card = -1;
    if snd_card_next(&mut card) < 0 || card < 0 {
        return;
    }

    config = get_alsalib_config();

    while card >= 0 {
        card_data_p = calloc(1, mem::size_of::<card_data>()) as *mut card_data;
        if card_data_p.is_null() {
            ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
        }

        sprintf(name.as_mut_ptr(), c"hw:%d".as_ptr(), card);

        err = snd_ctl_open_lconf(&mut handle, name.as_ptr(), 0, config);
        if err < 0 {
            ksft_print_msg(
                c"Failed to get hctl for card %d: %s\n".as_ptr(),
                card,
                snd_strerror(err),
            );
            snd_ctl_close(handle);
            if snd_card_next(&mut card) < 0 {
                ksft_print_msg(c"snd_card_next".as_ptr());
                break;
            }
            continue;
        }

        err = snd_card_get_name(card, &mut card_name);
        if err != 0 {
            card_name = c"Unknown".as_ptr() as *mut c_char;
        }
        err = snd_card_get_longname(card, &mut card_longname);
        if err != 0 {
            card_longname = c"Unknown".as_ptr() as *mut c_char;
        }

        err = snd_ctl_card_info_malloc(&mut (*card_data_p).info);
        if err != 0 {
            ksft_exit_fail_msg(c"Failed to allocate card info: %d\n".as_ptr(), err);
        }

        err = snd_ctl_card_info(handle, (*card_data_p).info);
        if err == 0 {
            (*card_data_p).name = snd_ctl_card_info_get_id((*card_data_p).info);
            if (*card_data_p).name.is_null() {
                ksft_print_msg(c"Failed to get card ID\n".as_ptr());
            }
        } else {
            ksft_print_msg(c"Failed to get card info: %d\n".as_ptr(), err);
        }

        if (*card_data_p).name.is_null() {
            (*card_data_p).name = c"Unknown".as_ptr();
        }

        ksft_print_msg(
            c"Card %d/%s - %s (%s)\n".as_ptr(),
            card,
            (*card_data_p).name,
            card_name,
            card_longname,
        );

        card_config = conf_by_card(card);

        (*card_data_p).card = card;
        (*card_data_p).next = card_list;
        card_list = card_data_p;

        dev = -1;
        loop {
            if snd_ctl_pcm_next_device(handle, &mut dev) < 0 {
                ksft_exit_fail_msg(c"snd_ctl_pcm_next_device\n".as_ptr());
            }
            if dev < 0 {
                break;
            }

            direction = 0;
            while direction < 2 {
                stream = if direction != 0 {
                    SND_PCM_STREAM_CAPTURE
                } else {
                    SND_PCM_STREAM_PLAYBACK
                };
                sprintf(
                    key.as_mut_ptr(),
                    c"pcm.%d.%s".as_ptr(),
                    dev,
                    snd_pcm_stream_name(stream),
                );
                pcm_config = conf_get_subtree(card_config, key.as_ptr(), ptr::null_mut());
                if conf_get_bool(card_config, key.as_ptr(), c"skip".as_ptr(), false) {
                    ksft_print_msg(
                        c"skipping pcm %d.%d.%s\n".as_ptr(),
                        card,
                        dev,
                        snd_pcm_stream_name(stream),
                    );
                    direction += 1;
                    continue;
                }
                snd_pcm_info_set_device(pcm_info, dev as c_uint);
                snd_pcm_info_set_subdevice(pcm_info, 0);
                snd_pcm_info_set_stream(pcm_info, stream);
                err = snd_ctl_pcm_info(handle, pcm_info);
                if err == -ENOENT {
                    direction += 1;
                    continue;
                }
                if err < 0 {
                    ksft_exit_fail_msg(c"snd_ctl_pcm_info: %d:%d:%d\n".as_ptr(), dev, 0, stream);
                }

                ksft_print_msg(
                    c"%s.0 - %s\n".as_ptr(),
                    (*card_data_p).name,
                    snd_pcm_info_get_id(pcm_info),
                );

                count = snd_pcm_info_get_subdevices_count(pcm_info) as c_int;
                subdev = 0;
                while subdev < count {
                    sprintf(
                        key.as_mut_ptr(),
                        c"pcm.%d.%d.%s".as_ptr(),
                        dev,
                        subdev,
                        snd_pcm_stream_name(stream),
                    );
                    if conf_get_bool(card_config, key.as_ptr(), c"skip".as_ptr(), false) {
                        ksft_print_msg(
                            c"skipping pcm %d.%d.%d.%s\n".as_ptr(),
                            card,
                            dev,
                            subdev,
                            snd_pcm_stream_name(stream),
                        );
                        subdev += 1;
                        continue;
                    }
                    pcm_data_p = calloc(1, mem::size_of::<pcm_data>()) as *mut pcm_data;
                    if pcm_data_p.is_null() {
                        ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
                    }
                    (*pcm_data_p).card = card;
                    (*pcm_data_p).device = dev;
                    (*pcm_data_p).subdevice = subdev;
                    (*pcm_data_p).card_name = (*card_data_p).name;
                    (*pcm_data_p).stream = stream;
                    (*pcm_data_p).pcm_config = conf_get_subtree(card_config, key.as_ptr(), ptr::null_mut());
                    (*pcm_data_p).next = pcm_list;
                    pcm_list = pcm_data_p;
                    subdev += 1;
                }
                direction += 1;
            }
        }

        /* check for missing devices */
        missing_devices(card, card_config);

        snd_ctl_close(handle);
        if snd_card_next(&mut card) < 0 {
            ksft_print_msg(c"snd_card_next".as_ptr());
            break;
        }
    }

    snd_config_delete(config);
}

unsafe fn test_pcm_time(
    data: *mut pcm_data,
    class: test_class,
    test_name: *const c_char,
    pcm_cfg: *mut snd_config_t,
) {
    let mut name = [0 as c_char; 64];
    let mut msg = [0 as c_char; 256];
    const duration_s: c_int = 2;
    const margin_ms: c_int = 100;
    const duration_ms: c_int = duration_s * 1000;
    let mut cs: *const c_char;
    let mut i: c_int;
    let mut err: c_int;
    let mut handle: *mut snd_pcm_t = ptr::null_mut();
    let access: snd_pcm_access_t = SND_PCM_ACCESS_RW_INTERLEAVED;
    let mut format: snd_pcm_format_t;
    let mut old_format: snd_pcm_format_t;
    let mut alt_formats: [*const c_char; 8] = [ptr::null(); 8];
    let mut samples: *mut u8 = ptr::null_mut();
    let mut frames: snd_pcm_sframes_t;
    let mut ms: i64;
    let mut rate: c_long;
    let mut channels: c_long;
    let mut period_size: c_long;
    let mut buffer_size: c_long;
    let mut rrate: c_uint;
    let mut rperiod_size: snd_pcm_uframes_t;
    let mut rbuffer_size: snd_pcm_uframes_t;
    let mut start_threshold: snd_pcm_uframes_t;
    let mut tstamp: timestamp_t = mem::zeroed();
    let mut pass = false;
    let mut hw_params: *mut snd_pcm_hw_params_t = ptr::null_mut();
    let mut sw_params: *mut snd_pcm_sw_params_t = ptr::null_mut();
    let mut test_class_name: *const c_char;
    let mut skip = true;
    let mut desc: *const c_char;

    match class {
        test_class::TEST_CLASS_DEFAULT => {
            test_class_name = c"default".as_ptr();
        }
        test_class::TEST_CLASS_SYSTEM => {
            test_class_name = c"system".as_ptr();
        }
    }

    desc = conf_get_string(pcm_cfg, c"description".as_ptr(), ptr::null(), ptr::null());
    if !desc.is_null() {
        ksft_print_msg(
            c"%s.%s.%s.%d.%d.%s - %s\n".as_ptr(),
            test_class_name,
            test_name,
            (*data).card_name,
            (*data).device,
            (*data).subdevice,
            snd_pcm_stream_name((*data).stream),
            desc,
        );
    }

    snd_pcm_hw_params_alloca(&mut hw_params);
    snd_pcm_sw_params_alloca(&mut sw_params);

    cs = conf_get_string(pcm_cfg, c"format".as_ptr(), ptr::null(), c"S16_LE".as_ptr());
    format = snd_pcm_format_value(cs);
    if format == SND_PCM_FORMAT_UNKNOWN {
        ksft_exit_fail_msg(c"Wrong format '%s'\n".as_ptr(), cs);
    }
    conf_get_string_array(
        pcm_cfg,
        c"alt_formats".as_ptr(),
        ptr::null(),
        alt_formats.as_mut_ptr(),
        ARRAY_SIZE(&alt_formats),
        ptr::null(),
    );
    rate = conf_get_long(pcm_cfg, c"rate".as_ptr(), ptr::null(), 48000);
    channels = conf_get_long(pcm_cfg, c"channels".as_ptr(), ptr::null(), 2);
    period_size = conf_get_long(pcm_cfg, c"period_size".as_ptr(), ptr::null(), 4096);
    buffer_size = conf_get_long(pcm_cfg, c"buffer_size".as_ptr(), ptr::null(), 16384);

    samples = malloc(((rate * channels * snd_pcm_format_physical_width(format) as c_long) / 8) as usize) as *mut u8;
    if samples.is_null() {
        ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
    }
    snd_pcm_format_set_silence(format, samples as *mut c_void, (rate * channels) as c_uint);

    sprintf(
        name.as_mut_ptr(),
        c"hw:%d,%d,%d".as_ptr(),
        (*data).card,
        (*data).device,
        (*data).subdevice,
    );
    err = snd_pcm_open(&mut handle, name.as_ptr(), (*data).stream, 0);
    if err < 0 {
        snprintf(msg.as_mut_ptr(), msg.len(), c"Failed to get pcm handle: %s".as_ptr(), snd_strerror(err));
        goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
        return;
    }

    macro_rules! close_on_err {
        ($cond:expr, $($arg:tt)*) => {
            if $cond {
                snprintf(msg.as_mut_ptr(), msg.len(), $($arg)*);
                goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
                return;
            }
        };
    }

    err = snd_pcm_hw_params_any(handle, hw_params);
    close_on_err!(err < 0, c"snd_pcm_hw_params_any: %s".as_ptr(), snd_strerror(err));
    err = snd_pcm_hw_params_set_rate_resample(handle, hw_params, 0);
    close_on_err!(err < 0, c"snd_pcm_hw_params_set_rate_resample: %s".as_ptr(), snd_strerror(err));
    err = snd_pcm_hw_params_set_access(handle, hw_params, access);
    close_on_err!(
        err < 0,
        c"snd_pcm_hw_params_set_access %s: %s".as_ptr(),
        snd_pcm_access_name(access),
        snd_strerror(err)
    );
    i = -1;
    loop {
        err = snd_pcm_hw_params_set_format(handle, hw_params, format);
        if err >= 0 {
            break;
        }
        i += 1;
        if (i as usize) < ARRAY_SIZE(&alt_formats) && !alt_formats[i as usize].is_null() {
            old_format = format;
            format = snd_pcm_format_value(alt_formats[i as usize]);
            if format != SND_PCM_FORMAT_UNKNOWN {
                ksft_print_msg(
                    c"%s.%s.%d.%d.%s.%s format %s -> %s\n".as_ptr(),
                    test_name,
                    (*data).card_name,
                    (*data).device,
                    (*data).subdevice,
                    snd_pcm_stream_name((*data).stream),
                    snd_pcm_access_name(access),
                    snd_pcm_format_name(old_format),
                    snd_pcm_format_name(format),
                );
                samples = realloc(
                    samples as *mut c_void,
                    ((rate * channels * snd_pcm_format_physical_width(format) as c_long) / 8) as usize,
                ) as *mut u8;
                if samples.is_null() {
                    ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
                }
                snd_pcm_format_set_silence(format, samples as *mut c_void, (rate * channels) as c_uint);
                continue;
            }
        }
        snprintf(
            msg.as_mut_ptr(),
            msg.len(),
            c"snd_pcm_hw_params_set_format %s: %s".as_ptr(),
            snd_pcm_format_name(format),
            snd_strerror(err),
        );
        goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
        return;
    }
    err = snd_pcm_hw_params_set_channels(handle, hw_params, channels as c_uint);
    close_on_err!(err < 0, c"snd_pcm_hw_params_set_channels %ld: %s".as_ptr(), channels, snd_strerror(err));
    rrate = rate as c_uint;
    err = snd_pcm_hw_params_set_rate_near(handle, hw_params, &mut rrate, ptr::null_mut());
    close_on_err!(err < 0, c"snd_pcm_hw_params_set_rate %ld: %s".as_ptr(), rate, snd_strerror(err));
    if rrate != rate as c_uint {
        snprintf(msg.as_mut_ptr(), msg.len(), c"rate mismatch %ld != %u".as_ptr(), rate, rrate);
        goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
        return;
    }
    rperiod_size = period_size as snd_pcm_uframes_t;
    err = snd_pcm_hw_params_set_period_size_near(handle, hw_params, &mut rperiod_size, ptr::null_mut());
    close_on_err!(err < 0, c"snd_pcm_hw_params_set_period_size %ld: %s".as_ptr(), period_size, snd_strerror(err));
    rbuffer_size = buffer_size as snd_pcm_uframes_t;
    err = snd_pcm_hw_params_set_buffer_size_near(handle, hw_params, &mut rbuffer_size);
    close_on_err!(err < 0, c"snd_pcm_hw_params_set_buffer_size %ld: %s".as_ptr(), buffer_size, snd_strerror(err));
    err = snd_pcm_hw_params(handle, hw_params);
    close_on_err!(err < 0, c"snd_pcm_hw_params: %s".as_ptr(), snd_strerror(err));

    err = snd_pcm_sw_params_current(handle, sw_params);
    close_on_err!(err < 0, c"snd_pcm_sw_params_current: %s".as_ptr(), snd_strerror(err));
    if (*data).stream == SND_PCM_STREAM_PLAYBACK {
        start_threshold = (rbuffer_size / rperiod_size) * rperiod_size;
    } else {
        start_threshold = rperiod_size;
    }
    err = snd_pcm_sw_params_set_start_threshold(handle, sw_params, start_threshold);
    close_on_err!(
        err < 0,
        c"snd_pcm_sw_params_set_start_threshold %ld: %s".as_ptr(),
        start_threshold as c_long,
        snd_strerror(err)
    );
    err = snd_pcm_sw_params_set_avail_min(handle, sw_params, rperiod_size);
    close_on_err!(
        err < 0,
        c"snd_pcm_sw_params_set_avail_min %ld: %s".as_ptr(),
        rperiod_size as c_long,
        snd_strerror(err)
    );
    err = snd_pcm_sw_params(handle, sw_params);
    close_on_err!(err < 0, c"snd_pcm_sw_params: %s".as_ptr(), snd_strerror(err));

    ksft_print_msg(
        c"%s.%s.%s.%d.%d.%s hw_params.%s.%s.%ld.%ld.%ld.%ld sw_params.%ld\n".as_ptr(),
        test_class_name,
        test_name,
        (*data).card_name,
        (*data).device,
        (*data).subdevice,
        snd_pcm_stream_name((*data).stream),
        snd_pcm_access_name(access),
        snd_pcm_format_name(format),
        rate,
        channels,
        rperiod_size as c_long,
        rbuffer_size as c_long,
        start_threshold as c_long,
    );

    /* Set all the params, actually run the test */
    skip = false;

    timestamp_now(&mut tstamp);
    i = 0;
    while i < duration_s {
        if (*data).stream == SND_PCM_STREAM_PLAYBACK {
            frames = snd_pcm_writei(handle, samples as *const c_void, rate as snd_pcm_uframes_t);
            if frames < 0 {
                snprintf(msg.as_mut_ptr(), msg.len(), c"Write failed: expected %ld, wrote %li".as_ptr(), rate, frames);
                goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
                return;
            }
            if frames < rate {
                snprintf(msg.as_mut_ptr(), msg.len(), c"expected %ld, wrote %li".as_ptr(), rate, frames);
                goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
                return;
            }
        } else {
            frames = snd_pcm_readi(handle, samples as *mut c_void, rate as snd_pcm_uframes_t);
            if frames < 0 {
                snprintf(msg.as_mut_ptr(), msg.len(), c"expected %ld, wrote %li".as_ptr(), rate, frames);
                goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
                return;
            }
            if frames < rate {
                snprintf(msg.as_mut_ptr(), msg.len(), c"expected %ld, wrote %li".as_ptr(), rate, frames);
                goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
                return;
            }
        }
        i += 1;
    }

    snd_pcm_drain(handle);
    ms = timestamp_diff_ms(&mut tstamp);
    if ms < (duration_ms - margin_ms) as i64 || ms > (duration_ms + margin_ms) as i64 {
        snprintf(msg.as_mut_ptr(), msg.len(), c"time mismatch: expected %dms got %lld".as_ptr(), duration_ms, ms);
        goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
        return;
    }

    msg[0] = 0;
    pass = true;
    goto_close(data, class, test_name, &mut test_class_name, skip, pass, msg.as_mut_ptr(), samples, handle);
}

unsafe fn goto_close(
    data: *mut pcm_data,
    class: test_class,
    test_name: *const c_char,
    test_class_name: *mut *const c_char,
    skip: bool,
    pass: bool,
    msg: *mut c_char,
    samples: *mut u8,
    handle: *mut snd_pcm_t,
) {
    pthread_mutex_lock(&mut results_lock);

    match class {
        test_class::TEST_CLASS_SYSTEM => {
            *test_class_name = c"system".as_ptr();
            /*
             * Anything specified as specific to this system
             * should always be supported.
             */
            ksft_test_result(
                !skip,
                c"%s.%s.%s.%d.%d.%s.params\n".as_ptr(),
                *test_class_name,
                test_name,
                (*data).card_name,
                (*data).device,
                (*data).subdevice,
                snd_pcm_stream_name((*data).stream),
            );
        }
        _ => {}
    }

    if !skip {
        ksft_test_result(
            pass,
            c"%s.%s.%s.%d.%d.%s\n".as_ptr(),
            *test_class_name,
            test_name,
            (*data).card_name,
            (*data).device,
            (*data).subdevice,
            snd_pcm_stream_name((*data).stream),
        );
    } else {
        ksft_test_result_skip(
            c"%s.%s.%s.%d.%d.%s\n".as_ptr(),
            *test_class_name,
            test_name,
            (*data).card_name,
            (*data).device,
            (*data).subdevice,
            snd_pcm_stream_name((*data).stream),
        );
    }

    if *msg != 0 {
        ksft_print_msg(c"%s\n".as_ptr(), msg);
    }

    pthread_mutex_unlock(&mut results_lock);

    free(samples as *mut c_void);
    if !handle.is_null() {
        snd_pcm_close(handle);
    }
}

unsafe extern "C" fn run_time_tests(pcm: *mut pcm_data, class: test_class, mut cfg: *mut snd_config_t) {
    let mut test_name: *const c_char = ptr::null();

    if cfg.is_null() {
        return;
    }

    cfg = conf_get_subtree(cfg, c"test".as_ptr(), ptr::null_mut());
    if cfg.is_null() {
        return;
    }

    snd_config_for_each(cfg, |i| unsafe {
        let pcm_cfg = snd_config_iterator_entry(i);
        if snd_config_get_id(pcm_cfg, &mut test_name) < 0 {
            ksft_exit_fail_msg(c"snd_config_get_id\n".as_ptr());
        }
        let test_type = conf_get_string(pcm_cfg, c"type".as_ptr(), ptr::null(), c"time".as_ptr());
        if strcmp(test_type, c"time".as_ptr()) == 0 {
            test_pcm_time(pcm, class, test_name, pcm_cfg);
        } else {
            ksft_exit_fail_msg(c"unknown test type '%s'\n".as_ptr(), test_type);
        }
    });
}

unsafe extern "C" fn card_thread(data: *mut c_void) -> *mut c_void {
    let card = data as *mut card_data;
    let mut pcm: *mut pcm_data;

    pcm = pcm_list;
    while !pcm.is_null() {
        if (*pcm).card != (*card).card {
            pcm = (*pcm).next;
            continue;
        }

        run_time_tests(pcm, test_class::TEST_CLASS_DEFAULT, default_pcm_config);
        run_time_tests(pcm, test_class::TEST_CLASS_SYSTEM, (*pcm).pcm_config);
        pcm = (*pcm).next;
    }

    ptr::null_mut()
}

pub unsafe fn main_0() -> c_int {
    let mut card: *mut card_data;
    let mut conf: *mut card_cfg_data;
    let mut pcm: *mut pcm_data;
    let mut global_config: *mut snd_config_t;
    let mut cfg: *mut snd_config_t;
    let mut num_pcm_tests: c_int = 0;
    let mut num_tests: c_int;
    let mut num_std_pcm_tests: c_int;
    let mut ret: c_int;
    let mut thread_ret: *mut c_void = ptr::null_mut();

    ksft_print_header();

    global_config = conf_load_from_file(c"pcm-test.conf".as_ptr());
    default_pcm_config = conf_get_subtree(global_config, c"pcm".as_ptr(), ptr::null_mut());
    if default_pcm_config.is_null() {
        ksft_exit_fail_msg(c"default pcm test configuration (pcm compound) is missing\n".as_ptr());
    }

    conf_load();

    find_pcms();

    conf = conf_cards;
    while !conf.is_null() {
        if card_cfg_card(conf) < 0 {
            num_missing += 1;
        }
        conf = card_cfg_next(conf);
    }

    num_std_pcm_tests = conf_get_count(default_pcm_config, c"test".as_ptr(), ptr::null());

    pcm = pcm_list;
    while !pcm.is_null() {
        num_pcm_tests += num_std_pcm_tests;
        cfg = (*pcm).pcm_config;
        if cfg.is_null() {
            pcm = (*pcm).next;
            continue;
        }
        /* Setting params is reported as a separate test */
        num_tests = conf_get_count(cfg, c"test".as_ptr(), ptr::null()) * 2;
        if num_tests > 0 {
            num_pcm_tests += num_tests;
        }
        pcm = (*pcm).next;
    }

    ksft_set_plan(num_missing + num_pcm_tests);

    conf = conf_cards;
    while !conf.is_null() {
        if card_cfg_card(conf) < 0 {
            ksft_test_result_fail(
                c"test.missing.%s.%s\n".as_ptr(),
                card_cfg_filename(conf),
                card_cfg_config_id(conf),
            );
        }
        conf = card_cfg_next(conf);
    }

    pcm = pcm_missing;
    while !pcm.is_null() {
        ksft_test_result(
            false,
            c"test.missing.%s.%d.%d.%s\n".as_ptr(),
            (*pcm).card_name,
            (*pcm).device,
            (*pcm).subdevice,
            snd_pcm_stream_name((*pcm).stream),
        );
        pcm = (*pcm).next;
    }

    card = card_list;
    while !card.is_null() {
        ret = pthread_create(&mut (*card).thread, ptr::null(), card_thread, card as *mut c_void);
        if ret != 0 {
            ksft_exit_fail_msg(
                c"Failed to create card %d thread: %d (%s)\n".as_ptr(),
                (*card).card,
                ret,
                strerror(errno),
            );
        }
        card = (*card).next;
    }

    card = card_list;
    while !card.is_null() {
        ret = pthread_join((*card).thread, &mut thread_ret);
        if ret != 0 {
            ksft_exit_fail_msg(
                c"Failed to join card %d thread: %d (%s)\n".as_ptr(),
                (*card).card,
                ret,
                strerror(errno),
            );
        }
        card = (*card).next;
    }

    snd_config_delete(global_config);
    conf_free();

    ksft_exit_pass();
}

fn main() {
    unsafe {
        main_0();
    }
}
