/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Direct Rust translation of vma.h. External kernel symbols are dependencies. */

use core::ffi::c_void;

#[repr(C)] pub struct vm_area_struct { pub vm_start: c_ulong, pub vm_end: c_ulong, pub vm_pgoff: pgoff_t, pub vm_file: *mut file, pub vm_flags: vm_flags_t, pub vm_mm: *mut mm_struct, pub vm_page_prot: pgprot_t, pub vm_ops: *const vm_operations_struct, pub vm_private_data: *mut c_void, pub anon_vma: *mut anon_vma, pub vm_userfaultfd_ctx: vm_userfaultfd_ctx, pub __vm_anon_pgoff_hi: u64, pub __vm_anon_pgoff_lo: u32, }
#[repr(C)] pub struct vma_iterator { pub mas: ma_state }
#[repr(C)] pub struct ma_state { pub status: c_int, pub index: c_ulong, pub last: c_ulong }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct file;
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct anon_vma;
#[repr(C)] pub struct mempolicy;
#[repr(C)] pub struct anon_vma_name;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct vm_userfaultfd_ctx;
#[repr(C)] pub struct vm_area_desc { pub pgoff: pgoff_t, pub vm_file: *mut file, pub vma_flags: vma_flags_t, pub page_prot: pgprot_t, pub vm_ops: *const vm_operations_struct, pub private_data: *mut c_void }
#[repr(C)] pub struct vm_operations_struct;
#[repr(C)] pub struct vm_unmapped_area_info;

pub type c_ulong = usize; pub type c_int = i32; pub type size_t = usize;
pub type pgoff_t = usize; pub type vm_flags_t = usize; pub type vma_flags_t = usize; pub type pgprot_t = usize; pub type gfp_t = usize;
pub const VMA_MERGE_START: c_int = 0; pub const VMA_MERGE_ERROR_NOMEM: c_int = 1; pub const VMA_MERGE_NOMERGE: c_int = 2; pub const VMA_MERGE_SUCCESS: c_int = 3;

#[repr(C)] pub struct vma_prepare { pub vma: *mut vm_area_struct, pub adj_next: *mut vm_area_struct, pub file: *mut file, pub mapping: *mut address_space, pub anon_vma: *mut anon_vma, pub insert: *mut vm_area_struct, pub remove: *mut vm_area_struct, pub remove2: *mut vm_area_struct, pub skip_vma_uprobe: bool }
#[repr(C)] pub struct unlink_vma_file_batch { pub count: c_int, pub vmas: [*mut vm_area_struct; 8] }
#[repr(C)] pub struct vma_munmap_struct { pub vmi: *mut vma_iterator, pub vma: *mut vm_area_struct, pub prev: *mut vm_area_struct, pub next: *mut vm_area_struct, pub uf: *mut list_head, pub start: c_ulong, pub end: c_ulong, pub unmap_start: c_ulong, pub unmap_end: c_ulong, pub vma_count: c_int, pub unlock: bool, pub clear_ptes: bool, pub nr_pages: c_ulong, pub locked_vm: c_ulong, pub nr_accounted: c_ulong, pub exec_vm: c_ulong, pub stack_vm: c_ulong, pub data_vm: c_ulong }
#[repr(C)] pub struct vma_merge_struct { pub mm: *mut mm_struct, pub vmi: *mut vma_iterator, pub prev: *mut vm_area_struct, pub middle: *mut vm_area_struct, pub next: *mut vm_area_struct, pub target: *mut vm_area_struct, pub start: c_ulong, pub end: c_ulong, pub pgoff: pgoff_t, pub anon_pgoff: pgoff_t, pub vm_flags: vm_flags_t, pub file: *mut file, pub anon_vma: *mut anon_vma, pub policy: *mut mempolicy, pub uffd_ctx: vm_userfaultfd_ctx, pub anon_name: *mut anon_vma_name, pub state: c_int, pub copied_from: *mut vm_area_struct, pub just_expand: bool, pub give_up_on_oom: bool, pub skip_vma_uprobe: bool, pub __adjust_middle_start: bool, pub __adjust_next_start: bool, pub __remove_middle: bool, pub __remove_next: bool }
#[repr(C)] pub struct unmap_desc { pub mas: *mut ma_state, pub first: *mut vm_area_struct, pub pg_start: c_ulong, pub pg_end: c_ulong, pub vma_start: c_ulong, pub vma_end: c_ulong, pub tree_end: c_ulong, pub tree_reset: c_ulong, pub mm_wr_locked: bool }

