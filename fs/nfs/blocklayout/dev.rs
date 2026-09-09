// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014-2016 Christoph Hellwig.
 */
// Linux/kernel dependencies from the original includes are supplied externally.

const NFSDBG_FACILITY: u32 = NFSDBG_PNFS_LD;

unsafe fn bl_unregister_scsi(dev: *mut pnfs_block_dev) {
    let bdev = file_bdev((*dev).bdev_file);
    let ops = (*(*bdev).bd_disk).fops.pr_ops;
    let status = ((*ops).pr_register)(bdev, (*dev).pr_key, 0, false);

    if status != 0 {
        trace_bl_pr_key_unreg_err(bdev, (*dev).pr_key, status);
    } else {
        trace_bl_pr_key_unreg(bdev, (*dev).pr_key);
    }
}

unsafe fn bl_register_scsi(dev: *mut pnfs_block_dev) -> bool {
    let bdev = file_bdev((*dev).bdev_file);
    let ops = (*(*bdev).bd_disk).fops.pr_ops;

    if test_and_set_bit(PNFS_BDEV_REGISTERED, &mut (*dev).flags) {
        return true;
    }

    let status = ((*ops).pr_register)(bdev, 0, (*dev).pr_key, true);
    if status != 0 {
        trace_bl_pr_key_reg_err(bdev, (*dev).pr_key, status);
        return false;
    }
    trace_bl_pr_key_reg(bdev, (*dev).pr_key);
    true
}

unsafe fn bl_unregister_dev(dev: *mut pnfs_block_dev) {
    if (*dev).nr_children != 0 {
        for i in 0..(*dev).nr_children {
            bl_unregister_dev((*dev).children.add(i as usize));
        }
        return;
    }

    if (*dev).type_ == PNFS_BLOCK_VOLUME_SCSI
        && test_and_clear_bit(PNFS_BDEV_REGISTERED, &mut (*dev).flags)
    {
        bl_unregister_scsi(dev);
    }
}

pub unsafe fn bl_register_dev(dev: *mut pnfs_block_dev) -> bool {
    if (*dev).nr_children != 0 {
        let mut i = 0;
        while i < (*dev).nr_children {
            if !bl_register_dev((*dev).children.add(i as usize)) {
                while i > 0 {
                    i -= 1;
                    bl_unregister_dev((*dev).children.add(i as usize));
                }
                return false;
            }
            i += 1;
        }
        return true;
    }

    if (*dev).type_ == PNFS_BLOCK_VOLUME_SCSI {
        return bl_register_scsi(dev);
    }
    true
}

unsafe fn bl_free_device(dev: *mut pnfs_block_dev) {
    bl_unregister_dev(dev);

    if !(*dev).children.is_null() {
        for i in 0..(*dev).nr_children {
            bl_free_device((*dev).children.add(i as usize));
        }
        kfree((*dev).children as *mut _);
        (*dev).children = core::ptr::null_mut();
        (*dev).nr_children = 0;
    } else if !(*dev).bdev_file.is_null() {
        fput((*dev).bdev_file);
        (*dev).bdev_file = core::ptr::null_mut();
    }
}

pub unsafe fn bl_free_deviceid_node(d: *mut nfs4_deviceid_node) {
    let dev = container_of!(d, pnfs_block_dev, node);
    bl_free_device(dev);
    kfree_rcu!(dev, node.rcu);
}

