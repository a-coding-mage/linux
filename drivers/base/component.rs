// SPDX-License-Identifier: GPL-2.0
/* Componentized device handling. */

use core::ffi::c_void;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct component_master_ops { pub bind: unsafe extern "C" fn(*mut device) -> i32, pub unbind: unsafe extern "C" fn(*mut device) }
#[repr(C)] pub struct component_ops { pub bind: unsafe extern "C" fn(*mut device, *mut device, *mut c_void) -> i32, pub unbind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void)> }

#[repr(C)] pub struct component_match_array { pub data: *mut c_void, pub compare: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> i32>, pub compare_typed: Option<unsafe extern "C" fn(*mut device, i32, *mut c_void) -> i32>, pub release: Option<unsafe extern "C" fn(*mut device, *mut c_void)>, pub component: *mut component, pub duplicate: bool }
#[repr(C)] pub struct component_match { pub alloc: usize, pub num: usize, pub compare: *mut component_match_array }
#[repr(C)] pub struct aggregate_device { pub node: list_head, pub bound: bool, pub ops: *const component_master_ops, pub parent: *mut device, pub r#match: *mut component_match }
#[repr(C)] pub struct component { pub node: list_head, pub adev: *mut aggregate_device, pub bound: bool, pub ops: *const component_ops, pub subcomponent: i32, pub dev: *mut device }

static mut COMPONENT_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut AGGREGATE_DEVICES: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
extern "C" { static mut component_mutex: c_void; }

extern "C" {
    fn device_match_of_node(*mut device, *mut c_void) -> i32;
    fn device_match_name(*mut device, *mut c_void) -> i32;
    fn of_node_put(*mut c_void);
    fn dev_name(*mut device) -> *const i8;
    fn devres_open_group(*mut device, *mut c_void, u32) -> *mut c_void;
    fn devres_close_group(*mut device, *mut c_void);
    fn devres_release_group(*mut device, *mut c_void);
    fn devres_remove_group(*mut device, *mut c_void);
}

/* Kernel list, mutex, devres, allocation, logging, and errno helpers are supplied by dependencies. */
extern "C" {
    fn component_list_for_each(_: *mut list_head, _: *mut c_void);
    fn component_list_add_tail(_: *mut list_head, _: *mut list_head);
    fn component_list_add(_: *mut list_head, _: *mut list_head);
    fn component_list_del(_: *mut list_head);
    fn component_mutex_lock(_: *mut c_void);
    fn component_mutex_unlock(_: *mut c_void);
    fn component_alloc(_: usize, _: bool) -> *mut c_void;
    fn component_free(_: *mut c_void);
}

pub unsafe extern "C" fn component_compare_of(dev: *mut device, data: *mut c_void) -> i32 { device_match_of_node(dev, data) }
pub unsafe extern "C" fn component_release_of(_: *mut device, data: *mut c_void) { of_node_put(data); }
pub unsafe extern "C" fn component_compare_dev(dev: *mut device, data: *mut c_void) -> i32 { (dev == data) as i32 }
pub unsafe extern "C" fn component_compare_dev_name(dev: *mut device, data: *mut c_void) -> i32 { device_match_name(dev, data) }

