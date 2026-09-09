// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of the BMC side of the SSIF interface.  Kernel-provided
 * types and functions are intentionally left as external dependencies. */

use core::{ffi::c_void, mem::size_of, ptr};

pub const DEVICE_NAME: &str = "ipmi-ssif-host";
pub const MAX_PAYLOAD_PER_TRANSACTION: usize = 32;
pub const MAX_TRANSACTION: usize = MAX_PAYLOAD_PER_TRANSACTION + 4;
pub const MAX_IPMI_DATA_PER_START_TRANSACTION: u8 = 30;
pub const MAX_IPMI_DATA_PER_MIDDLE_TRANSACTION: u8 = 31;
pub const SSIF_IPMI_SINGLEPART_WRITE: u8 = 0x2;
pub const SSIF_IPMI_SINGLEPART_READ: u8 = 0x3;
pub const SSIF_IPMI_MULTIPART_WRITE_START: u8 = 0x6;
pub const SSIF_IPMI_MULTIPART_WRITE_MIDDLE: u8 = 0x7;
pub const SSIF_IPMI_MULTIPART_WRITE_END: u8 = 0x8;
pub const SSIF_IPMI_MULTIPART_READ_START: u8 = 0x3;
pub const SSIF_IPMI_MULTIPART_READ_MIDDLE: u8 = 0x9;
pub const RESPONSE_TIMEOUT: u64 = 500;

pub const fn get_8bit_addr(addr_7bit: u8) -> u8 { addr_7bit.wrapping_shl(1) }

#[repr(C)] pub struct I2cClient { pub addr: u16, pub flags: u16, pub dev: *mut c_void }
#[repr(C)] pub struct File { pub private_data: *mut c_void, pub f_flags: u32 }
#[repr(C)] pub struct Inode;
#[repr(C)] pub struct MiscDevice;
#[repr(C)] pub struct Spinlock;
#[repr(C)] pub struct WaitQueue;
#[repr(C)] pub struct TimerList;
#[repr(C)] pub struct IpmiSsifMsg { pub len: u8, pub payload: [u8; 254] }

extern "C" {
    fn i2c_smbus_pec(crc: u8, p: *const u8, count: usize) -> u8;
    fn wake_up_all(q: *mut WaitQueue);
    fn timer_delete(t: *mut TimerList);
    fn memset(dst: *mut c_void, value: i32, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, n: usize) -> usize;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: usize) -> usize;
    fn i2c_get_clientdata(c: *mut I2cClient) -> *mut SsifBmcCtx;
}

#[repr(C)] pub struct SsifPartBuffer {
    pub address: u8, pub smbus_cmd: u8, pub length: u8,
    pub payload: [u8; MAX_PAYLOAD_PER_TRANSACTION], pub pec: u8, pub index: u8,
}
#[repr(u32)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum SsifState {
    Ready, Start, SmbusCmd, ReqRecving, ResSending, Aborting, StateMax,
}
#[repr(C)] pub struct SsifBmcCtx {
    pub client: *mut I2cClient, pub miscdev: MiscDevice, pub msg_idx: i32,
    pub pec_support: bool, pub lock: Spinlock, pub wait_queue: WaitQueue,
    pub running: u8, pub state: SsifState, pub response_timer: TimerList,
    pub response_timer_inited: bool, pub is_singlepart_read: bool,
    pub nbytes_processed: u8, pub remain_len: u8, pub recv_len: u8,
    pub block_num: u8, pub request_available: bool,
    pub response_in_progress: bool, pub busy: bool, pub aborting: bool,
    pub part_buf: SsifPartBuffer, pub response: IpmiSsifMsg, pub request: IpmiSsifMsg,
}

#[inline] unsafe fn to_ssif_bmc(file: *mut File) -> *mut SsifBmcCtx {
    (*file).private_data as *mut SsifBmcCtx
}
pub fn state_to_string(s: SsifState) -> &'static str { match s {
    SsifState::Ready => "SSIF_READY", SsifState::Start => "SSIF_START",
    SsifState::SmbusCmd => "SSIF_SMBUS_CMD", SsifState::ReqRecving => "SSIF_REQ_RECVING",
    SsifState::ResSending => "SSIF_RES_SENDING", SsifState::Aborting => "SSIF_ABORTING",
    SsifState::StateMax => "SSIF_STATE_UNKNOWN",
} }

