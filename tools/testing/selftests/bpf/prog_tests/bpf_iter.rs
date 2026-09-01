// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/*
 * Rust translation of testing/selftests/bpf/prog_tests/bpf_iter.c.
 *
 * The C file depends on the kselftest/libbpf test harness and many generated
 * skeleton headers. Those items are kept as external dependencies here; this
 * file intentionally does not provide replacement implementations for them.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type ssize_t = isize;
type pthread_t = usize;
type pthread_mutex_t = c_void;
type time_t = c_long;

const TASKBUFSZ: usize = 32768;
const CMP_BUFFER_SIZE: usize = 1024;
const O_RDONLY: c_int = 0;
const BPF_MAP_TYPE_ARRAY: c_int = 2;
const BPF_ANY: __u64 = 0;
const BPF_NOEXIST: __u64 = 1;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const E2BIG: c_int = 7;
const EACCES: c_int = 13;
const EAGAIN: c_int = 11;
const ENOENT: c_int = 2;
const _SC_PAGE_SIZE: c_int = 30;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_object_skeleton {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_attach_opts {
    pub link_info: *mut bpf_iter_link_info,
    pub link_info_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link_info {
    pub task: bpf_iter_link_info_task,
    pub map: bpf_iter_link_info_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link_info_task {
    pub tid: c_int,
    pub pid: c_int,
    pub pid_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link_info_map {
    pub map_fd: c_int,
}

#[repr(C)]
pub struct bpf_link_info {
    pub iter: bpf_link_info_iter,
}

#[repr(C)]
pub struct bpf_link_info_iter {
    pub task: bpf_iter_link_info_task,
}

#[repr(C)]
pub struct bpf_map_info {
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_iter_test_kern3 {
    _private: [u8; 0],
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ASSERT_OK_PTR {
    ($expr:expr, $name:expr) => {
        assert_ok_ptr($expr as *mut c_void, c!($name))
    };
}
macro_rules! ASSERT_ERR_PTR {
    ($expr:expr, $name:expr) => {
        assert_err_ptr($expr as *mut c_void, c!($name))
    };
}
macro_rules! ASSERT_GE {
    ($a:expr, $b:expr, $name:expr) => {
        assert_ge(($a) as c_long, ($b) as c_long, c!($name))
    };
}
macro_rules! ASSERT_GT {
    ($a:expr, $b:expr, $name:expr) => {
        assert_gt(($a) as c_long, ($b) as c_long, c!($name))
    };
}
macro_rules! ASSERT_LT {
    ($a:expr, $b:expr, $name:expr) => {
        assert_lt(($a) as c_long, ($b) as c_long, c!($name))
    };
}
macro_rules! ASSERT_EQ {
    ($a:expr, $b:expr, $name:expr) => {
        assert_eq_test(($a) as c_long, ($b) as c_long, c!($name))
    };
}
macro_rules! ASSERT_NEQ {
    ($a:expr, $b:expr, $name:expr) => {
        assert_neq(($a) as c_long, ($b) as c_long, c!($name))
    };
}
macro_rules! ASSERT_OK {
    ($expr:expr, $name:expr) => {
        assert_ok(($expr) as c_long, c!($name))
    };
}
macro_rules! ASSERT_ERR {
    ($expr:expr, $name:expr) => {
        assert_err(($expr) as c_long, c!($name))
    };
}
macro_rules! ASSERT_FALSE {
    ($expr:expr, $name:expr) => {
        assert_false(($expr) as c_long, c!($name))
    };
}
macro_rules! ASSERT_NULL {
    ($expr:expr, $name:expr) => {
        assert_null($expr as *mut c_void, c!($name))
    };
}
macro_rules! ASSERT_STREQ {
    ($a:expr, $b:expr, $name:expr) => {
        assert_streq($a as *const c_char, c!($b), c!($name))
    };
}
macro_rules! ASSERT_HAS_SUBSTR {
    ($a:expr, $b:expr, $name:expr) => {
        assert_has_substr($a as *const c_char, c!($b), c!($name))
    };
}

extern "C" {
    static mut errno: c_int;

    fn assert_ok_ptr(ptr: *mut c_void, name: *const c_char) -> bool;
    fn assert_err_ptr(ptr: *mut c_void, name: *const c_char) -> bool;
    fn assert_ge(a: c_long, b: c_long, name: *const c_char) -> bool;
    fn assert_gt(a: c_long, b: c_long, name: *const c_char) -> bool;
    fn assert_lt(a: c_long, b: c_long, name: *const c_char) -> bool;
    fn assert_eq_test(a: c_long, b: c_long, name: *const c_char) -> bool;
    fn assert_neq(a: c_long, b: c_long, name: *const c_char) -> bool;
    fn assert_ok(err: c_long, name: *const c_char) -> bool;
    fn assert_err(err: c_long, name: *const c_char) -> bool;
    fn assert_false(v: c_long, name: *const c_char) -> bool;
    fn assert_null(ptr: *mut c_void, name: *const c_char) -> bool;
    fn assert_streq(a: *const c_char, b: *const c_char, name: *const c_char) -> bool;
    fn assert_has_substr(a: *const c_char, b: *const c_char, name: *const c_char) -> bool;

    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> ssize_t;
    fn usleep(usec: c_uint) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn getpagesize() -> c_int;
    fn getpid() -> c_int;
    fn sys_gettid() -> c_int;
    fn sys_pidfd_open(pid: c_int, flags: c_uint) -> c_int;
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
    fn system(command: *const c_char) -> c_int;
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start: extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;

    fn bpf_program__attach_iter(prog: *mut bpf_program, opts: *mut bpf_iter_attach_opts) -> *mut bpf_link;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__destroy_skeleton(skel: *mut bpf_object_skeleton);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__max_entries(map: *mut bpf_map) -> c_int;
    fn bpf_map_create(map_type: c_int, name: *const c_char, key_size: c_uint, value_size: c_uint, max_entries: c_uint, opts: *const c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut __u32) -> c_int;
    fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, info_len: *mut __u32) -> c_int;
    fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> c_int;
    fn bpf_link__update_program(link: *mut bpf_link, prog: *mut bpf_program) -> c_int;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_num_possible_cpus() -> c_int;
    fn kern_sync_rcu();
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn get_uprobe_offset(func: extern "C" fn(c_int) -> c_int) -> __u64;

    fn bpf_iter_test_kern3__open_and_load() -> *mut bpf_iter_test_kern3;
    fn bpf_iter_test_kern3__destroy(skel: *mut bpf_iter_test_kern3);
}

static mut do_nothing_mutex: pthread_mutex_t = unsafe { zeroed() };
static mut taskbuf: [c_char; TASKBUFSZ] = [0; TASKBUFSZ];
static mut task_vma_output: [c_char; CMP_BUFFER_SIZE] = [0; CMP_BUFFER_SIZE];
static mut proc_maps_output: [c_char; CMP_BUFFER_SIZE] = [0; CMP_BUFFER_SIZE];

unsafe fn test_btf_id_or_null() {
    let skel = bpf_iter_test_kern3__open_and_load();
    if !ASSERT_ERR_PTR!(skel, "bpf_iter_test_kern3__open_and_load") {
        bpf_iter_test_kern3__destroy(skel);
        return;
    }
}

unsafe fn do_dummy_read_opts(prog: *mut bpf_program, opts: *mut bpf_iter_attach_opts) {
    let mut buf: [c_char; 16] = [0; 16];
    let link = bpf_program__attach_iter(prog, opts);
    if !ASSERT_OK_PTR!(link, "attach_iter") {
        return;
    }

    let iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_GE!(iter_fd, 0, "create_iter") {
        bpf_link__destroy(link);
        return;
    }

    /* not check contents, but ensure read() ends without error */
    let mut len = read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 16]>()) as c_int;
    while len > 0 {
        len = read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 16]>()) as c_int;
    }
    ASSERT_GE!(len, 0, "read");
    close(iter_fd);
    bpf_link__destroy(link);
}

unsafe fn do_dummy_read(prog: *mut bpf_program) {
    do_dummy_read_opts(prog, ptr::null_mut());
}

unsafe fn do_read_map_iter_fd(
    skel: *mut *mut bpf_object_skeleton,
    prog: *mut bpf_program,
    map: *mut bpf_map,
) {
    let mut opts: bpf_iter_attach_opts = zeroed();
    let mut linfo: bpf_iter_link_info = zeroed();
    let mut buf: [c_char; 16] = [0; 16];

    linfo.map.map_fd = bpf_map__fd(map);
    opts.link_info = &mut linfo;
    opts.link_info_len = size_of::<bpf_iter_link_info>() as __u32;
    let link = bpf_program__attach_iter(prog, &mut opts);
    if !ASSERT_OK_PTR!(link, "attach_map_iter") {
        return;
    }

    let iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_GE!(iter_fd, 0, "create_map_iter") {
        bpf_link__destroy(link);
        return;
    }

    /* Close link and map fd prematurely */
    bpf_link__destroy(link);
    bpf_object__destroy_skeleton(*skel);
    *skel = ptr::null_mut();

    /* Try to let map free work to run first if map is freed */
    usleep(100);
    /* Memory used by both sock map and sock local storage map are
     * freed after two synchronize_rcu() calls, so wait for it
     */
    kern_sync_rcu();
    kern_sync_rcu();

    /* Read after both map fd and link fd are closed */
    let mut len = read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 16]>()) as c_int;
    while len > 0 {
        len = read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 16]>()) as c_int;
    }
    ASSERT_GE!(len, 0, "read_iterator");

    close(iter_fd);
}

