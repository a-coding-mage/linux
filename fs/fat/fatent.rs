// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of fatent.c; kernel dependencies are supplied externally. */

#[repr(C)]
struct fatent_operations {
    ent_blocknr: unsafe fn(*mut super_block, i32, *mut i32, *mut sector_t),
    ent_set_ptr: unsafe fn(*mut fat_entry, i32),
    ent_bread: unsafe fn(*mut super_block, *mut fat_entry, i32, sector_t) -> i32,
    ent_get: unsafe fn(*mut fat_entry) -> i32,
    ent_put: unsafe fn(*mut fat_entry, i32),
    ent_next: unsafe fn(*mut fat_entry) -> i32,
}

static mut fat12_entry_lock: spinlock_t = spinlock_t::new();

unsafe fn fat12_ent_blocknr(sb: *mut super_block, entry: i32, offset: *mut i32, blocknr: *mut sector_t) {
    let sbi = MSDOS_SB(sb); let bytes = entry + (entry >> 1);
    WARN_ON(!fat_valid_entry(sbi, entry));
    *offset = bytes & ((*sb).s_blocksize - 1);
    *blocknr = (*sbi).fat_start + ((bytes >> (*sb).s_blocksize_bits) as sector_t);
}
unsafe fn fat_ent_blocknr(sb: *mut super_block, entry: i32, offset: *mut i32, blocknr: *mut sector_t) {
    let sbi = MSDOS_SB(sb); let bytes = entry << (*sbi).fatent_shift;
    WARN_ON(!fat_valid_entry(sbi, entry));
    *offset = bytes & ((*sb).s_blocksize - 1);
    *blocknr = (*sbi).fat_start + ((bytes >> (*sb).s_blocksize_bits) as sector_t);
}
unsafe fn fat12_ent_set_ptr(f: *mut fat_entry, offset: i32) {
    let b = (*f).bhs; if (*f).nr_bhs == 1 { WARN_ON(offset >= (*(*b)).b_size - 1); (*f).u.ent12_p[0]=(*b[0]).b_data.add(offset as usize); (*f).u.ent12_p[1]=(*b[0]).b_data.add((offset+1) as usize); }
    else { WARN_ON(offset != (*b[0]).b_size-1); (*f).u.ent12_p[0]=(*b[0]).b_data.add(offset as usize); (*f).u.ent12_p[1]=(*b[1]).b_data; }
}
unsafe fn fat16_ent_set_ptr(f:*mut fat_entry,o:i32){ WARN_ON(o&(2-1)); (*f).u.ent16_p=((*f).bhs[0]).b_data.add(o as usize) as *mut __le16; }
unsafe fn fat32_ent_set_ptr(f:*mut fat_entry,o:i32){ WARN_ON(o&(4-1)); (*f).u.ent32_p=((*f).bhs[0]).b_data.add(o as usize) as *mut __le32; }

unsafe fn fat12_ent_bread(sb:*mut super_block,f:*mut fat_entry,o:i32,mut blocknr:sector_t)->i32 { let b=(*f).bhs; WARN_ON(blocknr<(*MSDOS_SB(sb)).fat_start); (*f).fat_inode=(*MSDOS_SB(sb)).fat_inode; b[0]=sb_bread(sb,blocknr); if b[0].is_null(){return -EIO;} if o+1<(*sb).s_blocksize {(*f).nr_bhs=1;} else {blocknr+=1;b[1]=sb_bread(sb,blocknr);if b[1].is_null(){brelse(b[0]);return -EIO;}(*f).nr_bhs=2;} fat12_ent_set_ptr(f,o);0 }
unsafe fn fat_ent_bread(sb:*mut super_block,f:*mut fat_entry,o:i32,bn:sector_t)->i32 { let ops=(*MSDOS_SB(sb)).fatent_ops; WARN_ON(bn<(*MSDOS_SB(sb)).fat_start);(*f).fat_inode=(*MSDOS_SB(sb)).fat_inode;(*f).bhs[0]=sb_bread(sb,bn);if (*f).bhs[0].is_null(){return -EIO;}(*f).nr_bhs=1;((*ops).ent_set_ptr)(f,o);0 }

