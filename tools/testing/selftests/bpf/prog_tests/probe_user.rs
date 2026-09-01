// SPDX-License-Identifier: GPL-2.0
// Translated from C source. External test_progs/libbpf/libc symbols are
// expected to be supplied by the surrounding repository bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

#[repr(C)]
pub struct sockaddr {
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

pub enum bpf_object {}
pub enum bpf_program {}
pub enum bpf_link {}
pub enum bpf_map {}

#[repr(C)]
pub struct bpf_object_open_opts {
    _private: [u8; 0],
}

type __u32 = u32;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;

unsafe extern "C" {
    fn bpf_object__open_file(
        path: *const c_char,
        opts: *const bpf_object_open_opts,
    ) -> *mut bpf_object;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__value_size(map: *const bpf_map) -> usize;
    fn bpf_map__set_initial_value(map: *mut bpf_map, data: *const c_void, size: usize) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);

    fn getpid() -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn inet_addr(cp: *const c_char) -> c_uint;
    fn inet_ntoa(in_: in_addr) -> *mut c_char;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn close(fd: c_int) -> c_int;
}

unsafe extern "Rust" {
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: usize, right: usize, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn CHECK(cond: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_probe_user() {
    static PROG_NAME_0: &[u8] = b"handle_sys_connect\0";
    #[cfg(target_arch = "s390x")]
    static PROG_NAME_1: &[u8] = b"handle_sys_socketcall\0";

    #[cfg(target_arch = "s390x")]
    let prog_names: [*const c_char; 2] = [
        PROG_NAME_0.as_ptr() as *const c_char,
        PROG_NAME_1.as_ptr() as *const c_char,
    ];
    #[cfg(not(target_arch = "s390x"))]
    let prog_names: [*const c_char; 1] = [PROG_NAME_0.as_ptr() as *const c_char];

    let prog_count = prog_names.len();
    let obj_file = b"./test_probe_user.bpf.o\0".as_ptr() as *const c_char;
    let opts: bpf_object_open_opts = zeroed();
    let mut err: c_int;
    let mut results_map_fd: c_int;
    let sock_fd: c_int;
    let mut duration: c_int = 0;
    let _ = &mut duration;
    let mut curr: sockaddr = zeroed();
    let mut orig: sockaddr = zeroed();
    let mut tmp: sockaddr = zeroed();
    let mut in_: *mut sockaddr_in = &mut curr as *mut sockaddr as *mut sockaddr_in;
    let mut kprobe_links: Vec<*mut bpf_link> = vec![ptr::null_mut(); prog_count];
    let mut kprobe_progs: Vec<*mut bpf_program> = vec![ptr::null_mut(); prog_count];
    let obj: *mut bpf_object;
    static ZERO: c_int = 0;

    #[repr(C)]
    struct test_pro_bss {
        old: sockaddr_in,
        test_pid: __u32,
    }

    let mut results: test_pro_bss = zeroed();
    let mut i: usize;

    obj = bpf_object__open_file(obj_file, &opts);
    if !ASSERT_OK_PTR(obj, b"obj_open_file\0".as_ptr() as *const c_char) {
        return;
    }

    i = 0;
    while i < prog_count {
        kprobe_progs[i] = bpf_object__find_program_by_name(obj, prog_names[i]);
        if CHECK(
            kprobe_progs[i].is_null(),
            b"find_probe\0".as_ptr() as *const c_char,
            b"prog '%s' not found\n\0".as_ptr() as *const c_char,
            prog_names[i],
        ) {
            goto_cleanup(obj, &mut kprobe_links, prog_count);
            return;
        }
        i += 1;
    }

    {
        let mut bss_map: *mut bpf_map;
        let mut bss_init: test_pro_bss = zeroed();

        bss_init.test_pid = getpid() as __u32;
        bss_map = bpf_object__find_map_by_name(obj, b"test_pro.bss\0".as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(bss_map, b"find_bss_map\0".as_ptr() as *const c_char) {
            goto_cleanup(obj, &mut kprobe_links, prog_count);
            return;
        }
        if !ASSERT_EQ(
            bpf_map__value_size(bss_map),
            size_of::<test_pro_bss>(),
            b"bss_size\0".as_ptr() as *const c_char,
        ) {
            goto_cleanup(obj, &mut kprobe_links, prog_count);
            return;
        }
        err = bpf_map__set_initial_value(
            bss_map,
            &bss_init as *const test_pro_bss as *const c_void,
            size_of::<test_pro_bss>(),
        );
        if !ASSERT_OK(err, b"set_bss_init\0".as_ptr() as *const c_char) {
            goto_cleanup(obj, &mut kprobe_links, prog_count);
            return;
        }
    }

    err = bpf_object__load(obj);
    if CHECK(
        err != 0,
        b"obj_load\0".as_ptr() as *const c_char,
        b"err %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_cleanup(obj, &mut kprobe_links, prog_count);
        return;
    }

    results_map_fd = bpf_find_map(
        b"test_probe_user\0".as_ptr() as *const c_char,
        obj,
        b"test_pro.bss\0".as_ptr() as *const c_char,
    );
    if CHECK(
        results_map_fd < 0,
        b"find_bss_map\0".as_ptr() as *const c_char,
        b"err %d\n\0".as_ptr() as *const c_char,
        results_map_fd,
    ) {
        goto_cleanup(obj, &mut kprobe_links, prog_count);
        return;
    }

    i = 0;
    while i < prog_count {
        kprobe_links[i] = bpf_program__attach(kprobe_progs[i]);
        if !ASSERT_OK_PTR(kprobe_links[i], b"attach_kprobe\0".as_ptr() as *const c_char) {
            goto_cleanup(obj, &mut kprobe_links, prog_count);
            return;
        }
        i += 1;
    }

    memset(
        &mut curr as *mut sockaddr as *mut c_void,
        0,
        size_of::<sockaddr>(),
    );
    (*in_).sin_family = AF_INET as u16;
    (*in_).sin_port = htons(5555);
    (*in_).sin_addr.s_addr = inet_addr(b"255.255.255.255\0".as_ptr() as *const c_char);
    memcpy(
        &mut orig as *mut sockaddr as *mut c_void,
        &curr as *const sockaddr as *const c_void,
        size_of::<sockaddr>(),
    );

    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if CHECK(
        sock_fd < 0,
        b"create_sock_fd\0".as_ptr() as *const c_char,
        b"err %d\n\0".as_ptr() as *const c_char,
        sock_fd,
    ) {
        goto_cleanup(obj, &mut kprobe_links, prog_count);
        return;
    }

    connect(sock_fd, &curr, size_of::<sockaddr>() as u32);
    close(sock_fd);

    err = bpf_map_lookup_elem(
        results_map_fd,
        &ZERO as *const c_int as *const c_void,
        &mut results as *mut test_pro_bss as *mut c_void,
    );
    if CHECK(
        err != 0,
        b"get_kprobe_res\0".as_ptr() as *const c_char,
        b"failed to get kprobe res: %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_cleanup(obj, &mut kprobe_links, prog_count);
        return;
    }

    memcpy(
        &mut tmp as *mut sockaddr as *mut c_void,
        &results.old as *const sockaddr_in as *const c_void,
        size_of::<sockaddr>(),
    );

    in_ = &mut tmp as *mut sockaddr as *mut sockaddr_in;
    if CHECK(
        memcmp(
            &tmp as *const sockaddr as *const c_void,
            &orig as *const sockaddr as *const c_void,
            size_of::<sockaddr>(),
        ) != 0,
        b"check_kprobe_res\0".as_ptr() as *const c_char,
        b"wrong kprobe res from probe read: %s:%u\n\0".as_ptr() as *const c_char,
        inet_ntoa((*in_).sin_addr),
        ntohs((*in_).sin_port) as c_uint,
    ) {
        goto_cleanup(obj, &mut kprobe_links, prog_count);
        return;
    }

    memset(
        &mut tmp as *mut sockaddr as *mut c_void,
        0xab,
        size_of::<sockaddr>(),
    );

    in_ = &mut curr as *mut sockaddr as *mut sockaddr_in;
    if CHECK(
        memcmp(
            &curr as *const sockaddr as *const c_void,
            &tmp as *const sockaddr as *const c_void,
            size_of::<sockaddr>(),
        ) != 0,
        b"check_kprobe_res\0".as_ptr() as *const c_char,
        b"wrong kprobe res from probe write: %s:%u\n\0".as_ptr() as *const c_char,
        inet_ntoa((*in_).sin_addr),
        ntohs((*in_).sin_port) as c_uint,
    ) {
        goto_cleanup(obj, &mut kprobe_links, prog_count);
        return;
    }

    goto_cleanup(obj, &mut kprobe_links, prog_count);
}

unsafe fn goto_cleanup(obj: *mut bpf_object, kprobe_links: &mut [*mut bpf_link], prog_count: usize) {
    let mut i: usize = 0;

    while i < prog_count {
        bpf_link__destroy(kprobe_links[i]);
        i += 1;
    }
    bpf_object__close(obj);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
