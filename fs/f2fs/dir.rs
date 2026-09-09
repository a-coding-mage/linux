// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of fs/f2fs/dir.c.
// Kernel types, constants, macros, and functions referenced below are supplied
// by the surrounding F2FS/Linux compatibility layer.

#[inline]
unsafe fn f2fs_should_fallback_to_linear(dir: *mut inode) -> bool {
    match F2FS_OPTION(F2FS_I_SB(dir)).lookup_mode {
        LOOKUP_PERF => false,
        LOOKUP_COMPAT => true,
        LOOKUP_AUTO => !sb_no_casefold_compat_fallback((*F2FS_I_SB(dir)).sb),
        _ => false,
    }
}

#[cfg(CONFIG_UNICODE)]
extern "C" { static mut f2fs_cf_name_slab: *mut kmem_cache; }

unsafe fn dir_blocks(inode: *mut inode) -> c_ulong {
    (((i_size_read(inode) + PAGE_SIZE - 1) as u64) >> PAGE_SHIFT) as c_ulong
}
unsafe fn dir_buckets(level: c_uint, dir_level: c_int) -> c_uint {
    if level + dir_level as c_uint < MAX_DIR_HASH_DEPTH / 2 { BIT(level + dir_level as c_uint) } else { MAX_DIR_BUCKETS }
}
unsafe fn bucket_blocks(level: c_uint) -> c_uint { if level < MAX_DIR_HASH_DEPTH / 2 { 2 } else { 4 } }

#[cfg(CONFIG_UNICODE)]
pub unsafe fn f2fs_init_casefolded_name(dir: *const inode, fname: *mut f2fs_filename) -> c_int {
    let sb = (*dir).i_sb;
    if IS_CASEFOLDED(dir) && !name_is_dot_dotdot((*fname).usr_fname.name, (*fname).usr_fname.len) {
        let buf = f2fs_kmem_cache_alloc(f2fs_cf_name_slab, GFP_NOFS, false, F2FS_SB(sb));
        if buf.is_null() { return -ENOMEM; }
        let len = utf8_casefold((*sb).s_encoding, (*fname).usr_fname, buf, F2FS_NAME_LEN);
        if len <= 0 {
            kmem_cache_free(f2fs_cf_name_slab, buf);
            if sb_has_strict_encoding(sb) { return -EINVAL; }
            return 0;
        }
        (*fname).cf_name.name = buf; (*fname).cf_name.len = len;
    }
    0
}
#[cfg(CONFIG_UNICODE)]
pub unsafe fn f2fs_free_casefolded_name(fname: *mut f2fs_filename) {
    let buf = (*fname).cf_name.name as *mut u8;
    if !buf.is_null() { kmem_cache_free(f2fs_cf_name_slab, buf); (*fname).cf_name.name = core::ptr::null_mut(); }
}

unsafe fn __f2fs_setup_filename(dir: *const inode, crypt: *const fscrypt_name, fname: *mut f2fs_filename) -> c_int {
    core::ptr::write_bytes(fname as *mut u8, 0, core::mem::size_of::<f2fs_filename>());
    (*fname).usr_fname = (*crypt).usr_fname; (*fname).disk_name = (*crypt).disk_name;
    #[cfg(CONFIG_FS_ENCRYPTION)] { (*fname).crypto_buf = (*crypt).crypto_buf; }
    if (*crypt).is_nokey_name {
        (*fname).hash = cpu_to_le32((*crypt).hash);
    } else {
        let e = f2fs_init_casefolded_name(dir, fname); if e != 0 { f2fs_free_filename(fname); return e; }
        f2fs_hash_filename(dir, fname);
    } 0
}
pub unsafe fn f2fs_setup_filename(dir: *mut inode, iname: *const qstr, lookup: c_int, fname: *mut f2fs_filename) -> c_int {
    let mut c = core::mem::zeroed(); let e = fscrypt_setup_filename(dir, iname, lookup, &mut c); if e != 0 { e } else { __f2fs_setup_filename(dir, &c, fname) }
}
pub unsafe fn f2fs_prepare_lookup(dir: *mut inode, dentry: *mut dentry, fname: *mut f2fs_filename) -> c_int {
    let mut c = core::mem::zeroed(); let e = fscrypt_prepare_lookup(dir, dentry, &mut c); if e != 0 { e } else { __f2fs_setup_filename(dir, &c, fname) }
}
pub unsafe fn f2fs_free_filename(fname: *mut f2fs_filename) {
    #[cfg(CONFIG_FS_ENCRYPTION)] { kfree((*fname).crypto_buf.name); (*fname).crypto_buf.name = core::ptr::null_mut(); }
    f2fs_free_casefolded_name(fname);
}

