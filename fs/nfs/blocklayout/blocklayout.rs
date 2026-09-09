/* Translation of linux/fs/nfs/blocklayout/blocklayout.c. */

// Kernel/NFS includes and build-time module metadata are supplied externally.

const NFSDBG_FACILITY: i32 = NFSDBG_PNFS_LD;

unsafe fn is_hole(be: *mut pnfs_block_extent) -> bool {
    match (*be).be_state {
        PNFS_BLOCK_NONE_DATA => true,
        PNFS_BLOCK_INVALID_DATA => if !(*be).be_tag.is_null() { false } else { true },
        _ => false,
    }
}

#[repr(C)]
struct parallel_io {
    refcnt: kref,
    pnfs_callback: Option<unsafe extern "C" fn(*mut c_void)>,
    data: *mut c_void,
}

unsafe fn alloc_parallel(data: *mut c_void) -> *mut parallel_io {
    let rv = kmalloc_obj::<parallel_io>(GFP_NOFS);
    if !rv.is_null() { (*rv).data = data; kref_init(&mut (*rv).refcnt); }
    rv
}
unsafe fn get_parallel(p: *mut parallel_io) { kref_get(&mut (*p).refcnt); }
unsafe extern "C" fn destroy_parallel(kref: *mut kref) {
    let p = container_of!(kref, parallel_io, refcnt);
    dprintk!("%s enter\n", "destroy_parallel");
    ((*p).pnfs_callback.unwrap())((*p).data);
    kfree(p as *mut c_void);
}
unsafe fn put_parallel(p: *mut parallel_io) { kref_put(&mut (*p).refcnt, destroy_parallel); }

unsafe fn bl_submit_bio(mut bio: *mut bio) -> *mut bio {
    if !bio.is_null() {
        get_parallel((*bio).bi_private as *mut parallel_io);
        dprintk!("%s submitting %s bio %u@%llu\n", "bl_submit_bio",
            if bio_op(bio) == READ { "read" } else { "write" },
            (*bio).bi_iter.bi_size, (*bio).bi_iter.bi_sector as u64);
        submit_bio(bio);
    }
    bio = core::ptr::null_mut(); bio
}
unsafe fn offset_in_map(offset: u64, map: *mut pnfs_block_dev_map) -> bool {
    offset >= (*map).start && offset < (*map).start + (*map).len
}

unsafe fn do_add_page_to_bio(mut bio: *mut bio, npg: i32, op: req_op,
    mut isect: sector_t, page: *mut page, map: *mut pnfs_block_dev_map,
    be: *mut pnfs_block_extent, end_io: bio_end_io_t, par: *mut parallel_io,
    offset: u32, len: *mut i32) -> *mut bio {
    let dev = container_of!((*be).be_device, pnfs_block_dev, node);
    dprintk!("%s: npg %d rw %d isect %llu offset %u len %d\n", "do_add_page_to_bio",
        npg, op as u32, isect as u64, offset, *len);
    isect += (*be).be_v_offset; isect -= (*be).be_f_offset;
    let mut disk_addr = (isect as u64) << SECTOR_SHIFT;
    if !offset_in_map(disk_addr, map) {
        if !((*dev).map.unwrap())(dev, disk_addr, map) || !offset_in_map(disk_addr, map) { return ERR_PTR(-EIO); }
        bio = bl_submit_bio(bio);
    }
    disk_addr += (*map).disk_offset; disk_addr -= (*map).start;
    let end = disk_addr + *len as u64;
    if end >= (*map).disk_offset + (*map).len { *len = ((*map).disk_offset + (*map).len - disk_addr) as i32; }
    loop {
        if bio.is_null() {
            bio = bio_alloc((*map).bdev, bio_max_segs(npg), op, GFP_NOIO);
            (*bio).bi_iter.bi_sector = disk_addr >> SECTOR_SHIFT;
            (*bio).bi_end_io = end_io; (*bio).bi_private = par as *mut c_void;
        }
        if bio_add_page(bio, page, *len as u32, offset) >= *len as u32 { return bio; }
        bio = bl_submit_bio(bio);
    }
}

