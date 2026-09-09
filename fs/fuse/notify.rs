// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/FUSE translation.

unsafe fn fuse_notify_poll(fc: *mut fuse_conn, size: u32, cs: *mut fuse_copy_state) -> i32 {
    let mut outarg: fuse_notify_poll_wakeup_out = core::mem::zeroed();
    if size as usize != core::mem::size_of::<fuse_notify_poll_wakeup_out>() { return -EINVAL; }
    let err = fuse_copy_one(cs, &mut outarg as *mut _ as *mut _, core::mem::size_of::<fuse_notify_poll_wakeup_out>());
    if err != 0 { return err; }
    fuse_copy_finish(cs);
    fuse_notify_poll_wakeup(fc, &mut outarg)
}

unsafe fn fuse_notify_inval_inode(fc: *mut fuse_conn, size: u32, cs: *mut fuse_copy_state) -> i32 {
    let mut outarg: fuse_notify_inval_inode_out = core::mem::zeroed();
    if size as usize != core::mem::size_of::<fuse_notify_inval_inode_out>() { return -EINVAL; }
    let mut err = fuse_copy_one(cs, &mut outarg as *mut _ as *mut _, core::mem::size_of::<fuse_notify_inval_inode_out>());
    if err != 0 { return err; }
    fuse_copy_finish(cs);
    down_read(&mut (*fc).killsb);
    err = fuse_reverse_inval_inode(fc, outarg.ino, outarg.off, outarg.len);
    up_read(&mut (*fc).killsb);
    err
}