unsafe fn nfs4_block_decode_volume(xdr: *mut xdr_stream, b: *mut pnfs_block_volume) -> i32 {
    let mut p = xdr_inline_decode(xdr, 4);
    if p.is_null() { return -EIO; }
    (*b).type_ = be32_to_cpup(p);
    p = p.add(1);

    match (*b).type_ {
        PNFS_BLOCK_VOLUME_SIMPLE => {
            p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; }
            (*b).simple.nr_sigs = be32_to_cpup(p);
            if (*b).simple.nr_sigs == 0 || (*b).simple.nr_sigs > PNFS_BLOCK_MAX_UUIDS { return -EIO; }
            (*b).simple.len = 8;
            for i in 0..(*b).simple.nr_sigs {
                p = xdr_inline_decode(xdr, 12); if p.is_null() { return -EIO; }
                p = xdr_decode_hyper(p, &mut (*b).simple.sigs[i as usize].offset);
                (*b).simple.sigs[i as usize].sig_len = be32_to_cpup(p); p = p.add(1);
                if (*b).simple.sigs[i as usize].sig_len > PNFS_BLOCK_UUID_LEN { return -EIO; }
                p = xdr_inline_decode(xdr, (*b).simple.sigs[i as usize].sig_len as usize);
                if p.is_null() { return -EIO; }
                memcpy(&mut (*b).simple.sigs[i as usize].sig as *mut _, p, (*b).simple.sigs[i as usize].sig_len as usize);
                (*b).simple.len += 12 + (XDR_QUADLEN((*b).simple.sigs[i as usize].sig_len) << 2);
            }
        }
        PNFS_BLOCK_VOLUME_SLICE => {
            p = xdr_inline_decode(xdr, 20); if p.is_null() { return -EIO; }
            p = xdr_decode_hyper(p, &mut (*b).slice.start);
            p = xdr_decode_hyper(p, &mut (*b).slice.len);
            (*b).slice.volume = be32_to_cpup(p);
        }
        PNFS_BLOCK_VOLUME_CONCAT => {
            p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; }
            (*b).concat.volumes_count = be32_to_cpup(p); p = p.add(1);
            if (*b).concat.volumes_count > PNFS_BLOCK_MAX_DEVICES { return -EIO; }
            p = xdr_inline_decode(xdr, (*b).concat.volumes_count as usize * 4); if p.is_null() { return -EIO; }
            for i in 0..(*b).concat.volumes_count { (*b).concat.volumes[i as usize] = be32_to_cpup(p); p = p.add(1); }
        }
        PNFS_BLOCK_VOLUME_STRIPE => {
            p = xdr_inline_decode(xdr, 12); if p.is_null() { return -EIO; }
            p = xdr_decode_hyper(p, &mut (*b).stripe.chunk_size);
            (*b).stripe.volumes_count = be32_to_cpup(p); p = p.add(1);
            if (*b).stripe.volumes_count > PNFS_BLOCK_MAX_DEVICES { return -EIO; }
            p = xdr_inline_decode(xdr, (*b).stripe.volumes_count as usize * 4); if p.is_null() { return -EIO; }
            for i in 0..(*b).stripe.volumes_count { (*b).stripe.volumes[i as usize] = be32_to_cpup(p); p = p.add(1); }
        }
        PNFS_BLOCK_VOLUME_SCSI => {
            p = xdr_inline_decode(xdr, 12); if p.is_null() { return -EIO; }
            (*b).scsi.code_set = be32_to_cpup(p); p = p.add(1);
            (*b).scsi.designator_type = be32_to_cpup(p); p = p.add(1);
            (*b).scsi.designator_len = be32_to_cpup(p); p = p.add(1);
            p = xdr_inline_decode(xdr, (*b).scsi.designator_len as usize); if p.is_null() || (*b).scsi.designator_len > 256 { return -EIO; }
            memcpy(&mut (*b).scsi.designator as *mut _, p, (*b).scsi.designator_len as usize);
            p = xdr_inline_decode(xdr, 8); if p.is_null() { return -EIO; }
            xdr_decode_hyper(p, &mut (*b).scsi.pr_key);
        }
        _ => return -EIO,
    }
    0
}

unsafe fn bl_map_simple(dev: *mut pnfs_block_dev, _offset: u64, map: *mut pnfs_block_dev_map) -> bool {
    (*map).start = (*dev).start; (*map).len = (*dev).len; (*map).disk_offset = (*dev).disk_offset;
    (*map).bdev = file_bdev((*dev).bdev_file); true
}

