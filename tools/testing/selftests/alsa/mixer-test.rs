// SPDX-License-Identifier: GPL-2.0
//
// kselftest for the ALSA mixer API
//
// Original author: Mark Brown <broonie@kernel.org>
// Copyright (c) 2021-2 Arm Limited

// This test will iterate over all cards detected in the system, exercising
// every mixer control it can find.  This may conflict with other system
// software if there is audio activity so is best run on a system with a
// minimal active userspace.

use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_ushort, c_void};
use core::ptr;

const TESTS_PER_CONTROL: c_int = 7;
const EINVAL: c_int = 22;
const POLLIN: c_short = 0x0001;
const POLLERR: c_short = 0x0008;
const LONG_MIN: c_long = c_long::MIN;
const LONG_MAX: c_long = c_long::MAX;
const LLONG_MIN: i64 = i64::MIN;
const LLONG_MAX: i64 = i64::MAX;
const UINT_MAX: c_uint = c_uint::MAX;

const SND_CTL_EVENT_ELEM: c_int = 0;
const SND_CTL_EVENT_MASK_VALUE: c_uint = 0x1;
const SND_CTL_EVENT_MASK_REMOVE: c_uint = 0x4;
const SND_CTL_ELEM_TYPE_NONE: c_int = 0;
const SND_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
const SND_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SND_CTL_ELEM_TYPE_ENUMERATED: c_int = 3;
const SND_CTL_ELEM_TYPE_INTEGER64: c_int = 6;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct snd_ctl_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_card_info_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_list_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_id_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_info_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_event_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_config_t {
    _private: [u8; 0],
}

#[repr(C)]
struct card_data {
    handle: *mut snd_ctl_t,
    card: c_int,
    info: *mut snd_ctl_card_info_t,
    card_name: *const c_char,
    pollfd: pollfd,
    num_ctls: c_int,
    ctls: *mut snd_ctl_elem_list_t,
    next: *mut card_data,
}

#[repr(C)]
struct ctl_data {
    name: *const c_char,
    id: *mut snd_ctl_elem_id_t,
    info: *mut snd_ctl_elem_info_t,
    def_val: *mut snd_ctl_elem_value_t,
    elem: c_int,
    event_missing: c_int,
    event_spurious: c_int,
    card: *mut card_data,
    next: *mut ctl_data,
}

