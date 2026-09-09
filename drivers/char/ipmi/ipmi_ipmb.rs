// SPDX-License-Identifier: GPL-2.0
// Driver to talk to a remote management controller on IPMB.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

pub const DEVICE_NAME: &[u8] = b"ipmi-ipmb\0";
pub const IPMB_MAX_MSG_LEN: usize = IPMI_MAX_MSG_LENGTH + 5;

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type c_int = core::ffi::c_int;
pub type c_long = core::ffi::c_long;
pub type ulong = core::ffi::c_ulong;

pub const IPMI_MAX_MSG_LENGTH: usize = 256;
pub const IPMI_SMI_MSG_TYPE_IPMB_DIRECT: u8 = 1;
pub const IPMI_NETFN_APP_REQUEST: u8 = 0x06;
pub const IPMI_SEND_MSG_CMD: u8 = 0x34;
pub const IPMI_REQ_LEN_EXCEEDED_ERR: u8 = 0xc7;
pub const IPMI_BUS_ERR: u8 = 0x82;
pub const IPMI_TIMEOUT_ERR: u8 = 0xc3;
pub const IPMI_CC_NO_ERROR: c_int = 0;
pub const I2C_CLIENT_SLAVE: u32 = 1;

#[repr(C)] pub struct ipmi_smi { _private: [u8; 0] }
#[repr(C)] pub struct i2c_adapter { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct semaphore { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub addr: u16, pub flags: u32, pub adapter: *mut i2c_adapter, pub dev: device }
#[repr(C)] pub struct i2c_msg { pub addr: u16, pub flags: u16, pub len: u16, pub buf: *mut u8 }
#[repr(C)] pub struct ipmi_smi_msg { pub type_: u8, pub data_size: usize, pub data: [u8; IPMI_MAX_MSG_LENGTH], pub rsp: [u8; IPMI_MAX_MSG_LENGTH], pub rsp_size: usize }
#[repr(C)] pub struct ipmi_smi_handlers { pub flags: u32, pub start_processing: Option<unsafe extern "C" fn(*mut c_void, *mut ipmi_smi) -> c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut c_void)>, pub sender: Option<unsafe extern "C" fn(*mut c_void, *mut ipmi_smi_msg) -> c_int>, pub request_events: Option<unsafe extern "C" fn(*mut c_void)> }

extern "C" {
    fn ipmb_checksum(msg: *const u8, len: usize) -> u8;
    fn ipmi_alloc_smi_msg() -> *mut ipmi_smi_msg;
    fn ipmi_smi_msg_received(intf: *mut ipmi_smi, msg: *mut ipmi_smi_msg);
    fn i2c_get_clientdata(c: *mut i2c_client) -> *mut ipmi_ipmb_dev;
    fn i2c_transfer(a: *mut i2c_adapter, m: *mut i2c_msg, n: c_int) -> c_int;
    fn down_interruptible(s: *mut semaphore) -> c_long; fn down_timeout(s: *mut semaphore, t: c_long) -> c_long;
    fn down(s: *mut semaphore); fn up(s: *mut semaphore); fn kthread_should_stop() -> bool; fn kthread_stop(t: *mut task_struct);
    fn msecs_to_jiffies(ms: u32) -> c_long; fn ipmi_register_smi(h: *mut ipmi_smi_handlers, info: *mut c_void, d: *mut device, addr: u8) -> c_int; fn ipmi_unregister_smi(i: *mut ipmi_smi);
    fn i2c_slave_register(c: *mut i2c_client, cb: unsafe extern "C" fn(*mut i2c_client, c_int, *mut u8) -> c_int) -> c_int; fn i2c_slave_unregister(c: *mut i2c_client);
    fn i2c_unregister_device(c: *mut i2c_client); fn i2c_set_clientdata(c: *mut i2c_client, p: *mut ipmi_ipmb_dev);
    fn kthread_run(f: unsafe extern "C" fn(*mut c_void) -> c_int, d: *mut ipmi_ipmb_dev, name: *const u8, ...) -> *mut task_struct;
}

