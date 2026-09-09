// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of directory.c; kernel and UDF symbols are supplied externally. */

unsafe fn udf_verify_fi(iter: *mut udf_fileident_iter) -> c_int {
    let mut len: u32;
    if (*iter).fi.descTag.tagIdent != cpu_to_le16(TAG_IDENT_FID) {
        udf_err((*iter).dir.i_sb, "directory (ino %llu) has entry at pos %llu with incorrect tag %x\n", (*iter).dir.i_ino, (*iter).pos as u64, le16_to_cpu((*iter).fi.descTag.tagIdent));
        return -EFSCORRUPTED;
    }
    len = udf_dir_entry_len(&(*iter).fi);
    if le16_to_cpu((*iter).fi.lengthOfImpUse) & 3 != 0 {
        udf_err((*iter).dir.i_sb, "directory (ino %llu) has entry at pos %llu with unaligned length of impUse field\n", (*iter).dir.i_ino, (*iter).pos as u64);
        return -EFSCORRUPTED;
    }
    if len > (1u32 << (*iter).dir.i_blkbits) {
        udf_err((*iter).dir.i_sb, "directory (ino %llu) has too big (%u) entry at pos %llu\n", (*iter).dir.i_ino, len, (*iter).pos as u64);
        return -EFSCORRUPTED;
    }
    if (*iter).pos + len as loff_t > (*iter).dir.i_size {
        udf_err((*iter).dir.i_sb, "directory (ino %llu) has entry past directory size at pos %llu\n", (*iter).dir.i_ino, (*iter).pos as u64);
        return -EFSCORRUPTED;
    }
    if udf_dir_entry_len(&(*iter).fi) != core::mem::size_of::<tag>() as u32 + le16_to_cpu((*iter).fi.descTag.descCRCLength) as u32 {
        udf_err((*iter).dir.i_sb, "directory (ino %llu) has entry where CRC length (%u) does not match entry length (%u)\n", (*iter).dir.i_ino, le16_to_cpu((*iter).fi.descTag.descCRCLength), udf_dir_entry_len(&(*iter).fi) - core::mem::size_of::<tag>() as u32);
        return -EFSCORRUPTED;
    }
    0
}

unsafe fn udf_copy_fi(iter: *mut udf_fileident_iter) -> c_int {
    let iinfo = UDF_I((*iter).dir);
    let blksize = 1u32 << (*iter).dir.i_blkbits;
    let (mut off, mut len, mut nameoff): (u32, u32, u32);
    let err: c_int;
    if (*iter).pos >= (*iter).dir.i_size { (*iter).name = core::ptr::null_mut(); return 0; }
    if (*iter).dir.i_size < (*iter).pos + core::mem::size_of::<fileIdentDesc>() as loff_t { udf_err((*iter).dir.i_sb, "directory (ino %llu) has entry straddling EOF\n", (*iter).dir.i_ino); return -EFSCORRUPTED; }
    if iinfo.i_alloc_type == ICBTAG_FLAG_AD_IN_ICB {
        core::ptr::copy_nonoverlapping(iinfo.i_data.add(iinfo.i_lenEAttr as usize).add((*iter).pos as usize), &mut (*iter).fi, 1);
        err = udf_verify_fi(iter); if err < 0 { return err; }
        (*iter).name = iinfo.i_data.add(iinfo.i_lenEAttr as usize).add((*iter).pos as usize + core::mem::size_of::<fileIdentDesc>() + le16_to_cpu((*iter).fi.lengthOfImpUse) as usize); return 0;
    }
    off = (*iter).pos as u32 & (blksize - 1); len = core::cmp::min(core::mem::size_of::<fileIdentDesc>() as u32, blksize - off);
    core::ptr::copy_nonoverlapping((*iter).bh[0].b_data.add(off as usize), &mut (*iter).fi as *mut _ as *mut u8, len as usize);
    if len < core::mem::size_of::<fileIdentDesc>() as u32 { core::ptr::copy_nonoverlapping((*iter).bh[1].b_data, (&mut (*iter).fi as *mut _ as *mut u8).add(len as usize), core::mem::size_of::<fileIdentDesc>() - len as usize); }
    err = udf_verify_fi(iter); if err < 0 { return err; }
    nameoff = off + core::mem::size_of::<fileIdentDesc>() as u32 + le16_to_cpu((*iter).fi.lengthOfImpUse) as u32;
    if off + udf_dir_entry_len(&(*iter).fi) <= blksize { (*iter).name = (*iter).bh[0].b_data.add(nameoff as usize); }
    else if nameoff >= blksize { (*iter).name = (*iter).bh[1].b_data.add((nameoff - blksize) as usize); }
    else { (*iter).name = (*iter).namebuf; len = blksize - nameoff; core::ptr::copy_nonoverlapping((*iter).bh[0].b_data.add(nameoff as usize), (*iter).name, len as usize); core::ptr::copy_nonoverlapping((*iter).bh[1].b_data, (*iter).name.add(len as usize), (*iter).fi.lengthFileIdent as usize - len as usize); }
    0
}