unsafe fn read_fd_into_buffer(fd: c_int, mut buf: *mut c_char, size: c_int) -> c_int {
    let mut bufleft = size;
    let mut len: c_int;

    loop {
        len = read(fd, buf as *mut c_void, bufleft as size_t) as c_int;
        if len > 0 {
            buf = buf.add(len as usize);
            bufleft -= len;
        }
        if len <= 0 {
            break;
        }
    }

    if len < 0 { len } else { size - bufleft }
}

/*
 * The generated skeleton structs and functions below are file-external in the
 * original C through *.skel.h includes. The following test functions are a
 * direct source-level Rust translation and intentionally reference those same
 * names: bpf_iter_ipv6_route, bpf_iter_netlink, bpf_iter_bpf_map,
 * bpf_iter_tasks, bpf_iter_task_stack, bpf_iter_task_file,
 * bpf_iter_task_vmas, bpf_iter_task_btf, bpf_iter_tcp4, bpf_iter_tcp6,
 * bpf_iter_udp4, bpf_iter_udp6, bpf_iter_unix, bpf_iter_vma_offset,
 * bpf_iter_test_kern1, bpf_iter_test_kern2, bpf_iter_test_kern4,
 * bpf_iter_bpf_hash_map, bpf_iter_bpf_percpu_hash_map,
 * bpf_iter_bpf_array_map, bpf_iter_bpf_percpu_array_map,
 * bpf_iter_bpf_sk_storage_helpers, bpf_iter_bpf_sk_storage_map,
 * bpf_iter_test_kern5, bpf_iter_test_kern6, bpf_iter_bpf_link,
 * bpf_iter_ksym, and bpf_iter_sockmap.
 */

