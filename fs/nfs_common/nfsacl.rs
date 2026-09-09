// SPDX-License-Identifier: GPL-2.0-only
/*
 * fs/nfs_common/nfsacl.c
 *
 * Copyright (C) 2002-2003 Andreas Gruenbacher <agruen@suse.de>
 */

/* The Solaris nfsacl protocol represents some ACLs differently than POSIX. */

// C includes are supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct NfsaclEncodeDesc {
    pub desc: XdrArray2Desc,
    pub count: u32,
    pub acl: *mut PosixAcl,
    pub typeflag: i32,
    pub uid: Kuid,
    pub gid: Kgid,
}

#[repr(C)]
pub struct NfsaclSimpleAcl {
    pub acl: PosixAclHdr,
    pub ace: [PosixAclEntry; 4],
}

#[repr(C)]
pub struct NfsaclDecodeDesc {
    pub desc: XdrArray2Desc,
    pub count: u32,
    pub acl: *mut PosixAcl,
}

unsafe fn xdr_nfsace_encode(desc: *mut XdrArray2Desc, elem: *mut core::ffi::c_void) -> i32 {
    let nfsacl_desc = desc as *mut NfsaclEncodeDesc;
    let mut p = elem as *mut Be32;
    let entry = &mut (*(*nfsacl_desc).acl).a_entries[(*nfsacl_desc).count as usize];
    (*nfsacl_desc).count += 1;

    *p = htonl(entry.e_tag | (*nfsacl_desc).typeflag);
    p = p.add(1);
    match entry.e_tag {
        ACL_USER_OBJ => *p = htonl(from_kuid(&init_user_ns, (*nfsacl_desc).uid)),
        ACL_GROUP_OBJ => *p = htonl(from_kgid(&init_user_ns, (*nfsacl_desc).gid)),
        ACL_USER => *p = htonl(from_kuid(&init_user_ns, entry.e_uid)),
        ACL_GROUP => *p = htonl(from_kgid(&init_user_ns, entry.e_gid)),
        _ => *p = 0,
    }
    p = p.add(1);
    *p = htonl(entry.e_perm & S_IRWXO);
    0
}

pub unsafe fn nfsacl_encode(
    buf: *mut XdrBuf, base: u32, inode: *mut Inode, acl: *mut PosixAcl,
    encode_entries: i32, typeflag: i32,
) -> i32 {
    let entries = if !acl.is_null() && (*acl).a_count != 0 {
        core::cmp::max((*acl).a_count as i32, 4)
    } else { 0 };
    let mut nfsacl_desc = NfsaclEncodeDesc {
        desc: XdrArray2Desc { elem_size: 12, array_len: if encode_entries != 0 { entries as usize } else { 0 }, xcode: Some(xdr_nfsace_encode), array_maxlen: 0 },
        count: 0, acl, typeflag, uid: (*inode).i_uid, gid: (*inode).i_gid,
    };
    let mut aclbuf: NfsaclSimpleAcl = core::mem::zeroed();
    if entries > NFS_ACL_MAX_ENTRIES || xdr_encode_word(buf, base, entries as u32) != 0 { return -EINVAL; }
    if encode_entries != 0 && !acl.is_null() && (*acl).a_count == 3 {
        let acl2 = &mut aclbuf.acl as *mut PosixAclHdr as *mut PosixAcl;
        posix_acl_init(acl2, 4);
        (*acl2).a_entries[0] = (*acl).a_entries[0];
        (*acl2).a_entries[1] = (*acl).a_entries[1];
        (*acl2).a_entries[2] = (*acl).a_entries[1];
        (*acl2).a_entries[2].e_tag = ACL_MASK;
        (*acl2).a_entries[3] = (*acl).a_entries[2];
        nfsacl_desc.acl = acl2;
    }
    let err = xdr_encode_array2(buf, base + 4, &mut nfsacl_desc.desc);
    if err == 0 { 8 + (nfsacl_desc.desc.elem_size * nfsacl_desc.desc.array_len) as i32 } else { err }
}

pub unsafe fn nfs_stream_encode_acl(xdr: *mut XdrStream, inode: *mut Inode, acl: *mut PosixAcl, encode_entries: i32, typeflag: i32) -> bool {
    let elem_size = (XDR_UNIT * 3) as usize;
    let entries = if !acl.is_null() && (*acl).a_count != 0 { core::cmp::max((*acl).a_count as i32, 4) as u32 } else { 0 };
    let mut d = NfsaclEncodeDesc { desc: XdrArray2Desc { elem_size, array_len: if encode_entries != 0 { entries as usize } else { 0 }, xcode: Some(xdr_nfsace_encode), array_maxlen: 0 }, count: 0, acl, typeflag, uid: (*inode).i_uid, gid: (*inode).i_gid };
    let mut aclbuf: NfsaclSimpleAcl = core::mem::zeroed();
    if entries > NFS_ACL_MAX_ENTRIES || xdr_stream_encode_u32(xdr, entries) < 0 { return false; }
    if encode_entries != 0 && !acl.is_null() && (*acl).a_count == 3 {
        let a = &mut aclbuf.acl as *mut PosixAclHdr as *mut PosixAcl; posix_acl_init(a, 4);
        (*a).a_entries[0] = (*acl).a_entries[0]; (*a).a_entries[1] = (*acl).a_entries[1];
        (*a).a_entries[2] = (*acl).a_entries[1]; (*a).a_entries[2].e_tag = ACL_MASK; (*a).a_entries[3] = (*acl).a_entries[2]; d.acl = a;
    }
    let base = xdr_stream_pos(xdr); if !xdr_reserve_space(xdr, XDR_UNIT + elem_size * d.desc.array_len) { return false; }
    xdr_encode_array2((*xdr).buf, base, &mut d.desc) == 0
}