unsafe fn aggregate_find(parent: *mut device, ops: *const component_master_ops) -> *mut aggregate_device {
    let mut p = AGGREGATE_DEVICES.next as *mut aggregate_device;
    while !p.is_null() { if (*p).parent == parent && (ops.is_null() || (*p).ops == ops) { return p; } p = (*p).node.next as *mut aggregate_device; }
    core::ptr::null_mut()
}
unsafe fn find_component(adev: *mut aggregate_device, mc: *mut component_match_array) -> *mut component {
    let mut p = COMPONENT_LIST.next as *mut component;
    while !p.is_null() { if ((*p).adev.is_null() || (*p).adev == adev) && (mc.as_ref().unwrap().compare.map_or(false, |f| f((*p).dev, (*mc).data) != 0) || mc.as_ref().unwrap().compare_typed.map_or(false, |f| f((*p).dev, (*p).subcomponent, (*mc).data) != 0)) { return p; } p = (*p).node.next as *mut component; }
    core::ptr::null_mut()
}
unsafe fn find_components(adev: *mut aggregate_device) -> i32 {
    let m = (*adev).r#match; for i in 0..(*m).num { let mc = (*m).compare.add(i); if !(*mc).component.is_null() { continue; } let c = find_component(adev, mc); if c.is_null() { return -6; } (*mc).duplicate = !(*c).adev.is_null(); (*mc).component = c; (*c).adev = adev; } 0
}
unsafe fn remove_component(adev: *mut aggregate_device, c: *mut component) { for i in 0..(*(*adev).r#match).num { let mc = (*(*adev).r#match).compare.add(i); if (*mc).component == c { (*mc).component = core::ptr::null_mut(); } } }

unsafe fn try_to_bring_up_aggregate_device(adev: *mut aggregate_device, component: *mut component) -> i32 {
    if find_components(adev) != 0 || (!component.is_null() && (*component).adev != adev) { return 0; }
    if devres_open_group((*adev).parent, adev as *mut c_void, 0).is_null() { return -12; }
    let ret = ((*(*adev).ops).bind)((*adev).parent); if ret < 0 { devres_release_group((*adev).parent, core::ptr::null_mut()); return ret; }
    devres_close_group((*adev).parent, core::ptr::null_mut()); (*adev).bound = true; 1
}
unsafe fn try_to_bring_up_masters(component: *mut component) -> i32 { let mut p = AGGREGATE_DEVICES.next as *mut aggregate_device; let mut ret = 0; while !p.is_null() { if !(*p).bound { ret = try_to_bring_up_aggregate_device(p, component); if ret != 0 { break; } } p = (*p).node.next as *mut aggregate_device; } ret }
unsafe fn take_down_aggregate_device(adev: *mut aggregate_device) { if (*adev).bound { ((*(*adev).ops).unbind)((*adev).parent); devres_release_group((*adev).parent, adev as *mut c_void); (*adev).bound = false; } }

pub unsafe extern "C" fn component_master_add_with_match(parent: *mut device, ops: *const component_master_ops, m: *mut component_match) -> i32 { let adev = component_alloc(core::mem::size_of::<aggregate_device>(), true) as *mut aggregate_device; if adev.is_null() { return -12; } (*adev).parent=parent; (*adev).ops=ops; (*adev).r#match=m; component_list_add(&mut (*adev).node, &mut AGGREGATE_DEVICES); let r=try_to_bring_up_aggregate_device(adev, core::ptr::null_mut()); if r<0 { component_free(adev as *mut c_void); } if r<0 {r} else {0} }
pub unsafe extern "C" fn component_master_del(parent: *mut device, ops: *const component_master_ops) { let a=aggregate_find(parent,ops); if !a.is_null(){take_down_aggregate_device(a); component_free(a as *mut c_void);} }
pub unsafe extern "C" fn component_master_is_bound(parent:*mut device,ops:*const component_master_ops)->bool{let a=aggregate_find(parent,ops);!a.is_null()&&(*a).bound}

unsafe fn component_unbind(c:*mut component,a:*mut aggregate_device,data:*mut c_void){if let Some(f)=(*(*c).ops).unbind{f((*c).dev,(*a).parent,data);}(*c).bound=false;devres_release_group((*c).dev,c as *mut c_void);}
pub unsafe extern "C" fn component_unbind_all(parent:*mut device,data:*mut c_void){let a=aggregate_find(parent,core::ptr::null());if a.is_null(){return;}let m=(*a).r#match;for i in (0..(*m).num).rev(){let x=(*m).compare.add(i);if !x.as_ref().unwrap().duplicate{component_unbind((*x).component,a,data);}}}
unsafe fn component_bind(c:*mut component,a:*mut aggregate_device,data:*mut c_void)->i32{if devres_open_group((*a).parent,core::ptr::null_mut(),0).is_null(){return -12;}if devres_open_group((*c).dev,c as *mut c_void,0).is_null(){devres_release_group((*a).parent,core::ptr::null_mut());return -12;}let r=((*(*c).ops).bind)((*c).dev,(*a).parent,data);if r==0{(*c).bound=true;devres_close_group((*c).dev,core::ptr::null_mut());devres_remove_group((*a).parent,core::ptr::null_mut());}else{devres_release_group((*c).dev,core::ptr::null_mut());devres_release_group((*a).parent,core::ptr::null_mut());}r}
pub unsafe extern "C" fn component_bind_all(parent:*mut device,data:*mut c_void)->i32{let a=aggregate_find(parent,core::ptr::null());if a.is_null(){return -22;}let m=(*a).r#match;for i in 0..(*m).num{let x=(*m).compare.add(i);if !(*x).duplicate{let r=component_bind((*x).component,a,data);if r!=0{return r;}}}0}

unsafe fn component_add_inner(dev:*mut device,ops:*const component_ops,sub:i32)->i32{let c=component_alloc(core::mem::size_of::<component>(),true)as*mut component;if c.is_null(){return -12;}(*c).dev=dev;(*c).ops=ops;(*c).subcomponent=sub;component_list_add_tail(&mut(*c).node,&mut COMPONENT_LIST);let r=try_to_bring_up_masters(c);if r<0{component_free(c as*mut c_void);}if r<0{r}else{0}}
pub unsafe extern "C" fn component_add_typed(d:*mut device,o:*const component_ops,s:i32)->i32{if s==0{-22}else{component_add_inner(d,o,s)}}
pub unsafe extern "C" fn component_add(d:*mut device,o:*const component_ops)->i32{component_add_inner(d,o,0)}
pub unsafe extern "C" fn component_del(d:*mut device,o:*const component_ops){let mut c=COMPONENT_LIST.next as*mut component;while !c.is_null(){if(*c).dev==d&&(*c).ops==o{component_list_del(&mut(*c).node);if !(*c).adev.is_null(){take_down_aggregate_device((*c).adev);remove_component((*c).adev,c);}component_free(c as*mut c_void);return;}c=(*c).node.next as*mut component;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
