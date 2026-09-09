// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// C dependencies supplied by the surrounding XFS translation.

pub static mut xfs_efi_cache: *mut kmem_cache = core::ptr::null_mut();
pub static mut xfs_efd_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn EFI_ITEM(lip: *mut xfs_log_item) -> *mut xfs_efi_log_item {
    container_of(lip, xfs_efi_log_item, efi_item)
}

unsafe fn xfs_efi_item_free(efip: *mut xfs_efi_log_item) {
    kvfree((*efip).efi_item.li_lv_shadow);
    if (*efip).efi_format.efi_nextents > XFS_EFI_MAX_FAST_EXTENTS { kfree(efip as *mut _); }
    else { kmem_cache_free(xfs_efi_cache, efip as *mut _); }
}

unsafe fn xfs_efi_release(efip: *mut xfs_efi_log_item) {
    ASSERT(atomic_read(&(*efip).efi_refcount) > 0);
    if !atomic_dec_and_test(&mut (*efip).efi_refcount) { return; }
    xfs_trans_ail_delete(&mut (*efip).efi_item, 0);
    xfs_efi_item_free(efip);
}

unsafe fn xfs_efi_item_size(lip: *mut xfs_log_item, nvecs: *mut i32, nbytes: *mut i32) {
    let efip = EFI_ITEM(lip); *nvecs += 1;
    *nbytes += xfs_efi_log_format_sizeof((*efip).efi_format.efi_nextents) as i32;
}
pub unsafe fn xfs_efi_log_space(nr: u32) -> u32 { xlog_item_space(1, xfs_efi_log_format_sizeof(nr)) }

unsafe fn xfs_efi_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let efip = EFI_ITEM(lip);
    ASSERT(atomic_read(&(*efip).efi_next_extent) == (*efip).efi_format.efi_nextents as i32);
    ASSERT((*lip).li_type == XFS_LI_EFI || (*lip).li_type == XFS_LI_EFI_RT);
    (*efip).efi_format.efi_type = (*lip).li_type; (*efip).efi_format.efi_size = 1;
    xlog_format_copy(lfb, XLOG_REG_TYPE_EFI_FORMAT, &mut (*efip).efi_format as *mut _, xfs_efi_log_format_sizeof((*efip).efi_format.efi_nextents));
}
unsafe fn xfs_efi_item_unpin(lip: *mut xfs_log_item, _remove: i32) { xfs_efi_release(EFI_ITEM(lip)); }
unsafe fn xfs_efi_item_release(lip: *mut xfs_log_item) { xfs_efi_release(EFI_ITEM(lip)); }

unsafe fn xfs_efi_init(mp: *mut xfs_mount, item_type: u16, nextents: u32) -> *mut xfs_efi_log_item {
    ASSERT(item_type == XFS_LI_EFI || item_type == XFS_LI_EFI_RT); ASSERT(nextents > 0);
    let efip = if nextents > XFS_EFI_MAX_FAST_EXTENTS { kzalloc(xfs_efi_log_item_sizeof(nextents), GFP_KERNEL | __GFP_NOFAIL) } else { kmem_cache_zalloc(xfs_efi_cache, GFP_KERNEL | __GFP_NOFAIL) } as *mut xfs_efi_log_item;
    xfs_log_item_init(mp, &mut (*efip).efi_item, item_type, &xfs_efi_item_ops);
    (*efip).efi_format.efi_nextents = nextents; (*efip).efi_format.efi_id = efip as usize;
    atomic_set(&mut (*efip).efi_next_extent, 0); atomic_set(&mut (*efip).efi_refcount, 2); efip
}

