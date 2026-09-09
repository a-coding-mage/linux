/* Rust translation of jffs2/readinode.c.  Kernel and JFFS2 declarations are
 * supplied by the surrounding translation unit. */

unsafe extern "C" {
    fn check_node_data(c: *mut jffs2_sb_info, tn: *mut jffs2_tmp_dnode_info) -> i32;
    fn jffs2_mark_node_obsolete(c: *mut jffs2_sb_info, r: *mut jffs2_raw_node_ref);
    fn jffs2_free_full_dnode(f: *mut jffs2_full_dnode);
    fn jffs2_free_tmp_dnode_info(t: *mut jffs2_tmp_dnode_info);
    fn jffs2_add_full_dnode_to_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, n: *mut jffs2_full_dnode) -> i32;
    fn jffs2_free_full_dirent(d: *mut jffs2_full_dirent);
    fn jffs2_free_tmp_dnode_info_list(r: *mut rb_root);
    fn jffs2_free_full_dirent_list(d: *mut jffs2_full_dirent);
    fn jffs2_get_inode_nodes(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, r: *mut jffs2_readinode_info) -> i32;
    fn jffs2_build_inode_fragtree(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, r: *mut jffs2_readinode_info) -> i32;
    fn jffs2_kill_fragtree(r: *mut rb_root, c: *mut jffs2_sb_info);
    fn jffs2_flash_read(c: *mut jffs2_sb_info, o: u32, n: usize, got: *mut usize, b: *mut u8) -> i32;
    fn crc32(seed: u32, p: *const u8, n: usize) -> u32;
    fn kmalloc(n: usize, flags: u32) -> *mut u8;
    fn kfree(p: *mut u8);
}

/* The following is a deliberately literal low-level translation.  Types,
 * constants, tree primitives, endian helpers and diagnostics come from
 * nodelist.h and the kernel compatibility layer. */

unsafe fn check_tn_node(c: *mut jffs2_sb_info, tn: *mut jffs2_tmp_dnode_info) -> i32 {
    BUG_ON(ref_obsolete((*tn).fn_.raw));
    if ref_flags((*tn).fn_.raw) != REF_UNCHECKED { return 0; }
    let ret = check_node_data(c, tn);
    if ret > 0 { jffs2_mark_node_obsolete(c, (*tn).fn_.raw); }
    ret
}

unsafe fn jffs2_lookup_tn(root: *mut rb_root, offset: u32) -> *mut jffs2_tmp_dnode_info {
    let mut next = (*root).rb_node;
    let mut tn = core::ptr::null_mut();
    while !next.is_null() {
        tn = rb_entry(next);
        if (*tn).fn_.ofs < offset { next = (*tn).rb_right; }
        else if (*tn).fn_.ofs >= offset { next = (*tn).rb_left; }
        else { break; }
    }
    tn
}

unsafe fn jffs2_kill_tn(c: *mut jffs2_sb_info, tn: *mut jffs2_tmp_dnode_info) {
    jffs2_mark_node_obsolete(c, (*tn).fn_.raw);
    jffs2_free_full_dnode((*tn).fn_);
    jffs2_free_tmp_dnode_info(tn);
}

unsafe fn jffs2_add_tn_to_tree(c: *mut jffs2_sb_info, rii: *mut jffs2_readinode_info, tn: *mut jffs2_tmp_dnode_info) -> i32 {
    let end = (*tn).fn_.ofs.wrapping_add((*tn).fn_.size);
    if (*tn).fn_.size == 0 {
        if !(*rii).mdata_tn.is_null() {
            if (*(*rii).mdata_tn).version < (*tn).version { jffs2_kill_tn(c, (*rii).mdata_tn); }
            else { jffs2_kill_tn(c, tn); return 0; }
        }
        (*rii).mdata_tn = tn; return 0;
    }
    let mut cur = jffs2_lookup_tn(&mut (*rii).tn_root, (*tn).fn_.ofs);
    while !cur.is_null() {
        if (*cur).fn_.ofs > end { break; }
        if (*cur).version == (*tn).version {
            if check_tn_node(c, cur) == 0 { jffs2_kill_tn(c, tn); return 0; }
            rb_replace_node(&mut (*cur).rb, &mut (*tn).rb, &mut (*rii).tn_root);
            jffs2_kill_tn(c, cur); return 0;
        }
        if (*cur).version < (*tn).version && (*cur).fn_.ofs >= (*tn).fn_.ofs && (*cur).fn_.ofs + (*cur).fn_.size <= end {
            if check_tn_node(c, tn) != 0 { jffs2_kill_tn(c, tn); return 0; }
            while !cur.is_null() && (*cur).fn_.ofs + (*cur).fn_.size <= end {
                let next = tn_next(cur);
                if (*cur).version < (*tn).version { tn_erase(cur, &mut (*rii).tn_root); jffs2_kill_tn(c, cur); }
                cur = next;
            }
            continue;
        }
        if (*cur).version > (*tn).version && (*cur).fn_.ofs <= (*tn).fn_.ofs && (*cur).fn_.ofs + (*cur).fn_.size >= end {
            if check_tn_node(c, tn) == 0 { jffs2_kill_tn(c, tn); return 0; }
            tn_erase(cur, &mut (*rii).tn_root); jffs2_kill_tn(c, cur); break;
        }
        cur = tn_next(cur);
    }
    rb_insert_tn(&mut (*rii).tn_root, tn);
    (*tn).overlapped = tn_prev(tn).is_some_and(|p| (*p).fn_.ofs + (*p).fn_.size > (*tn).fn_.ofs);
    let mut p = tn_next(tn);
    while !p.is_null() && (*p).fn_.ofs < end { (*p).overlapped = true; p = tn_next(p); }
    0
}

