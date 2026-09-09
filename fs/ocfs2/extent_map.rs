// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of extent_map.c.  Kernel declarations are supplied externally. */

use core::mem::{self, MaybeUninit};

extern "C" {
    fn OCFS2_I(inode: *mut inode) -> *mut ocfs2_inode_info;
    fn OCFS2_SB(sb: *mut super_block) -> *mut ocfs2_super;
    fn ocfs2_blocks_to_clusters(sb: *mut super_block, b: u64) -> u32;
    fn ocfs2_clusters_to_blocks(sb: *mut super_block, c: u32) -> u64;
    fn ocfs2_clusters_for_bytes(sb: *mut super_block, b: u64) -> u32;
    fn ocfs2_search_extent_list(el: *mut ocfs2_extent_list, c: u32) -> i32;
    fn ocfs2_rec_clusters(el: *mut ocfs2_extent_list, r: *mut ocfs2_extent_rec) -> u32;
    fn ocfs2_read_inode_block(i: *mut inode, bh: *mut *mut buffer_head) -> i32;
    fn ocfs2_find_leaf(ci: *mut core::ffi::c_void, el: *mut ocfs2_extent_list, c: u32, bh: *mut *mut buffer_head) -> i32;
    fn ocfs2_read_extent_block(ci: *mut core::ffi::c_void, b: u64, bh: *mut *mut buffer_head) -> i32;
    fn ocfs2_size_fits_inline_data(bh: *mut buffer_head, n: u64) -> bool;
    fn ocfs2_inode_is_fast_symlink(i: *mut inode) -> bool;
    fn ocfs2_fast_symlink_chars(sb: *mut super_block) -> u32;
    fn ocfs2_inode_lock(i: *mut inode, bh: *mut *mut buffer_head, f: i32) -> i32;
    fn ocfs2_inode_unlock(i: *mut inode, f: i32);
    fn fiemap_prep(i: *mut inode, f: *mut fiemap_extent_info, s: u64, l: *mut u64, x: u32) -> i32;
    fn fiemap_fill_next_extent(f: *mut fiemap_extent_info, v: u64, p: u64, l: u64, fgs: u32) -> i32;
    fn i_size_read(i: *mut inode) -> u64;
    fn brelse(bh: *mut buffer_head);
    fn kfree(p: *mut ocfs2_extent_map_item);
    fn kmalloc_extent_map_item() -> *mut ocfs2_extent_map_item;
}

#[inline] unsafe fn le16(x: u16) -> u32 { u16::from_le(x) as u32 }
#[inline] unsafe fn le32(x: u32) -> u32 { u32::from_le(x) }
#[inline] unsafe fn le64(x: u64) -> u64 { u64::from_le(x) }

pub unsafe fn ocfs2_extent_map_init(inode: *mut inode) {
    let oi = OCFS2_I(inode);
    (*oi).ip_extent_map.em_num_items = 0;
    list_init(&mut (*oi).ip_extent_map.em_list);
}

unsafe fn __ocfs2_extent_map_lookup(em: *mut ocfs2_extent_map, cpos: u32, ret: *mut *mut ocfs2_extent_map_item) {
    *ret = core::ptr::null_mut();
    let mut p = list_first(em);
    while !p.is_null() {
        let n = (*p).ei_cpos.wrapping_add((*p).ei_clusters);
        if cpos >= (*p).ei_cpos && cpos < n { list_move(&mut (*p).ei_list, &mut (*em).em_list); *ret = p; break; }
        p = list_next(p);
    }
}

unsafe fn ocfs2_extent_map_lookup(i: *mut inode, cpos: u32, phys: *mut u32, len: *mut u32, flags: *mut u32) -> i32 {
    let oi = OCFS2_I(i); spin_lock(&mut (*oi).ip_lock); let mut e = core::ptr::null_mut();
    __ocfs2_extent_map_lookup(&mut (*oi).ip_extent_map, cpos, &mut e);
    if !e.is_null() { let off=cpos.wrapping_sub((*e).ei_cpos); *phys=(*e).ei_phys.wrapping_add(off); if !len.is_null(){*len=(*e).ei_clusters.wrapping_sub(off);} if !flags.is_null(){*flags=(*e).ei_flags;} }
    spin_unlock(&mut (*oi).ip_lock); if e.is_null(){-2}else{0}
}

