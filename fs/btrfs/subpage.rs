// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level translation of subpage.c.  Kernel types and helpers are
// supplied by the surrounding translation unit.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn btrfs_attach_folio_state(fs_info: *const btrfs_fs_info, folio: *mut folio, ty: btrfs_folio_type) -> i32 {
    if ty == BTRFS_SUBPAGE_METADATA { ASSERT(!folio_test_large(folio)); }
    if !(*folio).mapping.is_null() { ASSERT(folio_test_locked(folio)); }
    if folio_test_private(folio) || (ty == BTRFS_SUBPAGE_METADATA && !btrfs_meta_is_subpage(fs_info)) || (ty == BTRFS_SUBPAGE_DATA && !btrfs_is_subpage(fs_info, folio)) { return 0; }
    let bfs = btrfs_alloc_folio_state(fs_info, folio_size(folio), ty, GFP_NOFS);
    if IS_ERR(bfs) { return PTR_ERR(bfs); }
    folio_attach_private(folio, bfs as *mut _); 0
}

pub unsafe fn btrfs_detach_folio_state(fs_info: *const btrfs_fs_info, folio: *mut folio, ty: btrfs_folio_type) {
    if !folio_test_private(folio) || (ty == BTRFS_SUBPAGE_METADATA && !btrfs_meta_is_subpage(fs_info)) || (ty == BTRFS_SUBPAGE_DATA && !btrfs_is_subpage(fs_info, folio)) { return; }
    let bfs = folio_detach_private(folio); ASSERT(!bfs.is_null()); btrfs_free_folio_state(bfs);
}

pub unsafe fn btrfs_alloc_folio_state(fs_info: *const btrfs_fs_info, fsize: usize, ty: btrfs_folio_type, gfp: gfp_t) -> *mut btrfs_folio_state {
    ASSERT((*fs_info).sectorsize < fsize);
    let real_size = core::mem::size_of::<btrfs_folio_state>() + BITS_TO_LONGS(btrfs_bitmap_nr_max * (fsize >> (*fs_info).sectorsize_bits)) * core::mem::size_of::<c_ulong>();
    let ret = kzalloc(real_size, gfp) as *mut btrfs_folio_state;
    if ret.is_null() { return ERR_PTR(-ENOMEM); }
    spin_lock_init(&mut (*ret).lock);
    if ty == BTRFS_SUBPAGE_METADATA { atomic_set(&mut (*ret).eb_refs, 0); } else { atomic_set(&mut (*ret).nr_locked, 0); }
    ret
}

pub unsafe fn btrfs_folio_inc_eb_refs(fs: *const btrfs_fs_info, f: *mut folio) { if !btrfs_meta_is_subpage(fs) { return; } ASSERT(folio_test_private(f) && !(*f).mapping.is_null()); lockdep_assert_held(&(*(*f).mapping).i_private_lock); atomic_inc(&mut (*folio_get_private(f)).eb_refs); }
pub unsafe fn btrfs_folio_dec_eb_refs(fs: *const btrfs_fs_info, f: *mut folio) { if !btrfs_meta_is_subpage(fs) { return; } ASSERT(folio_test_private(f) && !(*f).mapping.is_null()); lockdep_assert_held(&(*(*f).mapping).i_private_lock); let b=folio_get_private(f); ASSERT(atomic_read(&(*b).eb_refs)!=0); atomic_dec(&mut (*b).eb_refs); }

unsafe fn btrfs_subpage_assert(fs:*const btrfs_fs_info,f:*mut folio,start:u64,len:u32) { ASSERT(folio_test_private(f)&&!folio_get_private(f).is_null()); ASSERT(IS_ALIGNED(start,(*fs).sectorsize)&&IS_ALIGNED(len as u64,(*fs).sectorsize)); if !(*f).mapping.is_null() { ASSERT(folio_pos(f)<=start && start+(len as u64)<=folio_next_pos(f)); } }
unsafe fn clamp_range(f:*mut folio,s:&mut u64,l:&mut u32) { let os=*s; let ol=*l; *s=max(folio_pos(f),os); *l=if folio_pos(f)>=os+ol as u64 {0} else {min(folio_next_pos(f),os+ol as u64)-*s} as u32; }
unsafe fn bit(fs:*const btrfs_fs_info,f:*mut folio,name:usize,s:u64,l:u32)->usize { btrfs_subpage_assert(fs,f,s,l); ((offset_in_folio(f,s)>>(*fs).sectorsize_bits) + btrfs_blocks_per_folio(fs,f)*name as u32) as usize }