unsafe fn udf_readahead_dir(iter: *mut udf_fileident_iter) {
    let mut ralen = 16u32 >> ((*iter).dir.i_blkbits - 9); let mut bha: [*mut buffer_head; 16] = [core::ptr::null_mut(); 16]; let mut num = 0usize;
    if (*iter).loffset & (ralen - 1) != 0 { return; }
    if (*iter).loffset + ralen > ((*iter).elen >> (*iter).dir.i_blkbits) { ralen = ((*iter).elen >> (*iter).dir.i_blkbits) - (*iter).loffset; }
    for i in 0..ralen { let tmp = sb_getblk((*iter).dir.i_sb, udf_get_lb_pblock((*iter).dir.i_sb, &(*iter).eloc, (*iter).loffset + i)); if !tmp.is_null() && !buffer_uptodate(tmp) && !buffer_locked(tmp) { bha[num] = tmp; num += 1; } else { brelse(tmp); } }
    if num != 0 { bh_readahead_batch(num as c_int, bha.as_mut_ptr(), REQ_RAHEAD); for i in 0..num { brelse(bha[i]); } }
}

unsafe fn udf_fiiter_bread_blk(iter: *mut udf_fileident_iter) -> *mut buffer_head { udf_readahead_dir(iter); sb_bread((*iter).dir.i_sb, udf_get_lb_pblock((*iter).dir.i_sb, &(*iter).eloc, (*iter).loffset)) }

unsafe fn udf_fiiter_advance_blk(iter: *mut udf_fileident_iter) -> c_int {
    let mut etype: i8 = -1; (*iter).loffset += 1;
    if (*iter).loffset < ((*iter).elen + ((1u32 << (*iter).dir.i_blkbits) - 1)) / (1u32 << (*iter).dir.i_blkbits) { return 0; }
    (*iter).loffset = 0; let err = udf_next_aext((*iter).dir, &mut (*iter).epos, &mut (*iter).eloc, &mut (*iter).elen, &mut etype, 1); if err < 0 { return err; }
    if err == 0 || etype != (EXT_RECORDED_ALLOCATED >> 30) { if (*iter).pos == (*iter).dir.i_size { (*iter).elen = 0; return 0; } udf_err((*iter).dir.i_sb, "extent after position %llu not allocated in directory (ino %llu)\n", (*iter).pos as u64, (*iter).dir.i_ino); return -EFSCORRUPTED; } 0
}