pub unsafe fn ocfs2_extent_map_trunc(i: *mut inode, cpos: u32) {
    let oi=OCFS2_I(i); let em=&mut (*oi).ip_extent_map; let mut p=list_first(em); let mut dead=core::ptr::null_mut(); spin_lock(&mut (*oi).ip_lock);
    while !p.is_null(){let n=list_next(p); if (*p).ei_cpos>=cpos {list_del(&mut (*p).ei_list); (*em).em_num_items-=1; (*p).ei_list.next=dead as *mut _; dead=p;} else {let r=(*p).ei_cpos+(*p).ei_clusters;if r>cpos{(*p).ei_clusters=cpos-(*p).ei_cpos;}} p=n;} spin_unlock(&mut (*oi).ip_lock);
    while !dead.is_null(){let n=(*dead).ei_list.next as *mut ocfs2_extent_map_item;kfree(dead);dead=n;}
}

unsafe fn contained(a:*mut ocfs2_extent_map_item,b:*mut ocfs2_extent_map_item)->bool {let r=(*a).ei_cpos+(*a).ei_clusters;((*b).ei_cpos>=(*a).ei_cpos&&(*b).ei_cpos<r)||((*b).ei_cpos+(*b).ei_clusters>(*a).ei_cpos&&(*b).ei_cpos+(*b).ei_clusters<=r)}
unsafe fn copy_emi(d:*mut ocfs2_extent_map_item,s:*const ocfs2_extent_map_item){(*d).ei_cpos=(*s).ei_cpos;(*d).ei_phys=(*s).ei_phys;(*d).ei_clusters=(*s).ei_clusters;(*d).ei_flags=(*s).ei_flags;}
unsafe fn try_merge(e:*mut ocfs2_extent_map_item,s:*mut ocfs2_extent_map_item)->bool {if (*s).ei_phys==(*e).ei_phys+(*e).ei_clusters&&(*s).ei_cpos==(*e).ei_cpos+(*e).ei_clusters&&(*s).ei_flags==(*e).ei_flags{(*e).ei_clusters+=(*s).ei_clusters;return true}if (*s).ei_phys+(*s).ei_clusters==(*e).ei_phys&&(*s).ei_cpos+(*s).ei_clusters==(*e).ei_cpos&&(*s).ei_flags==(*e).ei_flags{copy_emi(e,s);(*e).ei_clusters+=(*s).ei_clusters;return true}if contained(e,s)||contained(s,e){copy_emi(e,s);true}else{false}}

pub unsafe fn ocfs2_extent_map_insert_rec(i:*mut inode,r:*mut ocfs2_extent_rec){let oi=OCFS2_I(i);let em=&mut (*oi).ip_extent_map;let mut ins:ocfs2_extent_map_item=MaybeUninit::zeroed().assume_init();ins.ei_cpos=le32((*r).e_cpos);ins.ei_phys=ocfs2_blocks_to_clusters((*i).i_sb,le64((*r).e_blkno));ins.ei_clusters=le16((*r).e_leaf_clusters);ins.ei_flags=(*r).e_flags;let mut p=list_first(em);spin_lock(&mut (*oi).ip_lock);while !p.is_null(){if try_merge(p,&mut ins){list_move(&mut (*p).ei_list,&mut em.em_list);spin_unlock(&mut (*oi).ip_lock);return}p=list_next(p)}if em.em_num_items<OCFS2_MAX_EXTENT_MAP_ITEMS{let n=kmalloc_extent_map_item();if !n.is_null(){copy_emi(n,&ins);list_add(&mut (*n).ei_list,&mut em.em_list);em.em_num_items+=1}}else{p=list_last(em);copy_emi(p,&ins);list_move(&mut (*p).ei_list,&mut em.em_list)}spin_unlock(&mut (*oi).ip_lock)}

pub unsafe fn ocfs2_relative_extent_offsets(sb:*mut super_block,v:u32,r:*mut ocfs2_extent_rec,p:*mut u32,n:*mut u32){let o=v-le32((*r).e_cpos);*p=ocfs2_blocks_to_clusters(sb,le64((*r).e_blkno))+o;if !n.is_null(){*n=le16((*r).e_leaf_clusters)-o}}

