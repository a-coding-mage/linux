// SPDX-License-Identifier: GPL-2.0

// C includes translated as external dependencies:
// <linux/bpf.h>, <time.h>, <errno.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, and "bpf_misc.h".

#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_void = core::ffi::c_void;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_F_TIMER_CPU_PIN: u64 = 1 << 1;
const CLOCK_BOOTTIME: c_int = 7;

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elem {
    pub t: bpf_timer,
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct timer_map {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, int);
    // __type(value, struct elem);
    _private: [u8; 0],
}

// SEC(".maps")
#[no_mangle]
pub static mut timer1_map: timer_map = timer_map { _private: [] };

// SEC(".maps")
#[no_mangle]
pub static mut timer2_map: timer_map = timer_map { _private: [] };

#[no_mangle]
pub static mut timer1_err: c_int = 0;
#[no_mangle]
pub static mut timer2_err: c_int = 0;

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_timer_cancel(timer: *mut bpf_timer) -> c_int;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut c_void, clockid: c_int) -> c_int;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut c_void, *mut c_int, *mut elem) -> c_int,
    ) -> c_int;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> c_int;
}

unsafe extern "C" fn timer_cb1(_map: *mut c_void, _k: *mut c_int, _v: *mut elem) -> c_int {
    let mut timer: *mut bpf_timer;
    let key: c_int = 0;

    timer = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(timer2_map).cast::<c_void>(),
            core::ptr::addr_of!(key).cast::<c_void>(),
        )
    }
    .cast::<bpf_timer>();
    if !timer.is_null() {
        unsafe {
            timer2_err = bpf_timer_cancel(timer);
        }
    }

    0
}

unsafe extern "C" fn timer_cb2(_map: *mut c_void, _k: *mut c_int, _v: *mut elem) -> c_int {
    let mut timer: *mut bpf_timer;
    let key: c_int = 0;

    timer = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(timer1_map).cast::<c_void>(),
            core::ptr::addr_of!(key).cast::<c_void>(),
        )
    }
    .cast::<bpf_timer>();
    if !timer.is_null() {
        unsafe {
            timer1_err = bpf_timer_cancel(timer);
        }
    }

    0
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn timer1_prog(_ctx: *mut c_void) -> c_int {
    let mut timer: *mut bpf_timer;
    let key: c_int = 0;

    timer = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(timer1_map).cast::<c_void>(),
            core::ptr::addr_of!(key).cast::<c_void>(),
        )
    }
    .cast::<bpf_timer>();
    if !timer.is_null() {
        unsafe {
            bpf_timer_init(
                timer,
                core::ptr::addr_of_mut!(timer1_map).cast::<c_void>(),
                CLOCK_BOOTTIME,
            );
            bpf_timer_set_callback(timer, timer_cb1);
            bpf_timer_start(timer, 1, BPF_F_TIMER_CPU_PIN);
        }
    }

    0
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn timer2_prog(_ctx: *mut c_void) -> c_int {
    let mut timer: *mut bpf_timer;
    let key: c_int = 0;

    timer = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(timer2_map).cast::<c_void>(),
            core::ptr::addr_of!(key).cast::<c_void>(),
        )
    }
    .cast::<bpf_timer>();
    if !timer.is_null() {
        unsafe {
            bpf_timer_init(
                timer,
                core::ptr::addr_of_mut!(timer2_map).cast::<c_void>(),
                CLOCK_BOOTTIME,
            );
            bpf_timer_set_callback(timer, timer_cb2);
            bpf_timer_start(timer, 1, BPF_F_TIMER_CPU_PIN);
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
