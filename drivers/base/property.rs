// SPDX-License-Identifier: GPL-2.0
/* Unified device property interface. */

// C dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    fn of_fwnode_handle(node: *mut of_node) -> *mut fwnode_handle;
    fn fwnode_property_present(f: *const fwnode_handle, p: *const c_char) -> bool;
    fn fwnode_property_read_bool(f: *const fwnode_handle, p: *const c_char) -> bool;
    fn fwnode_property_read_u8_array(f: *const fwnode_handle, p: *const c_char, v: *mut u8, n: usize) -> c_int;
    fn fwnode_property_read_u16_array(f: *const fwnode_handle, p: *const c_char, v: *mut u16, n: usize) -> c_int;
    fn fwnode_property_read_u32_array(f: *const fwnode_handle, p: *const c_char, v: *mut u32, n: usize) -> c_int;
    fn fwnode_property_read_u64_array(f: *const fwnode_handle, p: *const c_char, v: *mut u64, n: usize) -> c_int;
    fn fwnode_property_read_string_array(f: *const fwnode_handle, p: *const c_char, v: *mut *const c_char, n: usize) -> c_int;
    fn fwnode_property_read_string(f: *const fwnode_handle, p: *const c_char, v: *mut *const c_char) -> c_int;
    fn fwnode_property_match_string(f: *const fwnode_handle, p: *const c_char, s: *const c_char) -> c_int;
    fn fwnode_property_string_array_count(f: *const fwnode_handle, p: *const c_char) -> c_int;
    fn fwnode_property_get_reference_args(f: *const fwnode_handle, p: *const c_char, np: *const c_char, na: c_uint, i: c_uint, a: *mut fwnode_reference_args) -> c_int;
    fn fwnode_call_ptr_op(f: *const fwnode_handle, op: *const c_char, ...) -> *mut c_void;
    fn fwnode_call_int_op(f: *const fwnode_handle, op: *const c_char, ...) -> c_int;
    fn fwnode_call_bool_op(f: *const fwnode_handle, op: *const c_char, ...) -> bool;
    fn fwnode_has_op(f: *const fwnode_handle, op: *const c_char) -> bool;
    fn fwnode_handle_get(f: *mut fwnode_handle) -> *mut fwnode_handle;
    fn fwnode_handle_put(f: *mut fwnode_handle);
    fn fwnode_get_parent(f: *const fwnode_handle) -> *mut fwnode_handle;
    fn fwnode_get_next_child_node(f: *const fwnode_handle, c: *mut fwnode_handle) -> *mut fwnode_handle;
    fn fwnode_graph_get_next_endpoint(f: *const fwnode_handle, p: *mut fwnode_handle) -> *mut fwnode_handle;
    fn fwnode_graph_get_remote_endpoint(f: *const fwnode_handle) -> *mut fwnode_handle;
    fn fwnode_graph_get_port_parent(f: *const fwnode_handle) -> *mut fwnode_handle;
    fn fwnode_device_is_available(f: *const fwnode_handle) -> bool;
    fn fwnode_graph_parse_endpoint(f: *const fwnode_handle, e: *mut fwnode_endpoint) -> c_int;
    fn fwnode_get_name(f: *const fwnode_handle) -> *const c_char;
    fn fwnode_graph_for_each_endpoint(f: *const fwnode_handle, cb: extern "C" fn(*mut fwnode_handle));
    fn match_string(v: *const *const c_char, n: c_int, s: *const c_char) -> c_int;
    fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int;
}

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct fwnode_handle { pub secondary: *mut fwnode_handle }
#[repr(C)] pub struct device { pub of_node: *mut of_node, pub fwnode: *mut fwnode_handle }
#[repr(C)] pub struct of_node;
#[repr(C)] pub struct fwnode_reference_args { pub fwnode: *mut fwnode_handle }
#[repr(C)] pub struct fwnode_endpoint { pub port: u32, pub id: u32 }
#[repr(C)] pub enum dev_dma_attr { DEV_DMA_NOT_SUPPORTED = 0 }
type devcon_match_fn_t = unsafe extern "C" fn(*mut fwnode_handle, *const c_char, *mut c_void) -> *mut c_void;