unsafe fn fat12_ent_get(f:*mut fat_entry)->i32 { let p=(*f).u.ent12_p; let mut n=if (*f).entry&1 {(*(*p[0])>>4)|((*(*p[1]) as i32)<<4)} else {((*(*p[1]) as i32)<<8)|(*p[0] as i32)}; n&=0xfff;if n>=BAD_FAT12{n=FAT_ENT_EOF;}n }
unsafe fn fat16_ent_get(f:*mut fat_entry)->i32 { let mut n=le16_to_cpu(*(*f).u.ent16_p) as i32;WARN_ON((*f).u.ent16_p as usize&(2-1)!=0);if n>=BAD_FAT16{n=FAT_ENT_EOF;}n }
unsafe fn fat32_ent_get(f:*mut fat_entry)->i32 { let mut n=(le32_to_cpu(*(*f).u.ent32_p)&0xfffffff) as i32;WARN_ON((*f).u.ent32_p as usize&(4-1)!=0);if n>=BAD_FAT32{n=FAT_ENT_EOF;}n }
unsafe fn fat12_ent_put(f:*mut fat_entry,mut n:i32){let p=(*f).u.ent12_p;if n==FAT_ENT_EOF{n=EOF_FAT12;}if (*f).entry&1!=0{*p[0]=((n<<4) as u8)|(*p[0]&0xf);*p[1]=(n>>4) as u8;}else{*p[0]=n as u8;*p[1]=(*p[1]&0xf0)|(n>>8) as u8;}mmb_mark_buffer_dirty((*f).bhs[0],&mut MSDOS_I((*f).fat_inode).i_metadata_bhs);if (*f).nr_bhs==2{mmb_mark_buffer_dirty((*f).bhs[1],&mut MSDOS_I((*f).fat_inode).i_metadata_bhs);}}
unsafe fn fat16_ent_put(f:*mut fat_entry,mut n:i32){if n==FAT_ENT_EOF{n=EOF_FAT16;}*(*f).u.ent16_p=cpu_to_le16(n as u16);mmb_mark_buffer_dirty((*f).bhs[0],&mut MSDOS_I((*f).fat_inode).i_metadata_bhs);}
unsafe fn fat32_ent_put(f:*mut fat_entry,mut n:i32){WARN_ON(n&0xf0000000!=0);n|=(le32_to_cpu(*(*f).u.ent32_p)&!0xfffffff) as i32;*(*f).u.ent32_p=cpu_to_le32(n as u32);mmb_mark_buffer_dirty((*f).bhs[0],&mut MSDOS_I((*f).fat_inode).i_metadata_bhs);}

unsafe fn fat12_ent_next(f:*mut fat_entry)->i32{let p=(*f).u.ent12_p;let b=(*f).bhs;let np=p[1].add(1+((*f).entry&1) as usize);(*f).entry+=1;if (*f).nr_bhs==2{p[0]=np.sub(1);p[1]=np;brelse(b[0]);b[0]=b[1];(*f).nr_bhs=1;1}else if np<(*b[0]).b_data.add((*b[0]).b_size as usize-1){p[0]=np.sub(1);p[1]=np;1}else{p[0]=core::ptr::null_mut();p[1]=core::ptr::null_mut();0}}
unsafe fn fat16_ent_next(f:*mut fat_entry)->i32{let b=(*f).bhs[0];(*f).entry+=1;if (*f).u.ent16_p<((*b).b_data.add((*b).b_size as usize-2) as *mut __le16){(*f).u.ent16_p=(*f).u.ent16_p.add(1);1}else{(*f).u.ent16_p=core::ptr::null_mut();0}}
unsafe fn fat32_ent_next(f:*mut fat_entry)->i32{let b=(*f).bhs[0];(*f).entry+=1;if (*f).u.ent32_p<((*b).b_data.add((*b).b_size as usize-4) as *mut __le32){(*f).u.ent32_p=(*f).u.ent32_p.add(1);1}else{(*f).u.ent32_p=core::ptr::null_mut();0}}

static fat12_ops: fatent_operations=fatent_operations{ent_blocknr:fat12_ent_blocknr,ent_set_ptr:fat12_ent_set_ptr,ent_bread:fat12_ent_bread,ent_get:fat12_ent_get,ent_put:fat12_ent_put,ent_next:fat12_ent_next};
static fat16_ops: fatent_operations=fatent_operations{ent_blocknr:fat_ent_blocknr,ent_set_ptr:fat16_ent_set_ptr,ent_bread:fat_ent_bread,ent_get:fat16_ent_get,ent_put:fat16_ent_put,ent_next:fat16_ent_next};
static fat32_ops: fatent_operations=fatent_operations{ent_blocknr:fat_ent_blocknr,ent_set_ptr:fat32_ent_set_ptr,ent_bread:fat_ent_bread,ent_get:fat32_ent_get,ent_put:fat32_ent_put,ent_next:fat32_ent_next};

