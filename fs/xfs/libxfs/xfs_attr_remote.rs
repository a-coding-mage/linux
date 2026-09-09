// SPDX-License-Identifier: GPL-2.0
/* Rust translation of xfs_attr_remote.c. External types and functions are
 * supplied by the surrounding XFS translation. */

const ATTR_RMTVALUE_MAPSIZE: usize = 1;

#[inline]
pub unsafe fn xfs_attr3_rmt_buf_space(mp: *mut xfs_mount) -> u32 {
    let blocksize = (*(*mp).m_attr_geo).blksize;
    if xfs_has_crc(mp) { blocksize - core::mem::size_of::<xfs_attr3_rmt_hdr>() as u32 } else { blocksize }
}

pub unsafe fn xfs_attr3_rmt_blocks(mp: *mut xfs_mount, attrlen: u32) -> u32 {
    if xfs_has_crc(mp) {
        howmany(attrlen, xfs_attr3_rmt_buf_space(mp))
    } else { XFS_B_TO_FSB(mp, attrlen) }
}

unsafe fn xfs_attr3_rmt_hdr_ok(ptr: *mut core::ffi::c_void, ino: xfs_ino_t, offset: u32, size: u32, bno: xfs_daddr_t) -> xfs_failaddr_t {
    let rmt = ptr as *mut xfs_attr3_rmt_hdr;
    if bno != be64_to_cpu((*rmt).rm_blkno) || offset != be32_to_cpu((*rmt).rm_offset) ||
       size != be32_to_cpu((*rmt).rm_bytes) || ino != be64_to_cpu((*rmt).rm_owner) { __this_address!() } else { core::ptr::null_mut() }
}

unsafe fn xfs_attr3_rmt_verify(mp: *mut xfs_mount, bp: *mut xfs_buf, ptr: *mut core::ffi::c_void, bno: xfs_daddr_t) -> xfs_failaddr_t {
    let rmt = ptr as *mut xfs_attr3_rmt_hdr;
    if !xfs_verify_magic(bp, (*rmt).rm_magic) || !uuid_equal(&(*rmt).rm_uuid, &(*mp).m_sb.sb_meta_uuid) ||
       be64_to_cpu((*rmt).rm_blkno) != bno || be32_to_cpu((*rmt).rm_bytes) > (*(*mp).m_attr_geo).blksize - core::mem::size_of::<xfs_attr3_rmt_hdr>() as u32 ||
       be32_to_cpu((*rmt).rm_offset) + be32_to_cpu((*rmt).rm_bytes) > XFS_XATTR_SIZE_MAX || (*rmt).rm_owner == 0 { __this_address!() } else { core::ptr::null_mut() }
}

unsafe fn __xfs_attr3_rmt_read_verify(bp: *mut xfs_buf, check_crc: bool, failaddr: *mut xfs_failaddr_t) -> i32 {
    let mp = (*bp).b_mount; let blksize = (*(*mp).m_attr_geo).blksize;
    if !xfs_has_crc(mp) { return 0; }
    let mut ptr = (*bp).b_addr as *mut u8; let mut bno = xfs_buf_daddr(bp); let mut len = BBTOB((*bp).b_length);
    ASSERT(len >= blksize);
    while len > 0 {
        if check_crc && !xfs_verify_cksum(ptr as *mut _, blksize, XFS_ATTR3_RMT_CRC_OFF) { *failaddr = __this_address!(); return -EFSBADCRC; }
        *failaddr = xfs_attr3_rmt_verify(mp, bp, ptr as *mut _, bno);
        if !(*failaddr).is_null() { return -EFSCORRUPTED; }
        len -= blksize; ptr = ptr.add(blksize as usize); bno += BTOBB(blksize);
    }
    if len != 0 { *failaddr = __this_address!(); return -EFSCORRUPTED; } 0
}

unsafe fn xfs_attr3_rmt_read_verify(bp: *mut xfs_buf) { let mut fa = core::ptr::null_mut(); let e = __xfs_attr3_rmt_read_verify(bp, true, &mut fa); if e != 0 { xfs_verifier_error(bp, e, fa); } }
unsafe fn xfs_attr3_rmt_verify_struct(bp: *mut xfs_buf) -> xfs_failaddr_t { let mut fa = core::ptr::null_mut(); let e = __xfs_attr3_rmt_read_verify(bp, false, &mut fa); if e != 0 { fa } else { core::ptr::null_mut() } }

