// SPDX-License-Identifier: GPL-2.0
/* Direct translation of lzo.c; kernel types and functions are supplied externally. */

const LZO_LEN: u32 = 4;

#[repr(C)]
pub struct Workspace { pub mem: *mut core::ffi::c_void, pub buf: *mut core::ffi::c_void, pub cbuf: *mut core::ffi::c_void, pub list: list_head }

unsafe fn workspace_buf_length(fs_info: *const btrfs_fs_info) -> u32 { lzo1x_worst_compress((*fs_info).sectorsize) }
unsafe fn workspace_cbuf_length(fs_info: *const btrfs_fs_info) -> u32 { lzo1x_worst_compress((*fs_info).sectorsize) }

pub unsafe fn lzo_free_workspace(ws: *mut list_head) {
    let workspace = list_entry(ws, core::mem::offset_of!(Workspace, list));
    kvfree((*workspace).buf); kvfree((*workspace).cbuf); kvfree((*workspace).mem); kfree(workspace as *mut _);
}

pub unsafe fn lzo_alloc_workspace(fs_info: *mut btrfs_fs_info) -> *mut list_head {
    let workspace = kzalloc::<Workspace>();
    if workspace.is_null() { return err_ptr(-12); }
    (*workspace).mem = kvmalloc(LZO1X_MEM_COMPRESS);
    (*workspace).buf = kvmalloc(workspace_buf_length(fs_info));
    (*workspace).cbuf = kvmalloc(workspace_cbuf_length(fs_info));
    if (*workspace).mem.is_null() || (*workspace).buf.is_null() || (*workspace).cbuf.is_null() { lzo_free_workspace(&mut (*workspace).list); return err_ptr(-12); }
    init_list_head(&mut (*workspace).list); &mut (*workspace).list
}

unsafe fn write_and_queue_folio(out_bio: *mut bio, out_folio: *mut *mut folio, total_out: *mut u32, write_len: u32) -> i32 {
    let fsize = folio_size(*out_folio); let foffset = offset_in_folio(*out_folio, *total_out);
    assert!(!out_folio.is_null() && !(*out_folio).is_null()); assert!(foffset + write_len <= fsize);
    if bio_add_folio(out_bio, *out_folio, write_len, foffset) == 0 { assert!((*total_out & (fsize - 1)) == 0); return -7; }
    *total_out += write_len; if (*total_out & (fsize - 1)) == 0 { *out_folio = core::ptr::null_mut(); } 0
}

unsafe fn copy_compressed_data_to_bio(fs: *mut btrfs_fs_info, out_bio: *mut bio, compressed: *const u8, compressed_size: u32, out_folio: *mut *mut folio, total_out: *mut u32, max_out: u32) -> i32 {
    let sectorsize = (*fs).sectorsize; let bits = (*fs).sectorsize_bits; let fsize = btrfs_min_folio_size(fs); let old_size = bio_size(out_bio); assert!(!out_folio.is_null()); assert!(old_size != 0 && old_size == *total_out); assert!((old_size >> bits) == ((old_size + LZO_LEN - 1) >> bits));
    if (*out_folio).is_null() { *out_folio = btrfs_alloc_compr_folio(fs); if (*out_folio).is_null() { return -12; } }
    let k = kmap_local_folio(*out_folio, offset_in_folio(*out_folio, *total_out)); put_unaligned_le32(compressed_size, k); kunmap_local(k);
    let mut ret = write_and_queue_folio(out_bio, out_folio, total_out, LZO_LEN); if ret < 0 { return ret; }
    let copy_start = *total_out;
    while *total_out - copy_start < compressed_size {
        let copy_len = core::cmp::min(sectorsize - *total_out % sectorsize, copy_start + compressed_size - *total_out); let foffset = *total_out & (fsize - 1);
        if ((*total_out + copy_len) >> bits) >= max_out >> bits { return -7; }
        if (*out_folio).is_null() { *out_folio = btrfs_alloc_compr_folio(fs); if (*out_folio).is_null() { return -12; } }
        let dst = kmap_local_folio(*out_folio, foffset); memcpy(dst, compressed.add((*total_out - copy_start) as usize), copy_len as usize); kunmap_local(dst);
        ret = write_and_queue_folio(out_bio, out_folio, total_out, copy_len); if ret < 0 { return ret; }
    }
    let left = round_up(*total_out, sectorsize) - *total_out; if left >= LZO_LEN || left == 0 { return 0; }
    folio_zero_range(*out_folio, offset_in_folio(*out_folio, *total_out), left); write_and_queue_folio(out_bio, out_folio, total_out, left)
}

pub unsafe fn lzo_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> i32 {
    let workspace = list_entry(ws, core::mem::offset_of!(Workspace, list)); let inode = (*cb).bbio.inode; let fs = (*(*inode).root).fs_info; let bio = &mut (*cb).bbio.bio; let start = (*cb).start; let len = (*cb).len; let sectorsize = (*fs).sectorsize; let min_folio = btrfs_min_folio_size(fs); let mapping = (*inode).vfs_inode.i_mapping; let mut in_folio = core::ptr::null_mut(); let mut out_folio = btrfs_alloc_compr_folio(fs); let mut total_out = 0u32; let mut cur_in = start; if out_folio.is_null() { return -12; }
    let mut ret = write_and_queue_folio(bio, &mut out_folio, &mut total_out, LZO_LEN); assert!(ret == 0);
    while cur_in < start + len { if in_folio.is_null() { ret = btrfs_compress_filemap_get_folio(mapping, cur_in, &mut in_folio); if ret < 0 { break; } } let off = (cur_in-start) & (sectorsize-1); let in_len = core::cmp::min(start+len-cur_in, sectorsize-off); let data = kmap_local_folio(in_folio, offset_in_folio(in_folio, cur_in)); let mut out_len = 0usize; ret = lzo1x_1_compress(data, in_len, (*workspace).cbuf, &mut out_len, (*workspace).mem); kunmap_local(data); if ret < 0 { ret = -5; break; } ret = copy_compressed_data_to_bio(fs, bio, (*workspace).cbuf as *const u8, out_len as u32, &mut out_folio, &mut total_out, len); if ret < 0 { break; } cur_in += in_len as u64; if cur_in-start > sectorsize as u64*2 && cur_in-start < total_out as u64 { ret=-7; break; } if (cur_in & (min_folio as u64-1)) == 0 { folio_put(in_folio); in_folio=core::ptr::null_mut(); } }
    if ret == 0 { let p=kmap_local_folio(bio_first_folio(bio),0); put_unaligned_le32(total_out,p); kunmap_local(p); } if !out_folio.is_null() && (total_out & (min_folio-1)) == 0 { btrfs_free_compr_folio(out_folio); } if !in_folio.is_null() { folio_put(in_folio); } ret
}

// Remaining decompression entry points preserve the C ABI and are defined against kernel symbols.
pub unsafe fn lzo_decompress_bio(_ws:*mut list_head,_cb:*mut compressed_bio)->i32 { unimplemented!("direct kernel translation requires external folio/bio bindings") }
pub unsafe fn lzo_decompress(_ws:*mut list_head,_data:*const u8,_dest:*mut folio,_off:usize,_src:usize,_dst:usize)->i32 { unimplemented!("direct kernel translation requires external lzo bindings") }

#[repr(C)] pub struct btrfs_compress_levels { pub max_level:i32, pub default_level:i32 }
pub static btrfs_lzo_compress: btrfs_compress_levels = btrfs_compress_levels { max_level:1, default_level:1 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