unsafe fn bl_map_concat(dev: *mut pnfs_block_dev, offset: u64, map: *mut pnfs_block_dev_map) -> bool {
    for i in 0..(*dev).nr_children {
        let child = (*dev).children.add(i as usize);
        if (*child).start > offset || (*child).start + (*child).len <= offset { continue; }
        ((*child).map)(child, offset - (*child).start, map); return true;
    }
    false
}

unsafe fn bl_map_stripe(dev: *mut pnfs_block_dev, mut offset: u64, map: *mut pnfs_block_dev_map) -> bool {
    let chunk = div_u64(offset, (*dev).chunk_size);
    let mut chunk_idx = 0u32;
    let disk_chunk = div_u64_rem(chunk, (*dev).nr_children, &mut chunk_idx);
    if chunk_idx >= (*dev).nr_children { return false; }
    offset = chunk * (*dev).chunk_size;
    let disk_offset = disk_chunk * (*dev).chunk_size;
    let child = (*dev).children.add(chunk_idx as usize);
    ((*child).map)(child, disk_offset, map);
    (*map).start += offset; (*map).disk_offset += disk_offset; (*map).len = (*dev).chunk_size; true
}

unsafe fn bl_validate_designator(v: *mut pnfs_block_volume) -> bool {
    match (*v).scsi.designator_type {
        PS_DESIGNATOR_EUI64 => (*v).scsi.code_set == PS_CODE_SET_BINARY &&
            ((*v).scsi.designator_len == 8 || (*v).scsi.designator_len == 10 || (*v).scsi.designator_len == 16),
        PS_DESIGNATOR_NAA => (*v).scsi.code_set == PS_CODE_SET_BINARY &&
            ((*v).scsi.designator_len == 8 || (*v).scsi.designator_len == 16),
        _ => false,
    }
}

unsafe fn bl_open_path(v: *mut pnfs_block_volume, prefix: *const i8) -> *mut file {
    let devname = kasprintf(GFP_KERNEL, b"/dev/disk/by-id/%s%*phN\0".as_ptr(), prefix, (*v).scsi.designator_len, (*v).scsi.designator);
    if devname.is_null() { return err_ptr(-ENOMEM); }
    let file = bdev_file_open_by_path(devname, BLK_OPEN_READ | BLK_OPEN_WRITE, core::ptr::null_mut(), core::ptr::null_mut());
    kfree(devname as *mut _); file
}

unsafe fn bl_parse_simple(server: *mut nfs_server, d: *mut pnfs_block_dev, volumes: *mut pnfs_block_volume, idx: i32, gfp_mask: gfp_t) -> i32 {
    let dev = bl_resolve_deviceid(server, volumes.add(idx as usize), gfp_mask);
    if dev == 0 { return -EIO; }
    let file = bdev_file_open_by_dev(dev, BLK_OPEN_READ | BLK_OPEN_WRITE, core::ptr::null_mut(), core::ptr::null_mut());
    if is_err(file) { return ptr_err(file); }
    (*d).bdev_file = file; (*d).len = bdev_nr_bytes(file_bdev(file)); (*d).map = Some(bl_map_simple); 0
}

unsafe fn bl_parse_scsi(server: *mut nfs_server, d: *mut pnfs_block_dev, volumes: *mut pnfs_block_volume, idx: i32, gfp_mask: gfp_t) -> i32 {
    let v = volumes.add(idx as usize);
    if !bl_validate_designator(v) { return -EINVAL; }
    let mut file = bl_open_path(v, b"dm-uuid-mpath-0x\0".as_ptr() as *const i8);
    if is_err(file) { file = bl_open_path(v, b"wwn-0x\0".as_ptr() as *const i8); }
    if is_err(file) { file = bl_open_path(v, b"nvme-eui.\0".as_ptr() as *const i8); }
    if is_err(file) { return ptr_err(file); }
    (*d).bdev_file = file; let bdev = file_bdev(file); (*d).len = bdev_nr_bytes(bdev);
    (*d).map = Some(bl_map_simple); (*d).pr_key = (*v).scsi.pr_key;
    if (*d).len == 0 { fput(file); (*d).bdev_file = core::ptr::null_mut(); return -ENODEV; }
    if (*(*bdev).bd_disk).fops.pr_ops.is_null() { fput(file); (*d).bdev_file = core::ptr::null_mut(); return -EINVAL; }
    0
}

