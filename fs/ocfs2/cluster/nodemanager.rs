// SPDX-License-Identifier: GPL-2.0-or-later
/* Literal translation of nodemanager.c; kernel and header dependencies remain external. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut o2nm_single_cluster: *mut o2nm_cluster;
    fn o2net_start_listening(n: *mut o2nm_node) -> c_int;
    fn o2net_complete_start_listening(n: *mut o2nm_node);
    fn o2net_stop_listening(n: *mut o2nm_node);
    fn o2net_disconnect_node(n: *mut o2nm_node);
    fn o2net_num_connected_peers() -> c_int;
    fn o2net_init() -> c_int; fn o2net_exit();
    fn o2net_register_hb_callbacks() -> c_int; fn o2net_unregister_hb_callbacks();
    fn o2hb_init(); fn o2hb_exit();
    fn o2hb_alloc_hb_set() -> *mut config_group; fn o2hb_free_hb_set(g: *mut config_group);
    fn o2cb_sys_init() -> c_int; fn o2cb_sys_shutdown();
    fn config_item_get(i: *mut config_item); fn config_item_put(i: *mut config_item);
    fn configfs_depend_item(s: *mut configfs_subsystem, i: *mut config_item) -> c_int;
    fn configfs_depend_item_unlocked(s: *mut configfs_subsystem, i: *mut config_item) -> c_int;
    fn configfs_undepend_item(i: *mut config_item);
}

const O2NM_MAX_NODES: usize = 255;
const O2NM_INVALID_NODE_NUM: u8 = 255;
const O2NM_FENCE_METHODS: usize = 2;
const O2NM_FENCE_RESET: u32 = 0;
const ENOSPC: c_int = 28; const ENOMEM: c_int = 12; const EINVAL: c_int = 22;
const EEXIST: c_int = 17; const EBUSY: c_int = 16; const ERANGE: c_int = 34;
const ENAMETOOLONG: c_int = 36;

#[repr(C)] pub struct config_item { pub ci_parent: *mut config_item, pub ci_namebuf: [c_char; 32], pub ci_type: *const c_void }
#[repr(C)] pub struct config_group { pub cg_item: config_item }
#[repr(C)] pub struct configfs_subsystem { pub su_group: config_group, pub su_mutex: c_void }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct o2nm_node { pub nd_item: config_item, pub nd_name: [c_char; 64], pub nd_num: usize, pub nd_ipv4_port: u16, pub nd_ipv4_address: u32, pub nd_local: usize, pub nd_set_attributes: c_ulong, pub nd_ip_node: rb_node, pub nd_lock: c_void }
#[repr(C)] pub struct o2nm_cluster { pub cl_group: config_group, pub cl_nodes_lock: c_void, pub cl_nodes: [*mut o2nm_node; O2NM_MAX_NODES], pub cl_nodes_bitmap: [c_ulong; 4], pub cl_node_ip_tree: rb_root, pub cl_has_local: usize, pub cl_local_node: u8, pub cl_reconnect_delay_ms: u32, pub cl_idle_timeout_ms: u32, pub cl_keepalive_delay_ms: u32, pub cl_fence_method: usize }
#[repr(C)] pub struct o2nm_node_group { pub ns_group: config_group }
#[repr(C)] pub struct o2nm_cluster_group { pub cs_subsys: configfs_subsystem }

static FENCE_METHOD_DESC: [&[u8]; O2NM_FENCE_METHODS] = [b"reset\0", b"panic\0"];

unsafe fn to_node(i: *mut config_item) -> *mut o2nm_node { i as *mut o2nm_node }
unsafe fn to_cluster(i: *mut config_item) -> *mut o2nm_cluster { i as *mut o2nm_cluster }
unsafe fn cluster_from_node(n: *mut o2nm_node) -> *mut o2nm_cluster { if (*n).nd_item.ci_parent.is_null() { core::ptr::null_mut() } else { (*n).nd_item.ci_parent.as_ref().unwrap().ci_parent as *mut o2nm_cluster } }

#[no_mangle] pub unsafe extern "C" fn o2nm_get_node_by_num(num: u8) -> *mut o2nm_node { if num as usize >= O2NM_MAX_NODES || o2nm_single_cluster.is_null() { return core::ptr::null_mut(); } let n=(*o2nm_single_cluster).cl_nodes[num as usize]; if !n.is_null(){config_item_get(&mut (*n).nd_item)} n }
#[no_mangle] pub unsafe extern "C" fn o2nm_configured_node_map(map:*mut c_ulong, bytes:usize)->c_int { if bytes < core::mem::size_of::<[c_ulong;4]>() { panic!() } if o2nm_single_cluster.is_null(){return -EINVAL} for i in 0..4 { *map.add(i)=(*o2nm_single_cluster).cl_nodes_bitmap[i]; } 0 }
#[no_mangle] pub unsafe extern "C" fn o2nm_get_node_by_ip(addr:u32)->*mut o2nm_node { if o2nm_single_cluster.is_null(){return core::ptr::null_mut()} let c=&mut *o2nm_single_cluster; for n in c.cl_nodes.iter(){if !n.is_null() && (**n).nd_ipv4_address==addr {config_item_get(&mut (**n).nd_item);return *n}} core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn o2nm_node_put(n:*mut o2nm_node){config_item_put(&mut (*n).nd_item)}
#[no_mangle] pub unsafe extern "C" fn o2nm_node_get(n:*mut o2nm_node){config_item_get(&mut (*n).nd_item)}
#[no_mangle] pub unsafe extern "C" fn o2nm_this_node()->u8{if !o2nm_single_cluster.is_null()&&(*o2nm_single_cluster).cl_has_local!=0{(*o2nm_single_cluster).cl_local_node}else{O2NM_INVALID_NODE_NUM}}

unsafe fn node_release(i:*mut config_item){drop(Box::from_raw(to_node(i)));}
unsafe fn parse_num(p:*const c_char)->Result<usize,c_int>{let s=core::ffi::CStr::from_ptr(p).to_bytes(); match core::str::from_utf8(s).unwrap_or("").trim().parse(){Ok(v)=>Ok(v),Err(_)=>Err(-EINVAL)}}
#[no_mangle] pub unsafe extern "C" fn o2nm_node_num_show(i:*mut config_item,page:*mut c_char)->isize{let n=to_node(i); libc_sprintf(page,b"%d\n\0".as_ptr() as _,(*n).nd_num as c_int)}
#[no_mangle] pub unsafe extern "C" fn o2nm_node_num_store(i:*mut config_item,page:*const c_char,count:usize)->isize{let n=to_node(i);let v=match parse_num(page){Ok(x)=>x,Err(e)=>return e as _};if v>=O2NM_MAX_NODES{return -ERANGE as _} if (*n).nd_set_attributes&(1<<2)==0||(*n).nd_set_attributes&(1<<1)==0{return -EINVAL as _}let c=cluster_from_node(n);if c.is_null(){return -EINVAL as _}if (*c).cl_nodes[v].is_null(){if (*n).nd_set_attributes&(1<<0)!=0{return -EBUSY as _}(*c).cl_nodes[v]=n;(*n).nd_num=v;(*n).nd_set_attributes|=1;count as _}else{-EEXIST as _}}
#[no_mangle] pub unsafe extern "C" fn o2nm_node_ipv4_port_store(i:*mut config_item,page:*const c_char,count:usize)->isize{let n=to_node(i);let v=match parse_num(page){Ok(x)=>x,Err(e)=>return e as _};if v==0{return -EINVAL as _}if v>=u16::MAX as usize{return -ERANGE as _}if (*n).nd_set_attributes&(1<<1)!=0{return -EBUSY as _}(*n).nd_set_attributes|=1<<1;(*n).nd_ipv4_port=(v as u16).to_be();count as _}
#[no_mangle] pub unsafe extern "C" fn o2nm_node_ipv4_address_store(i:*mut config_item,page:*const c_char,count:usize)->isize{let n=to_node(i);let s=core::ffi::CStr::from_ptr(page).to_str().unwrap_or("");let a:Vec<_>=s.trim().split('.').filter_map(|x|x.parse::<u32>().ok()).collect();if a.len()!=4||a.iter().any(|x|*x>255){return -EINVAL as _}let v=(a[3]<<24)|(a[2]<<16)|(a[1]<<8)|a[0];let c=cluster_from_node(n);if c.is_null(){return -EINVAL as _}for x in (*c).cl_nodes.iter(){if !x.is_null()&&**x as *const _ != n as *const _&&(**x).nd_ipv4_address==v{return -EEXIST as _}}if (*n).nd_set_attributes&(1<<2)!=0{return -EBUSY as _}(*n).nd_set_attributes|=1<<2;(*n).nd_ipv4_address=v;count as _}
#[no_mangle] pub unsafe extern "C" fn o2nm_node_local_store(i:*mut config_item,page:*const c_char,count:usize)->isize{let n=to_node(i);let v=match parse_num(page){Ok(x)=>(x!=0) as usize,Err(e)=>return e as _};if (*n).nd_set_attributes&7!=7{return -EINVAL as _}let c=cluster_from_node(n);if c.is_null(){return -EINVAL as _}if v!=0&&(*c).cl_has_local!=0&&(*c).cl_local_node!=(*n).nd_num as u8{return -EBUSY as _}if v!=0&&(*c).cl_has_local==0{let r=o2net_start_listening(n);if r!=0{return r as _}o2net_complete_start_listening(n);(*c).cl_has_local=1;(*c).cl_local_node=(*n).nd_num as u8}else if v==0&&(*c).cl_has_local!=0&&(*c).cl_local_node==(*n).nd_num as u8{o2net_stop_listening(n);(*c).cl_has_local=0;(*c).cl_local_node=O2NM_INVALID_NODE_NUM}(*n).nd_local=v;count as _}

extern "C" { fn libc_sprintf(p:*mut c_char,f:*const c_char,...)->isize; }

static mut o2nm_cluster_group: o2nm_cluster_group = o2nm_cluster_group { cs_subsys: configfs_subsystem { su_group: config_group { cg_item: config_item { ci_parent: core::ptr::null_mut(), ci_namebuf: [0;32], ci_type: core::ptr::null() } }, su_mutex: unsafe { core::mem::zeroed() } } };
#[no_mangle] pub unsafe extern "C" fn o2nm_depend_item(i:*mut config_item)->c_int{configfs_depend_item(&mut o2nm_cluster_group.cs_subsys,i)}
#[no_mangle] pub unsafe extern "C" fn o2nm_depend_item_unlocked(i:*mut config_item)->c_int{configfs_depend_item_unlocked(&mut o2nm_cluster_group.cs_subsys,i)}
#[no_mangle] pub unsafe extern "C" fn o2nm_undepend_item(i:*mut config_item){configfs_undepend_item(i)}
#[no_mangle] pub unsafe extern "C" fn o2nm_depend_node(num:u8)->c_int{let n=o2nm_get_node_by_num(num);if n.is_null(){-EINVAL}else{let r=o2nm_depend_item(&mut (*n).nd_item);o2nm_node_put(n);r}}
#[no_mangle] pub unsafe extern "C" fn o2nm_undepend_node(num:u8){let n=o2nm_get_node_by_num(num);if !n.is_null(){o2nm_undepend_item(&mut (*n).nd_item);o2nm_node_put(n)}}
#[no_mangle] pub unsafe extern "C" fn o2nm_depend_this_node()->c_int{o2nm_depend_node(o2nm_this_node())}
#[no_mangle] pub unsafe extern "C" fn o2nm_undepend_this_node(){o2nm_undepend_node(o2nm_this_node())}
#[no_mangle] pub unsafe extern "C" fn init_o2nm()->c_int{ o2hb_init();let r=o2net_init();if r!=0{o2hb_exit();return r}let r=o2net_register_hb_callbacks();if r!=0{o2net_exit();o2hb_exit();return r}0 }
#[no_mangle] pub unsafe extern "C" fn exit_o2nm(){o2net_unregister_hb_callbacks();o2cb_sys_shutdown();o2net_exit();o2hb_exit()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
