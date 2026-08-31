// SPDX-License-Identifier: GPL-2.0
// Translated from lib/perf/mmap.c. C include dependencies are represented as
// external declarations or assumed imported items.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type u16 = u16;
type u32 = u32;
type u64 = u64;

const MAP_SHARED: c_int = 0x01;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

#[cfg(any(target_arch = "riscv64"))]
const CSR_CYCLE: c_int = 0xc00;
#[cfg(any(target_arch = "riscv64"))]
const CSR_TIME: c_int = 0xc01;

unsafe extern "C" {
    static page_size: size_t;
    static MAP_FAILED: *mut c_void;

    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;

    fn zfree(ptr: *mut *mut c_void);
    fn refcount_set(r: *mut refcount_t, n: c_int);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn ring_buffer_write_tail(base: *mut c_void, tail: u64);
    fn ring_buffer_read_head(base: *mut c_void) -> u64;
    fn pr_debug(fmt: *const i8, ...);
    fn pr_debug2(fmt: *const i8, ...);
    fn pr_debug3(fmt: *const i8, ...);
    fn WARN_ONCE(condition: c_int, fmt: *const i8, ...) -> c_int;
    fn BUG_ON(condition: bool);
    fn mul_u64_u32_shr(a: u64, mul: u32, shift: u32) -> u64;
}

#[repr(C)]
pub struct refcount_t {
    refs: c_int,
}

pub type libperf_unmap_cb_t = Option<unsafe extern "C" fn(*mut perf_mmap)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_mmap_param {
    pub prot: c_int,
    pub mask: c_int,
}

#[repr(C)]
pub struct perf_mmap {
    pub fd: c_int,
    pub overwrite: bool,
    pub unmap_cb: libperf_unmap_cb_t,
    pub refcnt: refcount_t,
    pub next: *mut perf_mmap,
    pub prev: u64,
    pub mask: c_int,
    pub base: *mut c_void,
    pub cpu: perf_cpu,
    pub event_copy: *mut c_void,
    pub event_copy_sz: size_t,
    pub start: u64,
    pub end: u64,
    pub flush: u64,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct perf_event_mmap_page {
    pub version: u32,
    pub compat_version: u32,
    pub lock: u32,
    pub index: u32,
    pub offset: i64,
    pub time_enabled: u64,
    pub time_running: u64,
    pub capabilities: u64,
    pub pmc_width: u16,
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_offset: u64,
    pub time_zero: u64,
    pub size: u32,
    pub __reserved_1: u32,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub __reserved: [u8; 116 * 8],
    pub data_head: u64,
    pub data_tail: u64,
    pub data_offset: u64,
    pub data_size: u64,
    pub aux_head: u64,
    pub aux_tail: u64,
    pub aux_offset: u64,
    pub aux_size: u64,
}

impl perf_event_mmap_page {
    unsafe fn cap_user_rdpmc(&self) -> bool {
        (self.capabilities & (1 << 0)) != 0
    }

    unsafe fn cap_user_time(&self) -> bool {
        (self.capabilities & (1 << 1)) != 0
    }

