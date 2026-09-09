// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of iucv.c. Linux/kernel-provided types and operations are
 * intentionally referenced as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const IUCV_IPSRCCLS: u8 = 0x01;
const IUCV_IPTRGCLS: u8 = 0x01;
const IUCV_IPFGPID: u8 = 0x02;
const IUCV_IPFGMID: u8 = 0x04;
const IUCV_IPNORPY: u8 = 0x10;
const IUCV_IPALL: u8 = 0x80;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { pub name: *const u8, pub match_: Option<unsafe extern "C" fn(*mut device, *const device_driver)->i32> }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cpumask_t { pub bits: [usize; 1] }
pub type dma32_t = u32;

#[repr(C)] pub struct iucv_irq_data { pub ippathid:u16, pub ipflags1:u8, pub iptype:u8, pub res2:[u32;9] }
#[repr(C)] pub struct iucv_irq_list { pub list:list_head, pub data:iucv_irq_data }

#[repr(C, packed(8))] pub struct iucv_cmd_control { pub ippathid:u16,pub ipflags1:u8,pub iprcode:u8,pub ipmsglim:u16,pub res1:u16,pub ipvmid:[u8;8],pub ipuser:[u8;16],pub iptarget:[u8;8] }
#[repr(C, packed(8))] pub struct iucv_cmd_dpl { pub ippathid:u16,pub ipflags1:u8,pub iprcode:u8,pub ipmsgid:u32,pub iptrgcls:u32,pub iprmmsg:[u8;8],pub ipsrccls:u32,pub ipmsgtag:u32,pub ipbfadr2:dma32_t,pub ipbfln2f:u32,pub res:u32 }
#[repr(C, packed(8))] pub struct iucv_cmd_db { pub ippathid:u16,pub ipflags1:u8,pub iprcode:u8,pub ipmsgid:u32,pub iptrgcls:u32,pub ipbfadr1:dma32_t,pub ipbfln1f:u32,pub ipsrccls:u32,pub ipmsgtag:u32,pub ipbfadr2:dma32_t,pub ipbfln2f:u32,pub res:u32 }
#[repr(C, packed(8))] pub struct iucv_cmd_purge { pub ippathid:u16,pub ipflags1:u8,pub iprcode:u8,pub ipmsgid:u32,pub ipaudit:[u8;3],pub res1:[u8;5],pub res2:u32,pub ipsrccls:u32,pub ipmsgtag:u32,pub res3:[u32;3] }
#[repr(C, packed(8))] pub struct iucv_cmd_set_mask { pub ipmask:u8,pub res1:[u8;2],pub iprcode:u8,pub res2:[u32;9] }
#[repr(C)] pub union iucv_param { pub ctrl:iucv_cmd_control, pub dpl:iucv_cmd_dpl, pub db:iucv_cmd_db, pub purge:iucv_cmd_purge, pub set_mask:iucv_cmd_set_mask }

#[repr(C)] pub struct iucv_path { pub list:list_head,pub pathid:u16,pub msglim:u16,pub flags:u8,pub handler:*mut iucv_handler,pub private:*mut c_void }
#[repr(C)] pub struct iucv_handler { pub list:list_head,pub paths:list_head,pub path_pending:Option<unsafe extern "C" fn(*mut iucv_path,*mut u8,*mut u8)->i32>,pub path_complete:Option<unsafe extern "C" fn(*mut iucv_path,*mut u8)>,pub path_severed:Option<unsafe extern "C" fn(*mut iucv_path,*mut u8)>,pub path_quiesced:Option<unsafe extern "C" fn(*mut iucv_path,*mut u8)>,pub path_resumed:Option<unsafe extern "C" fn(*mut iucv_path,*mut u8)>,pub message_complete:Option<unsafe extern "C" fn(*mut iucv_path,*mut iucv_message)>,pub message_pending:Option<unsafe extern "C" fn(*mut iucv_path,*mut iucv_message)> }
#[repr(C)] pub struct iucv_message { pub flags:u8,pub id:u32,pub audit:u32,pub rmmsg:[u8;8],pub class:u32,pub tag:u32,pub length:u32,pub reply_size:u32 }

