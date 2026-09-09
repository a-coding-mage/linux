// SPDX-License-Identifier: GPL-2.0
/* Functions related to setting various queue properties from drivers. */

// External Linux kernel types, constants, macros, and functions are supplied by
// the surrounding translation unit.

pub unsafe fn blk_queue_rq_timeout(q: *mut request_queue, timeout: u32) {
    (*q).rq_timeout = timeout;
}

pub unsafe fn blk_set_stacking_limits(lim: *mut queue_limits) {
    core::ptr::write_bytes(lim, 0, 1);
    (*lim).logical_block_size = SECTOR_SIZE;
    (*lim).physical_block_size = SECTOR_SIZE;
    (*lim).io_min = SECTOR_SIZE;
    (*lim).discard_granularity = SECTOR_SIZE;
    (*lim).dma_alignment = SECTOR_SIZE - 1;
    (*lim).seg_boundary_mask = BLK_SEG_BOUNDARY_MASK;
    (*lim).max_segments = USHRT_MAX;
    (*lim).max_discard_segments = USHRT_MAX;
    (*lim).max_hw_sectors = UINT_MAX;
    (*lim).max_segment_size = UINT_MAX;
    (*lim).max_sectors = UINT_MAX;
    (*lim).max_dev_sectors = UINT_MAX;
    (*lim).max_write_zeroes_sectors = UINT_MAX;
    (*lim).max_hw_wzeroes_unmap_sectors = UINT_MAX;
    (*lim).max_user_wzeroes_unmap_sectors = UINT_MAX;
    (*lim).max_hw_zone_append_sectors = UINT_MAX;
    (*lim).max_user_discard_sectors = UINT_MAX;
    (*lim).atomic_write_hw_max = UINT_MAX;
}

pub unsafe fn blk_apply_bdi_limits(bdi: *mut backing_dev_info, lim: *mut queue_limits) {
    let mut io_opt = (*lim).io_opt as u64;
    if io_opt == 0 && ((*lim).features & BLK_FEAT_ROTATIONAL) != 0 {
        io_opt = ((*lim).max_sectors as u64) << SECTOR_SHIFT;
    }
    (*bdi).ra_pages = max3((*bdi).ra_pages, (io_opt * 2) >> PAGE_SHIFT, VM_READAHEAD_PAGES);
    (*bdi).io_pages = (*lim).max_sectors >> PAGE_SECTORS_SHIFT;
}

unsafe fn blk_validate_zoned_limits(lim: *mut queue_limits) -> i32 {
    if ((*lim).features & BLK_FEAT_ZONED) == 0 {
        if (*lim).max_open_zones != 0 || (*lim).max_active_zones != 0 ||
            (*lim).zone_write_granularity != 0 || (*lim).max_zone_append_sectors != 0 { return -EINVAL; }
        return 0;
    }
    if (*lim).max_active_zones != 0 && (*lim).max_open_zones > (*lim).max_active_zones { return -EINVAL; }
    if (*lim).zone_write_granularity < (*lim).logical_block_size { (*lim).zone_write_granularity = (*lim).logical_block_size; }
    (*lim).max_zone_append_sectors = min_not_zero((*lim).max_hw_zone_append_sectors, min((*lim).chunk_sectors, (*lim).max_hw_sectors));
    0
}

unsafe fn blk_validate_integrity_limits(lim: *mut queue_limits) -> i32 {
    let bi = &mut (*lim).integrity;
    if bi.metadata_size == 0 {
        if bi.csum_type != BLK_INTEGRITY_CSUM_NONE || bi.tag_size != 0 || (bi.flags & BLK_INTEGRITY_REF_TAG) != 0 { return -EINVAL; }
        bi.flags |= BLK_INTEGRITY_NOGENERATE | BLK_INTEGRITY_NOVERIFY;
        return 0;
    }
    if bi.csum_type == BLK_INTEGRITY_CSUM_NONE && (bi.flags & BLK_INTEGRITY_REF_TAG) != 0 { return -EINVAL; }
    if bi.pi_offset + bi.pi_tuple_size > bi.metadata_size { return -EINVAL; }
    match bi.csum_type {
        BLK_INTEGRITY_CSUM_NONE if bi.pi_tuple_size != 0 => return -EINVAL,
        BLK_INTEGRITY_CSUM_CRC | BLK_INTEGRITY_CSUM_IP if bi.pi_tuple_size != core::mem::size_of::<t10_pi_tuple>() as u32 => return -EINVAL,
        BLK_INTEGRITY_CSUM_CRC64 if bi.pi_tuple_size != core::mem::size_of::<crc64_pi_tuple>() as u32 => return -EINVAL,
        _ => {}
    }
    if bi.interval_exp == 0 { bi.interval_exp = ilog2((*lim).logical_block_size); }
    else if bi.interval_exp < SECTOR_SHIFT || bi.interval_exp > ilog2((*lim).logical_block_size) { return -EINVAL; }
    if (bi.flags & BLK_SPLIT_INTERVAL_CAPABLE) == 0 && bi.csum_type != 0 { (*lim).dma_alignment = max((*lim).dma_alignment, (1u32 << bi.interval_exp) - 1); }
    (*lim).max_sectors = min((*lim).max_sectors, max_integrity_io_size(lim) >> SECTOR_SHIFT);
    0
}