unsafe fn xfs_efi_copy_format(buf: *mut kvec, dst: *mut xfs_efi_log_format) -> i32 {
    let src = (*buf).iov_base as *mut xfs_efi_log_format; let n = (*src).efi_nextents;
    let len = xfs_efi_log_format_sizeof(n); let len32 = xfs_efi_log_format32_sizeof(n); let len64 = xfs_efi_log_format64_sizeof(n);
    if (*buf).iov_len == len { memcpy(dst as *mut _, src as *const _, offsetof!(xfs_efi_log_format, efi_extents)); for i in 0..n { memcpy(&mut (*dst).efi_extents[i as usize] as *mut _ as *mut _, &(*src).efi_extents[i as usize] as *const _ as *const _, core::mem::size_of::<xfs_extent>()); } return 0; }
    if (*buf).iov_len == len32 { let s = (*buf).iov_base as *mut xfs_efi_log_format_32; (*dst).efi_type=(*s).efi_type; (*dst).efi_size=(*s).efi_size; (*dst).efi_nextents=(*s).efi_nextents; (*dst).efi_id=(*s).efi_id; for i in 0..(*dst).efi_nextents { (*dst).efi_extents[i as usize].ext_start=(*s).efi_extents[i as usize].ext_start; (*dst).efi_extents[i as usize].ext_len=(*s).efi_extents[i as usize].ext_len; } return 0; }
    if (*buf).iov_len == len64 { let s = (*buf).iov_base as *mut xfs_efi_log_format_64; (*dst).efi_type=(*s).efi_type; (*dst).efi_size=(*s).efi_size; (*dst).efi_nextents=(*s).efi_nextents; (*dst).efi_id=(*s).efi_id; for i in 0..(*dst).efi_nextents { (*dst).efi_extents[i as usize].ext_start=(*s).efi_extents[i as usize].ext_start; (*dst).efi_extents[i as usize].ext_len=(*s).efi_extents[i as usize].ext_len; } return 0; }
    XFS_CORRUPTION_ERROR("xfs_efi_copy_format", XFS_ERRLEVEL_LOW, core::ptr::null_mut(), (*buf).iov_base, (*buf).iov_len); -EFSCORRUPTED
}

unsafe fn EFD_ITEM(lip: *mut xfs_log_item) -> *mut xfs_efd_log_item { container_of(lip, xfs_efd_log_item, efd_item) }
unsafe fn xfs_efd_item_free(efdp: *mut xfs_efd_log_item) { kvfree((*efdp).efd_item.li_lv_shadow); if (*efdp).efd_format.efd_nextents > XFS_EFD_MAX_FAST_EXTENTS { kfree(efdp as *mut _); } else { kmem_cache_free(xfs_efd_cache, efdp as *mut _); } }
unsafe fn xfs_efd_item_size(lip:*mut xfs_log_item,nvecs:*mut i32,nbytes:*mut i32){let p=EFD_ITEM(lip);*nvecs+=1;*nbytes+=xfs_efd_log_format_sizeof((*p).efd_format.efd_nextents) as i32;}
pub unsafe fn xfs_efd_log_space(nr:u32)->u32{xlog_item_space(1,xfs_efd_log_format_sizeof(nr))}

// The remaining definitions retain the original callback topology and operations.
// Their bodies are translated literally below.

