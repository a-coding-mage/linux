// SPDX-License-Identifier: GPL-2.0

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_mode: core::ffi::c_int,
}

#[repr(C)]
pub struct test_array {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 8);
    // __type(key, __u32);
    // __type(value, __u64);
    _private: [u8; 0],
}

// SEC(".maps")
#[no_mangle]
pub static mut test_array: test_array = test_array { _private: [] };

#[no_mangle]
pub static mut triggered: core::ffi::c_uint = 0;

unsafe extern "C" {
    pub fn bpf_for_each_map_elem(
        map: *mut bpf_map,
        callback: unsafe extern "C" fn(
            map: *mut bpf_map,
            key: *mut u32,
            val: *mut u64,
            data: *mut core::ffi::c_void,
        ) -> u64,
        data: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

unsafe extern "C" fn test_cb(
    map: *mut bpf_map,
    key: *mut u32,
    val: *mut u64,
    data: *mut core::ffi::c_void,
) -> u64 {
    1
}

// SEC("fexit/bpf_testmod_return_ptr")
#[no_mangle]
pub unsafe extern "C" fn handle_fexit_ret_subprogs(
    arg: core::ffi::c_int,
    ret: *mut file,
) -> core::ffi::c_int {
    core::ptr::read_volatile(ret as *const core::ffi::c_int);
    core::ptr::read_volatile(core::ptr::addr_of!((*ret).f_mode));
    bpf_for_each_map_elem(
        core::ptr::addr_of_mut!(test_array) as *mut bpf_map,
        test_cb,
        core::ptr::null_mut(),
        0,
    );
    triggered = triggered.wrapping_add(1);
    0
}

// SEC("fexit/bpf_testmod_return_ptr")
#[no_mangle]
pub unsafe extern "C" fn handle_fexit_ret_subprogs2(
    arg: core::ffi::c_int,
    ret: *mut file,
) -> core::ffi::c_int {
    core::ptr::read_volatile(ret as *const core::ffi::c_int);
    core::ptr::read_volatile(core::ptr::addr_of!((*ret).f_mode));
    bpf_for_each_map_elem(
        core::ptr::addr_of_mut!(test_array) as *mut bpf_map,
        test_cb,
        core::ptr::null_mut(),
        0,
    );
    triggered = triggered.wrapping_add(1);
    0
}

// SEC("fexit/bpf_testmod_return_ptr")
#[no_mangle]
pub unsafe extern "C" fn handle_fexit_ret_subprogs3(
    arg: core::ffi::c_int,
    ret: *mut file,
) -> core::ffi::c_int {
    core::ptr::read_volatile(ret as *const core::ffi::c_int);
    core::ptr::read_volatile(core::ptr::addr_of!((*ret).f_mode));
    bpf_for_each_map_elem(
        core::ptr::addr_of_mut!(test_array) as *mut bpf_map,
        test_cb,
        core::ptr::null_mut(),
        0,
    );
    triggered = triggered.wrapping_add(1);
    0
}

// SEC("license")
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
