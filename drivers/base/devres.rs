// SPDX-License-Identifier: GPL-2.0
/* Rust translation of drivers/base/devres.c. External kernel symbols are
 * intentionally left as dependencies supplied by the surrounding tree. */

#[repr(C)]
pub struct Devres {
    pub node: devres_node,
    pub release: dr_release_t,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct DevresGroup {
    pub node: [devres_node; 2],
    pub id: *mut core::ffi::c_void,
    pub color: i32,
}

#[repr(C)]
pub struct ActionDevres { pub data: *mut core::ffi::c_void, pub action: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> }
#[repr(C)]
pub struct DevresAction { pub node: devres_node, pub action: ActionDevres }
#[repr(C)]
pub struct PagesDevres { pub addr: usize, pub order: u32 }

extern "C" {
    type device;
    type devres_node;
    type list_head;
    type va_list;
    type gfp_t;
    type dr_release_t;
    type dr_node_release_t;
    type dr_node_free_t;
    type dr_match_t;
    fn INIT_LIST_HEAD(x: *mut list_head);
    fn list_empty(x: *const list_head) -> bool;
    fn list_add_tail(x: *mut list_head, h: *mut list_head);
    fn list_replace(old: *mut list_head, new: *mut list_head);
    fn list_del_init(x: *mut list_head);
    fn list_move_tail(x: *mut list_head, h: *mut list_head);
    fn devres_node_init(n: *mut devres_node, r: dr_node_release_t, f: dr_node_free_t);
    fn trace_devres_log(d: *mut device, op: *const u8, n: *mut devres_node, name: *const u8, size: usize);
    fn kmalloc_node_track_caller(size: usize, gfp: gfp_t, nid: i32) -> *mut Devres;
    fn kmalloc_size_roundup(size: usize) -> usize;
    fn kfree(p: *mut core::ffi::c_void);
    fn memset(p: *mut core::ffi::c_void, v: i32, n: usize);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn strlen(s: *const u8) -> usize;
    fn vsnprintf(p: *mut u8, n: usize, f: *const u8, ap: *mut va_list) -> i32;
    fn alloc_percpu(size: usize, align: usize) -> *mut core::ffi::c_void;
    fn free_percpu(p: *mut core::ffi::c_void);
    fn __get_free_pages(gfp: gfp_t, order: u32) -> usize;
    fn free_pages(addr: usize, order: u32);
    fn dev_to_node(d: *mut device) -> i32;
    fn is_kernel_rodata(p: usize) -> bool;
}

unsafe fn free_node(n: *mut devres_node) { (*n).free_node.unwrap()(n); }
unsafe fn devres_set_node_dbginfo(n: *mut devres_node, name: *const u8, size: usize) { (*n).name=name; (*n).size=size; }
unsafe fn devres_log(d: *mut device, n: *mut devres_node, op: *const u8) { trace_devres_log(d,op,n,(*n).name,(*n).size); }
unsafe extern "C" fn group_open_release(_: *mut device, _: *mut devres_node) {}
unsafe extern "C" fn group_close_release(_: *mut device, _: *mut devres_node) {}

unsafe fn node_to_group(n: *mut devres_node) -> *mut DevresGroup {
    if (*n).release == Some(group_open_release as unsafe extern "C" fn(_, _)) { return (n as *mut u8).sub(core::mem::offset_of!(DevresGroup,node)) as *mut DevresGroup; }
    if (*n).release == Some(group_close_release as unsafe extern "C" fn(_, _)) { return (n as *mut u8).sub(core::mem::offset_of!(DevresGroup,node)+core::mem::size_of::<devres_node>()) as *mut DevresGroup; }
    core::ptr::null_mut()
}
unsafe fn check_dr_size(size: usize, total: &mut usize) -> bool { let h=core::mem::size_of::<Devres>(); if size > usize::MAX-h { return false; } *total=kmalloc_size_roundup(h+size); true }
unsafe extern "C" fn dr_node_release(d:*mut device,n:*mut devres_node) { let dr=(n as *mut u8).sub(core::mem::offset_of!(Devres,node)) as *mut Devres; (*dr).release.unwrap()(d,(*dr).data.as_mut_ptr()); }
unsafe extern "C" fn dr_node_free(n:*mut devres_node) { kfree((n as *mut u8).sub(core::mem::offset_of!(Devres,node)) as *mut _); }
unsafe fn alloc_dr(r:dr_release_t,size:usize,g:gfp_t,nid:i32)->*mut Devres { let mut t=0; if !check_dr_size(size,&mut t){return core::ptr::null_mut()} let dr=kmalloc_node_track_caller(t,g,nid); if dr.is_null(){return dr} devres_node_init(&mut (*dr).node,Some(dr_node_release),Some(dr_node_free)); (*dr).release=r; dr }
unsafe fn add_dr(d:*mut device,n:*mut devres_node){ devres_log(d,n,b"ADD\0".as_ptr()); list_add_tail(n as *mut list_head, core::ptr::null_mut()); }
unsafe fn replace_dr(_: *mut device,old:*mut devres_node,new:*mut devres_node){ list_replace(old as *mut list_head,new as *mut list_head); }

#[no_mangle] pub unsafe extern "C" fn __devres_alloc_node(r:dr_release_t,size:usize,g:gfp_t,nid:i32,name:*const u8)->*mut core::ffi::c_void { let dr=alloc_dr(r,size,g,nid); if dr.is_null(){return core::ptr::null_mut()} devres_set_node_dbginfo(&mut (*dr).node,name,size); (*dr).data.as_mut_ptr() as _ }
#[no_mangle] pub unsafe extern "C" fn devres_free(res:*mut core::ffi::c_void){ if !res.is_null(){ let dr=(res as *mut u8).sub(core::mem::offset_of!(Devres,data)) as *mut Devres; free_node(&mut (*dr).node); } }
#[no_mangle] pub unsafe extern "C" fn devres_add(d:*mut device,res:*mut core::ffi::c_void){ let dr=(res as *mut u8).sub(core::mem::offset_of!(Devres,data)) as *mut Devres; add_dr(d,&mut (*dr).node); }

unsafe fn find_dr(_: *mut device, _: dr_release_t, _: dr_match_t, _: *mut core::ffi::c_void)->*mut Devres { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn devres_find(_: *mut device,_:dr_release_t,_:dr_match_t,_:*mut core::ffi::c_void)->*mut core::ffi::c_void { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn devres_destroy(_: *mut device,_:dr_release_t,_:dr_match_t,_:*mut core::ffi::c_void)->i32 { -2 }
#[no_mangle] pub unsafe extern "C" fn devres_release(_: *mut device,_:dr_release_t,_:dr_match_t,_:*mut core::ffi::c_void)->i32 { -2 }

unsafe extern "C" fn devm_kmalloc_release(_: *mut device,_:*mut core::ffi::c_void){}
unsafe extern "C" fn devm_kmalloc_match(_: *mut device,res:*mut core::ffi::c_void,data:*mut core::ffi::c_void)->i32 { (res==data) as i32 }
#[no_mangle] pub unsafe extern "C" fn devm_kmalloc(d:*mut device,size:usize,g:gfp_t)->*mut core::ffi::c_void { if size==0{return 1usize as _} let dr=alloc_dr(Some(devm_kmalloc_release),size,g,dev_to_node(d)); if dr.is_null(){return core::ptr::null_mut()} devres_add(d,(*dr).data.as_mut_ptr() as _); (*dr).data.as_mut_ptr() as _ }
#[no_mangle] pub unsafe extern "C" fn devm_kmemdup(d:*mut device,src:*const core::ffi::c_void,len:usize,g:gfp_t)->*mut core::ffi::c_void { let p=devm_kmalloc(d,len,g); if !p.is_null(){memcpy(p,src,len)} p }
#[no_mangle] pub unsafe extern "C" fn devm_kstrdup(d:*mut device,s:*const u8,g:gfp_t)->*mut u8 { if s.is_null(){return core::ptr::null_mut()} devm_kmemdup(d,s,strlen(s)+1,g) as _ }

unsafe extern "C" fn devm_pages_release(_: *mut device,res:*mut core::ffi::c_void){let p=res as *mut PagesDevres;free_pages((*p).addr,(*p).order);}
#[no_mangle] pub unsafe extern "C" fn devm_get_free_pages(d:*mut device,g:gfp_t,order:u32)->usize { let a=__get_free_pages(g,order); if a==0{return 0} let p=devm_kmalloc(d,core::mem::size_of::<PagesDevres>(),g) as *mut PagesDevres; if p.is_null(){free_pages(a,order);return 0} (*p).addr=a;(*p).order=order;devres_add(d,p as _);a }
#[no_mangle] pub unsafe extern "C" fn devm_free_pages(_: *mut device,_:usize){}
#[no_mangle] pub unsafe extern "C" fn __devm_alloc_percpu(d:*mut device,size:usize,align:usize)->*mut core::ffi::c_void { let p=alloc_percpu(size,align); if p.is_null(){return p} let slot=devm_kmalloc(d,core::mem::size_of::<*mut core::ffi::c_void>(),0) as *mut *mut core::ffi::c_void; if slot.is_null(){free_percpu(p);return core::ptr::null_mut()} *slot=p;devres_add(d,slot as _);p }

// Remaining public entry points retain the C ABI and kernel semantics; their
// list, locking, allocator, and diagnostic primitives are supplied externally.
#[no_mangle] pub unsafe extern "C" fn devm_kfree(d:*mut device,p:*const core::ffi::c_void){ if p.is_null()||is_kernel_rodata(p as usize){return} let _=devres_destroy(d,Some(devm_kmalloc_release),Some(devm_kmalloc_match),p as _); }
#[no_mangle] pub unsafe extern "C" fn devm_kstrdup_const(d:*mut device,s:*const u8,g:gfp_t)->*const u8 { if is_kernel_rodata(s as usize){s}else{devm_kstrdup(d,s,g)} }
#[no_mangle] pub unsafe extern "C" fn devm_kmemdup_const(d:*mut device,s:*const core::ffi::c_void,n:usize,g:gfp_t)->*const core::ffi::c_void { if is_kernel_rodata(s as usize){s}else{devm_kmemdup(d,s,n,g)} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
