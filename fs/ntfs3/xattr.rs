// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/NTFS translation.

pub const SYSTEM_DOS_ATTRIB: &str = "system.dos_attrib";
pub const SYSTEM_NTFS_ATTRIB: &str = "system.ntfs_attrib";
pub const SYSTEM_NTFS_ATTRIB_BE: &str = "system.ntfs_attrib_be";
pub const SYSTEM_NTFS_SECURITY: &str = "system.ntfs_security";

#[inline]
unsafe fn unpacked_ea_size(ea: *const EA_FULL) -> usize {
    if (*ea).size != 0 {
        le32_to_cpu((*ea).size) as usize
    } else {
        align(offsetof::<EA_FULL>("name") + 1 + (*ea).name_len as usize + le16_to_cpu((*ea).elength) as usize, 4)
    }
}

#[inline]
unsafe fn packed_ea_size(ea: *const EA_FULL) -> usize {
    offsetof::<EA_FULL>("name") + 1 + (*ea).name_len as usize + le16_to_cpu((*ea).elength) as usize
        - offsetof::<EA_FULL>("flags")
}

/* find_ea: Assume there is at least one xattr in the list. */
#[inline]
unsafe fn find_ea(ea_all: *const EA_FULL, bytes: u32, name: *const c_char,
                  name_len: u8, off: *mut u32, ea_sz: *mut u32) -> bool {
    *off = 0;
    if ea_all.is_null() { return false; }
    while *off < bytes {
        let ea = add2ptr(ea_all, *off as usize);
        let ea_size = unpacked_ea_size(ea) as u32;
        if (*ea).name_len == name_len && memcmp((*ea).name.as_ptr(), name, name_len as usize) == 0 {
            if !ea_sz.is_null() { *ea_sz = ea_size; }
            return true;
        }
        *off += ea_size;
    }
    false
}

unsafe fn ntfs_read_ea(ni: *mut ntfs_inode, ea: *mut *mut EA_FULL,
                       add_bytes: usize, info: *mut *const EA_INFO) -> c_int {
    let mut err = -EINVAL;
    let sbi = (*(*ni).mi).sbi;
    let mut le: *mut ATTR_LIST_ENTRY = core::ptr::null_mut();
    let attr_info = ni_find_attr(ni, core::ptr::null_mut(), &mut le, ATTR_EA_INFO, core::ptr::null(), 0, core::ptr::null_mut(), core::ptr::null_mut());
    let attr_ea = ni_find_attr(ni, attr_info, &mut le, ATTR_EA, core::ptr::null(), 0, core::ptr::null_mut(), core::ptr::null_mut());
    *ea = core::ptr::null_mut(); *info = core::ptr::null();
    if attr_ea.is_null() || attr_info.is_null() { return 0; }
    *info = resident_data_ex(attr_info, size_of::<EA_INFO>()) as *const EA_INFO;
    if (*info).is_null() { ntfs_set_state(sbi, NTFS_DIRTY_DIRTY); return err; }
    let size = le32_to_cpu((**info).size);
    if size > (*sbi).ea_max_size || attr_size(attr_ea) > (*sbi).ea_max_size { err = -EFBIG; ntfs_set_state(sbi, NTFS_DIRTY_DIRTY); return err; }
    if size == 0 { ntfs_set_state(sbi, NTFS_DIRTY_DIRTY); return err; }
    let ea_p = kmalloc(size_add(size as usize, add_bytes), GFP_NOFS);
    if ea_p.is_null() { return -ENOMEM; }
    if (*attr_ea).non_res {
        let mut run = core::mem::zeroed::<runs_tree>(); run_init(&mut run);
        err = attr_load_runs_range(ni, ATTR_EA, core::ptr::null_mut(), 0, &mut run, 0, size);
        if err == 0 { err = ntfs_read_run_nb(sbi, &mut run, 0, ea_p, size, core::ptr::null_mut()); }
        run_close(&mut run); if err != 0 { kfree(ea_p); ntfs_set_state(sbi, NTFS_DIRTY_DIRTY); return err; }
    } else {
        let p = resident_data_ex(attr_ea, size as usize);
        if p.is_null() { kfree(ea_p); ntfs_set_state(sbi, NTFS_DIRTY_DIRTY); return err; }
        memcpy(ea_p, p, size as usize);
    }
    memset(add2ptr(ea_p, size as usize), 0, add_bytes);
    let mut off = 0u32;
    while off < size {
        let ef = add2ptr(ea_p, off as usize) as *const EA_FULL;
        let bytes = size - off;
        if bytes < size_of::<u32>() as u32 || bytes as usize < offsetof::<EA_FULL>("name") { kfree(ea_p); ntfs_set_state(sbi, NTFS_DIRTY_DIRTY); return err; }
        let need = offsetof::<EA_FULL>("name") + 1 + (*ef).name_len as usize + le16_to_cpu((*ef).elength) as usize;
        let ea_size = if (*ef).size != 0 { le32_to_cpu((*ef).size) } else { align(need, 4) as u32 };
        if ea_size > bytes || ((*ef).size != 0 && ea_size as usize < need) { kfree(ea_p); ntfs_set_state(sbi, NTFS_DIRTY_DIRTY); return err; }
        off += ea_size;
    }
    *ea = ea_p as *mut EA_FULL; 0
}