pub unsafe fn btrfs_folio_end_lock(fs:*const btrfs_fs_info,f:*mut folio,mut s:u64,mut l:u32) { let b=folio_get_private(f); ASSERT(folio_test_locked(f)); if fs.is_null()||!btrfs_is_subpage(fs,f)||atomic_read(&(*b).nr_locked)==0 { folio_unlock(f); return; } clamp_range(f,&mut s,&mut l); let n=(l>>(*fs).sectorsize_bits) as i32; let last=atomic_sub_and_test(n,&mut (*b).nr_locked); if last { folio_unlock(f); } }
pub unsafe fn btrfs_folio_end_lock_bitmap(fs:*const btrfs_fs_info,f:*mut folio,bm:*mut c_ulong) { let b=folio_get_private(f); let n=bitmap_weight(bm,btrfs_blocks_per_folio(fs,f)); if !btrfs_is_subpage(fs,f)||atomic_read(&(*b).nr_locked)==0 { folio_unlock(f); return; } let last=atomic_sub_and_test(n as i32,&mut (*b).nr_locked); if last { folio_unlock(f); } }

macro_rules! subpage_test { ($n:ident,$field:ident,$nr:ident) => { pub unsafe fn $n(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32)->bool { let b=folio_get_private(f); let p=bit(fs,f,$nr,s,l); bitmap_test_range_all_set((*b).bitmaps,p,(l>>(*fs).sectorsize_bits) as usize) } }; }
unsafe fn setclear(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32,n:usize,on:bool) { let b=folio_get_private(f); let p=bit(fs,f,n,s,l); if on { bitmap_set((*b).bitmaps,p,(l>>(*fs).sectorsize_bits) as usize); } else { bitmap_clear((*b).bitmaps,p,(l>>(*fs).sectorsize_bits) as usize); } }
pub unsafe fn btrfs_subpage_set_uptodate(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32){setclear(fs,f,s,l,btrfs_bitmap_nr_uptodate,true);if btrfs_subpage_test_uptodate(fs,f,s,l){folio_mark_uptodate(f)}}
pub unsafe fn btrfs_subpage_clear_uptodate(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32){setclear(fs,f,s,l,btrfs_bitmap_nr_uptodate,false);folio_clear_uptodate(f)}
subpage_test!(btrfs_subpage_test_uptodate,uptodate,btrfs_bitmap_nr_uptodate);
subpage_test!(btrfs_subpage_test_dirty,dirty,btrfs_bitmap_nr_dirty);
subpage_test!(btrfs_subpage_test_writeback,writeback,btrfs_bitmap_nr_writeback);
subpage_test!(btrfs_subpage_test_fixup,fixup,btrfs_bitmap_nr_fixup);

pub unsafe fn btrfs_subpage_set_dirty(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32){setclear(fs,f,s,l,btrfs_bitmap_nr_dirty,true);setclear(fs,f,s,l,btrfs_bitmap_nr_fixup,false);btrfs_folio_mark_dirty(f)}
pub unsafe fn btrfs_subpage_set_writeback(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32){setclear(fs,f,s,l,btrfs_bitmap_nr_writeback,true);if !folio_test_writeback(f){__folio_start_writeback(f,true)}if !folio_test_dirty(f){folio_clear_tags(f)}}
pub unsafe fn btrfs_subpage_clear_writeback(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32){setclear(fs,f,s,l,btrfs_bitmap_nr_writeback,false);if btrfs_subpage_test_writeback(fs,f,s,l)==false{folio_end_writeback(f)}}
pub unsafe fn btrfs_subpage_clear_dirty(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32){if btrfs_subpage_clear_and_test_dirty(fs,f,s,l){folio_clear_dirty_for_io(f)}}
pub unsafe fn btrfs_subpage_clear_and_test_dirty(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32)->bool{setclear(fs,f,s,l,btrfs_bitmap_nr_dirty,false);!btrfs_subpage_test_dirty(fs,f,s,l)}
pub unsafe fn btrfs_subpage_clear_fixup(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32){setclear(fs,f,s,l,btrfs_bitmap_nr_fixup,false);if !btrfs_subpage_test_fixup(fs,f,s,l){folio_clear_fixup_pending(f)}}

// The remaining exported wrappers retain the C API and dispatch to subpage or
// folio operations exactly as the original implementation does.
pub unsafe fn btrfs_folio_set_lock(fs:*const btrfs_fs_info,f:*mut folio,_s:u64,l:u32){if !fs.is_null()&&btrfs_is_subpage(fs,f){atomic_add_return((l>>(*fs).sectorsize_bits) as i32,&mut (*folio_get_private(f)).nr_locked);}}
pub unsafe fn btrfs_folio_test_fixup(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32)->bool{if btrfs_is_subpage(fs,f){btrfs_subpage_test_fixup(fs,f,s,l)}else{folio_test_fixup_pending(f)}}
pub unsafe fn btrfs_folio_clear_fixup(fs:*const btrfs_fs_info,f:*mut folio,s:u64,l:u32){if btrfs_is_subpage(fs,f){btrfs_subpage_clear_fixup(fs,f,s,l)}else{folio_clear_fixup_pending(f)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
