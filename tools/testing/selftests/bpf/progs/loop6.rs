// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// <vmlinux.h>, <bpf/bpf_core_read.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, and "bpf_misc.h".

use core::ffi::c_void;
use core::mem;
use core::ptr;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* typically virtio scsi has max SGs of 6 */
const VIRTIO_MAX_SGS: u32 = 6;

/* Verifier will fail with SG_MAX = 128. The failure can be
 * workarounded with a smaller SG_MAX, e.g. 10.
 */
// #define WORKAROUND
const SG_MAX: u32 = 10;
// Without WORKAROUND: typically virtio blk has max SEG of 128.

const SG_CHAIN: u64 = 0x01;
const SG_END: u64 = 0x02;

// Provided by <vmlinux.h>.
#[repr(C)]
pub struct scatterlist {
    pub page_link: u64,
    pub length: u32,
}

extern "C" {
    fn bpf_probe_read_kernel(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i64;
    fn __sink(arg: u32);
}

#[inline]
unsafe fn sg_is_chain(sg: *const scatterlist) -> u64 {
    (*sg).page_link & SG_CHAIN
}

#[inline]
unsafe fn sg_is_last(sg: *const scatterlist) -> u64 {
    (*sg).page_link & SG_END
}

#[inline]
unsafe fn sg_chain_ptr(sg: *const scatterlist) -> *mut scatterlist {
    ((*sg).page_link & !(SG_CHAIN | SG_END)) as *mut scatterlist
}

#[inline]
unsafe fn __sg_next(mut sgp: *mut scatterlist) -> *mut scatterlist {
    let mut sg: scatterlist = mem::zeroed();

    bpf_probe_read_kernel(
        &mut sg as *mut scatterlist as *mut c_void,
        mem::size_of::<scatterlist>() as u32,
        sgp as *const c_void,
    );
    if sg_is_last(&sg as *const scatterlist) != 0 {
        return ptr::null_mut();
    }

    sgp = sgp.add(1);

    bpf_probe_read_kernel(
        &mut sg as *mut scatterlist as *mut c_void,
        mem::size_of::<scatterlist>() as u32,
        sgp as *const c_void,
    );
    if sg_is_chain(&sg as *const scatterlist) != 0 {
        sgp = sg_chain_ptr(&sg as *const scatterlist);
    }

    sgp
}

#[inline]
unsafe fn get_sgp(sgs: *mut *mut scatterlist, i: i32) -> *mut scatterlist {
    let mut sgp: *mut scatterlist = ptr::null_mut();

    bpf_probe_read_kernel(
        &mut sgp as *mut *mut scatterlist as *mut c_void,
        mem::size_of::<*mut scatterlist>() as u32,
        sgs.offset(i as isize) as *const c_void,
    );
    sgp
}

#[no_mangle]
pub static mut run_once: i32 = 0;
#[no_mangle]
pub static mut result: i32 = 0;

#[link_section = "kprobe/virtqueue_add_sgs"]
#[no_mangle]
pub unsafe extern "C" fn trace_virtqueue_add_sgs(
    _unused: *mut c_void,
    sgs: *mut *mut scatterlist,
    out_sgs: u32,
    in_sgs: u32,
) -> i32 {
    let mut sgp: *mut scatterlist;
    let mut length1: u64 = 0;
    let mut length2: u64 = 0;
    let mut i: u32;
    let mut n: u32;
    let mut len: u32;

    if run_once != 0 {
        return 0;
    }

    i = 0;
    while (i < VIRTIO_MAX_SGS) && (i < out_sgs) {
        __sink(out_sgs);
        n = 0;
        sgp = get_sgp(sgs, i as i32);
        while !sgp.is_null() && (n < SG_MAX) {
            // BPF_CORE_READ(sgp, length)
            len = ptr::read_volatile(ptr::addr_of!((*sgp).length));
            length1 = length1.wrapping_add(len as u64);
            n = n.wrapping_add(1);
            sgp = __sg_next(sgp);
        }
        i = i.wrapping_add(1);
    }

    i = 0;
    while (i < VIRTIO_MAX_SGS) && (i < in_sgs) {
        __sink(in_sgs);
        n = 0;
        sgp = get_sgp(sgs, i as i32);
        while !sgp.is_null() && (n < SG_MAX) {
            // BPF_CORE_READ(sgp, length)
            len = ptr::read_volatile(ptr::addr_of!((*sgp).length));
            length2 = length2.wrapping_add(len as u64);
            n = n.wrapping_add(1);
            sgp = __sg_next(sgp);
        }
        i = i.wrapping_add(1);
    }

    run_once = 1;
    result = length2.wrapping_sub(length1) as i32;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
