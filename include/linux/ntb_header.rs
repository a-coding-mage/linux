/* Translated from linux/ntb.h.  External kernel types and constants are
 * intentionally referenced rather than implemented here. */

pub type u64 = core::primitive::u64;
pub type u32 = core::primitive::u32;
pub type resource_size_t = usize;
pub type dma_addr_t = u64;
pub type phys_addr_t = u64;
pub type irq_handler_t = unsafe extern "C" fn() -> i32;

#[repr(C)] pub struct ntb_client;
#[repr(C)] pub struct ntb_msi;
#[repr(C)] pub struct pci_dev;
#[repr(C)] pub struct module;
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct device_driver;
#[repr(C)] pub struct completion;
#[repr(C)] pub struct spinlock_t;

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum ntb_topo { NTB_TOPO_NONE = -1, NTB_TOPO_PRI, NTB_TOPO_SEC,
    NTB_TOPO_B2B_USD, NTB_TOPO_B2B_DSD, NTB_TOPO_SWITCH, NTB_TOPO_CROSSLINK }
pub unsafe fn ntb_topo_is_b2b(t: ntb_topo) -> i32 { if matches!(t, ntb_topo::NTB_TOPO_B2B_USD|ntb_topo::NTB_TOPO_B2B_DSD) {1} else {0} }
pub unsafe fn ntb_topo_string(t: ntb_topo) -> *mut i8 { match t {
    ntb_topo::NTB_TOPO_NONE=>b"NTB_TOPO_NONE\0", ntb_topo::NTB_TOPO_PRI=>b"NTB_TOPO_PRI\0",
    ntb_topo::NTB_TOPO_SEC=>b"NTB_TOPO_SEC\0", ntb_topo::NTB_TOPO_B2B_USD=>b"NTB_TOPO_B2B_USD\0",
    ntb_topo::NTB_TOPO_B2B_DSD=>b"NTB_TOPO_B2B_DSD\0", ntb_topo::NTB_TOPO_SWITCH=>b"NTB_TOPO_SWITCH\0",
    ntb_topo::NTB_TOPO_CROSSLINK=>b"NTB_TOPO_CROSSLINK\0" }.as_ptr() as *mut i8 }

#[repr(C)] #[derive(Copy, Clone)] pub enum ntb_speed { NTB_SPEED_AUTO=-1, NTB_SPEED_NONE=0, NTB_SPEED_GEN1=1, NTB_SPEED_GEN2=2, NTB_SPEED_GEN3=3, NTB_SPEED_GEN4=4 }
#[repr(C)] #[derive(Copy, Clone)] pub enum ntb_width { NTB_WIDTH_AUTO=-1, NTB_WIDTH_NONE=0, NTB_WIDTH_1=1, NTB_WIDTH_2=2, NTB_WIDTH_4=4, NTB_WIDTH_8=8, NTB_WIDTH_12=12, NTB_WIDTH_16=16, NTB_WIDTH_32=32 }
#[repr(C)] #[derive(Copy, Clone)] pub enum ntb_default_port { NTB_PORT_PRI_USD, NTB_PORT_SEC_DSD }
pub const NTB_DEF_PEER_CNT:i32=1; pub const NTB_DEF_PEER_IDX:i32=0;

