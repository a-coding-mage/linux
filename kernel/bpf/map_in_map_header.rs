/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2017 Facebook
 */

// C dependency: <linux/types.h>

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

extern "C" {
    pub fn bpf_map_meta_alloc(inner_map_ufd: ::std::os::raw::c_int) -> *mut bpf_map;
    pub fn bpf_map_meta_free(map_meta: *mut bpf_map);
    pub fn bpf_map_fd_get_ptr(
        map: *mut bpf_map,
        map_file: *mut file,
        ufd: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
    pub fn bpf_map_fd_put_ptr(
        map: *mut bpf_map,
        ptr: *mut ::std::os::raw::c_void,
        need_defer: bool,
    );
    pub fn bpf_map_fd_sys_lookup_elem(ptr: *mut ::std::os::raw::c_void) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