#[inline] pub unsafe fn __dev_fwnode(dev: *mut device) -> *mut fwnode_handle {
    if !(*dev).of_node.is_null() { of_fwnode_handle((*dev).of_node) } else { (*dev).fwnode }
}
#[inline] pub unsafe fn __dev_fwnode_const(dev: *const device) -> *const fwnode_handle {
    if !(*dev).of_node.is_null() { of_fwnode_handle((*dev).of_node) } else { (*dev).fwnode }
}
pub unsafe fn device_property_present(d: *const device, p: *const c_char) -> bool { fwnode_property_present(__dev_fwnode_const(d), p) }
pub unsafe fn device_property_read_bool(d: *const device, p: *const c_char) -> bool { fwnode_property_read_bool(__dev_fwnode_const(d), p) }
pub unsafe fn device_property_read_u8_array(d:*const device,p:*const c_char,v:*mut u8,n:usize)->c_int{fwnode_property_read_u8_array(__dev_fwnode_const(d),p,v,n)}
pub unsafe fn device_property_read_u16_array(d:*const device,p:*const c_char,v:*mut u16,n:usize)->c_int{fwnode_property_read_u16_array(__dev_fwnode_const(d),p,v,n)}
pub unsafe fn device_property_read_u32_array(d:*const device,p:*const c_char,v:*mut u32,n:usize)->c_int{fwnode_property_read_u32_array(__dev_fwnode_const(d),p,v,n)}
pub unsafe fn device_property_read_u64_array(d:*const device,p:*const c_char,v:*mut u64,n:usize)->c_int{fwnode_property_read_u64_array(__dev_fwnode_const(d),p,v,n)}
pub unsafe fn device_property_read_string_array(d:*const device,p:*const c_char,v:*mut *const c_char,n:usize)->c_int{fwnode_property_read_string_array(__dev_fwnode_const(d),p,v,n)}
pub unsafe fn device_property_read_string(d:*const device,p:*const c_char,v:*mut *const c_char)->c_int{fwnode_property_read_string(__dev_fwnode_const(d),p,v)}
pub unsafe fn device_property_match_string(d:*const device,p:*const c_char,s:*const c_char)->c_int{fwnode_property_match_string(__dev_fwnode_const(d),p,s)}

pub unsafe fn fwnode_property_read_int_array(f:*const fwnode_handle,p:*const c_char,size:c_uint,v:*mut c_void,n:usize)->c_int {
    if f.is_null() { return -22; }
    let r=fwnode_call_int_op(f, core::ptr::null(),p,size,v,n);
    if r != -22 { r } else { fwnode_call_int_op((*f).secondary,core::ptr::null(),p,size,v,n) }
}
pub unsafe fn fwnode_property_read_u8_array_local(f:*const fwnode_handle,p:*const c_char,v:*mut u8,n:usize)->c_int{fwnode_property_read_int_array(f,p,1,v.cast(),n)}
pub unsafe fn fwnode_property_read_u16_array_local(f:*const fwnode_handle,p:*const c_char,v:*mut u16,n:usize)->c_int{fwnode_property_read_int_array(f,p,2,v.cast(),n)}
pub unsafe fn fwnode_property_read_u32_array_local(f:*const fwnode_handle,p:*const c_char,v:*mut u32,n:usize)->c_int{fwnode_property_read_int_array(f,p,4,v.cast(),n)}
pub unsafe fn fwnode_property_read_u64_array_local(f:*const fwnode_handle,p:*const c_char,v:*mut u64,n:usize)->c_int{fwnode_property_read_int_array(f,p,8,v.cast(),n)}
pub unsafe fn fwnode_property_read_string_local(f:*const fwnode_handle,p:*const c_char,v:*mut *const c_char)->c_int { let r=fwnode_property_read_string_array(f,p,v,1); if r<0 {r} else {0} }
pub unsafe fn fwnode_find_reference(f:*const fwnode_handle,p:*const c_char,i:c_uint)->*mut fwnode_handle { let mut a=fwnode_reference_args{fwnode:core::ptr::null_mut()}; let r=fwnode_property_get_reference_args(f,p,core::ptr::null(),0,i,&mut a); if r!=0 {r as isize as *mut fwnode_handle} else {a.fwnode} }
pub unsafe fn fwnode_get_next_parent(f:*mut fwnode_handle)->*mut fwnode_handle { let p=fwnode_get_parent(f); fwnode_handle_put(f); p }
pub unsafe fn fwnode_get_nth_parent(mut f:*mut fwnode_handle,mut depth:c_uint)->*mut fwnode_handle { if depth==0{return fwnode_handle_get(f)}; while !f.is_null(){f=fwnode_get_parent(f); depth-=1; if depth==0{return f}} core::ptr::null_mut() }
pub unsafe fn fwnode_handle_get_local(f:*mut fwnode_handle)->*mut fwnode_handle { if !fwnode_has_op(f,core::ptr::null()){f}else{fwnode_handle_get(f)} }
pub unsafe fn fwnode_device_is_available_local(f:*const fwnode_handle)->bool { if f.is_null(){false}else if !fwnode_has_op(f,core::ptr::null()){true}else{fwnode_call_bool_op(f,core::ptr::null())} }
pub unsafe fn fwnode_get_remote_port(f:*const fwnode_handle)->*mut fwnode_handle { fwnode_get_next_parent(fwnode_graph_get_remote_endpoint(f)) }
pub unsafe fn fwnode_graph_get_remote_port_parent_local(f:*const fwnode_handle)->*mut fwnode_handle { let e=fwnode_graph_get_remote_endpoint(f); let p=fwnode_graph_get_port_parent(e); fwnode_handle_put(e); p }
pub unsafe fn fwnode_graph_parse_endpoint_local(f:*const fwnode_handle,e:*mut fwnode_endpoint)->c_int { core::ptr::write_bytes(e,0,1); fwnode_graph_parse_endpoint(f,e) }

