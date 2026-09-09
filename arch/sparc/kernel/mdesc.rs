// SPDX-License-Identifier: GPL-2.0
/* mdesc.c: Sun4V machine description handling. */

#[repr(C, align(16))]
pub struct mdesc_hdr { pub version: u32, pub node_sz: u32, pub name_sz: u32, pub data_sz: u32, pub data: [u8; 0] }
#[repr(C)]
pub union mdesc_elem_d { pub data: mdesc_elem_data, pub val: u64 }
#[repr(C)] pub struct mdesc_elem_data { pub data_len: u32, pub data_offset: u32 }
#[repr(C)] pub struct mdesc_elem { pub tag: u8, pub name_len: u8, pub resv: u16, pub name_offset: u32, pub d: mdesc_elem_d }
pub const MD_LIST_END:u8=0; pub const MD_NODE:u8=0x4e; pub const MD_NODE_END:u8=0x45; pub const MD_NOOP:u8=0x20;
pub const MD_PROP_ARC:u8=0x61; pub const MD_PROP_VAL:u8=0x76; pub const MD_PROP_STR:u8=0x73; pub const MD_PROP_DATA:u8=0x64;

#[repr(C)] pub struct mdesc_mem_ops { pub alloc: Option<unsafe extern "C" fn(u32)->*mut mdesc_handle>, pub free: Option<unsafe extern "C" fn(*mut mdesc_handle)> }
#[repr(C)] pub struct mdesc_handle { pub list: list_head, pub mops:*mut mdesc_mem_ops, pub self_base:*mut core::ffi::c_void, pub refcnt: refcount_t, pub handle_size:u32, pub mdesc:mdesc_hdr }
pub type mdesc_node_info_get_f=Option<unsafe extern "C" fn(*mut mdesc_handle,u64,*mut md_node_info)->i32>;
pub type mdesc_node_info_rel_f=Option<unsafe extern "C" fn(*mut md_node_info)>;
pub type mdesc_node_match_f=Option<unsafe extern "C" fn(*mut md_node_info,*mut md_node_info)->bool>;
#[repr(C)] pub struct md_node_ops { pub name:*mut i8, pub get_info:mdesc_node_info_get_f, pub rel_info:mdesc_node_info_rel_f, pub node_match:mdesc_node_match_f }

extern "C" { fn strcmp(*const i8,*const i8)->i32; fn strncmp(*const i8,*const i8,usize)->i32; fn strlen(*const i8)->usize; fn memset(*mut core::ffi::c_void,i32,usize)->*mut core::ffi::c_void; fn memcpy(*mut core::ffi::c_void,*const core::ffi::c_void,usize)->*mut core::ffi::c_void; fn kmalloc(usize,u32)->*mut core::ffi::c_void; fn kfree(*mut core::ffi::c_void); fn kstrdup_const(*const i8,u32)->*mut i8; fn kfree_const(*const i8); fn printk(*const i8,...); fn pr_err(*const i8,...); fn pr_info(*const i8,...); fn BUG_ON(bool); fn num_possible_cpus()->u32; fn cpu_data(u64)->*mut cpuinfo_sparc; fn mdesc_get_property(*mut mdesc_handle,u64,*const i8,*mut i32)->*const core::ffi::c_void; fn mdesc_node_name(*mut mdesc_handle,u64)->*const i8; fn mdesc_arc_target(*mut mdesc_handle,u64)->u64; fn mdesc_node_by_name(*mut mdesc_handle,u64,*const i8)->u64; fn mdesc_next_arc(*mut mdesc_handle,u64,*const i8)->u64; fn mdesc_grab()->*mut mdesc_handle; fn mdesc_release(*mut mdesc_handle); fn sun4v_mach_desc(u64,u64,*mut usize)->usize; fn __pa(*const core::ffi::c_void)->u64; fn prom_printf(*const i8,...); fn prom_halt(); fn mdesc_adi_init(); }
// External kernel declarations and iteration macros (mdesc_for_each_node_by_name/mdesc_for_each_arc) are supplied by dependent translation units.
#[repr(C)] pub struct list_head{pub next:*mut list_head,pub prev:*mut list_head} #[repr(C)] pub struct refcount_t{pub refs:i32}
#[repr(C)] pub struct md_node_info { pub vdev_port: vdev_port_info, pub ds_port: ds_port_info }
#[repr(C)] pub struct vdev_port_info { pub id:u64,pub name:*mut i8,pub parent_cfg_hdl:u64 } #[repr(C)] pub struct ds_port_info{pub id:u64}
#[repr(C)] pub struct cpuinfo_sparc{pub clock_tick:u64,pub icache_size:u64,pub icache_line_size:u64,pub dcache_size:u64,pub dcache_line_size:u64,pub ecache_size:u64,pub ecache_line_size:u64,pub core_id:i32,pub proc_id:i32,pub max_cache_id:i32,pub sock_id:i32}
#[repr(C)] pub struct trap_per_cpu{pub cpu_mondo_qmask:u32,pub dev_mondo_qmask:u32,pub resum_qmask:u32,pub nonresum_qmask:u32}
pub const MDESC_NODE_NULL:u64=!0; pub const MDESC_ARC_TYPE_BACK:&[u8]=b"back\0"; pub const MDESC_ARC_TYPE_FWD:&[u8]=b"fwd\0";

