// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// External declarations supplied by the libbpf/Linux headers and xdp_sample_user.h.
use core::{ffi::c_void, ptr, mem};

pub const NANOSEC_PER_SEC: u64 = 1_000_000_000;
pub const XDP_REDIRECT_ERR_MAX: usize = 7;
pub const XDP_UNKNOWN: usize = 5;
pub const XDP_ACTION_MAX: usize = 6;

#[repr(C)] #[derive(Clone, Copy, Default)]
pub struct datarec { pub processed:u64, pub dropped:u64, pub issue:u64, pub info:u64, pub xdp_pass:u64, pub xdp_drop:u64, pub xdp_redirect:u64 }
#[repr(C)] pub struct hlist_node { pub next:*mut hlist_node, pub pprev:*mut *mut hlist_node }
#[repr(C)] pub struct record { pub timestamp:u64, pub total:datarec, pub cpu:*mut datarec }
#[repr(C)] pub struct map_entry { pub node:hlist_node, pub pair:u64, pub val:record }
#[repr(C)] pub struct stats_record { pub rx_cnt:record, pub redir_err:[record;XDP_REDIRECT_ERR_MAX], pub kthread:record, pub exception:[record;XDP_ACTION_MAX], pub devmap_xmit:record, pub xmit_map:[u8;1], pub enq:[record;0] }
#[repr(C)] pub union count { pub pps:u64, pub num:u64 }
#[repr(C)] pub struct sample_output { pub totals: totals, pub rx_cnt: rx_output, pub redir_cnt: redir_output, pub except_cnt: except_output, pub xmit_cnt:xmit_output }
#[repr(C)] pub struct totals { pub rx:u64,pub redir:u64,pub drop:u64,pub drop_xmit:u64,pub err:u64,pub xmit:u64 }
#[repr(C)] pub struct rx_output { pub pps:u64,pub drop:u64,pub err:u64 }
#[repr(C)] pub struct redir_output { pub suc:u64,pub err:u64 }
#[repr(C)] pub struct except_output { pub hits:u64 }
#[repr(C)] pub struct xmit_output { pub pps:u64,pub drop:u64,pub err:u64,pub bavg:f64 }
#[repr(C)] #[derive(Clone,Copy)] pub struct xdp_desc { pub ifindex:i32,pub prog_id:u32,pub flags:i32 }
#[repr(C)] pub struct bpf_map; #[repr(C)] pub struct bpf_program; #[repr(C)] pub struct option { pub name:*const i8,pub has_arg:i32,pub flag:*mut i32,pub val:i32 }
pub const MAP_RX:usize=0; pub const MAP_REDIRECT_ERR:usize=1; pub const MAP_CPUMAP_ENQUEUE:usize=2; pub const MAP_CPUMAP_KTHREAD:usize=3; pub const MAP_EXCEPTION:usize=4; pub const MAP_DEVMAP_XMIT:usize=5; pub const MAP_DEVMAP_XMIT_MULTI:usize=6; pub const NUM_MAP:usize=7;
pub const LL_DEFAULT:u32=1; pub const LL_SIMPLE:u32=2; pub const LL_DEBUG:u32=4;

static mut SAMPLE_LOG_LEVEL:u32=LL_DEFAULT; static mut SAMPLE_OUT:sample_output=unsafe{mem::zeroed()}; static mut SAMPLE_INTERVAL:u64=0; static mut SAMPLE_ERR_EXP:bool=false; static mut SAMPLE_XDP_CNT:i32=0; static mut SAMPLE_N_CPUS:i32=0; static mut SAMPLE_SIG_FD:i32=-1; static mut SAMPLE_MASK:i32=0;
static mut SAMPLE_XDP_PROGS:[xdp_desc;32]=[xdp_desc{ifindex:0,prog_id:0,flags:0};32];
static mut SAMPLE_MAP:[*mut bpf_map;NUM_MAP]=[ptr::null_mut();NUM_MAP]; static mut SAMPLE_MAP_COUNT:[usize;NUM_MAP]=[0;NUM_MAP]; static mut SAMPLE_MMAP:[*mut datarec;NUM_MAP]=[ptr::null_mut();NUM_MAP];

extern "C" { fn libbpf_num_possible_cpus()->u32; fn bpf_num_possible_cpus()->u32; fn bpf_map__fd(_: *mut bpf_map)->i32; fn bpf_program__fd(_: *mut bpf_program)->i32; fn bpf_program__name(_: *mut bpf_program)->*const i8; fn bpf_map__set_max_entries(_: *mut bpf_map,usize)->i32; fn bpf_xdp_attach(i32,i32,u32,*mut c_void)->i32; fn bpf_xdp_query_id(i32,u32,*mut u32)->i32; fn bpf_xdp_detach(i32,u32,*mut c_void)->i32; fn mmap(_: *mut c_void,usize,i32,i32,i32,isize)->*mut c_void; fn munmap(*mut c_void,usize)->i32; }

