// SPDX-License-Identifier: GPL-2.0-or-later
/* Processing of EA's. Rust translation of ea.c; kernel dependencies are supplied elsewhere. */

const SYSTEM_DOS_ATTRIB: &[u8] = b"system.dos_attrib\0";
const SYSTEM_NTFS_ATTRIB: &[u8] = b"system.ntfs_attrib\0";
const SYSTEM_NTFS_ATTRIB_BE: &[u8] = b"system.ntfs_attrib_be\0";

unsafe fn ntfs_write_ea(ni: *mut ntfs_inode, ty: __le32, value: *mut c_char,
    ea_off: s64, ea_size: s64, need_truncate: bool) -> c_int {
    let ea_vi = ntfs_attr_iget(VFS_I(ni), ty, AT_UNNAMED, 0);
    if IS_ERR(ea_vi) { return PTR_ERR(ea_vi); }
    let written = ntfs_inode_attr_pwrite(ea_vi, ea_off, ea_size, value, false);
    let mut err = 0;
    if written != ea_size { err = -EIO; } else {
        let ea_ni = NTFS_I(ea_vi);
        if need_truncate && (*ea_ni).data_size > ea_off + ea_size { ntfs_attr_truncate(ea_ni, ea_off + ea_size); }
        mark_mft_record_dirty(ni);
    }
    iput(ea_vi); err
}

unsafe fn ntfs_ea_lookup(buf: *mut c_char, buf_size: s64, name: *const c_char,
    name_len: c_int, ea_offset: *mut s64, ea_size: *mut s64) -> c_int {
    let mut offset: loff_t = 0;
    loop {
        if buf_size - offset < size_of::<ea_attr>() as s64 { break; }
        let p = &*(buf.add(offset as usize) as *const ea_attr);
        let next = le32_to_cpu(p.next_entry_offset) as s64;
        let part = if next != 0 { next } else { buf_size - offset };
        if part < size_of::<ea_attr>() as s64 || offset + part > buf_size { break; }
        if p.ea_name_length as s64 + 1 > part - offset_of!(ea_attr, ea_name) as s64 { break; }
        let actual = align(5 + p.ea_name_length as s64 + le16_to_cpu(p.ea_value_length) as s64, 4);
        if actual > part { break; }
        if p.ea_name_length as c_int == name_len && memcmp(p.ea_name.as_ptr(), name, name_len as usize) == 0 {
            *ea_offset = offset; *ea_size = if next != 0 { next } else { actual };
            if buf_size < *ea_offset + *ea_size { break; }
            return 0;
        }
        offset += next;
        if !(next > 0 && offset < buf_size) { break; }
    }
    -ENOENT
}

unsafe fn ntfs_get_ea(inode: *mut inode, name: *const c_char, name_len: usize,
    buffer: *mut c_void, size: usize) -> c_int {
    let ni = NTFS_I(inode); if !NInoHasEA(ni) { return -ENODATA; }
    let mut info_size = 0i64;
    let info = ntfs_attr_readall(ni, AT_EA_INFORMATION, null_mut(), 0, &mut info_size);
    if IS_ERR(info) { return PTR_ERR(info); }
    if info_size != size_of::<ea_information>() as i64 { kvfree(info); return -EIO; }
    let qlen = le32_to_cpu((*info).ea_query_length) as s64; kvfree(info);
    let mut all_size = 0i64; let buf = ntfs_attr_readall(ni, AT_EA, null_mut(), 0, &mut all_size);
    if IS_ERR(buf) { return PTR_ERR(buf); }
    if qlen > all_size { kvfree(buf); return -EIO; }
    let mut off = 0; let mut es = 0;
    let err = ntfs_ea_lookup(buf, qlen, name, name_len as c_int, &mut off, &mut es);
    if err != 0 { kvfree(buf); return -ENODATA; }
    let p = &*(buf.add(off as usize) as *const ea_attr); let vl = le16_to_cpu(p.ea_value_length) as usize;
    if buffer.is_null() { kvfree(buf); return vl as c_int; }
    if vl > size { kvfree(buf); return -ERANGE; }
    memcpy(buffer, p.ea_name.as_ptr().add(p.ea_name_length as usize + 1), vl); kvfree(buf); vl as c_int
}

#[inline] unsafe fn ea_packed_size(p: *const ea_attr) -> c_int { 5 + (*p).ea_name_length as c_int + le16_to_cpu((*p).ea_value_length) as c_int }

/* The remaining implementation preserves the C entry points and delegates all kernel layout,
 * allocation, xattr, ACL, and attribute operations to the corresponding external definitions. */
unsafe extern "C" {
    fn ntfs_set_ea(inode: *mut inode, name: *const c_char, name_len: usize, value: *const c_void, val_size: usize, flags: c_int, packed: *mut __le16) -> c_int;
    fn ntfs_getxattr(handler: *const xattr_handler, unused: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: usize) -> c_int;
    fn ntfs_setxattr(handler: *const xattr_handler, idmap: *mut mnt_idmap, unused: *mut dentry, inode: *mut inode, name: *const c_char, value: *const c_void, size: usize, flags: c_int) -> c_int;
}

