/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of linux/xarray.h.  Included dependencies are supplied externally. */

use core::ffi::c_void;

pub const BITS_PER_XA_VALUE: usize = usize::BITS as usize - 1;

#[inline] pub unsafe fn xa_mk_value(v: usize) -> *mut c_void { ((v << 1) | 1) as *mut c_void }
#[inline] pub unsafe fn xa_to_value(e: *const c_void) -> usize { (e as usize) >> 1 }
#[inline] pub unsafe fn xa_is_value(e: *const c_void) -> bool { (e as usize & 1) != 0 }
#[inline] pub unsafe fn xa_tag_pointer(p: *mut c_void, tag: usize) -> *mut c_void { (p as usize | tag) as *mut c_void }
#[inline] pub unsafe fn xa_untag_pointer(e: *mut c_void) -> *mut c_void { (e as usize & !3) as *mut c_void }
#[inline] pub unsafe fn xa_pointer_tag(e: *mut c_void) -> u32 { (e as usize & 3) as u32 }
#[inline] pub unsafe fn xa_mk_internal(v: usize) -> *mut c_void { ((v << 2) | 2) as *mut c_void }
#[inline] pub unsafe fn xa_to_internal(e: *const c_void) -> usize { (e as usize) >> 2 }
#[inline] pub unsafe fn xa_is_internal(e: *const c_void) -> bool { e as usize & 3 == 2 }
pub const XA_ZERO_ENTRY_VALUE: usize = (257usize << 2) | 2;
#[inline] pub unsafe fn xa_is_zero(e: *const c_void) -> bool { e as usize == XA_ZERO_ENTRY_VALUE }
#[inline] pub unsafe fn xa_is_err(e: *const c_void) -> bool { xa_is_internal(e) && (e as usize) >= ((-(4095isize)) as usize << 2 | 2) }
#[inline] pub unsafe fn xa_err(e: *mut c_void) -> i32 { if xa_is_err(e) { ((e as isize) >> 2) as i32 } else { 0 } }

#[repr(C)] #[derive(Copy, Clone)] pub struct xa_limit { pub max: u32, pub min: u32 }
pub const XA_MARK_0: u32 = 0; pub const XA_MARK_1: u32 = 1; pub const XA_MARK_2: u32 = 2;
pub const XA_PRESENT: u32 = 8; pub const XA_MARK_MAX: u32 = XA_MARK_2; pub const XA_FREE_MARK: u32 = XA_MARK_0;
#[repr(C)] pub enum xa_lock_type { XA_LOCK_IRQ = 1, XA_LOCK_BH = 2 }

/* Kernel-provided types and synchronization primitives. */
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
pub type gfp_t = usize;
#[repr(C)] pub struct list_lru { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
extern "C" {
    pub fn xa_load(_: *mut xarray, _: usize) -> *mut c_void;
    pub fn xa_store(_: *mut xarray, _: usize, _: *mut c_void, _: gfp_t) -> *mut c_void;
    pub fn xa_erase(_: *mut xarray, _: usize) -> *mut c_void;
    pub fn xa_find(_: *mut xarray, _: *mut usize, _: usize, _: u32) -> *mut c_void;
    pub fn xa_find_after(_: *mut xarray, _: *mut usize, _: usize, _: u32) -> *mut c_void;
    pub fn xa_destroy(_: *mut xarray);
    pub fn __xa_erase(_: *mut xarray, _: usize) -> *mut c_void;
    pub fn __xa_store(_: *mut xarray, _: usize, _: *mut c_void, _: gfp_t) -> *mut c_void;
    pub fn __xa_cmpxchg(_: *mut xarray, _: usize, _: *mut c_void, _: *mut c_void, _: gfp_t) -> *mut c_void;
    pub fn __xa_insert(_: *mut xarray, _: usize, _: *mut c_void, _: gfp_t) -> i32;
    pub fn __xa_alloc(_: *mut xarray, _: *mut u32, _: *mut c_void, _: xa_limit, _: gfp_t) -> i32;
    pub fn __xa_alloc_cyclic(_: *mut xarray, _: *mut u32, _: *mut c_void, _: xa_limit, _: *mut u32, _: gfp_t) -> i32;
    pub fn __xa_set_mark(_: *mut xarray, _: usize, _: u32); pub fn __xa_clear_mark(_: *mut xarray, _: usize, _: u32);
}

