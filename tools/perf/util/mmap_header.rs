/* Translated from perf/util/mmap.h. */
/* Dependencies from C includes:
 * <internal/mmap.h>, <linux/types.h>, <linux/bitops.h>, <perf/cpumap.h>,
 * "auxtrace.h", and "util/compress.h".
 * <aio.h> contents are used when HAVE_AIO_SUPPORT is enabled.
 */

use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct aiocb {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mmap_cpu_mask {
    pub bits: *mut c_ulong,
    pub nbits: usize,
}

/* Equivalent of:
 * #define MMAP_CPU_MASK_BYTES(m) \
 *      (BITS_TO_LONGS(((struct mmap_cpu_mask *)m)->nbits) * sizeof(unsigned long))
 *
 * BITS_TO_LONGS is supplied by <linux/bitops.h>.
 */
pub unsafe fn MMAP_CPU_MASK_BYTES(m: *const c_void) -> usize {
    BITS_TO_LONGS((m as *const mmap_cpu_mask).as_ref().unwrap().nbits) * size_of::<c_ulong>()
}

/**
 * struct mmap - perf's ring buffer mmap details
 *
 * @refcnt - e.g. code using PERF_EVENT_IOC_SET_OUTPUT to share this
 */
#[repr(C)]
pub struct mmap {
    pub core: perf_mmap,
    pub auxtrace_mmap: auxtrace_mmap,
    /* Present under HAVE_AIO_SUPPORT in the C header. */
    pub aio: mmap_aio,
    pub affinity_mask: mmap_cpu_mask,
    pub data: *mut c_void,
    pub file: *mut perf_data_file,
    pub zstd_data: zstd_data,
}

#[repr(C)]
pub struct mmap_aio {
    pub data: *mut *mut c_void,
    pub cblocks: *mut aiocb,
    pub aiocb: *mut *mut aiocb,
    pub nr_cblocks: c_int,
}

#[repr(C)]
pub struct mmap_params {
    pub core: perf_mmap_param,
    pub nr_cblocks: c_int,
    pub affinity: c_int,
    pub flush: c_int,
    pub comp_level: c_int,
    pub auxtrace_mp: auxtrace_mmap_params,
}

unsafe extern "C" {
    pub fn mmap__mmap(
        map: *mut mmap,
        mp: *mut mmap_params,
        fd: c_int,
        cpu: perf_cpu,
    ) -> c_int;
    pub fn mmap__munmap(map: *mut mmap);

    pub fn perf_mmap__read_forward(map: *mut mmap) -> *mut perf_event;

    pub fn perf_mmap__push(
        md: *mut mmap,
        to: *mut c_void,
        push: Option<
            unsafe extern "C" fn(
                map: *mut mmap,
                to: *mut c_void,
                buf: *mut c_void,
                size: usize,
            ) -> c_int,
        >,
    ) -> c_int;

    pub fn mmap__mmap_len(map: *mut mmap) -> usize;

    pub fn mmap_cpu_mask__scnprintf(mask: *mut mmap_cpu_mask, tag: *const c_char);
}