static mut IUCV_AVAILABLE:i32=0;
static mut IUCV_MAX_PATHID:usize=0;
static mut IUCV_PATH_TABLE:*mut *mut iucv_path=core::ptr::null_mut();
static mut IUCV_PARAM:[*mut iucv_param;256]=[core::ptr::null_mut();256];
static mut IUCV_PARAM_IRQ:[*mut iucv_param;256]=[core::ptr::null_mut();256];
static mut IUCV_IRQ_DATA:[*mut iucv_irq_data;256]=[core::ptr::null_mut();256];
static mut IUCV_ACTIVE_CPU:i32=-1;
static mut IUCV_NONSMP_HANDLER:i32=0;
static mut IUCV_ROOT:*mut device=core::ptr::null_mut();

extern "C" {
    fn virt_to_phys(p:*mut c_void)->usize;
    fn memset(p:*mut c_void,v:i32,n:usize)->*mut c_void;
    fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn smp_processor_id()->usize;
    fn iucv_path_alloc(msglim:u16,flags:u8,gfp:u32)->*mut iucv_path;
    fn iucv_path_free(p:*mut iucv_path);
}

#[inline] unsafe fn __iucv_call_b2f0(_command:i32,_parm:*mut iucv_param)->i32 { 0 }
#[inline] unsafe fn iucv_call_b2f0(command:i32,parm:*mut iucv_param)->i32 { let cc=__iucv_call_b2f0(command,parm); if cc==1 { (*parm).ctrl.iprcode as i32 } else { cc } }

unsafe fn iucv_sever_pathid(pathid:u16,userdata:*mut u8)->i32 { let parm=IUCV_PARAM_IRQ[smp_processor_id()]; memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>()); if !userdata.is_null(){memcpy((*parm).ctrl.ipuser.as_mut_ptr() as *mut c_void,userdata as *const c_void,16);} (*parm).ctrl.ippathid=pathid; iucv_call_b2f0(15,parm) }

pub unsafe extern "C" fn iucv_path_accept(path:*mut iucv_path,_handler:*mut iucv_handler,userdata:*mut u8,private:*mut c_void)->i32 { let parm=IUCV_PARAM[smp_processor_id()]; memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>()); (*parm).ctrl.ippathid=(*path).pathid; (*parm).ctrl.ipmsglim=(*path).msglim; (*parm).ctrl.ipflags1=(*path).flags; if !userdata.is_null(){memcpy((*parm).ctrl.ipuser.as_mut_ptr() as *mut c_void,userdata as *const c_void,16);} let rc=iucv_call_b2f0(10,parm); if rc==0 {(*path).private=private;(*path).msglim=(*parm).ctrl.ipmsglim;(*path).flags=(*parm).ctrl.ipflags1;} rc }
pub unsafe extern "C" fn iucv_path_quiesce(path:*mut iucv_path,userdata:*mut u8)->i32 { let parm=IUCV_PARAM[smp_processor_id()]; memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>()); (*parm).ctrl.ippathid=(*path).pathid; if !userdata.is_null(){memcpy((*parm).ctrl.ipuser.as_mut_ptr() as *mut c_void,userdata as *const c_void,16);} iucv_call_b2f0(13,parm) }
pub unsafe extern "C" fn iucv_path_resume(path:*mut iucv_path,userdata:*mut u8)->i32 { let parm=IUCV_PARAM[smp_processor_id()]; memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>()); (*parm).ctrl.ippathid=(*path).pathid; if !userdata.is_null(){memcpy((*parm).ctrl.ipuser.as_mut_ptr() as *mut c_void,userdata as *const c_void,16);} iucv_call_b2f0(14,parm) }
pub unsafe extern "C" fn iucv_path_sever(path:*mut iucv_path,userdata:*mut u8)->i32 { let rc=iucv_sever_pathid((*path).pathid,userdata); if !IUCV_PATH_TABLE.is_null(){*IUCV_PATH_TABLE.add((*path).pathid as usize)=core::ptr::null_mut();} rc }

