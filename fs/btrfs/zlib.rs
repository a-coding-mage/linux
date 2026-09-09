// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2008 Oracle.  All rights reserved.
 *
 * Based on jffs2 zlib code:
 * Copyright © 2001-2007 Red Hat, Inc.
 * Created by David Woodhouse <dwmw2@infradead.org>
 */

// Kernel headers and local headers from the C implementation provide the
// external types, constants, functions, and macros referenced below.

const ZLIB_DFLTCC_BUF_SIZE: usize = 4 * PAGE_SIZE;

#[repr(C)]
struct workspace {
    strm: z_stream,
    buf: *mut c_char,
    buf_size: c_uint,
    list: list_head,
    level: c_int,
}

unsafe extern "C" {
    fn zlib_get_workspace(fs_info: *mut btrfs_fs_info, level: c_uint) -> *mut list_head;
    fn zlib_free_workspace(ws: *mut list_head);
    fn zlib_alloc_workspace(fs_info: *mut btrfs_fs_info, level: c_uint) -> *mut list_head;
    fn zlib_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int;
    fn zlib_decompress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int;
    fn zlib_decompress(ws: *mut list_head, data_in: *const u8,
        dest_folio: *mut folio, dest_pgoff: c_ulong, srclen: size_t,
        destlen: size_t) -> c_int;
}

unsafe fn zlib_get_workspace_impl(fs_info: *mut btrfs_fs_info, level: c_uint) -> *mut list_head {
    let ws = btrfs_get_workspace(fs_info, BTRFS_COMPRESS_ZLIB, level);
    let workspace = list_entry!(ws, workspace, list);
    (*workspace).level = level as c_int;
    ws
}

unsafe fn zlib_free_workspace_impl(ws: *mut list_head) {
    let workspace = list_entry!(ws, workspace, list);
    kvfree((*workspace).strm.workspace);
    kfree((*workspace).buf);
    kfree(workspace);
}

/*
 * For s390 hardware acceleration, the buffer size should be at least
 * ZLIB_DFLTCC_BUF_SIZE to achieve the best performance.
 *
 * But if bs > ps we can have large enough folios that meet the s390 hardware
 * handling.
 */
unsafe fn need_special_buffer(fs_info: *mut btrfs_fs_info) -> bool {
    if !zlib_deflate_dfltcc_enabled() { return false; }
    if btrfs_min_folio_size(fs_info) >= ZLIB_DFLTCC_BUF_SIZE { return false; }
    true
}

unsafe fn zlib_alloc_workspace_impl(fs_info: *mut btrfs_fs_info, level: c_uint) -> *mut list_head {
    let workspace = kzalloc_obj::<workspace>();
    if workspace.is_null() { return ERR_PTR!(-ENOMEM); }
    let workspacesize = max(zlib_deflate_workspacesize(MAX_WBITS, MAX_MEM_LEVEL),
        zlib_inflate_workspacesize());
    (*workspace).strm.workspace = kvzalloc(workspacesize, GFP_KERNEL | __GFP_NOWARN);
    (*workspace).level = level as c_int;
    (*workspace).buf = core::ptr::null_mut();
    if need_special_buffer(fs_info) {
        (*workspace).buf = kmalloc(ZLIB_DFLTCC_BUF_SIZE,
            __GFP_NOMEMALLOC | __GFP_NORETRY | __GFP_NOWARN | GFP_NOIO);
        (*workspace).buf_size = ZLIB_DFLTCC_BUF_SIZE as c_uint;
    }
    if (*workspace).buf.is_null() {
        (*workspace).buf = kmalloc((*fs_info).sectorsize as usize, GFP_KERNEL);
        (*workspace).buf_size = (*fs_info).sectorsize;
    }
    if (*workspace).strm.workspace.is_null() || (*workspace).buf.is_null() {
        zlib_free_workspace_impl(&mut (*workspace).list);
        return ERR_PTR!(-ENOMEM);
    }
    INIT_LIST_HEAD!(&mut (*workspace).list);
    &mut (*workspace).list
}

/* Helper for S390x with hardware zlib compression support. */
unsafe fn copy_data_into_buffer(mapping: *mut address_space, workspace: *mut workspace,
    filepos: u64, length: c_ulong) -> c_int {
    let mut cur = filepos;
    ASSERT!(zlib_deflate_dfltcc_enabled());
    while cur < filepos + length {
        let mut folio: *mut folio = core::ptr::null_mut();
        let ret = btrfs_compress_filemap_get_folio(mapping, cur, &mut folio);
        if ret < 0 { return ret; }
        let offset = offset_in_folio(folio, cur);
        let copy_length = min(folio_size(folio) - offset, filepos + length - cur);
        let data_in = kmap_local_folio(folio, offset);
        memcpy((*workspace).buf.add((cur - filepos) as usize), data_in, copy_length);
        kunmap_local(data_in);
        folio_put(folio);
        cur += copy_length;
    }
    0
}