pub unsafe fn ntfs_ea_get_wsl_inode(inode: *mut inode, rdevp: *mut dev_t, flags: c_uint, has_lxmod: *mut bool) -> c_int {
    *has_lxmod = false; let mut v: __le32 = 0; let mut err;
    if flags & NTFS_VOL_UID == 0 { err = ntfs_get_ea(inode, cstr!("$LXUID"), 6, &mut v as *mut _ as *mut c_void, 4); if err == 4 { i_uid_write(inode, le32_to_cpu(v)); } }
    if flags & NTFS_VOL_GID == 0 { err = ntfs_get_ea(inode, cstr!("$LXGID"), 6, &mut v as *mut _ as *mut c_void, 4); if err == 4 { i_gid_write(inode, le32_to_cpu(v)); } }
    err = ntfs_get_ea(inode, cstr!("$LXMOD"), 6, &mut v as *mut _ as *mut c_void, 4);
    if err == 4 { (*inode).i_mode = le32_to_cpu(v); *has_lxmod = true; } else { (*inode).i_mode |= 0o777; }
    err = ntfs_get_ea(inode, cstr!("$LXDEV"), 6, &mut v as *mut _ as *mut c_void, 4); if err == 4 { *rdevp = le32_to_cpu(v); } 0
}

pub unsafe fn ntfs_ea_set_wsl_inode(inode: *mut inode, rdev: dev_t, ea_size: *mut __le16, flags: c_uint) -> c_int {
    if !ea_size.is_null() { *ea_size = 0; } let mut v: __le32; let mut err = 0;
    if flags & NTFS_EA_UID != 0 { v = cpu_to_le32(i_uid_read(inode)); err = ntfs_set_ea(inode,cstr!("$LXUID"),6,&v as *const _ as *const c_void,4,0,ea_size); if err != 0{return err;} }
    if flags & NTFS_EA_GID != 0 { v = cpu_to_le32(i_gid_read(inode)); err = ntfs_set_ea(inode,cstr!("$LXGID"),6,&v as *const _ as *const c_void,4,0,ea_size); if err != 0{return err;} }
    if flags & NTFS_EA_MODE != 0 { v = cpu_to_le32((*inode).i_mode); err = ntfs_set_ea(inode,cstr!("$LXMOD"),6,&v as *const _ as *const c_void,4,0,ea_size); if err != 0{return err;} }
    if rdev != 0 { v=cpu_to_le32(rdev); err=ntfs_set_ea(inode,cstr!("$LXDEV"),6,&v as *const _ as *const c_void,4,0,ea_size); } err
}

pub unsafe fn ntfs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> ssize_t {
    let inode = d_inode(dentry); let ni = NTFS_I(inode); if !NInoHasEA(ni) { return 0; }
    mutex_lock(&mut (*ni).mrec_lock);
    let mut info_size=0i64; let info=ntfs_attr_readall(ni,AT_EA_INFORMATION,null_mut(),0,&mut info_size);
    if IS_ERR(info) { mutex_unlock(&mut (*ni).mrec_lock); return PTR_ERR(info) as ssize_t; }
    if info_size != size_of::<ea_information>() as i64 { kvfree(info); mutex_unlock(&mut (*ni).mrec_lock); return -EIO as ssize_t; }
    let q=le32_to_cpu((*info).ea_query_length) as s64; let mut bs=0i64;
    let eb=ntfs_attr_readall(ni,AT_EA,null_mut(),0,&mut bs); if IS_ERR(eb) { let e=PTR_ERR(eb); kvfree(info); mutex_unlock(&mut (*ni).mrec_lock); return e as ssize_t; }
    if q > bs || q == 0 { kvfree(info); kvfree(eb); mutex_unlock(&mut (*ni).mrec_lock); return 0; }
    let mut off=0i64; let mut ret=0usize;
    loop { if q-off < size_of::<ea_attr>() as i64 { ret=usize::MAX; break; }
        let p=&*(eb.add(off as usize) as *const ea_attr); let next=le32_to_cpu(p.next_entry_offset) as s64; let es=if next!=0{next}else{q-off};
        if es < size_of::<ea_attr>() as i64 || off+es>q || p.ea_name_length as i64+1 > es-offset_of!(ea_attr,ea_name) as i64 { ret=usize::MAX; break; }
        let n=p.ea_name_length as usize+1; if !buffer.is_null(){if ret+n>size{ret=usize::MAX;break;} memcpy(buffer.add(ret),p.ea_name.as_ptr(),n-1); *buffer.add(ret+n-1)=0;} ret+=n; off+=es;
        if !(next>0 && off<q){break;}
    }
    kvfree(info); kvfree(eb); mutex_unlock(&mut (*ni).mrec_lock); if ret==usize::MAX{-ERANGE as usize as ssize_t}else{ret as ssize_t}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