unsafe fn xfs_attr3_rmt_write_verify(bp: *mut xfs_buf) {
    let mp = (*bp).b_mount; if !xfs_has_crc(mp) { return; }
    let blksize = (*(*mp).m_attr_geo).blksize; let mut ptr = (*bp).b_addr as *mut u8; let mut bno = xfs_buf_daddr(bp); let mut len = BBTOB((*bp).b_length);
    ASSERT(len >= blksize);
    while len > 0 { let rmt = ptr as *mut xfs_attr3_rmt_hdr; let fa = xfs_attr3_rmt_verify(mp, bp, ptr as *mut _, bno); if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); return; }
        if (*rmt).rm_lsn != cpu_to_be64(NULLCOMMITLSN) { xfs_verifier_error(bp, -EFSCORRUPTED, __this_address!()); return; }
        xfs_update_cksum(ptr as *mut _, blksize, XFS_ATTR3_RMT_CRC_OFF); len -= blksize; ptr = ptr.add(blksize as usize); bno += BTOBB(blksize); }
    if len != 0 { xfs_verifier_error(bp, -EFSCORRUPTED, __this_address!()); }
}

pub static mut xfs_attr3_rmt_buf_ops: xfs_buf_ops = xfs_buf_ops { name: "xfs_attr3_rmt", magic: [0, cpu_to_be32(XFS_ATTR3_RMT_MAGIC)], verify_read: Some(xfs_attr3_rmt_read_verify), verify_write: Some(xfs_attr3_rmt_write_verify), verify_struct: Some(xfs_attr3_rmt_verify_struct) };

unsafe fn xfs_attr3_rmt_hdr_set(mp: *mut xfs_mount, ptr: *mut core::ffi::c_void, ino: xfs_ino_t, offset: u32, size: u32, bno: xfs_daddr_t) -> i32 {
    if !xfs_has_crc(mp) { return 0; } let rmt = ptr as *mut xfs_attr3_rmt_hdr;
    (*rmt).rm_magic=cpu_to_be32(XFS_ATTR3_RMT_MAGIC); (*rmt).rm_offset=cpu_to_be32(offset); (*rmt).rm_bytes=cpu_to_be32(size); uuid_copy(&mut (*rmt).rm_uuid, &(*mp).m_sb.sb_meta_uuid); (*rmt).rm_owner=cpu_to_be64(ino); (*rmt).rm_blkno=cpu_to_be64(bno); (*rmt).rm_lsn=cpu_to_be64(NULLCOMMITLSN); core::mem::size_of::<xfs_attr3_rmt_hdr>() as i32
}

unsafe fn xfs_attr_rmtval_copyout(mp: *mut xfs_mount, bp: *mut xfs_buf, dp: *mut xfs_inode, owner: xfs_ino_t, offset: *mut u32, valuelen: *mut u32, dst: *mut *mut u8) -> i32 {
    let mut src=(*bp).b_addr as *mut u8; let mut bno=xfs_buf_daddr(bp); let mut len=BBTOB((*bp).b_length); let blksize=(*(*mp).m_attr_geo).blksize; ASSERT(len>=blksize);
    while len>0 && *valuelen>0 { let mut hdr=0; let mut cnt=core::cmp::min(*valuelen,xfs_attr3_rmt_buf_space(mp)); if xfs_has_crc(mp) { if !xfs_attr3_rmt_hdr_ok(src as *mut _,owner,*offset,cnt,bno).is_null() { xfs_alert(mp,"remote attribute header mismatch bno/off/len/owner",bno,*offset,cnt,owner); xfs_dirattr_mark_sick(dp,XFS_ATTR_FORK); return -EFSCORRUPTED; } hdr=core::mem::size_of::<xfs_attr3_rmt_hdr>() as u32; } core::ptr::copy_nonoverlapping(src.add(hdr as usize),*dst,cnt as usize); len-=blksize; src=src.add(blksize as usize); bno+=BTOBB(blksize); *valuelen-=cnt; *dst=(*dst).add(cnt as usize); *offset+=cnt; } 0
}