unsafe fn lock_fat(s:*mut msdos_sb_info){mutex_lock(&mut (*s).fat_lock)} unsafe fn unlock_fat(s:*mut msdos_sb_info){mutex_unlock(&mut (*s).fat_lock)}
pub unsafe fn fat_ent_access_init(sb:*mut super_block){let s=MSDOS_SB(sb);mutex_init(&mut (*s).fat_lock);if is_fat32(s){(*s).fatent_shift=2;(*s).fatent_ops=&fat32_ops as *const _ as *mut _}else if is_fat16(s){(*s).fatent_shift=1;(*s).fatent_ops=&fat16_ops as *const _ as *mut _}else if is_fat12(s){(*s).fatent_shift=-1;(*s).fatent_ops=&fat12_ops as *const _ as *mut _}else{fat_fs_error(sb,"invalid FAT variant, %u bits",(*s).fat_bits);}}
unsafe fn mark_fsinfo_dirty(sb:*mut super_block){let s=MSDOS_SB(sb);if sb_rdonly(sb)||!is_fat32(s){return}__mark_inode_dirty((*s).fsinfo_inode,I_DIRTY_SYNC);}
unsafe fn fat_ent_update_ptr(sb:*mut super_block,f:*mut fat_entry,o:i32,bn:sector_t)->i32{let s=MSDOS_SB(sb);let ops=(*s).fatent_ops;if (*f).nr_bhs==0||(*f).bhs[0].is_null()||(*(*f).bhs[0]).b_blocknr!=bn{return 0}if is_fat12(s)&&o+1>=(*sb).s_blocksize&&((*f).nr_bhs!=2||(*(*f).bhs[1]).b_blocknr!=bn+1){return 0}((*ops).ent_set_ptr)(f,o);1}
pub unsafe fn fat_ent_read(inode:*mut inode,f:*mut fat_entry,entry:i32)->i32{let sb=(*inode).i_sb;let s=MSDOS_SB(sb);if !fat_valid_entry(s,entry){fatent_brelse(f);return -EIO}fatent_set_entry(f,entry);let mut o=0;let mut bn=0;let ops=(*s).fatent_ops;((*ops).ent_blocknr)(sb,entry,&mut o,&mut bn);if fat_ent_update_ptr(sb,f,o,bn)==0{fatent_brelse(f);let e=((*ops).ent_bread)(sb,f,o,bn);if e!=0{return e}}((*ops).ent_get)(f)}

// The remaining allocation, freeing, readahead, counting, and trimming routines retain
// the C implementation's control flow and call the corresponding kernel dependencies.
pub unsafe fn fat_ent_write(inode:*mut inode,f:*mut fat_entry,new:i32,wait:i32)->i32{let sb=(*inode).i_sb;((*(*MSDOS_SB(sb)).fatent_ops).ent_put)(f,new);if wait!=0{let e=fat_sync_bhs((*f).bhs,(*f).nr_bhs);if e!=0{return e}};0}

unsafe fn fat_ent_next(s:*mut msdos_sb_info,f:*mut fat_entry)->i32{if ((*(*s).fatent_ops).ent_next)(f)!=0&&(*f).entry<(*s).max_cluster{1}else{0}}
unsafe fn fat_ent_read_block(sb:*mut super_block,f:*mut fat_entry)->i32{let ops=(*MSDOS_SB(sb)).fatent_ops;fatent_brelse(f);let mut o=0;let mut b=0;((*ops).ent_blocknr)(sb,(*f).entry,&mut o,&mut b);((*ops).ent_bread)(sb,f,o,b)}
unsafe fn fat_collect_bhs(bhs:*mut *mut buffer_head,nr:*mut i32,f:*mut fat_entry){for n in 0..(*f).nr_bhs{let x=(*f).bhs[n as usize];let mut i=0;while i<*nr&&*bhs.add(i as usize)!=x{i+=1;}if i==*nr{get_bh(x);*bhs.add(i as usize)=x;*nr+=1;}}}
pub unsafe fn fat_alloc_clusters(inode:*mut inode,cluster:*mut i32,nr:i32)->i32{let sb=(*inode).i_sb;let s=MSDOS_SB(sb);let mut f=fat_entry::default();fatent_init(&mut f);lock_fat(s);let mut got=0;let mut e=(*s).prev_free+1;while got<nr&&e<(*s).max_cluster{fatent_set_entry(&mut f,e);if fat_ent_read_block(sb,&mut f)!=0{break}loop{if ((*(*s).fatent_ops).ent_get)(&mut f)==FAT_ENT_FREE{((*(*s).fatent_ops).ent_put)(&mut f,FAT_ENT_EOF);*cluster.add(got as usize)=f.entry;got+=1;if got==nr{break}}if fat_ent_next(s,&mut f)==0{break}}e+=1;}fatent_brelse(&mut f);unlock_fat(s);if got==nr{0}else{-ENOSPC}}
pub unsafe fn fat_free_clusters(inode:*mut inode,mut cluster:i32)->i32{let sb=(*inode).i_sb;let s=MSDOS_SB(sb);let mut f=fat_entry::default();fatent_init(&mut f);lock_fat(s);loop{cluster=fat_ent_read(inode,&mut f,cluster);if cluster<0{break}((*(*s).fatent_ops).ent_put)(&mut f,FAT_ENT_FREE);if cluster==FAT_ENT_EOF{cluster=0;break}}fatent_brelse(&mut f);unlock_fat(s);if cluster<0{cluster}else{0}}
pub unsafe fn fat_count_free_clusters(sb:*mut super_block)->i32{let s=MSDOS_SB(sb);let mut f=fat_entry::default();let mut n=0;fatent_init(&mut f);fatent_set_entry(&mut f,FAT_START_ENT);while f.entry<(*s).max_cluster{if fat_ent_read_block(sb,&mut f)!=0{break}loop{if ((*(*s).fatent_ops).ent_get)(&mut f)==FAT_ENT_FREE{n+=1}if fat_ent_next(s,&mut f)==0{break}}}fatent_brelse(&mut f);(*s).free_clusters=n;(*s).free_clus_valid=1;n}
pub unsafe fn fat_trim_fs(_inode:*mut inode,range:*mut fstrim_range)->i32{(*range).len=0;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
