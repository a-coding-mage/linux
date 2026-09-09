// SPDX-License-Identifier: GPL-2.0
/* ldc.c: Logical Domain Channel link-layer protocol driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* Kernel and architecture definitions supplied by the surrounding tree. */
extern "C" {
    fn sun4v_ldc_tx_set_qtail(id: usize, tail: usize) -> usize;
    fn sun4v_ldc_rx_set_qhead(id: usize, head: usize) -> usize;
    fn sun4v_ldc_tx_get_state(id: usize, head: *mut usize, tail: *mut usize, state: *mut usize) -> usize;
    fn sun4v_ldc_rx_get_state(id: usize, head: *mut usize, tail: *mut usize, state: *mut usize) -> usize;
    fn sun4v_ldc_tx_qconf(id: usize, ra: usize, n: usize) -> usize;
    fn sun4v_ldc_rx_qconf(id: usize, ra: usize, n: usize) -> usize;
    fn sun4v_ldc_revoke(id: usize, cookie: u64, mte: u64);
    fn sun4v_ldc_set_map_table(id: usize, ra: usize, n: usize) -> usize;
    fn sun4v_ldc_copy(id: usize, dir: i32, remote: usize, local: usize, len: usize, actual: *mut usize) -> usize;
}

pub const LDC_PACKET_SIZE: usize = 64;
pub const PAGE_SIZE: usize = 8192;
pub const PAGE_SHIFT: usize = 13;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);
pub const LDC_CTRL: u8 = 1; pub const LDC_DATA: u8 = 2; pub const LDC_ERR: u8 = 0x10;
pub const LDC_INFO: u8 = 1; pub const LDC_ACK: u8 = 2; pub const LDC_NACK: u8 = 4;
pub const LDC_VERS: u8 = 1; pub const LDC_RTS: u8 = 2; pub const LDC_RTR: u8 = 3; pub const LDC_RDX: u8 = 4;
pub const LDC_CTRL_MSK: u8 = 0xf; pub const LDC_LEN: u8 = 0x3f; pub const LDC_START: u8 = 0x40; pub const LDC_STOP: u8 = 0x80;
pub const LDC_HS_CLOSED: u8 = 0; pub const LDC_HS_OPEN: u8 = 1; pub const LDC_HS_GOTVERS: u8 = 2;
pub const LDC_HS_SENTRTR: u8 = 3; pub const LDC_HS_GOTRTR: u8 = 4; pub const LDC_HS_COMPLETE: u8 = 0x10;
pub const LDC_FLAG_ALLOCED_QUEUES: u8 = 1; pub const LDC_FLAG_REGISTERED_QUEUES: u8 = 2;
pub const LDC_FLAG_REGISTERED_IRQS: u8 = 4; pub const LDC_FLAG_RESET: u8 = 0x10;
pub const LDC_DEFAULT_MTU: usize = 4 * LDC_PACKET_SIZE;
pub const LDC_DEFAULT_NUM_ENTRIES: usize = PAGE_SIZE / LDC_PACKET_SIZE;

#[repr(C)] pub union ldc_packet_data { pub u_data: [u8; LDC_PACKET_SIZE-8], pub r: ldc_packet_reliable }
#[repr(C)] pub struct ldc_packet_reliable { pub pad: u32, pub ackid: u32, pub r_data: [u8; LDC_PACKET_SIZE-16] }
#[repr(C)] pub struct ldc_packet { pub type_: u8, pub stype: u8, pub ctrl: u8, pub env: u8, pub seqid: u32, pub u: ldc_packet_data }
#[repr(C)] #[derive(Copy,Clone)] pub struct ldc_version { pub major: u16, pub minor: u16 }
#[repr(C)] pub struct ldc_mtable_entry { pub mte: u64, pub cookie: u64 }
#[repr(C)] pub struct ldc_trans_cookie { pub cookie_addr: u64, pub cookie_size: u64 }
#[repr(C)] pub struct ldc_iommu { pub lock: usize, pub page_table: *mut ldc_mtable_entry, pub iommu_map_table: [usize; 8] }
#[repr(C)] pub struct ldc_channel_config { pub mode: u8, pub mtu: u32, pub rx_irq: u32, pub tx_irq: u32, pub debug: u32, pub event: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)> }
#[repr(C)] pub struct ldc_channel { pub lock: usize, pub id: usize, pub mssbuf: *mut u8, pub mssbuf_len:u32, pub mssbuf_off:u32, pub tx_base:*mut ldc_packet, pub tx_head:usize,pub tx_tail:usize,pub tx_num_entries:usize,pub tx_ra:usize,pub tx_acked:usize, pub rx_base:*mut ldc_packet,pub rx_head:usize,pub rx_tail:usize,pub rx_num_entries:usize,pub rx_ra:usize,pub rcv_nxt:u32,pub snd_nxt:u32,pub chan_state:usize,pub cfg:ldc_channel_config,pub event_arg:*mut core::ffi::c_void,pub mops:*const ldc_mode_ops,pub iommu:ldc_iommu,pub ver:ldc_version,pub hs_state:u8,pub flags:u8,pub mss:u8,pub state:u8 }
#[repr(C)] pub struct ldc_mode_ops { pub write: Option<unsafe extern "C" fn(*mut ldc_channel,*const u8,u32)->i32>, pub read: Option<unsafe extern "C" fn(*mut ldc_channel,*mut u8,u32)->i32> }

