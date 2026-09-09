/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of linux/fs/ufs/util.h. */

#[inline]
pub unsafe fn UCPI_UBH(cpi: *mut ufs_cg_private_info) -> *mut ufs_buffer_head { &mut (*cpi).c_ubh }
#[inline]
pub unsafe fn USPI_UBH(spi: *mut ufs_sb_private_info) -> *mut ufs_buffer_head { &mut (*spi).s_ubh }

#[inline]
pub unsafe fn ufs_get_fs_state(sb: *mut super_block, usb1: *mut ufs_super_block_first, usb3: *mut ufs_super_block_third) -> s32 {
    match (*UFS_SB(sb)).s_flags & UFS_ST_MASK {
        UFS_ST_SUNOS => { if fs32_to_cpu(sb, (*usb3).fs_postblformat) == UFS_42POSTBLFMT { return fs32_to_cpu(sb, (*usb1).fs_u0.fs_sun.fs_state); } fs32_to_cpu(sb, (*usb3).fs_un2.fs_sun.fs_state) },
        UFS_ST_SUN => fs32_to_cpu(sb, (*usb3).fs_un2.fs_sun.fs_state),
        UFS_ST_SUNx86 => fs32_to_cpu(sb, (*usb1).fs_u1.fs_sunx86.fs_state),
        _ => fs32_to_cpu(sb, (*usb3).fs_un2.fs_44.fs_state),
    }
}

#[inline]
pub unsafe fn ufs_set_fs_state(sb: *mut super_block, usb1: *mut ufs_super_block_first, usb3: *mut ufs_super_block_third, value: s32) {
    match (*UFS_SB(sb)).s_flags & UFS_ST_MASK {
        UFS_ST_SUNOS => { if fs32_to_cpu(sb, (*usb3).fs_postblformat) == UFS_42POSTBLFMT { (*usb1).fs_u0.fs_sun.fs_state = cpu_to_fs32(sb, value); } else { (*usb3).fs_un2.fs_sun.fs_state = cpu_to_fs32(sb, value); } },
        UFS_ST_SUN => (*usb3).fs_un2.fs_sun.fs_state = cpu_to_fs32(sb, value),
        UFS_ST_SUNx86 => (*usb1).fs_u1.fs_sunx86.fs_state = cpu_to_fs32(sb, value),
        UFS_ST_44BSD => (*usb3).fs_un2.fs_44.fs_state = cpu_to_fs32(sb, value),
        _ => (),
    }
}

#[inline]
pub unsafe fn ufs_get_fs_npsect(sb: *mut super_block, usb1: *mut ufs_super_block_first, usb3: *mut ufs_super_block_third) -> u32 {
    if (*UFS_SB(sb)).s_flags & UFS_ST_MASK == UFS_ST_SUNx86 { fs32_to_cpu(sb, (*usb3).fs_un2.fs_sunx86.fs_npsect) } else { fs32_to_cpu(sb, (*usb1).fs_u1.fs_sun.fs_npsect) }
}

#[inline]
pub unsafe fn ufs_get_fs_qbmask(sb: *mut super_block, usb3: *mut ufs_super_block_third) -> u64 {
    let mut tmp: __fs64 = core::mem::zeroed();
    let p = &mut tmp as *mut __fs64 as *mut __fs32;
    match (*UFS_SB(sb)).s_flags & UFS_ST_MASK { UFS_ST_SUNOS | UFS_ST_SUN => { *p = (*usb3).fs_un2.fs_sun.fs_qbmask[0]; *p.add(1) = (*usb3).fs_un2.fs_sun.fs_qbmask[1]; }, UFS_ST_SUNx86 => { *p = (*usb3).fs_un2.fs_sunx86.fs_qbmask[0]; *p.add(1) = (*usb3).fs_un2.fs_sunx86.fs_qbmask[1]; }, UFS_ST_44BSD => { *p = (*usb3).fs_un2.fs_44.fs_qbmask[0]; *p.add(1) = (*usb3).fs_un2.fs_44.fs_qbmask[1]; }, _ => () }
    fs64_to_cpu(sb, tmp)
}

#[inline]
pub unsafe fn ufs_get_fs_qfmask(sb: *mut super_block, usb3: *mut ufs_super_block_third) -> u64 {
    let mut tmp: __fs64 = core::mem::zeroed(); let p = &mut tmp as *mut __fs64 as *mut __fs32;
    match (*UFS_SB(sb)).s_flags & UFS_ST_MASK { UFS_ST_SUNOS | UFS_ST_SUN => { *p = (*usb3).fs_un2.fs_sun.fs_qfmask[0]; *p.add(1) = (*usb3).fs_un2.fs_sun.fs_qfmask[1]; }, UFS_ST_SUNx86 => { *p = (*usb3).fs_un2.fs_sunx86.fs_qfmask[0]; *p.add(1) = (*usb3).fs_un2.fs_sunx86.fs_qfmask[1]; }, UFS_ST_44BSD => { *p = (*usb3).fs_un2.fs_44.fs_qfmask[0]; *p.add(1) = (*usb3).fs_un2.fs_44.fs_qfmask[1]; }, _ => () }
    fs64_to_cpu(sb, tmp)
}

