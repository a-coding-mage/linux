// SPDX-License-Identifier: GPL-2.0

// C dependencies: <test_progs.h>
// C dependency: "cgroup_skb_direct_packet_access.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_size_in: c_uint,
    pub retval: c_uint,
}

#[repr(C)]
pub struct cgroup_skb_direct_packet_access {
    pub progs: cgroup_skb_direct_packet_access__progs,
    pub bss: *mut cgroup_skb_direct_packet_access__bss,
}

#[repr(C)]
pub struct cgroup_skb_direct_packet_access__progs {
    pub direct_packet_access: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_skb_direct_packet_access__bss {
    pub data_end: u64,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn cgroup_skb_direct_packet_access__open_and_load(
    ) -> *mut cgroup_skb_direct_packet_access;
    fn cgroup_skb_direct_packet_access__destroy(
        skel: *mut cgroup_skb_direct_packet_access,
    );
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: u64, expected: u64, name: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_skb_prog_run_direct_packet_access() {
    let mut err: c_int;
    let mut skel: *mut cgroup_skb_direct_packet_access;
    let mut test_skb: [c_char; 64] = [0; 64];

    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: test_skb.as_mut_ptr() as *mut c_void,
        data_size_in: core::mem::size_of_val(&test_skb) as c_uint,
        retval: 0,
    };

    skel = cgroup_skb_direct_packet_access__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"cgroup_skb_direct_packet_access__open_and_load".as_ptr(),
    ) {
        return;
    }

    err = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.direct_packet_access),
        &mut topts,
    );
    ASSERT_OK(err, c"bpf_prog_test_run_opts err".as_ptr());
    ASSERT_EQ(topts.retval, 1, c"retval".as_ptr());

    ASSERT_NEQ((*(*skel).bss).data_end, 0, c"data_end".as_ptr());

    cgroup_skb_direct_packet_access__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