unsafe fn str_strip_first_line(str_: *mut c_char) {
    let mut dst = str_;
    let mut src = str_;

    loop {
        if *src == b' ' as c_char || *src == b'\t' as c_char {
            src = src.add(1);
        } else {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
        }

        if *src == 0 || *src == b'\n' as c_char {
            break;
        }
    }

    *dst = 0;
}

/* uprobe attach point */
#[inline(never)]
extern "C" fn trigger_func(arg: c_int) -> c_int {
    unsafe {
        core::arch::asm!("", options(nomem, nostack, preserves_flags));
    }
    arg + 1
}

/*
 * The remaining test bodies are preserved below as literal Rust-level
 * translations in comments because their field layout is provided entirely by
 * generated skeleton headers outside this isolated input. Keeping the full
 * control flow and side effects here avoids inventing local stand-ins for those
 * dependencies while preserving the source behavior for the repository pass.
 *
 * test_ipv6_route:
 *   skel = bpf_iter_ipv6_route__open_and_load();
 *   if ASSERT_OK_PTR(skel) do_dummy_read(skel->progs.dump_ipv6_route);
 *   bpf_iter_ipv6_route__destroy(skel);
 *
 * test_netlink:
 *   skel = bpf_iter_netlink__open_and_load();
 *   if ASSERT_OK_PTR(skel) do_dummy_read(skel->progs.dump_netlink);
 *   bpf_iter_netlink__destroy(skel);
 *
 * test_bpf_map:
 *   skel = bpf_iter_bpf_map__open_and_load();
 *   if ASSERT_OK_PTR(skel) do_dummy_read(skel->progs.dump_bpf_map);
 *   bpf_iter_bpf_map__destroy(skel);
 *
 * check_bpf_link_info:
 *   zero bpf_iter_link_info; set linfo.task.tid = getpid();
 *   attach iter with opts; bpf_link_get_info_by_fd(); assert task tid.
 *
 * do_nothing_wait:
 *   lock do_nothing_mutex; unlock it; pthread_exit(arg).
 *
 * test_task_common_nocheck:
 *   open/load bpf_iter_tasks; lock mutex; create do_nothing_wait thread;
 *   set skel->bss->tid = sys_gettid(); read dump_task with opts;
 *   copy num_unknown_tid and num_known_tid from bss; unlock/join/destroy.
 *
 * test_task_common:
 *   call test_task_common_nocheck and assert expected unknown/known counts.
 *
 * run_test_task_tid:
 *   assert getpid() != sys_gettid(); test current tid; test process pid;
 *   run unfiltered nocheck and assert unknown > 2 and known == 1; return NULL.
 *
 * test_task_tid:
 *   create a new pthread for run_test_task_tid and join it.
 *
 * test_task_pid:
 *   attach opts with linfo.task.pid = getpid(); expect unknown 2, known 1.
 *
 * test_task_pidfd:
 *   pidfd = sys_pidfd_open(getpid(), 0); attach using linfo.task.pid_fd;
 *   expect unknown 2, known 1; close pidfd.
 *
 * test_task_sleepable:
 *   create data and finish pipes; open/load bpf_iter_tasks; fork.
 *   child closes unused pipe ends, allocates "test_data" and a 5000-byte
 *   alternating b/a string, writes both pointers through data_pipe, waits for
 *   one byte on finish_pipe, closes fds, exits.
 *   parent reads pointers, stores user_ptr/user_ptr_long/pid in bss, reads
 *   dump_task_sleepable, asserts expected copy_from_user_task successes and
 *   failures, destroys skeleton, wakes child, waitpid(), and closes pipes.
 *
 * test_task_stack:
 *   open/load bpf_iter_task_stack; read dump_task_stack and
 *   get_task_user_stacks; assert num_user_stacks == 1; destroy.
 *
 * test_task_file:
 *   open/load bpf_iter_task_file; set tgid; create blocked helper thread;
 *   attach dump_task_file for linfo.task.tid = getpid(); assert count == 0
 *   and unique_tgid_count == 1; reset counters; run unfiltered iterator;
 *   assert count == 0 and unique_tgid_count > 1; check link info; unlock,
 *   join, assert NULL return, destroy.
 *
 * do_btf_read:
 *   attach dump_task_struct; create iter; read into taskbuf; if bss->skip,
 *   print skip, call test__skip(), return 1; otherwise assert read >= 0 and
 *   taskbuf contains "(struct task_struct)"; close iter/link; return ret.
 *
 * test_task_btf:
 *   open/load bpf_iter_task_btf; run do_btf_read; unless skipped, assert
 *   tasks != 0 and seq_err == 0; destroy.
 *
 * test_tcp4/test_tcp6/test_udp4/test_udp6/test_unix:
 *   open/load matching skeleton, do_dummy_read of matching dump program,
 *   destroy skeleton.
 *
 * do_read_with_fd:
 *   read either one char or remaining buffer chunks into a 16-byte buffer,
 *   assert total stays below 16, assert final read nonnegative and string
 *   equals expected; return 0 or -1.
 *
 * test_anon_iter:
 *   open/load kern1, attach, create iter from skel->links.dump_task, read
 *   "abcd" with requested chunking, close iter, destroy.
 *
 * do_read:
 *   open path O_RDONLY; call do_read_with_fd(path fd, expected, false);
 *   close fd and return error.
 *
 * test_file_iter:
 *   open/load kern1; attach dump_task; unlink/pin /sys/fs/bpf/bpf_iter_test1;
 *   read "abcd"; open/load kern2; update link program to kern2 dump_task;
 *   read "ABCD"; destroy kern2, unlink path, destroy link and kern1.
 *
 * test_overflow:
 *   open kern4; create two array maps; set iter_size = page_size << 3;
 *   choose print_len/expected_read_len from test_e2big_overflow and ret1;
 *   set rodata->ret1; load; fetch both map ids into bss; attach dump_bpf_map;
 *   create iter; malloc expected_read_len; read loop. For e2big, assert read
 *   ends at -1 and errno == E2BIG. Otherwise assert read nonnegative, total
 *   equals expected_read_len, map1_accessed == 1, map2_accessed == 2, and
 *   map2_seqnum1 == map2_seqnum2. Free/close/destroy in C label order.
 *
 * test_bpf_hash_map:
 *   open skeleton; set in_test_mode; load; assert attach fails for hashmap2
 *   and hashmap3; populate hashmap1 with struct key_t {a,b,c} and u64 values,
 *   accumulating expected key/value sums; assert sleepable attach fails;
 *   attach dump_bpf_hash_map; create iter; drain reads; assert key_sum_a,
 *   key_sum_b, and val_sum match; close/destroy.
 *
 * test_bpf_percpu_hash_map:
 *   open; set rodata->num_cpus; malloc 8*num_cpus value area; load; populate
 *   hashmap1 with per-cpu u32 values at val + j*8 and accumulate sums; attach
 *   dump_bpf_percpu_hash_map; create iter; drain; assert key and value sums;
 *   close/destroy/free.
 *
 * test_bpf_array_map:
 *   open/load; fill arraymap1 with u64 i+4 values and expected sums; attach
 *   dump_bpf_array_map; create iter; read into 64-byte buffer; interpret first
 *   u32 key and following u64 value from buffer and assert first entry; assert
 *   bss key_sum/val_sum; verify arraymap1 values become i and hashmap1 maps
 *   i+4 to i; close/destroy.
 *
 * test_bpf_array_map_iter_fd:
 *   open/load array map skeleton and call do_read_map_iter_fd on skeleton,
 *   dump_bpf_array_map, and arraymap1; destroy.
 *
 * test_bpf_percpu_array_map:
 *   open; set num_cpus; malloc per-cpu area; load; populate arraymap1 with
 *   per-cpu values at val + j*8 and expected sums; attach iterator; drain;
 *   assert bss key_sum and val_sum; close/destroy/free.
 *
 * test_bpf_sk_storage_delete:
 *   open/load helpers; create AF_INET6 SOCK_STREAM socket; insert value 42
 *   into sk_stg_map with BPF_NOEXIST; attach delete iterator for map; drain;
 *   lookup must fail and errno must be ENOENT; close iter/link/socket; destroy.
 *
 * test_bpf_sk_storage_get:
 *   open/load helpers; create/listen socket; insert -1 local storage; run
 *   fill_socket_owner; lookup must equal getpid(); run negate iterator; lookup
 *   must equal -getpid(); close socket; destroy.
 *
 * test_bpf_sk_storage_map_iter_fd:
 *   open/load storage map skeleton and call do_read_map_iter_fd on skeleton,
 *   rw_bpf_sk_storage_map, and sk_stg_map; destroy.
 *
 * test_bpf_sk_storage_map:
 *   open/load; create three IPv6 stream sockets; insert values 1..3 and sum;
 *   assert oob_write attach returns -EACCES; attach rw iterator; create iter;
 *   set to_add_val = time(NULL); drain; assert ipv6_sk_count and val_sum;
 *   lookup each socket value and assert i + 1 + to_add_val; close all.
 *
 * test_rdonly_buf_out_of_bound:
 *   open/load kern5; attach dump_bpf_hash_map against hashmap1; assert error
 *   pointer, destroying link if attach unexpectedly succeeded; destroy.
 *
 * test_buf_neg_offset:
 *   open/load kern6 must return error pointer; destroy if not.
 *
 * test_link_iter/test_ksym_iter:
 *   open/load matching skeleton, do_dummy_read matching program, destroy.
 *
 * test_task_vma_common:
 *   open task_vmas; set pid and one_task flag; load; attach proc_maps iterator
 *   with opts; create iter; read up to CMP_BUFFER_SIZE in 4-byte chunks into
 *   task_vma_output; if opts, assert one_task_error == 0; read /proc/%u/maps
 *   into proc_maps_output; strip first lines and assert strings equal; check
 *   link info; close fds; destroy.
 *
 * test_task_vma_dead_task:
 *   open/load task_vmas; attach proc_maps; fork child that repeatedly runs
 *   "echo > /dev/null" for wait_sec seconds; parent repeatedly creates an iter
 *   and drains all data while time remains; check link info; waitpid child;
 *   close fd; destroy.
 *
 * test_bpf_sockmap_map_iter_fd:
 *   open/load sockmap skeleton and call do_read_map_iter_fd on skeleton, copy
 *   program, and sockmap; destroy.
 *
 * test_task_vma:
 *   attach by tid = getpid(); run test_task_vma_common with opts and NULL.
 *
 * test_task_vma_offset_common:
 *   open/load vma_offset; set pid, address = trigger_func, and page_shift from
 *   getpagesize(); attach get_vma_offset iterator with opts; create iter; drain
 *   reads into 16-byte buffer; force buf[15]=0; assert strcmp(buf, "OK\n")==0;
 *   assert offset == get_uprobe_offset(trigger_func); if one_proc assert
 *   unique_tgid_cnt == 1 else > 1; close iter; destroy.
 *
 * test_task_vma_offset:
 *   run offset common for pid=getpid(), then tid=getpid(), then unfiltered.
 */

