// SPDX-License-Identifier: GPL-2.0-only
/* NFC Digital Protocol stack */

// C dependency declarations are supplied by the surrounding NFC implementation.

const DIGITAL_CMD_SENS_REQ: u8 = 0x26; const DIGITAL_CMD_ALL_REQ: u8 = 0x52;
const DIGITAL_CMD_SEL_REQ_CL1: u8 = 0x93; const DIGITAL_CMD_SEL_REQ_CL2: u8 = 0x95; const DIGITAL_CMD_SEL_REQ_CL3: u8 = 0x97;
const DIGITAL_SDD_REQ_SEL_PAR: u8 = 0x20; const DIGITAL_SDD_RES_CT: u8 = 0x88;
const DIGITAL_SDD_RES_LEN: usize = 5; const DIGITAL_SEL_RES_LEN: usize = 1;
const DIGITAL_MIFARE_READ_RES_LEN: usize = 16; const DIGITAL_MIFARE_ACK_RES: u8 = 0x0a;
const DIGITAL_CMD_SENSB_REQ: u8 = 5; const DIGITAL_CMD_SENSB_RES: u8 = 0x50; const DIGITAL_CMD_ATTRIB_REQ: u8 = 0x1d;
const DIGITAL_CMD_SENSF_REQ: u8 = 0; const DIGITAL_CMD_SENSF_RES: u8 = 1; const DIGITAL_SENSF_RES_MIN_LENGTH: usize = 17;
const DIGITAL_SENSF_RES_RD_AP_B1: u8 = 0; const DIGITAL_SENSF_RES_RD_AP_B2: u8 = 0x8f;
const DIGITAL_SENSF_REQ_RC_NONE: u8 = 0; const DIGITAL_SENSF_REQ_RC_SC: u8 = 1; const DIGITAL_SENSF_REQ_RC_AP: u8 = 2;
const DIGITAL_CMD_ISO15693_INVENTORY_REQ: u8 = 1; const DIGITAL_ISO_DEP_I_PCB: u8 = 2; const DIGITAL_ISO_DEP_I_BLOCK: u8 = 0;
const DIGITAL_ATS_MAX_FSC: u16 = 256; const DIGITAL_RATS_BYTE1: u8 = 0xe0; const DIGITAL_RATS_PARAM: u8 = 0x80;
static DIGITAL_ATS_FSC: [u8; 8] = [16,24,32,40,48,64,96,128];

#[repr(C, packed)] pub struct digital_sdd_res { pub nfcid1: [u8;4], pub bcc: u8 }
#[repr(C, packed)] pub struct digital_sel_req { pub sel_cmd:u8, pub b2:u8, pub nfcid1:[u8;4], pub bcc:u8 }
#[repr(C, packed)] pub struct digital_sensb_req { pub cmd:u8, pub afi:u8, pub param:u8 }
#[repr(C, packed)] pub struct digital_sensb_res { pub cmd:u8, pub nfcid0:[u8;4], pub app_data:[u8;4], pub proto_info:[u8;3] }
#[repr(C, packed)] pub struct digital_attrib_req { pub cmd:u8, pub nfcid0:[u8;4], pub param1:u8, pub param2:u8, pub param3:u8, pub param4:u8 }
#[repr(C, packed)] pub struct digital_attrib_res { pub mbli_did:u8 }
#[repr(C, packed)] pub struct digital_sensf_req { pub cmd:u8, pub sc1:u8, pub sc2:u8, pub rc:u8, pub tsn:u8 }
#[repr(C, packed)] pub struct digital_sensf_res { pub cmd:u8, pub nfcid2:[u8;8], pub pad0:[u8;2], pub pad1:[u8;3], pub mrti_check:u8, pub mrti_update:u8, pub pad2:u8, pub rd:[u8;2] }
#[repr(C, packed)] pub struct digital_iso15693_inv_req { pub flags:u8, pub cmd:u8, pub mask_len:u8, pub mask:u64 }
#[repr(C, packed)] pub struct digital_iso15693_inv_res { pub flags:u8, pub dsfid:u8, pub uid:u64 }