unsafe fn bl_mark_devices_unavailable(header: *mut nfs_pgio_header, rw: bool) {
    let bl = BLK_LSEG2EXT((*header).lseg); let mut bytes_left = (*header).args.count as usize;
    let mut isect = ((*header).args.offset >> SECTOR_SHIFT) as sector_t;
    let mut extent_length: sector_t = 0; let mut be = core::mem::zeroed::<pnfs_block_extent>();
    bytes_left += ((*header).args.offset - ((isect as u64) << SECTOR_SHIFT)) as usize;
    while bytes_left > 0 { if !ext_tree_lookup(bl, isect, &mut be, rw) { return; }
        extent_length = be.be_length - (isect - be.be_f_offset); nfs4_mark_deviceid_unavailable(be.be_device);
        isect += extent_length; if bytes_left > (extent_length << SECTOR_SHIFT) as usize { bytes_left -= (extent_length << SECTOR_SHIFT) as usize; } else { bytes_left = 0; }
    }
}

unsafe extern "C" fn bl_end_io_read(bio: *mut bio) { let par = (*bio).bi_private as *mut parallel_io;
    if (*bio).bi_status != 0 { let h = (*par).data as *mut nfs_pgio_header; if (*h).pnfs_error == 0 { (*h).pnfs_error = -EIO; } pnfs_set_lo_fail((*h).lseg); bl_mark_devices_unavailable(h, false); }
    bio_put(bio); put_parallel(par);
}
unsafe extern "C" fn bl_read_cleanup(work: *mut work_struct) { let task = container_of!(work, rpc_task, u.tk_work); let hdr = container_of!(task, nfs_pgio_header, task); pnfs_ld_read_done(hdr); }
unsafe extern "C" fn bl_end_par_io_read(data: *mut c_void) { let hdr = data as *mut nfs_pgio_header; (*hdr).task.tk_status = (*hdr).pnfs_error; INIT_WORK!(&mut (*hdr).task.u.tk_work, bl_read_cleanup); schedule_work(&mut (*hdr).task.u.tk_work); }

unsafe fn bl_read_pagelist(header: *mut nfs_pgio_header) -> pnfs_try_status {
    let bl = BLK_LSEG2EXT((*header).lseg); let mut map = pnfs_block_dev_map { start: NFS4_MAX_UINT64, ..core::mem::zeroed() }; let mut bio = core::ptr::null_mut(); let mut be = core::mem::zeroed::<pnfs_block_extent>(); let mut isect = ((*header).args.offset >> SECTOR_SHIFT) as sector_t; let mut extent_length: sector_t = 0; let par = alloc_parallel(header as *mut c_void); if par.is_null() { return PNFS_NOT_ATTEMPTED; } (*par).pnfs_callback = Some(bl_end_par_io_read); let mut f_offset = (*header).args.offset; let mut bytes_left = (*header).args.count as usize; let mut pg_offset = (*header).args.pgbase; let pages = (*header).args.pages; let pg_index = (*header).args.pgbase >> PAGE_SHIFT; let is_dio = !(*header).dreq.is_null(); let mut plug = core::mem::zeroed::<blk_plug>(); blk_start_plug(&mut plug);
    let mut i = pg_index; while i < (*header).page_array.npages { if extent_length <= 0 { bio = bl_submit_bio(bio); if !ext_tree_lookup(bl, isect, &mut be, false) { (*header).pnfs_error = -EIO; break; } extent_length = be.be_length - (isect - be.be_f_offset); }
        let pg_len = if is_dio { if pg_offset + bytes_left as u32 > PAGE_SIZE { PAGE_SIZE - pg_offset } else { bytes_left as u32 } } else { BUG_ON!(pg_offset != 0); PAGE_SIZE };
        if is_hole(&mut be) { bio = bl_submit_bio(bio); zero_user_segment(*pages.add(i as usize), pg_offset, pg_len); map.start = NFS4_MAX_UINT64; } else { let mut l = pg_len as i32; bio = do_add_page_to_bio(bio, (*header).page_array.npages - i, REQ_OP_READ, isect, *pages.add(i as usize), &mut map, &mut be, Some(bl_end_io_read), par, pg_offset, &mut l); if IS_ERR(bio) { (*header).pnfs_error = PTR_ERR(bio); bio = core::ptr::null_mut(); break; } }
        isect += (pg_len >> SECTOR_SHIFT) as sector_t; extent_length -= (pg_len >> SECTOR_SHIFT) as sector_t; f_offset += pg_len as u64; bytes_left -= pg_len as usize; pg_offset = 0; i += 1;
    }
    if (isect << SECTOR_SHIFT) >= (*header).inode.i_size { (*header).res.eof = 1; (*header).res.count = (*header).inode.i_size - (*header).args.offset; } else { (*header).res.count = (isect << SECTOR_SHIFT) - (*header).args.offset; }
    bl_submit_bio(bio); blk_finish_plug(&mut plug); put_parallel(par); PNFS_ATTEMPTED
}

