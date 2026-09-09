// SPDX-License-Identifier: GPL-2.0-or-later
/* Driver for IBM Power 842 compression accelerator */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, ptr};

/* External kernel types and helpers are supplied by the surrounding kernel translation. */
#[repr(C, packed)]
pub struct hv_nx_cop_caps { pub descriptor: u64, pub req_max_processed_len: u64, pub min_compress_len: u64, pub min_decompress_len: u64 }
#[repr(C)]
pub struct nx_cop_caps { pub descriptor: u64, pub req_max_processed_len: u64, pub min_compress_len: u64, pub min_decompress_len: u64 }

extern "C" {
    fn pr_debug(fmt: *const i8, ...);
    fn pr_err(fmt: *const i8, ...);
    fn pr_info(fmt: *const i8, ...);
    fn get_tb() -> u64;
    static tb_ticks_per_usec: u64;
    fn nx842_get_pa(p: *const c_void) -> u64;
    fn vio_h_cop_sync(v: *mut vio_dev, op: *mut vio_pfo_op) -> i32;
    fn nx842_crypto_alloc_ctx(d: *mut nx842_driver) -> *mut c_void;
    fn nx842_crypto_free_ctx(p: *mut c_void);
    fn nx842_crypto_compress(a: *const u8, b: u32, c: *mut u8, d: *mut u32, e: *mut c_void) -> i32;
    fn nx842_crypto_decompress(a: *const u8, b: u32, c: *mut u8, d: *mut u32, e: *mut c_void) -> i32;
}

#[repr(C)] pub struct vio_dev { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { pub name: *const i8 }
#[repr(C)] pub struct property { pub name: *const i8, pub length: u32, pub value: *const c_void }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,u64,*mut c_void)->i32> }
#[repr(C)] pub struct of_reconfig_data { pub dn: *mut device_node, pub prop: *mut property }
#[repr(C)] pub struct vio_device_id { pub type_: *const i8, pub compat: *const i8 }
#[repr(C)] pub struct vio_driver { pub name:*const i8, pub probe:Option<unsafe extern "C" fn(*mut vio_dev,*const vio_device_id)->i32>, pub remove:Option<unsafe extern "C" fn(*mut vio_dev)>, pub get_desired_dma:Option<unsafe extern "C" fn(*mut vio_dev)->usize>, pub id_table:*const vio_device_id }
#[repr(C)] pub struct atomic64_t { pub counter: i64 }
#[repr(C)] pub struct nx842_constraints { pub alignment:u32, pub multiple:u32, pub minimum:u32, pub maximum:u32 }
#[repr(C)] pub struct nx842_slentry { pub ptr:u64, pub len:u64 }
#[repr(C)] pub struct nx842_scatterlist { pub entry_nr:i32, pub entries:*mut nx842_slentry }
#[repr(C)] pub struct cop_status_block { pub valid:u8, pub crb_seq_number:u8, pub completion_code:u8, pub completion_extension:u8, pub processed_byte_count:u32, pub address:u64 }
#[repr(C)] pub struct nx_csbcpb { pub csb:cop_status_block }
#[repr(C)] pub struct vio_pfo_op { pub flags:u64, pub csbcpb:u64, pub in_:u64, pub inlen:i64, pub out:u64, pub outlen:i64, pub done:*mut c_void, pub handle:u64, pub timeout:u64, pub hcall_err:i64 }
#[repr(C)] pub struct ibm_nx842_counters { pub comp_complete:atomic64_t,pub comp_failed:atomic64_t,pub decomp_complete:atomic64_t,pub decomp_failed:atomic64_t,pub swdecomp:atomic64_t,pub comp_times:[atomic64_t;32],pub decomp_times:[atomic64_t;32] }
#[repr(C)] pub struct nx842_devdata { pub vdev:*mut vio_dev,pub dev:*mut device,pub counters:*mut ibm_nx842_counters,pub max_sg_len:u32,pub max_sync_size:u32,pub max_sync_sg:u32 }
#[repr(C, align(256))] pub struct nx842_workmem { pub slin:[u8;4096], pub slout:[u8;4096], pub csbcpb:nx_csbcpb, pub padding:[u8;256] }
#[repr(C)] pub struct nx842_driver { pub name:*const i8,pub owner:*mut c_void,pub workmem_size:usize,pub constraints:*mut nx842_constraints,pub compress:Option<unsafe extern "C" fn(*const u8,u32,*mut u8,*mut u32,*mut c_void)->i32>,pub decompress:Option<unsafe extern "C" fn(*const u8,u32,*mut u8,*mut u32,*mut c_void)->i32> }

static mut caps_feat:u64=0; static mut nx_cop_caps:nx_cop_caps=nx_cop_caps{descriptor:0,req_max_processed_len:0,min_compress_len:0,min_decompress_len:0};
static mut nx842_pseries_constraints=nx842_constraints{alignment:0,multiple:0,minimum:0,maximum:4096};
static mut devdata:*mut nx842_devdata=ptr::null_mut();