unsafe fn udf_fiiter_load_bhs(iter: *mut udf_fileident_iter) -> c_int {
    let blksize = 1i32 << (*iter).dir.i_blkbits; let off = (*iter).pos as i32 & (blksize - 1); let mut err: c_int;
    if (*iter).bh[0].is_null() && (*iter).elen != 0 { (*iter).bh[0] = udf_fiiter_bread_blk(iter); if (*iter).bh[0].is_null() { err = -ENOMEM; return udf_fiiter_load_bhs_out(iter, err); } if !buffer_uptodate((*iter).bh[0]) { err = -EIO; return udf_fiiter_load_bhs_out(iter, err); } }
    if (*iter).pos >= (*iter).dir.i_size { return 0; }
    if off + core::mem::size_of::<fileIdentDesc>() as i32 > blksize || off + udf_dir_entry_len(&*((*iter).bh[0].b_data.add(off as usize) as *const fileIdentDesc)) as i32 > blksize {
        err = udf_fiiter_advance_blk(iter); if err != 0 { return udf_fiiter_load_bhs_out(iter, err); }
        (*iter).bh[1] = udf_fiiter_bread_blk(iter); if (*iter).bh[1].is_null() { err = -ENOMEM; return udf_fiiter_load_bhs_out(iter, err); } if !buffer_uptodate((*iter).bh[1]) { err = -EIO; return udf_fiiter_load_bhs_out(iter, err); }
    } 0
}
unsafe fn udf_fiiter_load_bhs_out(iter: *mut udf_fileident_iter, err: c_int) -> c_int { brelse((*iter).bh[0]); brelse((*iter).bh[1]); (*iter).bh[0] = core::ptr::null_mut(); (*iter).bh[1] = core::ptr::null_mut(); err }

pub unsafe fn udf_fiiter_init(iter: *mut udf_fileident_iter, dir: *mut inode, pos: loff_t) -> c_int { let iinfo = UDF_I(dir); (*iter).dir=dir; (*iter).bh=[core::ptr::null_mut(),core::ptr::null_mut()]; (*iter).pos=pos; (*iter).elen=0; (*iter).epos.bh=core::ptr::null_mut(); (*iter).name=core::ptr::null_mut(); (*iter).namebuf=kmalloc(UDF_NAME_LEN_CS0, GFP_KERNEL|__GFP_NOFAIL); if iinfo.i_alloc_type==ICBTAG_FLAG_AD_IN_ICB { let e=udf_copy_fi(iter); if e<0 { udf_fiiter_release(iter); } return e; } let mut etype=0i8; let mut e=inode_bmap(dir,pos>>dir.i_blkbits,&mut (*iter).epos,&mut (*iter).eloc,&mut (*iter).elen,&mut (*iter).loffset,&mut etype); if e<=0 || etype!=(EXT_RECORDED_ALLOCATED>>30) { if pos==dir.i_size{return 0;} udf_err(dir.i_sb,"position %llu not allocated in directory (ino %llu)\n",pos as u64,dir.i_ino); return -EFSCORRUPTED; } e=udf_fiiter_load_bhs(iter); if e<0{return e;} e=udf_copy_fi(iter); if e<0{udf_fiiter_release(iter);} e }

pub unsafe fn udf_fiiter_advance(iter:*mut udf_fileident_iter)->c_int { let oldoff=(*iter).pos as u32 & ((1u32<<(*iter).dir.i_blkbits)-1); let len=udf_dir_entry_len(&(*iter).fi); (*iter).pos+=len as loff_t; if UDF_I((*iter).dir).i_alloc_type!=ICBTAG_FLAG_AD_IN_ICB && oldoff+len >= (1u32<<(*iter).dir.i_blkbits) { brelse((*iter).bh[0]); (*iter).bh[0]=(*iter).bh[1]; (*iter).bh[1]=core::ptr::null_mut(); if (*iter).bh[0].is_null(){let e=udf_fiiter_advance_blk(iter);if e<0{return e;}} let e=udf_fiiter_load_bhs(iter);if e<0{return e;} } udf_copy_fi(iter) }
pub unsafe fn udf_fiiter_release(iter:*mut udf_fileident_iter){(*iter).dir=core::ptr::null_mut();brelse((*iter).bh[0]);brelse((*iter).bh[1]);(*iter).bh=[core::ptr::null_mut(),core::ptr::null_mut()];kfree((*iter).namebuf);(*iter).namebuf=core::ptr::null_mut();}