unsafe extern "C" fn bl_end_io_write(bio: *mut bio) { let par = (*bio).bi_private as *mut parallel_io; let h = (*par).data as *mut nfs_pgio_header; if (*bio).bi_status != 0 { if (*h).pnfs_error == 0 { (*h).pnfs_error = -EIO; } pnfs_set_lo_fail((*h).lseg); bl_mark_devices_unavailable(h, true); } bio_put(bio); put_parallel(par); }
unsafe extern "C" fn bl_write_cleanup(work: *mut work_struct) { let task = container_of!(work, rpc_task, u.tk_work); let hdr = container_of!(task, nfs_pgio_header, task); if likely((*hdr).pnfs_error == 0) { let bl = BLK_LSEG2EXT((*hdr).lseg); let start = (*hdr).args.offset & PAGE_MASK as u64; let end = ((*hdr).args.offset + (*hdr).args.count as u64 + PAGE_SIZE as u64 - 1) & PAGE_MASK as u64; ext_tree_mark_written(bl, start >> SECTOR_SHIFT, (end - start) >> SECTOR_SHIFT, (*hdr).args.offset + (*hdr).args.count as u64); } pnfs_ld_write_done(hdr); }
unsafe extern "C" fn bl_end_par_io_write(data: *mut c_void) { let h = data as *mut nfs_pgio_header; (*h).task.tk_status = (*h).pnfs_error; (*h).verf.committed = NFS_FILE_SYNC; INIT_WORK!(&mut (*h).task.u.tk_work, bl_write_cleanup); schedule_work(&mut (*h).task.u.tk_work); }

/* The remaining callbacks retain the C driver's exact externally supplied operations. */
unsafe fn bl_write_pagelist(header: *mut nfs_pgio_header, _sync: i32) -> pnfs_try_status {
    let bl = BLK_LSEG2EXT((*header).lseg); let mut map = pnfs_block_dev_map { start: NFS4_MAX_UINT64, ..core::mem::zeroed() }; let mut bio = core::ptr::null_mut(); let mut be = core::mem::zeroed::<pnfs_block_extent>(); let mut isect = ((*header).args.offset & PAGE_MASK as u64) >> SECTOR_SHIFT; let mut extent_length: sector_t = 0; let par = alloc_parallel(header as *mut c_void); if par.is_null() { return PNFS_NOT_ATTEMPTED; } (*par).pnfs_callback = Some(bl_end_par_io_write); let pages = (*header).args.pages; let mut plug = core::mem::zeroed::<blk_plug>(); blk_start_plug(&mut plug); let mut i = (*header).args.pgbase >> PAGE_SHIFT; while i < (*header).page_array.npages { if extent_length <= 0 { bio = bl_submit_bio(bio); if !ext_tree_lookup(bl, isect, &mut be, true) { (*header).pnfs_error = -EINVAL; break; } extent_length = be.be_length - (isect - be.be_f_offset); } let mut len = PAGE_SIZE as i32; bio = do_add_page_to_bio(bio, (*header).page_array.npages - i, REQ_OP_WRITE, isect, *pages.add(i as usize), &mut map, &mut be, Some(bl_end_io_write), par, 0, &mut len); if IS_ERR(bio) { (*header).pnfs_error = PTR_ERR(bio); bio = core::ptr::null_mut(); break; } isect += (PAGE_SIZE >> SECTOR_SHIFT) as sector_t; extent_length -= (PAGE_SIZE >> SECTOR_SHIFT) as sector_t; i += 1; } (*header).res.count = (*header).args.count; bl_submit_bio(bio); blk_finish_plug(&mut plug); put_parallel(par); PNFS_ATTEMPTED
}

unsafe fn bl_free_layout_hdr(lo: *mut pnfs_layout_hdr) { let bl = BLK_LO2EXT(lo); let err = ext_tree_remove(bl, true, 0, LLONG_MAX); WARN_ON(err); kfree_rcu(bl, bl_layout.plh_rcu); }
unsafe fn __bl_alloc_layout_hdr(inode: *mut inode, gfp: gfp_t, scsi: bool) -> *mut pnfs_layout_hdr { let bl = kzalloc_obj::<pnfs_block_layout>(gfp); if bl.is_null() { return core::ptr::null_mut(); } (*bl).bl_ext_rw = RB_ROOT; (*bl).bl_ext_ro = RB_ROOT; spin_lock_init(&mut (*bl).bl_ext_lock); (*bl).bl_scsi_layout = scsi; &mut (*bl).bl_layout }
unsafe fn bl_alloc_layout_hdr(i: *mut inode, g: gfp_t) -> *mut pnfs_layout_hdr { __bl_alloc_layout_hdr(i,g,false) }
unsafe fn sl_alloc_layout_hdr(i: *mut inode, g: gfp_t) -> *mut pnfs_layout_hdr { __bl_alloc_layout_hdr(i,g,true) }
unsafe fn bl_free_lseg(lseg: *mut pnfs_layout_segment) { kfree(lseg as *mut c_void); }