    unsafe fn cap_user_time_short(&self) -> bool {
        (self.capabilities & (1 << 4)) != 0
    }
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[inline]
unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    ptr::read_volatile(p)
}

#[inline]
unsafe fn barrier() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__init(
    map: *mut perf_mmap,
    prev: *mut perf_mmap,
    overwrite: bool,
    unmap_cb: libperf_unmap_cb_t,
) {
    /* Assume fields were zero initialized. */
    unsafe {
        (*map).fd = -1;
        (*map).overwrite = overwrite;
        (*map).unmap_cb = unmap_cb;
        refcount_set(&mut (*map).refcnt, 0);
        if !prev.is_null() {
            (*prev).next = map;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__mmap_len(map: *mut perf_mmap) -> size_t {
    unsafe { ((*map).mask as size_t).wrapping_add(1).wrapping_add(page_size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__mmap(
    map: *mut perf_mmap,
    mp: *mut perf_mmap_param,
    fd: c_int,
    cpu: perf_cpu,
) -> c_int {
    unsafe {
        (*map).prev = 0;
        (*map).mask = (*mp).mask;
        (*map).base = mmap(
            ptr::null_mut(),
            perf_mmap__mmap_len(map),
            (*mp).prot,
            MAP_SHARED,
            fd,
            0,
        );
        if (*map).base == MAP_FAILED {
            (*map).base = ptr::null_mut();
            return -1;
        }

        (*map).fd = fd;
        (*map).cpu = cpu;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__munmap(map: *mut perf_mmap) {
    unsafe {
        if map.is_null() {
            return;
        }

        zfree(&mut (*map).event_copy);
        (*map).event_copy_sz = 0;
        if !(*map).base.is_null() {
            munmap((*map).base, perf_mmap__mmap_len(map));
            (*map).base = ptr::null_mut();
            (*map).fd = -1;
            refcount_set(&mut (*map).refcnt, 0);
        }
        if let Some(unmap_cb) = (*map).unmap_cb {
            unmap_cb(map);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__get(map: *mut perf_mmap) {
    unsafe {
        refcount_inc(&mut (*map).refcnt);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__put(map: *mut perf_mmap) {
    unsafe {
        BUG_ON(!(*map).base.is_null() && refcount_read(&(*map).refcnt) == 0);

        if refcount_dec_and_test(&mut (*map).refcnt) {
            perf_mmap__munmap(map);
        }
    }
}

#[inline]
unsafe fn perf_mmap__write_tail(md: *mut perf_mmap, tail: u64) {
    unsafe {
        ring_buffer_write_tail((*md).base, tail);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__read_head(map: *mut perf_mmap) -> u64 {
    unsafe { ring_buffer_read_head((*map).base) }
}

unsafe fn perf_mmap__empty(map: *mut perf_mmap) -> bool {
    unsafe {
        let pc = (*map).base as *mut perf_event_mmap_page;

        perf_mmap__read_head(map) == (*map).prev && (*pc).aux_size == 0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__consume(map: *mut perf_mmap) {
    unsafe {
        if !(*map).overwrite {
            let old = (*map).prev;

            perf_mmap__write_tail(map, old);
        }

        if refcount_read(&(*map).refcnt) == 1 && perf_mmap__empty(map) {
            perf_mmap__put(map);
        }
    }
}

unsafe fn overwrite_rb_find_range(
    buf: *mut c_void,
    mask: c_int,
    start: *mut u64,
    end: *mut u64,
) -> c_int {
    unsafe {
        let mut pheader: *mut perf_event_header;
        let mut evt_head = *start;
        let size = mask + 1;

        pr_debug2(
            c"%s: buf=%p, start=%lx\n".as_ptr(),
            c"overwrite_rb_find_range".as_ptr(),
            buf,
            *start,
        );
        pheader = (buf as *mut u8).add((*start & mask as u64) as usize) as *mut perf_event_header;
        loop {
            if evt_head.wrapping_sub(*start) >= size as c_uint as u64 {
                pr_debug(c"Finished reading overwrite ring buffer: rewind\n".as_ptr());
                if evt_head.wrapping_sub(*start) > size as c_uint as u64 {
                    evt_head = evt_head.wrapping_sub((*pheader).size as u64);
                }
                *end = evt_head;
                return 0;
            }

            pheader = (buf as *mut u8).add((evt_head & mask as u64) as usize)
                as *mut perf_event_header;

            if (*pheader).size == 0 {
                pr_debug(c"Finished reading overwrite ring buffer: get start\n".as_ptr());
                *end = evt_head;
                return 0;
            }

            evt_head = evt_head.wrapping_add((*pheader).size as u64);
            pr_debug3(c"move evt_head: %lx\n".as_ptr(), evt_head);
        }
    }

    #[allow(unreachable_code)]
    unsafe {
        WARN_ONCE(1, c"Shouldn't get here\n".as_ptr());
    }
    -1
}

/*
 * Report the start and end of the available data in ringbuffer
 */
unsafe fn __perf_mmap__read_init(md: *mut perf_mmap) -> c_int {
    unsafe {
        let head = perf_mmap__read_head(md);
        let old = (*md).prev;
        let data = ((*md).base as *mut u8).add(page_size);
        let size: c_ulong;

        (*md).start = if (*md).overwrite { head } else { old };
        (*md).end = if (*md).overwrite { old } else { head };

        if (*md).end.wrapping_sub((*md).start) < (*md).flush {
            return -EAGAIN;
        }

        size = (*md).end.wrapping_sub((*md).start) as c_ulong;
        if size > ((*md).mask as c_ulong).wrapping_add(1) {
            if !(*md).overwrite {
                WARN_ONCE(
                    1,
                    c"failed to keep up with mmap data. (warn only once)\n".as_ptr(),
                );

                (*md).prev = head;
                perf_mmap__consume(md);
                return -EAGAIN;
            }

            /*
             * Backward ring buffer is full. We still have a chance to read
             * most of data from it.
             */
            if overwrite_rb_find_range(data as *mut c_void, (*md).mask, &mut (*md).start, &mut (*md).end) != 0 {
                return -EINVAL;
            }
        }

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int {
    unsafe {
        /*
         * Check if event was unmapped due to a POLLHUP/POLLERR.
         */
        if refcount_read(&(*map).refcnt) == 0 {
            return -ENOENT;
        }

        __perf_mmap__read_init(map)
    }
}

/*
 * Mandatory for overwrite mode
 * The direction of overwrite mode is backward.
 * The last perf_mmap__read() will set tail to map->core.prev.
 * Need to correct the map->core.prev to head which is the end of next read.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__read_done(map: *mut perf_mmap) {
    unsafe {
        /*
         * Check if event was unmapped due to a POLLHUP/POLLERR.
         */
        if refcount_read(&(*map).refcnt) == 0 {
            return;
        }

        (*map).prev = perf_mmap__read_head(map);
    }
}

/* When check_messup is true, 'end' must points to a good entry */
unsafe fn perf_mmap__read(
    map: *mut perf_mmap,
    startp: *mut u64,
    end: u64,
) -> *mut perf_event {
    unsafe {
        let data = ((*map).base as *mut u8).add(page_size);
        let mut event: *mut perf_event = ptr::null_mut();
        let diff = end.wrapping_sub(*startp) as c_int;

        if diff >= size_of::<perf_event_header>() as c_int {
            let size: size_t;

            event = data.add((*startp & (*map).mask as u64) as usize) as *mut perf_event;
            size = (*event).header.size as size_t;

            if size < size_of::<perf_event_header>() || diff < size as c_int {
                return ptr::null_mut();
            }

            /*
             * Event straddles the mmap boundary -- header should always
             * be inside due to u64 alignment of output.
             */
            if ((*startp & (*map).mask as u64).wrapping_add(size as u64))
                != ((*startp).wrapping_add(size as u64) & (*map).mask as u64)
            {
                let mut offset = *startp as c_uint;
                let mut len = size as c_uint;
                let mut cpy: c_uint;
                let mut dst = (*map).event_copy;

                if size > (*map).event_copy_sz {
                    dst = realloc((*map).event_copy, size);
                    if dst.is_null() {
                        return ptr::null_mut();
                    }
                    (*map).event_copy = dst;
                    (*map).event_copy_sz = size;
                }

                loop {
                    cpy = core::cmp::min(
                        ((*map).mask + 1 - (offset as c_int & (*map).mask)) as c_uint,
                        len,
                    );
                    memcpy(
                        dst,
                        data.add((offset as c_int & (*map).mask) as usize) as *const c_void,
                        cpy as size_t,
                    );
                    offset = offset.wrapping_add(cpy);
                    dst = (dst as *mut u8).add(cpy as usize) as *mut c_void;
                    len = len.wrapping_sub(cpy);
                    if len == 0 {
                        break;
                    }
                }

                event = (*map).event_copy as *mut perf_event;
            }

            *startp = (*startp).wrapping_add(size as u64);
        }

        event
    }
}

/*
 * Read event from ring buffer one by one.
 * Return one event for each call.
 *
 * Usage:
 * perf_mmap__read_init()
 * while(event = perf_mmap__read_event()) {
 *	//process the event
 *	perf_mmap__consume()
 * }
 * perf_mmap__read_done()
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event {
    unsafe {
        let event: *mut perf_event;

        /*
         * Check if event was unmapped due to a POLLHUP/POLLERR.
         */
        if refcount_read(&(*map).refcnt) == 0 {
            return ptr::null_mut();
        }

        /* non-overwrite doesn't pause the ringbuffer */
        if !(*map).overwrite {
            (*map).end = perf_mmap__read_head(map);
        }

        event = perf_mmap__read(map, &mut (*map).start, (*map).end);

        if !(*map).overwrite {
            (*map).prev = (*map).start;
        }

        event
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn read_perf_counter(counter: c_uint) -> u64 {
    unsafe {
        let low: c_uint;
        let high: c_uint;

        asm!("rdpmc", out("eax") low, out("edx") high, in("ecx") counter);

        low as u64 | ((high as u64) << 32)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn read_timestamp() -> u64 {
    unsafe {
        let low: c_uint;
        let high: c_uint;

        asm!("rdtsc", out("eax") low, out("edx") high);

        low as u64 | ((high as u64) << 32)
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn read_pmccntr() -> u64 {
    unsafe {
        let val: u64;
        asm!("mrs {0}, pmccntr_el0", out(reg) val);
        val
    }
}

macro_rules! pmevcntr_read {
    ($name:ident, $reg:literal) => {
        #[cfg(target_arch = "aarch64")]
        unsafe fn $name() -> u64 {
            unsafe {
                let val: u64;
                asm!(concat!("mrs {0}, ", $reg), out(reg) val);
                val
            }
        }
    };
}

pmevcntr_read!(read_pmevcntr_0, "pmevcntr0_el0");
pmevcntr_read!(read_pmevcntr_1, "pmevcntr1_el0");
pmevcntr_read!(read_pmevcntr_2, "pmevcntr2_el0");
pmevcntr_read!(read_pmevcntr_3, "pmevcntr3_el0");
pmevcntr_read!(read_pmevcntr_4, "pmevcntr4_el0");
pmevcntr_read!(read_pmevcntr_5, "pmevcntr5_el0");
pmevcntr_read!(read_pmevcntr_6, "pmevcntr6_el0");
pmevcntr_read!(read_pmevcntr_7, "pmevcntr7_el0");
pmevcntr_read!(read_pmevcntr_8, "pmevcntr8_el0");
pmevcntr_read!(read_pmevcntr_9, "pmevcntr9_el0");
pmevcntr_read!(read_pmevcntr_10, "pmevcntr10_el0");
pmevcntr_read!(read_pmevcntr_11, "pmevcntr11_el0");
pmevcntr_read!(read_pmevcntr_12, "pmevcntr12_el0");
pmevcntr_read!(read_pmevcntr_13, "pmevcntr13_el0");
pmevcntr_read!(read_pmevcntr_14, "pmevcntr14_el0");
pmevcntr_read!(read_pmevcntr_15, "pmevcntr15_el0");
pmevcntr_read!(read_pmevcntr_16, "pmevcntr16_el0");
pmevcntr_read!(read_pmevcntr_17, "pmevcntr17_el0");
pmevcntr_read!(read_pmevcntr_18, "pmevcntr18_el0");
pmevcntr_read!(read_pmevcntr_19, "pmevcntr19_el0");
pmevcntr_read!(read_pmevcntr_20, "pmevcntr20_el0");
pmevcntr_read!(read_pmevcntr_21, "pmevcntr21_el0");
pmevcntr_read!(read_pmevcntr_22, "pmevcntr22_el0");
pmevcntr_read!(read_pmevcntr_23, "pmevcntr23_el0");
pmevcntr_read!(read_pmevcntr_24, "pmevcntr24_el0");
pmevcntr_read!(read_pmevcntr_25, "pmevcntr25_el0");
pmevcntr_read!(read_pmevcntr_26, "pmevcntr26_el0");
pmevcntr_read!(read_pmevcntr_27, "pmevcntr27_el0");
pmevcntr_read!(read_pmevcntr_28, "pmevcntr28_el0");
pmevcntr_read!(read_pmevcntr_29, "pmevcntr29_el0");
pmevcntr_read!(read_pmevcntr_30, "pmevcntr30_el0");

/*
 * Read a value direct from PMEVCNTR<idx>
 */
#[cfg(target_arch = "aarch64")]
unsafe fn read_perf_counter(counter: c_uint) -> u64 {
    unsafe {
        let read_f: [unsafe fn() -> u64; 32] = [
            read_pmevcntr_0,
            read_pmevcntr_1,
            read_pmevcntr_2,
            read_pmevcntr_3,
            read_pmevcntr_4,
            read_pmevcntr_5,
            read_pmevcntr_6,
            read_pmevcntr_7,
            read_pmevcntr_8,
            read_pmevcntr_9,
            read_pmevcntr_10,
            read_pmevcntr_11,
            read_pmevcntr_13,
            read_pmevcntr_12,
            read_pmevcntr_14,
            read_pmevcntr_15,
            read_pmevcntr_16,
            read_pmevcntr_17,
            read_pmevcntr_18,
            read_pmevcntr_19,
            read_pmevcntr_20,
            read_pmevcntr_21,
            read_pmevcntr_22,
            read_pmevcntr_23,
            read_pmevcntr_24,
            read_pmevcntr_25,
            read_pmevcntr_26,
            read_pmevcntr_27,
            read_pmevcntr_28,
            read_pmevcntr_29,
            read_pmevcntr_30,
            read_pmccntr,
        ];

        if (counter as usize) < read_f.len() {
            return read_f[counter as usize]();
        }

        0
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn read_timestamp() -> u64 {
    unsafe {
        let val: u64;
        asm!("mrs {0}, cntvct_el0", out(reg) val);
        val
    }
}

/* __riscv_xlen contains the witdh of the native base integer, here 64-bit */
#[cfg(target_arch = "riscv64")]
unsafe fn csr_read_num(csr_num: c_int) -> c_ulong {
    let mut ret: c_ulong = 0;

    match csr_num {
        0xc00 => unsafe { asm!("csrr {0}, 0xc00", out(reg) ret) },
        0xc01 => unsafe { asm!("csrr {0}, 0xc01", out(reg) ret) },
        0xc02 => unsafe { asm!("csrr {0}, 0xc02", out(reg) ret) },
        0xc03 => unsafe { asm!("csrr {0}, 0xc03", out(reg) ret) },
        0xc04 => unsafe { asm!("csrr {0}, 0xc04", out(reg) ret) },
        0xc05 => unsafe { asm!("csrr {0}, 0xc05", out(reg) ret) },
        0xc06 => unsafe { asm!("csrr {0}, 0xc06", out(reg) ret) },
        0xc07 => unsafe { asm!("csrr {0}, 0xc07", out(reg) ret) },
        0xc08 => unsafe { asm!("csrr {0}, 0xc08", out(reg) ret) },
        0xc09 => unsafe { asm!("csrr {0}, 0xc09", out(reg) ret) },
        0xc0a => unsafe { asm!("csrr {0}, 0xc0a", out(reg) ret) },
        0xc0b => unsafe { asm!("csrr {0}, 0xc0b", out(reg) ret) },
        0xc0c => unsafe { asm!("csrr {0}, 0xc0c", out(reg) ret) },
        0xc0d => unsafe { asm!("csrr {0}, 0xc0d", out(reg) ret) },
        0xc0e => unsafe { asm!("csrr {0}, 0xc0e", out(reg) ret) },
        0xc0f => unsafe { asm!("csrr {0}, 0xc0f", out(reg) ret) },
        0xc10 => unsafe { asm!("csrr {0}, 0xc10", out(reg) ret) },
        0xc11 => unsafe { asm!("csrr {0}, 0xc11", out(reg) ret) },
        0xc12 => unsafe { asm!("csrr {0}, 0xc12", out(reg) ret) },
        0xc13 => unsafe { asm!("csrr {0}, 0xc13", out(reg) ret) },
        0xc14 => unsafe { asm!("csrr {0}, 0xc14", out(reg) ret) },
        0xc15 => unsafe { asm!("csrr {0}, 0xc15", out(reg) ret) },
        0xc16 => unsafe { asm!("csrr {0}, 0xc16", out(reg) ret) },
        0xc17 => unsafe { asm!("csrr {0}, 0xc17", out(reg) ret) },
        0xc18 => unsafe { asm!("csrr {0}, 0xc18", out(reg) ret) },
        0xc19 => unsafe { asm!("csrr {0}, 0xc19", out(reg) ret) },
        0xc1a => unsafe { asm!("csrr {0}, 0xc1a", out(reg) ret) },
        0xc1b => unsafe { asm!("csrr {0}, 0xc1b", out(reg) ret) },
        0xc1c => unsafe { asm!("csrr {0}, 0xc1c", out(reg) ret) },
        0xc1d => unsafe { asm!("csrr {0}, 0xc1d", out(reg) ret) },
        0xc1e => unsafe { asm!("csrr {0}, 0xc1e", out(reg) ret) },
        0xc1f => unsafe { asm!("csrr {0}, 0xc1f", out(reg) ret) },
        _ => {}
    }

    ret
}

#[cfg(target_arch = "riscv64")]
unsafe fn read_perf_counter(counter: c_uint) -> u64 {
    unsafe { csr_read_num(CSR_CYCLE + counter as c_int) as u64 }
}

#[cfg(target_arch = "riscv64")]
unsafe fn read_timestamp() -> u64 {
    unsafe { csr_read_num(CSR_TIME) as u64 }
}

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64"
)))]
unsafe fn read_perf_counter(_counter: c_uint) -> u64 {
    0
}

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64"
)))]
unsafe fn read_timestamp() -> u64 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_mmap__read_self(
    map: *mut perf_mmap,
    count: *mut perf_counts_values,
) -> c_int {
    unsafe {
        let pc = (*map).base as *mut perf_event_mmap_page;
        let mut seq: u32;
        let idx: u32;
        let mut time_mult: u32 = 0;
        let mut time_shift: u32 = 0;
        let mut cnt: u64;
        let mut cyc: u64 = 0;
        let mut time_offset: u64 = 0;
        let mut time_cycles: u64 = 0;
        let mut time_mask: u64 = !0_u64;

        if pc.is_null() || !(*pc).cap_user_rdpmc() {
            return -1;
        }

        loop {
            seq = READ_ONCE(&(*pc).lock);
            barrier();

            (*count).ena = READ_ONCE(&(*pc).time_enabled);
            (*count).run = READ_ONCE(&(*pc).time_running);

            if (*pc).cap_user_time() && (*count).ena != (*count).run {
                cyc = read_timestamp();
                time_mult = READ_ONCE(&(*pc).time_mult);
                time_shift = READ_ONCE(&(*pc).time_shift) as u32;
                time_offset = READ_ONCE(&(*pc).time_offset);

                if (*pc).cap_user_time_short() {
                    time_cycles = READ_ONCE(&(*pc).time_cycles);
                    time_mask = READ_ONCE(&(*pc).time_mask);
                }
            }

            idx = READ_ONCE(&(*pc).index);
            cnt = READ_ONCE(&(*pc).offset) as u64;
            if (*pc).cap_user_rdpmc() && idx != 0 {
                let mut evcnt = read_perf_counter(idx - 1);
                let width: u16 = READ_ONCE(&(*pc).pmc_width);

                evcnt <<= 64 - width;
                evcnt >>= 64 - width;
                cnt = cnt.wrapping_add(evcnt);
            } else {
                return -1;
            }

            barrier();
            if READ_ONCE(&(*pc).lock) == seq {
                break;
            }
        }

        if (*count).ena != (*count).run {
            let delta: u64;

            /* Adjust for cap_usr_time_short, a nop if not */
            cyc = time_cycles.wrapping_add(cyc.wrapping_sub(time_cycles) & time_mask);

            delta = time_offset.wrapping_add(mul_u64_u32_shr(cyc, time_mult, time_shift));

            (*count).ena = (*count).ena.wrapping_add(delta);
            if idx != 0 {
                (*count).run = (*count).run.wrapping_add(delta);
            }
        }

        (*count).val = cnt;

        0
    }
}