static mut NUM_CARDS: c_int = 0;
static mut NUM_CONTROLS: c_int = 0;
static mut CARD_LIST: *mut card_data = ptr::null_mut();
static mut CTL_LIST: *mut ctl_data = ptr::null_mut();

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;

    static mut errno: c_int;

    fn get_alsalib_config() -> *mut snd_config_t;
    fn snd_config_delete(config: *mut snd_config_t) -> c_int;
    fn snd_card_next(card: *mut c_int) -> c_int;
    fn snd_ctl_open_lconf(
        ctl: *mut *mut snd_ctl_t,
        name: *const c_char,
        mode: c_int,
        lconf: *mut snd_config_t,
    ) -> c_int;
    fn snd_strerror(errnum: c_int) -> *const c_char;
    fn snd_card_get_name(card: c_int, name: *mut *mut c_char) -> c_int;
    fn snd_card_get_longname(card: c_int, name: *mut *mut c_char) -> c_int;
    fn snd_ctl_card_info_malloc(ptr: *mut *mut snd_ctl_card_info_t) -> c_int;
    fn snd_ctl_card_info(ctl: *mut snd_ctl_t, info: *mut snd_ctl_card_info_t) -> c_int;
    fn snd_ctl_card_info_get_id(obj: *const snd_ctl_card_info_t) -> *const c_char;
    fn snd_ctl_elem_list_malloc(ptr: *mut *mut snd_ctl_elem_list_t) -> c_int;
    fn snd_ctl_elem_list(ctl: *mut snd_ctl_t, list: *mut snd_ctl_elem_list_t) -> c_int;
    fn snd_ctl_elem_list_get_count(obj: *const snd_ctl_elem_list_t) -> c_uint;
    fn snd_ctl_elem_list_alloc_space(obj: *mut snd_ctl_elem_list_t, entries: c_uint) -> c_int;
    fn snd_ctl_elem_list_get_name(obj: *const snd_ctl_elem_list_t, idx: c_uint) -> *const c_char;
    fn snd_ctl_elem_id_malloc(ptr: *mut *mut snd_ctl_elem_id_t) -> c_int;
    fn snd_ctl_elem_info_malloc(ptr: *mut *mut snd_ctl_elem_info_t) -> c_int;
    fn snd_ctl_elem_value_malloc(ptr: *mut *mut snd_ctl_elem_value_t) -> c_int;
    fn snd_ctl_elem_list_get_id(
        obj: *const snd_ctl_elem_list_t,
        idx: c_uint,
        ptr: *mut snd_ctl_elem_id_t,
    );
    fn snd_ctl_elem_info_set_id(obj: *mut snd_ctl_elem_info_t, ptr: *const snd_ctl_elem_id_t);
    fn snd_ctl_elem_info(ctl: *mut snd_ctl_t, info: *mut snd_ctl_elem_info_t) -> c_int;
    fn snd_ctl_elem_value_set_id(obj: *mut snd_ctl_elem_value_t, ptr: *const snd_ctl_elem_id_t);
    fn snd_ctl_subscribe_events(ctl: *mut snd_ctl_t, subscribe: c_int) -> c_int;
    fn snd_ctl_poll_descriptors_count(ctl: *mut snd_ctl_t) -> c_int;
    fn snd_ctl_poll_descriptors(ctl: *mut snd_ctl_t, pfds: *mut pollfd, space: c_uint) -> c_int;
    fn snd_ctl_poll_descriptors_revents(
        ctl: *mut snd_ctl_t,
        pfds: *mut pollfd,
        nfds: c_uint,
        revents: *mut c_ushort,
    ) -> c_int;
    fn snd_ctl_event_malloc(ptr: *mut *mut snd_ctl_event_t) -> c_int;
    fn snd_ctl_read(ctl: *mut snd_ctl_t, event: *mut snd_ctl_event_t) -> c_int;
    fn snd_ctl_event_get_type(obj: *const snd_ctl_event_t) -> c_int;
    fn snd_ctl_event_elem_get_mask(obj: *const snd_ctl_event_t) -> c_uint;
    fn snd_ctl_event_elem_get_numid(obj: *const snd_ctl_event_t) -> c_uint;
    fn snd_ctl_event_elem_get_name(obj: *const snd_ctl_event_t) -> *const c_char;
    fn snd_ctl_elem_info_get_numid(obj: *const snd_ctl_elem_info_t) -> c_uint;
    fn snd_ctl_elem_info_get_type(obj: *const snd_ctl_elem_info_t) -> c_int;
    fn snd_ctl_elem_value_get_boolean(obj: *const snd_ctl_elem_value_t, idx: c_uint) -> c_int;
    fn snd_ctl_elem_value_get_integer(obj: *const snd_ctl_elem_value_t, idx: c_uint) -> c_long;
    fn snd_ctl_elem_value_get_integer64(obj: *const snd_ctl_elem_value_t, idx: c_uint) -> i64;
    fn snd_ctl_elem_value_get_enumerated(obj: *const snd_ctl_elem_value_t, idx: c_uint) -> c_uint;
    fn snd_ctl_elem_info_get_min(obj: *const snd_ctl_elem_info_t) -> c_long;
    fn snd_ctl_elem_info_get_max(obj: *const snd_ctl_elem_info_t) -> c_long;
    fn snd_ctl_elem_info_get_step(obj: *const snd_ctl_elem_info_t) -> c_long;
    fn snd_ctl_elem_info_get_min64(obj: *const snd_ctl_elem_info_t) -> i64;
    fn snd_ctl_elem_info_get_max64(obj: *const snd_ctl_elem_info_t) -> i64;
    fn snd_ctl_elem_info_get_step64(obj: *const snd_ctl_elem_info_t) -> i64;
    fn snd_ctl_elem_info_get_items(obj: *const snd_ctl_elem_info_t) -> c_uint;
    fn snd_ctl_elem_info_get_count(obj: *const snd_ctl_elem_info_t) -> c_uint;
    fn snd_ctl_elem_info_is_inactive(obj: *const snd_ctl_elem_info_t) -> c_int;
    fn snd_ctl_elem_info_is_readable(obj: *const snd_ctl_elem_info_t) -> c_int;
    fn snd_ctl_elem_info_is_writable(obj: *const snd_ctl_elem_info_t) -> c_int;
    fn snd_ctl_elem_info_is_volatile(obj: *const snd_ctl_elem_info_t) -> c_int;
    fn snd_ctl_elem_read(ctl: *mut snd_ctl_t, value: *mut snd_ctl_elem_value_t) -> c_int;
    fn snd_ctl_elem_write(ctl: *mut snd_ctl_t, value: *mut snd_ctl_elem_value_t) -> c_int;
    fn snd_ctl_elem_value_copy(dst: *mut snd_ctl_elem_value_t, src: *const snd_ctl_elem_value_t);
    fn snd_ctl_elem_value_compare(
        obj1: *const snd_ctl_elem_value_t,
        obj2: *const snd_ctl_elem_value_t,
    ) -> c_int;
    fn snd_ctl_elem_value_set_boolean(obj: *mut snd_ctl_elem_value_t, idx: c_uint, val: c_long);
    fn snd_ctl_elem_value_set_integer(obj: *mut snd_ctl_elem_value_t, idx: c_uint, val: c_long);
    fn snd_ctl_elem_value_set_integer64(obj: *mut snd_ctl_elem_value_t, idx: c_uint, val: i64);
    fn snd_ctl_elem_value_set_enumerated(obj: *mut snd_ctl_elem_value_t, idx: c_uint, val: c_uint);

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit_pass() -> !;
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_test_result(pass: bool, msg: *const c_char, ...);
    fn ksft_test_result_skip(msg: *const c_char, ...);
}

unsafe fn snd_ctl_event_alloca(event: *mut *mut snd_ctl_event_t) {
    if snd_ctl_event_malloc(event) < 0 {
        ksft_exit_fail_msg(cstr!("Out of memory\n"));
    }
}

unsafe fn snd_ctl_elem_value_alloca(value: *mut *mut snd_ctl_elem_value_t) {
    if snd_ctl_elem_value_malloc(value) < 0 {
        ksft_exit_fail_msg(cstr!("Out of memory\n"));
    }
}