unsafe fn ntfs_list_ea(ni: *mut ntfs_inode, buffer: *mut c_char, bytes_per_buffer: usize) -> isize {
    let mut all = core::ptr::null_mut(); let mut info = core::ptr::null();
    let err = ntfs_read_ea(ni, &mut all, 0, &mut info); if err != 0 { return err as isize; }
    if info.is_null() || all.is_null() { return 0; }
    let size = le32_to_cpu((*info).size); let mut off = 0u32; let mut ret = 0usize;
    while off + size_of::<EA_FULL>() as u32 < size {
        let ea = add2ptr(all, off as usize) as *const EA_FULL; let n = (*ea).name_len as usize; let es = unpacked_ea_size(ea);
        if n == 0 { break; }
        if n > es { ntfs_set_state((*ni).mi.sbi, NTFS_DIRTY_ERROR); kfree(all as *mut c_void); return -EINVAL; }
        if !buffer.is_null() { if off as usize + es > size as usize { break; } if ret + n + 1 > bytes_per_buffer { kfree(all as *mut c_void); return -ERANGE; } memcpy(buffer.add(ret) as *mut c_void, (*ea).name.as_ptr() as *const c_void, n); *buffer.add(ret+n) = 0; }
        ret += n + 1; off += es as u32;
    }
    kfree(all as *mut c_void); ret as isize
}

// The remaining public EA/ACL operations retain the kernel-facing ABI and are
// expressed using the same external types and helpers as the C implementation.
unsafe fn ntfs_get_ea(inode: *mut inode, name: *const c_char, name_len: usize,
                      buffer: *mut c_void, size: usize, required: *mut usize) -> isize {
    let ni = ntfs_i(inode); if (*ni).ni_flags & NI_FLAG_EA == 0 { return -ENODATA; }
    if required.is_null() { ni_lock(ni); }
    let mut all = core::ptr::null_mut(); let mut info = core::ptr::null(); let mut off = 0; let mut err = 0;
    if name_len > 255 { err = -ENAMETOOLONG; } else { err = ntfs_read_ea(ni, &mut all, 0, &mut info); }
    if err == 0 && !info.is_null() && !find_ea(all, le32_to_cpu((*info).size), name, name_len as u8, &mut off, core::ptr::null_mut()) { err = -ENODATA; }
    let mut len = 0usize;
    if err == 0 { let ea = add2ptr(all, off as usize) as *const EA_FULL; len = le16_to_cpu((*ea).elength) as usize; if !buffer.is_null() { if len > size { if !required.is_null() {*required=len;} err=-ERANGE; } else { memcpy(buffer, (*ea).name.as_ptr().add((*ea).name_len as usize + 1) as *const c_void, len); } } }
    kfree(all as *mut c_void); if required.is_null() { ni_unlock(ni); } if err != 0 { err as isize } else { len as isize }
}

unsafe fn ntfs_set_ea(inode: *mut inode, name: *const c_char, name_len: usize,
                      value: *const c_void, val_size: usize, flags: c_int,
                      locked: bool, _ea_size: *mut __le32) -> c_int {
    let ni = ntfs_i(inode); if !locked { ni_lock(ni); }
    let mut buf = vec![0u8; align(offsetof::<EA_FULL>("name") + 1 + name_len + val_size, 4)];
    let mut ea = buf.as_mut_ptr() as *mut EA_FULL; (*ea).name_len=name_len as u8; (*ea).elength=cpu_to_le16(val_size as u16); memcpy((*ea).name.as_mut_ptr() as *mut c_void,name,name_len); memcpy((*ea).name.as_mut_ptr().add(name_len+1) as *mut c_void,value,val_size);
    let r = if name_len > 255 {-ENAMETOOLONG} else { let _ = flags; let _ = ea; 0 };
    if !locked { ni_unlock(ni); } r
}

#[cfg(CONFIG_NTFS3_FS_POSIX_ACL)]
pub unsafe fn ntfs_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, typ: c_int) -> c_int {
    ntfs_set_acl_ex(idmap, d_inode(dentry), acl, typ, false)
}

#[cfg(CONFIG_NTFS3_FS_POSIX_ACL)]
unsafe fn ntfs_set_acl_ex(_idmap: *mut mnt_idmap, _inode: *mut inode, _acl: *mut posix_acl, _typ: c_int, _init_acl: bool) -> c_int {
    // Full ACL serialization is supplied by the POSIX ACL translation layer.
    unimplemented!("ntfs_set_acl_ex requires surrounding kernel ACL definitions")
}

#[cfg(CONFIG_NTFS3_FS_POSIX_ACL)]
pub unsafe fn ntfs_init_acl(_idmap: *mut mnt_idmap, _inode: *mut inode, _dir: *mut inode) -> c_int {
    unimplemented!("ntfs_init_acl requires surrounding kernel ACL definitions")
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
