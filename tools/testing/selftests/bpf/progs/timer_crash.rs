// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub tgid: i32,
}

#[repr(C)]
pub struct map_elem {
    pub timer: bpf_timer,
    pub lock: bpf_spin_lock,
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, struct map_elem);
// } amap SEC(".maps");
#[repr(C)]
pub struct amap {
    _private: [u8; 0],
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, struct map_elem);
// } hmap SEC(".maps");
#[repr(C)]
pub struct hmap {
    _private: [u8; 0],
}

unsafe extern "C" {
    #[link_name = "amap"]
    pub static mut AMAP: amap;
    #[link_name = "hmap"]
    pub static mut HMAP: hmap;

    pub fn bpf_get_current_task_btf() -> *mut task_struct;
    pub fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    pub fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_timer_cancel(timer: *mut bpf_timer) -> i64;
}

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut crash_map: i32 = 0; /* 0 for amap, 1 for hmap */

// SEC("fentry/do_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter(ctx: *mut core::ffi::c_void) -> i32 {
    let mut e: *mut map_elem;
    let mut value: map_elem = core::mem::zeroed();
    let map: *mut core::ffi::c_void = if crash_map != 0 {
        &raw mut HMAP as *mut core::ffi::c_void
    } else {
        &raw mut AMAP as *mut core::ffi::c_void
    };

    let _ = ctx;

    if (*bpf_get_current_task_btf()).tgid != pid {
        return 0;
    }

    *(&mut value as *mut map_elem as *mut *mut core::ffi::c_void) =
        0xdeadcaf3usize as *mut core::ffi::c_void;

    bpf_map_update_elem(
        map,
        &(0i32) as *const i32 as *const core::ffi::c_void,
        &value as *const map_elem as *const core::ffi::c_void,
        0,
    );
    /* For array map, doing bpf_map_update_elem will do a
     * check_and_free_timer_in_array, which will trigger the crash if timer
     * pointer was overwritten, for hmap we need to use bpf_timer_cancel.
     */
    if crash_map == 1 {
        e = bpf_map_lookup_elem(map, &(0i32) as *const i32 as *const core::ffi::c_void)
            as *mut map_elem;
        if e.is_null() {
            return 0;
        }
        bpf_timer_cancel(&mut (*e).timer);
    }
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";
