// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/bpf/prog_tests/bpf_qdisc.c.
// C include dependencies:
// <linux/pkt_sched.h>, <linux/rtnetlink.h>, <test_progs.h>,
// "network_helpers.h", and the referenced BPF skeleton headers.

use core::ffi::{c_char, c_int, c_uint, c_void};

const LO_IFINDEX: c_int = 1;
const TOTAL_BYTES: c_uint = 10 * 1024 * 1024;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const BPF_TC_QDISC: c_int = 4;
const TC_H_ROOT: u32 = 0xffff_ffff;
const IFNAMSIZ: usize = 16;
const EFAULT: c_int = 14;

#[inline]
const fn TC_H_MAKE(maj: u32, min: u32) -> u32 {
    maj | min
}

#[repr(C)]
pub struct bpf_tc_hook {
    pub sz: usize,
    pub ifindex: c_int,
    pub attach_point: c_int,
    pub parent: u32,
    pub handle: u32,
    pub qdisc: *mut c_char,
}

#[repr(C)]
pub struct bpf_qdisc_fifo {
    pub maps: bpf_qdisc_fifo_maps,
    pub bss: *mut bpf_qdisc_fifo_bss,
}

#[repr(C)]
pub struct bpf_qdisc_fifo_maps {
    pub test: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_qdisc_fifo_bss {
    pub init_called: bool,
}

#[repr(C)]
pub struct bpf_qdisc_fq {
    pub maps: bpf_qdisc_fq_maps,
}

#[repr(C)]
pub struct bpf_qdisc_fq_maps {
    pub test: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_qdisc_fail__incompl_ops {
    pub maps: bpf_qdisc_fail__incompl_ops_maps,
}

#[repr(C)]
pub struct bpf_qdisc_fail__incompl_ops_maps {
    pub test: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netns_obj {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;

    fn start_server(family: c_int, type_: c_int, addr: *const c_void, port: c_int, timeout_ms: c_int) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn send_recv_data(server_fd: c_int, client_fd: c_int, total_bytes: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn bpf_qdisc_fifo__open_and_load() -> *mut bpf_qdisc_fifo;
    fn bpf_qdisc_fifo__attach(skel: *mut bpf_qdisc_fifo) -> c_int;
    fn bpf_qdisc_fifo__destroy(skel: *mut bpf_qdisc_fifo);

    fn bpf_qdisc_fq__open_and_load() -> *mut bpf_qdisc_fq;
    fn bpf_qdisc_fq__attach(skel: *mut bpf_qdisc_fq) -> c_int;
    fn bpf_qdisc_fq__destroy(skel: *mut bpf_qdisc_fq);

    fn bpf_qdisc_fail__incompl_ops__open_and_load() -> *mut bpf_qdisc_fail__incompl_ops;
    fn bpf_qdisc_fail__incompl_ops__destroy(skel: *mut bpf_qdisc_fail__incompl_ops);

    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn write_sysctl(path: *const c_char, value: *const c_char) -> c_int;
    fn netns_new(name: *const c_char, attach: bool) -> *mut netns_obj;
    fn netns_free(netns: *mut netns_obj);

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ_bool(actual: bool, expected: bool, name: *const c_char) -> bool;

    // Local Rust stand-in for the test_progs SYS(label, command) macro dependency.
    fn SYS(command: *const c_char) -> c_int;

    // Local Rust stand-in for RUN_TESTS(skeleton_name) macro expansion.
    fn RUN_TESTS(test_name: *const c_char);
}

unsafe fn do_test(qdisc: *mut c_char) {
    let mut hook = bpf_tc_hook {
        sz: core::mem::size_of::<bpf_tc_hook>(),
        ifindex: LO_IFINDEX,
        attach_point: BPF_TC_QDISC,
        parent: TC_H_ROOT,
        handle: 0x8000000,
        qdisc,
    };
    let mut srv_fd: c_int = -1;
    let mut cli_fd: c_int = -1;
    let mut err: c_int;

    err = unsafe { bpf_tc_hook_create(&mut hook) };
    if !unsafe { ASSERT_OK(err, c"attach qdisc".as_ptr()) } {
        return;
    }

    'done: {
        srv_fd = unsafe { start_server(AF_INET6, SOCK_STREAM, core::ptr::null(), 0, 0) };
        if !unsafe { ASSERT_OK_FD(srv_fd, c"start server".as_ptr()) } {
            break 'done;
        }

        cli_fd = unsafe { connect_to_fd(srv_fd, 0) };
        if !unsafe { ASSERT_OK_FD(cli_fd, c"connect to client".as_ptr()) } {
            break 'done;
        }

        err = unsafe { send_recv_data(srv_fd, cli_fd, TOTAL_BYTES) };
        unsafe {
            ASSERT_OK(err, c"send_recv_data".as_ptr());
        }
    }

    if srv_fd != -1 {
        unsafe {
            close(srv_fd);
        }
    }
    if cli_fd != -1 {
        unsafe {
            close(cli_fd);
        }
    }

    unsafe {
        bpf_tc_hook_destroy(&mut hook);
    }
}

unsafe fn test_fifo() {
    let fifo_skel: *mut bpf_qdisc_fifo;

    fifo_skel = unsafe { bpf_qdisc_fifo__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(fifo_skel.cast(), c"bpf_qdisc_fifo__open_and_load".as_ptr()) } {
        return;
    }

    'out: {
        if !unsafe { ASSERT_OK(bpf_qdisc_fifo__attach(fifo_skel), c"bpf_qdisc_fifo__attach".as_ptr()) } {
            break 'out;
        }

        unsafe {
            do_test(c"bpf_fifo".as_ptr() as *mut c_char);
        }
    }

