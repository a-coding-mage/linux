// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025. Huawei Technologies Co., Ltd */

// C dependencies translated as external Rust/BPF dependencies:
// <linux/bpf.h>, <time.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>

const MAX_ENTRIES: u32 = 8;

/* clang considers 'sum += 1' as usage but 'sum++' as non-usage.  GCC
 * is more consistent and considers both 'sum += 1' and 'sum++' as
 * non-usage.  This triggers warnings in the functions below.
 *
 * Starting with GCC 16 -Wunused-but-set-variable=2 can be used to
 * mimic clang's behavior.
 *
 * C conditional preserved:
 * #if !defined(__clang__) && __GNUC__ > 15
 * #pragma GCC diagnostic ignored "-Wunused-but-set-variable"
 * #endif
 */

#[repr(C)]
pub struct map_value {
    pub timer: bpf_timer,
}

#[repr(C)]
pub struct map_def {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, int);
    // __type(value, struct map_value);
    // __uint(max_entries, MAX_ENTRIES);
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[unsafe(link_section = ".maps")]
#[used]
pub static mut map: map_def = map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    max_entries: MAX_ENTRIES,
};

unsafe extern "C" fn timer_cb(
    map: *mut core::ffi::c_void,
    key: *mut core::ffi::c_void,
    value: *mut map_value,
) -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32;

    i = 0;
    while i < 1024 * 1024 {
        let old = core::ptr::read_volatile(&sum);
        core::ptr::write_volatile(&mut sum, old.wrapping_add(i));
        i = i.wrapping_add(1);
    }

    0
}

unsafe extern "C" fn start_cb(key: i32) -> i32 {
    let mut value: *mut map_value;

    value = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(map) as *mut core::ffi::c_void,
        (&key as *const i32).cast::<core::ffi::c_void>() as *mut core::ffi::c_void,
    ) as *mut map_value;
    if value.is_null() {
        return 0;
    }

    bpf_timer_init(
        core::ptr::addr_of_mut!((*value).timer),
        core::ptr::addr_of_mut!(map) as *mut core::ffi::c_void,
        CLOCK_MONOTONIC,
    );
    bpf_timer_set_callback(core::ptr::addr_of_mut!((*value).timer), timer_cb);
    /* Hope 100us will be enough to wake-up and run the overwrite thread */
    bpf_timer_start(
        core::ptr::addr_of_mut!((*value).timer),
        100000,
        BPF_F_TIMER_CPU_PIN,
    );

    0
}

unsafe extern "C" fn overwrite_cb(key: i32) -> i32 {
    let zero: map_value = core::mem::zeroed();

    /* Free the timer which may run on other CPU */
    bpf_map_update_elem(
        core::ptr::addr_of_mut!(map) as *mut core::ffi::c_void,
        (&key as *const i32).cast::<core::ffi::c_void>() as *mut core::ffi::c_void,
        (&zero as *const map_value).cast::<core::ffi::c_void>() as *mut core::ffi::c_void,
        BPF_ANY,
    );

    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_timer(ctx: *mut core::ffi::c_void) -> i32 {
    bpf_loop(
        MAX_ENTRIES,
        start_cb as *mut core::ffi::c_void,
        core::ptr::null_mut(),
        0,
    );
    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn overwrite_timer(ctx: *mut core::ffi::c_void) -> i32 {
    bpf_loop(
        MAX_ENTRIES,
        overwrite_cb as *mut core::ffi::c_void,
        core::ptr::null_mut(),
        0,
    );
    0
}

#[unsafe(link_section = "license")]
#[used]
pub static _license: [u8; 4] = *b"GPL\0";