#[inline] pub unsafe fn ufs_get_de_namlen(sb: *mut super_block, de: *mut ufs_dir_entry) -> u16 { if (*UFS_SB(sb)).s_flags & UFS_DE_MASK == UFS_DE_OLD { fs16_to_cpu(sb, (*de).d_u.d_namlen) } else { (*de).d_u.d_44.d_namlen } }
#[inline] pub unsafe fn ufs_set_de_namlen(sb: *mut super_block, de: *mut ufs_dir_entry, value: u16) { if (*UFS_SB(sb)).s_flags & UFS_DE_MASK == UFS_DE_OLD { (*de).d_u.d_namlen = cpu_to_fs16(sb, value) } else { (*de).d_u.d_44.d_namlen = value } }
#[inline] pub unsafe fn ufs_set_de_type(sb: *mut super_block, de: *mut ufs_dir_entry, mode: i32) { if (*UFS_SB(sb)).s_flags & UFS_DE_MASK != UFS_DE_44BSD { return; } (*de).d_u.d_44.d_type = match mode & S_IFMT { S_IFSOCK=>DT_SOCK, S_IFLNK=>DT_LNK, S_IFREG=>DT_REG, S_IFBLK=>DT_BLK, S_IFDIR=>DT_DIR, S_IFCHR=>DT_CHR, S_IFIFO=>DT_FIFO, _=>DT_UNKNOWN }; }

#[inline] pub unsafe fn ufs_get_inode_uid(sb:*mut super_block, inode:*mut ufs_inode)->u32 { match (*UFS_SB(sb)).s_flags & UFS_UID_MASK { UFS_UID_44BSD=>fs32_to_cpu(sb,(*inode).ui_u3.ui_44.ui_uid), UFS_UID_EFT=>if (*inode).ui_u1.oldids.ui_suid==0xffff { fs32_to_cpu(sb,(*inode).ui_u3.ui_sun.ui_uid) } else { fs16_to_cpu(sb,(*inode).ui_u1.oldids.ui_suid) }, _=>fs16_to_cpu(sb,(*inode).ui_u1.oldids.ui_suid) } }
#[inline] pub unsafe fn ufs_set_inode_uid(sb:*mut super_block,inode:*mut ufs_inode,mut value:u32){match (*UFS_SB(sb)).s_flags&UFS_UID_MASK{UFS_UID_44BSD=>{(*inode).ui_u3.ui_44.ui_uid=cpu_to_fs32(sb,value);(*inode).ui_u1.oldids.ui_suid=cpu_to_fs16(sb,value)},UFS_UID_EFT=>{(*inode).ui_u3.ui_sun.ui_uid=cpu_to_fs32(sb,value);if value>0xffff{value=0xffff};(*inode).ui_u1.oldids.ui_suid=cpu_to_fs16(sb,value)},_=>{(*inode).ui_u1.oldids.ui_suid=cpu_to_fs16(sb,value)}}}
#[inline] pub unsafe fn ufs_get_inode_gid(sb:*mut super_block,inode:*mut ufs_inode)->u32{match (*UFS_SB(sb)).s_flags&UFS_UID_MASK{UFS_UID_44BSD=>fs32_to_cpu(sb,(*inode).ui_u3.ui_44.ui_gid),UFS_UID_EFT=>if (*inode).ui_u1.oldids.ui_sgid==0xffff{fs32_to_cpu(sb,(*inode).ui_u3.ui_sun.ui_gid)}else{fs16_to_cpu(sb,(*inode).ui_u1.oldids.ui_sgid)},_=>fs16_to_cpu(sb,(*inode).ui_u1.oldids.ui_sgid)}}
#[inline] pub unsafe fn ufs_set_inode_gid(sb:*mut super_block,inode:*mut ufs_inode,mut value:u32){match (*UFS_SB(sb)).s_flags&UFS_UID_MASK{UFS_UID_44BSD=>{(*inode).ui_u3.ui_44.ui_gid=cpu_to_fs32(sb,value);(*inode).ui_u1.oldids.ui_sgid=cpu_to_fs16(sb,value)},UFS_UID_EFT=>{(*inode).ui_u3.ui_sun.ui_gid=cpu_to_fs32(sb,value);if value>0xffff{value=0xffff};(*inode).ui_u1.oldids.ui_sgid=cpu_to_fs16(sb,value)},_=>{(*inode).ui_u1.oldids.ui_sgid=cpu_to_fs16(sb,value)}}}

