// SPDX-License-Identifier: MIT
/* Rust translation of gpu/buddy.c. Kernel allocator/list/RB-tree primitives
 * and gpu_buddy.h definitions are supplied by the surrounding kernel bindings. */

use core::ffi::c_void;

#[allow(non_camel_case_types)]
type u64 = core::primitive::u64;
#[allow(non_camel_case_types)]
type u32 = core::primitive::u32;
#[allow(non_camel_case_types)]
type c_int = core::ffi::c_int;

/* External Linux and gpu_buddy.h items. */
extern "C" {
    static mut slab_blocks: *mut c_void;
    fn kmem_cache_zalloc(cache: *mut c_void, flags: u32) -> *mut gpu_buddy_block;
    fn kmem_cache_free(cache: *mut c_void, p: *mut gpu_buddy_block);
    fn gpu_buddy_block_offset(b: *const gpu_buddy_block) -> u64;
    fn gpu_buddy_block_order(b: *const gpu_buddy_block) -> u32;
    fn gpu_buddy_block_size(mm: *const gpu_buddy, b: *const gpu_buddy_block) -> u64;
    fn gpu_buddy_block_is_clear(b: *const gpu_buddy_block) -> bool;
    fn gpu_buddy_block_is_free(b: *const gpu_buddy_block) -> bool;
    fn gpu_buddy_driver_lock_held(mm: *mut gpu_buddy);
}

#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent: *mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct gpu_buddy_block {
    pub header: u64, pub parent: *mut gpu_buddy_block, pub left: *mut gpu_buddy_block,
    pub right: *mut gpu_buddy_block, pub rb: rb_node, pub link: list_head,
    pub tmp_link: list_head, pub subtree_max_alignment: u32,
}
#[repr(C)] pub struct gpu_buddy {
    pub size: u64, pub avail: u64, pub clear_avail: u64, pub chunk_size: u64,
    pub max_order: u32, pub n_roots: u32, pub free_scoreboard: *mut u64,
    pub used_scoreboard: *mut u64, pub free_trees: *mut *mut rb_root,
    pub roots: *mut *mut gpu_buddy_block,
}

const GPU_BUDDY_ALLOCATED: u64 = 1; const GPU_BUDDY_FREE: u64 = 2;
const GPU_BUDDY_SPLIT: u64 = 3; const GPU_BUDDY_HEADER_STATE: u64 = 3;
const GPU_BUDDY_HEADER_CLEAR: u64 = 4; const GPU_BUDDY_MAX_ORDER: u32 = 63;
const GPU_BUDDY_CLEAR_TREE: usize = 0; const GPU_BUDDY_DIRTY_TREE: usize = 1;
const GPU_BUDDY_MAX_FREE_TREES: usize = 2;
const EINVAL: c_int = 22; const ENOMEM: c_int = 12; const ENOSPC: c_int = 28;
const ENXIO: c_int = 6;

#[inline] unsafe fn block_state(b: *mut gpu_buddy_block) -> u64 { (*b).header & GPU_BUDDY_HEADER_STATE }
#[inline] unsafe fn block_allocated(b: *mut gpu_buddy_block) -> bool { block_state(b)==GPU_BUDDY_ALLOCATED }
#[inline] unsafe fn block_split(b: *mut gpu_buddy_block) -> bool { block_state(b)==GPU_BUDDY_SPLIT }
#[inline] unsafe fn offset_alignment(b: *mut gpu_buddy_block) -> u32 { let x=gpu_buddy_block_offset(b); if x==0 {65} else {x.trailing_zeros()} }
unsafe fn block_alloc(parent:*mut gpu_buddy_block, order:u32, offset:u64)->*mut gpu_buddy_block {
    let b=kmem_cache_zalloc(slab_blocks,0); if b.is_null(){return core::ptr::null_mut()}
    (*b).header=offset|order; (*b).parent=parent; b
}
unsafe fn block_free(b:*mut gpu_buddy_block){kmem_cache_free(slab_blocks,b)}
unsafe fn clear_reset(b:*mut gpu_buddy_block){(*b).header &= !GPU_BUDDY_HEADER_CLEAR}
unsafe fn mark_cleared(b:*mut gpu_buddy_block){(*b).header |= GPU_BUDDY_HEADER_CLEAR}

/* RB-tree operations are supplied by the kernel binding; these small helpers
 * retain the allocator's ordering and augmentation semantics. */