#[repr(C)] pub struct xarray { pub xa_lock: spinlock_t, pub xa_flags: gfp_t, pub xa_head: *mut c_void }
#[repr(C)] pub union xa_node_union { pub private_list: list_head, pub rcu_head: rcu_head }
pub const XA_CHUNK_SHIFT: usize = 6; pub const XA_CHUNK_SIZE: usize = 1 << XA_CHUNK_SHIFT; pub const XA_CHUNK_MASK: usize = XA_CHUNK_SIZE - 1;
pub const XA_MAX_MARKS: usize = 3; pub const XA_MARK_LONGS: usize = (XA_CHUNK_SIZE + usize::BITS as usize - 1) / usize::BITS as usize;
#[repr(C)] pub struct xa_node { pub shift:u8, pub offset:u8, pub count:u8, pub nr_values:u8, pub parent:*mut xa_node, pub array:*mut xarray, pub u:xa_node_union, pub slots:[*mut c_void; XA_CHUNK_SIZE], pub tags:[[usize; XA_MARK_LONGS]; XA_MAX_MARKS] }

#[inline] pub unsafe fn xa_mk_node(n:*const xa_node)->*mut c_void {(n as usize|2) as *mut c_void}
#[inline] pub unsafe fn xa_to_node(e:*const c_void)->*mut xa_node {(e as usize-2) as *mut xa_node}
#[inline] pub unsafe fn xa_is_node(e:*const c_void)->bool {xa_is_internal(e)&&(e as usize)>4096}
#[inline] pub unsafe fn xa_mk_sibling(o:usize)->*mut c_void {xa_mk_internal(o)}
#[inline] pub unsafe fn xa_to_sibling(e:*const c_void)->usize {xa_to_internal(e)}
#[inline] pub unsafe fn xa_is_sibling(e:*const c_void)->bool {xa_is_internal(e)&&(e as usize)<((XA_CHUNK_SIZE-1)<<2|2)}
pub const XA_RETRY_ENTRY_VALUE:usize=(256<<2)|2;
#[inline] pub unsafe fn xa_is_retry(e:*const c_void)->bool {e as usize==XA_RETRY_ENTRY_VALUE}
#[inline] pub unsafe fn xa_is_advanced(e:*const c_void)->bool {xa_is_internal(e)&&(e as usize)<=XA_RETRY_ENTRY_VALUE}