#[repr(C)] pub struct ipmi_ipmb_dev {
    pub intf: *mut ipmi_smi, pub client: *mut i2c_client, pub slave: *mut i2c_client, pub handlers: ipmi_smi_handlers,
    pub ready: bool, pub curr_seq: u8, pub bmcaddr: u8, pub retry_time_ms: u32, pub max_retries: u32,
    pub next_msg: *mut ipmi_smi_msg, pub working_msg: *mut ipmi_smi_msg, pub thread: *mut task_struct,
    pub wake_thread: semaphore, pub got_rsp: semaphore, pub lock: spinlock_t, pub stopping: bool,
    pub xmitmsg: [u8; IPMB_MAX_MSG_LEN], pub xmitlen: usize, pub rcvmsg: [u8; IPMB_MAX_MSG_LEN], pub rcvlen: usize, pub overrun: bool,
}

#[inline] unsafe fn valid_ipmb(d: *mut ipmi_ipmb_dev) -> bool { if (*d).overrun || (*d).rcvlen < 7 { return false; } let m=(*d).rcvmsg.as_ptr(); if ( *m.add(1)>>2)&1 != 0 && (*d).rcvlen < 8 { return false; } ipmb_checksum(m,3)==0 && ipmb_checksum(m.add(3),(*d).rcvlen-3)==0 }

#[no_mangle] pub unsafe extern "C" fn ipmi_ipmb_check_msg_done(d: *mut ipmi_ipmb_dev) {
    if (*d).rcvlen == 0 { return; } if !valid_ipmb(d) { (*d).overrun=false; (*d).rcvlen=0; return; }
    let m=(*d).rcvmsg.as_ptr(); let is_cmd=(((*m.add(1)>>2)&1)==0); let mut imsg: *mut ipmi_smi_msg=core::ptr::null_mut();
    if is_cmd { if !(*d).ready { (*d).overrun=false; (*d).rcvlen=0; return; } imsg=ipmi_alloc_smi_msg(); if imsg.is_null(){return;} (*imsg).type_=IPMI_SMI_MSG_TYPE_IPMB_DIRECT; (*imsg).data_size=0; }
    else if !(*d).working_msg.is_null() { let seq=*m.add(4)>>2; let x=((*(*d).working_msg).data[0]>>2)&1; if x==0 && seq==(*d).curr_seq { (*d).curr_seq=((*d).curr_seq+1)&0x3f; imsg=(*d).working_msg; (*d).working_msg=core::ptr::null_mut(); } }
    if !imsg.is_null() { (*imsg).rsp[0]=*m.add(1); if (*imsg).type_==IPMI_SMI_MSG_TYPE_IPMB_DIRECT { core::ptr::copy_nonoverlapping(m.add(3),(*imsg).rsp.as_mut_ptr().add(1),(*d).rcvlen-4); (*imsg).rsp_size=(*d).rcvlen-3; } else { core::ptr::copy_nonoverlapping(m.add(5),(*imsg).rsp.as_mut_ptr().add(1),(*d).rcvlen-6); (*imsg).rsp_size=(*d).rcvlen-5; } ipmi_smi_msg_received((*d).intf,imsg); if !is_cmd {up(&mut (*d).got_rsp);} }
    (*d).overrun=false; (*d).rcvlen=0;
}

// The remaining driver entry points retain the C ABI and are declarations of the translated kernel callbacks.
pub unsafe extern "C" fn ipmi_ipmb_slave_cb(c:*mut i2c_client,event:c_int,val:*mut u8)->c_int { let d=i2c_get_clientdata(c); match event { 0=>{ipmi_ipmb_check_msg_done(d);(*d).rcvmsg[0]=((*c).addr<<1) as u8;(*d).rcvlen=1},1=>if (*d).rcvlen>=IPMB_MAX_MSG_LEN{(*d).overrun=true}else{(*d).rcvmsg[(*d).rcvlen]=*val;(*d).rcvlen+=1},2=>{*val=0xff;ipmi_ipmb_check_msg_done(d)},3=>ipmi_ipmb_check_msg_done(d),4=>*val=0xff,_=>{}} 0 }