unsafe fn blk_queue_max_guaranteed_bio(lim: *mut queue_limits) -> u32 {
    let max_segments = min(BIO_MAX_VECS, (*lim).max_segments);
    let mut length = min(max_segments, 2) * (*lim).logical_block_size;
    if max_segments > 2 { length += (max_segments - 2) * PAGE_SIZE; }
    length
}

unsafe fn blk_atomic_writes_update_limits(lim: *mut queue_limits) {
    let unit_limit = rounddown_pow_of_two(min((*lim).max_hw_sectors << SECTOR_SHIFT, blk_queue_max_guaranteed_bio(lim)));
    (*lim).atomic_write_max_sectors = min((*lim).atomic_write_hw_max >> SECTOR_SHIFT, (*lim).max_hw_sectors);
    (*lim).atomic_write_unit_min = min((*lim).atomic_write_hw_unit_min, unit_limit);
    (*lim).atomic_write_unit_max = min((*lim).atomic_write_hw_unit_max, unit_limit);
    (*lim).atomic_write_boundary_sectors = (*lim).atomic_write_hw_boundary >> SECTOR_SHIFT;
}

unsafe fn blk_valid_atomic_writes_boundary(chunk: u32, boundary: u32) -> bool {
    if chunk == 0 || boundary == 0 { return true; }
    if boundary > chunk && boundary % chunk != 0 { return false; }
    if chunk > boundary && chunk % boundary != 0 { return false; }
    true
}

unsafe fn blk_validate_atomic_write_limits(lim: *mut queue_limits) {
    let unsupported = (*lim).features & BLK_FEAT_ATOMIC_WRITES == 0 || (*lim).atomic_write_hw_max == UINT_MAX || (*lim).atomic_write_hw_max == 0 ||
        !is_power_of_2((*lim).atomic_write_hw_unit_min) || !is_power_of_2((*lim).atomic_write_hw_unit_max) ||
        (*lim).atomic_write_hw_unit_min > (*lim).atomic_write_hw_unit_max || (*lim).atomic_write_hw_unit_max > (*lim).atomic_write_hw_max ||
        ((*lim).chunk_sectors != 0 && ((*lim).atomic_write_hw_max >> SECTOR_SHIFT) > (*lim).chunk_sectors);
    if unsupported { (*lim).atomic_write_max_sectors=0; (*lim).atomic_write_boundary_sectors=0; (*lim).atomic_write_unit_min=0; (*lim).atomic_write_unit_max=0; return; }
    let boundary = (*lim).atomic_write_hw_boundary >> SECTOR_SHIFT;
    if boundary != 0 && ((*lim).atomic_write_hw_max > (*lim).atomic_write_hw_boundary || !blk_valid_atomic_writes_boundary((*lim).chunk_sectors, boundary) || !is_power_of_2(boundary)) {
        (*lim).atomic_write_max_sectors=0; (*lim).atomic_write_boundary_sectors=0; (*lim).atomic_write_unit_min=0; (*lim).atomic_write_unit_max=0; return;
    }
    blk_atomic_writes_update_limits(lim);
}