unsafe fn xdr_nfsace_decode(desc: *mut XdrArray2Desc, elem: *mut core::ffi::c_void) -> i32 {
    let d = desc as *mut NfsaclDecodeDesc; let p = elem as *mut Be32;
    if (*d).acl.is_null() { if (*desc).array_len > NFS_ACL_MAX_ENTRIES { return -EINVAL; } (*d).acl = posix_acl_alloc((*desc).array_len, GFP_KERNEL); if (*d).acl.is_null() { return -ENOMEM; } (*d).count = 0; }
    let e = &mut (*(*d).acl).a_entries[(*d).count as usize]; (*d).count += 1;
    e.e_tag = ntohl(*p) & !NFS_ACL_DEFAULT; let id = ntohl(*p.add(1)); e.e_perm = ntohl(*p.add(2));
    match e.e_tag { ACL_USER => { e.e_uid = make_kuid(&init_user_ns, id); if !uid_valid(e.e_uid) { return -EINVAL; } }, ACL_GROUP => { e.e_gid = make_kgid(&init_user_ns, id); if !gid_valid(e.e_gid) { return -EINVAL; } }, ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_OTHER => if e.e_perm & !S_IRWXO != 0 { return -EINVAL; }, ACL_MASK => e.e_perm &= S_IRWXO, _ => return -EINVAL }
    0
}

unsafe fn cmp_acl_entry(x: *const core::ffi::c_void, y: *const core::ffi::c_void) -> i32 {
    let a = &*(x as *const PosixAclEntry); let b = &*(y as *const PosixAclEntry);
    if a.e_tag != b.e_tag { return a.e_tag - b.e_tag; }
    if a.e_tag == ACL_USER { if uid_gt(a.e_uid,b.e_uid) { 1 } else if uid_lt(a.e_uid,b.e_uid) { -1 } else { 0 } }
    else if a.e_tag == ACL_GROUP { if gid_gt(a.e_gid,b.e_gid) { 1 } else if gid_lt(a.e_gid,b.e_gid) { -1 } else { 0 } } else { 0 }
}

unsafe fn posix_acl_from_nfsacl(acl: *mut PosixAcl) -> i32 {
    if acl.is_null() { return 0; }
    sort((*acl).a_entries.as_mut_ptr() as *mut core::ffi::c_void, (*acl).a_count as usize, core::mem::size_of::<PosixAclEntry>(), Some(cmp_acl_entry), core::ptr::null_mut());
    let mut group_obj: *mut PosixAclEntry = core::ptr::null_mut(); let mut mask: *mut PosixAclEntry = core::ptr::null_mut();
    for i in 0..(*acl).a_count as usize { let p = &mut (*acl).a_entries[i]; match p.e_tag { ACL_GROUP_OBJ => group_obj=p, ACL_MASK => mask=p, _ => {} } }
    if (*acl).a_count == 4 && !group_obj.is_null() && !mask.is_null() && (*mask).e_perm == (*group_obj).e_perm { let off = mask.offset_from((*acl).a_entries.as_mut_ptr()); core::ptr::copy(mask.add(1), mask, (3 - off) as usize); (*acl).a_count=3; }
    0
}

pub unsafe fn nfsacl_decode(buf: *mut XdrBuf, base: u32, aclcnt: *mut u32, pacl: *mut *mut PosixAcl) -> i32 {
    let mut d: NfsaclDecodeDesc = core::mem::zeroed(); d.desc.elem_size=12; d.desc.xcode=if !pacl.is_null() { Some(xdr_nfsace_decode) } else { None };
    let mut entries=0u32; if xdr_decode_word(buf,base,&mut entries)!=0 || entries>NFS_ACL_MAX_ENTRIES { return -EINVAL; } d.desc.array_maxlen=entries as usize;
    let err=xdr_decode_array2(buf,base+4,&mut d.desc); if err!=0{return err;} if !pacl.is_null() { if entries as usize != d.desc.array_len || posix_acl_from_nfsacl(d.acl)!=0 { posix_acl_release(d.acl); return -EINVAL; } *pacl=d.acl; } if !aclcnt.is_null(){*aclcnt=entries;} 8+(d.desc.elem_size*d.desc.array_len) as i32
}

pub unsafe fn nfs_stream_decode_acl(xdr: *mut XdrStream, aclcnt: *mut u32, pacl: *mut *mut PosixAcl) -> bool {
    let elem_size=(XDR_UNIT*3) as usize; let mut d:NfsaclDecodeDesc=core::mem::zeroed(); d.desc.elem_size=elem_size; d.desc.xcode=if !pacl.is_null(){Some(xdr_nfsace_decode)}else{None}; let mut entries=0u32;
    if xdr_stream_decode_u32(xdr,&mut entries)<0 || entries>NFS_ACL_MAX_ENTRIES{return false;} let base=xdr_stream_pos(xdr); if !xdr_inline_decode(xdr,XDR_UNIT+elem_size*entries as usize){return false;} d.desc.array_maxlen=entries as usize; if xdr_decode_array2((*xdr).buf,base,&mut d.desc)!=0{return false;}
    if !pacl.is_null(){if entries as usize != d.desc.array_len || posix_acl_from_nfsacl(d.acl)!=0{posix_acl_release(d.acl);return false;}*pacl=d.acl;} if !aclcnt.is_null(){*aclcnt=entries;} true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