pub unsafe extern "C" fn ipmi_ipmb_send_response(d:*mut ipmi_ipmb_dev,m:*mut ipmi_smi_msg,cc:u8){if ((*m).data[0]>>2)&1!=0{(*m).data[0]=(IPMI_NETFN_APP_REQUEST|1)<<2;(*m).data[3]=IPMI_SEND_MSG_CMD;(*m).data[4]=cc;(*m).data_size=5;}(*m).rsp[0]=(*m).data[0]|(1<<2);if (*m).type_==IPMI_SMI_MSG_TYPE_IPMB_DIRECT{(*m).rsp[1]=(*m).data[1];(*m).rsp[2]=(*m).data[2];(*m).rsp[3]=(*m).data[3];(*m).rsp[4]=cc;(*m).rsp_size=5}else{(*m).rsp[1]=(*m).data[1];(*m).rsp[2]=cc;(*m).rsp_size=3}ipmi_smi_msg_received((*d).intf,m)}
pub unsafe extern "C" fn ipmi_ipmb_format_for_xmit(d:*mut ipmi_ipmb_dev,m:*mut ipmi_smi_msg){if (*m).type_==IPMI_SMI_MSG_TYPE_IPMB_DIRECT{(*d).xmitmsg[0]=(*m).data[1];(*d).xmitmsg[1]=(*m).data[0];core::ptr::copy_nonoverlapping((*m).data.as_ptr().add(2),(*d).xmitmsg.as_mut_ptr().add(4),(*m).data_size-2);(*d).xmitlen=(*m).data_size+2}else{(*d).xmitmsg[0]=(*d).bmcaddr;(*d).xmitmsg[1]=(*m).data[0];(*d).xmitmsg[4]=0;core::ptr::copy_nonoverlapping((*m).data.as_ptr().add(1),(*d).xmitmsg.as_mut_ptr().add(5),(*m).data_size-1);(*d).xmitlen=(*m).data_size+4}(*d).xmitmsg[3]=((*d).slave).as_ref().unwrap().addr as u8<<1;if ((*m).data[0]>>2)&1==0{(*d).xmitmsg[4]=((*d).xmitmsg[4]&3)|((*d).curr_seq<<2)}(*d).xmitmsg[2]=ipmb_checksum((*d).xmitmsg.as_ptr(),2);let n=(*d).xmitlen;(*d).xmitmsg[n]=ipmb_checksum((*d).xmitmsg.as_ptr().add(3),n-3);(*d).xmitlen+=1}
pub unsafe extern "C" fn ipmi_ipmb_start_processing(info:*mut c_void,intf:*mut ipmi_smi)->c_int{let d=info as *mut ipmi_ipmb_dev;(*d).intf=intf;(*d).ready=true;0}
pub unsafe extern "C" fn ipmi_ipmb_shutdown(info:*mut c_void){let d=info as *mut ipmi_ipmb_dev;(*d).stopping=true}
pub unsafe extern "C" fn ipmi_ipmb_sender(info:*mut c_void,m:*mut ipmi_smi_msg)->c_int{let d=info as *mut ipmi_ipmb_dev;(*d).next_msg=m;up(&mut (*d).wake_thread);IPMI_CC_NO_ERROR}
pub unsafe extern "C" fn ipmi_ipmb_request_events(_info:*mut c_void){}
pub unsafe extern "C" fn ipmi_ipmb_remove(_client:*mut i2c_client){}
pub unsafe extern "C" fn ipmi_ipmb_probe(_client:*mut i2c_client)->c_int{-12}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
