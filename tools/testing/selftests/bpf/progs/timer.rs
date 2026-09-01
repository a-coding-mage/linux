// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u64 = u64;
type __s32 = i32;

const CLOCK_MONOTONIC: i32 = 1;
const CLOCK_BOOTTIME: i32 = 7;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_LRU_HASH: u32 = 9;
const BPF_F_NO_PREALLOC: __u64 = 1;
const BPF_F_TIMER_ABS: __u64 = 1 << 0;
const BPF_F_TIMER_CPU_PIN: __u64 = 1 << 1;
const BPF_ANY: __u64 = 0;
const EINVAL: i32 = 22;
const EDEADLK: i32 = 35;
const EBUSY: i32 = 16;

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct hmap_elem {
    pub counter: i32,
    pub timer: bpf_timer,
    pub lock: bpf_spin_lock, /* unused */
}

#[repr(C)]
pub struct elem {
    pub t: bpf_timer,
}

/* BPF map definitions translated from SEC(".maps") declarations. */
#[repr(C)]
pub struct hmap_map {
    pub type_: u32,
    pub max_entries: u32,
}

#[repr(C)]
pub struct hmap_malloc_map {
    pub type_: u32,
    pub map_flags: __u64,
    pub max_entries: u32,
}

#[repr(C)]
pub struct array_map {
    pub type_: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut hmap: hmap_map = hmap_map {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1000,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut hmap_malloc: hmap_malloc_map = hmap_malloc_map {
    type_: BPF_MAP_TYPE_HASH,
    map_flags: BPF_F_NO_PREALLOC,
    max_entries: 1000,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut array: array_map = array_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 2,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut lru: array_map = array_map {
    type_: BPF_MAP_TYPE_LRU_HASH,
    max_entries: 4,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut abs_timer: array_map = array_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut soft_timer_pinned: array_map = array_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut abs_timer_pinned: array_map = array_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut race_array: array_map = array_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[no_mangle]
pub static mut bss_data: __u64 = 0;
#[no_mangle]
pub static mut abs_data: __u64 = 0;
#[no_mangle]
pub static mut err: __u64 = 0;
#[no_mangle]
pub static mut ok: __u64 = 0;
#[no_mangle]
pub static mut test_hits: __u64 = 0;
#[no_mangle]
pub static mut update_hits: __u64 = 0;
#[no_mangle]
pub static mut cancel_hits: __u64 = 0;
#[no_mangle]
pub static mut callback_check: __u64 = 52;
#[no_mangle]
pub static mut callback2_check: __u64 = 52;
#[no_mangle]
pub static mut pinned_callback_check: __u64 = 0;
#[no_mangle]
pub static mut pinned_cpu: __s32 = 0;
#[no_mangle]
pub static mut async_cancel: bool = false;

const ARRAY: i32 = 1;
const HTAB: i32 = 2;
const HTAB_MALLOC: i32 = 3;
const LRU: i32 = 4;

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i32;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut core::ffi::c_void, clockid: i32) -> i32;
    fn bpf_timer_set_callback(timer: *mut bpf_timer, callback: *const core::ffi::c_void) -> i32;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: __u64, flags: __u64) -> i32;
    fn bpf_timer_cancel(timer: *mut bpf_timer) -> i32;
    fn bpf_timer_cancel_async(timer: *mut bpf_timer) -> i32;
    fn bpf_ktime_get_boot_ns() -> __u64;
    fn bpf_get_smp_processor_id() -> __s32;
    fn bpf_printk(fmt: *const u8, ...) -> i32;
}

/* callback for array and lru timers */
unsafe extern "C" fn timer_cb1(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    timer: *mut bpf_timer,
) -> i32 {
    /* increment bss variable twice.
     * Once via array timer callback and once via lru timer callback
     */
    bss_data += 5;

    /* *key == 0 - the callback was called for array timer.
     * *key == 4 - the callback was called from lru timer.
     */
    if *key == ARRAY {
        let mut lru_timer: *mut bpf_timer;
        let lru_key: i32 = LRU;

        /* rearm array timer to be called again in ~35 seconds */
        if bpf_timer_start(timer, 1u64 << 35, 0) != 0 {
            err |= 1;
        }

        lru_timer = bpf_map_lookup_elem(
            &raw mut lru as *mut core::ffi::c_void,
            &lru_key as *const _ as *const core::ffi::c_void,
        ) as *mut bpf_timer;
        if lru_timer.is_null() {
            return 0;
        }
        bpf_timer_set_callback(lru_timer, timer_cb1 as *const core::ffi::c_void);
        if bpf_timer_start(lru_timer, 0, 0) != 0 {
            err |= 2;
        }
    } else if *key == LRU {
        let mut lru_key: i32;
        let mut i: i32;

        i = LRU + 1;
        while i <= 100 {
            /* for current LRU eviction algorithm this number
             * should be larger than ~ lru->max_entries * 2
             */
            let init: elem = core::mem::zeroed();

            /* lru_key cannot be used as loop induction variable
             * otherwise the loop will be unbounded.
             */
            lru_key = i;

            /* add more elements into lru map to push out current
             * element and force deletion of this timer
             */
            bpf_map_update_elem(
                map,
                &lru_key as *const _ as *const core::ffi::c_void,
                &init as *const _ as *const core::ffi::c_void,
                0,
            );
            /* look it up to bump it into active list */
            bpf_map_lookup_elem(map, &lru_key as *const _ as *const core::ffi::c_void);

            /* keep adding until *key changes underneath,
             * which means that key/timer memory was reused
             */
            if *key != LRU {
                break;
            }
            i += 1;
        }

        /* check that the timer was removed */
        if bpf_timer_cancel(timer) != -EINVAL {
            err |= 4;
        }
        ok |= 1;
    }
    0
}

#[link_section = "fentry/bpf_fentry_test1"]
#[no_mangle]
pub unsafe extern "C" fn test1(a: i32) -> i32 {
    let mut arr_timer: *mut bpf_timer;
    let mut lru_timer: *mut bpf_timer;
    let init: elem = core::mem::zeroed();
    let lru_key: i32 = LRU;
    let mut array_key: i32 = ARRAY;

    arr_timer = bpf_map_lookup_elem(
        &raw mut array as *mut core::ffi::c_void,
        &array_key as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_timer;
    if arr_timer.is_null() {
        return 0;
    }
    bpf_timer_init(arr_timer, &raw mut array as *mut core::ffi::c_void, CLOCK_MONOTONIC);

    bpf_map_update_elem(
        &raw mut lru as *mut core::ffi::c_void,
        &lru_key as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );
    lru_timer = bpf_map_lookup_elem(
        &raw mut lru as *mut core::ffi::c_void,
        &lru_key as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_timer;
    if lru_timer.is_null() {
        return 0;
    }
    bpf_timer_init(lru_timer, &raw mut lru as *mut core::ffi::c_void, CLOCK_MONOTONIC);

    bpf_timer_set_callback(arr_timer, timer_cb1 as *const core::ffi::c_void);
    bpf_timer_start(arr_timer, 0, 0); /* call timer_cb1 asap */

    /* init more timers to check that array destruction
     * doesn't leak timer memory.
     */
    array_key = 0;
    arr_timer = bpf_map_lookup_elem(
        &raw mut array as *mut core::ffi::c_void,
        &array_key as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_timer;
    if arr_timer.is_null() {
        return 0;
    }
    bpf_timer_init(arr_timer, &raw mut array as *mut core::ffi::c_void, CLOCK_MONOTONIC);
    0
}

unsafe extern "C" fn timer_error(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    timer: *mut bpf_timer,
) -> i32 {
    err = 42;
    0
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_async_cancel_succeed(ctx: *mut core::ffi::c_void) -> i32 {
    let mut arr_timer: *mut bpf_timer;
    let array_key: i32 = ARRAY;

    arr_timer = bpf_map_lookup_elem(
        &raw mut array as *mut core::ffi::c_void,
        &array_key as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_timer;
    if arr_timer.is_null() {
        return 0;
    }
    bpf_timer_init(arr_timer, &raw mut array as *mut core::ffi::c_void, CLOCK_MONOTONIC);
    bpf_timer_set_callback(arr_timer, timer_error as *const core::ffi::c_void);
    bpf_timer_start(arr_timer, 100000, 0); /* 100us */
    bpf_timer_cancel_async(arr_timer);
    ok = 7;
    0
}

/* callback for prealloc and non-prealloca hashtab timers */
unsafe extern "C" fn timer_cb2(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    val: *mut hmap_elem,
) -> i32 {
    if *key == HTAB {
        callback_check -= 1;
    } else {
        callback2_check -= 1;
    }
    if (*val).counter > 0 && {
        (*val).counter -= 1;
        (*val).counter != 0
    } {
        /* re-arm the timer again to execute after 1 usec */
        bpf_timer_start(&mut (*val).timer, 1000, 0);
    } else if *key == HTAB {
        let mut arr_timer: *mut bpf_timer;
        let array_key: i32 = ARRAY;

        /* cancel arr_timer otherwise bpf_fentry_test1 prog
         * will stay alive forever.
         */
        arr_timer = bpf_map_lookup_elem(
            &raw mut array as *mut core::ffi::c_void,
            &array_key as *const _ as *const core::ffi::c_void,
        ) as *mut bpf_timer;
        if arr_timer.is_null() {
            return 0;
        }
        if bpf_timer_cancel(arr_timer) != 1 {
            /* bpf_timer_cancel should return 1 to indicate
             * that arr_timer was active at this time
             */
            err |= 8;
        }

        /* try to cancel ourself. It shouldn't deadlock. */
        if bpf_timer_cancel(&mut (*val).timer) != -EDEADLK {
            err |= 16;
        }

        /* delete this key and this timer anyway.
         * It shouldn't deadlock either.
         */
        bpf_map_delete_elem(map, key as *const core::ffi::c_void);

        /* in preallocated hashmap both 'key' and 'val' could have been
         * reused to store another map element (like in LRU above),
         * but in controlled test environment the below test works.
         * It's not a use-after-free. The memory is owned by the map.
         */
        if bpf_timer_start(&mut (*val).timer, 1000, 0) != -EINVAL {
            err |= 32;
        }
        ok |= 2;
    } else {
        if *key != HTAB_MALLOC {
            err |= 64;
        }

        /* try to cancel ourself. It shouldn't deadlock. */
        if bpf_timer_cancel(&mut (*val).timer) != -EDEADLK {
            err |= 128;
        }

        /* delete this key and this timer anyway.
         * It shouldn't deadlock either.
         */
        bpf_map_delete_elem(map, key as *const core::ffi::c_void);

        ok |= 4;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_timer_test() -> i32 {
    let mut val: *mut hmap_elem;
    let key: i32 = HTAB;
    let key_malloc: i32 = HTAB_MALLOC;

    val = bpf_map_lookup_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if !val.is_null() {
        if bpf_timer_init(&mut (*val).timer, &raw mut hmap as *mut core::ffi::c_void, CLOCK_BOOTTIME) != 0 {
            err |= 512;
        }
        bpf_timer_set_callback(&mut (*val).timer, timer_cb2 as *const core::ffi::c_void);
        bpf_timer_start(&mut (*val).timer, 1000, 0);
    }
    val = bpf_map_lookup_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if !val.is_null() {
        if bpf_timer_init(
            &mut (*val).timer,
            &raw mut hmap_malloc as *mut core::ffi::c_void,
            CLOCK_BOOTTIME,
        ) != 0
        {
            err |= 1024;
        }
        bpf_timer_set_callback(&mut (*val).timer, timer_cb2 as *const core::ffi::c_void);
        bpf_timer_start(&mut (*val).timer, 1000, 0);
    }
    0
}

#[link_section = "fentry/bpf_fentry_test2"]
#[no_mangle]
pub unsafe extern "C" fn test2(a: i32, b: i32) -> i32 {
    let mut init: hmap_elem = core::mem::zeroed();
    let mut val: *mut hmap_elem;
    let mut key: i32 = HTAB;
    let mut key_malloc: i32 = HTAB_MALLOC;

    init.counter = 10; /* number of times to trigger timer_cb2 */
    bpf_map_update_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );
    val = bpf_map_lookup_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if !val.is_null() {
        bpf_timer_init(&mut (*val).timer, &raw mut hmap as *mut core::ffi::c_void, CLOCK_BOOTTIME);
    }
    /* update the same key to free the timer */
    bpf_map_update_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );

    bpf_map_update_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );
    val = bpf_map_lookup_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if !val.is_null() {
        bpf_timer_init(
            &mut (*val).timer,
            &raw mut hmap_malloc as *mut core::ffi::c_void,
            CLOCK_BOOTTIME,
        );
    }
    /* update the same key to free the timer */
    bpf_map_update_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );

    /* init more timers to check that htab operations
     * don't leak timer memory.
     */
    key = 0;
    bpf_map_update_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );
    val = bpf_map_lookup_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if !val.is_null() {
        bpf_timer_init(&mut (*val).timer, &raw mut hmap as *mut core::ffi::c_void, CLOCK_BOOTTIME);
    }
    bpf_map_delete_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    );
    bpf_map_update_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );
    val = bpf_map_lookup_elem(
        &raw mut hmap as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if !val.is_null() {
        bpf_timer_init(&mut (*val).timer, &raw mut hmap as *mut core::ffi::c_void, CLOCK_BOOTTIME);
    }

    /* and with non-prealloc htab */
    key_malloc = 0;
    bpf_map_update_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );
    val = bpf_map_lookup_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if !val.is_null() {
        bpf_timer_init(
            &mut (*val).timer,
            &raw mut hmap_malloc as *mut core::ffi::c_void,
            CLOCK_BOOTTIME,
        );
    }
    bpf_map_delete_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
    );
    bpf_map_update_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        0,
    );
    val = bpf_map_lookup_elem(
        &raw mut hmap_malloc as *mut core::ffi::c_void,
        &key_malloc as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if !val.is_null() {
        bpf_timer_init(
            &mut (*val).timer,
            &raw mut hmap_malloc as *mut core::ffi::c_void,
            CLOCK_BOOTTIME,
        );
    }

    bpf_timer_test()
}