pub unsafe fn blk_validate_limits(lim: *mut queue_limits) -> i32 {
    if (*lim).logical_block_size == 0 { (*lim).logical_block_size = SECTOR_SIZE; } else if blk_validate_block_size((*lim).logical_block_size) != 0 { return -EINVAL; }
    if (*lim).physical_block_size < (*lim).logical_block_size { (*lim).physical_block_size = (*lim).logical_block_size; } else if !is_power_of_2((*lim).physical_block_size) { return -EINVAL; }
    (*lim).io_min = max((*lim).io_min, (*lim).physical_block_size);
    (*lim).io_opt = round_down((*lim).io_opt, (*lim).physical_block_size);
    if (*lim).max_hw_sectors == 0 { (*lim).max_hw_sectors = BLK_SAFE_MAX_SECTORS; }
    if (*lim).max_hw_sectors < PAGE_SECTORS { return -EINVAL; }
    let logical = (*lim).logical_block_size >> SECTOR_SHIFT;
    if logical > (*lim).max_hw_sectors { return -EINVAL; }
    (*lim).max_hw_sectors = round_down((*lim).max_hw_sectors, logical);
    let max_hw = min_not_zero((*lim).max_hw_sectors, (*lim).max_dev_sectors);
    (*lim).max_sectors = if (*lim).max_user_sectors != 0 { if (*lim).max_user_sectors < BLK_MIN_SEGMENT_SIZE / SECTOR_SIZE { return -EINVAL; } min(max_hw, (*lim).max_user_sectors) } else if (*lim).io_opt > (BLK_DEF_MAX_SECTORS_CAP << SECTOR_SHIFT) { min(max_hw, (*lim).io_opt >> SECTOR_SHIFT) } else if (*lim).io_min > (BLK_DEF_MAX_SECTORS_CAP << SECTOR_SHIFT) { min(max_hw, (*lim).io_min >> SECTOR_SHIFT) } else { min(max_hw, BLK_DEF_MAX_SECTORS_CAP) };
    (*lim).max_sectors = round_down((*lim).max_sectors, logical);
    if (*lim).max_segments == 0 { (*lim).max_segments = BLK_MAX_SEGMENTS; }
    if (*lim).max_hw_wzeroes_unmap_sectors != 0 && (*lim).max_hw_wzeroes_unmap_sectors != (*lim).max_write_zeroes_sectors { return -EINVAL; }
    (*lim).max_wzeroes_unmap_sectors = min((*lim).max_hw_wzeroes_unmap_sectors, (*lim).max_user_wzeroes_unmap_sectors);
    (*lim).max_discard_sectors = min((*lim).max_hw_discard_sectors, (*lim).max_user_discard_sectors);
    (*lim).discard_granularity = if (*lim).max_discard_sectors != 0 { max((*lim).discard_granularity, (*lim).physical_block_size) } else { 0 };
    if (*lim).max_discard_segments == 0 { (*lim).max_discard_segments = 1; }
    if (*lim).seg_boundary_mask == 0 { (*lim).seg_boundary_mask = BLK_SEG_BOUNDARY_MASK; }
    if (*lim).seg_boundary_mask < BLK_MIN_SEGMENT_SIZE - 1 { return -EINVAL; }
    if (*lim).virt_boundary_mask != 0 { if (*lim).max_segment_size == 0 { (*lim).max_segment_size = UINT_MAX; } } else { if (*lim).max_segment_size == 0 { (*lim).max_segment_size = BLK_MAX_SEGMENT_SIZE; } if (*lim).max_segment_size < BLK_MIN_SEGMENT_SIZE { return -EINVAL; } }
    let seg_size = if (*lim).seg_boundary_mask > (*lim).max_segment_size - 1 { (*lim).max_segment_size } else { (*lim).seg_boundary_mask + 1 };
    (*lim).max_fast_segment_size = min(seg_size, PAGE_SIZE);
    if (*lim).dma_alignment == 0 { (*lim).dma_alignment = SECTOR_SIZE - 1; }
    if (*lim).dma_alignment > PAGE_SIZE { return -EINVAL; }
    if (*lim).alignment_offset != 0 { (*lim).alignment_offset &= (*lim).physical_block_size - 1; (*lim).flags &= !BLK_FLAG_MISALIGNED; }
    if (*lim).features & BLK_FEAT_WRITE_CACHE == 0 { (*lim).features &= !BLK_FEAT_FUA; }
    blk_validate_atomic_write_limits(lim);
    let err = blk_validate_integrity_limits(lim); if err != 0 { return err; }
    blk_validate_zoned_limits(lim)
}

pub unsafe fn blk_set_default_limits(lim: *mut queue_limits) -> i32 { (*lim).max_user_discard_sectors=UINT_MAX; (*lim).max_user_wzeroes_unmap_sectors=UINT_MAX; blk_validate_limits(lim) }

pub unsafe fn queue_limits_set(q: *mut request_queue, lim: *mut queue_limits) -> i32 { mutex_lock(&mut (*q).limits_lock); queue_limits_commit_update(q, lim) }

pub unsafe fn queue_limits_commit_update(q: *mut request_queue, lim: *mut queue_limits) -> i32 {
    let error = blk_validate_limits(lim); if error != 0 { mutex_unlock(&mut (*q).limits_lock); return error; }
    (*q).limits = *lim;
    if !(*q).disk.is_null() { blk_apply_bdi_limits((*(*q).disk).bdi, lim); }
    mutex_unlock(&mut (*q).limits_lock); 0
}

