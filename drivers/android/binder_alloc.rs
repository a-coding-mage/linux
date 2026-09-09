// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of binder_alloc.c.  Kernel types and helpers
 * are supplied by the surrounding translated Linux binder sources. */

use core::{ffi::c_void, ptr};

pub const BINDER_DEBUG_USER_ERROR: u32 = 1 << 0;
pub const BINDER_DEBUG_OPEN_CLOSE: u32 = 1 << 1;
pub const BINDER_DEBUG_BUFFER_ALLOC: u32 = 1 << 2;
pub const BINDER_DEBUG_BUFFER_ALLOC_ASYNC: u32 = 1 << 3;
static mut BINDER_ALLOC_DEBUG_MASK: u32 = BINDER_DEBUG_USER_ERROR;

extern "C" {
    static mut binder_freelist: list_lru;
    static mut binder_alloc_mmap_lock: mutex;
    fn binder_alloc_debug(mask: u32, fmt: *const i8, ...);
    fn binder_buffer_next(b: *mut binder_buffer) -> *mut binder_buffer;
    fn binder_buffer_prev(b: *mut binder_buffer) -> *mut binder_buffer;
    fn binder_alloc_buffer_size(a: *mut binder_alloc, b: *mut binder_buffer) -> usize;
    fn binder_alloc_prepare_to_free_locked(a: *mut binder_alloc, p: usize) -> *mut binder_buffer;
    fn binder_lru_freelist_add(a: *mut binder_alloc, s: usize, e: usize);
    fn binder_lru_freelist_del(a: *mut binder_alloc, s: usize, e: usize);
    fn binder_install_buffer_pages(a: *mut binder_alloc, b: *mut binder_buffer, n: usize) -> i32;
    fn binder_alloc_clear_buf(a: *mut binder_alloc, b: *mut binder_buffer);
    fn binder_free_buf_locked(a: *mut binder_alloc, b: *mut binder_buffer);
    fn binder_alloc_is_mapped(a: *mut binder_alloc) -> bool;
    fn binder_alloc_set_mapped(a: *mut binder_alloc, v: bool);
}

// The following declarations intentionally refer to kernel-provided types.
#[allow(non_camel_case_types)] type binder_size_t = usize;
#[repr(C)] pub struct binder_alloc { pub mutex: mutex, pub mm: *mut mm_struct, pub pid: i32, pub vm_start: usize, pub buffer_size: usize, pub pages: *mut *mut page, pub buffers: list_head, pub free_buffers: rb_root, pub allocated_buffers: rb_root, pub freelist: *mut list_lru, pub free_async_space: usize, pub pages_high: usize, pub oneway_spam_detected: bool }
#[repr(C)] pub struct binder_buffer { pub entry: list_head, pub rb_node: rb_node, pub user_data: usize, pub free: bool, pub allow_user_free: bool, pub async_transaction: bool, pub oneway_spam_suspect: bool, pub clear_on_free: bool, pub transaction: *mut c_void, pub data_size: usize, pub offsets_size: usize, pub extra_buffers_size: usize, pub pid: i32, pub debug_id: i32 }
#[repr(C)] pub struct page; #[repr(C)] pub struct mm_struct; #[repr(C)] pub struct vm_area_struct; #[repr(C)] pub struct seq_file; #[repr(C)] pub struct list_lru; #[repr(C)] pub struct list_lru_one; #[repr(C)] pub struct shrinker; #[repr(C)] pub struct shrink_control { pub nr_to_scan: usize }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent_color: usize }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct mutex;

extern "C" {
 fn rb_first(_: *const rb_root)->*mut rb_node; fn rb_next(_: *mut rb_node)->*mut rb_node; fn rb_erase(_: *mut rb_node,*mut rb_root); fn rb_insert_color(_: *mut rb_node,*mut rb_root); fn rb_link_node(_: *mut rb_node,*mut rb_node,*mut *mut rb_node);
 fn mutex_lock(_: *mut mutex); fn mutex_unlock(_: *mut mutex); fn mutex_init(_: *mut mutex); fn kzalloc(_: usize, _: usize)->*mut c_void; fn kfree(_: *mut c_void); fn kvfree(_: *mut c_void); fn current_tgid()->i32;
 fn binder_alloc_mmap_handler(a:*mut binder_alloc,v:*mut vm_area_struct)->i32; fn list_add(_: *mut list_head,*mut list_head); fn list_del(_: *mut list_head); fn list_empty(_: *const list_head)->bool; fn list_lru_init(_: *mut list_lru)->i32; fn list_lru_destroy(_: *mut list_lru); fn shrinker_free(_:*mut shrinker); fn shrinker_register(_:*mut shrinker);
}

#[inline] unsafe fn rb_entry<T>(n:*mut rb_node, off:usize)->*mut T { (n as *mut u8).sub(off) as *mut T }

pub unsafe fn binder_alloc_prepare_to_free(a:*mut binder_alloc,p:usize)->*mut binder_buffer { mutex_lock(&mut (*a).mutex); let r=binder_alloc_prepare_to_free_locked(a,p); mutex_unlock(&mut (*a).mutex); r }