    unsafe {
        bpf_qdisc_fifo__destroy(fifo_skel);
    }
}

unsafe fn test_fq() {
    let fq_skel: *mut bpf_qdisc_fq;

    fq_skel = unsafe { bpf_qdisc_fq__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(fq_skel.cast(), c"bpf_qdisc_fq__open_and_load".as_ptr()) } {
        return;
    }

    'out: {
        if !unsafe { ASSERT_OK(bpf_qdisc_fq__attach(fq_skel), c"bpf_qdisc_fq__attach".as_ptr()) } {
            break 'out;
        }

        unsafe {
            do_test(c"bpf_fq".as_ptr() as *mut c_char);
        }
    }

    unsafe {
        bpf_qdisc_fq__destroy(fq_skel);
    }
}

unsafe fn test_qdisc_attach_to_mq() {
    let mut hook = bpf_tc_hook {
        sz: core::mem::size_of::<bpf_tc_hook>(),
        ifindex: 0,
        attach_point: BPF_TC_QDISC,
        parent: TC_H_MAKE(1 << 16, 1),
        handle: 0x11 << 16,
        qdisc: c"bpf_fifo".as_ptr() as *mut c_char,
    };
    let fifo_skel: *mut bpf_qdisc_fifo;
    let mut err: c_int;

    fifo_skel = unsafe { bpf_qdisc_fifo__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(fifo_skel.cast(), c"bpf_qdisc_fifo__open_and_load".as_ptr()) } {
        return;
    }

    'out: {
        if !unsafe { ASSERT_OK(bpf_qdisc_fifo__attach(fifo_skel), c"bpf_qdisc_fifo__attach".as_ptr()) } {
            break 'out;
        }

        if unsafe { SYS(c"ip link add veth0 type veth peer veth1".as_ptr()) } != 0 {
            break 'out;
        }
        hook.ifindex = unsafe { if_nametoindex(c"veth0".as_ptr()) as c_int };
        if unsafe { SYS(c"tc qdisc add dev veth0 root handle 1: mq".as_ptr()) } != 0 {
            break 'out;
        }

        err = unsafe { bpf_tc_hook_create(&mut hook) };
        unsafe {
            ASSERT_OK(err, c"attach qdisc".as_ptr());
        }

        unsafe {
            bpf_tc_hook_destroy(&mut hook);
        }

        if unsafe { SYS(c"tc qdisc delete dev veth0 root mq".as_ptr()) } != 0 {
            break 'out;
        }
    }

    unsafe {
        bpf_qdisc_fifo__destroy(fifo_skel);
    }
}

unsafe fn test_qdisc_attach_to_non_root() {
    let mut hook = bpf_tc_hook {
        sz: core::mem::size_of::<bpf_tc_hook>(),
        ifindex: LO_IFINDEX,
        attach_point: BPF_TC_QDISC,
        parent: TC_H_MAKE(1 << 16, 1),
        handle: 0x11 << 16,
        qdisc: c"bpf_fifo".as_ptr() as *mut c_char,
    };
    let fifo_skel: *mut bpf_qdisc_fifo;
    let mut err: c_int;

    fifo_skel = unsafe { bpf_qdisc_fifo__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(fifo_skel.cast(), c"bpf_qdisc_fifo__open_and_load".as_ptr()) } {
        return;
    }

    'out: {
        if !unsafe { ASSERT_OK(bpf_qdisc_fifo__attach(fifo_skel), c"bpf_qdisc_fifo__attach".as_ptr()) } {
            break 'out;
        }

        if unsafe { SYS(c"tc qdisc add dev lo root handle 1: htb".as_ptr()) } != 0 {
            break 'out;
        }

        'out_del_htb: {
            if unsafe { SYS(c"tc class add dev lo parent 1: classid 1:1 htb rate 75Kbit".as_ptr()) } != 0 {
                break 'out_del_htb;
            }

            err = unsafe { bpf_tc_hook_create(&mut hook) };
            if !unsafe { ASSERT_ERR(err, c"attach qdisc".as_ptr()) } {
                unsafe {
                    bpf_tc_hook_destroy(&mut hook);
                }
            }
        }

        if unsafe { SYS(c"tc qdisc delete dev lo root htb".as_ptr()) } != 0 {
            break 'out;
        }
    }

    unsafe {
        bpf_qdisc_fifo__destroy(fifo_skel);
    }
}

