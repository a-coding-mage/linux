// SPDX-License-Identifier: GPL-2.0
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct hmap_elem {
    pub timer: bpf_timer,
}

// Original C used BPF map definition macros:
// __uint(type, BPF_MAP_TYPE_HASH);
// __uint(max_entries, 64);
// __type(key, int);
// __type(value, struct hmap_elem);
#[link_section = ".maps"]
#[no_mangle]
pub static mut hmap: hmap_elem = hmap_elem {
    timer: unsafe { core::mem::zeroed() },
};

#[inline(never)]
unsafe fn timer_cb(map: *mut core::ffi::c_void, key: *mut core::ffi::c_int, timer: *mut bpf_timer) -> core::ffi::c_int {
    let _ = map;
    let _ = key;
    let _ = timer;
    let mut buf: [core::ffi::c_char; 256] = [0; 256];
    unsafe { core::ptr::read_volatile(buf.as_mut_ptr().add(69)) as core::ffi::c_int }
}

#[inline(never)]
unsafe fn bad_timer_cb(map: *mut core::ffi::c_void, key: *mut core::ffi::c_int, timer: *mut bpf_timer) -> core::ffi::c_int {
    let _ = map;
    let _ = key;
    let mut buf: [core::ffi::c_char; 300] = [0; 300];
    unsafe {
        (core::ptr::read_volatile(buf.as_mut_ptr().add(255)) as core::ffi::c_int)
            + timer_cb(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut())
    }
}

// SEC("tc")
// __failure __msg("combined stack size of 2 calls is")
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn pseudo_call_check(ctx: *mut __sk_buff) -> core::ffi::c_int {
    let _ = ctx;
    let mut elem: *mut hmap_elem;
    let mut buf: [core::ffi::c_char; 256] = [0; 256];

    elem = unsafe {
        let key: core::ffi::c_int = 0;
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(hmap) as *mut core::ffi::c_void,
            core::ptr::addr_of!(key) as *const core::ffi::c_void,
        ) as *mut hmap_elem
    };
    if elem.is_null() {
        return 0;
    }

    unsafe {
        timer_cb(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
        bpf_timer_set_callback(core::ptr::addr_of_mut!((*elem).timer), Some(timer_cb))
            + (core::ptr::read_volatile(buf.as_mut_ptr().add(0)) as core::ffi::c_int)
    }
}

// SEC("tc")
// __failure __msg("combined stack size of 2 calls is")
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn async_call_root_check(ctx: *mut __sk_buff) -> core::ffi::c_int {
    let _ = ctx;
    let mut elem: *mut hmap_elem;
    let mut buf: [core::ffi::c_char; 256] = [0; 256];

    elem = unsafe {
        let key: core::ffi::c_int = 0;
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(hmap) as *mut core::ffi::c_void,
            core::ptr::addr_of!(key) as *const core::ffi::c_void,
        ) as *mut hmap_elem
    };
    if elem.is_null() {
        return 0;
    }

    unsafe {
        bpf_timer_set_callback(core::ptr::addr_of_mut!((*elem).timer), Some(bad_timer_cb))
            + (core::ptr::read_volatile(buf.as_mut_ptr().add(0)) as core::ffi::c_int)
    }
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