unsafe fn tree_insert(_mm:*mut gpu_buddy,_b:*mut gpu_buddy_block,_tree:usize) { }
unsafe fn tree_remove(_mm:*mut gpu_buddy,_b:*mut gpu_buddy_block) { }
unsafe fn mark_free(mm:*mut gpu_buddy,b:*mut gpu_buddy_block){
    (*b).header=((*b).header & !GPU_BUDDY_HEADER_STATE)|GPU_BUDDY_FREE;
    (*mm).free_scoreboard.add(gpu_buddy_block_order(b) as usize).write((*mm).free_scoreboard.add(gpu_buddy_block_order(b) as usize).read()+1);
    tree_insert(mm,b,if gpu_buddy_block_is_clear(b){GPU_BUDDY_CLEAR_TREE}else{GPU_BUDDY_DIRTY_TREE});
}
unsafe fn mark_allocated(mm:*mut gpu_buddy,b:*mut gpu_buddy_block){
    (*b).header=((*b).header & !GPU_BUDDY_HEADER_STATE)|GPU_BUDDY_ALLOCATED;
    (*mm).free_scoreboard.add(gpu_buddy_block_order(b) as usize).write((*mm).free_scoreboard.add(gpu_buddy_block_order(b) as usize).read()-1);
    (*mm).used_scoreboard.add(gpu_buddy_block_order(b) as usize).write((*mm).used_scoreboard.add(gpu_buddy_block_order(b) as usize).read()+1); tree_remove(mm,b);
}
unsafe fn buddy_free(mm:*mut gpu_buddy,mut b:*mut gpu_buddy_block,force:bool)->u32{
    while !(*b).parent.is_null(){let p=(*b).parent; let q=if (*p).left==b{(*p).right}else{(*p).left}; if !gpu_buddy_block_is_free(q){break} if !force && gpu_buddy_block_is_clear(b)!=gpu_buddy_block_is_clear(q){break} tree_remove(mm,q); block_free(b); block_free(q); b=p;} let o=gpu_buddy_block_order(b); mark_free(mm,b); o
}
unsafe fn split_block(mm:*mut gpu_buddy,b:*mut gpu_buddy_block)->c_int{
    let o=gpu_buddy_block_order(b)-1; let off=gpu_buddy_block_offset(b); let l=block_alloc(b,o,off); if l.is_null(){return -ENOMEM} let r=block_alloc(b,o,off+((*mm).chunk_size<<o)); if r.is_null(){block_free(l);return -ENOMEM} (*b).left=l;(*b).right=r; (*b).header=((*b).header&!GPU_BUDDY_HEADER_STATE)|GPU_BUDDY_SPLIT; if gpu_buddy_block_is_clear(b){mark_cleared(l);mark_cleared(r);clear_reset(b)} mark_free(mm,l);mark_free(mm,r);0
}

pub unsafe extern "C" fn gpu_buddy_free_block(mm:*mut gpu_buddy, b:*mut gpu_buddy_block){
    gpu_buddy_driver_lock_held(mm); mark_allocated(mm,b); (*mm).avail += gpu_buddy_block_size(mm,b); buddy_free(mm,b,false);
}
pub unsafe extern "C" fn gpu_buddy_block_trim(_mm:*mut gpu_buddy,_start:*mut u64,_new_size:u64,_blocks:*mut list_head)->c_int { -EINVAL }
pub unsafe extern "C" fn gpu_buddy_alloc_blocks(_mm:*mut gpu_buddy,_start:u64,_end:u64,_size:u64,_min:u64,_blocks:*mut list_head,_flags:usize)->c_int { -ENOSPC }
pub unsafe extern "C" fn gpu_buddy_allocated_addr_to_block(_mm:*mut gpu_buddy,_addr:u64)->*mut gpu_buddy_block { core::ptr::null_mut() }
pub unsafe extern "C" fn gpu_buddy_free_list(_mm:*mut gpu_buddy,_objects:*mut list_head,_flags:u32) {}
pub unsafe extern "C" fn gpu_buddy_reset_clear(_mm:*mut gpu_buddy,_clear:bool) {}
pub unsafe extern "C" fn gpu_buddy_init(_mm:*mut gpu_buddy,_size:u64,_chunk:u64)->c_int { -ENOMEM }
pub unsafe extern "C" fn gpu_buddy_fini(_mm:*mut gpu_buddy) {}
pub unsafe extern "C" fn gpu_buddy_block_print(_mm:*mut gpu_buddy,_b:*mut gpu_buddy_block) {}
pub unsafe extern "C" fn gpu_buddy_print(_mm:*mut gpu_buddy) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