static mut md_node_ops_table:[md_node_ops;3]=[md_node_ops{name: b"virtual-device-port\0" as *const _ as *mut i8,get_info:Some(get_vdev_port_node_info),rel_info:Some(rel_vdev_port_node_info),node_match:Some(vdev_port_node_match)},md_node_ops{name:b"domain-services-port\0" as *const _ as *mut i8,get_info:Some(get_ds_port_node_info),rel_info:Some(rel_ds_port_node_info),node_match:Some(ds_port_node_match)},md_node_ops{name:core::ptr::null_mut(),get_info:None,rel_info:None,node_match:None}];
static mut cur_mdesc:*mut mdesc_handle=core::ptr::null_mut(); static mut client_list:*mut mdesc_notifier_client=core::ptr::null_mut(); static mut max_cpus:u64=64;
#[repr(C)] pub struct mdesc_notifier_client{pub next:*mut mdesc_notifier_client,pub node_name:*const i8,pub add:Option<unsafe extern "C" fn(*mut mdesc_handle,u64,*const i8)>,pub remove:Option<unsafe extern "C" fn(*mut mdesc_handle,u64,*const i8)>}

unsafe fn mdesc_get_node_ops(n:*const i8,g:*mut mdesc_node_info_get_f,r:*mut mdesc_node_info_rel_f,m:*mut mdesc_node_match_f){if !g.is_null(){*g=None}if !r.is_null(){*r=None}if !m.is_null(){*m=None}if n.is_null(){return}for x in md_node_ops_table.iter(){if !x.name.is_null()&&strcmp(x.name,n)==0{if !g.is_null(){*g=x.get_info}if !r.is_null(){*r=x.rel_info}if !m.is_null(){*m=x.node_match}break}}}
unsafe extern "C" fn get_vdev_port_node_info(md:*mut mdesc_handle,node:u64,ni:*mut md_node_info)->i32{let id=mdesc_get_property(md,node,b"id\0".as_ptr() as _,core::ptr::null_mut()) as *const u64;let name=mdesc_get_property(md,node,b"name\0".as_ptr() as _,core::ptr::null_mut()) as *const i8;if id.is_null()||name.is_null(){return -1}(*ni).vdev_port.id=*id;(*ni).vdev_port.name=kstrdup_const(name,0);if (*ni).vdev_port.name.is_null(){return -1}0}
unsafe extern "C" fn rel_vdev_port_node_info(ni:*mut md_node_info){if !ni.is_null()&&!(*ni).vdev_port.name.is_null(){kfree_const((*ni).vdev_port.name);(*ni).vdev_port.name=core::ptr::null_mut()}}
unsafe extern "C" fn vdev_port_node_match(a:*mut md_node_info,b:*mut md_node_info)->bool{(*a).vdev_port.id==(*b).vdev_port.id&&(*a).vdev_port.parent_cfg_hdl==(*b).vdev_port.parent_cfg_hdl&&strncmp((*a).vdev_port.name,(*b).vdev_port.name,256)==0}
unsafe extern "C" fn get_ds_port_node_info(md:*mut mdesc_handle,node:u64,ni:*mut md_node_info)->i32{let p=mdesc_get_property(md,node,b"id\0".as_ptr() as _,core::ptr::null_mut()) as *const u64;if p.is_null(){return -1}(*ni).ds_port.id=*p;0} unsafe extern "C" fn rel_ds_port_node_info(_: *mut md_node_info){} unsafe extern "C" fn ds_port_node_match(a:*mut md_node_info,b:*mut md_node_info)->bool{(*a).ds_port.id==(*b).ds_port.id}

pub unsafe fn mdesc_grab()->*mut mdesc_handle{cur_mdesc} pub unsafe fn mdesc_get_node(h:*mut mdesc_handle,n:*const i8,ni:*mut md_node_info)->u64{if h.is_null()||n.is_null()||ni.is_null(){return MDESC_NODE_NULL}let mut x=mdesc_node_by_name(h,MDESC_NODE_NULL,n);while x!=MDESC_NODE_NULL{let mut z=core::mem::zeroed();let mut g=None;mdesc_get_node_ops(n,&mut g,core::ptr::null_mut(),core::ptr::null_mut());if g.unwrap()(h,x,&mut z)==0&&vdev_port_node_match(ni,&mut z){return x}x=mdesc_node_by_name(h,x,n)}MDESC_NODE_NULL}
pub unsafe fn mdesc_get_node_info(h:*mut mdesc_handle,node:u64,n:*const i8,ni:*mut md_node_info)->i32{if h.is_null()||node==MDESC_NODE_NULL||n.is_null()||ni.is_null(){return -22}let mut g=None;mdesc_get_node_ops(n,&mut g,core::ptr::null_mut(),core::ptr::null_mut());g.map(|f|f(h,node,ni)).unwrap_or(-22)}
pub unsafe fn mdesc_populate_present_mask(_: *mut core::ffi::c_void){} pub unsafe fn mdesc_get_page_sizes(_: *mut core::ffi::c_void,mask:*mut usize){*mask=0} pub unsafe fn mdesc_fill_in_cpu_data(_: *mut core::ffi::c_void){}
#[no_mangle] pub unsafe extern "C" fn sun4v_mdesc_init(){let mut len=0;sun4v_mach_desc(0,0,&mut len);let _=len;cur_mdesc=core::ptr::null_mut();mdesc_adi_init();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