unsafe fn udf_copy_to_bufs(buf1:*mut u8,len1:c_int,buf2:*mut u8,len2:c_int,mut off:c_int,mut src:*const u8,mut len:c_int){if off<len1{let copy=core::cmp::min(off+len,len1)-off;core::ptr::copy_nonoverlapping(src,buf1.add(off as usize),copy as usize);src=src.add(copy as usize);len-=copy;off=0}else{off-=len1;}if len>0{if off+len>len2||buf2.is_null(){return;}core::ptr::copy_nonoverlapping(src,buf2.add(off as usize),len as usize);}}
unsafe fn udf_crc_fi_bufs(buf1:*mut u8,len1:c_int,buf2:*mut u8,len2:c_int,mut off:c_int,mut len:c_int)->u16{let mut crc=0u16;if off<len1{let copy=core::cmp::min(off+len,len1)-off;crc=crc_itu_t(crc,buf1.add(off as usize),copy);len-=copy;off=0}else{off-=len1;}if len>0{if off+len>len2||buf2.is_null(){return 0;}crc=crc_itu_t(crc,buf2.add(off as usize),len);}crc}

unsafe fn udf_copy_fi_to_bufs(buf1:*mut u8,len1:c_int,buf2:*mut u8,len2:c_int,off:c_int,fi:*mut fileIdentDesc,impuse:*mut u8,name:*mut u8){let mut o=off;let crcoff=off+core::mem::size_of::<tag>() as c_int;let crclen=udf_dir_entry_len(&*fi)-core::mem::size_of::<tag>() as u32;let zeros=[0u8;UDF_NAME_PAD];udf_copy_to_bufs(buf1,len1,buf2,len2,o,fi as *const _ as *const u8,core::mem::size_of::<fileIdentDesc>() as c_int);o+=core::mem::size_of::<fileIdentDesc>() as c_int;if !impuse.is_null(){udf_copy_to_bufs(buf1,len1,buf2,len2,o,impuse,le16_to_cpu((*fi).lengthOfImpUse) as c_int);}o+=le16_to_cpu((*fi).lengthOfImpUse) as c_int;if !name.is_null(){udf_copy_to_bufs(buf1,len1,buf2,len2,o,name,(*fi).lengthFileIdent as c_int);o+=(*fi).lengthFileIdent as c_int;udf_copy_to_bufs(buf1,len1,buf2,len2,o,zeros.as_ptr(),off+udf_dir_entry_len(&*fi) as c_int-o);}let crc=udf_crc_fi_bufs(buf1,len1,buf2,len2,crcoff,crclen as c_int);(*fi).descTag.descCRC=cpu_to_le16(crc);(*fi).descTag.descCRCLength=cpu_to_le16(crclen as u16);(*fi).descTag.tagChecksum=udf_tag_checksum(&mut (*fi).descTag);udf_copy_to_bufs(buf1,len1,buf2,len2,off,fi as *const _ as *const u8,core::mem::size_of::<tag>() as c_int);}