unsafe fn find_controls() {
    let mut name: [c_char; 32] = [0; 32];
    let mut card: c_int;
    let mut ctl: c_int;
    let mut err: c_int;
    let mut card_data_ptr: *mut card_data;
    let mut ctl_data_ptr: *mut ctl_data;
    let config: *mut snd_config_t;
    let mut card_name: *mut c_char = ptr::null_mut();
    let mut card_longname: *mut c_char = ptr::null_mut();

    card = -1;
    if snd_card_next(&mut card) < 0 || card < 0 {
        return;
    }

    config = get_alsalib_config();

    while card >= 0 {
        sprintf(name.as_mut_ptr(), cstr!("hw:%d"), card);

        card_data_ptr = malloc(core::mem::size_of::<card_data>()) as *mut card_data;
        if card_data_ptr.is_null() {
            ksft_exit_fail_msg(cstr!("Out of memory\n"));
        }

        err = snd_ctl_open_lconf(&mut (*card_data_ptr).handle, name.as_ptr(), 0, config);
        if err < 0 {
            ksft_print_msg(
                cstr!("Failed to get hctl for card %d: %s\n"),
                card,
                snd_strerror(err),
            );
            free(card_data_ptr as *mut c_void);
            if snd_card_next(&mut card) < 0 {
                ksft_print_msg(cstr!("snd_card_next"));
                break;
            }
            continue;
        }

        err = snd_card_get_name(card, &mut card_name);
        if err != 0 {
            card_name = cstr!("Unknown") as *mut c_char;
        }
        err = snd_card_get_longname(card, &mut card_longname);
        if err != 0 {
            card_longname = cstr!("Unknown") as *mut c_char;
        }

        err = snd_ctl_card_info_malloc(&mut (*card_data_ptr).info);
        if err != 0 {
            ksft_exit_fail_msg(cstr!("Failed to allocate card info: %d\n"), err);
        }

        err = snd_ctl_card_info((*card_data_ptr).handle, (*card_data_ptr).info);
        if err == 0 {
            (*card_data_ptr).card_name = snd_ctl_card_info_get_id((*card_data_ptr).info);
            if (*card_data_ptr).card_name.is_null() {
                ksft_print_msg(cstr!("Failed to get card ID\n"));
            }
        } else {
            ksft_print_msg(cstr!("Failed to get card info: %d\n"), err);
        }

        if (*card_data_ptr).card_name.is_null() {
            (*card_data_ptr).card_name = cstr!("Unknown");
        }

        ksft_print_msg(
            cstr!("Card %d/%s - %s (%s)\n"),
            card,
            (*card_data_ptr).card_name,
            card_name,
            card_longname,
        );

        /* Count controls */
        snd_ctl_elem_list_malloc(&mut (*card_data_ptr).ctls);
        snd_ctl_elem_list((*card_data_ptr).handle, (*card_data_ptr).ctls);
        (*card_data_ptr).num_ctls = snd_ctl_elem_list_get_count((*card_data_ptr).ctls) as c_int;

        /* Enumerate control information */
        snd_ctl_elem_list_alloc_space((*card_data_ptr).ctls, (*card_data_ptr).num_ctls as c_uint);
        snd_ctl_elem_list((*card_data_ptr).handle, (*card_data_ptr).ctls);

        (*card_data_ptr).card = NUM_CARDS;
        NUM_CARDS += 1;
        (*card_data_ptr).next = CARD_LIST;
        CARD_LIST = card_data_ptr;

        NUM_CONTROLS += (*card_data_ptr).num_ctls;

        ctl = 0;
        while ctl < (*card_data_ptr).num_ctls {
            ctl_data_ptr = malloc(core::mem::size_of::<ctl_data>()) as *mut ctl_data;
            if ctl_data_ptr.is_null() {
                ksft_exit_fail_msg(cstr!("Out of memory\n"));
            }

            memset(
                ctl_data_ptr as *mut c_void,
                0,
                core::mem::size_of::<ctl_data>(),
            );
            (*ctl_data_ptr).card = card_data_ptr;
            (*ctl_data_ptr).elem = ctl;
            (*ctl_data_ptr).name =
                snd_ctl_elem_list_get_name((*card_data_ptr).ctls, ctl as c_uint);

            err = snd_ctl_elem_id_malloc(&mut (*ctl_data_ptr).id);
            if err < 0 {
                ksft_exit_fail_msg(cstr!("Out of memory\n"));
            }

            err = snd_ctl_elem_info_malloc(&mut (*ctl_data_ptr).info);
            if err < 0 {
                ksft_exit_fail_msg(cstr!("Out of memory\n"));
            }

            err = snd_ctl_elem_value_malloc(&mut (*ctl_data_ptr).def_val);
            if err < 0 {
                ksft_exit_fail_msg(cstr!("Out of memory\n"));
            }

            snd_ctl_elem_list_get_id((*card_data_ptr).ctls, ctl as c_uint, (*ctl_data_ptr).id);
            snd_ctl_elem_info_set_id((*ctl_data_ptr).info, (*ctl_data_ptr).id);
            err = snd_ctl_elem_info((*card_data_ptr).handle, (*ctl_data_ptr).info);
            if err < 0 {
                ksft_print_msg(
                    cstr!("%s getting info for %s\n"),
                    snd_strerror(err),
                    (*ctl_data_ptr).name,
                );
            }

            snd_ctl_elem_value_set_id((*ctl_data_ptr).def_val, (*ctl_data_ptr).id);

            (*ctl_data_ptr).next = CTL_LIST;
            CTL_LIST = ctl_data_ptr;
            ctl += 1;
        }

        /* Set up for events */
        err = snd_ctl_subscribe_events((*card_data_ptr).handle, true as c_int);
        if err < 0 {
            ksft_exit_fail_msg(
                cstr!("snd_ctl_subscribe_events() failed for card %d: %d\n"),
                card,
                err,
            );
        }

        err = snd_ctl_poll_descriptors_count((*card_data_ptr).handle);
        if err != 1 {
            ksft_exit_fail_msg(
                cstr!("Unexpected descriptor count %d for card %d\n"),
                err,
                card,
            );
        }

        err = snd_ctl_poll_descriptors(
            (*card_data_ptr).handle,
            &mut (*card_data_ptr).pollfd,
            1,
        );
        if err != 1 {
            ksft_exit_fail_msg(
                cstr!("snd_ctl_poll_descriptors() failed for card %d: %d\n"),
                card,
                err,
            );
        }

        if snd_card_next(&mut card) < 0 {
            ksft_print_msg(cstr!("snd_card_next"));
            break;
        }
    }

    snd_config_delete(config);
}

/*
 * Block for up to timeout ms for an event, returns a negative value
 * on error, 0 for no event and 1 for an event.
 */