#[repr(C)] pub struct ntb_client_ops { pub probe: Option<unsafe extern "C" fn(*mut ntb_client,*mut ntb_dev)->i32>, pub remove: Option<unsafe extern "C" fn(*mut ntb_client,*mut ntb_dev)> }
#[repr(C)] pub struct ntb_ctx_ops { pub link_event: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub db_event: Option<unsafe extern "C" fn(*mut core::ffi::c_void,i32)>, pub msg_event: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> }
#[repr(C)] pub struct ntb_dev_ops {
 pub port_number:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>, pub peer_port_count:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>, pub peer_port_number:Option<unsafe extern "C" fn(*mut ntb_dev,i32)->i32>, pub peer_port_idx:Option<unsafe extern "C" fn(*mut ntb_dev,i32)->i32>,
 pub link_is_up:Option<unsafe extern "C" fn(*mut ntb_dev,*mut ntb_speed,*mut ntb_width)->u64>, pub link_enable:Option<unsafe extern "C" fn(*mut ntb_dev,ntb_speed,ntb_width)->i32>, pub link_disable:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>,
 pub mw_count:Option<unsafe extern "C" fn(*mut ntb_dev,i32)->i32>, pub mw_get_align:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32,*mut resource_size_t,*mut resource_size_t,*mut resource_size_t)->i32>, pub mw_set_trans:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32,dma_addr_t,resource_size_t)->i32>, pub mw_clear_trans:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32)->i32>, pub peer_mw_count:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>, pub peer_mw_get_addr:Option<unsafe extern "C" fn(*mut ntb_dev,i32,*mut phys_addr_t,*mut resource_size_t)->i32>, pub peer_mw_set_trans:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32,u64,resource_size_t)->i32>, pub peer_mw_clear_trans:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32)->i32>,
 pub db_is_unsafe:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>, pub db_valid_mask:Option<unsafe extern "C" fn(*mut ntb_dev)->u64>, pub db_vector_count:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>, pub db_vector_mask:Option<unsafe extern "C" fn(*mut ntb_dev,i32)->u64>, pub db_read:Option<unsafe extern "C" fn(*mut ntb_dev)->u64>, pub db_set:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub db_clear:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub db_read_mask:Option<unsafe extern "C" fn(*mut ntb_dev)->u64>, pub db_set_mask:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub db_clear_mask:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>,
 pub peer_db_addr:Option<unsafe extern "C" fn(*mut ntb_dev,*mut phys_addr_t,*mut resource_size_t,*mut u64,i32)->i32>, pub peer_db_read:Option<unsafe extern "C" fn(*mut ntb_dev)->u64>, pub peer_db_set:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub peer_db_clear:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub peer_db_read_mask:Option<unsafe extern "C" fn(*mut ntb_dev)->u64>, pub peer_db_set_mask:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub peer_db_clear_mask:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>,
 pub spad_is_unsafe:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>, pub spad_count:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>, pub spad_read:Option<unsafe extern "C" fn(*mut ntb_dev,i32)->u32>, pub spad_write:Option<unsafe extern "C" fn(*mut ntb_dev,i32,u32)->i32>, pub peer_spad_addr:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32,*mut phys_addr_t)->i32>, pub peer_spad_read:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32)->u32>, pub peer_spad_write:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32,u32)->i32>,
 pub msg_count:Option<unsafe extern "C" fn(*mut ntb_dev)->i32>, pub msg_inbits:Option<unsafe extern "C" fn(*mut ntb_dev)->u64>, pub msg_outbits:Option<unsafe extern "C" fn(*mut ntb_dev)->u64>, pub msg_read_sts:Option<unsafe extern "C" fn(*mut ntb_dev)->u64>, pub msg_clear_sts:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub msg_set_mask:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub msg_clear_mask:Option<unsafe extern "C" fn(*mut ntb_dev,u64)->i32>, pub msg_read:Option<unsafe extern "C" fn(*mut ntb_dev,*mut i32,i32)->u32>, pub peer_msg_write:Option<unsafe extern "C" fn(*mut ntb_dev,i32,i32,u32)->i32>, pub get_dma_dev:Option<unsafe extern "C" fn(*mut ntb_dev)->*mut device> }

#[repr(C)] pub struct ntb_client { pub drv: device_driver, pub ops: ntb_client_ops }
#[repr(C)] pub struct ntb_dev { pub dev:device, pub pdev:*mut pci_dev, pub topo:ntb_topo, pub ops:*const ntb_dev_ops, pub ctx:*mut core::ffi::c_void, pub ctx_ops:*const ntb_ctx_ops, pub ctx_lock:spinlock_t, pub released:completion, pub msi:*mut ntb_msi }
#[repr(C)] pub struct ntb_msi_desc { pub addr_offset:u32, pub data:u32 }