extern "C" { static FIRST_USER_ADDRESS: c_ulong; static USER_PGTABLES_CEILING: c_ulong; static ULONG_MAX: c_ulong; static PAGE_SHIFT: u32; }
extern "C" { fn vma_iter_set(vmi: *mut vma_iterator, x: c_ulong); fn __mas_set_range(mas: *mut ma_state, index: c_ulong, last: c_ulong); fn mas_store_gfp(mas: *mut ma_state, vma: *mut vm_area_struct, gfp: gfp_t); fn mas_store_prealloc(mas: *mut ma_state, vma: *mut vm_area_struct); fn mas_reset(mas: *mut ma_state); fn mas_walk(mas: *mut ma_state) -> *mut vm_area_struct; fn mas_prev(mas: *mut ma_state, min: c_ulong) -> *mut vm_area_struct; fn mas_prev_range(mas: *mut ma_state, min: c_ulong) -> *mut vm_area_struct; fn mas_next_range(mas: *mut ma_state, max: c_ulong) -> *mut vm_area_struct; fn mas_empty_area(mas: *mut ma_state, min: c_ulong, max: c_ulong, size: c_ulong) -> c_int; fn mas_empty_area_rev(mas: *mut ma_state, min: c_ulong, max: c_ulong, size: c_ulong) -> c_int; fn mas_preallocate(mas: *mut ma_state, vma: *mut vm_area_struct, gfp: gfp_t) -> c_int; fn vma_iter_invalidate(vmi: *mut vma_iterator); fn mas_is_err(mas: *mut ma_state) -> bool; fn vma_mark_attached(vma: *mut vm_area_struct); fn vma_assert_attached(vma: *mut vm_area_struct); fn vma_assert_can_modify(vma: *mut vm_area_struct); fn vma_next(vmi: *mut vma_iterator) -> *mut vm_area_struct; fn vma_prev(vmi: *mut vma_iterator) -> *mut vm_area_struct; }

#[inline] pub unsafe fn unmap_all_init(unmap: *mut unmap_desc, vmi: *mut vma_iterator, vma: *mut vm_area_struct) { (*unmap).mas=&mut (*vmi).mas; (*unmap).first=vma; (*unmap).pg_start=FIRST_USER_ADDRESS; (*unmap).pg_end=USER_PGTABLES_CEILING; (*unmap).vma_start=0; (*unmap).vma_end=ULONG_MAX; (*unmap).tree_end=ULONG_MAX; (*unmap).tree_reset=(*vma).vm_end; (*unmap).mm_wr_locked=false; }
#[inline] pub unsafe fn unmap_pgtable_init(unmap: *mut unmap_desc, vmi: *mut vma_iterator) { vma_iter_set(vmi, (*unmap).tree_reset); (*unmap).vma_start=FIRST_USER_ADDRESS; (*unmap).vma_end=USER_PGTABLES_CEILING; (*unmap).tree_end=USER_PGTABLES_CEILING; }
#[inline] pub unsafe fn vmg_nomem(vmg: *const vma_merge_struct) -> bool { (*vmg).state==VMA_MERGE_ERROR_NOMEM }
#[inline] pub unsafe fn vmg_pages(vmg: *const vma_merge_struct) -> pgoff_t { ((*vmg).end.wrapping_sub((*vmg).start) >> PAGE_SHIFT) as pgoff_t }
#[inline] pub unsafe fn vmg_start_pgoff(vmg: *const vma_merge_struct)->pgoff_t { (*vmg).pgoff }
#[inline] pub unsafe fn vmg_end_pgoff(vmg: *const vma_merge_struct)->pgoff_t { vmg_start_pgoff(vmg).wrapping_add(vmg_pages(vmg)) }
#[inline] pub unsafe fn vmg_start_anon_pgoff(vmg: *const vma_merge_struct)->pgoff_t { (*vmg).anon_pgoff }
#[inline] pub unsafe fn vmg_end_anon_pgoff(vmg: *const vma_merge_struct)->pgoff_t { vmg_start_anon_pgoff(vmg).wrapping_add(vmg_pages(vmg)) }
#[inline] pub unsafe fn __vma_set_anon_pgoff(vma:*mut vm_area_struct,pgoff:pgoff_t){(*vma).__vm_anon_pgoff_hi=(pgoff>>32) as u64;(*vma).__vm_anon_pgoff_lo=(pgoff & 0xffff_ffff) as u32;}
#[inline] pub unsafe fn vma_set_pgoff(vma:*mut vm_area_struct,pgoff:pgoff_t){vma_assert_can_modify(vma);(*vma).vm_pgoff=pgoff;}
#[inline] pub unsafe fn vma_set_anon_pgoff(vma:*mut vm_area_struct,pgoff:pgoff_t){vma_assert_can_modify(vma);__vma_set_anon_pgoff(vma,pgoff);}
#[inline] pub unsafe fn vma_add_pgoff(vma:*mut vm_area_struct,delta:pgoff_t){vma_assert_can_modify(vma);vma_set_pgoff(vma,(*vma).vm_pgoff.wrapping_add(delta));vma_set_anon_pgoff(vma,delta);}
#[inline] pub unsafe fn vma_sub_pgoff(vma:*mut vm_area_struct,delta:pgoff_t){vma_assert_can_modify(vma);vma_set_pgoff(vma,(*vma).vm_pgoff.wrapping_sub(delta));vma_set_anon_pgoff(vma,delta.wrapping_neg());}