unsafe fn wait_for_event(ctl: *mut ctl_data, timeout: c_int) -> c_int {
    let mut revents: c_ushort = 0;
    let mut event: *mut snd_ctl_event_t = ptr::null_mut();
    let mut err: c_int;
    let mut mask: c_uint = 0;
    let mut ev_id: c_uint;

    snd_ctl_event_alloca(&mut event);

    loop {
        err = poll(&mut (*(*ctl).card).pollfd, 1, timeout);
        if err < 0 {
            ksft_print_msg(
                cstr!("poll() failed for %s: %s (%d)\n"),
                (*ctl).name,
                strerror(errno),
                errno,
            );
            return -1;
        }
        /* Timeout */
        if err == 0 {
            return 0;
        }

        err = snd_ctl_poll_descriptors_revents(
            (*(*ctl).card).handle,
            &mut (*(*ctl).card).pollfd,
            1,
            &mut revents,
        );
        if err < 0 {
            ksft_print_msg(
                cstr!("snd_ctl_poll_descriptors_revents() failed for %s: %d\n"),
                (*ctl).name,
                err,
            );
            return err;
        }
        if (revents & POLLERR as c_ushort) != 0 {
            ksft_print_msg(
                cstr!("snd_ctl_poll_descriptors_revents() reported POLLERR for %s\n"),
                (*ctl).name,
            );
            return -1;
        }
        /* No read events */
        if (revents & POLLIN as c_ushort) == 0 {
            ksft_print_msg(cstr!("No POLLIN\n"));
            continue;
        }

        err = snd_ctl_read((*(*ctl).card).handle, event);
        if err < 0 {
            ksft_print_msg(cstr!("snd_ctl_read() failed for %s: %d\n"), (*ctl).name, err);
            return err;
        }

        if snd_ctl_event_get_type(event) != SND_CTL_EVENT_ELEM {
            continue;
        }

        /* The ID returned from the event is 1 less than numid */
        mask = snd_ctl_event_elem_get_mask(event);
        ev_id = snd_ctl_event_elem_get_numid(event);
        if ev_id != snd_ctl_elem_info_get_numid((*ctl).info) {
            ksft_print_msg(
                cstr!("Event for unexpected ctl %s\n"),
                snd_ctl_event_elem_get_name(event),
            );
            continue;
        }

        if (mask & SND_CTL_EVENT_MASK_REMOVE) == SND_CTL_EVENT_MASK_REMOVE {
            ksft_print_msg(cstr!("Removal event for %s\n"), (*ctl).name);
            return -1;
        }

        if (mask & SND_CTL_EVENT_MASK_VALUE) == SND_CTL_EVENT_MASK_VALUE {
            break;
        }
    }

    1
}

unsafe fn ctl_value_index_valid(
    ctl: *mut ctl_data,
    val: *mut snd_ctl_elem_value_t,
    index: c_int,
) -> bool {
    let mut int_val: c_long;
    let mut int64_val: i64;

    match snd_ctl_elem_info_get_type((*ctl).info) {
        SND_CTL_ELEM_TYPE_NONE => {
            ksft_print_msg(cstr!("%s.%d Invalid control type NONE\n"), (*ctl).name, index);
            false
        }
        SND_CTL_ELEM_TYPE_BOOLEAN => {
            int_val = snd_ctl_elem_value_get_boolean(val, index as c_uint) as c_long;
            match int_val {
                0 | 1 => true,
                _ => {
                    ksft_print_msg(
                        cstr!("%s.%d Invalid boolean value %ld\n"),
                        (*ctl).name,
                        index,
                        int_val,
                    );
                    false
                }
            }
        }
        SND_CTL_ELEM_TYPE_INTEGER => {
            int_val = snd_ctl_elem_value_get_integer(val, index as c_uint);

            if int_val < snd_ctl_elem_info_get_min((*ctl).info) {
                ksft_print_msg(
                    cstr!("%s.%d value %ld less than minimum %ld\n"),
                    (*ctl).name,
                    index,
                    int_val,
                    snd_ctl_elem_info_get_min((*ctl).info),
                );
                return false;
            }

            if int_val > snd_ctl_elem_info_get_max((*ctl).info) {
                ksft_print_msg(
                    cstr!("%s.%d value %ld more than maximum %ld\n"),
                    (*ctl).name,
                    index,
                    int_val,
                    snd_ctl_elem_info_get_max((*ctl).info),
                );
                return false;
            }

            /* Only check step size if there is one and we're in bounds */
            if snd_ctl_elem_info_get_step((*ctl).info) != 0
                && (int_val - snd_ctl_elem_info_get_min((*ctl).info)
                    % snd_ctl_elem_info_get_step((*ctl).info))
                    != 0
            {
                ksft_print_msg(
                    cstr!("%s.%d value %ld invalid for step %ld minimum %ld\n"),
                    (*ctl).name,
                    index,
                    int_val,
                    snd_ctl_elem_info_get_step((*ctl).info),
                    snd_ctl_elem_info_get_min((*ctl).info),
                );
                return false;
            }
            true
        }
        SND_CTL_ELEM_TYPE_INTEGER64 => {
            int64_val = snd_ctl_elem_value_get_integer64(val, index as c_uint);

            if int64_val < snd_ctl_elem_info_get_min64((*ctl).info) {
                ksft_print_msg(
                    cstr!("%s.%d value %lld less than minimum %lld\n"),
                    (*ctl).name,
                    index,
                    int64_val,
                    snd_ctl_elem_info_get_min64((*ctl).info),
                );
                return false;
            }

            if int64_val > snd_ctl_elem_info_get_max64((*ctl).info) {
                ksft_print_msg(
                    cstr!("%s.%d value %lld more than maximum %lld\n"),
                    (*ctl).name,
                    index,
                    int64_val,
                    snd_ctl_elem_info_get_max64((*ctl).info),
                );
                return false;
            }

            /* Only check step size if there is one and we're in bounds */
            if snd_ctl_elem_info_get_step64((*ctl).info) != 0
                && (int64_val - snd_ctl_elem_info_get_min64((*ctl).info))
                    % snd_ctl_elem_info_get_step64((*ctl).info)
                    != 0
            {
                ksft_print_msg(
                    cstr!("%s.%d value %lld invalid for step %lld minimum %lld\n"),
                    (*ctl).name,
                    index,
                    int64_val,
                    snd_ctl_elem_info_get_step64((*ctl).info),
                    snd_ctl_elem_info_get_min64((*ctl).info),
                );
                return false;
            }
            true
        }
        SND_CTL_ELEM_TYPE_ENUMERATED => {
            int_val = snd_ctl_elem_value_get_enumerated(val, index as c_uint) as c_long;

            if int_val < 0 {
                ksft_print_msg(
                    cstr!("%s.%d negative value %ld for enumeration\n"),
                    (*ctl).name,
                    index,
                    int_val,
                );
                return false;
            }

            if int_val >= snd_ctl_elem_info_get_items((*ctl).info) as c_long {
                ksft_print_msg(
                    cstr!("%s.%d value %ld more than item count %u\n"),
                    (*ctl).name,
                    index,
                    int_val,
                    snd_ctl_elem_info_get_items((*ctl).info),
                );
                return false;
            }
            true
        }
        _ => {
            /* No tests for other types */
            true
        }
    }
}

