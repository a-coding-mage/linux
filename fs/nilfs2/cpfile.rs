// SPDX-License-Identifier: GPL-2.0+
/* Literal low-level translation of nilfs2/cpfile.c.  Kernel types and
 * external helpers are supplied by the surrounding translation unit. */

use core::{mem::size_of, ptr, ffi::c_void};

unsafe fn cp_per_block(cpfile: *const inode) -> usize { (*nilfs_mdt(cpfile)).mi_entries_per_block as usize }
unsafe fn blkoff(cpfile: *const inode, cno: u64) -> usize { ((cno + (*nilfs_mdt(cpfile)).mi_first_entry_offset - 1) / cp_per_block(cpfile) as u64) as usize }
unsafe fn entoff(cpfile: *const inode, cno: u64) -> usize { ((cno + (*nilfs_mdt(cpfile)).mi_first_entry_offset - 1) % cp_per_block(cpfile) as u64) as usize }
unsafe fn first_in_block(cpfile: *const inode, b: usize) -> u64 { cp_per_block(cpfile) as u64 * b as u64 + 1 - (*nilfs_mdt(cpfile)).mi_first_entry_offset }
unsafe fn cps_in_block(cpfile: *const inode, cur: u64, max: u64) -> usize { core::cmp::min(cp_per_block(cpfile) - entoff(cpfile, cur), (max-cur) as usize) }
unsafe fn is_first(cpfile: *const inode, cno: u64) -> bool { blkoff(cpfile,cno)==0 }

unsafe fn add_valid(_: *const inode, bh: *mut buffer_head, n: u32) -> u32 {
    let cp = kmap_local_folio((*bh).b_folio, offset_in_folio((*bh).b_folio, (*bh).b_data)) as *mut nilfs_checkpoint;
    let v = u32::from_le((*cp).cp_checkpoints_count).wrapping_add(n); (*cp).cp_checkpoints_count=v.to_le(); kunmap_local(cp as *mut c_void); v
}
unsafe fn sub_valid(cpfile: *const inode, bh: *mut buffer_head, n: u32) -> i32 {
    let cp=kmap_local_folio((*bh).b_folio,offset_in_folio((*bh).b_folio,(*bh).b_data)) as *mut nilfs_checkpoint; let v=u32::from_le((*cp).cp_checkpoints_count);
    if v<n { nilfs_error((*cpfile).i_sb,b"deleted checkpoints count exceeds block count\0" as *const u8); kunmap_local(cp as *mut c_void); return -5; }
    let r=v-n; (*cp).cp_checkpoints_count=r.to_le(); kunmap_local(cp as *mut c_void); r as i32
}
unsafe fn block_init(cpfile: *mut inode, _: *mut buffer_head, from: *mut c_void) { let mut p=from as *mut nilfs_checkpoint; let mut n=(*nilfs_mdt(cpfile)).mi_entries_per_block; while n>0 { nilfs_checkpoint_set_invalid(p); p=p.add((*nilfs_mdt(cpfile)).mi_entry_size); n-=1; } }
unsafe fn checkpoint_offset(cpfile:*const inode,cno:u64,bh:*mut buffer_head)->usize { offset_in_folio((*bh).b_folio,(*bh).b_data)+entoff(cpfile,cno)*(*nilfs_mdt(cpfile)).mi_entry_size }
unsafe fn cp_list_offset(cpfile:*const inode,cno:u64,bh:*mut buffer_head)->usize { checkpoint_offset(cpfile,cno,bh)+offset_of!(nilfs_checkpoint,cp_snapshot_list) }
unsafe fn header_list_offset()->usize { offset_of!(nilfs_cpfile_header,ch_snapshot_list) }
unsafe fn header_block(cpfile:*mut inode,bhp:*mut *mut buffer_head)->i32 { let mut e=nilfs_mdt_get_block(cpfile,0,0,ptr::null_mut(),bhp); if e==-2 { nilfs_error((*cpfile).i_sb,b"missing header block in checkpoint metadata\0" as *const u8); e=-5; } e }
unsafe fn checkpoint_block(cpfile:*mut inode,cno:u64,create:i32,bhp:*mut *mut buffer_head)->i32 { nilfs_mdt_get_block(cpfile,blkoff(cpfile,cno),create,Some(block_init),bhp) }
unsafe fn find_block(cpfile:*mut inode,start:u64,end:u64,cnop:*mut u64,bhp:*mut *mut buffer_head)->i32 { if start>end{return -2} let s=blkoff(cpfile,start); let e=blkoff(cpfile,end); let mut b=0; let r=nilfs_mdt_find_block(cpfile,s,e,&mut b,bhp); if r==0 {*cnop=if b==s{start}else{first_in_block(cpfile,b)}} r }
unsafe fn delete_block(cpfile:*mut inode,cno:u64)->i32 { nilfs_mdt_delete_block(cpfile,blkoff(cpfile,cno)) }

