// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011-2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 *
 * Parts came from evlist.c builtin-{top,stat,record}.c, see those files for further
 * copyright notes.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const MASK_SIZE: usize = 1023;
const EAGAIN: c_int = 11;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const _SC_AIO_PRIO_DELTA_MAX: c_int = 25;
const MPOL_BIND: c_int = 2;

extern "C" {
    static mut errno: c_int;
    static mut verbose: c_int;
    static page_size: usize;
    static MAP_FAILED: *mut c_void;

    fn bitmap_scnprintf(
        bitmap: *const c_ulong,
        nbits: usize,
        buf: *mut c_char,
        size: usize,
    ) -> usize;
    fn bitmap_zalloc(nbits: c_ulong) -> *mut c_ulong;
    fn bitmap_free(bitmap: *mut c_ulong);
    fn __set_bit(nr: c_ulong, addr: *mut c_ulong);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn perf_mmap__mmap_len(map: *mut perf_mmap) -> usize;
    fn perf_mmap__mmap(
        map: *mut perf_mmap,
        mp: *mut perf_mmap_params,
        fd: c_int,
        cpu: perf_cpu,
    ) -> c_int;
    fn perf_mmap__read_head(map: *mut perf_mmap) -> u64;
    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__consume(map: *mut perf_mmap);

    fn cpu__max_node() -> c_int;
    fn cpu__get_node(cpu: perf_cpu) -> c_int;
    fn cpu__max_cpu() -> perf_cpu;
    fn cpu_map__online() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpu_map: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpu_map: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__put(cpu_map: *mut perf_cpu_map);

    fn zstd_init(data: *mut zstd_data, comp_level: c_int) -> c_int;
    fn zstd_fini(data: *mut zstd_data);

    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sysconf(name: c_int) -> c_long;
    fn mbind(
        start: *mut c_void,
        len: usize,
        mode: c_int,
        nodemask: *const c_ulong,
        maxnode: c_ulong,
        flags: c_uint,
    ) -> c_int;
}

type c_uint = u32;
type off_t = i64;

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct mmap_cpu_mask {
    pub bits: *mut c_ulong,
    pub nbits: usize,
}

#[repr(C)]
pub struct perf_mmap {
    pub base: *mut u8,
    pub mask: c_ulong,
    pub start: c_ulong,
    pub end: c_ulong,
    pub prev: u64,
    pub flush: c_int,
    pub cpu: perf_cpu,
}

#[repr(C)]
pub struct perf_mmap_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxtrace_mmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxtrace_mmap_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zstd_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aiocb {
    pub aio_fildes: c_int,
    pub aio_reqprio: c_int,
}

#[repr(C)]
pub struct aio_state {
    pub nr_cblocks: c_int,
    pub aiocb: *mut *mut aiocb,
    pub cblocks: *mut aiocb,
    pub data: *mut *mut c_void,
}

#[repr(C)]
pub struct mmap_params {
    pub core: perf_mmap_params,
    pub nr_cblocks: c_int,
    pub affinity: c_int,
    pub flush: c_int,
    pub comp_level: c_int,
    pub auxtrace_mp: auxtrace_mmap_params,
}

#[repr(C)]
pub struct mmap_struct {
    pub core: perf_mmap,
    pub aio: aio_state,
    pub data: *mut c_void,
    pub auxtrace_mmap: auxtrace_mmap,
    pub affinity_mask: mmap_cpu_mask,
    pub zstd_data: zstd_data,
}

const PERF_AFFINITY_SYS: c_int = 0;
const PERF_AFFINITY_NODE: c_int = 1;
const PERF_AFFINITY_CPU: c_int = 2;

