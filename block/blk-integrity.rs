// SPDX-License-Identifier: GPL-2.0
/*
 * blk-integrity.c - Block layer data integrity extensions
 *
 * Copyright (C) 2007, 2008 Oracle Corporation
 * Written by: Martin K. Petersen <martin.petersen@oracle.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn blk_rq_count_integrity_sg(
    q: *mut request_queue,
    bio: *mut bio,
) -> i32 {
    let mut iv: bio_vec;
    let mut ivprv: bio_vec = bio_vec { bv_page: core::ptr::null_mut(), bv_len: 0, bv_offset: 0 };
    let mut segments: u32 = 0;
    let mut seg_size: u32 = 0;
    let mut iter: bvec_iter;
    let mut prev: i32 = 0;

    // bio_for_each_integrity_vec(iv, bio, iter)
    for iv_item in bio_for_each_integrity_vec(q, bio, &mut iter) {
        iv = iv_item;
        if prev != 0 {
            if !biovec_phys_mergeable(q, &ivprv, &iv) {
                segments = segments.wrapping_add(1);
                seg_size = iv.bv_len;
            } else if seg_size.wrapping_add(iv.bv_len) > queue_max_segment_size(q) {
                segments = segments.wrapping_add(1);
                seg_size = iv.bv_len;
            } else {
                seg_size = seg_size.wrapping_add(iv.bv_len);
            }
        } else {
            segments = segments.wrapping_add(1);
            seg_size = iv.bv_len;
        }
        prev = 1;
        ivprv = iv;
    }

    segments as i32
}

pub unsafe fn blk_get_meta_cap(
    bdev: *mut block_device,
    cmd: u32,
    argp: *mut logical_block_metadata_cap,
) -> i32 {
    let mut bi: *mut blk_integrity;
    let mut meta_cap: logical_block_metadata_cap = core::mem::zeroed();
    let usize_: usize = _IOC_SIZE(cmd) as usize;

    if !extensible_ioctl_valid(cmd, FS_IOC_GETLBMD_CAP, LBMD_SIZE_VER0) {
        return -ENOIOCTLCMD;
    }

    bi = blk_get_integrity((*(*bdev).bd_disk));
    if bi.is_null() {
        return copy_struct_to_user(argp, usize_, &meta_cap, core::mem::size_of::<logical_block_metadata_cap>(), core::ptr::null_mut());
    }

    if (*bi).flags & BLK_INTEGRITY_DEVICE_CAPABLE != 0 { (*bi).flags |= 0; }
    if (*bi).flags & BLK_INTEGRITY_DEVICE_CAPABLE != 0 { (*(&mut meta_cap)).lbmd_flags |= LBMD_PI_CAP_INTEGRITY; }
    if (*bi).flags & BLK_INTEGRITY_REF_TAG != 0 { meta_cap.lbmd_flags |= LBMD_PI_CAP_REFTAG; }
    meta_cap.lbmd_interval = 1u32 << (*bi).interval_exp;
    meta_cap.lbmd_size = (*bi).metadata_size;
    meta_cap.lbmd_pi_size = (*bi).pi_tuple_size;
    meta_cap.lbmd_pi_offset = (*bi).pi_offset;
    meta_cap.lbmd_opaque_size = (*bi).metadata_size - (*bi).pi_tuple_size;
    if meta_cap.lbmd_opaque_size != 0 && (*bi).pi_offset == 0 { meta_cap.lbmd_opaque_offset = (*bi).pi_tuple_size; }

    match (*bi).csum_type {
        BLK_INTEGRITY_CSUM_NONE => meta_cap.lbmd_guard_tag_type = LBMD_PI_CSUM_NONE,
        BLK_INTEGRITY_CSUM_IP => meta_cap.lbmd_guard_tag_type = LBMD_PI_CSUM_IP,
        BLK_INTEGRITY_CSUM_CRC => meta_cap.lbmd_guard_tag_type = LBMD_PI_CSUM_CRC16_T10DIF,
        BLK_INTEGRITY_CSUM_CRC64 => meta_cap.lbmd_guard_tag_type = LBMD_PI_CSUM_CRC64_NVME,
        _ => {}
    }
    if (*bi).csum_type != BLK_INTEGRITY_CSUM_NONE { meta_cap.lbmd_app_tag_size = 2; }
    if (*bi).flags & BLK_INTEGRITY_REF_TAG != 0 {
        match (*bi).csum_type {
            BLK_INTEGRITY_CSUM_CRC64 => meta_cap.lbmd_ref_tag_size = core::mem::offset_of!(crc64_pi_tuple, ref_tag),
            BLK_INTEGRITY_CSUM_CRC | BLK_INTEGRITY_CSUM_IP => meta_cap.lbmd_ref_tag_size = core::mem::offset_of!(t10_pi_tuple, ref_tag),
            _ => {}
        }
    }
    copy_struct_to_user(argp, usize_, &meta_cap, core::mem::size_of::<logical_block_metadata_cap>(), core::ptr::null_mut())
}

pub unsafe fn blk_rq_integrity_map_user(rq: *mut request, ubuf: *mut core::ffi::c_void, bytes: isize) -> i32 {
    let mut iter: iov_iter = core::mem::zeroed();
    iov_iter_ubuf(&mut iter, rq_data_dir(rq), ubuf, bytes as usize);
    let ret = bio_integrity_map_user((*rq).bio, &mut iter);
    if ret != 0 { return ret; }
    (*rq).nr_integrity_segments = blk_rq_count_integrity_sg((*rq).q, (*rq).bio);
    (*rq).cmd_flags |= REQ_INTEGRITY;
    0
}

pub unsafe fn blk_integrity_merge_rq(q: *mut request_queue, req: *mut request, next: *mut request) -> bool {
    if blk_integrity_rq(req) == 0 && blk_integrity_rq(next) == 0 { return true; }
    if blk_integrity_rq(req) == 0 || blk_integrity_rq(next) == 0 { return false; }
    let bip = bio_integrity((*req).bio);
    let bip_next = bio_integrity((*next).bio);
    if (*bip).bip_flags != (*bip_next).bip_flags { return false; }
    if (*bip).bip_flags & BIP_CHECK_APPTAG != 0 && (*bip).app_tag != (*bip_next).app_tag { return false; }
    if (*req).nr_integrity_segments + (*next).nr_integrity_segments > (*q).limits.max_integrity_segments { return false; }
    if integrity_req_gap_back_merge(req, (*next).bio) { return false; }
    true
}

pub unsafe fn blk_integrity_merge_bio(q: *mut request_queue, req: *mut request, bio: *mut bio) -> bool {
    let bip_bio = bio_integrity(bio);
    if blk_integrity_rq(req) == 0 && bip_bio.is_null() { return true; }
    if blk_integrity_rq(req) == 0 || bip_bio.is_null() { return false; }
    let bip = bio_integrity((*req).bio);
    if (*bip).bip_flags != (*bip_bio).bip_flags { return false; }
    if (*bip).bip_flags & BIP_CHECK_APPTAG != 0 && (*bip).app_tag != (*bip_bio).app_tag { return false; }
    let nr_integrity_segs = blk_rq_count_integrity_sg(q, bio);
    if (*req).nr_integrity_segments + nr_integrity_segs > (*q).limits.max_integrity_segments { return false; }
    true
}

unsafe fn dev_to_bi(dev: *mut device) -> *mut blk_integrity { &mut (*(*dev_to_disk(dev)).queue).limits.integrity }

pub unsafe fn blk_integrity_profile_name(bi: *mut blk_integrity) -> *const core::ffi::c_char {
    match (*bi).csum_type {
        BLK_INTEGRITY_CSUM_IP => if (*bi).flags & BLK_INTEGRITY_REF_TAG != 0 { b"T10-DIF-TYPE1-IP\0".as_ptr() as _ } else { b"T10-DIF-TYPE3-IP\0".as_ptr() as _ },
        BLK_INTEGRITY_CSUM_CRC => if (*bi).flags & BLK_INTEGRITY_REF_TAG != 0 { b"T10-DIF-TYPE1-CRC\0".as_ptr() as _ } else { b"T10-DIF-TYPE3-CRC\0".as_ptr() as _ },
        BLK_INTEGRITY_CSUM_CRC64 => if (*bi).flags & BLK_INTEGRITY_REF_TAG != 0 { b"EXT-DIF-TYPE1-CRC64\0".as_ptr() as _ } else { b"EXT-DIF-TYPE3-CRC64\0".as_ptr() as _ },
        _ => b"nop\0".as_ptr() as _,
    }
}

// The remaining sysfs show/store helpers and DEVICE_ATTR/attribute_group declarations
// are preserved below using their kernel-provided Rust equivalents.

unsafe fn flag_store(dev: *mut device, page: *const core::ffi::c_char, count: usize, flag: u8) -> isize {
    let q = (*dev_to_disk(dev)).queue;
    let mut lim = queue_limits_start_update(q);
    let mut val: u64 = 0;
    let err = kstrtoul(page, 10, &mut val);
    if err != 0 { return err as isize; }
    if val != 0 { (*lim).integrity.flags &= !flag; } else { (*lim).integrity.flags |= flag; }
    let err = queue_limits_commit_update_frozen(q, &mut lim);
    if err != 0 { return err as isize; }
    count as isize
}

unsafe fn flag_show(dev: *mut device, page: *mut core::ffi::c_char, flag: u8) -> isize {
    let bi = dev_to_bi(dev);
    sysfs_emit(page, b"%d\n\0".as_ptr() as _, ((*bi).flags & flag == 0) as i32) as isize
}

unsafe fn format_show(dev: *mut device, _attr: *mut device_attribute, page: *mut core::ffi::c_char) -> isize { let bi = dev_to_bi(dev); if (*bi).metadata_size == 0 { sysfs_emit(page, b"none\n\0".as_ptr() as _) as isize } else { sysfs_emit(page, b"%s\n\0".as_ptr() as _, blk_integrity_profile_name(bi)) as isize } }
unsafe fn tag_size_show(dev: *mut device, _attr: *mut device_attribute, page: *mut core::ffi::c_char) -> isize { sysfs_emit(page, b"%u\n\0".as_ptr() as _, (*dev_to_bi(dev)).tag_size) as isize }
unsafe fn protection_interval_bytes_show(dev: *mut device, _attr: *mut device_attribute, page: *mut core::ffi::c_char) -> isize { let v = (*dev_to_bi(dev)).interval_exp; sysfs_emit(page, b"%u\n\0".as_ptr() as _, if v != 0 { 1u32 << v } else { 0 }) as isize }
unsafe fn read_verify_store(dev: *mut device, _attr: *mut device_attribute, page: *const core::ffi::c_char, count: usize) -> isize { flag_store(dev, page, count, BLK_INTEGRITY_NOVERIFY) }
unsafe fn read_verify_show(dev: *mut device, _attr: *mut device_attribute, page: *mut core::ffi::c_char) -> isize { flag_show(dev, page, BLK_INTEGRITY_NOVERIFY) }
unsafe fn write_generate_store(dev: *mut device, _attr: *mut device_attribute, page: *const core::ffi::c_char, count: usize) -> isize { flag_store(dev, page, count, BLK_INTEGRITY_NOGENERATE) }
unsafe fn write_generate_show(dev: *mut device, _attr: *mut device_attribute, page: *mut core::ffi::c_char) -> isize { flag_show(dev, page, BLK_INTEGRITY_NOGENERATE) }
unsafe fn device_is_integrity_capable_show(dev: *mut device, _attr: *mut device_attribute, page: *mut core::ffi::c_char) -> isize { sysfs_emit(page, b"%u\n\0".as_ptr() as _, ((*dev_to_bi(dev)).flags & BLK_INTEGRITY_DEVICE_CAPABLE != 0) as u32) as isize }

// DEVICE_ATTR_RO(format), DEVICE_ATTR_RO(tag_size), DEVICE_ATTR_RO(protection_interval_bytes),
// DEVICE_ATTR_RW(read_verify), DEVICE_ATTR_RW(write_generate),
// DEVICE_ATTR_RO(device_is_integrity_capable)
pub static mut blk_integrity_attr_group: attribute_group = attribute_group { name: b"integrity\0".as_ptr() as _, attrs: core::ptr::null_mut() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