unsafe fn dir_block_index(level: c_uint, dl: c_int, idx: c_uint) -> c_ulong {
    let mut b = 0; for i in 0..level { b += mul_u32_u32(dir_buckets(i, dl), bucket_blocks(i)) as c_ulong; } b + idx as c_ulong * bucket_blocks(level) as c_ulong
}
unsafe fn f2fs_match_name(dir: *const inode, fname: *const f2fs_filename, n: *const u8, len: u32) -> c_int {
    #[cfg(CONFIG_UNICODE)] if !(*fname).cf_name.name.is_null() { return generic_ci_match(dir, (*fname).usr_fname, &(*fname).cf_name, n, len); }
    let mut f: fscrypt_name = core::mem::zeroed(); f.usr_fname = (*fname).usr_fname; f.disk_name = (*fname).disk_name;
    #[cfg(CONFIG_FS_ENCRYPTION)] { f.crypto_buf = (*fname).crypto_buf; } fscrypt_match_name(&f, n, len)
}

pub unsafe fn f2fs_find_target_dentry(d: *const f2fs_dentry_ptr, fname: *const f2fs_filename, max_slots: *mut c_int, use_hash: bool) -> *mut f2fs_dir_entry {
    if !max_slots.is_null() { *max_slots = 0; } let mut bit = 0; let mut max_len = 0; let mut de;
    while bit < (*d).max { if !test_bit_le(bit, (*d).bitmap) { bit += 1; max_len += 1; continue; }
        de = (*d).dentry.add(bit as usize); if (*de).name_len == 0 { bit += 1; continue; }
        let nl = le16_to_cpu((*de).name_len); if nl > F2FS_NAME_LEN || bit + GET_DENTRY_SLOTS(nl) > (*d).max { return ERR_PTR(-EFSCORRUPTED); }
        if !use_hash || (*de).hash_code == (*fname).hash { let r = f2fs_match_name((*d).inode, fname, (*d).filename[bit as usize], nl as u32); if r < 0 { return ERR_PTR(r); } if r != 0 { return de; } }
        if !max_slots.is_null() && max_len > *max_slots { *max_slots = max_len; } max_len = 0; bit += GET_DENTRY_SLOTS(nl);
    }
    if !max_slots.is_null() && max_len > *max_slots { *max_slots = max_len; } core::ptr::null_mut()
}

// Remaining directory operations retain the C control flow and kernel ABI.
// They are declared with Rust signatures so dependent translation units can
// provide the surrounding implementation.
extern "C" {
    fn f2fs_find_entry(dir: *mut inode, child: *const qstr, res: *mut *mut folio) -> *mut f2fs_dir_entry;
}