extern "C" { pub fn __ntb_register_client(c:*mut ntb_client,m:*mut module,n:*const i8)->i32; pub fn ntb_unregister_client(c:*mut ntb_client); pub fn ntb_register_device(n:*mut ntb_dev)->i32; pub fn ntb_unregister_device(n:*mut ntb_dev); pub fn ntb_set_ctx(n:*mut ntb_dev,c:*mut core::ffi::c_void,o:*const ntb_ctx_ops)->i32; pub fn ntb_clear_ctx(n:*mut ntb_dev); pub fn ntb_link_event(n:*mut ntb_dev); pub fn ntb_db_event(n:*mut ntb_dev,v:i32); pub fn ntb_msg_event(n:*mut ntb_dev); pub fn ntb_default_port_number(n:*mut ntb_dev)->i32; pub fn ntb_default_peer_port_count(n:*mut ntb_dev)->i32; pub fn ntb_default_peer_port_number(n:*mut ntb_dev,p:i32)->i32; pub fn ntb_default_peer_port_idx(n:*mut ntb_dev,p:i32)->i32; }

pub unsafe fn ntb_port_number(n:*mut ntb_dev)->i32 { match (*n).ops.as_ref().unwrap().port_number {Some(f)=>f(n),None=>ntb_default_port_number(n)} }
pub unsafe fn ntb_peer_port_count(n:*mut ntb_dev)->i32 { match (*n).ops.as_ref().unwrap().peer_port_count {Some(f)=>f(n),None=>ntb_default_peer_port_count(n)} }
pub unsafe fn ntb_peer_port_number(n:*mut ntb_dev,p:i32)->i32 { match (*n).ops.as_ref().unwrap().peer_port_number {Some(f)=>f(n),None=>ntb_default_peer_port_number(n,p)} }
pub unsafe fn ntb_peer_port_idx(n:*mut ntb_dev,p:i32)->i32 { match (*n).ops.as_ref().unwrap().peer_port_idx {Some(f)=>f(n,p),None=>ntb_default_peer_port_idx(n,p)} }
pub unsafe fn ntb_link_is_up(n:*mut ntb_dev,s:*mut ntb_speed,w:*mut ntb_width)->u64 { ((*n).ops).as_ref().unwrap().link_is_up.unwrap()(n,s,w) }
pub unsafe fn ntb_link_enable(n:*mut ntb_dev,s:ntb_speed,w:ntb_width)->i32 { ((*n).ops).as_ref().unwrap().link_enable.unwrap()(n,s,w) }
pub unsafe fn ntb_link_disable(n:*mut ntb_dev)->i32 { ((*n).ops).as_ref().unwrap().link_disable.unwrap()(n) }
pub unsafe fn ntb_db_read(n:*mut ntb_dev)->u64 { ((*n).ops).as_ref().unwrap().db_read.unwrap()(n) }
pub unsafe fn ntb_db_clear(n:*mut ntb_dev,b:u64)->i32 { ((*n).ops).as_ref().unwrap().db_clear.unwrap()(n,b) }
pub unsafe fn ntb_db_valid_mask(n:*mut ntb_dev)->u64 { ((*n).ops).as_ref().unwrap().db_valid_mask.unwrap()(n) }
pub unsafe fn ntb_db_vector_count(n:*mut ntb_dev)->i32 { match ((*n).ops).as_ref().unwrap().db_vector_count {Some(f)=>f(n),None=>1} }
pub unsafe fn ntb_db_vector_mask(n:*mut ntb_dev,v:i32)->u64 { match ((*n).ops).as_ref().unwrap().db_vector_mask {Some(f)=>f(n,v),None=>ntb_db_valid_mask(n)} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