/* callback for absolute timer */
unsafe extern "C" fn timer_cb3(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    timer: *mut bpf_timer,
) -> i32 {
    abs_data += 6;

    if abs_data < 12 {
        bpf_timer_start(timer, bpf_ktime_get_boot_ns() + 1000, BPF_F_TIMER_ABS);
    } else {
        /* Re-arm timer ~35 seconds in future */
        bpf_timer_start(
            timer,
            bpf_ktime_get_boot_ns() + (1u64 << 35),
            BPF_F_TIMER_ABS,
        );
    }

    0
}

#[link_section = "fentry/bpf_fentry_test3"]
#[no_mangle]
pub unsafe extern "C" fn test3(a: i32) -> i32 {
    let key: i32 = 0;
    let mut timer: *mut bpf_timer;

    bpf_printk(b"test3\0".as_ptr());

    timer = bpf_map_lookup_elem(
        &raw mut abs_timer as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_timer;
    if !timer.is_null() {
        if bpf_timer_init(timer, &raw mut abs_timer as *mut core::ffi::c_void, CLOCK_BOOTTIME) != 0 {
            err |= 2048;
        }
        bpf_timer_set_callback(timer, timer_cb3 as *const core::ffi::c_void);
        bpf_timer_start(timer, bpf_ktime_get_boot_ns() + 1000, BPF_F_TIMER_ABS);
    }

    0
}

/* callback for pinned timer */
unsafe extern "C" fn timer_cb_pinned(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    timer: *mut bpf_timer,
) -> i32 {
    let cpu: __s32 = bpf_get_smp_processor_id();

    if cpu != pinned_cpu {
        err |= 16384;
    }

    pinned_callback_check += 1;
    0
}

unsafe fn test_pinned_timer(soft: bool) {
    let key: i32 = 0;
    let map: *mut core::ffi::c_void;
    let mut timer: *mut bpf_timer;
    let mut flags: __u64 = BPF_F_TIMER_CPU_PIN;
    let start_time: __u64;

    if soft {
        map = &raw mut soft_timer_pinned as *mut core::ffi::c_void;
        start_time = 0;
    } else {
        map = &raw mut abs_timer_pinned as *mut core::ffi::c_void;
        start_time = bpf_ktime_get_boot_ns();
        flags |= BPF_F_TIMER_ABS;
    }

    timer = bpf_map_lookup_elem(map, &key as *const _ as *const core::ffi::c_void) as *mut bpf_timer;
    if !timer.is_null() {
        if bpf_timer_init(timer, map, CLOCK_BOOTTIME) != 0 {
            err |= 4096;
        }
        bpf_timer_set_callback(timer, timer_cb_pinned as *const core::ffi::c_void);
        pinned_cpu = bpf_get_smp_processor_id();
        bpf_timer_start(timer, start_time + 1000, flags);
    } else {
        err |= 8192;
    }
}

#[link_section = "fentry/bpf_fentry_test4"]
#[no_mangle]
pub unsafe extern "C" fn test4(a: i32) -> i32 {
    bpf_printk(b"test4\0".as_ptr());
    test_pinned_timer(true);

    0
}

#[link_section = "fentry/bpf_fentry_test5"]
#[no_mangle]
pub unsafe extern "C" fn test5(a: i32) -> i32 {
    bpf_printk(b"test5\0".as_ptr());
    test_pinned_timer(false);

    0
}

unsafe extern "C" fn race_timer_callback(
    race_array: *mut core::ffi::c_void,
    race_key: *mut i32,
    timer: *mut bpf_timer,
) -> i32 {
    bpf_timer_start(timer, 1000000, 0);
    0
}

/* Callback that updates its own map element */
unsafe extern "C" fn update_self_callback(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    timer: *mut bpf_timer,
) -> i32 {
    let init: elem = core::mem::zeroed();

    bpf_map_update_elem(
        map,
        key as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        BPF_ANY,
    );
    core::intrinsics::atomic_xadd_relaxed(&mut update_hits, 1);
    0
}

/* Callback that cancels itself using async cancel */
unsafe extern "C" fn cancel_self_callback(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    timer: *mut bpf_timer,
) -> i32 {
    bpf_timer_cancel_async(timer);
    core::intrinsics::atomic_xadd_relaxed(&mut cancel_hits, 1);
    0
}

#[repr(C)]
pub enum test_mode {
    TEST_RACE_SYNC,
    TEST_RACE_ASYNC,
    TEST_UPDATE,
    TEST_CANCEL,
}

#[inline(always)]
unsafe fn test_common(mode: test_mode) -> i32 {
    let mut timer: *mut bpf_timer;
    let mut init: elem = core::mem::zeroed();
    let ret: i32;
    let key: i32 = 0;

    core::ptr::write_bytes(&mut init as *mut elem, 0, 1);

    bpf_map_update_elem(
        &raw mut race_array as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
        &init as *const _ as *const core::ffi::c_void,
        BPF_ANY,
    );
    timer = bpf_map_lookup_elem(
        &raw mut race_array as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_timer;
    if timer.is_null() {
        return 0;
    }

    ret = bpf_timer_init(timer, &raw mut race_array as *mut core::ffi::c_void, CLOCK_MONOTONIC);
    if ret != 0 && ret != -EBUSY {
        return 0;
    }

    match mode {
        test_mode::TEST_RACE_SYNC | test_mode::TEST_RACE_ASYNC => {
            bpf_timer_set_callback(timer, race_timer_callback as *const core::ffi::c_void);
        }
        test_mode::TEST_UPDATE => {
            bpf_timer_set_callback(timer, update_self_callback as *const core::ffi::c_void);
        }
        test_mode::TEST_CANCEL => {
            bpf_timer_set_callback(timer, cancel_self_callback as *const core::ffi::c_void);
        }
    }

    bpf_timer_start(timer, 0, 0);

    match mode {
        test_mode::TEST_RACE_ASYNC => {
            bpf_timer_cancel_async(timer);
        }
        test_mode::TEST_RACE_SYNC => {
            bpf_timer_cancel(timer);
        }
        _ => {}
    }

    0
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn race(ctx: *mut core::ffi::c_void) -> i32 {
    test_common(if async_cancel {
        test_mode::TEST_RACE_ASYNC
    } else {
        test_mode::TEST_RACE_SYNC
    })
}

#[link_section = "perf_event"]
#[no_mangle]
pub unsafe extern "C" fn nmi_race(ctx: *mut core::ffi::c_void) -> i32 {
    core::intrinsics::atomic_xadd_relaxed(&mut test_hits, 1);
    test_common(test_mode::TEST_RACE_ASYNC)
}

#[link_section = "perf_event"]
#[no_mangle]
pub unsafe extern "C" fn nmi_update(ctx: *mut core::ffi::c_void) -> i32 {
    core::intrinsics::atomic_xadd_relaxed(&mut test_hits, 1);
    test_common(test_mode::TEST_UPDATE)
}

#[link_section = "perf_event"]
#[no_mangle]
pub unsafe extern "C" fn nmi_cancel(ctx: *mut core::ffi::c_void) -> i32 {
    core::intrinsics::atomic_xadd_relaxed(&mut test_hits, 1);
    test_common(test_mode::TEST_CANCEL)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