pub unsafe fn queue_limits_commit_update_frozen(q: *mut request_queue, lim: *mut queue_limits) -> i32 { let flags=blk_mq_freeze_queue(q); let ret=queue_limits_commit_update(q,lim); blk_mq_unfreeze_queue(q,flags); ret }

unsafe fn queue_limit_alignment_offset(lim: *const queue_limits, mut sector: sector_t) -> u32 {
    let granularity = max((*lim).physical_block_size, (*lim).io_min);
    let alignment = sector_div(&mut sector, granularity >> SECTOR_SHIFT) << SECTOR_SHIFT;
    (granularity + (*lim).alignment_offset - alignment) % granularity
}

unsafe fn queue_limit_discard_alignment(lim: *const queue_limits, mut sector: sector_t) -> u32 {
    if (*lim).max_discard_sectors == 0 { return 0; }
    let alignment = (*lim).discard_alignment >> SECTOR_SHIFT;
    let granularity = (*lim).discard_granularity >> SECTOR_SHIFT;
    let offset = sector_div(&mut sector, granularity);
    ((granularity + alignment - offset) % granularity) << SECTOR_SHIFT
}

unsafe fn blk_round_down_sectors(mut sectors: u32, lbs: u32) -> u32 {
    sectors = round_down(sectors, lbs >> SECTOR_SHIFT);
    if sectors < PAGE_SIZE >> SECTOR_SHIFT { sectors = PAGE_SIZE >> SECTOR_SHIFT; }
    sectors
}

unsafe fn blk_stack_atomic_writes_limits(t: *mut queue_limits, b: *mut queue_limits, start: sector_t) {
    if (*b).features & BLK_FEAT_ATOMIC_WRITES == 0 || (*b).atomic_write_hw_unit_min == 0 || !blk_atomic_write_start_sect_aligned(start,b) { (*t).atomic_write_hw_max=0; (*t).atomic_write_hw_unit_max=0; (*t).atomic_write_hw_unit_min=0; (*t).atomic_write_hw_boundary=0; return; }
    if (*t).atomic_write_hw_max == UINT_MAX { (*t).atomic_write_hw_unit_max=(*b).atomic_write_hw_unit_max; (*t).atomic_write_hw_unit_min=(*b).atomic_write_hw_unit_min; (*t).atomic_write_hw_max=(*b).atomic_write_hw_max; (*t).atomic_write_hw_boundary=(*b).atomic_write_hw_boundary; }
    else { if (*t).atomic_write_hw_boundary != (*b).atomic_write_hw_boundary || (*t).atomic_write_hw_unit_min > (*b).atomic_write_hw_unit_max || (*t).atomic_write_hw_unit_max < (*b).atomic_write_hw_unit_min { (*t).atomic_write_hw_max=0; (*t).atomic_write_hw_unit_max=0; (*t).atomic_write_hw_unit_min=0; (*t).atomic_write_hw_boundary=0; return; } (*t).atomic_write_hw_max=min((*t).atomic_write_hw_max,(*b).atomic_write_hw_max); (*t).atomic_write_hw_unit_min=max((*t).atomic_write_hw_unit_min,(*b).atomic_write_hw_unit_min); (*t).atomic_write_hw_unit_max=min((*t).atomic_write_hw_unit_max,(*b).atomic_write_hw_unit_max); }
}