#[inline] pub unsafe fn vma_iter_store_gfp(vmi:*mut vma_iterator,vma:*mut vm_area_struct,gfp:gfp_t)->c_int{if (*vmi).mas.status!=0&&((*vmi).mas.index>(*vma).vm_start||(*vmi).mas.last<(*vma).vm_start){vma_iter_invalidate(vmi);}__mas_set_range(&mut (*vmi).mas,(*vma).vm_start,(*vma).vm_end-1);mas_store_gfp(&mut (*vmi).mas,vma,gfp);if mas_is_err(&mut (*vmi).mas){return -12;}vma_mark_attached(vma);0}
#[inline] pub unsafe fn vma_iter_config(vmi:*mut vma_iterator,index:c_ulong,last:c_ulong){__mas_set_range(&mut (*vmi).mas,index,last-1)}
#[inline] pub unsafe fn vma_iter_reset(vmi:*mut vma_iterator){mas_reset(&mut (*vmi).mas)}
#[inline] pub unsafe fn vma_iter_load(vmi:*mut vma_iterator)->*mut vm_area_struct{mas_walk(&mut (*vmi).mas)}
#[inline] pub unsafe fn vma_iter_addr(vmi:*mut vma_iterator)->c_ulong{(*vmi).mas.index}
#[inline] pub unsafe fn vma_iter_end(vmi:*mut vma_iterator)->c_ulong{(*vmi).mas.last+1}
#[inline] pub unsafe fn vma_iter_prev_range(vmi:*mut vma_iterator)->*mut vm_area_struct{mas_prev_range(&mut (*vmi).mas,0)}
#[inline] pub unsafe fn vma_iter_prev_range_limit(vmi:*mut vma_iterator,min:c_ulong)->*mut vm_area_struct{mas_prev_range(&mut (*vmi).mas,min)}
#[inline] pub unsafe fn vma_iter_next_range_limit(vmi:*mut vma_iterator,max:c_ulong)->*mut vm_area_struct{mas_next_range(&mut (*vmi).mas,max)}
#[inline] pub unsafe fn vma_iter_area_lowest(vmi:*mut vma_iterator,min:c_ulong,max:c_ulong,size:c_ulong)->c_int{mas_empty_area(&mut (*vmi).mas,min,max-1,size)}
#[inline] pub unsafe fn vma_iter_area_highest(vmi:*mut vma_iterator,min:c_ulong,max:c_ulong,size:c_ulong)->c_int{mas_empty_area_rev(&mut (*vmi).mas,min,max-1,size)}
#[inline] pub unsafe fn vma_iter_prealloc(vmi:*mut vma_iterator,vma:*mut vm_area_struct)->c_int{mas_preallocate(&mut (*vmi).mas,vma,0)}
#[inline] pub unsafe fn vma_iter_clear(vmi:*mut vma_iterator){mas_store_prealloc(&mut (*vmi).mas,core::ptr::null_mut())}