pub unsafe fn f2fs_parent_dir(dir: *mut inode, f: *mut *mut folio) -> *mut f2fs_dir_entry { f2fs_find_entry(dir, &dotdot_name, f) }
pub unsafe fn f2fs_inode_by_name(dir: *mut inode, q: *const qstr, folio: *mut *mut folio) -> ino_t {
    let de = f2fs_find_entry(dir, q, folio); if de.is_null() { 0 } else { let r = le32_to_cpu((*de).ino); f2fs_folio_put(*folio, false); r }
}
pub unsafe fn f2fs_set_link(dir: *mut inode, de: *mut f2fs_dir_entry, folio: *mut folio, inode: *mut inode) {
    folio_lock(folio); f2fs_folio_wait_writeback(folio, if f2fs_has_inline_dentry(dir) { NODE } else { DATA }, true, true);
    (*de).ino = cpu_to_le32((*inode).i_ino); (*de).file_type = fs_umode_to_ftype((*inode).i_mode); folio_mark_dirty(folio);
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir)); f2fs_mark_inode_dirty_sync(dir, true); f2fs_folio_put(folio, true);
}
pub unsafe fn f2fs_do_make_empty_dir(inode: *mut inode, parent: *mut inode, d: *mut f2fs_dentry_ptr) {
    let dot = FSTR_INIT(b".".as_ptr() as *mut u8, 1); let dotdot = FSTR_INIT(b"..".as_ptr() as *mut u8, 2);
    f2fs_update_dentry((*inode).i_ino, (*inode).i_mode, d, &dot, 0, 0); f2fs_update_dentry((*parent).i_ino, (*parent).i_mode, d, &dotdot, 0, 1);
}
pub unsafe fn f2fs_room_for_filename(bitmap: *const c_void, slots: c_int, max: c_int) -> c_int {
    let mut start = 0; loop { let zs = find_next_zero_bit_le(bitmap, max, start); if zs >= max { return max; } let ze = find_next_bit_le(bitmap, max, zs); if ze-zs >= slots { return zs; } start=ze+1; if ze+1>=max { return max; } }
}
pub unsafe fn f2fs_has_enough_room(dir: *mut inode, ifolio: *mut folio, fname: *const f2fs_filename) -> bool {
    let mut d: f2fs_dentry_ptr = core::mem::zeroed(); make_dentry_ptr_inline(dir, &mut d, inline_data_addr(dir, ifolio));
    f2fs_room_for_filename(d.bitmap, GET_DENTRY_SLOTS((*fname).disk_name.len) as c_int, d.max as c_int) < d.max as c_int
}
pub unsafe fn f2fs_update_dentry(ino: nid_t, mode: umode_t, d: *mut f2fs_dentry_ptr, name: *const fscrypt_str, hash: f2fs_hash_t, pos: c_uint) {
    let slots=GET_DENTRY_SLOTS((*name).len); let de=(*d).dentry.add(pos as usize); (*de).hash_code=hash; (*de).name_len=cpu_to_le16((*name).len); memcpy((*d).filename[pos as usize], (*name).name, (*name).len); (*de).ino=cpu_to_le32(ino); (*de).file_type=fs_umode_to_ftype(mode);
    for i in 0..slots { __set_bit_le(pos+i, (*d).bitmap as *mut c_void); if i != 0 { (*de.add(i as usize)).name_len=0; } }
}
pub unsafe fn f2fs_empty_dir(dir: *mut inode) -> bool { if f2fs_has_inline_dentry(dir) { return f2fs_empty_inline_dir(dir); } let mut b=0; while b<dir_blocks(dir) { let mut next=0; let p=f2fs_find_data_folio(dir,b,&mut next); if IS_ERR(p) { if PTR_ERR(p)==-ENOENT { b=next; continue; } return false; } let blk=folio_address(p); let from=if b==0 {2} else {0}; let bit=find_next_bit_le(&(*blk).dentry_bitmap,NR_DENTRY_IN_BLOCK,from); f2fs_folio_put(p,false); if bit<NR_DENTRY_IN_BLOCK { return false; } b+=1; } true }

#[repr(C)] pub struct file_operations { pub llseek: Option<unsafe extern "C" fn()>, pub read: Option<unsafe extern "C" fn()>, pub iterate_shared: Option<unsafe extern "C" fn()>, pub fsync: Option<unsafe extern "C" fn()>, pub unlocked_ioctl: Option<unsafe extern "C" fn()> }
extern "C" { fn f2fs_readdir(); fn generic_file_llseek(); fn generic_read_dir(); fn f2fs_sync_file(); fn f2fs_ioctl(); }
#[no_mangle] pub static f2fs_dir_operations: file_operations = file_operations { llseek: Some(generic_file_llseek), read: Some(generic_read_dir), iterate_shared: Some(f2fs_readdir), fsync: Some(f2fs_sync_file), unlocked_ioctl: Some(f2fs_ioctl) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