pub unsafe fn blk_stack_limits(t: *mut queue_limits, b: *mut queue_limits, start: sector_t) -> i32 {
    (*t).features |= (*b).features & BLK_FEAT_INHERIT_MASK;
    for (flag, keep) in [(BLK_FEAT_NOWAIT,true),(BLK_FEAT_POLL,true),(BLK_FEAT_PCI_P2PDMA,true)] { if (*b).features & flag == 0 { (*t).features &= !flag; } let _=keep; }
    (*t).flags |= (*b).flags & BLK_FLAG_MISALIGNED;
    (*t).max_sectors=min_not_zero((*t).max_sectors,(*b).max_sectors); (*t).max_user_sectors=min_not_zero((*t).max_user_sectors,(*b).max_user_sectors); (*t).max_hw_sectors=min_not_zero((*t).max_hw_sectors,(*b).max_hw_sectors); (*t).max_dev_sectors=min_not_zero((*t).max_dev_sectors,(*b).max_dev_sectors);
    (*t).max_segments=min_not_zero((*t).max_segments,(*b).max_segments); (*t).max_discard_segments=min_not_zero((*t).max_discard_segments,(*b).max_discard_segments); (*t).max_segment_size=min_not_zero((*t).max_segment_size,(*b).max_segment_size);
    let alignment=queue_limit_alignment_offset(b,start); (*t).logical_block_size=max((*t).logical_block_size,(*b).logical_block_size); (*t).physical_block_size=max((*t).physical_block_size,(*b).physical_block_size); (*t).io_min=max((*t).io_min,(*b).io_min); (*t).io_opt=lcm_not_zero((*t).io_opt,(*b).io_opt); (*t).dma_alignment=max((*t).dma_alignment,(*b).dma_alignment);
    if (*b).chunk_sectors != 0 { (*t).chunk_sectors=gcd((*t).chunk_sectors,(*b).chunk_sectors); }
    if (*t).physical_block_size & ((*t).logical_block_size-1) != 0 { (*t).physical_block_size=(*t).logical_block_size; (*t).flags|=BLK_FLAG_MISALIGNED; }
    if (*t).io_min & ((*t).physical_block_size-1) != 0 { (*t).io_min=(*t).physical_block_size; (*t).flags|=BLK_FLAG_MISALIGNED; }
    if (*t).io_opt & ((*t).physical_block_size-1) != 0 { (*t).io_opt=0; (*t).flags|=BLK_FLAG_MISALIGNED; }
    (*t).alignment_offset=lcm_not_zero((*t).alignment_offset,alignment)%max((*t).physical_block_size,(*t).io_min);
    (*t).max_sectors=blk_round_down_sectors((*t).max_sectors,(*t).logical_block_size); (*t).max_hw_sectors=blk_round_down_sectors((*t).max_hw_sectors,(*t).logical_block_size); (*t).max_dev_sectors=blk_round_down_sectors((*t).max_dev_sectors,(*t).logical_block_size);
    (*t).zone_write_granularity=max((*t).zone_write_granularity,(*b).zone_write_granularity); if (*t).features & BLK_FEAT_ZONED == 0 { (*t).zone_write_granularity=0; (*t).max_zone_append_sectors=0; } blk_stack_atomic_writes_limits(t,b,start); 0
}

pub unsafe fn queue_limits_stack_integrity(t: *mut queue_limits, b: *mut queue_limits) -> bool {
    let ti=&mut (*t).integrity; let bi=&(*b).integrity;
    if ti.flags & BLK_INTEGRITY_STACKED != 0 {
        if ti.metadata_size != bi.metadata_size || ti.interval_exp != bi.interval_exp || ti.tag_size != bi.tag_size || ti.csum_type != bi.csum_type || ti.pi_tuple_size != bi.pi_tuple_size || (ti.flags & BLK_INTEGRITY_REF_TAG) != (bi.flags & BLK_INTEGRITY_REF_TAG) { core::ptr::write_bytes(ti,0,1); return false; }
        if ti.flags & BLK_SPLIT_INTERVAL_CAPABLE != 0 && bi.flags & BLK_SPLIT_INTERVAL_CAPABLE == 0 { ti.flags &= !BLK_SPLIT_INTERVAL_CAPABLE; }
    } else { ti.flags=BLK_INTEGRITY_STACKED | (bi.flags & (BLK_INTEGRITY_DEVICE_CAPABLE|BLK_INTEGRITY_REF_TAG|BLK_SPLIT_INTERVAL_CAPABLE)); ti.csum_type=bi.csum_type; ti.pi_tuple_size=bi.pi_tuple_size; ti.metadata_size=bi.metadata_size; ti.pi_offset=bi.pi_offset; ti.interval_exp=bi.interval_exp; ti.tag_size=bi.tag_size; }
    true
}

pub unsafe fn blk_set_queue_depth(q: *mut request_queue, depth: u32) { (*q).queue_depth=depth; rq_qos_queue_depth_changed(q); }

pub unsafe fn bdev_alignment_offset(bdev: *mut block_device) -> i32 { let q=bdev_get_queue(bdev); if (*q).limits.flags & BLK_FLAG_MISALIGNED != 0 { -1 } else if bdev_is_partition(bdev) { queue_limit_alignment_offset(&(*q).limits, (*bdev).bd_start_sect) as i32 } else { (*q).limits.alignment_offset as i32 } }

pub unsafe fn bdev_discard_alignment(bdev: *mut block_device) -> u32 { let q=bdev_get_queue(bdev); if bdev_is_partition(bdev) { queue_limit_discard_alignment(&(*q).limits, (*bdev).bd_start_sect) } else { (*q).limits.discard_alignment } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