pub unsafe fn fwnode_get_name_prefix(f:*const fwnode_handle)->*const c_char { fwnode_call_ptr_op(f,core::ptr::null()) as *const c_char }
pub unsafe fn fwnode_get_parent_local(f:*const fwnode_handle)->*mut fwnode_handle { fwnode_call_ptr_op(f,core::ptr::null()) }
pub unsafe fn fwnode_get_next_child_node_local(f:*const fwnode_handle,c:*mut fwnode_handle)->*mut fwnode_handle {
    if f.is_null(){return core::ptr::null_mut()}; let n=fwnode_get_next_child_node(f,c); if n.is_null(){fwnode_get_next_child_node((*f).secondary,core::ptr::null_mut())}else{n}
}
pub unsafe fn fwnode_get_next_available_child_node(f:*const fwnode_handle,mut c:*mut fwnode_handle)->*mut fwnode_handle {
    if f.is_null(){return core::ptr::null_mut()}; loop { c=fwnode_get_next_child_node_local(f,c); if c.is_null()||fwnode_device_is_available_local(c){return c} }
}
pub unsafe fn device_get_next_child_node(d:*const device,c:*mut fwnode_handle)->*mut fwnode_handle { fwnode_get_next_child_node_local(__dev_fwnode_const(d),c) }
pub unsafe fn fwnode_get_named_child_node(f:*const fwnode_handle,n:*const c_char)->*mut fwnode_handle { fwnode_call_ptr_op(f,core::ptr::null(),n) }
pub unsafe fn device_get_named_child_node(d:*const device,n:*const c_char)->*mut fwnode_handle { fwnode_get_named_child_node(__dev_fwnode_const(d),n) }
pub unsafe fn fwnode_count_parents(mut f:*const fwnode_handle)->c_uint { let mut n=0; while !f.is_null(){f=fwnode_get_parent(f); n+=1;} n }
pub unsafe fn fwnode_get_child_node_count(f:*const fwnode_handle)->c_uint { let mut n=0; let mut c=core::ptr::null_mut(); while {c=fwnode_get_next_child_node_local(f,c);!c.is_null()}{n+=1}; n }
pub unsafe fn device_dma_supported(d:*const device)->bool { fwnode_call_bool_op(__dev_fwnode_const(d),core::ptr::null()) }
pub unsafe fn device_get_dma_attr(d:*const device)->dev_dma_attr { if !fwnode_has_op(__dev_fwnode_const(d),core::ptr::null()){dev_dma_attr::DEV_DMA_NOT_SUPPORTED}else{core::mem::transmute(fwnode_call_int_op(__dev_fwnode_const(d),core::ptr::null()))} }
pub unsafe fn fwnode_iomap(f:*mut fwnode_handle,i:c_int)->*mut c_void { fwnode_call_ptr_op(f,core::ptr::null(),i) }
pub unsafe fn fwnode_irq_get(f:*const fwnode_handle,i:c_uint)->c_int { let r=fwnode_call_int_op(f,core::ptr::null(),i); if r==0{-22}else{r} }
pub unsafe fn fwnode_irq_get_byname(f:*const fwnode_handle,n:*const c_char)->c_int { if n.is_null(){return -22}; let i=fwnode_property_match_string(f,core::ptr::null(),n); if i<0{i}else{fwnode_irq_get(f,i as c_uint)} }
pub unsafe fn fwnode_graph_get_next_endpoint_local(f:*const fwnode_handle,p:*mut fwnode_handle)->*mut fwnode_handle { let n=fwnode_graph_get_next_endpoint(f,p); if n.is_null(){fwnode_graph_get_next_endpoint((*f).secondary,core::ptr::null_mut())}else{n} }
pub unsafe fn fwnode_graph_get_remote_endpoint_local(f:*const fwnode_handle)->*mut fwnode_handle { fwnode_call_ptr_op(f,core::ptr::null()) }
pub unsafe fn fwnode_name_eq(f:*const fwnode_handle,n:*const c_char)->bool { let a=fwnode_get_name(f); if a.is_null(){false}else{strcasecmp(a,n)==0} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
