/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/blk-integrity.h. Included C headers provide the referenced types and symbols.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum blk_integrity_flags {
    BLK_INTEGRITY_NOVERIFY = 1 << 0,
    BLK_INTEGRITY_NOGENERATE = 1 << 1,
    BLK_INTEGRITY_DEVICE_CAPABLE = 1 << 2,
    BLK_INTEGRITY_REF_TAG = 1 << 3,
    BLK_INTEGRITY_STACKED = 1 << 4,
    BLK_SPLIT_INTERVAL_CAPABLE = 1 << 5,
}

extern "C" {
    pub fn blk_integrity_profile_name(bi: *mut blk_integrity) -> *const ::core::ffi::c_char;
    pub fn queue_limits_stack_integrity(t: *mut queue_limits, b: *mut queue_limits) -> bool;
}

#[inline]
pub unsafe fn queue_limits_stack_integrity_bdev(t: *mut queue_limits, bdev: *mut block_device) -> bool {
    queue_limits_stack_integrity(t, &mut (*(*bdev).bd_disk).queue.as_mut().unwrap().limits)
}

// CONFIG_BLK_DEV_INTEGRITY conditional declarations are preserved with cfg attributes.
#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
extern "C" {
    pub fn blk_rq_map_integrity_sg(rq: *mut request, s: *mut scatterlist) -> ::core::ffi::c_int;
    pub fn blk_rq_count_integrity_sg(q: *mut request_queue, b: *mut bio) -> ::core::ffi::c_int;
    pub fn blk_rq_integrity_map_user(rq: *mut request, ubuf: *mut ::core::ffi::c_void, bytes: isize) -> ::core::ffi::c_int;
    pub fn blk_get_meta_cap(bdev: *mut block_device, cmd: ::core::ffi::c_uint, argp: *mut logical_block_metadata_cap) -> ::core::ffi::c_int;
    pub fn blk_rq_integrity_dma_map_iter_start(req: *mut request, dma_dev: *mut device, state: *mut dma_iova_state, iter: *mut blk_dma_iter) -> bool;
    pub fn blk_rq_integrity_dma_map_iter_next(req: *mut request, dma_dev: *mut device, iter: *mut blk_dma_iter) -> bool;
}

#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
#[inline]
pub unsafe fn blk_integrity_queue_supports_integrity(q: *mut request_queue) -> bool { (*q).limits.integrity.metadata_size != 0 }

#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
#[inline]
pub unsafe fn blk_get_integrity(disk: *mut gendisk) -> *mut blk_integrity {
    if !blk_integrity_queue_supports_integrity((*disk).queue) { core::ptr::null_mut() } else { &mut (*(*disk).queue).limits.integrity }
}

#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
#[inline]
pub unsafe fn bdev_get_integrity(bdev: *mut block_device) -> *mut blk_integrity { blk_get_integrity((*bdev).bd_disk) }

#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
#[inline]
pub unsafe fn queue_max_integrity_segments(q: *const request_queue) -> u16 { (*q).limits.max_integrity_segments }

#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
#[inline]
pub unsafe fn bio_integrity_intervals(bi: *mut blk_integrity, sectors: ::core::ffi::c_uint) -> ::core::ffi::c_uint { sectors >> ((*bi).interval_exp - 9) }
#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
#[inline]
pub unsafe fn bio_integrity_bytes(bi: *mut blk_integrity, sectors: ::core::ffi::c_uint) -> ::core::ffi::c_uint { bio_integrity_intervals(bi, sectors) * (*bi).metadata_size }
#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
#[inline]
pub unsafe fn blk_integrity_rq(rq: *const request) -> bool { (*rq).cmd_flags & REQ_INTEGRITY != 0 }
#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
#[inline]
pub unsafe fn rq_integrity_vec(rq: *mut request) -> bio_vec { mp_bvec_iter_bvec((*(*rq).bio).bi_integrity.bip_vec, (*(*rq).bio).bi_integrity.bip_iter) }

#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_get_meta_cap(_: *mut block_device, _: ::core::ffi::c_uint, _: *mut logical_block_metadata_cap) -> ::core::ffi::c_int { -ENOIOCTLCMD }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_rq_count_integrity_sg(_: *mut request_queue, _: *mut bio) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_rq_map_integrity_sg(_: *mut request, _: *mut scatterlist) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_rq_integrity_map_user(_: *mut request, _: *mut ::core::ffi::c_void, _: isize) -> ::core::ffi::c_int { -EINVAL }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_rq_integrity_dma_map_iter_start(_: *mut request, _: *mut device, _: *mut dma_iova_state, _: *mut blk_dma_iter) -> bool { false }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_rq_integrity_dma_map_iter_next(_: *mut request, _: *mut device, _: *mut blk_dma_iter) -> bool { false }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn bdev_get_integrity(_: *mut block_device) -> *mut blk_integrity { core::ptr::null_mut() }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_get_integrity(_: *mut gendisk) -> *mut blk_integrity { core::ptr::null_mut() }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_integrity_queue_supports_integrity(_: *mut request_queue) -> bool { false }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn queue_max_integrity_segments(_: *const request_queue) -> u16 { 0 }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn bio_integrity_intervals(_: *mut blk_integrity, _: ::core::ffi::c_uint) -> ::core::ffi::c_uint { 0 }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn bio_integrity_bytes(_: *mut blk_integrity, _: ::core::ffi::c_uint) -> ::core::ffi::c_uint { 0 }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn blk_integrity_rq(_: *const request) -> bool { false }
#[cfg(not(CONFIG_BLK_DEV_INTEGRITY))]
#[inline] pub unsafe fn rq_integrity_vec(_: *mut request) -> bio_vec { core::mem::zeroed() }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bio_integrity_action {
    BI_ACT_BUFFER = 1u32 << 0,
    BI_ACT_CHECK = 1u32 << 1,
    BI_ACT_ZERO = 1u32 << 2,
}

extern "C" { pub fn __bio_integrity_action(bio: *mut bio) -> ::core::ffi::c_uint; }
#[inline]
pub unsafe fn bio_integrity_action(bio: *mut bio) -> ::core::ffi::c_uint {
    if blk_get_integrity((*(*bio).bi_bdev).bd_disk).is_null() || bio_integrity(bio) { 0 } else { __bio_integrity_action(bio) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