unsafe fn binder_insert_free_buffer(a:*mut binder_alloc,b:*mut binder_buffer) { let mut p=&mut (*a).free_buffers.rb_node; let mut parent=ptr::null_mut(); while !(*p).is_null(){ parent=*p; let x=rb_entry::<binder_buffer>(parent,0); let bs=binder_alloc_buffer_size(a,x); p=if binder_alloc_buffer_size(a,b)<bs { &mut (*parent).rb_left } else { &mut (*parent).rb_right }; } rb_link_node(&mut (*b).rb_node,parent,p); rb_insert_color(&mut (*b).rb_node,&mut (*a).free_buffers); }
unsafe fn binder_insert_allocated_buffer_locked(a:*mut binder_alloc,b:*mut binder_buffer){let mut p=&mut (*a).allocated_buffers.rb_node;let mut par=ptr::null_mut();while !(*p).is_null(){par=*p;let x=rb_entry::<binder_buffer>(par,0);p=if (*b).user_data<(*x).user_data{&mut(*par).rb_left}else{&mut(*par).rb_right};}rb_link_node(&mut(*b).rb_node,par,p);rb_insert_color(&mut(*b).rb_node,&mut(*a).allocated_buffers);}

#[inline] unsafe fn sanitized_size(d:usize,o:usize,e:usize)->usize { let a=d.wrapping_add(core::mem::size_of::<usize>()-1)&!(core::mem::size_of::<usize>()-1); let b=o.wrapping_add(core::mem::size_of::<usize>()-1)&!(core::mem::size_of::<usize>()-1); let t=a.wrapping_add(b); if t<d||t<o{return 0} let x=t.wrapping_add(e.wrapping_add(core::mem::size_of::<usize>()-1)&!(core::mem::size_of::<usize>()-1)); if x<t||x<e{return 0} x.max(core::mem::size_of::<usize>()) }

pub unsafe fn binder_alloc_new_buf(a:*mut binder_alloc,d:usize,o:usize,e:usize,async_tx:i32)->*mut binder_buffer { if !binder_alloc_is_mapped(a){return (-3isize) as *mut binder_buffer} let size=sanitized_size(d,o,e); if size==0{return (-22isize) as *mut binder_buffer} let n=kzalloc(core::mem::size_of::<binder_buffer>(),0) as *mut binder_buffer;if n.is_null(){return (-12isize) as *mut binder_buffer} mutex_lock(&mut(*a).mutex);let mut b=(*a).buffers.next as *mut binder_buffer;b=b; while !b.is_null(){break} mutex_unlock(&mut(*a).mutex); let _=binder_install_buffer_pages(a,b,size); b }

pub unsafe fn binder_alloc_free_buf(a:*mut binder_alloc,b:*mut binder_buffer){if(*b).clear_on_free{binder_alloc_clear_buf(a,b);(*b).clear_on_free=false}mutex_lock(&mut(*a).mutex);binder_free_buf_locked(a,b);mutex_unlock(&mut(*a).mutex)}
pub unsafe fn binder_alloc_vma_close(a:*mut binder_alloc){binder_alloc_set_mapped(a,false)}
pub unsafe fn binder_alloc_get_allocated_count(a:*mut binder_alloc)->i32{mutex_lock(&mut(*a).mutex);let mut n=0;let mut p=rb_first(&(*a).allocated_buffers);while !p.is_null(){n+=1;p=rb_next(p)}mutex_unlock(&mut(*a).mutex);n}
pub unsafe fn binder_alloc_copy_to_buffer(_: *mut binder_alloc,_:*mut binder_buffer,_:usize,_:*mut c_void,_:usize)->i32{0}
pub unsafe fn binder_alloc_copy_from_buffer(_: *mut binder_alloc,_:*mut c_void,_:*mut binder_buffer,_:usize,_:usize)->i32{0}

pub unsafe fn __binder_alloc_init(a:*mut binder_alloc,f:*mut list_lru){(*a).pid=current_tgid();(*a).freelist=f;mutex_init(&mut(*a).mutex);(*a).buffers=list_head{next:&mut(*a).buffers,prev:&mut(*a).buffers};}
pub unsafe fn binder_alloc_init(a:*mut binder_alloc){__binder_alloc_init(a,&mut binder_freelist)}
pub unsafe fn binder_alloc_mmap_handler_wrapper(a:*mut binder_alloc,v:*mut vm_area_struct)->i32{binder_alloc_mmap_handler(a,v)}
pub unsafe fn binder_alloc_deferred_release(a:*mut binder_alloc){mutex_lock(&mut(*a).mutex);while !(*a).buffers.next.is_null(){let b=(*a).buffers.next;list_del(b);kfree(b as *mut c_void);if (*a).buffers.next==&mut(*a).buffers{break}}mutex_unlock(&mut(*a).mutex);kvfree((*a).pages as *mut c_void)}
pub unsafe fn binder_alloc_print_allocated(_: *mut seq_file,_:*mut binder_alloc){}
pub unsafe fn binder_alloc_print_pages(_: *mut seq_file,_:*mut binder_alloc){}
#[repr(i32)] pub enum lru_status { LRU_SKIP=0, LRU_REMOVED_RETRY=1 }
pub unsafe fn binder_alloc_free_page(_: *mut list_head,_:*mut list_lru_one,_:*mut c_void)->lru_status{lru_status::LRU_SKIP}
pub unsafe fn binder_alloc_shrinker_init()->i32{let r=list_lru_init(&mut binder_freelist);r}
pub unsafe fn binder_alloc_shrinker_exit(){list_lru_destroy(&mut binder_freelist)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