unsafe fn fuse_notify_inval_entry(fc: *mut fuse_conn, size: u32, cs: *mut fuse_copy_state) -> i32 {
    let mut outarg: fuse_notify_inval_entry_out = core::mem::zeroed();
    if size as usize < core::mem::size_of::<fuse_notify_inval_entry_out>() { return -EINVAL; }
    let mut err = fuse_copy_one(cs, &mut outarg as *mut _ as *mut _, core::mem::size_of::<fuse_notify_inval_entry_out>());
    if err != 0 { return err; }
    if outarg.namelen > (*fc).name_max { return -ENAMETOOLONG; }
    if size as usize != core::mem::size_of::<fuse_notify_inval_entry_out>() + outarg.namelen as usize + 1 { return -EINVAL; }
    let buf = kzalloc(outarg.namelen as usize + 1, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    let mut name: qstr = core::mem::zeroed(); name.name = buf; name.len = outarg.namelen;
    err = fuse_copy_one(cs, buf as *mut _, outarg.namelen as usize + 1);
    if err == 0 { fuse_copy_finish(cs); *buf.add(outarg.namelen as usize) = 0; down_read(&mut (*fc).killsb); err = fuse_reverse_inval_entry(fc, outarg.parent, 0, &mut name, outarg.flags); up_read(&mut (*fc).killsb); }
    kfree(buf as *mut _); err
}

unsafe fn fuse_notify_delete(fc: *mut fuse_conn, size: u32, cs: *mut fuse_copy_state) -> i32 {
    let mut outarg: fuse_notify_delete_out = core::mem::zeroed();
    if size as usize < core::mem::size_of::<fuse_notify_delete_out>() { return -EINVAL; }
    let mut err = fuse_copy_one(cs, &mut outarg as *mut _ as *mut _, core::mem::size_of::<fuse_notify_delete_out>());
    if err != 0 { return err; }
    if outarg.namelen > (*fc).name_max { return -ENAMETOOLONG; }
    if size as usize != core::mem::size_of::<fuse_notify_delete_out>() + outarg.namelen as usize + 1 { return -EINVAL; }
    let buf = kzalloc(outarg.namelen as usize + 1, GFP_KERNEL); if buf.is_null() { return -ENOMEM; }
    let mut name: qstr = core::mem::zeroed(); name.name = buf; name.len = outarg.namelen;
    err = fuse_copy_one(cs, buf as *mut _, outarg.namelen as usize + 1);
    if err == 0 { fuse_copy_finish(cs); *buf.add(outarg.namelen as usize) = 0; down_read(&mut (*fc).killsb); err = fuse_reverse_inval_entry(fc, outarg.parent, outarg.child, &mut name, 0); up_read(&mut (*fc).killsb); }
    kfree(buf as *mut _); err
}

#[repr(C)]
struct fuse_retrieve_args { ap: fuse_args_pages, inarg: fuse_notify_retrieve_in }

unsafe fn fuse_retrieve_end(args: *mut fuse_args, _error: i32) { let ra = container_of!(args, fuse_retrieve_args, ap.args); release_pages((*ra).ap.folios, (*ra).ap.num_folios); kfree(ra as *mut _); }

unsafe fn fuse_notify_store(fc: *mut fuse_conn, size: u32, cs: *mut fuse_copy_state) -> i32 {
    let mut outarg: fuse_notify_store_out = core::mem::zeroed();
    if size as usize < core::mem::size_of::<fuse_notify_store_out>() { return -EINVAL; }
    let mut err = fuse_copy_one(cs, &mut outarg as *mut _ as *mut _, core::mem::size_of::<fuse_notify_store_out>()); if err != 0 { return err; }
    if size as usize - core::mem::size_of::<fuse_notify_store_out>() != outarg.size as usize || outarg.offset >= MAX_LFS_FILESIZE { return -EINVAL; }
    let nodeid = outarg.nodeid; let mut pos = outarg.offset; let mut num = core::cmp::min(outarg.size, MAX_LFS_FILESIZE - pos);
    down_read(&mut (*fc).killsb); let inode = fuse_ilookup(fc, nodeid, core::ptr::null_mut()); if inode.is_null() { up_read(&mut (*fc).killsb); return -ENOENT; }
    if !S_ISREG((*inode).i_mode) { iput(inode); up_read(&mut (*fc).killsb); return -EINVAL; }
    let file_size = i_size_read(inode); let end = pos + num as i64; if end > file_size { fuse_write_update_attr(inode, end, num); }
    while num != 0 { let index = pos >> PAGE_SHIFT; let folio = filemap_grab_folio((*inode).i_mapping, index); if IS_ERR(folio) { err = PTR_ERR(folio); break; } let off = offset_in_folio(folio, pos); let n = core::cmp::min(num, folio_size(folio) - off); err = fuse_copy_folio(cs, &folio, off, n, 0); if !folio_test_uptodate(folio) && err == 0 && off == 0 && (n == folio_size(folio) || file_size == end) { folio_zero_segment(folio, n, folio_size(folio)); iomap_folio_mark_uptodate(folio); } folio_unlock(folio); folio_put(folio); if err != 0 { break; } pos += n as u64; num -= n; }
    iput(inode); up_read(&mut (*fc).killsb); err
}

unsafe fn fuse_retrieve(fm: *mut fuse_mount, inode: *mut inode, outarg: *mut fuse_notify_retrieve_out) -> i32 {
    let mut ra: *mut fuse_retrieve_args = kzalloc(core::mem::size_of::<fuse_retrieve_args>(), GFP_KERNEL) as *mut _; if ra.is_null() { return -ENOMEM; }
    (*ra).inarg.offset = (*outarg).offset; (*ra).inarg.size = core::cmp::min((*outarg).size, (*(*fm).fc).max_write) as _;
    (*ra).ap.args.nodeid = (*outarg).nodeid; (*ra).ap.args.opcode = FUSE_NOTIFY_REPLY; (*ra).ap.args.in_numargs = 3; (*ra).ap.args.in_pages = true; (*ra).ap.args.end = fuse_retrieve_end;
    fuse_set_zero_arg0(&mut (*ra).ap.args); (*ra).ap.args.in_args[1].size = core::mem::size_of::<fuse_notify_retrieve_in>(); (*ra).ap.args.in_args[1].value = &mut (*ra).inarg as *mut _ as *mut _; (*ra).ap.args.in_args[2].size = (*ra).inarg.size as usize;
    let err = fuse_simple_notify_reply(fm, &mut (*ra).ap.args, (*outarg).notify_unique); if err != 0 { fuse_retrieve_end(&mut (*ra).ap.args, err); } err
}

unsafe fn fuse_notify_retrieve(fc: *mut fuse_conn, size: u32, cs: *mut fuse_copy_state) -> i32 {
    let mut outarg: fuse_notify_retrieve_out = core::mem::zeroed(); if size as usize != core::mem::size_of::<fuse_notify_retrieve_out>() { return -EINVAL; }
    let err = fuse_copy_one(cs, &mut outarg as *mut _ as *mut _, core::mem::size_of::<fuse_notify_retrieve_out>()); if err != 0 { return err; } fuse_copy_finish(cs); if outarg.offset >= MAX_LFS_FILESIZE { return -EINVAL; }
    down_read(&mut (*fc).killsb); let mut fm: *mut fuse_mount = core::ptr::null_mut(); let inode = fuse_ilookup(fc, outarg.nodeid, &mut fm); let result = if inode.is_null() { -ENOENT } else { let r = if S_ISREG((*inode).i_mode) { fuse_retrieve(fm, inode, &mut outarg) } else { -EINVAL }; iput(inode); r }; up_read(&mut (*fc).killsb); result
}

unsafe fn fuse_notify_resend(fc: *mut fuse_conn) -> i32 { fuse_chan_resend((*fc).chan); 0 }

/* Increments the fuse connection epoch and schedules cache invalidation. */
unsafe fn fuse_notify_inc_epoch(fc: *mut fuse_conn) -> i32 { atomic_inc(&mut (*fc).epoch); if !inval_wq.is_null() { schedule_work(&mut (*fc).epoch_work); } 0 }

unsafe fn fuse_notify_prune(fc: *mut fuse_conn, size: u32, cs: *mut fuse_copy_state) -> i32 {
    let batch: u32 = 512; let nodeids = kmalloc((core::mem::size_of::<u64>() as u32 * batch) as usize, GFP_KERNEL) as *mut u64;
    if nodeids.is_null() { return -ENOMEM; }
    let mut outarg: fuse_notify_prune_out = core::mem::zeroed(); if size as usize < core::mem::size_of::<fuse_notify_prune_out>() { return -EINVAL; }
    let mut err = fuse_copy_one(cs, &mut outarg as *mut _ as *mut _, core::mem::size_of::<fuse_notify_prune_out>()); if err != 0 { return err; }
    if size as usize - core::mem::size_of::<fuse_notify_prune_out>() != (outarg.count as usize * core::mem::size_of::<u64>()) { return -EINVAL; }
    while outarg.count != 0 { let num = core::cmp::min(batch, outarg.count); err = fuse_copy_one(cs, nodeids as *mut _, (num as usize) * core::mem::size_of::<u64>()); if err != 0 { return err; } down_read(&mut (*fc).killsb); for i in 0..num { fuse_try_prune_one_inode(fc, *nodeids.add(i as usize)); } up_read(&mut (*fc).killsb); outarg.count -= num; }
    kfree(nodeids as *mut _); 0
}

pub unsafe fn fuse_notify(fc: *mut fuse_conn, code: fuse_notify_code, size: u32, cs: *mut fuse_copy_state) -> i32 {
    match code {
        FUSE_NOTIFY_POLL => fuse_notify_poll(fc, size, cs),
        FUSE_NOTIFY_INVAL_INODE => fuse_notify_inval_inode(fc, size, cs),
        FUSE_NOTIFY_INVAL_ENTRY => fuse_notify_inval_entry(fc, size, cs),
        FUSE_NOTIFY_STORE => fuse_notify_store(fc, size, cs),
        FUSE_NOTIFY_RETRIEVE => fuse_notify_retrieve(fc, size, cs),
        FUSE_NOTIFY_DELETE => fuse_notify_delete(fc, size, cs),
        FUSE_NOTIFY_RESEND => fuse_notify_resend(fc),
        FUSE_NOTIFY_INC_EPOCH => fuse_notify_inc_epoch(fc),
        FUSE_NOTIFY_PRUNE => fuse_notify_prune(fc, size, cs),
        _ => -EINVAL,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