pub type xa_update_node_t = unsafe extern "C" fn(*mut xa_node);
#[repr(C)] pub struct xa_state { pub xa:*mut xarray, pub xa_index:usize, pub xa_shift:u8, pub xa_sibs:u8, pub xa_offset:u8, pub xa_pad:u8, pub xa_node:*mut xa_node, pub xa_alloc:*mut xa_node, pub xa_update:Option<xa_update_node_t>, pub xa_lru:*mut list_lru }
pub const XA_CHECK_SCHED:u32=4096;
extern "C" { pub fn xas_load(_: *mut xa_state)->*mut c_void; pub fn xas_store(_: *mut xa_state,_:*mut c_void)->*mut c_void; pub fn xas_find(_: *mut xa_state,_:usize)->*mut c_void; pub fn xas_find_marked(_: *mut xa_state,_:usize,_:u32)->*mut c_void; pub fn xas_next(_: *mut xa_state)->*mut c_void; pub fn xas_prev(_: *mut xa_state)->*mut c_void; }
extern "C" {
    pub fn xa_get_mark(_: *mut xarray, _: usize, _: u32)->bool; pub fn xa_set_mark(_: *mut xarray, _: usize, _: u32); pub fn xa_clear_mark(_: *mut xarray, _: usize, _: u32);
    pub fn xa_store_range(_: *mut xarray, _: usize, _: usize, _: *mut c_void, _: gfp_t)->*mut c_void;
    pub fn xa_extract(_: *mut xarray, _: *mut *mut c_void, _: usize, _: usize, _: u32, _: u32)->u32;
    pub fn xa_dump(_: *const xarray); pub fn xa_dump_node(_: *const xa_node); pub fn xa_delete_node(_: *mut xa_node, _: xa_update_node_t);
    pub fn xas_find_conflict(_: *mut xa_state)->*mut c_void; pub fn xas_get_mark(_: *const xa_state, _:u32)->bool; pub fn xas_set_mark(_: *const xa_state, _:u32); pub fn xas_clear_mark(_: *const xa_state, _:u32); pub fn xas_init_marks(_: *const xa_state);
    pub fn xas_nomem(_: *mut xa_state, _:gfp_t)->bool; pub fn xas_destroy(_: *mut xa_state); pub fn xas_pause(_: *mut xa_state); pub fn xas_create_range(_: *mut xa_state);
    pub fn __xas_next(_: *mut xa_state)->*mut c_void; pub fn __xas_prev(_: *mut xa_state)->*mut c_void;
}

#[inline] pub unsafe fn xa_is_retry_entry(e:*const c_void)->bool { xa_is_retry(e) }
#[inline] pub unsafe fn xas_error(x:*const xa_state)->i32 { xa_err((*x).xa_node as *mut c_void) }
#[inline] pub unsafe fn xas_set_err(x:*mut xa_state, e:isize) { (*x).xa_node=((e as usize)<<2|2) as *mut xa_node }
#[inline] pub unsafe fn xas_invalid(x:*const xa_state)->bool {(*x).xa_node as usize&3!=0}
#[inline] pub unsafe fn xas_valid(x:*const xa_state)->bool {!xas_invalid(x)}
#[inline] pub unsafe fn xas_is_node(x:*const xa_state)->bool {xas_valid(x)&&!(*x).xa_node.is_null()}
#[inline] pub unsafe fn xas_not_node(n:*mut xa_node)->bool {n.is_null()||(n as usize&3)!=0}
#[inline] pub unsafe fn xas_reset(x:*mut xa_state){(*x).xa_node=3usize as *mut xa_node}
#[inline] pub unsafe fn xas_set(x:*mut xa_state,i:usize){(*x).xa_index=i;xas_reset(x)}
#[inline] pub unsafe fn xas_set_update(x:*mut xa_state,u:xa_update_node_t){(*x).xa_update=Some(u)}
#[inline] pub unsafe fn xa_head(x:*const xarray)->*mut c_void {(*x).xa_head}
#[inline] pub unsafe fn xa_entry(_: *const xarray,n:*const xa_node,o:usize)->*mut c_void {(*n).slots[o]}
#[inline] pub unsafe fn xa_parent(_: *const xarray,n:*const xa_node)->*mut xa_node {(*n).parent}
#[inline] pub unsafe fn xas_frozen(n:*mut xa_node)->bool {n as usize&2!=0}
#[inline] pub unsafe fn xas_top(n:*mut xa_node)->bool {(n as usize)<=3}
#[inline] pub unsafe fn xas_retry(x:*mut xa_state,e:*const c_void)->bool {if xa_is_zero(e){true}else if xa_is_retry(e){xas_reset(x);true}else{false}}
#[inline] pub unsafe fn xas_set_order(x:*mut xa_state,i:usize,order:usize){(*x).xa_index=(i>>order)<<order;(*x).xa_shift=(order-(order%XA_CHUNK_SHIFT)) as u8;(*x).xa_sibs=((1usize<<(order%XA_CHUNK_SHIFT))-1) as u8;xas_reset(x)}
#[inline] pub unsafe fn xas_set_lru(x:*mut xa_state,l:*mut list_lru){(*x).xa_lru=l}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