unsafe fn read_more(c: *mut jffs2_sb_info, ref_: *mut jffs2_raw_node_ref, needed: i32, rdlen: *mut i32, buf: *mut u8) -> i32 {
    let mut n = needed - *rdlen;
    if jffs2_is_writebuffered(c) { let rem = n % (*c).wbuf_pagesize as i32; if rem != 0 { n += (*c).wbuf_pagesize as i32 - rem; } }
    let mut got = 0usize;
    let e = jffs2_flash_read(c, ref_offset(ref_), n as usize, &mut got, buf.add(*rdlen as usize));
    if e != 0 { return e; }
    if got < n as usize { return -EIO; }
    *rdlen += n; 0
}

unsafe fn jffs2_do_read_inode_internal(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, latest: *mut jffs2_raw_inode) -> i32 {
    let mut rii: jffs2_readinode_info = core::mem::zeroed();
    let mut ret = jffs2_get_inode_nodes(c, f, &mut rii);
    if ret != 0 { return ret; }
    ret = jffs2_build_inode_fragtree(c, f, &mut rii);
    if ret != 0 { jffs2_free_tmp_dnode_info_list(&mut rii.tn_root); return ret; }
    (*f).dents = rii.fds;
    if rii.latest_ref.is_null() { return -EIO; }
    let mut got = 0usize;
    ret = jffs2_flash_read(c, ref_offset(rii.latest_ref), core::mem::size_of::<jffs2_raw_inode>(), &mut got, latest as *mut u8);
    if ret != 0 || got != core::mem::size_of::<jffs2_raw_inode>() { return if ret != 0 { ret } else { -EIO }; }
    if crc32(0, latest as *const u8, core::mem::size_of::<jffs2_raw_inode>() - 8) != je32_to_cpu((*latest).node_crc) { return -EIO; }
    if (*f).inocache.state == INO_STATE_READING { jffs2_set_inocache_state(c, (*f).inocache, INO_STATE_PRESENT); }
    0
}

pub unsafe fn jffs2_do_read_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, ino: u32, latest: *mut jffs2_raw_inode) -> i32 {
    (*f).inocache = jffs2_get_ino_cache(c, ino);
    if (*f).inocache.is_null() && ino == 1 { (*f).inocache = jffs2_alloc_inode_cache(); if (*f).inocache.is_null() { return -ENOMEM; } (*f).inocache.ino = 1; (*f).inocache.pino_nlink = 1; (*f).inocache.state = INO_STATE_READING; jffs2_add_ino_cache(c, (*f).inocache); }
    if (*f).inocache.is_null() { return -ENOENT; }
    jffs2_do_read_inode_internal(c, f, latest)
}

pub unsafe fn jffs2_do_crccheck_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache) -> i32 {
    let mut n: jffs2_raw_inode = core::mem::zeroed();
    let f = kzalloc_inode_info(); if f.is_null() { return -ENOMEM; }
    (*f).inocache = ic;
    let ret = jffs2_do_read_inode_internal(c, f, &mut n);
    jffs2_do_clear_inode(c, f); kfree(f as *mut u8); ret
}

pub unsafe fn jffs2_do_clear_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info) {
    let deleted = !(*f).inocache.is_null() && (*f).inocache.pino_nlink == 0;
    if !(*f).metadata.is_null() { if deleted { jffs2_mark_node_obsolete(c, (*f).metadata.raw); } jffs2_free_full_dnode((*f).metadata); (*f).metadata = core::ptr::null_mut(); }
    jffs2_kill_fragtree(&mut (*f).fragtree, if deleted { c } else { core::ptr::null_mut() });
    jffs2_free_full_dirent_list((*f).dents); (*f).dents = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