static mut VER_ARR: [ldc_version;1] = [ldc_version{major:1,minor:0}];
pub static mut ldom_domaining_enabled: i32 = 0;

unsafe fn advance(off: usize, n: usize) -> usize { let x=off+LDC_PACKET_SIZE; if x==n*LDC_PACKET_SIZE {0} else {x} }
unsafe fn tx_packet(lp:*mut ldc_channel, off:usize)->*mut ldc_packet { (*lp).tx_base.add(off/LDC_PACKET_SIZE) }
unsafe fn rx_packet(lp:*mut ldc_channel, off:usize)->*mut ldc_packet { (*lp).rx_base.add(off/LDC_PACKET_SIZE) }
unsafe fn tx_space(lp:*mut ldc_channel)->bool { advance((*lp).tx_tail,(*lp).tx_num_entries)!=(*lp).tx_head }
unsafe fn set_tx_tail(lp:*mut ldc_channel, tail:usize)->i32 { (*lp).tx_tail=tail; for _ in 0..1000 { let e=sun4v_ldc_tx_set_qtail((*lp).id,tail); if e==0{return 0}; if e!=0x17 {return -22;} } -16 }
unsafe fn set_rx_head(lp:*mut ldc_channel, head:usize)->i32 { for _ in 0..1000 { let e=sun4v_ldc_rx_set_qhead((*lp).id,head); if e==0 {(*lp).rx_head=head;return 0}; if e!=0x17{return -22} } -16 }

#[no_mangle] pub unsafe extern "C" fn ldc_state(lp:*mut ldc_channel)->i32 { (*lp).state as i32 }
#[no_mangle] pub unsafe extern "C" fn ldc_mode(lp:*mut ldc_channel)->i32 { (*lp).cfg.mode as i32 }
#[no_mangle] pub unsafe extern "C" fn ldc_set_state(lp:*mut ldc_channel,state:u8){(*lp).state=state;}
#[no_mangle] pub unsafe extern "C" fn ldc_rx_reset(lp:*mut ldc_channel)->i32 { set_rx_head(lp,(*lp).rx_tail) }
#[no_mangle] pub unsafe extern "C" fn ldc_write(lp:*mut ldc_channel,buf:*const u8,size:u32)->i32 { if buf.is_null(){return -22}; if size==0{return 0}; if (*lp).hs_state!=LDC_HS_COMPLETE{return -107}; if !tx_space(lp){return -11}; let p=tx_packet(lp,(*lp).tx_tail); ptr::copy_nonoverlapping(buf,(*p).u.u_data.as_mut_ptr(),size.min((LDC_PACKET_SIZE-8) as u32) as usize); (*p).type_=LDC_DATA; let n=advance((*lp).tx_tail,(*lp).tx_num_entries); let e=set_tx_tail(lp,n); if e==0{size as i32}else{e} }
#[no_mangle] pub unsafe extern "C" fn ldc_read(lp:*mut ldc_channel,buf:*mut u8,size:u32)->i32 { if buf.is_null(){return -22}; if size==0{return 0}; if (*lp).hs_state!=LDC_HS_COMPLETE{return -107}; if (*lp).rx_head==(*lp).rx_tail{return 0}; let p=rx_packet(lp,(*lp).rx_head); ptr::copy_nonoverlapping((*p).u.u_data.as_ptr(),buf,size.min((LDC_PACKET_SIZE-8) as u32) as usize); let n=advance((*lp).rx_head,(*lp).rx_num_entries); set_rx_head(lp,n); size.min((LDC_PACKET_SIZE-8) as u32) as i32 }

#[no_mangle] pub unsafe extern "C" fn ldc_copy(lp:*mut ldc_channel,dir:i32,buf:*mut u8,len:u32,offset:usize,cookies:*mut ldc_trans_cookie,n:i32)->i32 { if dir!=0 && dir!=1{return -22}; if (*lp).hs_state!=LDC_HS_COMPLETE{return -104}; let mut done=0usize; for i in 0..n as usize { let mut actual=0; let l=((*cookies.add(i)).cookie_size as usize).saturating_sub(offset).min(len as usize-done); let e=sun4v_ldc_copy((*lp).id,dir,((*cookies.add(i)).cookie_addr as usize)+offset,buf.add(done),l,&mut actual); if e!=0{return -14}; done+=actual; if done==len as usize{break} } done as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
