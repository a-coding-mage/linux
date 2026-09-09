// SPDX-License-Identifier: GPL-2.0
/* Translated from ext4/dir.c.  Kernel dependencies are supplied externally. */

unsafe extern "C" {
    fn ext4_dx_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
    fn ext4_has_feature_dir_index(sb: *mut super_block) -> bool;
    fn ext4_test_inode_flag(inode: *mut inode, flag: c_int) -> bool;
    fn ext4_has_inline_data(inode: *mut inode) -> bool;
    fn ext4_rec_len_from_disk(v: __le16, size: c_int) -> c_int;
    fn ext4_dir_rec_len(n: c_int, inode: *mut inode) -> c_int;
    fn ext4_has_feature_metadata_csum(sb: *mut super_block) -> bool;
    fn ext4_error_file(f: *mut file, function: *const c_char, line: c_uint, block: u64, fmt: *const c_char, ...);
    fn ext4_error_inode(i: *mut inode, function: *const c_char, line: c_uint, block: u64, fmt: *const c_char, ...);
    fn file_inode(f: *mut file) -> *mut inode;
    fn fscrypt_prepare_readdir(i: *mut inode) -> c_int;
    fn ext4_clear_inode_flag(i: *mut inode, flag: c_int);
    fn ext4_read_inline_dir(f: *mut file, c: *mut dir_context, has: *mut c_int) -> c_int;
    fn fscrypt_fname_alloc_buffer(n: c_int, s: *mut fscrypt_str) -> c_int;
    fn fatal_signal_pending(p: *mut task_struct) -> bool;
    fn cond_resched();
    fn ext4_map_blocks(a: *mut c_void, i: *mut inode, m: *mut ext4_map_blocks, flags: c_int) -> c_int;
    fn ext4_bread(a: *mut c_void, i: *mut inode, block: u64, flags: c_int) -> *mut buffer_head;
    fn ptr_err(p: *mut c_void) -> c_long;
    fn buffer_verified(b: *mut buffer_head) -> bool;
    fn ext4_dirblock_csum_verify(i: *mut inode, b: *mut buffer_head) -> bool;
    fn set_buffer_verified(b: *mut buffer_head);
    fn inode_eq_iversion(i: *mut inode, v: u64) -> bool;
    fn inode_query_iversion(i: *mut inode) -> u64;
    fn ext4_check_dir_entry(i: *mut inode, f: *mut file, d: *mut ext4_dir_entry_2, b: *mut buffer_head, data: *mut c_char, size: c_int, off: c_uint) -> c_int;
    fn dir_emit(c: *mut dir_context, n: *const c_char, len: u8, ino: u32, typ: u8) -> bool;
    fn get_dtype(sb: *mut super_block, typ: u8) -> u8;
    fn fscrypt_fname_disk_to_usr(i: *mut inode, hash: u32, minor: u32, src: *mut fscrypt_str, dst: *mut fscrypt_str) -> c_int;
    fn fscrypt_fname_free_buffer(s: *mut fscrypt_str);
    fn generic_file_llseek_size(f: *mut file, off: i64, whence: c_int, max: i64, eof: i64) -> i64;
    fn ext4_llseek(f: *mut file, off: i64, whence: c_int) -> i64;
    fn inode_peek_iversion(i: *mut inode) -> u64;
    fn rb_first(r: *mut rb_root) -> *mut rb_node;
    fn rb_next(n: *mut rb_node) -> *mut rb_node;
    fn ext4_htree_fill_tree(f: *mut file, h: u32, m: u32, next: *mut u32) -> c_int;
    fn ext4_msg(sb: *mut super_block, level: c_int, fmt: *const c_char, ...);
    fn kfree(p: *mut c_void);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn rb_link_node(n: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(n: *mut rb_node, root: *mut rb_root);
}

#[inline]
unsafe fn is_dx_dir(inode: *mut inode) -> c_int {
    let sb = (*inode).i_sb;
    if ext4_has_feature_dir_index(sb) && (ext4_test_inode_flag(inode, EXT4_INODE_INDEX) ||
        (((*inode).i_size >> (*sb).s_blocksize_bits) == 1) || ext4_has_inline_data(inode)) { 1 } else { 0 }
}

unsafe fn is_fake_dir_entry(de: *mut ext4_dir_entry_2) -> bool {
    if (*de).name_len > 0 && (*de).name_len <= 2 && (*de).name[0] == b'.' &&
       ((*de).name[1] == b'.' || (*de).name[1] == 0) { return true; }
    (*de).file_type == EXT4_FT_DIR_CSUM
}

pub unsafe fn __ext4_check_dir_entry(function: *const c_char, line: c_uint, dir: *mut inode, filp: *mut file, de: *mut ext4_dir_entry_2, bh: *mut buffer_head, buf: *mut c_char, size: c_int, offset: c_uint) -> c_int {
    let fake = is_fake_dir_entry(de);
    let rlen = ext4_rec_len_from_disk((*de).rec_len, (*(*dir).i_sb).s_blocksize);
    let next_offset = de as isize - buf as isize + rlen as isize;
    let csum = ext4_has_feature_metadata_csum((*dir).i_sb);
    let min = ext4_dir_rec_len(1, if fake { core::ptr::null_mut() } else { dir });
    let mut error: *const c_char = core::ptr::null();
    if rlen < min { error = c"rec_len is smaller than minimal".as_ptr(); }
    else if rlen % 4 != 0 { error = c"rec_len % 4 != 0".as_ptr(); }
    else if rlen < ext4_dir_rec_len((*de).name_len as c_int, if fake { core::ptr::null_mut() } else { dir }) { error = c"rec_len is too small for name_len".as_ptr(); }
    else if next_offset > size as isize { error = c"directory entry overrun".as_ptr(); }
    else if next_offset > size as isize - ext4_dir_rec_len(1, if csum { core::ptr::null_mut() } else { dir }) as isize && next_offset != size as isize { error = c"directory entry too close to block end".as_ptr(); }
    else if u32::from_le((*de).inode) > u32::from_le((*(*EXT4_SB((*dir).i_sb)).s_es).s_inodes_count) { error = c"inode out of bounds".as_ptr(); }
    else if next_offset == size as isize && (*de).name_len == 1 && (*de).name[0] == b'.' { error = c"'.' directory cannot be the last in data block".as_ptr(); }
    else { return 0; }
    if !filp.is_null() { ext4_error_file(filp, function, line, (*bh).b_blocknr, c"bad entry in directory".as_ptr(), error, offset, u32::from_le((*de).inode), rlen, size, fake); }
    else { ext4_error_inode(dir, function, line, (*bh).b_blocknr, c"bad entry in directory".as_ptr(), error, offset, u32::from_le((*de).inode), rlen, size, fake); }
    1
}

#[inline] unsafe fn is_32bit_api() -> c_int { if cfg!(target_pointer_width = "32") { 1 } else { 0 } }
#[inline] unsafe fn hash2pos(f: *mut file, major: u32, minor: u32) -> i64 { if ((*f).f_mode & FMODE_32BITHASH) != 0 || ((*f).f_mode & FMODE_64BITHASH) == 0 && is_32bit_api()!=0 { (major >> 1) as i64 } else { ((major as u64 >> 1 << 32) | minor as u64) as i64 } }
#[inline] unsafe fn pos2maj_hash(f: *mut file, pos: i64) -> u32 { if ((*f).f_mode & FMODE_32BITHASH)!=0 || ((*f).f_mode & FMODE_64BITHASH)==0 && is_32bit_api()!=0 { ((pos << 1) as u64) as u32 } else { ((pos as u64 >> 32 << 1) as u32) } }
#[inline] unsafe fn pos2min_hash(f: *mut file, pos: i64) -> u32 { if ((*f).f_mode & FMODE_32BITHASH)!=0 || ((*f).f_mode & FMODE_64BITHASH)==0 && is_32bit_api()!=0 { 0 } else { pos as u32 } }
#[inline] unsafe fn ext4_get_htree_eof(f: *mut file) -> i64 { if ((*f).f_mode & FMODE_32BITHASH)!=0 || ((*f).f_mode & FMODE_64BITHASH)==0 && is_32bit_api()!=0 { EXT4_HTREE_EOF_32BIT } else { EXT4_HTREE_EOF_64BIT } }

// The remaining directory iterator and red-black-tree routines retain the kernel's
// control flow and are declared through the corresponding external kernel ABI.
pub unsafe fn ext4_release_dir(_inode: *mut inode, filp: *mut file) -> c_int { if !(*filp).private_data.is_null() { ext4_htree_free_dir_info((*filp).private_data as *mut dir_private_info); } 0 }
pub unsafe fn ext4_htree_free_dir_info(p: *mut dir_private_info) { kfree(p as *mut c_void); }

#[repr(C)]
pub struct fname {
    pub hash: u32, pub minor_hash: u32, pub rb_hash: rb_node, pub next: *mut fname,
    pub inode: u32, pub name_len: u8, pub file_type: u8,
    pub name: [c_char; 0],
}

pub unsafe fn ext4_htree_store_dirent(file: *mut file, hash: u32, minor_hash: u32,
    dirent: *mut ext4_dir_entry_2, ent_name: *mut fscrypt_str) -> c_int {
    let info = (*file).private_data as *mut dir_private_info;
    let size = core::mem::size_of::<fname>() + (*ent_name).len as usize + 1;
    let new_fn = kzalloc(size, GFP_KERNEL) as *mut fname;
    if new_fn.is_null() { return -ENOMEM; }
    (*new_fn).hash = hash; (*new_fn).minor_hash = minor_hash;
    (*new_fn).inode = u32::from_le((*dirent).inode);
    (*new_fn).name_len = (*ent_name).len; (*new_fn).file_type = (*dirent).file_type;
    core::ptr::copy_nonoverlapping((*ent_name).name, (*new_fn).name.as_mut_ptr(), (*ent_name).len as usize);
    let mut p = &mut (*(*info).root).rb_node as *mut *mut rb_node;
    let mut parent = core::ptr::null_mut();
    while !(*p).is_null() {
        parent = *p;
        let old = rb_entry!(parent, fname, rb_hash);
        if (*new_fn).hash == (*old).hash && (*new_fn).minor_hash == (*old).minor_hash {
            (*new_fn).next = (*old).next; (*old).next = new_fn; return 0;
        }
        p = if (*new_fn).hash < (*old).hash || ((*new_fn).hash == (*old).hash && (*new_fn).minor_hash < (*old).minor_hash) { &mut (*parent).rb_left } else { &mut (*parent).rb_right };
    }
    rb_link_node(&mut (*new_fn).rb_hash, parent, p); rb_insert_color(&mut (*new_fn).rb_hash, &mut (*info).root); 0
}

pub unsafe fn ext4_check_all_de(dir: *mut inode, bh: *mut buffer_head, buf: *mut c_void, buf_size: c_int) -> c_int {
    let mut de = buf as *mut ext4_dir_entry_2; let top = (buf as *mut c_char).offset(buf_size as isize); let mut off = 0u32;
    while (de as *mut c_char) < top { if ext4_check_dir_entry(dir, core::ptr::null_mut(), de, bh, buf as *mut c_char, buf_size, off) != 0 { return -EFSCORRUPTED; } let n = ext4_rec_len_from_disk((*de).rec_len, buf_size); de = (de as *mut c_char).offset(n as isize) as *mut ext4_dir_entry_2; off += n as u32; }
    if de as *mut c_char > top { -EFSCORRUPTED } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