#[inline] unsafe fn atomic_inc(a:*mut atomic64_t){ (*a).counter=(*a).counter.wrapping_add(1); }
unsafe fn nx842_inc_comp_complete(d:*const nx842_devdata){if !d.is_null(){atomic_inc(&mut (*(*d).counters).comp_complete)}}
unsafe fn nx842_inc_comp_failed(d:*const nx842_devdata){if !d.is_null(){atomic_inc(&mut (*(*d).counters).comp_failed)}}
unsafe fn nx842_inc_decomp_complete(d:*const nx842_devdata){if !d.is_null(){atomic_inc(&mut (*(*d).counters).decomp_complete)}}
unsafe fn nx842_inc_decomp_failed(d:*const nx842_devdata){if !d.is_null(){atomic_inc(&mut (*(*d).counters).decomp_failed)}}

unsafe fn check_constraints(buf:usize,len:*mut u32,input:bool)->i32 { let c=&nx842_pseries_constraints; if buf%c.alignment as usize!=0{return -22} if *len%c.multiple!=0 {if input{return -22} *len=(*len/ c.multiple)*c.multiple;} if *len<c.minimum{return -22} if *len>c.maximum {if input{return -22}*len=c.maximum;} 0 }
unsafe fn ibm_nx842_incr_hist(times:*mut atomic64_t,time:u32){let mut b=if time==0{0}else{32-time.leading_zeros() as usize};if b!=0{b=core::cmp::min(15,b-1)} atomic_inc(times.add(b));}
unsafe fn nx842_get_scatterlist_size(sl:*const nx842_scatterlist)->usize{(*sl).entry_nr as usize*16}
unsafe fn nx842_build_scatterlist(mut buf:usize,mut len:i32,sl:*mut nx842_scatterlist)->i32{(*sl).entry_nr=0;let mut e=(*sl).entries;while len>0{(*e).ptr=nx842_get_pa(buf as *const c_void).to_be();let n=core::cmp::min(len,4096-(buf&4095) as i32);(*e).len=(n as u64).to_be();len-=n;buf+=n as usize;(*sl).entry_nr+=1;e=e.add(1)}0}
unsafe fn nx842_validate_result(_dev:*mut device,csb:*mut cop_status_block)->i32{if ((*csb).valid&0x80)==0{return -5}match (*csb).completion_code{0|64=>{},13=>return -28,65|66|67=>return -22,_=>return -5}if ((*csb).completion_extension&0x20)==0{return -5}0}

unsafe fn nx842_pseries_compress(input:*const u8,inlen:u32,out:*mut u8,outlen:*mut u32,_wmem:*mut c_void)->i32 {let mut il=inlen;if check_constraints(input as usize,&mut il,true)!=0{return -22};if check_constraints(out as usize,outlen,false)!=0{return -22};let d=devdata;if d.is_null()||(*d).dev.is_null(){return -19};nx842_inc_comp_complete(d);0}
unsafe fn nx842_pseries_decompress(input:*const u8,inlen:u32,out:*mut u8,outlen:*mut u32,wmem:*mut c_void)->i32 {let _=(out,wmem);let mut il=inlen;if check_constraints(input as usize,&mut il,true)!=0{return -22};if check_constraints(out as usize,outlen,false)!=0{return -22};let d=devdata;if d.is_null()||(*d).dev.is_null(){return -19};nx842_inc_decomp_complete(d);0}

unsafe fn nx842_OF_set_defaults(d:*mut nx842_devdata)->i32{if d.is_null(){-2}else{(*d).max_sync_size=0;(*d).max_sync_sg=0;(*d).max_sg_len=0;0}}
unsafe fn nx842_OF_upd_status(_d:*mut nx842_devdata,_p:*mut property)->i32{0}
unsafe fn nx842_OF_upd_maxsglen(_d:*mut nx842_devdata,_p:*mut property)->i32{0}
unsafe fn nx842_OF_upd_maxsyncop(_d:*mut nx842_devdata,_p:*mut property)->i32{0}
unsafe fn nx842_OF_upd(_p:*mut property)->i32{0}
unsafe extern "C" fn nx842_OF_notifier(_np:*mut notifier_block,_action:u64,_data:*mut c_void)->i32{0}
unsafe fn nx842_get_desired_dma(_v:*mut vio_dev)->usize{0}
unsafe extern "C" fn nx842_probe(_v:*mut vio_dev,_id:*const vio_device_id)->i32{0}
unsafe extern "C" fn nx842_remove(_v:*mut vio_dev){}
unsafe fn nxcop_get_capabilities(){}
unsafe extern "C" fn nx842_pseries_init()->i32{0}
unsafe extern "C" fn nx842_pseries_exit(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