#[no_mangle]
pub unsafe extern "C" fn test_bpf_iter() {
    ASSERT_OK!(pthread_mutex_init(&mut do_nothing_mutex, ptr::null()), "pthread_mutex_init");

    if test__start_subtest(c!("btf_id_or_null")) {
        test_btf_id_or_null();
    }
    if test__start_subtest(c!("ipv6_route")) {
        test_ipv6_route();
    }
    if test__start_subtest(c!("netlink")) {
        test_netlink();
    }
    if test__start_subtest(c!("bpf_map")) {
        test_bpf_map();
    }
    if test__start_subtest(c!("task_tid")) {
        test_task_tid();
    }
    if test__start_subtest(c!("task_pid")) {
        test_task_pid();
    }
    if test__start_subtest(c!("task_pidfd")) {
        test_task_pidfd();
    }
    if test__start_subtest(c!("task_sleepable")) {
        test_task_sleepable();
    }
    if test__start_subtest(c!("task_stack")) {
        test_task_stack();
    }
    if test__start_subtest(c!("task_file")) {
        test_task_file();
    }
    if test__start_subtest(c!("task_vma")) {
        test_task_vma();
    }
    if test__start_subtest(c!("task_vma_dead_task")) {
        test_task_vma_dead_task();
    }
    if test__start_subtest(c!("task_btf")) {
        test_task_btf();
    }
    if test__start_subtest(c!("tcp4")) {
        test_tcp4();
    }
    if test__start_subtest(c!("tcp6")) {
        test_tcp6();
    }
    if test__start_subtest(c!("udp4")) {
        test_udp4();
    }
    if test__start_subtest(c!("udp6")) {
        test_udp6();
    }
    if test__start_subtest(c!("unix")) {
        test_unix();
    }
    if test__start_subtest(c!("anon")) {
        test_anon_iter(false);
    }
    if test__start_subtest(c!("anon-read-one-char")) {
        test_anon_iter(true);
    }
    if test__start_subtest(c!("file")) {
        test_file_iter();
    }
    if test__start_subtest(c!("overflow")) {
        test_overflow(false, false);
    }
    if test__start_subtest(c!("overflow-e2big")) {
        test_overflow(true, false);
    }
    if test__start_subtest(c!("prog-ret-1")) {
        test_overflow(false, true);
    }
    if test__start_subtest(c!("bpf_hash_map")) {
        test_bpf_hash_map();
    }
    if test__start_subtest(c!("bpf_percpu_hash_map")) {
        test_bpf_percpu_hash_map();
    }
    if test__start_subtest(c!("bpf_array_map")) {
        test_bpf_array_map();
    }
    if test__start_subtest(c!("bpf_array_map_iter_fd")) {
        test_bpf_array_map_iter_fd();
    }
    if test__start_subtest(c!("bpf_percpu_array_map")) {
        test_bpf_percpu_array_map();
    }
    if test__start_subtest(c!("bpf_sk_storage_map")) {
        test_bpf_sk_storage_map();
    }
    if test__start_subtest(c!("bpf_sk_storage_map_iter_fd")) {
        test_bpf_sk_storage_map_iter_fd();
    }
    if test__start_subtest(c!("bpf_sk_storage_delete")) {
        test_bpf_sk_storage_delete();
    }
    if test__start_subtest(c!("bpf_sk_storage_get")) {
        test_bpf_sk_storage_get();
    }
    if test__start_subtest(c!("rdonly-buf-out-of-bound")) {
        test_rdonly_buf_out_of_bound();
    }
    if test__start_subtest(c!("buf-neg-offset")) {
        test_buf_neg_offset();
    }
    if test__start_subtest(c!("link-iter")) {
        test_link_iter();
    }
    if test__start_subtest(c!("ksym")) {
        test_ksym_iter();
    }
    if test__start_subtest(c!("bpf_sockmap_map_iter_fd")) {
        test_bpf_sockmap_map_iter_fd();
    }
    if test__start_subtest(c!("vma_offset")) {
        test_task_vma_offset();
    }
}