#[no_mangle]
pub unsafe extern "C" fn mmap_cpu_mask__scnprintf(mask: *mut mmap_cpu_mask, tag: *const c_char) {
    let mut buf = [0 as c_char; MASK_SIZE + 1];
    let len: usize;

    len = bitmap_scnprintf((*mask).bits, (*mask).nbits, buf.as_mut_ptr(), MASK_SIZE);
    buf[len] = b'\0' as c_char;
    pr_debug(
        b"%p: %s mask[%zd]: %s\n\0".as_ptr() as *const c_char,
        mask,
        tag,
        (*mask).nbits,
        buf.as_ptr(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn mmap__mmap_len(map: *mut mmap_struct) -> usize {
    perf_mmap__mmap_len(&mut (*map).core)
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_mmap__mmap(
    _mm: *mut auxtrace_mmap,
    _mp: *mut auxtrace_mmap_params,
    _userpg: *mut c_void,
    _fd: c_int,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_mmap__munmap(_mm: *mut auxtrace_mmap) {}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_mmap_params__init(
    _mp: *mut auxtrace_mmap_params,
    _auxtrace_offset: off_t,
    _auxtrace_pages: c_uint,
    _auxtrace_overwrite: bool,
) {
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_mmap_params__set_idx(
    _mp: *mut auxtrace_mmap_params,
    _evlist: *mut evlist,
    _evsel: *mut evsel,
    _idx: c_int,
) {
}

/* HAVE_AIO_SUPPORT */
unsafe fn perf_mmap__aio_enabled(map: *mut mmap_struct) -> c_int {
    ((*map).aio.nr_cblocks > 0) as c_int
}

/* HAVE_LIBNUMA_SUPPORT */
unsafe fn perf_mmap__aio_alloc(map: *mut mmap_struct, idx: c_int) -> c_int {
    *(*map).aio.data.add(idx as usize) = mmap(
        core::ptr::null_mut(),
        mmap__mmap_len(map),
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        0,
        0,
    );
    if *(*map).aio.data.add(idx as usize) == MAP_FAILED {
        *(*map).aio.data.add(idx as usize) = core::ptr::null_mut();
        return -1;
    }

    0
}

unsafe fn perf_mmap__aio_free(map: *mut mmap_struct, idx: c_int) {
    if (*map).aio.data.is_null() || (*(*map).aio.data.add(idx as usize)).is_null() {
        return;
    }
    munmap(*(*map).aio.data.add(idx as usize), mmap__mmap_len(map));
    *(*map).aio.data.add(idx as usize) = core::ptr::null_mut();
}

unsafe fn perf_mmap__aio_bind(
    map: *mut mmap_struct,
    idx: c_int,
    cpu: perf_cpu,
    affinity: c_int,
) -> c_int {
    let data: *mut c_void;
    let mmap_len: usize;
    let node_mask: *mut c_ulong;
    let node_index: c_ulong;
    let mut err = 0;

    if affinity != PERF_AFFINITY_SYS && cpu__max_node() > 1 {
        let node: c_int;

        data = *(*map).aio.data.add(idx as usize);
        mmap_len = mmap__mmap_len(map);
        node = cpu__get_node(cpu);
        /* -1 sign-extends to ULONG_MAX, wrapping bitmap_zalloc(0) and OOB __set_bit */
        if node < 0 {
            return 0;
        }
        node_index = node as c_ulong;
        node_mask = bitmap_zalloc(node_index + 1);
        if node_mask.is_null() {
            pr_err(b"Failed to allocate node mask for mbind: error %m\n\0".as_ptr() as *const c_char);
            return -1;
        }
        __set_bit(node_index, node_mask);
        if mbind(data, mmap_len, MPOL_BIND, node_mask, node_index + 1 + 1, 0) != 0 {
            pr_err(
                b"Failed to bind [%p-%p] AIO buffer to node %lu: error %m\n\0".as_ptr()
                    as *const c_char,
                data,
                (data as *mut u8).add(mmap_len) as *mut c_void,
                node_index,
            );
            err = -1;
        }
        bitmap_free(node_mask);
    }

    err
}

/* !HAVE_LIBNUMA_SUPPORT fallback in C used malloc/free and a no-op bind. */

unsafe fn perf_mmap__aio_mmap(map: *mut mmap_struct, mp: *mut mmap_params) -> c_int {
    let delta_max: c_int;
    let mut i: c_int;
    let prio: c_int;
    let mut ret: c_int;

    (*map).aio.nr_cblocks = (*mp).nr_cblocks;
    if (*map).aio.nr_cblocks != 0 {
        (*map).aio.aiocb = calloc(
            (*map).aio.nr_cblocks as usize,
            core::mem::size_of::<*mut aiocb>(),
        ) as *mut *mut aiocb;
        if (*map).aio.aiocb.is_null() {
            pr_debug2(
                b"failed to allocate aiocb for data buffer, error %m\n\0".as_ptr()
                    as *const c_char,
            );
            return -1;
        }
        (*map).aio.cblocks = calloc(
            (*map).aio.nr_cblocks as usize,
            core::mem::size_of::<aiocb>(),
        ) as *mut aiocb;
        if (*map).aio.cblocks.is_null() {
            pr_debug2(
                b"failed to allocate cblocks for data buffer, error %m\n\0".as_ptr()
                    as *const c_char,
            );
            return -1;
        }
        (*map).aio.data = calloc(
            (*map).aio.nr_cblocks as usize,
            core::mem::size_of::<*mut c_void>(),
        ) as *mut *mut c_void;
        if (*map).aio.data.is_null() {
            pr_debug2(
                b"failed to allocate data buffer, error %m\n\0".as_ptr() as *const c_char,
            );
            return -1;
        }
        delta_max = sysconf(_SC_AIO_PRIO_DELTA_MAX) as c_int;
        i = 0;
        while i < (*map).aio.nr_cblocks {
            ret = perf_mmap__aio_alloc(map, i);
            if ret == -1 {
                pr_debug2(
                    b"failed to allocate data buffer area, error %m\0".as_ptr()
                        as *const c_char,
                );
                return -1;
            }
            ret = perf_mmap__aio_bind(map, i, (*map).core.cpu, (*mp).affinity);
            if ret == -1 {
                return -1;
            }
            /*
             * Use cblock.aio_fildes value different from -1
             * to denote started aio write operation on the
             * cblock so it requires explicit record__aio_sync()
             * call prior the cblock may be reused again.
             */
            (*(*map).aio.cblocks.add(i as usize)).aio_fildes = -1;
            /*
             * Allocate cblocks with priority delta to have
             * faster aio write system calls because queued requests
             * are kept in separate per-prio queues and adding
             * a new request will iterate thru shorter per-prio
             * list. Blocks with numbers higher than
             *  _SC_AIO_PRIO_DELTA_MAX go with priority 0.
             */
            prio = delta_max - i;
            (*(*map).aio.cblocks.add(i as usize)).aio_reqprio = if prio >= 0 { prio } else { 0 };
            i += 1;
        }
    }

    0
}

unsafe fn perf_mmap__aio_munmap(map: *mut mmap_struct) {
    let mut i: c_int;

    i = 0;
    while i < (*map).aio.nr_cblocks {
        perf_mmap__aio_free(map, i);
        i += 1;
    }
    if !(*map).aio.data.is_null() {
        free((*map).aio.data as *mut c_void);
        (*map).aio.data = core::ptr::null_mut();
    }
    free((*map).aio.cblocks as *mut c_void);
    (*map).aio.cblocks = core::ptr::null_mut();
    free((*map).aio.aiocb as *mut c_void);
    (*map).aio.aiocb = core::ptr::null_mut();
}

/* !HAVE_AIO_SUPPORT in C defines perf_mmap__aio_enabled/mmap/munmap as no-ops. */

#[no_mangle]
pub unsafe extern "C" fn mmap__munmap(map: *mut mmap_struct) {
    bitmap_free((*map).affinity_mask.bits);
    (*map).affinity_mask.bits = core::ptr::null_mut();
    (*map).affinity_mask.nbits = 0;

    zstd_fini(&mut (*map).zstd_data);

    perf_mmap__aio_munmap(map);
    if !(*map).data.is_null() {
        munmap((*map).data, mmap__mmap_len(map));
        (*map).data = core::ptr::null_mut();
    }
    auxtrace_mmap__munmap(&mut (*map).auxtrace_mmap);
}

unsafe fn build_node_mask(node: c_int, mask: *mut mmap_cpu_mask) {
    let mut idx: c_int;
    let nr_cpus: c_int;
    let mut cpu: perf_cpu;
    let cpu_map: *mut perf_cpu_map = cpu_map__online();

    if cpu_map.is_null() {
        return;
    }

    nr_cpus = perf_cpu_map__nr(cpu_map);
    idx = 0;
    while idx < nr_cpus {
        cpu = perf_cpu_map__cpu(cpu_map, idx); /* map c index to online cpu index */
        if cpu__get_node(cpu) == node {
            __set_bit(cpu.cpu as c_ulong, (*mask).bits);
        }
        idx += 1;
    }
    perf_cpu_map__put(cpu_map);
}

unsafe fn perf_mmap__setup_affinity_mask(map: *mut mmap_struct, mp: *mut mmap_params) -> c_int {
    (*map).affinity_mask.nbits = cpu__max_cpu().cpu as usize;
    (*map).affinity_mask.bits = bitmap_zalloc((*map).affinity_mask.nbits as c_ulong);
    if (*map).affinity_mask.bits.is_null() {
        return -1;
    }

    if (*mp).affinity == PERF_AFFINITY_NODE && cpu__max_node() > 1 {
        build_node_mask(cpu__get_node((*map).core.cpu), &mut (*map).affinity_mask);
    } else if (*mp).affinity == PERF_AFFINITY_CPU {
        __set_bit((*map).core.cpu.cpu as c_ulong, (*map).affinity_mask.bits);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mmap__mmap(
    map: *mut mmap_struct,
    mp: *mut mmap_params,
    fd: c_int,
    cpu: perf_cpu,
) -> c_int {
    if perf_mmap__mmap(&mut (*map).core, &mut (*mp).core, fd, cpu) != 0 {
        pr_debug2(
            b"failed to mmap perf event ring buffer, error %d\n\0".as_ptr() as *const c_char,
            errno,
        );
        return -1;
    }

    if (*mp).affinity != PERF_AFFINITY_SYS && perf_mmap__setup_affinity_mask(map, mp) != 0 {
        pr_debug2(
            b"failed to alloc mmap affinity mask, error %d\n\0".as_ptr() as *const c_char,
            errno,
        );
        return -1;
    }

    if verbose == 2 {
        mmap_cpu_mask__scnprintf(&mut (*map).affinity_mask, b"mmap\0".as_ptr() as *const c_char);
    }

    (*map).core.flush = (*mp).flush;

    if zstd_init(&mut (*map).zstd_data, (*mp).comp_level) != 0 {
        pr_debug2(
            b"failed to init mmap compressor, error %d\n\0".as_ptr() as *const c_char,
            errno,
        );
        return -1;
    }

    if (*mp).comp_level != 0 && perf_mmap__aio_enabled(map) == 0 {
        (*map).data = mmap(
            core::ptr::null_mut(),
            mmap__mmap_len(map),
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            0,
            0,
        );
        if (*map).data == MAP_FAILED {
            pr_debug2(
                b"failed to mmap data buffer, error %d\n\0".as_ptr() as *const c_char,
                errno,
            );
            (*map).data = core::ptr::null_mut();
            return -1;
        }
    }

    if auxtrace_mmap__mmap(
        &mut (*map).auxtrace_mmap,
        &mut (*mp).auxtrace_mp,
        (*map).core.base as *mut c_void,
        fd,
    ) != 0
    {
        return -1;
    }

    perf_mmap__aio_mmap(map, mp)
}

#[no_mangle]
pub unsafe extern "C" fn perf_mmap__push(
    md: *mut mmap_struct,
    to: *mut c_void,
    push: unsafe extern "C" fn(
        map: *mut mmap_struct,
        to: *mut c_void,
        buf: *mut c_void,
        size: usize,
    ) -> c_int,
) -> c_int {
    let head: u64 = perf_mmap__read_head(&mut (*md).core);
    let data: *mut u8 = (*md).core.base.add(page_size);
    let mut size: c_ulong;
    let mut buf: *mut c_void;
    let mut rc: c_int = 0;

    rc = perf_mmap__read_init(&mut (*md).core);
    if rc < 0 {
        return if rc == -EAGAIN { 1 } else { -1 };
    }

    size = (*md).core.end - (*md).core.start;

    if ((*md).core.start & (*md).core.mask) + size != ((*md).core.end & (*md).core.mask) {
        buf = data.add(((*md).core.start & (*md).core.mask) as usize) as *mut c_void;
        size = (*md).core.mask + 1 - ((*md).core.start & (*md).core.mask);
        (*md).core.start += size;

        if push(md, to, buf, size as usize) < 0 {
            rc = -1;
        } else {
            buf = data.add(((*md).core.start & (*md).core.mask) as usize) as *mut c_void;
            size = (*md).core.end - (*md).core.start;
            (*md).core.start += size;

            if push(md, to, buf, size as usize) < 0 {
                rc = -1;
            } else {
                (*md).core.prev = head;
                perf_mmap__consume(&mut (*md).core);
            }
        }
    } else {
        buf = data.add(((*md).core.start & (*md).core.mask) as usize) as *mut c_void;
        size = (*md).core.end - (*md).core.start;
        (*md).core.start += size;

        if push(md, to, buf, size as usize) < 0 {
            rc = -1;
        } else {
            (*md).core.prev = head;
            perf_mmap__consume(&mut (*md).core);
        }
    }

    rc
}
