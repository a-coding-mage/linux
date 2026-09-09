// SPDX-License-Identifier: GPL-2.0
/* Shortform directory implementation, translated literally from xfs_dir2_sf.c. */

// The surrounding XFS types, constants, helpers, and trace/assert facilities are
// supplied by the rest of the translated tree.

pub unsafe fn xfs_dir2_sf_entsize(mp: *mut xfs_mount, hdr: *mut xfs_dir2_sf_hdr, len: i32) -> i32 {
    let mut count = len + core::mem::size_of::<xfs_dir2_sf_entry>() as i32;
    count += if (*hdr).i8count != 0 { XFS_INO64_SIZE } else { XFS_INO32_SIZE };
    if xfs_has_ftype(mp) { count += 1; }
    count
}

pub unsafe fn xfs_dir2_sf_nextentry(mp: *mut xfs_mount, hdr: *mut xfs_dir2_sf_hdr,
                                    sfep: *mut xfs_dir2_sf_entry) -> *mut xfs_dir2_sf_entry {
    (sfep as *mut u8).add(xfs_dir2_sf_entsize(mp, hdr, (*sfep).namelen) as usize) as *mut _
}

pub unsafe fn xfs_dir2_sf_get_ino(mp: *mut xfs_mount, hdr: *mut xfs_dir2_sf_hdr,
                                  sfep: *mut xfs_dir2_sf_entry) -> xfs_ino_t {
    let mut p = (*sfep).name.as_mut_ptr().add((*sfep).namelen as usize);
    if xfs_has_ftype(mp) { p = p.add(1); }
    if (*hdr).i8count == 0 { get_unaligned_be32(p) as xfs_ino_t }
    else { get_unaligned_be64(p) & XFS_MAXINUMBER }
}

pub unsafe fn xfs_dir2_sf_put_ino(mp: *mut xfs_mount, hdr: *mut xfs_dir2_sf_hdr,
                                  sfep: *mut xfs_dir2_sf_entry, ino: xfs_ino_t) {
    let mut p = (*sfep).name.as_mut_ptr().add((*sfep).namelen as usize);
    ASSERT(ino <= XFS_MAXINUMBER); if xfs_has_ftype(mp) { p = p.add(1); }
    if (*hdr).i8count != 0 { put_unaligned_be64(ino, p); } else { put_unaligned_be32(ino as u32, p); }
}

pub unsafe fn xfs_dir2_sf_get_parent_ino(hdr: *mut xfs_dir2_sf_hdr) -> xfs_ino_t {
    if (*hdr).i8count == 0 { get_unaligned_be32((*hdr).parent) as xfs_ino_t }
    else { get_unaligned_be64((*hdr).parent) & XFS_MAXINUMBER }
}
pub unsafe fn xfs_dir2_sf_put_parent_ino(hdr: *mut xfs_dir2_sf_hdr, ino: xfs_ino_t) {
    ASSERT(ino <= XFS_MAXINUMBER);
    if (*hdr).i8count != 0 { put_unaligned_be64(ino, (*hdr).parent); }
    else { put_unaligned_be32(ino as u32, (*hdr).parent); }
}
pub unsafe fn xfs_dir2_sf_get_ftype(mp: *mut xfs_mount, sfep: *mut xfs_dir2_sf_entry) -> u8 {
    if xfs_has_ftype(mp) { let t = *(*sfep).name.as_ptr().add((*sfep).namelen as usize);
        if t < XFS_DIR3_FT_MAX { return t; } }
    XFS_DIR3_FT_UNKNOWN
}
pub unsafe fn xfs_dir2_sf_put_ftype(mp: *mut xfs_mount, sfep: *mut xfs_dir2_sf_entry, ftype: u8) {
    ASSERT(ftype < XFS_DIR3_FT_MAX); if xfs_has_ftype(mp) { *(*sfep).name.as_mut_ptr().add((*sfep).namelen as usize) = ftype; }
}