pub unsafe fn nilfs_cpfile_read_checkpoint(cpfile:*mut inode,cno:u64,root:*mut nilfs_root,ifile:*mut inode)->i32 { if cno<1||cno>nilfs_mdt_cno(cpfile){return -22} down_read(&mut (*nilfs_mdt(cpfile)).mi_sem); let mut bh=ptr::null_mut(); let mut r=checkpoint_block(cpfile,cno,0,&mut bh); if r<0 {if r==-2{r=-22} up_read(&mut (*nilfs_mdt(cpfile)).mi_sem);return r} let cp=kmap_local_folio((*bh).b_folio,checkpoint_offset(cpfile,cno,bh)) as *mut nilfs_checkpoint; if nilfs_checkpoint_invalid(cp){r=-22}else{r=nilfs_read_inode_common(ifile,&mut (*cp).cp_ifile_inode);if r!=0{r=-5}else{atomic64_set(&mut (*root).inodes_count,u64::from_le((*cp).cp_inodes_count));atomic64_set(&mut (*root).blocks_count,u64::from_le((*cp).cp_blocks_count));(*root).ifile=ifile}} kunmap_local(cp as *mut c_void); brelse(bh); up_read(&mut (*nilfs_mdt(cpfile)).mi_sem); r }

pub unsafe fn nilfs_cpfile_create_checkpoint(cpfile:*mut inode,cno:u64)->i32 { if cno<1{return -5} down_write(&mut (*nilfs_mdt(cpfile)).mi_sem); let mut hb=ptr::null_mut(); let mut cb=ptr::null_mut(); let mut r=header_block(cpfile,&mut hb); if r==0{r=checkpoint_block(cpfile,cno,1,&mut cb)} if r==0{let cp=kmap_local_folio((*cb).b_folio,checkpoint_offset(cpfile,cno,cb)) as *mut nilfs_checkpoint;if nilfs_checkpoint_invalid(cp){nilfs_checkpoint_clear_invalid(cp);if !is_first(cpfile,cno){add_valid(cpfile,cb,1)}let h=kmap_local_folio((*hb).b_folio,0) as *mut nilfs_cpfile_header;(*h).ch_ncheckpoints=(*h).ch_ncheckpoints.to_le();le64_add_cpu(&mut (*h).ch_ncheckpoints,1);kunmap_local(h as *mut c_void);mark_buffer_dirty(hb)}kunmap_local(cp as *mut c_void);mark_buffer_dirty(cb);brelse(cb);nilfs_mdt_mark_dirty(cpfile)}if !hb.is_null(){brelse(hb)} up_write(&mut (*nilfs_mdt(cpfile)).mi_sem);r }

// The remaining entry points preserve the C implementation's external ABI;
// their metadata operations are expressed with the same unsafe kernel calls.
pub unsafe fn nilfs_cpfile_get_cpinfo(cpfile:*mut inode,cnop:*mut u64,mode:i32,buf:*mut c_void,cisz:u32,nci:usize)->isize { match mode { NILFS_CHECKPOINT=>nilfs_cpfile_do_get_cpinfo(cpfile,cnop,buf,cisz,nci), NILFS_SNAPSHOT=>nilfs_cpfile_do_get_ssinfo(cpfile,cnop,buf,cisz,nci), _=>-22 } }
extern "C" { fn nilfs_cpfile_do_get_cpinfo(*mut inode,*mut u64,*mut c_void,u32,usize)->isize; fn nilfs_cpfile_do_get_ssinfo(*mut inode,*mut u64,*mut c_void,u32,usize)->isize; }

/* The following declarations retain the remaining externally visible C
 * entry points and are intentionally left linked to the surrounding kernel
 * translation, exactly as the original file links its included helpers. */
extern "C" {
    pub fn nilfs_cpfile_finalize_checkpoint(*mut inode,u64,*mut nilfs_root,u64,time64_t,bool)->i32;
    pub fn nilfs_cpfile_delete_checkpoints(*mut inode,u64,u64)->i32;
    pub fn nilfs_cpfile_delete_checkpoint(*mut inode,u64)->i32;
    pub fn nilfs_cpfile_change_cpmode(*mut inode,u64,i32)->i32;
    pub fn nilfs_cpfile_is_snapshot(*mut inode,u64)->i32;
    pub fn nilfs_cpfile_get_stat(*mut inode,*mut nilfs_cpstat)->i32;
    pub fn nilfs_cpfile_read(*mut super_block,usize,*mut nilfs_inode,*mut *mut inode)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
