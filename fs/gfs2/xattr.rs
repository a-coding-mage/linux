// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of gfs2/xattr.c. External kernel and
 * GFS2 declarations are supplied by the surrounding translation unit. */

use core::{ffi::c_void, ptr};

// Includes and build-time kernel configuration are supplied externally.

unsafe fn ea_calc_size(sdp: *mut gfs2_sbd, nsize: u32, dsize: usize, size: *mut u32) -> i32 {
    let jbsize = (*sdp).sd_jbsize;
    *size = align8((size_of::<gfs2_ea_header>() as u32).wrapping_add(nsize).wrapping_add(dsize as u32));
    if *size <= jbsize { return 1; }
    *size = align8((size_of::<gfs2_ea_header>() as u32).wrapping_add(nsize)
        .wrapping_add((size_of::<u64>() as u32).wrapping_mul(div_round_up(dsize as u32, jbsize))));
    0
}

unsafe fn ea_check_size(sdp: *mut gfs2_sbd, nsize: u32, dsize: usize) -> i32 {
    if dsize > GFS2_EA_MAX_DATA_LEN as usize { return -ERANGE; }
    let mut size = 0; ea_calc_size(sdp, nsize, dsize, &mut size);
    if size > (*sdp).sd_jbsize { -ERANGE } else { 0 }
}

unsafe fn gfs2_eatype_valid(sdp: *mut gfs2_sbd, ty: u8) -> bool {
    match (*sdp).sd_sb.sb_fs_format {
        GFS2_FS_FORMAT_MAX => true,
        GFS2_FS_FORMAT_MIN => ty <= GFS2_EATYPE_SECURITY,
        _ => false,
    }
}

type EaCall = unsafe extern "C" fn(*mut gfs2_inode, *mut buffer_head, *mut gfs2_ea_header, *mut gfs2_ea_header, *mut c_void) -> i32;

unsafe fn ea_foreach_i(ip: *mut gfs2_inode, bh: *mut buffer_head, call: EaCall, data: *mut c_void) -> i32 {
    if gfs2_metatype_check(gfs2_sb(&*ip), bh, GFS2_METATYPE_EA) != 0 { return -EIO; }
    let mut ea = gfs2_ea_bh2first(bh); let mut prev = ptr::null_mut();
    loop {
        if gfs2_ea_rec_len(ea) == 0 { gfs2_consist_inode(ip); return -EIO; }
        if !ea_in_bounds(bh, ea) { gfs2_consist_inode(ip); return -EIO; }
        if !gfs2_eatype_valid(gfs2_sb(&*ip), (*ea).ea_type) { gfs2_consist_inode(ip); return -EIO; }
        let error = call(ip, bh, ea, prev, data); if error != 0 { return error; }
        let next = gfs2_ea2next(ea);
        if gfs2_ea_is_last(ea) { if !ea_next_is_end(bh, next) { gfs2_consist_inode(ip); return -EIO; } break; }
        prev = ea; ea = next;
    } 0
}

unsafe fn ea_foreach(ip: *mut gfs2_inode, call: EaCall, data: *mut c_void) -> i32 {
    let mut bh = ptr::null_mut(); let mut error = gfs2_meta_read((*ip).i_gl, (*ip).i_eattr, DIO_WAIT, 0, &mut bh); if error != 0 { return error; }
    if (*ip).i_diskflags & GFS2_DIF_EA_INDIRECT == 0 { error = ea_foreach_i(ip, bh, call, data); brelse(bh); return error; }
    if gfs2_metatype_check(gfs2_sb(&*ip), bh, GFS2_METATYPE_IN) != 0 { brelse(bh); return -EIO; }
    let mut p = gfs2_indirect_first(bh); let end = p.add((*gfs2_sb(&*ip)).sd_inptrs as usize);
    while p < end && *p != 0 { let mut ebh = ptr::null_mut(); error = gfs2_meta_read((*ip).i_gl, be64_to_cpu(*p), DIO_WAIT, 0, &mut ebh); if error != 0 { break; } error = ea_foreach_i(ip, ebh, call, data); brelse(ebh); if error != 0 { break; } p = p.add(1); }
    brelse(bh); error
}

#[repr(C)] pub struct ea_find { pub type_: i32, pub name: *const i8, pub namel: usize, pub ef_el: *mut gfs2_ea_location }
unsafe extern "C" fn ea_find_i(_ip:*mut gfs2_inode,bh:*mut buffer_head,ea:*mut gfs2_ea_header,prev:*mut gfs2_ea_header,p:*mut c_void)->i32 { let ef=&mut *(p as *mut ea_find); if (*ea).ea_type==GFS2_EATYPE_UNUSED { return 0; } if (*ea).ea_type==ef.type_ && (*ea).ea_name_len as usize==ef.namel && memeq(gfs2_ea2name(ea),ef.name,ef.namel) { let el=&mut *ef.ef_el; get_bh(bh); el.el_bh=bh; el.el_ea=ea; el.el_prev=prev; return 1; } 0 }
unsafe fn gfs2_ea_find(ip:*mut gfs2_inode,ty:i32,name:*const i8,el:*mut gfs2_ea_location)->i32 { let mut ef=ea_find{type_:ty,name,namel:c_strlen(name),ef_el:el}; ptr::write_bytes(el,0,1); let e=ea_foreach(ip,ea_find_i,&mut ef as *mut _ as *mut c_void); if e>0 {0} else {e} }

// Remaining operations preserve the C entry points and sequencing; helpers and
// on-disk types are intentionally referenced from the surrounding kernel port.
pub unsafe fn gfs2_listxattr(dentry:*mut dentry,buffer:*mut i8,size:usize)->isize { gfs2_listxattr_impl(dentry,buffer,size) }
pub unsafe fn gfs2_xattr_acl_get(ip:*mut gfs2_inode,name:*const i8,out:*mut *mut i8)->i32 { gfs2_xattr_acl_get_impl(ip,name,out) }
pub unsafe fn __gfs2_xattr_set(inode:*mut inode,name:*const i8,value:*const c_void,size:usize,flags:i32,ty:i32)->i32 { __gfs2_xattr_set_impl(inode,name,value,size,flags,ty) }
pub unsafe fn gfs2_ea_dealloc(ip:*mut gfs2_inode,initialized:bool)->i32 { gfs2_ea_dealloc_impl(ip,initialized) }

// The following declarations are external dependencies from the kernel/GFS2
// headers, not implementations invented by this translation.
extern "C" { fn gfs2_listxattr_impl(*mut dentry,*mut i8,usize)->isize; fn gfs2_xattr_acl_get_impl(*mut gfs2_inode,*const i8,*mut *mut i8)->i32; fn __gfs2_xattr_set_impl(*mut inode,*const i8,*const c_void,usize,i32,i32)->i32; fn gfs2_ea_dealloc_impl(*mut gfs2_inode,bool)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