unsafe fn xfs_attr_rmtval_copyin(mp:*mut xfs_mount,bp:*mut xfs_buf,ino:xfs_ino_t,offset:*mut u32,valuelen:*mut u32,src:*mut *mut u8){let mut dst=(*bp).b_addr as *mut u8;let mut bno=xfs_buf_daddr(bp);let mut len=BBTOB((*bp).b_length);let bs=(*(*mp).m_attr_geo).blksize;ASSERT(len>=bs);while len>0&&*valuelen>0{let cnt=core::cmp::min(*valuelen,xfs_attr3_rmt_buf_space(mp));let hdr=xfs_attr3_rmt_hdr_set(mp,dst as *mut _,ino,*offset,cnt,bno) as u32;core::ptr::copy_nonoverlapping(*src,dst.add(hdr as usize),cnt as usize);if cnt+hdr<bs{ASSERT(*valuelen-cnt==0);ASSERT(len==bs);core::ptr::write_bytes(dst.add((hdr+cnt) as usize),0,(bs-hdr-cnt) as usize);}len-=bs;dst=dst.add(bs as usize);bno+=BTOBB(bs);*valuelen-=cnt;*src=(*src).add(cnt as usize);*offset+=cnt;}}

pub unsafe fn xfs_attr_rmtval_get(args:*mut xfs_da_args)->i32{let mp=(*(*args).dp).i_mount;let mut map=[core::mem::zeroed::<xfs_bmbt_irec>();ATTR_RMTVALUE_MAPSIZE];let mut bp=core::ptr::null_mut();let mut blk=(*args).rmtblkno;let mut dst=(*args).value;let mut left=(*args).rmtvaluelen;let mut cnt=(*args).rmtblkcnt;let mut off=0;ASSERT((*args).valuelen!=0);ASSERT((*args).rmtvaluelen==(*args).valuelen);trace_xfs_attr_rmtval_get(args);while left>0{let mut n=ATTR_RMTVALUE_MAPSIZE as i32;let e=xfs_bmapi_read((*args).dp,blk as xfs_fileoff_t,cnt,map.as_mut_ptr(),&mut n,XFS_BMAPI_ATTRFORK);if e!=0{return e;}ASSERT(n>=1);for i in 0..n as usize{if left==0{break;}ASSERT(map[i].br_startblock!=DELAYSTARTBLOCK&&map[i].br_startblock!=HOLESTARTBLOCK);let db=XFS_FSB_TO_DADDR(mp,map[i].br_startblock);let dc=XFS_FSB_TO_BB(mp,map[i].br_blockcount);let mut e=xfs_buf_read((*mp).m_ddev_targp,db,dc,0,&mut bp,&raw mut xfs_attr3_rmt_buf_ops);if xfs_metadata_is_sick(e){xfs_dirattr_mark_sick((*args).dp,XFS_ATTR_FORK);}if e==-ENODATA{e=-EIO;}if e!=0{return e;}e=xfs_attr_rmtval_copyout(mp,bp,(*args).dp,(*args).owner,&mut off,&mut left,&mut dst);xfs_buf_relse(bp);if e!=0{return e;}blk+=map[i].br_blockcount;cnt-=map[i].br_blockcount;}}ASSERT(left==0);0}

pub unsafe fn xfs_attr_rmt_find_hole(args:*mut xfs_da_args)->i32{let mp=(*(*args).dp).i_mount;let mut off=0;xfs_attr3_rmt_blocks(mp,(*args).rmtvaluelen);let e=xfs_bmap_first_unused((*args).trans,(*args).dp,xfs_attr3_rmt_blocks(mp,(*args).rmtvaluelen),&mut off,XFS_ATTR_FORK);if e!=0{return e;}(*args).rmtblkno=off as xfs_dablk_t;(*args).rmtblkcnt=xfs_attr3_rmt_blocks(mp,(*args).rmtvaluelen);0}

pub unsafe fn xfs_attr_rmtval_set_value(args:*mut xfs_da_args)->i32{let dp=(*args).dp;let mp=(*dp).i_mount;let mut map=core::mem::zeroed::<xfs_bmbt_irec>();let mut blk=(*args).rmtblkno;let mut bc=(*args).rmtblkcnt;let mut left=(*args).rmtvaluelen;let mut src=(*args).value;let mut off=0;while left>0{let mut n=1;let e=xfs_bmapi_read(dp,blk as xfs_fileoff_t,bc,&mut map,&mut n,XFS_BMAPI_ATTRFORK);if e!=0{return e;}ASSERT(n==1&&map.br_startblock!=DELAYSTARTBLOCK&&map.br_startblock!=HOLESTARTBLOCK);let db=XFS_FSB_TO_DADDR(mp,map.br_startblock);let dc=XFS_FSB_TO_BB(mp,map.br_blockcount);let mut bp=core::ptr::null_mut();let mut e=xfs_buf_get((*mp).m_ddev_targp,db,dc,&mut bp);if e!=0{return e;}(*bp).b_ops=&raw mut xfs_attr3_rmt_buf_ops;xfs_attr_rmtval_copyin(mp,bp,(*args).owner,&mut off,&mut left,&mut src);e=xfs_bwrite(bp);xfs_buf_relse(bp);if e!=0{return e;}blk+=map.br_blockcount;bc-=map.br_blockcount;}ASSERT(left==0);0}