extern "C" { pub fn ufs_get_inode_dev(sb:*mut super_block, inode:*mut ufs_inode_info)->dev_t; pub fn ufs_set_inode_dev(sb:*mut super_block,inode:*mut ufs_inode_info,dev:dev_t); pub fn ufs_prepare_chunk(folio:*mut folio,pos:loff_t,len:usize)->i32; pub fn _ubh_bread_(uspi:*mut ufs_sb_private_info,sb:*mut super_block,fragment:u64,size:u64)->*mut ufs_buffer_head; pub fn ubh_bread_uspi(uspi:*mut ufs_sb_private_info,sb:*mut super_block,fragment:u64,size:u64)->*mut ufs_buffer_head; pub fn ubh_brelse(ubh:*mut ufs_buffer_head); pub fn ubh_brelse_uspi(uspi:*mut ufs_sb_private_info); pub fn ubh_mark_buffer_dirty(ubh:*mut ufs_buffer_head); pub fn ubh_sync_block(ubh:*mut ufs_buffer_head); pub fn ubh_bforget(ubh:*mut ufs_buffer_head); pub fn ubh_buffer_dirty(ubh:*mut ufs_buffer_head)->i32; pub fn ufs_get_locked_folio(mapping:*mut address_space,index:pgoff_t)->*mut folio; }

#[inline] pub unsafe fn ufs_put_locked_folio(folio:*mut folio){folio_unlock(folio);folio_put(folio)}
#[inline] pub unsafe fn get_usb_offset(uspi:*mut ufs_sb_private_info,mut offset:u32)->*mut u8{let index=offset>>(*uspi).s_fshift;offset&= !(*uspi).s_fmask;(*(*uspi).s_ubh.bh.add(index as usize)).b_data.add(offset as usize)}
#[inline] pub unsafe fn ufs_freefrags(uspi:*mut ufs_sb_private_info)->u64{ufs_blkstofrags((*uspi).cs_total.cs_nbfree)+(*uspi).cs_total.cs_nffree as u64}

#[inline] pub unsafe fn _ubh_find_next_zero_bit_(uspi:*mut ufs_sb_private_info,ubh:*mut ufs_buffer_head,mut begin:u32,mut size:u32,mut offset:u32)->u32{size-=offset;begin<<=3;offset+=begin;let mut base=offset>>(*uspi).s_bpfshift;offset&=(*uspi).s_bpfmask;loop{let count=core::cmp::min(size+offset,(*uspi).s_bpf);size-=count-offset;let pos=find_next_zero_bit_le((*(*ubh).bh.add(base as usize)).b_data,count,offset);if pos<count||size==0{return (base<<(*uspi).s_bpfshift)+pos-begin}base+=1;offset=0}}
#[inline] pub unsafe fn find_last_zero_bit(bitmap:*mut u8,size:u32,offset:u32)->u32{let mut mapp=bitmap.add((size>>3)as usize);let mut map=*mapp;mapp=mapp.sub(1);let mut bit=1u8<<(size&7);let mut i=size;while i>offset{if map&bit==0{break}if i&7!=0{bit>>=1}else{map=*mapp;mapp=mapp.sub(1);bit=0x80}i-=1}i}
#[inline] pub unsafe fn _ubh_find_last_zero_bit_(uspi:*mut ufs_sb_private_info,ubh:*mut ufs_buffer_head,mut begin:u32,start:u32,end:u32)->u32{let mut size=start-end;begin<<=3;let mut start=start+begin;let mut base=start>>(*uspi).s_bpfshift;start&=(*uspi).s_bpfmask;loop{let count=core::cmp::min(size+(*uspi).s_bpf-start,(*uspi).s_bpf)-((*uspi).s_bpf-start);size-=count;let pos=find_last_zero_bit((*(*ubh).bh.add(base as usize)).b_data,start,start-count);if pos>start-count||size==0{return (base<<(*uspi).s_bpfshift)+pos-begin}base-=1;start=(*uspi).s_bpf}}

