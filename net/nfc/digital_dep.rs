// SPDX-License-Identifier: GPL-2.0-only
/* NFC Digital Protocol stack; literal low-level translation of digital_dep.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

const DIGITAL_NFC_DEP_N_RETRY_NACK: u8 = 2;
const DIGITAL_NFC_DEP_N_RETRY_ATN: u8 = 2;
const DIGITAL_NFC_DEP_FRAME_DIR_OUT: u8 = 0xd4;
const DIGITAL_NFC_DEP_FRAME_DIR_IN: u8 = 0xd5;
const DIGITAL_NFC_DEP_NFCA_SOD_SB: u8 = 0xf0;
const DIGITAL_CMD_ATR_REQ: u8 = 0;
const DIGITAL_CMD_ATR_RES: u8 = 1;
const DIGITAL_CMD_PSL_REQ: u8 = 4;
const DIGITAL_CMD_PSL_RES: u8 = 5;
const DIGITAL_CMD_DEP_REQ: u8 = 6;
const DIGITAL_CMD_DEP_RES: u8 = 7;
const DIGITAL_ATR_REQ_MIN_SIZE: usize = 16;
const DIGITAL_ATR_REQ_MAX_SIZE: usize = 64;
const DIGITAL_DID_MAX: u8 = 14;
const DIGITAL_PAYLOAD_SIZE_MAX: u8 = 254;
const DIGITAL_GB_BIT: u8 = 2;
const DIGITAL_NFC_DEP_PFB_I_PDU: u8 = 0;
const DIGITAL_NFC_DEP_PFB_ACK_NACK_PDU: u8 = 0x40;
const DIGITAL_NFC_DEP_PFB_SUPERVISOR_PDU: u8 = 0x80;
const DIGITAL_NFC_DEP_IN_MAX_WT: usize = 14;
const DIGITAL_NFC_DEP_TG_MAX_WT: u8 = 14;
const DIGITAL_NFC_DEP_RTOX_MAX: u8 = 59;

#[repr(C, packed)] pub struct digital_atr_req { pub dir:u8,pub cmd:u8,pub nfcid3:[u8;10],pub did:u8,pub bs:u8,pub br:u8,pub pp:u8,pub gb:[u8;0] }
#[repr(C, packed)] pub struct digital_atr_res { pub dir:u8,pub cmd:u8,pub nfcid3:[u8;10],pub did:u8,pub bs:u8,pub br:u8,pub to:u8,pub pp:u8,pub gb:[u8;0] }
#[repr(C, packed)] pub struct digital_psl_req { pub dir:u8,pub cmd:u8,pub did:u8,pub brs:u8,pub fsl:u8 }
#[repr(C, packed)] pub struct digital_psl_res { pub dir:u8,pub cmd:u8,pub did:u8 }
#[repr(C, packed)] pub struct digital_dep_req_res { pub dir:u8,pub cmd:u8,pub pfb:u8 }

// Types and functions below are supplied by digital.h / the surrounding kernel port.
extern "C" {
    fn digital_payload_size_to_bits(s:u8)->u8;
    fn digital_payload_bits_to_size(s:u8)->u8;
}

static DIGITAL_PAYLOAD_BITS_MAP:[u8;4]=[64,128,192,254];
static DIGITAL_RWT_MAP:[u16;15]=[100,101,101,102,105,110,119,139,177,255,409,719,1337,2575,5049];

#[inline] fn payload_bits_to_size(s:u8)->u8 { if (s as usize)>=4 {0} else {DIGITAL_PAYLOAD_BITS_MAP[s as usize]} }
#[inline] fn payload_size_to_bits(s:u8)->u8 { for i in 0..4 { if DIGITAL_PAYLOAD_BITS_MAP[i]==s{return i as u8;} } 0xff }
#[inline] fn pfb_type(x:u8)->u8{x&0xe0}
#[inline] fn pni(x:u8)->u8{x&3}

/* The following declarations intentionally retain the C ABI and pointer semantics. */
extern "C" {
    fn digital_in_send_atr_req(ddev:*mut nfc_digital_dev,target:*mut nfc_target,comm_mode:u8,gb:*mut u8,gb_len:usize)->i32;
    fn digital_in_send_dep_req(ddev:*mut nfc_digital_dev,target:*mut nfc_target,skb:*mut sk_buff,data_exch:*mut digital_data_exch)->i32;
    fn digital_tg_send_dep_res(ddev:*mut nfc_digital_dev,skb:*mut sk_buff)->i32;
}
#[repr(C)] pub struct nfc_digital_dev { pub curr_rf_tech:u8,pub remote_payload_max:u8,pub local_payload_max:u8,pub curr_nfc_dep_pni:u8,pub did:u8,pub atn_count:u8,pub nack_count:u8,pub dep_rwt:u16,pub curr_protocol:u32,pub protocols:u32,pub poll_tech_count:u8,pub chaining_skb:*mut sk_buff,pub saved_skb:*mut sk_buff,pub data_exch:*mut digital_data_exch, pub nfc_dev:*mut nfc_dev, pub skb_add_crc:Option<unsafe extern "C" fn(*mut sk_buff)>,pub skb_check_crc:Option<unsafe extern "C" fn(*mut sk_buff)> }
#[repr(C)] pub struct sk_buff { pub next:*mut sk_buff,pub data:*mut u8,pub len:usize }
#[repr(C)] pub struct nfc_target { pub idx:u32,pub nfcid2_len:u8,pub nfcid2:*mut u8 }
#[repr(C)] pub struct nfc_dev;
#[repr(C)] pub struct digital_data_exch { pub cb:Option<unsafe extern "C" fn(*mut u8,*mut sk_buff,i32)>,pub cb_context:*mut u8 }