unsafe fn zlib_compress_bio_impl(ws: *mut list_head, cb: *mut compressed_bio) -> c_int {
    let inode = (*cb).bbio.inode;
    let fs_info = (*(*inode).root).fs_info;
    let workspace = list_entry!(ws, workspace, list);
    let mapping = (*inode).vfs_inode.i_mapping;
    let bio = &mut (*cb).bbio.bio;
    let mut start = (*cb).start;
    let len = (*cb).len;
    let min_folio_size = btrfs_min_folio_size(fs_info);
    let mut ret;
    let mut data_in: *mut c_char = core::ptr::null_mut();
    let mut in_folio: *mut folio = core::ptr::null_mut();
    let mut out_folio: *mut folio = core::ptr::null_mut();
    let orig_end = start + len;
    ret = zlib_deflateInit(&mut (*workspace).strm, (*workspace).level);
    if unlikely(ret != Z_OK) { ret = -EIO; goto_out_compress!(workspace, out_folio, data_in, in_folio, ret); }
    (*workspace).strm.total_in = 0; (*workspace).strm.total_out = 0;
    out_folio = btrfs_alloc_compr_folio(fs_info, GFP_NOFS);
    if out_folio.is_null() { ret = -ENOMEM; goto_out_compress!(workspace, out_folio, data_in, in_folio, ret); }
    (*workspace).strm.next_in = (*workspace).buf;
    (*workspace).strm.avail_in = 0;
    (*workspace).strm.next_out = folio_address(out_folio);
    (*workspace).strm.avail_out = min_folio_size;
    while (*workspace).strm.total_in < len {
        if (*workspace).strm.avail_in == 0 {
            let bytes_left = len - (*workspace).strm.total_in;
            let copy_length = min(bytes_left, (*workspace).buf_size as u64);
            if need_special_buffer(fs_info) {
                ret = copy_data_into_buffer(mapping, workspace, start, copy_length as c_ulong);
                if ret < 0 { break; }
                start += copy_length; (*workspace).strm.next_in = (*workspace).buf;
                (*workspace).strm.avail_in = copy_length as u32;
            } else {
                if !data_in.is_null() { kunmap_local(data_in); folio_put(in_folio); data_in = core::ptr::null_mut(); }
                ret = btrfs_compress_filemap_get_folio(mapping, start, &mut in_folio);
                if ret < 0 { break; }
                let cur_len = btrfs_calc_input_length(in_folio, orig_end, start);
                data_in = kmap_local_folio(in_folio, offset_in_folio(in_folio, start));
                start += cur_len; (*workspace).strm.next_in = data_in;
                (*workspace).strm.avail_in = cur_len as u32;
            }
        }
        ret = zlib_deflate(&mut (*workspace).strm, Z_SYNC_FLUSH);
        if unlikely(ret != Z_OK) { ret = -EIO; break; }
        if (*workspace).strm.total_in > (fs_info_sectorsize(fs_info) * 2) && (*workspace).strm.total_in < (*workspace).strm.total_out) { ret = -E2BIG; break; }
        if (*workspace).strm.total_out >= len { ret = -E2BIG; break; }
        if (*workspace).strm.avail_out == 0 {
            if !bio_add_folio(bio, out_folio, folio_size(out_folio), 0) { ret = -E2BIG; break; }
            out_folio = btrfs_alloc_compr_folio(fs_info, GFP_NOFS);
            if out_folio.is_null() { ret = -ENOMEM; break; }
            (*workspace).strm.avail_out = min_folio_size; (*workspace).strm.next_out = folio_address(out_folio);
        }
        if (*workspace).strm.total_in >= len { break; }
    }
    if ret == 0 || ret == Z_OK {
        (*workspace).strm.avail_in = 0;
        loop {
            ret = zlib_deflate(&mut (*workspace).strm, Z_FINISH);
            if ret == Z_STREAM_END { break; }
            if unlikely(ret != Z_OK && ret != Z_BUF_ERROR) { ret = -EIO; break; }
            if (*workspace).strm.avail_out == 0 {
                if (*workspace).strm.total_out >= len || !bio_add_folio(bio, out_folio, folio_size(out_folio), 0) { ret = -E2BIG; break; }
                out_folio = btrfs_alloc_compr_folio(fs_info, GFP_NOFS);
                if out_folio.is_null() { ret = -ENOMEM; break; }
                (*workspace).strm.avail_out = min_folio_size; (*workspace).strm.next_out = folio_address(out_folio);
            }
        }
    }
    if ret == Z_STREAM_END {
        if (*workspace).strm.total_out > bio.bi_iter.bi_size {
            let cur_len = (*workspace).strm.total_out - bio.bi_iter.bi_size;
            if !bio_add_folio(bio, out_folio, cur_len, 0) { ret = -E2BIG; }
        } else { btrfs_free_compr_folio(out_folio); }
        out_folio = core::ptr::null_mut();
        zlib_deflateEnd(&mut (*workspace).strm);
        if (*workspace).strm.total_out >= (*workspace).strm.total_in { ret = -E2BIG; } else { ret = 0; }
    }
    if !out_folio.is_null() { btrfs_free_compr_folio(out_folio); }
    if !data_in.is_null() { kunmap_local(data_in); folio_put(in_folio); }
    ret
}