#[inline] pub unsafe fn ubh_isblockset(uspi:*mut ufs_sb_private_info,ucpi:*mut ufs_cg_private_info,frag:u32)->i32{let p=ubh_get_addr(UCPI_UBH(ucpi),(*ucpi).c_freeoff+(frag>>3),uspi);match (*uspi).s_fpb{8=>if *p==0xff{1}else{0},4=>if(*p&(0x0f<<((frag&4)as u8)))==(0x0f<<((frag&4)as u8)){1}else{0},2=>if(*p&(3<<((frag&6)as u8)))==(3<<((frag&6)as u8)){1}else{0},1=>if(*p&(1<<((frag&7)as u8)))!=0{1}else{0},_=>0}}
#[inline] pub unsafe fn ubh_clrblock(uspi:*mut ufs_sb_private_info,ucpi:*mut ufs_cg_private_info,frag:u32){let p=ubh_get_addr(UCPI_UBH(ucpi),(*ucpi).c_freeoff+(frag>>3),uspi);match (*uspi).s_fpb{8=>*p=0,4=>*p&=!(0x0f<<((frag&4)as u8)),2=>*p&=!(3<<((frag&6)as u8)),1=>*p&=!(1<<((frag&7)as u8)),_=>()}}
#[inline] pub unsafe fn ubh_setblock(uspi:*mut ufs_sb_private_info,ucpi:*mut ufs_cg_private_info,frag:u32){let p=ubh_get_addr(UCPI_UBH(ucpi),(*ucpi).c_freeoff+(frag>>3),uspi);match (*uspi).s_fpb{8=>*p=0xff,4=>*p|=0x0f<<((frag&4)as u8),2=>*p|=3<<((frag&6)as u8),1=>*p|=1<<((frag&7)as u8),_=>()}}

#[inline] pub unsafe fn ufs_fragacct(sb:*mut super_block,blockmap:u32,fraglist:*mut __fs32,cnt:i32){let uspi=(*UFS_SB(sb)).s_uspi;let mut fragsize=0;for pos in 0..(*uspi).s_fpb{if blockmap&(1<<pos)!=0{fragsize+=1}else if fragsize>0{fs32_add(sb,fraglist.add(fragsize as usize),cnt);fragsize=0}}if fragsize>0&&fragsize<(*uspi).s_fpb{fs32_add(sb,fraglist.add(fragsize as usize),cnt)}}
#[inline] pub unsafe fn ufs_get_direct_data_ptr(uspi:*mut ufs_sb_private_info,ufsi:*mut ufs_inode_info,blk:u32)->*mut core::ffi::c_void{BUG_ON(blk>UFS_TIND_BLOCK);if(*uspi).fs_magic==UFS2_MAGIC{&mut(*ufsi).i_u1.u2_i_data[blk as usize] as *mut _ as *mut _}else{&mut(*ufsi).i_u1.i_data[blk as usize] as *mut _ as *mut _}}
#[inline] pub unsafe fn ufs_data_ptr_to_cpu(sb:*mut super_block,p:*mut core::ffi::c_void)->u64{if(*(*UFS_SB(sb)).s_uspi).fs_magic==UFS2_MAGIC{fs64_to_cpu(sb,*(p as *mut __fs64))}else{fs32_to_cpu(sb,*(p as *mut __fs32)) as u64}}
#[inline] pub unsafe fn ufs_cpu_to_data_ptr(sb:*mut super_block,p:*mut core::ffi::c_void,val:u64){if(*(*UFS_SB(sb)).s_uspi).fs_magic==UFS2_MAGIC{*(p as *mut __fs64)=cpu_to_fs64(sb,val)}else{*(p as *mut __fs32)=cpu_to_fs32(sb,val as u32)}}
#[inline] pub unsafe fn ufs_data_ptr_clear(uspi:*mut ufs_sb_private_info,p:*mut core::ffi::c_void){if(*uspi).fs_magic==UFS2_MAGIC{*(p as *mut __fs64)=0}else{*(p as *mut __fs32)=0}}
#[inline] pub unsafe fn ufs_is_data_ptr_zero(uspi:*mut ufs_sb_private_info,p:*mut core::ffi::c_void)->i32{if(*uspi).fs_magic==UFS2_MAGIC{(*(p as *mut __fs64)==0)as i32}else{(*(p as *mut __fs32)==0)as i32}}
#[inline] pub unsafe fn ufs_get_seconds(sbp:*mut super_block)->__fs32{let now=ktime_get_real_seconds();cpu_to_fs32(sbp,(now as u64&0xffff_ffff)as u32)}

// The following helpers correspond to the header's macros and retain their pointer-based semantics.
#[inline] pub unsafe fn ubh_get_addr(ubh:*mut ufs_buffer_head,begin:u32,uspi:*mut ufs_sb_private_info)->*mut u8{(*(*ubh).bh.add((begin>>(*uspi).s_fshift)as usize)).b_data.add((begin& !(*uspi).s_fmask)as usize)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
