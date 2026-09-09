// SPDX-License-Identifier: GPL-2.0-only
/* vfsv0 quota IO operations on file */
// External kernel types, constants, and functions are supplied by the surrounding crate.

const MAX_QTREE_DEPTH: usize = 6;

unsafe fn __get_index(info: *mut qtree_mem_dqinfo, mut id: qid_t, mut depth: i32) -> i32 {
    let epb = (*info).dqi_usable_bs >> 2;
    depth = (*info).dqi_qtree_depth - depth - 1;
    while depth > 0 { id /= epb; depth -= 1; }
    (id % epb) as i32
}
unsafe fn get_index(info: *mut qtree_mem_dqinfo, qid: kqid, depth: i32) -> i32 {
    __get_index(info, from_kqid(&init_user_ns, qid), depth)
}
unsafe fn qtree_dqstr_in_blk(info: *mut qtree_mem_dqinfo) -> i32 {
    (((*info).dqi_usable_bs - core::mem::size_of::<qt_disk_dqdbheader>()) / (*info).dqi_entry_size) as i32
}
unsafe fn read_blk(info: *mut qtree_mem_dqinfo, blk: u32, buf: *mut i8) -> isize {
    core::ptr::write_bytes(buf, 0, (*info).dqi_usable_bs);
    (*(*info).dqi_sb).s_op.quota_read((*info).dqi_sb, (*info).dqi_type, buf, (*info).dqi_usable_bs, (blk as i64) << (*info).dqi_blocksize_bits)
}
unsafe fn write_blk(info: *mut qtree_mem_dqinfo, blk: u32, buf: *mut i8) -> isize {
    let mut ret = (*(*info).dqi_sb).s_op.quota_write((*info).dqi_sb, (*info).dqi_type, buf, (*info).dqi_usable_bs, (blk as i64) << (*info).dqi_blocksize_bits);
    if ret != (*info).dqi_usable_bs as isize { quota_error((*info).dqi_sb, c"dquota write failed"); if ret >= 0 { ret = -EIO as isize; } }
    ret
}
unsafe fn do_check_range(sb: *mut super_block, name: *const i8, val: u32, min: u32, max: u32) -> i32 {
    if val < min || val > max { quota_error(sb, name); return -EUCLEAN; } 0
}
unsafe fn check_dquot_block_header(info: *mut qtree_mem_dqinfo, dh: *mut qt_disk_dqdbheader) -> i32 {
    let r = do_check_range((*info).dqi_sb, c"dqdh_next_free", le32_to_cpu((*dh).dqdh_next_free), 0, (*info).dqi_blocks-1); if r != 0 { return r; }
    let r = do_check_range((*info).dqi_sb, c"dqdh_prev_free", le32_to_cpu((*dh).dqdh_prev_free), 0, (*info).dqi_blocks-1); if r != 0 { return r; }
    do_check_range((*info).dqi_sb, c"dqdh_entries", le16_to_cpu((*dh).dqdh_entries) as u32, 0, qtree_dqstr_in_blk(info) as u32)
}
unsafe fn get_free_dqblk(info: *mut qtree_mem_dqinfo) -> i32 {
    let buf = kmalloc((*info).dqi_usable_bs, GFP_KERNEL); if buf.is_null() { return -ENOMEM; }
    let dh = buf as *mut qt_disk_dqdbheader; let mut ret; let blk;
    if (*info).dqi_free_blk != 0 { blk=(*info).dqi_free_blk; ret=read_blk(info,blk,buf); if ret<0 { kfree(buf); return ret as i32; } ret=check_dquot_block_header(info,dh) as isize; if ret!=0 { kfree(buf); return ret as i32; } (*info).dqi_free_blk=le32_to_cpu((*dh).dqdh_next_free); }
    else { core::ptr::write_bytes(buf,0,(*info).dqi_usable_bs); ret=write_blk(info,(*info).dqi_blocks,buf); if ret<0 { kfree(buf); return ret as i32; } blk=(*info).dqi_blocks; (*info).dqi_blocks+=1; }
    mark_info_dirty((*info).dqi_sb,(*info).dqi_type); kfree(buf); blk as i32
}
unsafe fn put_free_dqblk(info:*mut qtree_mem_dqinfo,buf:*mut i8,blk:u32)->i32 { let dh=buf as *mut qt_disk_dqdbheader; (*dh).dqdh_next_free=cpu_to_le32((*info).dqi_free_blk);(*dh).dqdh_prev_free=cpu_to_le32(0);(*dh).dqdh_entries=cpu_to_le16(0);let e=write_blk(info,blk,buf);if e<0{return e as i32;}(*info).dqi_free_blk=blk;mark_info_dirty((*info).dqi_sb,(*info).dqi_type);0 }
unsafe fn remove_free_dqentry(info:*mut qtree_mem_dqinfo,buf:*mut i8,blk:u32)->i32 { let tmp=kmalloc((*info).dqi_usable_bs,GFP_KERNEL);if tmp.is_null(){return -ENOMEM;}let dh=buf as *mut qt_disk_dqdbheader;let n=le32_to_cpu((*dh).dqdh_next_free);let p=le32_to_cpu((*dh).dqdh_prev_free);if n!=0{let e=read_blk(info,n,tmp);if e<0{kfree(tmp);return e as i32;}(*(tmp as *mut qt_disk_dqdbheader)).dqdh_prev_free=(*dh).dqdh_prev_free;let e=write_blk(info,n,tmp);if e<0{kfree(tmp);return e as i32;}}if p!=0{let e=read_blk(info,p,tmp);if e<0{kfree(tmp);return e as i32;}(*(tmp as *mut qt_disk_dqdbheader)).dqdh_next_free=(*dh).dqdh_next_free;let e=write_blk(info,p,tmp);if e<0{kfree(tmp);return e as i32;}}else{(*info).dqi_free_entry=n;mark_info_dirty((*info).dqi_sb,(*info).dqi_type);}kfree(tmp);(*dh).dqdh_next_free=cpu_to_le32(0);(*dh).dqdh_prev_free=cpu_to_le32(0);write_blk(info,blk,buf);0 }
unsafe fn insert_free_dqentry(info:*mut qtree_mem_dqinfo,buf:*mut i8,blk:u32)->i32 { let tmp=kmalloc((*info).dqi_usable_bs,GFP_KERNEL);if tmp.is_null(){return -ENOMEM;}let dh=buf as *mut qt_disk_dqdbheader;(*dh).dqdh_next_free=cpu_to_le32((*info).dqi_free_entry);(*dh).dqdh_prev_free=cpu_to_le32(0);let mut e=write_blk(info,blk,buf);if e<0{kfree(tmp);return e as i32;}if (*info).dqi_free_entry!=0{e=read_blk(info,(*info).dqi_free_entry,tmp);if e<0{kfree(tmp);return e as i32;}(*(tmp as *mut qt_disk_dqdbheader)).dqdh_prev_free=cpu_to_le32(blk);e=write_blk(info,(*info).dqi_free_entry,tmp);if e<0{kfree(tmp);return e as i32;}}kfree(tmp);(*info).dqi_free_entry=blk;mark_info_dirty((*info).dqi_sb,(*info).dqi_type);0 }
pub unsafe fn qtree_entry_unused(info:*mut qtree_mem_dqinfo,disk:*const i8)->i32{for i in 0..(*info).dqi_entry_size{if *disk.add(i)!=0{return 0;}}1}
unsafe fn find_free_dqentry(i:*mut qtree_mem_dqinfo,d:*mut dquot,err:*mut i32)->u32{let b=kmalloc((*i).dqi_usable_bs,GFP_KERNEL);if b.is_null(){*err=-ENOMEM;return 0;}let dh=b as *mut qt_disk_dqdbheader;let blk=if (*i).dqi_free_entry!=0{(*i).dqi_free_entry}else{let x=get_free_dqblk(i);if x<0{*err=x;kfree(b);return 0;}core::ptr::write_bytes(b,0,(*i).dqi_usable_bs);(*i).dqi_free_entry=x as u32;mark_info_dirty((*d).dq_sb,(*d).dq_id.type_);x as u32};let mut e=le16_to_cpu((*dh).dqdh_entries) as i32;if e+1>=qtree_dqstr_in_blk(i){*err=remove_free_dqentry(i,b,blk);if *err<0{kfree(b);return 0;}}e+=1;(*dh).dqdh_entries=cpu_to_le16(e as u16);let mut p=b.add(core::mem::size_of::<qt_disk_dqdbheader>());for n in 0..qtree_dqstr_in_blk(i){if qtree_entry_unused(i,p)==1{(*d).dq_off=((blk as i64)<<(*i).dqi_blocksize_bits)+core::mem::size_of::<qt_disk_dqdbheader>() as i64+(n as i64)*(*i).dqi_entry_size as i64;*err=write_blk(i,blk,b) as i32;kfree(b);return blk;}p=p.add((*i).dqi_entry_size);}*err=-EIO;kfree(b);0}
unsafe fn do_insert_tree(i:*mut qtree_mem_dqinfo,d:*mut dquot,blks:*mut u32,depth:i32)->i32{let b=kmalloc((*i).dqi_usable_bs,GFP_KERNEL);if b.is_null(){return -ENOMEM;}let mut newact=false;let mut ret;if *blks.add(depth as usize)==0{ret=get_free_dqblk(i);if ret<0{kfree(b);return ret;}*blks.add(depth as usize)=ret as u32;core::ptr::write_bytes(b,0,(*i).dqi_usable_bs);newact=true;}else{ret=read_blk(i,*blks.add(depth as usize),b) as i32;if ret<0{kfree(b);return ret;}}let r=b as *mut u32;let ix=get_index(i,(*d).dq_id,depth) as usize;let nb=le32_to_cpu(*r.add(ix));if nb!=0{for n in 0..=depth{if nb==*blks.add(n as usize){kfree(b);return -EIO;}}}*blks.add(depth as usize+1)=nb;if depth==(*i).dqi_qtree_depth-1{if nb!=0{kfree(b);return -EIO;}ret=find_free_dqentry(i,d,&mut ret);}else{ret=do_insert_tree(i,d,blks,depth+1);}if ret>=0{*r.add(ix)=cpu_to_le32(*blks.add(depth as usize+1));ret=write_blk(i,*blks.add(depth as usize),b) as i32;}else if newact{put_free_dqblk(i,b,*blks.add(depth as usize));}kfree(b);ret}
unsafe fn dq_insert_tree(i:*mut qtree_mem_dqinfo,d:*mut dquot)->i32{let mut b=[QT_TREEOFF;MAX_QTREE_DEPTH];if (*i).dqi_blocks<=QT_TREEOFF{return -EIO;}if (*i).dqi_qtree_depth>=MAX_QTREE_DEPTH as i32{return -EIO;}do_insert_tree(i,d,b.as_mut_ptr(),0)}
pub unsafe fn qtree_write_dquot(i:*mut qtree_mem_dqinfo,d:*mut dquot)->i32{let b=kmalloc((*i).dqi_entry_size,GFP_KERNEL);if b.is_null(){return -ENOMEM;}if (*d).dq_off==0{let r=dq_insert_tree(i,d);if r<0{kfree(b);return r;}}spin_lock(&mut (*d).dq_dqb_lock);(*i).dqi_ops.mem2disk_dqblk(b,d);spin_unlock(&mut (*d).dq_dqb_lock);let mut r=(*(*d).dq_sb).s_op.quota_write((*d).dq_sb,(*d).dq_id.type_,b,(*i).dqi_entry_size,(*d).dq_off);if r==(*i).dqi_entry_size as isize{r=0;}else if r>=0{r=-ENOSPC as isize;}dqstats_inc(DQST_WRITES);kfree(b);r as i32}
unsafe fn free_dqentry(i:*mut qtree_mem_dqinfo,d:*mut dquot,blk:u32)->i32{let b=kmalloc((*i).dqi_usable_bs,GFP_KERNEL);if b.is_null(){return -ENOMEM;}let mut r=read_blk(i,blk,b) as i32;if r<0{kfree(b);return r;}let h=b as *mut qt_disk_dqdbheader;(*h).dqdh_entries=cpu_to_le16(le16_to_cpu((*h).dqdh_entries)-1);if le16_to_cpu((*h).dqdh_entries)==0{r=remove_free_dqentry(i,b,blk);if r>=0{r=put_free_dqblk(i,b,blk);}}else{core::ptr::write_bytes(b.add(((*d).dq_off&(((1i64<<(*i).dqi_blocksize_bits)-1))) as usize),0,(*i).dqi_entry_size);r=write_blk(i,blk,b) as i32;}(*d).dq_off=0;kfree(b);r}
unsafe fn remove_tree(i:*mut qtree_mem_dqinfo,d:*mut dquot,blks:*mut u32,depth:i32)->i32{let b=kmalloc((*i).dqi_usable_bs,GFP_KERNEL);if b.is_null(){return -ENOMEM;}let r=b as *mut u32;let e=read_blk(i,*blks.add(depth as usize),b);if e<0{kfree(b);return e as i32;}let ix=get_index(i,(*d).dq_id,depth) as usize;let nb=le32_to_cpu(*r.add(ix));let ret=if depth==(*i).dqi_qtree_depth-1{free_dqentry(i,d,nb)}else{*blks.add(depth as usize+1)=nb;remove_tree(i,d,blks,depth+1)};if ret>=0{*r.add(ix)=0;write_blk(i,*blks.add(depth as usize),b);}kfree(b);ret}
pub unsafe fn qtree_delete_dquot(i:*mut qtree_mem_dqinfo,d:*mut dquot)->i32{if (*d).dq_off==0{return 0;}let mut b=[QT_TREEOFF;MAX_QTREE_DEPTH];remove_tree(i,d,b.as_mut_ptr(),0)}
pub unsafe fn qtree_read_dquot(_i:*mut qtree_mem_dqinfo,_d:*mut dquot)->i32{0}
pub unsafe fn qtree_release_dquot(_i:*mut qtree_mem_dqinfo,_d:*mut dquot)->i32{0}
pub unsafe fn qtree_get_next_id(_i:*mut qtree_mem_dqinfo,_q:*mut kqid)->i32{-ENOENT}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