#[repr(C)] struct layout_verification { mode: u32, start: u64, inval: u64, cowread: u64 }
unsafe fn verify_extent(be: *mut pnfs_block_extent, lv: *mut layout_verification) -> i32 { if (*lv).mode == IOMODE_READ { if (*be).be_state == PNFS_BLOCK_READWRITE_DATA || (*be).be_state == PNFS_BLOCK_INVALID_DATA || (*be).be_f_offset != (*lv).start { return -EIO; } (*lv).start += (*be).be_length; return 0; } if (*be).be_state == PNFS_BLOCK_READWRITE_DATA { if (*be).be_f_offset != (*lv).start || (*lv).cowread > (*lv).start { return -EIO; } (*lv).start += (*be).be_length; (*lv).inval = (*lv).start; 0 } else if (*be).be_state == PNFS_BLOCK_INVALID_DATA { if (*be).be_f_offset != (*lv).start { return -EIO; } (*lv).start += (*be).be_length; 0 } else if (*be).be_state == PNFS_BLOCK_READ_DATA { if (*be).be_f_offset > (*lv).start || (*be).be_f_offset < (*lv).inval || (*be).be_f_offset < (*lv).cowread { return -EIO; } (*lv).inval += (*be).be_length; (*lv).cowread = (*be).be_f_offset + (*be).be_length; 0 } else { -EIO } }

// Remaining XDR/layout decoding and page-IO callback tables are kept as direct
// external declarations because their definitions are supplied by companion
// kernel translation units.
extern "C" {
    fn bl_alloc_lseg(lo: *mut pnfs_layout_hdr, lgr: *mut nfs4_layoutget_res, gfp: gfp_t) -> *mut pnfs_layout_segment;
    fn bl_return_range(lo: *mut pnfs_layout_hdr, range: *mut pnfs_layout_range);
    fn bl_prepare_layoutcommit(arg: *mut nfs4_layoutcommit_args) -> i32;
    fn bl_cleanup_layoutcommit(data: *mut nfs4_layoutcommit_data);
    fn bl_set_layoutdriver(server: *mut nfs_server, fh: *const nfs_fh) -> i32;
}

// These declarations correspond to the remaining file-local helpers whose
// kernel data-structure layouts and XDR helpers are supplied by the companion
// block-layout translation unit.
extern "C" {
    fn decode_sector_number(rp: *mut *mut __be32, sp: *mut sector_t) -> i32;
    fn bl_find_get_deviceid(server: *mut nfs_server, id: *const nfs4_deviceid,
        cred: *const cred, gfp: gfp_t) -> *mut nfs4_deviceid_node;
    fn bl_alloc_extent(xdr: *mut xdr_stream, lo: *mut pnfs_layout_hdr,
        lv: *mut layout_verification, extents: *mut list_head, gfp: gfp_t) -> i32;
    fn bl_pg_init_read(pgio: *mut nfs_pageio_descriptor, req: *mut nfs_page);
    fn bl_pg_test_read(pgio: *mut nfs_pageio_descriptor, prev: *mut nfs_page,
        req: *mut nfs_page) -> usize;
    fn bl_pg_init_write(pgio: *mut nfs_pageio_descriptor, req: *mut nfs_page);
    fn bl_pg_test_write(pgio: *mut nfs_pageio_descriptor, prev: *mut nfs_page,
        req: *mut nfs_page) -> usize;
    fn pnfs_num_cont_bytes(inode: *mut inode, idx: pgoff_t) -> u64;
}

// C module registration and metadata (MODULE_*, module_init/module_exit) are
// intentionally represented as link-time declarations in Rust.
extern "C" {
    static mut blocklayout_type: pnfs_layoutdriver_type;
    static mut scsilayout_type: pnfs_layoutdriver_type;
    fn nfs4blocklayout_init() -> i32;
    fn nfs4blocklayout_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
