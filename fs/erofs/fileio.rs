// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2024, Alibaba Cloud
 */

// Declarations supplied by the surrounding kernel/EROFS translation.

#[repr(C)]
struct erofs_fileio_rq {
    bvecs: [bio_vec; 16],
    bio: bio,
    iocb: kiocb,
    sb: *mut super_block,
    ref_: refcount_t,
}

#[repr(C)]
struct erofs_fileio {
    map: erofs_map_blocks,
    dev: erofs_map_dev,
    rq: *mut erofs_fileio_rq,
}

unsafe fn erofs_fileio_ki_complete(iocb: *mut kiocb, mut ret: c_long) {
    let rq = container_of!(iocb, erofs_fileio_rq, iocb);
    let mut fi: folio_iter = core::mem::zeroed();

    if ret >= 0 && ret != (*rq).bio.bi_iter.bi_size {
        ret = -EIO;
    }
    if (*rq).bio.bi_end_io.is_none() {
        bio_for_each_folio_all!(fi, &mut (*rq).bio) {
            DBG_BUGON!(folio_test_uptodate(fi.folio));
            erofs_onlinefolio_end(fi.folio, ret < 0, false);
        }
    } else if ret < 0 && (*rq).bio.bi_status == 0 {
        (*rq).bio.bi_status = errno_to_blk_status(ret);
    }
    bio_endio(&mut (*rq).bio);
    bio_uninit(&mut (*rq).bio);
    if refcount_dec_and_test(&mut (*rq).ref_) {
        kfree(rq.cast());
    }
}

unsafe fn erofs_fileio_rq_submit(rq: *mut erofs_fileio_rq) {
    if rq.is_null() {
        return;
    }
    let mut iter: iov_iter = core::mem::zeroed();
    let mut ret: ssize_t;

    (*rq).iocb.ki_pos = (*rq).bio.bi_iter.bi_sector << SECTOR_SHIFT;
    (*rq).iocb.ki_ioprio = get_current_ioprio();
    (*rq).iocb.ki_complete = Some(erofs_fileio_ki_complete);
    if test_opt(&mut EROFS_SB((*rq).sb).opt, DIRECT_IO)
        && (*(*rq).iocb.ki_filp).f_mode & FMODE_CAN_ODIRECT != 0
    {
        (*rq).iocb.ki_flags = IOCB_DIRECT;
    }
    iov_iter_bvec(&mut iter, ITER_DEST, (*rq).bvecs.as_mut_ptr(), (*rq).bio.bi_vcnt,
                  (*rq).bio.bi_iter.bi_size);
    scoped_with_creds!((*rq).iocb.ki_filp.f_cred) {
        ret = vfs_iocb_iter_read((*rq).iocb.ki_filp, &mut (*rq).iocb, &mut iter);
    }
    if ret != -EIOCBQUEUED {
        erofs_fileio_ki_complete(&mut (*rq).iocb, ret);
    }
    if refcount_dec_and_test(&mut (*rq).ref_) {
        kfree(rq.cast());
    }
}

unsafe fn erofs_fileio_rq_alloc(mdev: *mut erofs_map_dev) -> *mut erofs_fileio_rq {
    let rq = kzalloc_obj!(erofs_fileio_rq, GFP_KERNEL | __GFP_NOFAIL);
    bio_init(&mut (*rq).bio, core::ptr::null_mut(), (*rq).bvecs.as_mut_ptr(), 16, REQ_OP_READ);
    (*rq).iocb.ki_filp = (*(*mdev).m_dif).file;
    (*rq).sb = (*mdev).m_sb;
    refcount_set(&mut (*rq).ref_, 2);
    rq
}

pub unsafe fn erofs_fileio_bio_alloc(mdev: *mut erofs_map_dev) -> *mut bio {
    &mut (*erofs_fileio_rq_alloc(mdev)).bio
}

pub unsafe fn erofs_fileio_submit_bio(bio: *mut bio) {
    erofs_fileio_rq_submit(container_of!(bio, erofs_fileio_rq, bio));
}

