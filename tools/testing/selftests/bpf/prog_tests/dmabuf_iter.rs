// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Google */

/*
 * Translated from C source:
 * testing/selftests/bpf/prog_tests/dmabuf_iter.c
 *
 * C includes translated as external dependencies:
 * test_progs.h, bpf/libbpf.h, bpf/btf.h, dmabuf_iter.skel.h,
 * fcntl.h, stdbool.h, stdio.h, stdlib.h, string.h, sys/ioctl.h,
 * sys/mman.h, unistd.h, linux/dma-buf.h, linux/dma-heap.h,
 * linux/udmabuf.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type FILE = c_void;

const DMA_BUF_NAME_LEN: usize = 32;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 0o2;
const O_CLOEXEC: c_int = 0o2000000;
const MFD_ALLOW_SEALING: c_uint = 0x0002;
const F_ADD_SEALS: c_int = 1033;
const F_SEAL_SHRINK: c_int = 0x0002;
const UDMABUF_FLAGS_CLOEXEC: u32 = 0x01;
const BPF_ANY: u64 = 0;

type c_uint = u32;

/* Ioctl request values are supplied by Linux UAPI headers in the original C. */
extern "C" {
    static UDMABUF_CREATE: c_ulong;
    static DMA_BUF_SET_NAME_B: c_ulong;
    static DMA_HEAP_IOCTL_ALLOC: c_ulong;
}

#[repr(C)]
struct udmabuf_create {
    memfd: c_uint,
    flags: c_uint,
    offset: u64,
    size: u64,
}

#[repr(C)]
struct dma_heap_allocation_data {
    len: u64,
    fd: u32,
    fd_flags: u32,
    heap_flags: u64,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: size_t,
    retval: u32,
}

#[repr(C)]
struct dmabuf_iter {
    links: dmabuf_iter_links,
    progs: dmabuf_iter_progs,
    maps: dmabuf_iter_maps,
}

#[repr(C)]
struct dmabuf_iter_links {
    dmabuf_collector: *mut c_void,
}

#[repr(C)]
struct dmabuf_iter_progs {
    iter_dmabuf_for_each: *mut c_void,
}

#[repr(C)]
struct dmabuf_iter_maps {
    testbuf_hash: *mut c_void,
}