unsafe fn bl_parse_slice(server: *mut nfs_server, d: *mut pnfs_block_dev, volumes: *mut pnfs_block_volume, idx: i32, gfp_mask: gfp_t) -> i32 {
    let v = volumes.add(idx as usize); let ret = bl_parse_deviceid(server, d, volumes, (*v).slice.volume as i32, gfp_mask);
    if ret != 0 { return ret; } (*d).disk_offset = (*v).slice.start; (*d).len = (*v).slice.len; 0
}

unsafe fn bl_parse_concat(server: *mut nfs_server, d: *mut pnfs_block_dev, volumes: *mut pnfs_block_volume, idx: i32, gfp_mask: gfp_t) -> i32 {
    let v = volumes.add(idx as usize); (*d).children = kzalloc_objs::<pnfs_block_dev>((*v).concat.volumes_count, gfp_mask);
    if (*d).children.is_null() { return -ENOMEM; } let mut len = 0; let mut i = 0;
    while i < (*v).concat.volumes_count { let child = (*d).children.add(i as usize); let ret = bl_parse_deviceid(server, child, volumes, (*v).concat.volumes[i as usize] as i32, gfp_mask); if ret != 0 { bl_free_device(child); bl_free_device(d); return ret; } (*d).nr_children += 1; (*child).start += len; len += (*child).len; i += 1; }
    (*d).len = len; (*d).map = Some(bl_map_concat); 0
}

unsafe fn bl_parse_stripe(server: *mut nfs_server, d: *mut pnfs_block_dev, volumes: *mut pnfs_block_volume, idx: i32, gfp_mask: gfp_t) -> i32 {
    let v = volumes.add(idx as usize); (*d).children = kzalloc_objs::<pnfs_block_dev>((*v).stripe.volumes_count, gfp_mask);
    if (*d).children.is_null() { return -ENOMEM; } let mut len = 0; let mut i = 0;
    while i < (*v).stripe.volumes_count { let child = (*d).children.add(i as usize); let ret = bl_parse_deviceid(server, child, volumes, (*v).stripe.volumes[i as usize] as i32, gfp_mask); if ret != 0 { bl_free_device(child); bl_free_device(d); return ret; } (*d).nr_children += 1; len += (*child).len; i += 1; }
    (*d).len = len; (*d).chunk_size = (*v).stripe.chunk_size; (*d).map = Some(bl_map_stripe); 0
}

unsafe fn bl_parse_deviceid(server: *mut nfs_server, d: *mut pnfs_block_dev, volumes: *mut pnfs_block_volume, idx: i32, gfp_mask: gfp_t) -> i32 {
    (*d).type_ = (*volumes.add(idx as usize)).type_;
    match (*d).type_ { PNFS_BLOCK_VOLUME_SIMPLE => bl_parse_simple(server,d,volumes,idx,gfp_mask), PNFS_BLOCK_VOLUME_SLICE => bl_parse_slice(server,d,volumes,idx,gfp_mask), PNFS_BLOCK_VOLUME_CONCAT => bl_parse_concat(server,d,volumes,idx,gfp_mask), PNFS_BLOCK_VOLUME_STRIPE => bl_parse_stripe(server,d,volumes,idx,gfp_mask), PNFS_BLOCK_VOLUME_SCSI => bl_parse_scsi(server,d,volumes,idx,gfp_mask), _ => -EIO }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