unsafe fn erofs_fileio_scan_folio(io: *mut erofs_fileio, inode: *mut inode,
                                  folio: *mut folio) -> c_int {
    let map = &mut (*io).map;
    let (mut cur, end) = (0u32, folio_size(folio));
    let (mut attached, mut len): (u32, u32) = (0, 0);
    let (pos, mut ofs): (loff_t, loff_t) = (folio_pos(folio), 0);
    let mut err = 0;

    erofs_onlinefolio_init(folio);
    while cur < end {
        if !in_range(pos + cur as loff_t, map.m_la, map.m_llen) {
            map.m_la = pos + cur as loff_t;
            map.m_llen = (end - cur) as loff_t;
            err = erofs_map_blocks(inode, map);
            if err != 0 { break; }
        }
        ofs = folio_pos(folio) + cur as loff_t - map.m_la;
        len = min_t!(loff_t, map.m_llen - ofs, (end - cur) as loff_t) as u32;
        if map.m_flags & EROFS_MAP_META != 0 {
            let mut buf: erofs_buf = EROFS_BUF_INITIALIZER!();
            let src = erofs_read_metabuf(&mut buf, (*inode).i_sb,
                map.m_pa + ofs, erofs_inode_in_metabox(inode));
            if IS_ERR(src) { err = PTR_ERR(src); break; }
            memcpy_to_folio(folio, cur, src, len);
            erofs_put_metabuf(&mut buf);
        } else if map.m_flags & EROFS_MAP_MAPPED == 0 {
            folio_zero_segment(folio, cur, cur + len);
            attached = 0;
        } else {
            if !(*io).rq.is_null() && (map.m_pa + ofs != (*io).dev.m_pa || map.m_deviceid != (*io).dev.m_deviceid) {
                erofs_fileio_rq_submit((*io).rq); (*io).rq = core::ptr::null_mut();
            }
            if (*io).rq.is_null() {
                (*io).dev.m_pa = map.m_pa + ofs;
                (*io).dev.m_deviceid = map.m_deviceid;
                err = erofs_map_dev((*inode).i_sb, &mut (*io).dev);
                if err != 0 { break; }
                (*io).rq = erofs_fileio_rq_alloc(&mut (*io).dev);
                (*(*io).rq).bio.bi_iter.bi_sector = ((*(*io).dev.m_dif).fsoff + (*io).dev.m_pa) >> 9;
                attached = 0;
            }
            if bio_add_folio(&mut (*(*io).rq).bio, folio, len, cur) == 0 {
                erofs_fileio_rq_submit((*io).rq); (*io).rq = core::ptr::null_mut(); continue;
            }
            if attached == 0 { erofs_onlinefolio_split(folio); }
            attached += 1;
            (*io).dev.m_pa += len as u64;
        }
        cur += len;
    }
    erofs_onlinefolio_end(folio, err != 0, false);
    err
}

unsafe fn erofs_fileio_read_folio(_file: *mut file, folio: *mut folio) -> c_int {
    let mut need_iput = false;
    let realinode = erofs_real_inode(folio_inode(folio), &mut need_iput);
    let mut io: erofs_fileio = core::mem::zeroed();
    trace_erofs_read_folio!(realinode, folio, true);
    let err = erofs_fileio_scan_folio(&mut io, realinode, folio);
    erofs_fileio_rq_submit(io.rq);
    if need_iput { iput(realinode); }
    err
}

unsafe fn erofs_fileio_readahead(rac: *mut readahead_control) {
    let mut need_iput = false;
    let realinode = erofs_real_inode((*(*rac).mapping).host, &mut need_iput);
    let mut io: erofs_fileio = core::mem::zeroed();
    trace_erofs_readahead!(realinode, readahead_index(rac), readahead_count(rac), true);
    while let Some(folio) = readahead_folio(rac) {
        let err = erofs_fileio_scan_folio(&mut io, realinode, folio);
        if err != 0 && err != -EINTR { erofs_err!((*realinode).i_sb, "readahead error at folio {} @ nid {}", (*folio).index, EROFS_I(realinode).nid); }
    }
    erofs_fileio_rq_submit(io.rq);
    if need_iput { iput(realinode); }
}

pub static erofs_fileio_aops: address_space_operations = address_space_operations {
    read_folio: Some(erofs_fileio_read_folio),
    readahead: Some(erofs_fileio_readahead),
};

pub unsafe fn erofs_read_meta_folio(_file: *mut file, folio: *mut folio) -> c_int {
    let mut io: erofs_fileio = erofs_fileio { map: core::mem::zeroed(), dev: core::mem::zeroed(), rq: core::ptr::null_mut() };
    io.dev.m_pa = folio_pos(folio);
    let inode = folio_inode(folio);
    let err = erofs_map_dev((*inode).i_sb, &mut io.dev);
    if err != 0 { return err; }
    io.rq = erofs_fileio_rq_alloc(&mut io.dev);
    (*io.rq).bio.bi_iter.bi_sector = ((*(*io.dev.m_dif).fsoff + io.dev.m_pa) >> 9);
    erofs_onlinefolio_init(folio);
    bio_add_folio_nofail(&mut (*io.rq).bio, folio, folio_size(folio), 0);
    erofs_fileio_rq_submit(io.rq);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