pub unsafe fn udf_fiiter_write_fi(iter:*mut udf_fileident_iter,impuse:*mut u8){let iinfo=UDF_I((*iter).dir);let off=(*iter).pos as c_int&((1<<(*iter).dir.i_blkbits)-1);let (buf1,len1,buf2,len2) = if iinfo.i_alloc_type==ICBTAG_FLAG_AD_IN_ICB {(iinfo.i_data.add(iinfo.i_lenEAttr as usize),(*iter).dir.i_size as c_int,core::ptr::null_mut(),0)}else{((*iter).bh[0].b_data,1<<(*iter).dir.i_blkbits,if !(*iter).bh[1].is_null(){(*iter).bh[1].b_data}else{core::ptr::null_mut()},if !(*iter).bh[1].is_null(){1<<(*iter).dir.i_blkbits}else{0})};let name=if (*iter).name==(*iter).namebuf{(*iter).name}else{core::ptr::null_mut()};udf_copy_fi_to_bufs(buf1,len1,buf2,len2,off,&mut (*iter).fi,impuse,name);if iinfo.i_alloc_type==ICBTAG_FLAG_AD_IN_ICB{mark_inode_dirty((*iter).dir);}else{mmb_mark_buffer_dirty((*iter).bh[0],&mut iinfo.i_metadata_bhs);if !(*iter).bh[1].is_null(){mmb_mark_buffer_dirty((*iter).bh[1],&mut iinfo.i_metadata_bhs);}}inode_inc_iversion((*iter).dir);}

pub unsafe fn udf_fiiter_update_elen(iter:*mut udf_fileident_iter,new_elen:u32){let iinfo=UDF_I((*iter).dir);if (*iter).elen==0{return;}let diff=new_elen as i32-(*iter).elen as i32;(*iter).elen=new_elen;if iinfo.i_alloc_type==ICBTAG_FLAG_AD_SHORT{(*iter).epos.offset-=core::mem::size_of::<short_ad>() as u32;}else if iinfo.i_alloc_type==ICBTAG_FLAG_AD_LONG{(*iter).epos.offset-=core::mem::size_of::<long_ad>() as u32;}udf_write_aext((*iter).dir,&mut (*iter).epos,&mut (*iter).eloc,(*iter).elen,1);iinfo.i_lenExtents+=diff;mark_inode_dirty((*iter).dir);}

pub unsafe fn udf_fiiter_append_blk(iter:*mut udf_fileident_iter)->c_int{let iinfo=UDF_I((*iter).dir);let blksize=1u32<<(*iter).dir.i_blkbits;if iinfo.i_alloc_type==ICBTAG_FLAG_AD_IN_ICB{return -EINVAL;}let old=(*iter).elen;udf_fiiter_update_elen(iter,(old+blksize-1)&!(blksize-1));let block=iinfo.i_lenExtents>>(*iter).dir.i_blkbits;let mut err=0;let bh=udf_bread((*iter).dir,block,1,&mut err);if bh.is_null(){udf_fiiter_update_elen(iter,old);return err;}let mut etype=0i8;err=inode_bmap((*iter).dir,block,&mut (*iter).epos,&mut (*iter).eloc,&mut (*iter).elen,&mut (*iter).loffset,&mut etype);if err<=0||etype!=(EXT_RECORDED_ALLOCATED>>30){return -EFSCORRUPTED;}if (*iter).pos&(blksize-1)==0{brelse((*iter).bh[0]);(*iter).bh[0]=bh;}else{(*iter).bh[1]=bh;}0}

pub unsafe fn udf_get_fileshortad(ptr:*mut u8,maxoffset:c_int,offset:*mut u32,inc:c_int)->*mut short_ad{if ptr.is_null()||offset.is_null(){return core::ptr::null_mut();}if *offset+core::mem::size_of::<short_ad>() as u32>maxoffset as u32{return core::ptr::null_mut();}let sa=ptr as *mut short_ad;if (*sa).extLength==0{return core::ptr::null_mut();}if inc!=0{*offset+=core::mem::size_of::<short_ad>() as u32;}sa}
pub unsafe fn udf_get_filelongad(ptr:*mut u8,maxoffset:c_int,offset:*mut u32,inc:c_int)->*mut long_ad{if ptr.is_null()||offset.is_null(){return core::ptr::null_mut();}if *offset+core::mem::size_of::<long_ad>() as u32>maxoffset as u32{return core::ptr::null_mut();}let la=ptr as *mut long_ad;if (*la).extLength==0{return core::ptr::null_mut();}if inc!=0{*offset+=core::mem::size_of::<long_ad>() as u32;}la}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