unsafe fn test_incompl_ops() {
    let skel: *mut bpf_qdisc_fail__incompl_ops;
    let link: *mut bpf_link;

    skel = unsafe { bpf_qdisc_fail__incompl_ops__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel.cast(), c"bpf_qdisc_fifo__open_and_load".as_ptr()) } {
        return;
    }

    link = unsafe { bpf_map__attach_struct_ops((*skel).maps.test) };
    if !unsafe { ASSERT_ERR_PTR(link.cast(), c"bpf_map__attach_struct_ops".as_ptr()) } {
        unsafe {
            bpf_link__destroy(link);
        }
    }

    unsafe {
        bpf_qdisc_fail__incompl_ops__destroy(skel);
    }
}

unsafe fn get_default_qdisc(qdisc_name: *mut c_char) -> c_int {
    let f: *mut FILE;
    let num: c_int;

    f = unsafe { fopen(c"/proc/sys/net/core/default_qdisc".as_ptr(), c"r".as_ptr()) };
    if f.is_null() {
        return -unsafe { errno };
    }

    num = unsafe { fscanf(f, c"%s".as_ptr(), qdisc_name) };
    unsafe {
        fclose(f);
    }

    if num == 1 { 0 } else { -EFAULT }
}

unsafe fn test_default_qdisc_attach_to_mq() {
    let mut default_qdisc: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    let fifo_skel: *mut bpf_qdisc_fifo;
    let mut netns: *mut netns_obj = core::ptr::null_mut();
    let mut err: c_int;

    fifo_skel = unsafe { bpf_qdisc_fifo__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(fifo_skel.cast(), c"bpf_qdisc_fifo__open_and_load".as_ptr()) } {
        return;
    }

    'out: {
        if !unsafe { ASSERT_OK(bpf_qdisc_fifo__attach(fifo_skel), c"bpf_qdisc_fifo__attach".as_ptr()) } {
            break 'out;
        }

        err = unsafe { get_default_qdisc(default_qdisc.as_mut_ptr()) };
        if !unsafe { ASSERT_OK(err, c"read sysctl net.core.default_qdisc".as_ptr()) } {
            break 'out;
        }

        err = unsafe {
            write_sysctl(
                c"/proc/sys/net/core/default_qdisc".as_ptr(),
                c"bpf_fifo".as_ptr(),
            )
        };
        if !unsafe { ASSERT_OK(err, c"write sysctl net.core.default_qdisc".as_ptr()) } {
            break 'out;
        }

        netns = unsafe { netns_new(c"bpf_qdisc_ns".as_ptr(), true) };
        if !unsafe { ASSERT_OK_PTR(netns.cast(), c"netns_new".as_ptr()) } {
            break 'out;
        }

        if unsafe { SYS(c"ip link add veth0 type veth peer veth1".as_ptr()) } != 0 {
            break 'out;
        }
        if unsafe { SYS(c"tc qdisc add dev veth0 root handle 1: mq".as_ptr()) } != 0 {
            break 'out;
        }

        unsafe {
            ASSERT_EQ_bool((*(*fifo_skel).bss).init_called, true, c"init_called".as_ptr());
        }

        if unsafe { SYS(c"tc qdisc delete dev veth0 root mq".as_ptr()) } != 0 {
            break 'out;
        }
    }

    unsafe {
        netns_free(netns);
    }
    if default_qdisc[0] != 0 {
        unsafe {
            write_sysctl(
                c"/proc/sys/net/core/default_qdisc".as_ptr(),
                default_qdisc.as_ptr(),
            );
        }
    }

    unsafe {
        bpf_qdisc_fifo__destroy(fifo_skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ns_bpf_qdisc() {
    if unsafe { test__start_subtest(c"fifo".as_ptr()) } {
        unsafe {
            test_fifo();
        }
    }
    if unsafe { test__start_subtest(c"fq".as_ptr()) } {
        unsafe {
            test_fq();
        }
    }
    if unsafe { test__start_subtest(c"attach to mq".as_ptr()) } {
        unsafe {
            test_qdisc_attach_to_mq();
        }
    }
    if unsafe { test__start_subtest(c"attach to non root".as_ptr()) } {
        unsafe {
            test_qdisc_attach_to_non_root();
        }
    }
    if unsafe { test__start_subtest(c"incompl_ops".as_ptr()) } {
        unsafe {
            test_incompl_ops();
        }
    }
    unsafe {
        RUN_TESTS(c"bpf_qdisc_fail__invalid_dynptr".as_ptr());
        RUN_TESTS(c"bpf_qdisc_fail__invalid_dynptr_cross_frame".as_ptr());
        RUN_TESTS(c"bpf_qdisc_fail__invalid_dynptr_slice".as_ptr());
        RUN_TESTS(c"bpf_qdisc_fail__untrusted_write".as_ptr());
        RUN_TESTS(c"bpf_qdisc_dynptr_use_after_invalidate_clone".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_bpf_qdisc_default() {
    unsafe {
        test_default_qdisc_attach_to_mq();
    }
}
