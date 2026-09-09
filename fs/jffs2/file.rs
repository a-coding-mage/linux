/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// Linux kernel headers and "nodelist.h" provide the external types, constants,
// macros, functions, and structure layouts referenced below.

unsafe extern "C" {
    fn jffs2_flush_wbuf_gc(c: *mut jffs2_sb_info, ino: u64);
    fn file_write_and_wait_range(file: *mut file, start: i64, end: i64) -> i32;
    fn inode_lock(inode: *mut inode);
    fn inode_unlock(inode: *mut inode);
    fn jffs2_ioctl() -> usize;
    fn jffs2_get_acl() -> usize;
    fn jffs2_set_acl() -> usize;
    fn jffs2_setattr() -> usize;
    fn jffs2_listxattr() -> usize;
    fn jffs2_read_inode_range(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
                              kaddr: *mut u8, offset: usize, size: usize) -> i32;
    fn jffs2_reserve_space(c: *mut jffs2_sb_info, len: usize, alloc_len: *mut u32,
                           alloc_type: u32, summary_size: usize) -> i32;
    fn jffs2_write_dnode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
                         ri: *mut jffs2_raw_inode, data: *mut core::ffi::c_void,
                         len: u32, alloc_type: u32) -> *mut jffs2_full_dnode;
    fn jffs2_add_full_dnode_to_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
                                     fn_: *mut jffs2_full_dnode) -> i32;
    fn jffs2_mark_node_obsolete(c: *mut jffs2_sb_info, raw: *mut core::ffi::c_void);
    fn jffs2_free_full_dnode(node: *mut jffs2_full_dnode);
    fn jffs2_complete_reservation(c: *mut jffs2_sb_info);
    fn jffs2_alloc_raw_inode() -> *mut jffs2_raw_inode;
    fn jffs2_write_inode_range(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
                               ri: *mut jffs2_raw_inode, buf: *mut u8,
                               pos: u64, len: u32, writtenlen: *mut u32) -> i32;
    fn jffs2_free_raw_inode(ri: *mut jffs2_raw_inode);
}

extern "C" {
    static jffs2_file_operations: file_operations;
    static jffs2_file_inode_operations: inode_operations;
    static jffs2_file_address_operations: address_space_operations;
}