pub unsafe extern "C" fn __iucv_message_receive(path:*mut iucv_path,msg:*mut iucv_message,_flags:u8,buffer:*mut c_void,size:usize,residual:*mut usize)->i32 { let parm=IUCV_PARAM[smp_processor_id()]; memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>()); (*parm).db.ippathid=(*path).pathid; (*parm).db.ipmsgid=(*msg).id; (*parm).db.ipbfadr1=buffer as usize as u32; (*parm).db.ipbfln1f=size as u32; let rc=iucv_call_b2f0(5,parm); if rc==0||rc==5 {(*msg).flags=(*parm).db.ipflags1;if !residual.is_null(){*residual=(*parm).db.ipbfln1f as usize;}} rc }
pub unsafe extern "C" fn iucv_message_receive(path:*mut iucv_path,msg:*mut iucv_message,flags:u8,buffer:*mut c_void,size:usize,residual:*mut usize)->i32 { __iucv_message_receive(path,msg,flags,buffer,size,residual) }
pub unsafe extern "C" fn iucv_message_reject(path:*mut iucv_path,msg:*mut iucv_message)->i32 { let parm=IUCV_PARAM[smp_processor_id()];memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>());(*parm).db.ippathid=(*path).pathid;(*parm).db.ipmsgid=(*msg).id;iucv_call_b2f0(8,parm) }
pub unsafe extern "C" fn iucv_message_purge(path:*mut iucv_path,msg:*mut iucv_message,srccls:u32)->i32 { let parm=IUCV_PARAM[smp_processor_id()];memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>());(*parm).purge.ippathid=(*path).pathid;(*parm).purge.ipmsgid=(*msg).id;(*parm).purge.ipsrccls=srccls;iucv_call_b2f0(9,parm) }
pub unsafe extern "C" fn iucv_message_reply(path:*mut iucv_path,msg:*mut iucv_message,flags:u8,reply:*mut c_void,size:usize)->i32 { let parm=IUCV_PARAM[smp_processor_id()];memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>());(*parm).db.ippathid=(*path).pathid;(*parm).db.ipmsgid=(*msg).id;(*parm).db.ipflags1=flags;(*parm).db.ipbfadr1=reply as usize as u32;(*parm).db.ipbfln1f=size as u32;iucv_call_b2f0(6,parm) }
pub unsafe extern "C" fn __iucv_message_send(path:*mut iucv_path,msg:*mut iucv_message,flags:u8,srccls:u32,buffer:*mut c_void,size:usize)->i32 { let parm=IUCV_PARAM[smp_processor_id()];memset(parm as *mut c_void,0,core::mem::size_of::<iucv_param>());(*parm).db.ippathid=(*path).pathid;(*parm).db.ipflags1=flags|IUCV_IPNORPY;(*parm).db.iptrgcls=(*msg).class;(*parm).db.ipsrccls=srccls;(*parm).db.ipbfadr1=buffer as usize as u32;(*parm).db.ipbfln1f=size as u32;let rc=iucv_call_b2f0(4,parm);if rc==0{(*msg).id=(*parm).db.ipmsgid;}rc }
pub unsafe extern "C" fn iucv_message_send(path:*mut iucv_path,msg:*mut iucv_message,flags:u8,srccls:u32,buffer:*mut c_void,size:usize)->i32 {__iucv_message_send(path,msg,flags,srccls,buffer,size)}
pub unsafe extern "C" fn iucv_message_send2way(path:*mut iucv_path,msg:*mut iucv_message,flags:u8,srccls:u32,buffer:*mut c_void,size:usize,answer:*mut c_void,asize:usize,_residual:*mut usize)->i32 {let rc=__iucv_message_send(path,msg,flags,srccls,buffer,size);let _=(answer,asize);rc}

pub unsafe extern "C" fn iucv_register(_handler:*mut iucv_handler,_smp:i32)->i32 { if IUCV_AVAILABLE==0 {-38} else {0} }
pub unsafe extern "C" fn iucv_unregister(_handler:*mut iucv_handler,_smp:i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