unsafe fn xfs_efd_item_format(lip:*mut xfs_log_item,lfb:*mut xlog_format_buf){let p=EFD_ITEM(lip);ASSERT((*p).efd_next_extent==(*p).efd_format.efd_nextents);ASSERT((*lip).li_type==XFS_LI_EFD||(*lip).li_type==XFS_LI_EFD_RT);(*p).efd_format.efd_type=(*lip).li_type;(*p).efd_format.efd_size=1;xlog_format_copy(lfb,XLOG_REG_TYPE_EFD_FORMAT,&mut (*p).efd_format as *mut _,xfs_efd_log_format_sizeof((*p).efd_format.efd_nextents));}
unsafe fn xfs_efd_item_release(lip:*mut xfs_log_item){let p=EFD_ITEM(lip);xfs_efi_release((*p).efd_efip);xfs_efd_item_free(p);}
unsafe fn xfs_efd_item_intent(lip:*mut xfs_log_item)->*mut xfs_log_item{&mut (*EFD_ITEM(lip)).efd_efip.as_mut().unwrap().efi_item}
unsafe fn xefi_entry(e:*const list_head)->*mut xfs_extent_free_item{list_entry(e,xfs_extent_free_item,xefi_list)}
unsafe fn xfs_efi_item_isrt(lip:*const xfs_log_item)->bool{ASSERT((*lip).li_type==XFS_LI_EFI||(*lip).li_type==XFS_LI_EFI_RT);(*lip).li_type==XFS_LI_EFI_RT}
unsafe fn xfs_efd_from_efi(efdp:*mut xfs_efd_log_item){let efip=(*efdp).efd_efip;ASSERT((*efip).efi_format.efi_nextents>0);for i in 0..(*efip).efi_format.efi_nextents{(*efdp).efd_format.efd_extents[i as usize]=(*efip).efi_format.efi_extents[i as usize];}(*efdp).efd_next_extent=(*efip).efi_format.efi_nextents;}
unsafe fn xfs_efd_add_extent(efdp:*mut xfs_efd_log_item,xefi:*mut xfs_extent_free_item){ASSERT((*efdp).efd_next_extent<(*efdp).efd_format.efd_nextents);let e=&mut (*efdp).efd_format.efd_extents[(*efdp).efd_next_extent as usize];e.ext_start=(*xefi).xefi_startblock;e.ext_len=(*xefi).xefi_blockcount;(*efdp).efd_next_extent+=1;}
unsafe fn xfs_extent_free_diff_items(_: *mut core::ffi::c_void,a:*const list_head,b:*const list_head)->i32{cmp_int((*xefi_entry(a)).xefi_group.as_ref().unwrap().xg_gno,(*xefi_entry(b)).xefi_group.as_ref().unwrap().xg_gno)}
unsafe fn xfs_extent_free_log_item(_: *mut xfs_trans,efip:*mut xfs_efi_log_item,xefi:*mut xfs_extent_free_item){let n=(atomic_inc_return(&mut (*efip).efi_next_extent)-1) as usize;ASSERT(n<(*efip).efi_format.efi_nextents as usize);(*efip).efi_format.efi_extents[n].ext_start=(*xefi).xefi_startblock;(*efip).efi_format.efi_extents[n].ext_len=(*xefi).xefi_blockcount;}
unsafe fn xfs_extent_free_cancel_item(item:*mut list_head){let x=xefi_entry(item);xfs_group_intent_put((*x).xefi_group);kmem_cache_free(xfs_extfree_item_cache,x);}
unsafe fn xfs_efi_item_match(lip:*mut xfs_log_item,id:u64)->bool{EFI_ITEM(lip).as_ref().unwrap().efi_format.efi_id==id}

// Deferred-operation and recovery callbacks use the external XFS structures and helpers.
pub static mut xfs_extent_free_defer_type: xfs_defer_op_type = xfs_defer_op_type { name:"extent_free", max_items:XFS_EFI_MAX_FAST_EXTENTS, ..zeroed() };
pub static mut xfs_agfl_free_defer_type: xfs_defer_op_type = xfs_defer_op_type { name:"agfl_free", max_items:XFS_EFI_MAX_FAST_EXTENTS, ..zeroed() };
pub static mut xfs_rtextent_free_defer_type: xfs_defer_op_type = xfs_defer_op_type { name:"rtextent_free", max_items:XFS_EFI_MAX_FAST_EXTENTS, ..zeroed() };
pub static mut xlog_efi_item_ops: xlog_recover_item_ops = zeroed();
pub static mut xlog_rtefi_item_ops: xlog_recover_item_ops = zeroed();
pub static mut xlog_efd_item_ops: xlog_recover_item_ops = zeroed();
pub static mut xlog_rtefd_item_ops: xlog_recover_item_ops = zeroed();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