unsafe fn jffs2_do_readpage_nolock(inode: *mut inode, folio: *mut folio) -> i32 {
    let f = JFFS2_INODE_INFO(inode);
    let c = JFFS2_SB_INFO((*(*inode).i_sb));
    let kaddr = kmap_local_folio(folio, 0);
    let ret = jffs2_read_inode_range(c, f, kaddr, (*folio).index << PAGE_SHIFT, PAGE_SIZE);
    kunmap_local(kaddr);
    if ret == 0 { folio_mark_uptodate(folio); }
    flush_dcache_folio(folio);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn jffs2_fsync(filp: *mut file, start: i64, end: i64, _datasync: i32) -> i32 {
    let inode = (*(*filp).f_mapping).host;
    let c = JFFS2_SB_INFO((*inode).i_sb);
    let ret = file_write_and_wait_range(filp, start, end);
    if ret != 0 { return ret; }
    inode_lock(inode);
    jffs2_flush_wbuf_gc(c, (*inode).i_ino);
    inode_unlock(inode);
    0
}

pub unsafe extern "C" fn __jffs2_read_folio(_file: *mut file, folio: *mut folio) -> i32 {
    let ret = jffs2_do_readpage_nolock((*(*folio).mapping).host, folio);
    folio_unlock(folio);
    ret
}

unsafe fn jffs2_read_folio(file: *mut file, folio: *mut folio) -> i32 {
    let f = JFFS2_INODE_INFO((*(*folio).mapping).host);
    mutex_lock(&mut (*f).sem);
    let ret = __jffs2_read_folio(file, folio);
    mutex_unlock(&mut (*f).sem);
    ret
}

unsafe fn jffs2_write_begin(_iocb: *const kiocb, mapping: *mut address_space,
                            pos: i64, _len: u32, foliop: *mut *mut folio,
                            _fsdata: *mut *mut core::ffi::c_void) -> i32 {
    let inode = (*mapping).host;
    let f = JFFS2_INODE_INFO(inode);
    let c = JFFS2_SB_INFO((*inode).i_sb);
    let index = (pos as u64) >> PAGE_SHIFT;
    let mut ret = 0;
    if pos > (*inode).i_size {
        let mut ri: jffs2_raw_inode = core::mem::zeroed();
        let mut alloc_len = 0u32;
        ret = jffs2_reserve_space(c, core::mem::size_of::<jffs2_raw_inode>(), &mut alloc_len,
                                  ALLOC_NORMAL, JFFS2_SUMMARY_INODE_SIZE);
        if ret != 0 { return ret; }
        mutex_lock(&mut (*f).sem);
        ri.magic = cpu_to_je16(JFFS2_MAGIC_BITMASK);
        ri.nodetype = cpu_to_je16(JFFS2_NODETYPE_INODE);
        ri.totlen = cpu_to_je32(core::mem::size_of::<jffs2_raw_inode>() as u32);
        ri.hdr_crc = cpu_to_je32(crc32(0, &ri as *const _ as *const u8,
                                       core::mem::size_of::<jffs2_unknown_node>() - 4));
        ri.ino = cpu_to_je32((*(*f).inocache).ino);
        (*f).highest_version = (*f).highest_version.wrapping_add(1);
        ri.version = cpu_to_je32((*f).highest_version);
        ri.mode = cpu_to_jemode((*inode).i_mode);
        ri.uid = cpu_to_je16(i_uid_read(inode));
        ri.gid = cpu_to_je16(i_gid_read(inode));
        ri.isize = cpu_to_je32(pos as u32);
        ri.atime = cpu_to_je32(JFFS2_NOW()); ri.ctime = ri.atime; ri.mtime = ri.atime;
        ri.offset = cpu_to_je32((*inode).i_size as u32);
        ri.dsize = cpu_to_je32((pos as u32).wrapping_sub((*inode).i_size as u32));
        ri.csize = cpu_to_je32(0); ri.compr = JFFS2_COMPR_ZERO;
        ri.node_crc = cpu_to_je32(crc32(0, &ri as *const _ as *const u8,
                                        core::mem::size_of::<jffs2_raw_inode>() - 8));
        ri.data_crc = cpu_to_je32(0);
        let node = jffs2_write_dnode(c, f, &mut ri, core::ptr::null_mut(), 0, ALLOC_NORMAL);
        if IS_ERR(node) { ret = PTR_ERR(node); jffs2_complete_reservation(c); mutex_unlock(&mut (*f).sem); return ret; }
        ret = jffs2_add_full_dnode_to_inode(c, f, node);
        if ret != 0 { jffs2_mark_node_obsolete(c, (*node).raw); jffs2_free_full_dnode(node); jffs2_complete_reservation(c); mutex_unlock(&mut (*f).sem); return ret; }
        jffs2_complete_reservation(c); (*inode).i_size = pos; mutex_unlock(&mut (*f).sem);
    }
    mutex_lock(&mut (*c).alloc_sem);
    let folio = __filemap_get_folio(mapping, index, FGP_WRITEBEGIN, mapping_gfp_mask(mapping));
    if IS_ERR(folio) { ret = PTR_ERR(folio); mutex_unlock(&mut (*c).alloc_sem); return ret; }
    *foliop = folio;
    if !folio_test_uptodate(folio) {
        mutex_lock(&mut (*f).sem); ret = jffs2_do_readpage_nolock(inode, folio); mutex_unlock(&mut (*f).sem);
        if ret != 0 { folio_unlock(folio); folio_put(folio); mutex_unlock(&mut (*c).alloc_sem); return ret; }
    }
    mutex_unlock(&mut (*c).alloc_sem); ret
}

unsafe fn jffs2_write_end(_iocb: *const kiocb, mapping: *mut address_space, pos: i64,
                          _len: u32, copied: u32, folio: *mut folio,
                          _fsdata: *mut core::ffi::c_void) -> i32 {
    let inode = (*mapping).host; let f = JFFS2_INODE_INFO(inode); let c = JFFS2_SB_INFO((*inode).i_sb);
    let start = (pos as usize) & (PAGE_SIZE - 1); let end = start + copied as usize;
    let mut aligned_start = start & !3; let mut ret = 0; let mut writtenlen = 0u32;
    if end == PAGE_SIZE { aligned_start = 0; }
    let ri = jffs2_alloc_raw_inode();
    if ri.is_null() { folio_unlock(folio); folio_put(folio); return -12; }
    (*ri).ino = cpu_to_je32((*inode).i_ino); (*ri).mode = cpu_to_jemode((*inode).i_mode);
    (*ri).uid = cpu_to_je16(i_uid_read(inode)); (*ri).gid = cpu_to_je16(i_gid_read(inode));
    (*ri).isize = cpu_to_je32((*inode).i_size as u32);
    (*ri).atime = cpu_to_je32(JFFS2_NOW()); (*ri).ctime = (*ri).atime; (*ri).mtime = (*ri).atime;
    let buf = kmap_local_folio(folio, aligned_start);
    ret = jffs2_write_inode_range(c, f, ri, buf, folio_pos(folio) + aligned_start as u64,
                                  (end - aligned_start) as u32, &mut writtenlen);
    kunmap_local(buf); if ret != 0 { mapping_set_error(mapping, ret); }
    writtenlen = writtenlen.wrapping_sub(core::cmp::min(writtenlen, (start - aligned_start) as u32));
    if writtenlen != 0 && (*inode).i_size < pos + writtenlen as i64 { (*inode).i_size = pos + writtenlen as i64; (*inode).i_blocks = ((*inode).i_size + 511) >> 9; }
    jffs2_free_raw_inode(ri); if start + writtenlen as usize < end { folio_clear_uptodate(folio); }
    folio_unlock(folio); folio_put(folio); if writtenlen > 0 { writtenlen as i32 } else { ret }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