/*
 * Check that the provided value meets the constraints for the
 * provided control.
 */
unsafe fn ctl_value_valid(ctl: *mut ctl_data, val: *mut snd_ctl_elem_value_t) -> bool {
    let mut i: c_int;
    let mut valid = true;

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        if !ctl_value_index_valid(ctl, val, i) {
            valid = false;
        }
        i += 1;
    }

    valid
}

/*
 * Check that we can read the default value and it is valid. Write
 * tests use the read value to restore the default.
 */
unsafe fn test_ctl_get_value(ctl: *mut ctl_data) {
    let mut err: c_int;

    /* If the control is turned off let's be polite */
    if snd_ctl_elem_info_is_inactive((*ctl).info) != 0 {
        ksft_print_msg(cstr!("%s is inactive\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("get_value.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    /* Can't test reading on an unreadable control */
    if snd_ctl_elem_info_is_readable((*ctl).info) == 0 {
        ksft_print_msg(cstr!("%s is not readable\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("get_value.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    err = snd_ctl_elem_read((*(*ctl).card).handle, (*ctl).def_val);
    if err < 0 {
        ksft_print_msg(cstr!("snd_ctl_elem_read() failed: %s\n"), snd_strerror(err));
    } else if !ctl_value_valid(ctl, (*ctl).def_val) {
        err = -EINVAL;
    }

    ksft_test_result(err >= 0, cstr!("get_value.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
}

unsafe fn strend(haystack: *const c_char, needle: *const c_char) -> bool {
    let haystack_len = strlen(haystack);
    let needle_len = strlen(needle);

    if needle_len > haystack_len {
        return false;
    }
    strcmp(haystack.add(haystack_len - needle_len), needle) == 0
}

unsafe fn test_ctl_name(ctl: *mut ctl_data) {
    let mut name_ok = true;

    ksft_print_msg(cstr!("%s.%d %s\n"), (*(*ctl).card).card_name, (*ctl).elem, (*ctl).name);

    /* Only boolean controls should end in Switch */
    if strend((*ctl).name, cstr!(" Switch")) {
        if snd_ctl_elem_info_get_type((*ctl).info) != SND_CTL_ELEM_TYPE_BOOLEAN {
            ksft_print_msg(
                cstr!("%d.%d %s ends in Switch but is not boolean\n"),
                (*(*ctl).card).card,
                (*ctl).elem,
                (*ctl).name,
            );
            name_ok = false;
        }
    }

    /* Writeable boolean controls should end in Switch */
    if snd_ctl_elem_info_get_type((*ctl).info) == SND_CTL_ELEM_TYPE_BOOLEAN
        && snd_ctl_elem_info_is_writable((*ctl).info) != 0
    {
        if !strend((*ctl).name, cstr!(" Switch")) {
            ksft_print_msg(
                cstr!("%d.%d %s is a writeable boolean but not a Switch\n"),
                (*(*ctl).card).card,
                (*ctl).elem,
                (*ctl).name,
            );
            name_ok = false;
        }
    }

    ksft_test_result(name_ok, cstr!("name.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
}

unsafe fn show_values(
    ctl: *mut ctl_data,
    orig_val: *mut snd_ctl_elem_value_t,
    read_val: *mut snd_ctl_elem_value_t,
) {
    let mut orig_int: i64;
    let mut read_int: i64;
    let mut i: c_int = 0;

    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        match snd_ctl_elem_info_get_type((*ctl).info) {
            SND_CTL_ELEM_TYPE_BOOLEAN => {
                orig_int = snd_ctl_elem_value_get_boolean(orig_val, i as c_uint) as i64;
                read_int = snd_ctl_elem_value_get_boolean(read_val, i as c_uint) as i64;
            }
            SND_CTL_ELEM_TYPE_INTEGER => {
                orig_int = snd_ctl_elem_value_get_integer(orig_val, i as c_uint) as i64;
                read_int = snd_ctl_elem_value_get_integer(read_val, i as c_uint) as i64;
            }
            SND_CTL_ELEM_TYPE_INTEGER64 => {
                orig_int = snd_ctl_elem_value_get_integer64(orig_val, i as c_uint);
                read_int = snd_ctl_elem_value_get_integer64(read_val, i as c_uint);
            }
            SND_CTL_ELEM_TYPE_ENUMERATED => {
                orig_int = snd_ctl_elem_value_get_enumerated(orig_val, i as c_uint) as i64;
                read_int = snd_ctl_elem_value_get_enumerated(read_val, i as c_uint) as i64;
            }
            _ => return,
        }

        ksft_print_msg(
            cstr!("%s.%d orig %lld read %lld, is_volatile %d\n"),
            (*ctl).name,
            i,
            orig_int,
            read_int,
            snd_ctl_elem_info_is_volatile((*ctl).info),
        );
        i += 1;
    }
}

unsafe fn show_mismatch(
    ctl: *mut ctl_data,
    index: c_int,
    read_val: *mut snd_ctl_elem_value_t,
    expected_val: *mut snd_ctl_elem_value_t,
) -> bool {
    let mut expected_int: i64;
    let mut read_int: i64;

    /*
     * We factor out the code to compare values representable as
     * integers, ensure that check doesn't log otherwise.
     */
    expected_int = 0;
    read_int = 0;

    match snd_ctl_elem_info_get_type((*ctl).info) {
        SND_CTL_ELEM_TYPE_BOOLEAN => {
            expected_int = snd_ctl_elem_value_get_boolean(expected_val, index as c_uint) as i64;
            read_int = snd_ctl_elem_value_get_boolean(read_val, index as c_uint) as i64;
        }
        SND_CTL_ELEM_TYPE_INTEGER => {
            expected_int = snd_ctl_elem_value_get_integer(expected_val, index as c_uint) as i64;
            read_int = snd_ctl_elem_value_get_integer(read_val, index as c_uint) as i64;
        }
        SND_CTL_ELEM_TYPE_INTEGER64 => {
            expected_int = snd_ctl_elem_value_get_integer64(expected_val, index as c_uint);
            read_int = snd_ctl_elem_value_get_integer64(read_val, index as c_uint);
        }
        SND_CTL_ELEM_TYPE_ENUMERATED => {
            expected_int = snd_ctl_elem_value_get_enumerated(expected_val, index as c_uint) as i64;
            read_int = snd_ctl_elem_value_get_enumerated(read_val, index as c_uint) as i64;
        }
        _ => {}
    }

    if expected_int != read_int {
        /*
         * NOTE: The volatile attribute means that the hardware
         * can voluntarily change the state of control element
         * independent of any operation by software.
         */
        let is_volatile = snd_ctl_elem_info_is_volatile((*ctl).info);
        ksft_print_msg(
            cstr!("%s.%d expected %lld but read %lld, is_volatile %d\n"),
            (*ctl).name,
            index,
            expected_int,
            read_int,
            is_volatile,
        );
        is_volatile == 0
    } else {
        false
    }
}

/*
 * Write a value then if possible verify that we get the expected
 * result.  An optional expected value can be provided if we expect
 * the write to fail, for verifying that invalid writes don't corrupt
 * anything.
 */
unsafe fn write_and_verify(
    ctl: *mut ctl_data,
    write_val: *mut snd_ctl_elem_value_t,
    mut expected_val: *mut snd_ctl_elem_value_t,
) -> c_int {
    let mut err: c_int;
    let mut i: c_int;
    let error_expected: bool;
    let mut mismatch_shown: bool;
    let mut initial_val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    let mut read_val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    let mut w_val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut initial_val);
    snd_ctl_elem_value_alloca(&mut read_val);
    snd_ctl_elem_value_alloca(&mut w_val);

    /*
     * We need to copy the write value since writing can modify
     * the value which causes surprises, and allocate an expected
     * value if we expect to read back what we wrote.
     */
    snd_ctl_elem_value_copy(w_val, write_val);
    if !expected_val.is_null() {
        error_expected = true;
    } else {
        error_expected = false;
        snd_ctl_elem_value_alloca(&mut expected_val);
        snd_ctl_elem_value_copy(expected_val, write_val);
    }

    /* Store the value before we write */
    if snd_ctl_elem_info_is_readable((*ctl).info) != 0 {
        snd_ctl_elem_value_set_id(initial_val, (*ctl).id);

        err = snd_ctl_elem_read((*(*ctl).card).handle, initial_val);
        if err < 0 {
            ksft_print_msg(cstr!("snd_ctl_elem_read() failed: %s\n"), snd_strerror(err));
            return err;
        }
    }

    /*
     * Do the write, if we have an expected value ignore the error
     * and carry on to validate the expected value.
     */
    err = snd_ctl_elem_write((*(*ctl).card).handle, w_val);
    if err < 0 && !error_expected {
        ksft_print_msg(cstr!("snd_ctl_elem_write() failed: %s\n"), snd_strerror(err));
        return err;
    }

    /* Can we do the verification part? */
    if snd_ctl_elem_info_is_readable((*ctl).info) == 0 {
        return err;
    }

    snd_ctl_elem_value_set_id(read_val, (*ctl).id);

    err = snd_ctl_elem_read((*(*ctl).card).handle, read_val);
    if err < 0 {
        ksft_print_msg(cstr!("snd_ctl_elem_read() failed: %s\n"), snd_strerror(err));
        return err;
    }

    /*
     * We can't verify any specific value for volatile controls
     * but we should still check that whatever we read is a valid
     * vale for the control.
     */
    if snd_ctl_elem_info_is_volatile((*ctl).info) != 0 {
        if !ctl_value_valid(ctl, read_val) {
            ksft_print_msg(cstr!("Volatile control %s has invalid value\n"), (*ctl).name);
            return -EINVAL;
        }

        return 0;
    }

    /*
     * Check for an event if the value changed, or confirm that
     * there was none if it didn't.  We rely on the kernel
     * generating the notification before it returns from the
     * write, this is currently true, should that ever change this
     * will most likely break and need updating.
     */
    err = wait_for_event(ctl, 0);
    if snd_ctl_elem_value_compare(initial_val, read_val) != 0 {
        if err < 1 {
            ksft_print_msg(cstr!("No event generated for %s\n"), (*ctl).name);
            show_values(ctl, initial_val, read_val);
            (*ctl).event_missing += 1;
        }
    } else if err != 0 {
        ksft_print_msg(cstr!("Spurious event generated for %s\n"), (*ctl).name);
        show_values(ctl, initial_val, read_val);
        (*ctl).event_spurious += 1;
    }

    /*
     * Use the libray to compare values, if there's a mismatch
     * carry on and try to provide a more useful diagnostic than
     * just "mismatch".
     */
    if snd_ctl_elem_value_compare(expected_val, read_val) == 0 {
        return 0;
    }

    mismatch_shown = false;
    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        if show_mismatch(ctl, i, read_val, expected_val) {
            mismatch_shown = true;
        }
        i += 1;
    }

    if !mismatch_shown {
        ksft_print_msg(cstr!("%s read and written values differ\n"), (*ctl).name);
    }

    -1
}

/*
 * Make sure we can write the default value back to the control, this
 * should validate that at least some write works.
 */
unsafe fn test_ctl_write_default(ctl: *mut ctl_data) {
    let err: c_int;

    /* If the control is turned off let's be polite */
    if snd_ctl_elem_info_is_inactive((*ctl).info) != 0 {
        ksft_print_msg(cstr!("%s is inactive\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("write_default.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    if snd_ctl_elem_info_is_writable((*ctl).info) == 0 {
        ksft_print_msg(cstr!("%s is not writeable\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("write_default.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    /* No idea what the default was for unreadable controls */
    if snd_ctl_elem_info_is_readable((*ctl).info) == 0 {
        ksft_print_msg(cstr!("%s couldn't read default\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("write_default.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    err = write_and_verify(ctl, (*ctl).def_val, ptr::null_mut());

    ksft_test_result(err >= 0, cstr!("write_default.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
}

unsafe fn test_ctl_write_valid_boolean(ctl: *mut ctl_data) -> bool {
    let mut err: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut fail = false;
    let mut val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut val);

    snd_ctl_elem_value_set_id(val, (*ctl).id);

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        j = 0;
        while j < 2 {
            snd_ctl_elem_value_set_boolean(val, i as c_uint, j as c_long);
            err = write_and_verify(ctl, val, ptr::null_mut());
            if err != 0 {
                fail = true;
            }
            j += 1;
        }
        i += 1;
    }

    !fail
}

unsafe fn test_ctl_write_valid_integer(ctl: *mut ctl_data) -> bool {
    let mut err: c_int;
    let mut i: c_int;
    let mut j: c_long;
    let mut step: c_long;
    let mut fail = false;
    let mut val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut val);

    snd_ctl_elem_value_set_id(val, (*ctl).id);

    step = snd_ctl_elem_info_get_step((*ctl).info);
    if step == 0 {
        step = 1;
    }

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        j = snd_ctl_elem_info_get_min((*ctl).info);
        while j <= snd_ctl_elem_info_get_max((*ctl).info) {
            snd_ctl_elem_value_set_integer(val, i as c_uint, j);
            err = write_and_verify(ctl, val, ptr::null_mut());
            if err != 0 {
                fail = true;
            }
            j += step;
        }
        i += 1;
    }

    !fail
}

unsafe fn test_ctl_write_valid_integer64(ctl: *mut ctl_data) -> bool {
    let mut err: c_int;
    let mut i: c_int;
    let mut j: i64;
    let mut step: i64;
    let mut fail = false;
    let mut val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut val);

    snd_ctl_elem_value_set_id(val, (*ctl).id);

    step = snd_ctl_elem_info_get_step64((*ctl).info);
    if step == 0 {
        step = 1;
    }

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        j = snd_ctl_elem_info_get_min64((*ctl).info);
        while j <= snd_ctl_elem_info_get_max64((*ctl).info) {
            snd_ctl_elem_value_set_integer64(val, i as c_uint, j);
            err = write_and_verify(ctl, val, ptr::null_mut());
            if err != 0 {
                fail = true;
            }
            j += step;
        }
        i += 1;
    }

    !fail
}

unsafe fn test_ctl_write_valid_enumerated(ctl: *mut ctl_data) -> bool {
    let mut err: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut fail = false;
    let mut val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut val);

    snd_ctl_elem_value_set_id(val, (*ctl).id);

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        j = 0;
        while j < snd_ctl_elem_info_get_items((*ctl).info) as c_int {
            snd_ctl_elem_value_set_enumerated(val, i as c_uint, j as c_uint);
            err = write_and_verify(ctl, val, ptr::null_mut());
            if err != 0 {
                fail = true;
            }
            j += 1;
        }
        i += 1;
    }

    !fail
}

unsafe fn test_ctl_write_valid(ctl: *mut ctl_data) {
    let pass: bool;

    /* If the control is turned off let's be polite */
    if snd_ctl_elem_info_is_inactive((*ctl).info) != 0 {
        ksft_print_msg(cstr!("%s is inactive\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("write_valid.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    if snd_ctl_elem_info_is_writable((*ctl).info) == 0 {
        ksft_print_msg(cstr!("%s is not writeable\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("write_valid.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    match snd_ctl_elem_info_get_type((*ctl).info) {
        SND_CTL_ELEM_TYPE_BOOLEAN => pass = test_ctl_write_valid_boolean(ctl),
        SND_CTL_ELEM_TYPE_INTEGER => pass = test_ctl_write_valid_integer(ctl),
        SND_CTL_ELEM_TYPE_INTEGER64 => pass = test_ctl_write_valid_integer64(ctl),
        SND_CTL_ELEM_TYPE_ENUMERATED => pass = test_ctl_write_valid_enumerated(ctl),
        _ => {
            /* No tests for this yet */
            ksft_test_result_skip(cstr!("write_valid.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
            return;
        }
    }

    /* Restore the default value to minimise disruption */
    write_and_verify(ctl, (*ctl).def_val, ptr::null_mut());

    ksft_test_result(pass, cstr!("write_valid.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
}

unsafe fn test_ctl_write_invalid_value(
    ctl: *mut ctl_data,
    val: *mut snd_ctl_elem_value_t,
) -> bool {
    let mut err: c_int;

    /* Ideally this will fail... */
    err = snd_ctl_elem_write((*(*ctl).card).handle, val);
    if err < 0 {
        return false;
    }

    /* ...but some devices will clamp to an in range value */
    err = snd_ctl_elem_read((*(*ctl).card).handle, val);
    if err < 0 {
        ksft_print_msg(cstr!("%s failed to read: %s\n"), (*ctl).name, snd_strerror(err));
        return true;
    }

    !ctl_value_valid(ctl, val)
}

unsafe fn test_ctl_write_invalid_boolean(ctl: *mut ctl_data) -> bool {
    let mut i: c_int;
    let mut fail = false;
    let mut val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut val);

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        snd_ctl_elem_value_copy(val, (*ctl).def_val);
        snd_ctl_elem_value_set_boolean(val, i as c_uint, 2);

        if test_ctl_write_invalid_value(ctl, val) {
            fail = true;
        }
        i += 1;
    }

    !fail
}

unsafe fn test_ctl_write_invalid_integer(ctl: *mut ctl_data) -> bool {
    let mut i: c_int;
    let mut fail = false;
    let mut val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut val);

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        if snd_ctl_elem_info_get_min((*ctl).info) != LONG_MIN {
            /* Just under range */
            snd_ctl_elem_value_copy(val, (*ctl).def_val);
            snd_ctl_elem_value_set_integer(
                val,
                i as c_uint,
                snd_ctl_elem_info_get_min((*ctl).info) - 1,
            );

            if test_ctl_write_invalid_value(ctl, val) {
                fail = true;
            }

            /* Minimum representable value */
            snd_ctl_elem_value_copy(val, (*ctl).def_val);
            snd_ctl_elem_value_set_integer(val, i as c_uint, LONG_MIN);

            if test_ctl_write_invalid_value(ctl, val) {
                fail = true;
            }
        }

        if snd_ctl_elem_info_get_max((*ctl).info) != LONG_MAX {
            /* Just over range */
            snd_ctl_elem_value_copy(val, (*ctl).def_val);
            snd_ctl_elem_value_set_integer(
                val,
                i as c_uint,
                snd_ctl_elem_info_get_max((*ctl).info) + 1,
            );

            if test_ctl_write_invalid_value(ctl, val) {
                fail = true;
            }

            /* Maximum representable value */
            snd_ctl_elem_value_copy(val, (*ctl).def_val);
            snd_ctl_elem_value_set_integer(val, i as c_uint, LONG_MAX);

            if test_ctl_write_invalid_value(ctl, val) {
                fail = true;
            }
        }
        i += 1;
    }

    !fail
}

unsafe fn test_ctl_write_invalid_integer64(ctl: *mut ctl_data) -> bool {
    let mut i: c_int;
    let mut fail = false;
    let mut val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut val);

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        if snd_ctl_elem_info_get_min64((*ctl).info) != LLONG_MIN {
            /* Just under range */
            snd_ctl_elem_value_copy(val, (*ctl).def_val);
            snd_ctl_elem_value_set_integer64(
                val,
                i as c_uint,
                snd_ctl_elem_info_get_min64((*ctl).info) - 1,
            );

            if test_ctl_write_invalid_value(ctl, val) {
                fail = true;
            }

            /* Minimum representable value */
            snd_ctl_elem_value_copy(val, (*ctl).def_val);
            snd_ctl_elem_value_set_integer64(val, i as c_uint, LLONG_MIN);

            if test_ctl_write_invalid_value(ctl, val) {
                fail = true;
            }
        }

        if snd_ctl_elem_info_get_max64((*ctl).info) != LLONG_MAX {
            /* Just over range */
            snd_ctl_elem_value_copy(val, (*ctl).def_val);
            snd_ctl_elem_value_set_integer64(
                val,
                i as c_uint,
                snd_ctl_elem_info_get_max64((*ctl).info) + 1,
            );

            if test_ctl_write_invalid_value(ctl, val) {
                fail = true;
            }

            /* Maximum representable value */
            snd_ctl_elem_value_copy(val, (*ctl).def_val);
            snd_ctl_elem_value_set_integer64(val, i as c_uint, LLONG_MAX);

            if test_ctl_write_invalid_value(ctl, val) {
                fail = true;
            }
        }
        i += 1;
    }

    !fail
}

unsafe fn test_ctl_write_invalid_enumerated(ctl: *mut ctl_data) -> bool {
    let mut i: c_int;
    let mut fail = false;
    let mut val: *mut snd_ctl_elem_value_t = ptr::null_mut();
    snd_ctl_elem_value_alloca(&mut val);

    snd_ctl_elem_value_set_id(val, (*ctl).id);

    i = 0;
    while i < snd_ctl_elem_info_get_count((*ctl).info) as c_int {
        /* One beyond maximum */
        snd_ctl_elem_value_copy(val, (*ctl).def_val);
        snd_ctl_elem_value_set_enumerated(
            val,
            i as c_uint,
            snd_ctl_elem_info_get_items((*ctl).info),
        );

        if test_ctl_write_invalid_value(ctl, val) {
            fail = true;
        }

        /* Maximum representable value */
        snd_ctl_elem_value_copy(val, (*ctl).def_val);
        snd_ctl_elem_value_set_enumerated(val, i as c_uint, UINT_MAX);

        if test_ctl_write_invalid_value(ctl, val) {
            fail = true;
        }

        i += 1;
    }

    !fail
}

unsafe fn test_ctl_write_invalid(ctl: *mut ctl_data) {
    let pass: bool;

    /* If the control is turned off let's be polite */
    if snd_ctl_elem_info_is_inactive((*ctl).info) != 0 {
        ksft_print_msg(cstr!("%s is inactive\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("write_invalid.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    if snd_ctl_elem_info_is_writable((*ctl).info) == 0 {
        ksft_print_msg(cstr!("%s is not writeable\n"), (*ctl).name);
        ksft_test_result_skip(cstr!("write_invalid.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
        return;
    }

    match snd_ctl_elem_info_get_type((*ctl).info) {
        SND_CTL_ELEM_TYPE_BOOLEAN => pass = test_ctl_write_invalid_boolean(ctl),
        SND_CTL_ELEM_TYPE_INTEGER => pass = test_ctl_write_invalid_integer(ctl),
        SND_CTL_ELEM_TYPE_INTEGER64 => pass = test_ctl_write_invalid_integer64(ctl),
        SND_CTL_ELEM_TYPE_ENUMERATED => pass = test_ctl_write_invalid_enumerated(ctl),
        _ => {
            /* No tests for this yet */
            ksft_test_result_skip(cstr!("write_invalid.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
            return;
        }
    }

    /* Restore the default value to minimise disruption */
    write_and_verify(ctl, (*ctl).def_val, ptr::null_mut());

    ksft_test_result(pass, cstr!("write_invalid.%s.%d\n"), (*(*ctl).card).card_name, (*ctl).elem);
}

unsafe fn test_ctl_event_missing(ctl: *mut ctl_data) {
    ksft_test_result(
        (*ctl).event_missing == 0,
        cstr!("event_missing.%s.%d\n"),
        (*(*ctl).card).card_name,
        (*ctl).elem,
    );
}

unsafe fn test_ctl_event_spurious(ctl: *mut ctl_data) {
    ksft_test_result(
        (*ctl).event_spurious == 0,
        cstr!("event_spurious.%s.%d\n"),
        (*(*ctl).card).card_name,
        (*ctl).elem,
    );
}

fn main() {
    unsafe {
        let mut ctl: *mut ctl_data;

        ksft_print_header();

        find_controls();

        ksft_set_plan((NUM_CONTROLS * TESTS_PER_CONTROL) as c_uint);

        ctl = CTL_LIST;
        while !ctl.is_null() {
            /*
             * Must test get_value() before we write anything, the
             * test stores the default value for later cleanup.
             */
            test_ctl_get_value(ctl);
            test_ctl_name(ctl);
            test_ctl_write_default(ctl);
            test_ctl_write_valid(ctl);
            test_ctl_write_invalid(ctl);
            test_ctl_event_missing(ctl);
            test_ctl_event_spurious(ctl);

            ctl = (*ctl).next;
        }

        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