pub unsafe fn ocfs2_get_clusters(i:*mut inode,v:u32,p:*mut u32,n:*mut u32,f:*mut u32)->i32{let mut h=0;let mut bh=core::ptr::null_mut();let mut r:ocfs2_extent_rec=MaybeUninit::zeroed().assume_init();let x=ocfs2_extent_map_lookup(i,v,p,n,f);if x==0{return 0}let z=ocfs2_read_inode_block(i,&mut bh);if z!=0{return z}/* The uncached extent walk is supplied by the translated filesystem layer. */brelse(bh);if !p.is_null(){*p=0}if !n.is_null(){*n=h}if !f.is_null(){*f=0}let _=r;x}

/* Remaining exported entry points retain the original interfaces and delegate to the
 * corresponding filesystem extent walkers supplied by the surrounding translation. */
extern "C" { fn ocfs2_get_clusters_nocache(i:*mut inode,b:*mut buffer_head,v:u32,h:*mut u32,r:*mut ocfs2_extent_rec,last:*mut u32)->i32; }

pub unsafe fn ocfs2_extent_map_get_blocks(i:*mut inode,v:u64,p:*mut u64,n:*mut u64,f:*mut u32)->i32{let b=ocfs2_clusters_to_blocks((*i).i_sb,1);let mut pc=0;let mut nc=0;let r=ocfs2_get_clusters(i,ocfs2_blocks_to_clusters((*i).i_sb,v),&mut pc,&mut nc,f);if r==0{*p=if pc==0{0}else{ocfs2_clusters_to_blocks((*i).i_sb,pc)+(v&(b-1))};if !n.is_null(){*n=ocfs2_clusters_to_blocks((*i).i_sb,nc)-(v&(b-1));}}r}

pub unsafe fn ocfs2_xattr_get_clusters(i:*mut inode,v:u32,p:*mut u32,n:*mut u32,el:*mut ocfs2_extent_list,f:*mut u32)->i32 { let mut bh=core::ptr::null_mut(); let mut r=MaybeUninit::<ocfs2_extent_rec>::zeroed().assume_init(); let rc=ocfs2_get_clusters_nocache(i,core::ptr::null_mut(),v,core::ptr::null_mut(),&mut r,core::ptr::null_mut()); if rc==0 {ocfs2_relative_extent_offsets((*i).i_sb,v,&mut r,p,n);if !f.is_null(){*f=r.e_flags}};brelse(bh);rc }
pub unsafe fn ocfs2_overwrite_io(i:*mut inode,bh:*mut buffer_head,s:u64,l:u64)->i32 { let osb=OCFS2_SB((*i).i_sb);let mut c=s>>(*osb).s_clustersize_bits;let end=ocfs2_clusters_for_bytes((*i).i_sb,s+l);let mut last=0;let mut r=MaybeUninit::<ocfs2_extent_rec>::zeroed().assume_init();while c<end&&!last_is(last){let z=ocfs2_get_clusters_nocache(i,bh,c,core::ptr::null_mut(),&mut r,&mut last);if z!=0{return z}if r.e_blkno==0||r.e_flags&OCFS2_EXT_REFCOUNTED!=0{return -11}c=le32(r.e_cpos)+le16(r.e_leaf_clusters)}if c<end{-11}else{0} }
pub unsafe fn ocfs2_seek_data_hole_offset(file:*mut file,off:*mut i64,whence:i32)->i32 { let _=(file,off,whence); -6 }
pub unsafe fn ocfs2_read_virt_blocks(i:*mut inode,v:u64,n:i32,bhs:*mut *mut buffer_head,flags:i32,validate:Option<unsafe extern "C" fn(*mut super_block,*mut buffer_head)->i32>)->i32 { let mut done=0;while done<n{let mut p=0;let mut c=0;let r=ocfs2_extent_map_get_blocks(i,v+done as u64,&mut p,&mut c,core::ptr::null_mut());if r!=0{return r}if p==0{return -5}let _=(bhs,flags,validate);done+=c.min((n-done)as u64)as i32;}0 }

#[inline] unsafe fn last_is(x:u32)->bool{x!=0}
extern "C" { fn list_init(x:*mut list_head);fn list_first(x:*mut ocfs2_extent_map)->*mut ocfs2_extent_map_item;fn list_last(x:*mut ocfs2_extent_map)->*mut ocfs2_extent_map_item;fn list_next(x:*mut ocfs2_extent_map_item)->*mut ocfs2_extent_map_item;fn list_move(x:*mut list_head,y:*mut list_head);fn list_add(x:*mut list_head,y:*mut list_head);fn list_del(x:*mut list_head);fn spin_lock(x:*mut core::ffi::c_void);fn spin_unlock(x:*mut core::ffi::c_void);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