// The following functions preserve the C implementation's pointer-oriented behavior.
pub unsafe fn digital_in_iso_dep_pull_sod(ddev:*mut nfc_digital_dev, skb:*mut sk_buff)->i32 { let _=ddev; if (*skb).len<1{return -EIO;} let pcb=*(*skb).data; if pcb&0xc0!=0 { pr_err("ISO_DEP R-block and S-block not supported\n"); return -EIO;} if pcb&8!=0 {pr_err("DID field in ISO_DEP PCB not supported\n");return -EIO;} skb_pull(skb,1);0 }
pub unsafe fn digital_in_iso_dep_push_sod(ddev:*mut nfc_digital_dev,skb:*mut sk_buff)->i32 { if (*skb).len+3>(*ddev).target_fsc as usize{return -EIO;} skb_push(skb,1);*(*skb).data=2|(*ddev).curr_nfc_dep_pni;(*ddev).curr_nfc_dep_pni=((*ddev).curr_nfc_dep_pni+1)&1;0 }

pub unsafe fn digital_in_send_sens_req(ddev:*mut nfc_digital_dev,_rf_tech:u8)->i32 { let mut r=digital_in_configure_hw(ddev,NFC_DIGITAL_CONFIG_RF_TECH,NFC_DIGITAL_RF_TECH_106A);if r!=0{return r} r=digital_in_configure_hw(ddev,NFC_DIGITAL_CONFIG_FRAMING,NFC_DIGITAL_FRAMING_NFCA_SHORT);if r!=0{return r} let skb=digital_skb_alloc(ddev,1);if skb.is_null(){return -ENOMEM} skb_put_u8(skb,DIGITAL_CMD_SENS_REQ);r=digital_in_send_cmd(ddev,skb,30,digital_in_recv_sens_res(core::ptr::null_mut()),core::ptr::null_mut());if r!=0{kfree_skb(skb)}r }

pub unsafe fn digital_in_recv_mifare_res(resp:*mut sk_buff)->i32 { if (*resp).len==DIGITAL_MIFARE_READ_RES_LEN+DIGITAL_CRC_LEN {if digital_skb_check_crc_a(resp)!=0{return -EIO}return 0} if (*resp).len==1&&*(*resp).data==DIGITAL_MIFARE_ACK_RES {*(*resp).data=0;return 0}-EIO }

pub unsafe fn digital_tg_listen_nfca(ddev:*mut nfc_digital_dev,_rf_tech:u8)->i32 { let r=digital_tg_configure_hw(ddev,NFC_DIGITAL_CONFIG_RF_TECH,NFC_DIGITAL_RF_TECH_106A);if r!=0{return r} let r=digital_tg_configure_hw(ddev,NFC_DIGITAL_CONFIG_FRAMING,NFC_DIGITAL_FRAMING_NFCA_NFC_DEP);if r!=0{return r} digital_tg_listen(ddev,300,digital_tg_recv_sens_req,core::ptr::null_mut()) }
pub unsafe fn digital_tg_listen_nfcf(ddev:*mut nfc_digital_dev,rf_tech:u8)->i32 { let r=digital_tg_configure_hw(ddev,NFC_DIGITAL_CONFIG_RF_TECH,rf_tech);if r!=0{return r} let r=digital_tg_configure_hw(ddev,NFC_DIGITAL_CONFIG_FRAMING,NFC_DIGITAL_FRAMING_NFCF_NFC_DEP);if r!=0{return r} digital_tg_listen(ddev,300,digital_tg_recv_sensf_req,core::ptr::null_mut()) }

// External callbacks and helpers referenced by the remaining protocol paths.
extern "C" {
    fn digital_in_recv_sens_res(ddev:*mut nfc_digital_dev,arg:*mut core::ffi::c_void,resp:*mut sk_buff);
    fn digital_tg_recv_sens_req(ddev:*mut nfc_digital_dev,arg:*mut core::ffi::c_void,resp:*mut sk_buff);
    fn digital_tg_recv_sensf_req(ddev:*mut nfc_digital_dev,arg:*mut core::ffi::c_void,resp:*mut sk_buff);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