extern "C" {
 fn skb_push(*mut sk_buff,usize)->*mut u8; fn skb_pull(*mut sk_buff,usize)->*mut u8; fn skb_put(*mut sk_buff,usize)->*mut u8; fn skb_put_data(*mut sk_buff,*const u8,usize)->*mut u8;
 fn digital_skb_alloc(*mut nfc_digital_dev,usize)->*mut sk_buff; fn kfree_skb(*mut sk_buff); fn dev_kfree_skb(*mut sk_buff); fn pskb_copy(*mut sk_buff,u32)->*mut sk_buff; fn skb_get(*mut sk_buff);
 fn digital_in_send_cmd(*mut nfc_digital_dev,*mut sk_buff,u16,Option<unsafe extern "C" fn(*mut nfc_digital_dev,*mut u8,*mut sk_buff)>,*mut u8)->i32;
 fn digital_tg_send_cmd(*mut nfc_digital_dev,*mut sk_buff,u16,Option<unsafe extern "C" fn(*mut nfc_digital_dev,*mut u8,*mut sk_buff)>,*mut u8)->i32;
}

unsafe fn digital_skb_push_dep_sod(d:*mut nfc_digital_dev,s:*mut sk_buff){ skb_push(s,1); (*s).data.write((*s).len as u8); if (*d).curr_rf_tech==0 { *skb_push(s,1)=DIGITAL_NFC_DEP_NFCA_SOD_SB; } }
unsafe fn digital_skb_pull_dep_sod(d:*mut nfc_digital_dev,s:*mut sk_buff)->i32 { if (*s).len<2{return -5;} if (*d).curr_rf_tech==0{skb_pull(s,1);} if *(*s).data != (*s).len as u8{return -5;} skb_pull(s,1);0 }

// Protocol callbacks and packet construction retain the original C entry points.
#[no_mangle] pub unsafe extern "C" fn digital_in_send_atr_req_rs(d:*mut nfc_digital_dev,t:*mut nfc_target,cm:u8,g:*mut u8,l:usize)->i32 { digital_in_send_atr_req(d,t,cm,g,l) }
#[no_mangle] pub unsafe extern "C" fn digital_in_send_dep_req_rs(d:*mut nfc_digital_dev,t:*mut nfc_target,s:*mut sk_buff,x:*mut digital_data_exch)->i32 { digital_in_send_dep_req(d,t,s,x) }
#[no_mangle] pub unsafe extern "C" fn digital_tg_send_dep_res_rs(d:*mut nfc_digital_dev,s:*mut sk_buff)->i32 { digital_tg_send_dep_res(d,s) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