pub unsafe fn xfs_attr_rmtval_stale(ip:*mut xfs_inode,map:*mut xfs_bmbt_irec,flags:xfs_buf_flags_t)->i32{let mp=(*ip).i_mount;xfs_assert_ilocked(ip,XFS_ILOCK_EXCL);if XFS_IS_CORRUPT(mp,(*map).br_startblock==DELAYSTARTBLOCK||(*map).br_startblock==HOLESTARTBLOCK){xfs_bmap_mark_sick(ip,XFS_ATTR_FORK);return -EFSCORRUPTED;}let mut bp=core::ptr::null_mut();let e=xfs_buf_incore((*mp).m_ddev_targp,XFS_FSB_TO_DADDR(mp,(*map).br_startblock),XFS_FSB_TO_BB(mp,(*map).br_blockcount),flags,&mut bp);if e!=0{return if e==-ENOENT{0}else{e}} xfs_buf_stale(bp);xfs_buf_relse(bp);0}

pub unsafe fn xfs_attr_rmtval_find_space(attr:*mut xfs_attr_intent)->i32{let args=(*attr).xattri_da_args;let map=&mut (*attr).xattri_map;(*attr).xattri_lblkno=0;(*attr).xattri_blkcnt=0;(*args).rmtblkcnt=0;(*args).rmtblkno=0;core::ptr::write_bytes(map as *mut _,0,1);let e=xfs_attr_rmt_find_hole(args);if e!=0{return e;}(*attr).xattri_blkcnt=(*args).rmtblkcnt;(*attr).xattri_lblkno=(*args).rmtblkno;0}
pub unsafe fn xfs_attr_rmtval_set_blk(attr:*mut xfs_attr_intent)->i32{let args=(*attr).xattri_da_args;let mut n=1;let e=xfs_bmapi_write((*args).trans,(*args).dp,(*attr).xattri_lblkno as xfs_fileoff_t,(*attr).xattri_blkcnt,XFS_BMAPI_ATTRFORK,(*args).total,&mut (*attr).xattri_map,&mut n);if e!=0{return e;}ASSERT(n==1);(*attr).xattri_lblkno+=(*attr).xattri_map.br_blockcount;(*attr).xattri_blkcnt-=(*attr).xattri_map.br_blockcount;0}
pub unsafe fn xfs_attr_rmtval_invalidate(args:*mut xfs_da_args)->i32{let mut blk=(*args).rmtblkno;let mut cnt=(*args).rmtblkcnt;while cnt>0{let mut map=core::mem::zeroed();let mut n=1;let e=xfs_bmapi_read((*args).dp,blk as xfs_fileoff_t,cnt,&mut map,&mut n,XFS_BMAPI_ATTRFORK);if e!=0{return e;}if XFS_IS_CORRUPT((*(*args).dp).i_mount,n!=1){xfs_bmap_mark_sick((*args).dp,XFS_ATTR_FORK);return -EFSCORRUPTED;}let e=xfs_attr_rmtval_stale((*args).dp,&mut map,XBF_TRYLOCK);if e!=0{return e;}blk+=map.br_blockcount;cnt-=map.br_blockcount;}0}
pub unsafe fn xfs_attr_rmtval_remove(attr:*mut xfs_attr_intent)->i32{let args=(*attr).xattri_da_args;let mut done=0;let e=xfs_bunmapi((*args).trans,(*args).dp,(*args).rmtblkno,(*args).rmtblkcnt,XFS_BMAPI_ATTRFORK,1,&mut done);if e!=0{return e;}if done==0{trace_xfs_attr_rmtval_remove_return((*attr).xattri_dela_state,(*args).dp);return -EAGAIN;}(*args).rmtblkno=0;(*args).rmtblkcnt=0;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