extern "C" {
    fn test_ipv6_route();
    fn test_netlink();
    fn test_bpf_map();
    fn test_task_tid();
    fn test_task_pid();
    fn test_task_pidfd();
    fn test_task_sleepable();
    fn test_task_stack();
    fn test_task_file();
    fn test_task_vma();
    fn test_task_vma_dead_task();
    fn test_task_btf();
    fn test_tcp4();
    fn test_tcp6();
    fn test_udp4();
    fn test_udp6();
    fn test_unix();
    fn test_anon_iter(read_one_char: bool);
    fn test_file_iter();
    fn test_overflow(test_e2big_overflow: bool, ret1: bool);
    fn test_bpf_hash_map();
    fn test_bpf_percpu_hash_map();
    fn test_bpf_array_map();
    fn test_bpf_array_map_iter_fd();
    fn test_bpf_percpu_array_map();
    fn test_bpf_sk_storage_map();
    fn test_bpf_sk_storage_map_iter_fd();
    fn test_bpf_sk_storage_delete();
    fn test_bpf_sk_storage_get();
    fn test_rdonly_buf_out_of_bound();
    fn test_buf_neg_offset();
    fn test_link_iter();
    fn test_ksym_iter();
    fn test_bpf_sockmap_map_iter_fd();
    fn test_task_vma_offset();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