extern "C" {
    fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn getpagesize() -> c_int;
    fn ftruncate(fd: c_int, length: c_long) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn fdopen(fd: c_int, modes: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn fclose(stream: *mut FILE) -> c_int;

    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_link__fd(link: *mut c_void) -> c_int;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map__fd(map: *mut c_void) -> c_int;

    fn dmabuf_iter__open_and_load() -> *mut dmabuf_iter;
    fn dmabuf_iter__attach(skel: *mut dmabuf_iter) -> c_int;
    fn dmabuf_iter__destroy(skel: *mut dmabuf_iter);

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_LE(a: size_t, b: size_t, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(a: c_long, b: c_long, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;
    fn ASSERT_GT(a: size_t, b: size_t, name: *const c_char) -> bool;
}

static mut udmabuf: c_int = -1;
static udmabuf_test_buffer_name: [c_char; DMA_BUF_NAME_LEN] = cstr32(b"udmabuf_test_buffer_for_iter\0");
static mut udmabuf_test_buffer_size: size_t = 0;
static mut sysheap_dmabuf: c_int = -1;
static sysheap_test_buffer_name: [c_char; DMA_BUF_NAME_LEN] = cstr32(b"sysheap_test_buffer_for_iter\0");
static mut sysheap_test_buffer_size: size_t = 0;

const fn cstr32(bytes: &[u8]) -> [c_char; DMA_BUF_NAME_LEN] {
    let mut out = [0 as c_char; DMA_BUF_NAME_LEN];
    let mut i = 0;
    while i < bytes.len() && i < DMA_BUF_NAME_LEN {
        out[i] = bytes[i] as c_char;
        i += 1;
    }
    out
}

unsafe fn create_udmabuf() -> c_int {
    let mut create: udmabuf_create;
    let dev_udmabuf: c_int;
    let memfd: c_int;
    let local_udmabuf: c_int;

    udmabuf_test_buffer_size = (10 * getpagesize()) as size_t;

    if !ASSERT_LE(
        size_of::<[c_char; DMA_BUF_NAME_LEN]>(),
        DMA_BUF_NAME_LEN,
        b"NAMETOOLONG\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    memfd = memfd_create(b"memfd_test\0".as_ptr() as *const c_char, MFD_ALLOW_SEALING);
    if !ASSERT_OK_FD(memfd, b"memfd_create\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_OK(
        ftruncate(memfd, udmabuf_test_buffer_size as c_long),
        b"ftruncate\0".as_ptr() as *const c_char,
    ) {
        close(memfd);
        return -1;
    }

    if !ASSERT_OK(
        fcntl(memfd, F_ADD_SEALS, F_SEAL_SHRINK),
        b"seal\0".as_ptr() as *const c_char,
    ) {
        close(memfd);
        return -1;
    }

    dev_udmabuf = open(b"/dev/udmabuf\0".as_ptr() as *const c_char, O_RDONLY);
    if !ASSERT_OK_FD(dev_udmabuf, b"open udmabuf\0".as_ptr() as *const c_char) {
        close(memfd);
        return -1;
    }

    create = zeroed();
    memset(
        &mut create as *mut _ as *mut c_void,
        0,
        size_of::<udmabuf_create>(),
    );
    create.memfd = memfd as c_uint;
    create.flags = UDMABUF_FLAGS_CLOEXEC;
    create.offset = 0;
    create.size = udmabuf_test_buffer_size as u64;

    local_udmabuf = ioctl(dev_udmabuf, UDMABUF_CREATE, &mut create);
    close(dev_udmabuf);
    if !ASSERT_OK_FD(local_udmabuf, b"udmabuf_create\0".as_ptr() as *const c_char) {
        close(memfd);
        return -1;
    }

    if !ASSERT_OK(
        ioctl(local_udmabuf, DMA_BUF_SET_NAME_B, udmabuf_test_buffer_name.as_ptr()),
        b"name\0".as_ptr() as *const c_char,
    ) {
        close(local_udmabuf);
        close(memfd);
        return -1;
    }

    local_udmabuf
}

unsafe fn create_sys_heap_dmabuf(bytes: size_t) -> c_int {
    let mut data = dma_heap_allocation_data {
        len: bytes as u64,
        fd: 0,
        fd_flags: (O_RDWR | O_CLOEXEC) as u32,
        heap_flags: 0,
    };
    let heap_fd: c_int;
    let ret: c_int;

    if !ASSERT_LE(
        size_of::<[c_char; DMA_BUF_NAME_LEN]>(),
        DMA_BUF_NAME_LEN,
        b"NAMETOOLONG\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    heap_fd = open(b"/dev/dma_heap/system\0".as_ptr() as *const c_char, O_RDONLY);
    if !ASSERT_OK_FD(heap_fd, b"open dma heap\0".as_ptr() as *const c_char) {
        return -1;
    }

    ret = ioctl(heap_fd, DMA_HEAP_IOCTL_ALLOC, &mut data);
    close(heap_fd);
    if !ASSERT_OK(ret, b"syheap alloc\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_OK(
        ioctl(data.fd as c_int, DMA_BUF_SET_NAME_B, sysheap_test_buffer_name.as_ptr()),
        b"name\0".as_ptr() as *const c_char,
    ) {
        close(data.fd as c_int);
        return -1;
    }

    data.fd as c_int
}

unsafe fn create_test_buffers() -> c_int {
    udmabuf = create_udmabuf();

    sysheap_test_buffer_size = (20 * getpagesize()) as size_t;
    sysheap_dmabuf = create_sys_heap_dmabuf(sysheap_test_buffer_size);

    if udmabuf < 0 || sysheap_dmabuf < 0 {
        return -1;
    }

    0
}

unsafe fn destroy_test_buffers() {
    close(udmabuf);
    udmabuf = -1;

    close(sysheap_dmabuf);
    sysheap_dmabuf = -1;
}

#[repr(C)]
enum Fields {
    INODE,
    SIZE,
    NAME,
    EXPORTER,
    FIELD_COUNT,
}

#[repr(C)]
struct DmabufInfo {
    inode: c_ulong,
    size: c_ulong,
    name: [c_char; DMA_BUF_NAME_LEN],
    exporter: [c_char; 32],
}

unsafe fn check_dmabuf_info(
    bufinfo: *const DmabufInfo,
    size: c_ulong,
    name: *const c_char,
    exporter: *const c_char,
) -> bool {
    size == (*bufinfo).size
        && strcmp(name, (*bufinfo).name.as_ptr()) == 0
        && strcmp(exporter, (*bufinfo).exporter.as_ptr()) == 0
}

unsafe fn subtest_dmabuf_iter_check_no_infinite_reads(skel: *mut dmabuf_iter) {
    let iter_fd: c_int;
    let mut buf = [0 as c_char; 256];

    iter_fd = bpf_iter_create(bpf_link__fd((*skel).links.dmabuf_collector));
    if !ASSERT_OK_FD(iter_fd, b"iter_create\0".as_ptr() as *const c_char) {
        return;
    }

    while read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) > 0 {
        /* Read out all contents */
    }

    /* Next reads should return 0 */
    ASSERT_EQ(
        read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) as c_long,
        0,
        b"read\0".as_ptr() as *const c_char,
    );

    close(iter_fd);
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

unsafe fn subtest_dmabuf_iter_check_default_iter(skel: *mut dmabuf_iter) {
    let mut found_test_sysheap_dmabuf = false;
    let mut found_test_udmabuf = false;
    let mut bufinfo: DmabufInfo = zeroed();
    let mut linesize: size_t = 0;
    let mut line: *mut c_char = ptr::null_mut();
    let iter_file: *mut FILE;
    let iter_fd: c_int;
    let mut f: c_int = Fields::INODE as c_int;

    iter_fd = bpf_iter_create(bpf_link__fd((*skel).links.dmabuf_collector));
    if !ASSERT_OK_FD(iter_fd, b"iter_create\0".as_ptr() as *const c_char) {
        return;
    }

    iter_file = fdopen(iter_fd, b"r\0".as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(iter_file as *const c_void, b"fdopen\0".as_ptr() as *const c_char) {
        close(iter_fd);
        return;
    }

    while getline(&mut line, &mut linesize, iter_file) != -1 {
        if f % Fields::FIELD_COUNT as c_int == Fields::INODE as c_int {
            ASSERT_EQ(
                sscanf(
                    line,
                    b"%ld\0".as_ptr() as *const c_char,
                    &mut bufinfo.inode,
                ) as c_long,
                1,
                b"read inode\0".as_ptr() as *const c_char,
            );
        } else if f % Fields::FIELD_COUNT as c_int == Fields::SIZE as c_int {
            ASSERT_EQ(
                sscanf(
                    line,
                    b"%ld\0".as_ptr() as *const c_char,
                    &mut bufinfo.size,
                ) as c_long,
                1,
                b"read size\0".as_ptr() as *const c_char,
            );
        } else if f % Fields::FIELD_COUNT as c_int == Fields::NAME as c_int {
            ASSERT_EQ(
                sscanf(
                    line,
                    b"%s\0".as_ptr() as *const c_char,
                    bufinfo.name.as_mut_ptr(),
                ) as c_long,
                1,
                b"read name\0".as_ptr() as *const c_char,
            );
        } else if f % Fields::FIELD_COUNT as c_int == Fields::EXPORTER as c_int {
            ASSERT_EQ(
                sscanf(
                    line,
                    b"%31s\0".as_ptr() as *const c_char,
                    bufinfo.exporter.as_mut_ptr(),
                ) as c_long,
                1,
                b"read exporter\0".as_ptr() as *const c_char,
            );

            if check_dmabuf_info(
                &bufinfo,
                sysheap_test_buffer_size as c_ulong,
                sysheap_test_buffer_name.as_ptr(),
                b"system\0".as_ptr() as *const c_char,
            ) {
                found_test_sysheap_dmabuf = true;
            } else if check_dmabuf_info(
                &bufinfo,
                udmabuf_test_buffer_size as c_ulong,
                udmabuf_test_buffer_name.as_ptr(),
                b"udmabuf\0".as_ptr() as *const c_char,
            ) {
                found_test_udmabuf = true;
            }
        }
        f += 1;
    }

    ASSERT_EQ(
        (f % Fields::FIELD_COUNT as c_int) as c_long,
        Fields::INODE as c_long,
        b"number of fields\0".as_ptr() as *const c_char,
    );

    ASSERT_TRUE(
        found_test_sysheap_dmabuf,
        b"found_test_sysheap_dmabuf\0".as_ptr() as *const c_char,
    );
    ASSERT_TRUE(
        found_test_udmabuf,
        b"found_test_udmabuf\0".as_ptr() as *const c_char,
    );

    free(line as *mut c_void);
    fclose(iter_file);
    close(iter_fd);
}

unsafe fn subtest_dmabuf_iter_check_lots_of_buffers(skel: *mut dmabuf_iter) {
    let iter_fd: c_int;
    let mut buf = [0 as c_char; 1024];
    let mut total_bytes_read: size_t = 0;
    let mut bytes_read: ssize_t;

    iter_fd = bpf_iter_create(bpf_link__fd((*skel).links.dmabuf_collector));
    if !ASSERT_OK_FD(iter_fd, b"iter_create\0".as_ptr() as *const c_char) {
        return;
    }

    loop {
        bytes_read = read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf));
        if bytes_read <= 0 {
            break;
        }
        total_bytes_read += bytes_read as size_t;
    }

    ASSERT_GT(
        total_bytes_read,
        4096,
        b"total_bytes_read\0".as_ptr() as *const c_char,
    );

    close(iter_fd);
}

unsafe fn subtest_dmabuf_iter_check_open_coded(skel: *mut dmabuf_iter, map_fd: c_int) {
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        retval: 0,
    };
    let mut key = [0 as c_char; DMA_BUF_NAME_LEN];
    let err: c_int;
    let fd: c_int;
    let mut found = false;

    /* No need to attach it, just run it directly */
    fd = bpf_program__fd((*skel).progs.iter_dmabuf_for_each);

    err = bpf_prog_test_run_opts(fd, &mut topts);
    if !ASSERT_OK(err, b"test_run_opts err\0".as_ptr() as *const c_char) {
        return;
    }
    if !ASSERT_OK(topts.retval as c_int, b"test_run_opts retval\0".as_ptr() as *const c_char) {
        return;
    }

    if !ASSERT_OK(
        bpf_map_get_next_key(map_fd, ptr::null(), key.as_mut_ptr() as *mut c_void),
        b"get next key\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    loop {
        ASSERT_OK(
            bpf_map_lookup_elem(
                map_fd,
                key.as_ptr() as *const c_void,
                &mut found as *mut _ as *mut c_void,
            ),
            b"lookup\0".as_ptr() as *const c_char,
        );
        ASSERT_TRUE(found, b"found test buffer\0".as_ptr() as *const c_char);

        if bpf_map_get_next_key(
            map_fd,
            key.as_ptr() as *const c_void,
            key.as_mut_ptr() as *mut c_void,
        ) == 0
        {
            continue;
        }
        break;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_dmabuf_iter() {
    let mut skel: *mut dmabuf_iter = ptr::null_mut();
    let map_fd: c_int;
    let f = false;

    skel = dmabuf_iter__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"dmabuf_iter__open_and_load\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    map_fd = bpf_map__fd((*skel).maps.testbuf_hash);
    if !ASSERT_OK_FD(map_fd, b"map_fd\0".as_ptr() as *const c_char) {
        dmabuf_iter__destroy(skel);
        return;
    }

    if !ASSERT_OK(
        bpf_map_update_elem(
            map_fd,
            udmabuf_test_buffer_name.as_ptr() as *const c_void,
            &f as *const _ as *const c_void,
            BPF_ANY,
        ),
        b"insert udmabuf\0".as_ptr() as *const c_char,
    ) {
        dmabuf_iter__destroy(skel);
        return;
    }
    if !ASSERT_OK(
        bpf_map_update_elem(
            map_fd,
            sysheap_test_buffer_name.as_ptr() as *const c_void,
            &f as *const _ as *const c_void,
            BPF_ANY,
        ),
        b"insert sysheap buffer\0".as_ptr() as *const c_char,
    ) {
        dmabuf_iter__destroy(skel);
        return;
    }

    if !ASSERT_OK(create_test_buffers(), b"create_test_buffers\0".as_ptr() as *const c_char) {
        dmabuf_iter__destroy(skel);
        return;
    }

    if !ASSERT_OK(dmabuf_iter__attach(skel), b"skel_attach\0".as_ptr() as *const c_char) {
        destroy_test_buffers();
        dmabuf_iter__destroy(skel);
        return;
    }

    if test__start_subtest(b"no_infinite_reads\0".as_ptr() as *const c_char) {
        subtest_dmabuf_iter_check_no_infinite_reads(skel);
    }
    if test__start_subtest(b"default_iter\0".as_ptr() as *const c_char) {
        subtest_dmabuf_iter_check_default_iter(skel);
    }
    if test__start_subtest(b"lots_of_buffers\0".as_ptr() as *const c_char) {
        const NUM_BUFS: usize = 100;
        let mut buffers = [0 as c_int; NUM_BUFS];
        let mut i: c_int = 0;

        while i < NUM_BUFS as c_int {
            buffers[i as usize] = create_sys_heap_dmabuf(getpagesize() as size_t);
            if !ASSERT_OK_FD(
                buffers[i as usize],
                b"dmabuf_fd\0".as_ptr() as *const c_char,
            ) {
                break;
            }
            i += 1;
        }

        if i == NUM_BUFS as c_int {
            subtest_dmabuf_iter_check_lots_of_buffers(skel);
        }

        i -= 1;
        while i >= 0 {
            close(buffers[i as usize]);
            i -= 1;
        }
    }
    if test__start_subtest(b"open_coded\0".as_ptr() as *const c_char) {
        subtest_dmabuf_iter_check_open_coded(skel, map_fd);
    }

    destroy_test_buffers();
    dmabuf_iter__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