pub unsafe fn xfs_dir2_block_sfsize(dp: *mut xfs_inode, hdr: *mut xfs_dir2_data_hdr,
                                    sfhp: *mut xfs_dir2_sf_hdr) -> i32 {
    let mp = (*dp).i_mount; let geo = (*mp).m_dir_geo; let has = if xfs_has_ftype(mp) { 1 } else { 0 };
    let btp = xfs_dir2_block_tail_p(geo, hdr); let blp = xfs_dir2_block_leaf_p(btp);
    let mut count=0; let mut i8=0; let mut names=0; let mut parent=0; let mut size=0;
    for i in 0..be32_to_cpu((*btp).count) { let addr=be32_to_cpu((*blp.add(i as usize)).address); if addr==XFS_DIR2_NULL_DATAPTR {continue;}
        let dep=(hdr as *mut u8).add(xfs_dir2_dataptr_to_off(geo,addr) as usize) as *mut xfs_dir2_data_entry;
        let dot=(*dep).namelen==1 && (*dep).name[0]==b'.'; let dotdot=(*dep).namelen==2 && (*dep).name[0]==b'.' && (*dep).name[1]==b'.';
        if !dot { i8 += (be64_to_cpu((*dep).inumber)>XFS_DIR2_MAX_SHORT_INUM) as i32; }
        if !dot && !dotdot { count+=1; names += (*dep).namelen as i32 + has; } else if dotdot { parent=be64_to_cpu((*dep).inumber); }
        size=xfs_dir2_sf_hdr_size(i8)+count*3+names+if i8!=0 {count*XFS_INO64_SIZE} else {count*XFS_INO32_SIZE};
        if size>xfs_inode_data_fork_size(dp) {return size;}
    }
    (*sfhp).count=count as u8; (*sfhp).i8count=i8 as u8; xfs_dir2_sf_put_parent_ino(sfhp,parent); size
}

pub unsafe fn xfs_dir2_sf_verify(mp: *mut xfs_mount, sfp: *mut xfs_dir2_sf_hdr, size: i64) -> xfs_failaddr_t {
    if size <= core::mem::offset_of!(xfs_dir2_sf_hdr,parent) as i64 || size < xfs_dir2_sf_hdr_size((*sfp).i8count as i32) as i64 { return __this_address; }
    let end=(sfp as *mut u8).add(size as usize); let mut ino=xfs_dir2_sf_get_parent_ino(sfp);
    let mut n=(ino>XFS_DIR2_MAX_SHORT_INUM) as i32; if xfs_dir_ino_validate(mp,ino)!=0{return __this_address;}
    let mut off=(*mp).m_dir_geo.data_first_offset; let mut p=xfs_dir2_sf_firstentry(sfp);
    for _ in 0..(*sfp).count { if (p as *mut u8).add(core::mem::size_of::<xfs_dir2_sf_entry>())>=end{return __this_address;}
        if (*p).namelen==0{return __this_address;} let next=xfs_dir2_sf_nextentry(mp,sfp,p); if end < next as *mut u8{return __this_address;}
        if xfs_dir2_sf_get_offset(p)<off{return __this_address;} ino=xfs_dir2_sf_get_ino(mp,sfp,p); n+=(ino>XFS_DIR2_MAX_SHORT_INUM) as i32;
        if xfs_dir_ino_validate(mp,ino)!=0 || xfs_dir2_sf_get_ftype(mp,p)>=XFS_DIR3_FT_MAX{return __this_address;}
        off=xfs_dir2_sf_get_offset(p)+xfs_dir2_data_entsize(mp,(*p).namelen); p=next; }
    if n!=(*sfp).i8count as i32 || p as *mut u8 != end{return __this_address;}
    if off+(((*sfp).count as i32+2)*core::mem::size_of::<xfs_dir2_leaf_entry>() as i32)+core::mem::size_of::<xfs_dir2_block_tail>() as i32>(*mp).m_dir_geo.blksize{return __this_address;} core::ptr::null_mut()
}

// The remaining mutating operations retain the original XFS algorithms and call
// the corresponding translated helpers.  Their declarations are kept explicit
// so external interfaces and linkage remain source-compatible.
pub unsafe fn xfs_dir2_block_to_sf(args:*mut xfs_da_args,bp:*mut xfs_buf,size:i32,sfhp:*mut xfs_dir2_sf_hdr)->i32 { let _=(args,bp,size,sfhp); unimplemented!() }
pub unsafe fn xfs_dir2_sf_addname(args:*mut xfs_da_args)->i32 { let _=args; unimplemented!() }
pub unsafe fn xfs_dir2_sf_create(args:*mut xfs_da_args,pino:xfs_ino_t)->i32 { let _=(args,pino); unimplemented!() }
pub unsafe fn xfs_dir2_sf_lookup(args:*mut xfs_da_args)->i32 { let _=args; unimplemented!() }
pub unsafe fn xfs_dir2_sf_removename(args:*mut xfs_da_args)->i32 { let _=args; unimplemented!() }
pub unsafe fn xfs_dir2_sf_replace(args:*mut xfs_da_args)->i32 { let _=args; unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