unsafe fn complete_response(s: *mut SsifBmcCtx) {
    (*s).response.len = 0; (*s).response_in_progress = false;
    (*s).nbytes_processed = 0; (*s).remain_len = 0; (*s).busy = false;
    wake_up_all(&mut (*s).wait_queue);
}
unsafe fn calculate_response_part_pec(p: *mut SsifPartBuffer) {
    let mut a = (*p).address;
    (*p).pec = i2c_smbus_pec(0, &a, 1);
    (*p).pec = i2c_smbus_pec((*p).pec, &(*p).smbus_cmd, 1);
    a |= 1; (*p).pec = i2c_smbus_pec((*p).pec, &a, 1);
    (*p).pec = i2c_smbus_pec((*p).pec, &(*p).length, 1);
    if (*p).length != 0 { (*p).pec = i2c_smbus_pec((*p).pec, (*p).payload.as_ptr(), (*p).length as usize); }
}
unsafe fn set_singlepart_response_buffer(s: *mut SsifBmcCtx) {
    let p = &mut (*s).part_buf; p.address = get_8bit_addr((*(*s).client).addr as u8);
    p.length = (*s).response.len; let n = p.length as usize;
    ptr::write_bytes(p.payload.as_mut_ptr().add(n), 0, MAX_PAYLOAD_PER_TRANSACTION - n);
    ptr::copy_nonoverlapping((*s).response.payload.as_ptr(), p.payload.as_mut_ptr(), n);
}
unsafe fn set_multipart_response_buffer(s: *mut SsifBmcCtx) {
    let p = &mut (*s).part_buf; p.address = get_8bit_addr((*(*s).client).addr as u8);
    let mut n = 0u8;
    match p.smbus_cmd {
        SSIF_IPMI_MULTIPART_READ_START => { (*s).nbytes_processed=0; (*s).block_num=0; p.length=32; n=30; (*s).remain_len=(*s).response.len-n; p.payload[0]=0; p.payload[1]=1; ptr::copy_nonoverlapping((*s).response.payload.as_ptr(), p.payload.as_mut_ptr().add(2), n as usize); }
        SSIF_IPMI_MULTIPART_READ_MIDDLE => { if (*s).remain_len <= 31 { ptr::write_bytes(p.payload.as_mut_ptr(),0,32); p.length=(*s).remain_len+1; n=(*s).remain_len; (*s).block_num=0xff; p.payload[0]=0xff; } else { p.length=32; n=31; p.payload[0]=(*s).block_num; (*s).block_num=(*s).block_num.wrapping_add(1); } (*s).remain_len-=n; ptr::copy_nonoverlapping((*s).response.payload.as_ptr().add((*s).nbytes_processed as usize), p.payload.as_mut_ptr().add(1), n as usize); }
        _ => {}
    } (*s).nbytes_processed=(*s).nbytes_processed.wrapping_add(n);
}
pub fn supported_read_cmd(c:u8)->bool { c==3 || c==9 }
pub fn supported_write_cmd(c:u8)->bool { c==2 || c==6 || c==7 || c==8 }
pub fn supported_write_start_cmd(c:u8)->bool { c==2 || c==6 }

unsafe fn handle_read_processed(s:*mut SsifBmcCtx, v:*mut u8) { let p=&mut (*s).part_buf; *v=if p.index<p.length {p.payload[p.index as usize]} else if p.index==p.length && (*s).pec_support {p.pec} else {0}; p.index=p.index.wrapping_add(1); }
unsafe fn handle_write_received(s:*mut SsifBmcCtx,v:u8) { if (*s).msg_idx<1 || (*s).msg_idx>MAX_TRANSACTION as i32{return} if (*s).msg_idx==1 {(*s).part_buf.length=v;(*s).part_buf.index=0} else {let i=(*s).part_buf.index as usize; if i<32 {(*s).part_buf.payload[i]=v};(*s).part_buf.index=(*s).part_buf.index.wrapping_add(1)} (*s).msg_idx+=1; }
unsafe fn process_smbus_cmd(s:*mut SsifBmcCtx,v:u8) {(*s).part_buf.smbus_cmd=v;(*s).msg_idx=1;ptr::write_bytes((*s).part_buf.payload.as_mut_ptr(),0,32);if v==2||v==6 {if (*s).response_in_progress{complete_response(s)};(*s).aborting=false}}
unsafe fn on_read_requested_event(s:*mut SsifBmcCtx,v:*mut u8){if (*s).state==SsifState::Ready||(*s).state==SsifState::Start||(*s).state==SsifState::ReqRecving||(*s).state==SsifState::ResSending {(*s).state=SsifState::Aborting;*v=0;return} if (*s).state==SsifState::SmbusCmd {if !supported_read_cmd((*s).part_buf.smbus_cmd){(*s).aborting=true};(*s).state=if (*s).aborting{SsifState::Aborting}else{SsifState::ResSending}} (*s).msg_idx=0;if !(*s).response_in_progress||(*s).state==SsifState::Aborting{*v=0;return} if (*s).is_singlepart_read{set_singlepart_response_buffer(s)}else{set_multipart_response_buffer(s)} calculate_response_part_pec(&mut (*s).part_buf);(*s).part_buf.index=0;*v=(*s).part_buf.length;}

/* Remaining event dispatch, probe/remove, file operations, driver registration,
 * and the CONFIG_SSIF_IPMI_BMC_KUNIT_TEST cases retain the exact C interfaces
 * and are represented below as external kernel entry points. */
extern "C" { pub fn ssif_bmc_cb(client:*mut I2cClient,event:u32,val:*mut u8)->i32; pub fn ssif_bmc_probe(client:*mut I2cClient)->i32; pub fn ssif_bmc_remove(client:*mut I2cClient); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