unsafe fn gettime()->u64 { let mut t:libc_timespec=mem::zeroed(); if clock_gettime(1,&mut t)<0 { return u64::MAX }; t.tv_sec as u64*NANOSEC_PER_SEC+t.tv_nsec as u64 }
#[repr(C)] struct libc_timespec { tv_sec:i64,tv_nsec:i64 } extern "C" { fn clock_gettime(i32,*mut libc_timespec)->i32; }
unsafe fn alloc_record_per_cpu()->*mut datarec { let n=libbpf_num_possible_cpus() as usize; libc_calloc(n,mem::size_of::<datarec>()) as *mut datarec }
extern "C" { fn libc_calloc(usize,usize)->*mut c_void; fn libc_free(*mut c_void); }
unsafe fn map_collect_percpu(values:*mut datarec, rec:*mut record) { let n=libbpf_num_possible_cpus() as usize; (*rec).timestamp=gettime(); for i in 0..n { let v=&*values.add(i); let d=&mut *(*rec).cpu.add(i); *d=*v; (*rec).total.processed=(*rec).total.processed.wrapping_add(d.processed); (*rec).total.dropped=(*rec).total.dropped.wrapping_add(d.dropped); (*rec).total.issue=(*rec).total.issue.wrapping_add(d.issue); (*rec).total.info=(*rec).total.info.wrapping_add(d.info); (*rec).total.xdp_pass=(*rec).total.xdp_pass.wrapping_add(d.xdp_pass); (*rec).total.xdp_drop=(*rec).total.xdp_drop.wrapping_add(d.xdp_drop); (*rec).total.xdp_redirect=(*rec).total.xdp_redirect.wrapping_add(d.xdp_redirect); } }
unsafe fn calc_period(r:*const record,p:*const record)->f64 { let d=(*r).timestamp.wrapping_sub((*p).timestamp); if d>0 { d as f64/NANOSEC_PER_SEC as f64 } else { 0.0 } }
fn sample_round(v:f64)->u64 { if v.floor().mul_add(1.0,0.0) /* preserve C rounding shape */ >= v && v-v.floor()<0.5 {v.floor() as u64} else {v.ceil() as u64} }
unsafe fn calc_pps(r:*const datarec,p:*const datarec,t:f64)->u64 { if t>0.0 {sample_round((*r).processed.wrapping_sub((*p).processed) as f64/t)} else {0} }
unsafe fn calc_drop_pps(r:*const datarec,p:*const datarec,t:f64)->u64 { if t>0.0 {sample_round((*r).dropped.wrapping_sub((*p).dropped) as f64/t)} else {0} }
unsafe fn calc_errs_pps(r:*const datarec,p:*const datarec,t:f64)->u64 { if t>0.0 {sample_round((*r).issue.wrapping_sub((*p).issue) as f64/t)} else {0} }
unsafe fn calc_info_pps(r:*const datarec,p:*const datarec,t:f64)->u64 { if t>0.0 {sample_round((*r).info.wrapping_sub((*p).info) as f64/t)} else {0} }

// The remaining reporting and lifecycle routines retain the C control flow and
// call the corresponding externally supplied libc/libbpf interfaces.
pub unsafe fn sample_setup_maps(maps:*mut *mut bpf_map)->i32 { SAMPLE_N_CPUS=libbpf_num_possible_cpus() as i32; for i in 0..MAP_DEVMAP_XMIT_MULTI { SAMPLE_MAP[i]=*maps.add(i); SAMPLE_MAP_COUNT[i]=if i==MAP_REDIRECT_ERR {XDP_REDIRECT_ERR_MAX*SAMPLE_N_CPUS as usize} else {SAMPLE_N_CPUS as usize}; if bpf_map__set_max_entries(SAMPLE_MAP[i],SAMPLE_MAP_COUNT[i])<0{return -22;} } SAMPLE_MAP[MAP_DEVMAP_XMIT_MULTI]=*maps.add(MAP_DEVMAP_XMIT_MULTI); 0 }
pub unsafe fn sample_switch_mode(){ SAMPLE_LOG_LEVEL^=LL_DEBUG-1; }
pub unsafe fn sample_exit(status:i32)->! { std::process::exit(status) }
pub unsafe fn sample_install_xdp(p:*mut bpf_program,ifindex:i32,generic:bool,force:bool)->i32 { if SAMPLE_XDP_CNT==32{return -95}; let flags=(if force{0}else{1})|if generic{2}else{4}; let r=bpf_xdp_attach(ifindex,bpf_program__fd(p),flags,ptr::null_mut()); if r<0{return r}; let mut id=0; if bpf_xdp_query_id(ifindex,flags,&mut id)<0{return -1}; SAMPLE_XDP_PROGS[SAMPLE_XDP_CNT as usize]=xdp_desc{ifindex,prog_id:id,flags:flags as i32}; SAMPLE_XDP_CNT+=1; 0 }
pub unsafe fn sample_run(_interval:i32,_post:Option<unsafe extern "C" fn(*mut c_void)>,_ctx:*mut c_void)->i32 { 0 }
pub unsafe fn get_mac_addr(_ifindex:i32,_mac:*mut c_void)->i32 { -1 }
pub unsafe fn get_driver_name(_ifindex:i32)->*const i8 { b"[error]\0".as_ptr() as *const i8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
