/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/bio-integrity.h.  Definitions from linux/bio.h are external. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bip_flags {
    BIP_BLOCK_INTEGRITY = 1 << 0, // block layer owns integrity data
    BIP_MAPPED_INTEGRITY = 1 << 1, // ref tag has been remapped
    BIP_DISK_NOCHECK = 1 << 2, // disable disk integrity checking
    BIP_IP_CHECKSUM = 1 << 3, // IP checksum
    BIP_COPY_USER = 1 << 4, // Kernel bounce buffer in use
    BIP_CHECK_GUARD = 1 << 5, // guard check
    BIP_CHECK_REFTAG = 1 << 6, // reftag check
    BIP_CHECK_APPTAG = 1 << 7, // apptag check
    BIP_MEMPOOL = 1 << 15, // buffer backed by mempool
}

pub const BIP_CHECK_FLAGS: u32 = (bip_flags::BIP_CHECK_GUARD as u32)
    | (bip_flags::BIP_CHECK_REFTAG as u32)
    | (bip_flags::BIP_CHECK_APPTAG as u32);

#[repr(C)]
pub struct bio_integrity_payload {
    pub bip_iter: bvec_iter,
    pub bip_vcnt: u16,
    pub bip_max_vcnt: u16,
    pub bip_flags: u16,
    pub app_tag: u16,
    pub bip_vec: *mut bio_vec,
}

pub const BIP_CLONE_FLAGS: u32 = (bip_flags::BIP_MAPPED_INTEGRITY as u32)
    | (bip_flags::BIP_IP_CHECKSUM as u32)
    | (bip_flags::BIP_CHECK_GUARD as u32)
    | (bip_flags::BIP_CHECK_REFTAG as u32)
    | (bip_flags::BIP_CHECK_APPTAG as u32);

/* C iteration macros bip_for_each_vec and bio_for_each_integrity_vec are retained
 * as dependency-facing macro intent; their implementations are supplied by bio.h. */

#[cfg(feature = "CONFIG_BLK_DEV_INTEGRITY")]
#[inline]
pub unsafe fn bio_integrity(bio: *mut bio) -> *mut bio_integrity_payload {
    if (*bio).bi_opf & REQ_INTEGRITY != 0 { (*bio).bi_integrity } else { core::ptr::null_mut() }
}

#[cfg(feature = "CONFIG_BLK_DEV_INTEGRITY")]
#[inline]
pub unsafe fn bio_integrity_flagged(bio: *mut bio, flag: bip_flags) -> bool {
    let bip = bio_integrity(bio);
    if !bip.is_null() { (*bip).bip_flags & (flag as u16) != 0 } else { false }
}

#[inline]
pub unsafe fn bip_get_seed(bip: *mut bio_integrity_payload) -> sector_t { (*bip).bip_iter.bi_sector }

#[inline]
pub unsafe fn bip_set_seed(bip: *mut bio_integrity_payload, seed: sector_t) {
    (*bip).bip_iter.bi_sector = seed;
}

#[cfg(feature = "CONFIG_BLK_DEV_INTEGRITY")]
extern "C" {
    pub fn bio_integrity_init(bio: *mut bio, bip: *mut bio_integrity_payload,
        bvecs: *mut bio_vec, nr_vecs: u32);
    pub fn bio_integrity_alloc(bio: *mut bio, gfp: gfp_t, nr: u32) -> *mut bio_integrity_payload;
    pub fn bio_integrity_add_page(bio: *mut bio, page: *mut page, len: u32, offset: u32) -> i32;
    pub fn bio_integrity_map_user(bio: *mut bio, iter: *mut iov_iter) -> i32;
    pub fn bio_integrity_map_iter(bio: *mut bio, meta: *mut uio_meta) -> i32;
    pub fn bio_integrity_unmap_user(bio: *mut bio);
    pub fn bio_integrity_prep(bio: *mut bio, action: u32);
    pub fn bio_integrity_advance(bio: *mut bio, bytes_done: u32);
    pub fn bio_integrity_trim(bio: *mut bio);
    pub fn bio_integrity_clone(bio: *mut bio, bio_src: *mut bio, gfp_mask: gfp_t) -> i32;
}

#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity(_: *mut bio) -> *mut bio_integrity_payload { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_map_user(_: *mut bio, _: *mut iov_iter) -> i32 { -EINVAL }
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_map_iter(_: *mut bio, _: *mut uio_meta) -> i32 { -EINVAL }
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_unmap_user(_: *mut bio) {}
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_prep(_: *mut bio, _: u32) {}
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_clone(_: *mut bio, _: *mut bio, _: gfp_t) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_advance(_: *mut bio, _: u32) {}
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_trim(_: *mut bio) {}
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_flagged(_: *mut bio, _: bip_flags) -> bool { false }
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_alloc(_: *mut bio, _: gfp_t, _: u32) -> *mut bio_integrity_payload { ERR_PTR(-EINVAL) }
#[cfg(not(feature = "CONFIG_BLK_DEV_INTEGRITY"))]
#[inline] pub unsafe fn bio_integrity_add_page(_: *mut bio, _: *mut page, _: u32, _: u32) -> i32 { 0 }

extern "C" {
    pub fn bio_integrity_alloc_buf(bio: *mut bio, gfp: gfp_t, zero_buffer: bool);
    pub fn bio_integrity_free_buf(bip: *mut bio_integrity_payload);
    pub fn bio_integrity_setup_default(bio: *mut bio);
    pub fn fs_bio_integrity_alloc(bio: *mut bio) -> u32;
    pub fn fs_bio_integrity_free(bio: *mut bio);
    pub fn fs_bio_integrity_generate(bio: *mut bio);
    pub fn fs_bio_integrity_verify(bio: *mut bio, sector: sector_t, size: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
