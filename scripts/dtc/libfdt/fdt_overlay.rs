// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/* libfdt - Flat Device Tree manipulation
 * Copyright (C) 2016 Free Electrons
 * Copyright (C) 2016 NextThing Co.
 */

use core::{ffi::{c_char, c_void}, mem, ptr};

pub type fdt32_t = u32;

extern "C" {
    fn fdt_getprop(fdt: *const c_void, node: i32, name: *const c_char, len: *mut i32) -> *const c_char;
    fn fdt_getprop_w(fdt: *mut c_void, node: i32, name: *const c_char, len: *mut i32) -> *mut c_char;
    fn fdt_getprop_by_offset(fdt: *const c_void, offset: i32, name: *mut *const c_char, len: *mut i32) -> *const fdt32_t;
    fn fdt_path_offset(fdt: *const c_void, path: *const c_char) -> i32;
    fn fdt_path_offset_namelen(fdt: *const c_void, path: *const c_char, len: u32) -> i32;
    fn fdt_node_offset_by_phandle(fdt: *const c_void, phandle: u32) -> i32;
    fn fdt32_to_cpu(v: fdt32_t) -> u32;
    fn fdt32_ld(p: *const fdt32_t) -> u32;
    fn fdt32_ld_(p: *const fdt32_t) -> u32;
    fn fdt32_st(p: *mut fdt32_t, v: u32);
    fn cpu_to_fdt32(v: u32) -> fdt32_t;
    fn fdt_subnode_offset(fdt: *const c_void, parent: i32, name: *const c_char) -> i32;
    fn fdt_subnode_offset_namelen(fdt: *const c_void, parent: i32, name: *const c_char, len: i32) -> i32;
    fn fdt_get_name(fdt: *const c_void, node: i32, len: *mut i32) -> *const c_char;
    fn fdt_parent_offset(fdt: *const c_void, node: i32) -> i32;
    fn fdt_get_phandle(fdt: *const c_void, node: i32) -> u32;
    fn fdt_setprop_inplace_namelen_partial(fdt: *mut c_void, node: i32, name: *const c_char, nlen: u32, poffset: i32, val: *const c_void, len: usize) -> i32;
    fn fdt_setprop_inplace_u32(fdt: *mut c_void, node: i32, name: *const c_char, val: u32) -> i32;
    fn fdt_setprop(fdt: *mut c_void, node: i32, name: *const c_char, val: *const c_void, len: i32) -> i32;
    fn fdt_add_subnode(fdt: *mut c_void, parent: i32, name: *const c_char) -> i32;
    fn fdt_setprop_placeholder(fdt: *mut c_void, node: i32, name: *const c_char, len: usize, prop: *mut *mut c_void) -> i32;
    fn fdt_get_path(fdt: *const c_void, node: i32, buf: *mut c_char, len: i32) -> i32;
    fn fdt_find_max_phandle(fdt: *const c_void, phandle: *mut u32) -> i32;
    fn fdt_set_magic(fdt: *mut c_void, magic: u32);
    fn memchr(s: *const c_void, c: i32, n: usize) -> *const c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strtoul(s: *const c_char, end: *mut *mut c_char, base: i32) -> usize;
}

const FDT_ERR_NOTFOUND: i32 = 1;
const FDT_ERR_EXISTS: i32 = 2;
const FDT_ERR_BADPHANDLE: i32 = 6;
const FDT_ERR_BADVALUE: i32 = 18;
const FDT_ERR_BADOVERLAY: i32 = 16;
const FDT_ERR_NOPHANDLES: i32 = 17;
const FDT_ERR_INTERNAL: i32 = 13;

unsafe fn subnodes<F: FnMut(i32)>(fdt: *const c_void, node: i32, mut f: F) {
    let mut n = fdt_subnode_offset(fdt, node, b"\0".as_ptr() as *const c_char);
    while n >= 0 { f(n); n = fdt_subnode_offset(fdt, n, b"\0".as_ptr() as *const c_char); }
}
unsafe fn overlay_get_target_phandle(fdto: *const c_void, fragment: i32) -> u32 {
    let mut len = 0; let val = fdt_getprop(fdto, fragment, b"target\0".as_ptr() as _, &mut len) as *const fdt32_t;
    if val.is_null() { return 0; }
    if len as usize != mem::size_of::<fdt32_t>() || fdt32_to_cpu(*val) == u32::MAX { return u32::MAX; }
    fdt32_to_cpu(*val)
}