#[inline] pub unsafe fn vma_iter_next_rewind(vmi:*mut vma_iterator,pprev:*mut *mut vm_area_struct)->*mut vm_area_struct{let next=vma_next(vmi);let prev=vma_prev(vmi);if !prev.is_null(){vma_iter_next_range(vmi);}if !pprev.is_null(){*pprev=prev;}next}
extern "C" { fn vma_iter_next_range(vmi:*mut vma_iterator); fn vma_expand(vmg:*mut vma_merge_struct)->c_int; fn vma_shrink(vmi:*mut vma_iterator,vma:*mut vm_area_struct,end:c_ulong)->c_int; fn do_vmi_align_munmap(vmi:*mut vma_iterator,vma:*mut vm_area_struct,mm:*mut mm_struct,start:c_ulong,end:c_ulong,uf:*mut list_head,unlock:bool)->c_int; fn do_vmi_munmap(vmi:*mut vma_iterator,mm:*mut mm_struct,start:c_ulong,len:size_t,uf:*mut list_head,unlock:bool)->c_int; fn remove_vma(vma:*mut vm_area_struct); fn unmap_region(unmap:*mut unmap_desc); fn vma_modify_flags(vmi:*mut vma_iterator,prev:*mut vm_area_struct,vma:*mut vm_area_struct,start:c_ulong,end:c_ulong,flags:*mut vma_flags_t)->*mut vm_area_struct; fn vma_modify_name(vmi:*mut vma_iterator,prev:*mut vm_area_struct,vma:*mut vm_area_struct,start:c_ulong,end:c_ulong,name:*mut anon_vma_name)->*mut vm_area_struct; fn vma_modify_policy(vmi:*mut vma_iterator,prev:*mut vm_area_struct,vma:*mut vm_area_struct,start:c_ulong,end:c_ulong,pol:*mut mempolicy)->*mut vm_area_struct; fn vma_merge_new_range(vmg:*mut vma_merge_struct)->*mut vm_area_struct; fn vma_merge_extend(vmi:*mut vma_iterator,vma:*mut vm_area_struct,delta:c_ulong)->*mut vm_area_struct; fn unlink_file_vma_batch_init(vb:*mut unlink_vma_file_batch); fn unlink_file_vma_batch_final(vb:*mut unlink_vma_file_batch); fn unlink_file_vma_batch_add(vb:*mut unlink_vma_file_batch,vma:*mut vm_area_struct); fn copy_vma(vmap:*mut *mut vm_area_struct,addr:c_ulong,len:c_ulong,pgoff:pgoff_t,anon_pgoff:pgoff_t,locks:*mut bool)->*mut vm_area_struct; fn find_mergeable_anon_vma(vma:*mut vm_area_struct)->*mut anon_vma; fn mm_take_all_locks(mm:*mut mm_struct)->c_int; fn mm_drop_all_locks(mm:*mut mm_struct); fn mmap_region(file:*mut file,addr:c_ulong,len:c_ulong,flags:vma_flags_t,pgoff:c_ulong,uf:*mut list_head)->c_ulong; fn do_brk_flags(vmi:*mut vma_iterator,brkvma:*mut vm_area_struct,addr:c_ulong,request:c_ulong,flags:vma_flags_t)->c_int; fn unmapped_area(info:*mut vm_unmapped_area_info)->c_ulong; fn unmapped_area_topdown(info:*mut vm_unmapped_area_info)->c_ulong; fn expand_downwards(vma:*mut vm_area_struct,address:c_ulong)->c_int; fn __vm_munmap(start:c_ulong,len:size_t,unlock:bool)->c_int; fn insert_vm_struct(mm:*mut mm_struct,vma:*mut vm_area_struct)->c_int; fn vma_state_init(); fn vm_area_alloc(mm:*mut mm_struct)->*mut vm_area_struct; fn vm_area_dup(orig:*mut vm_area_struct)->*mut vm_area_struct; fn vm_area_free(vma:*mut vm_area_struct); fn __install_special_mapping(mm:*mut mm_struct,addr:c_ulong,len:c_ulong,flags:vm_flags_t,priv_:*mut c_void,ops:*const vm_operations_struct)->*mut vm_area_struct; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