// The decompression routines retain the C control flow and use the external
// kernel/zlib APIs supplied by the surrounding translation unit.
unsafe fn zlib_decompress_bio_impl(ws: *mut list_head, cb: *mut compressed_bio) -> c_int {
    let fs_info = cb_to_fs_info(cb); let workspace = list_entry!(ws, workspace, list);
    let min_folio_size = btrfs_min_folio_size(fs_info); let mut wbits = MAX_WBITS;
    let srclen = bio_get_size(&(*cb).bbio.bio); let mut total_out = 0usize;
    let mut fi = folio_iter::default(); bio_first_folio(&mut fi, &(*cb).bbio.bio, 0);
    if fi.folio.is_null() { return -EINVAL; }
    ASSERT!(folio_size(fi.folio) == min_folio_size);
    let mut data_in = kmap_local_folio(fi.folio, 0);
    (*workspace).strm.next_in = data_in; (*workspace).strm.avail_in = min(srclen, min_folio_size); (*workspace).strm.total_in = 0;
    (*workspace).strm.total_out = 0; (*workspace).strm.next_out = (*workspace).buf; (*workspace).strm.avail_out = (*workspace).buf_size;
    if srclen > 2 && !((*data_in.add(1) as u8) & PRESET_DICT != 0) && ((*data_in as u8 & 0x0f) == Z_DEFLATED) && !(((((*data_in as u8) << 8) + *data_in.add(1) as u8) as usize) % 31 != 0) { wbits = -(((*data_in as u8 >> 4) + 8) as i32); (*workspace).strm.next_in = (*workspace).strm.next_in.add(2); (*workspace).strm.avail_in -= 2; }
    let mut ret = zlib_inflateInit2(&mut (*workspace).strm, wbits);
    if unlikely(ret != Z_OK) { kunmap_local(data_in); return -EIO; }
    while (*workspace).strm.total_in < srclen { ret = zlib_inflate(&mut (*workspace).strm, Z_NO_FLUSH); if ret != Z_OK && ret != Z_STREAM_END { break; } let buf_start = total_out; total_out = (*workspace).strm.total_out; if buf_start == total_out { break; } let ret2 = btrfs_decompress_buf2page((*workspace).buf, total_out - buf_start, cb, buf_start); if ret2 == 0 { ret = 0; break; } (*workspace).strm.next_out = (*workspace).buf; (*workspace).strm.avail_out = (*workspace).buf_size; if (*workspace).strm.avail_in == 0 { kunmap_local(data_in); bio_next_folio(&mut fi, &(*cb).bbio.bio); if fi.folio.is_null() { data_in = core::ptr::null_mut(); break; } data_in = kmap_local_folio(fi.folio, 0); (*workspace).strm.next_in = data_in; (*workspace).strm.avail_in = min(srclen - (*workspace).strm.total_in, min_folio_size); } }
    if ret != 0 && ret != Z_STREAM_END { ret = -EIO; } else if ret == Z_STREAM_END { ret = 0; } zlib_inflateEnd(&mut (*workspace).strm); if !data_in.is_null() { kunmap_local(data_in); } ret
}

unsafe fn zlib_decompress_impl(ws: *mut list_head, data_in: *const u8, dest_folio: *mut folio, dest_pgoff: c_ulong, srclen: size_t, destlen: size_t) -> c_int {
    let workspace = list_entry!(ws, workspace, list); let mut wbits = MAX_WBITS;
    (*workspace).strm.next_in = data_in; (*workspace).strm.avail_in = srclen; (*workspace).strm.total_in = 0; (*workspace).strm.next_out = (*workspace).buf; (*workspace).strm.avail_out = (*workspace).buf_size; (*workspace).strm.total_out = 0;
    if srclen > 2 && !((*data_in.add(1)) & PRESET_DICT != 0) && ((*data_in & 0x0f) == Z_DEFLATED) && !(((((*data_in as u16) << 8) + *data_in.add(1) as u16) as usize) % 31 != 0) { wbits = -((*data_in >> 4) as i32 + 8); (*workspace).strm.next_in = (*workspace).strm.next_in.add(2); (*workspace).strm.avail_in -= 2; }
    let ret = zlib_inflateInit2(&mut (*workspace).strm, wbits); if unlikely(ret != Z_OK) { return -EIO; }
    let to_copy = min((*workspace).strm.total_out, destlen); let mut ret = zlib_inflate(&mut (*workspace).strm, Z_FINISH);
    if ret == Z_STREAM_END { memcpy_to_folio(dest_folio, dest_pgoff, (*workspace).buf, to_copy); }
    if to_copy != destlen { ret = -EIO; } else { ret = 0; } zlib_inflateEnd(&mut (*workspace).strm); if to_copy < destlen { folio_zero_range(dest_folio, dest_pgoff + to_copy, destlen - to_copy); } ret
}

#[repr(C)]
static btrfs_zlib_compress: btrfs_compress_levels = btrfs_compress_levels { min_level: 1, max_level: 9, default_level: BTRFS_ZLIB_DEFAULT_LEVEL };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