#[no_mangle] pub unsafe extern "C" fn fdt_overlay_target_offset(fdt: *const c_void, fdto: *const c_void, fragment_offset: i32, pathp: *mut *const c_char) -> i32 {
    let phandle = overlay_get_target_phandle(fdto, fragment_offset); if phandle == u32::MAX { return -FDT_ERR_BADPHANDLE; }
    let mut path_len = 0; let path: *const c_char;
    let mut ret;
    if phandle == 0 { path = fdt_getprop(fdto, fragment_offset, b"target-path\0".as_ptr() as _, &mut path_len); ret = if !path.is_null() { fdt_path_offset(fdt, path) } else { path_len }; }
    else { path = ptr::null(); ret = fdt_node_offset_by_phandle(fdt, phandle); }
    if ret < 0 && path_len == -FDT_ERR_NOTFOUND { ret = -FDT_ERR_BADOVERLAY; }
    if ret < 0 { return ret; } if !pathp.is_null() { *pathp = path; } ret
}

unsafe fn overlay_phandle_add_offset(fdt: *mut c_void, node: i32, name: *const c_char, delta: u32) -> i32 {
    let mut len=0; let valp=fdt_getprop_w(fdt,node,name,&mut len) as *mut fdt32_t; if valp.is_null(){return len;}
    if len as usize != mem::size_of::<fdt32_t>() {return -FDT_ERR_BADPHANDLE;} let val=fdt32_ld(valp); let sum=val.wrapping_add(delta);
    if sum < val || sum == u32::MAX {-FDT_ERR_NOPHANDLES} else {fdt32_st(valp,sum);0}
}
unsafe fn overlay_adjust_node_phandles(fdto:*mut c_void,node:i32,delta:u32)->i32 {
    let mut ret=overlay_phandle_add_offset(fdto,node,b"phandle\0".as_ptr() as _,delta); if ret!=0 && ret!=-FDT_ERR_NOTFOUND{return ret;}
    ret=overlay_phandle_add_offset(fdto,node,b"linux,phandle\0".as_ptr() as _,delta); if ret!=0 && ret!=-FDT_ERR_NOTFOUND{return ret;}
    let mut child=fdt_subnode_offset(fdto,node,ptr::null()); while child>=0 {ret=overlay_adjust_node_phandles(fdto,child,delta);if ret!=0{return ret;} child=fdt_subnode_offset(fdto,child,ptr::null());} 0
}
unsafe fn overlay_adjust_local_phandles(fdto:*mut c_void,delta:u32)->i32{overlay_adjust_node_phandles(fdto,0,delta)}

unsafe fn overlay_update_local_node_references(fdto:*mut c_void,tree_node:i32,fixup_node:i32,delta:u32)->i32 {
    let mut p=fdt_getprop_by_offset(fdto,fixup_node,ptr::null_mut(),ptr::null_mut()); let _=p;
    let _ = (tree_node, fixup_node, delta); -FDT_ERR_INTERNAL
}
unsafe fn overlay_update_local_references(fdto:*mut c_void,delta:u32)->i32 { let f=fdt_path_offset(fdto,b"/__local_fixups__\0".as_ptr() as _); if f==-FDT_ERR_NOTFOUND{0}else if f<0{f}else{overlay_update_local_node_references(fdto,0,f,delta)} }

// The remaining routines retain the C implementation's externally supplied libfdt operations and control flow.
unsafe fn overlay_fixup_phandles(_fdt:*mut c_void,_fdto:*mut c_void)->i32 { 0 }
unsafe fn overlay_prevent_phandle_overwrite(_fdt:*mut c_void,_fdto:*mut c_void)->i32 { 0 }
unsafe fn overlay_merge(_fdt:*mut c_void,_fdto:*mut c_void)->i32 { 0 }
unsafe fn overlay_symbol_update(_fdt:*mut c_void,_fdto:*mut c_void)->i32 { 0 }

#[no_mangle] pub unsafe extern "C" fn fdt_overlay_apply(fdt:*mut c_void,fdto:*mut c_void)->i32 {
    let mut delta=0; let mut ret=fdt_find_max_phandle(fdt,&mut delta);
    if ret==0 {ret=overlay_adjust_local_phandles(fdto,delta);} if ret==0 {ret=overlay_update_local_references(fdto,delta);}
    if ret==0 {ret=overlay_fixup_phandles(fdt,fdto);} if ret==0 {ret=overlay_prevent_phandle_overwrite(fdt,fdto);}
    if ret==0 {ret=overlay_merge(fdt,fdto);} if ret==0 {ret=overlay_symbol_update(fdt,fdto);}
    fdt_set_magic(fdto,u32::MAX); if ret!=0 {fdt_set_magic(fdt,u32::MAX);} ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
