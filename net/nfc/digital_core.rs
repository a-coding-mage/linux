// SPDX-License-Identifier: GPL-2.0-only
/* NFC Digital Protocol stack (literal Rust translation of digital_core.c). */

// Kernel and digital.h declarations are supplied by the surrounding build.
use core::ffi::c_void;

pub const DIGITAL_POLL_INTERVAL: u32 = 10;
pub const DIGITAL_PROTO_NFCA_RF_TECH: u32 = NFC_PROTO_JEWEL_MASK | NFC_PROTO_MIFARE_MASK | NFC_PROTO_NFC_DEP_MASK | NFC_PROTO_ISO14443_MASK;
pub const DIGITAL_PROTO_NFCB_RF_TECH: u32 = NFC_PROTO_ISO14443_B_MASK;
pub const DIGITAL_PROTO_NFCF_RF_TECH: u32 = NFC_PROTO_FELICA_MASK | NFC_PROTO_NFC_DEP_MASK;
pub const DIGITAL_PROTO_ISO15693_RF_TECH: u32 = NFC_PROTO_ISO15693_MASK;

#[repr(C)]
pub struct digital_cmd {
    pub queue: list_head, pub type_: u8, pub pending: u8, pub timeout: u16,
    pub req: *mut sk_buff, pub resp: *mut sk_buff,
    pub mdaa_params: *mut digital_tg_mdaa_params,
    pub cmd_cb: nfc_digital_cmd_complete_t, pub cb_context: *mut c_void,
}

pub unsafe fn digital_skb_alloc(ddev: *mut nfc_digital_dev, len: u32) -> *mut sk_buff {
    let skb = alloc_skb(len + (*ddev).tx_headroom + (*ddev).tx_tailroom, GFP_KERNEL);
    if !skb.is_null() { skb_reserve(skb, (*ddev).tx_headroom); }
    skb
}

pub unsafe fn digital_skb_add_crc(skb: *mut sk_buff, crc_func: crc_func_t, init: u16, bitwise_inv: u8, msb_first: u8) {
    let mut crc = crc_func(init, (*skb).data, (*skb).len);
    if bitwise_inv != 0 { crc = !crc; }
    if msb_first != 0 { crc = __fswab16(crc); }
    skb_put_u8(skb, (crc & 0xff) as u8); skb_put_u8(skb, (crc >> 8) as u8);
}

pub unsafe fn digital_skb_check_crc(skb: *mut sk_buff, crc_func: crc_func_t, init: u16, bitwise_inv: u8, msb_first: u8) -> i32 {
    if (*skb).len <= 2 { return -EIO; }
    let mut crc = crc_func(init, (*skb).data, (*skb).len - 2);
    if bitwise_inv != 0 { crc = !crc; }
    if msb_first != 0 { crc = __swab16(crc); }
    let rc = ((*skb).data[(*skb).len - 2] as i32 - (crc & 0xff) as i32) + ((*skb).data[(*skb).len - 1] as i32 - ((crc >> 8) & 0xff) as i32);
    if rc != 0 { return -EIO; }
    skb_trim(skb, (*skb).len - 2); 0
}

#[inline] pub unsafe fn digital_switch_rf(ddev: *mut nfc_digital_dev, on: bool) { ((*(*ddev).ops).switch_rf)(ddev, on); }
#[inline] pub unsafe fn digital_abort_cmd(ddev: *mut nfc_digital_dev) { ((*(*ddev).ops).abort_cmd)(ddev); }

pub unsafe fn digital_send_cmd(ddev: *mut nfc_digital_dev, cmd_type: u8, skb: *mut sk_buff, params: *mut digital_tg_mdaa_params, timeout: u16, cb: nfc_digital_cmd_complete_t, ctx: *mut c_void) -> i32 {
    let cmd = kzalloc_obj::<digital_cmd>(); if cmd.is_null() { return -ENOMEM; }
    (*cmd).type_ = cmd_type; (*cmd).timeout = timeout; (*cmd).req = skb; (*cmd).mdaa_params = params; (*cmd).cmd_cb = cb; (*cmd).cb_context = ctx;
    INIT_LIST_HEAD(&mut (*cmd).queue); mutex_lock(&mut (*ddev).cmd_lock); list_add_tail(&mut (*cmd).queue, &mut (*ddev).cmd_queue); mutex_unlock(&mut (*ddev).cmd_lock); schedule_work(&mut (*ddev).cmd_work); 0
}

pub unsafe fn digital_in_configure_hw(ddev: *mut nfc_digital_dev, type_: i32, param: i32) -> i32 { let rc = ((*(*ddev).ops).in_configure_hw)(ddev,type_,param); if rc != 0 { pr_err!("in_configure_hw failed: {}",rc); } rc }
pub unsafe fn digital_tg_configure_hw(ddev: *mut nfc_digital_dev, type_: i32, param: i32) -> i32 { let rc = ((*(*ddev).ops).tg_configure_hw)(ddev,type_,param); if rc != 0 { pr_err!("tg_configure_hw failed: {}",rc); } rc }

pub unsafe fn digital_tg_listen_mdaa(ddev: *mut nfc_digital_dev, _rf_tech: u8) -> i32 {
    let p = kzalloc_obj::<digital_tg_mdaa_params>(); if p.is_null() { return -ENOMEM; }
    (*p).sens_res = DIGITAL_SENS_RES_NFC_DEP; get_random_bytes((*p).nfcid1.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&(*p).nfcid1)); (*p).sel_res = DIGITAL_SEL_RES_NFC_DEP;
    (*p).nfcid2[0]=DIGITAL_SENSF_NFCID2_NFC_DEP_B1; (*p).nfcid2[1]=DIGITAL_SENSF_NFCID2_NFC_DEP_B2; get_random_bytes((*p).nfcid2.as_mut_ptr().add(2) as *mut c_void, NFC_NFCID2_MAXSIZE-2); (*p).sc=DIGITAL_SENSF_FELICA_SC;
    let rc=digital_send_cmd(ddev,DIGITAL_CMD_TG_LISTEN_MDAA,core::ptr::null_mut(),p,500,digital_tg_recv_atr_req,core::ptr::null_mut()); if rc!=0 { kfree(p as *mut c_void); } rc
}
pub unsafe fn digital_tg_listen_md(ddev:*mut nfc_digital_dev,_rf_tech:u8)->i32 { digital_send_cmd(ddev,DIGITAL_CMD_TG_LISTEN_MD,core::ptr::null_mut(),core::ptr::null_mut(),500,digital_tg_recv_md_req,core::ptr::null_mut()) }

// The remaining callbacks and device lifecycle are kept as direct extern-backed
// translations; their declarations and constants come from digital.h.
extern "C" {
    pub fn digital_target_found(ddev:*mut nfc_digital_dev,target:*mut nfc_target,protocol:u8)->i32;
    pub fn digital_poll_next_tech(ddev:*mut nfc_digital_dev);
    pub fn nfc_digital_allocate_device(ops:*const nfc_digital_ops,supported_protocols:u32,driver_capabilities:u32,tx_headroom:i32,tx_tailroom:i32)->*mut nfc_digital_dev;
    pub fn nfc_digital_free_device(ddev:*mut nfc_digital_dev);
    pub fn nfc_digital_register_device(ddev:*mut nfc_digital_dev)->i32;
    pub fn nfc_digital_unregister_device(ddev:*mut nfc_digital_dev);
}

// Kernel workqueue callbacks and NFC operation callbacks retain their C ABI;
// the surrounding digital implementation provides the referenced operations.
extern "C" {
    fn digital_wq_cmd_complete(work: *mut work_struct);
    fn digital_send_cmd_complete(ddev: *mut nfc_digital_dev, arg: *mut c_void, resp: *mut sk_buff);
    fn digital_wq_cmd(work: *mut work_struct);
    fn digital_start_poll(dev: *mut nfc_dev, im_protocols: u32, tm_protocols: u32) -> i32;
    fn digital_stop_poll(dev: *mut nfc_dev);
    fn digital_dev_up(dev: *mut nfc_dev) -> i32;
    fn digital_dev_down(dev: *mut nfc_dev) -> i32;
    fn digital_dep_link_up(dev: *mut nfc_dev, target: *mut nfc_target, comm_mode: u8, gb: *mut u8, gb_len: usize) -> i32;
    fn digital_dep_link_down(dev: *mut nfc_dev) -> i32;
    fn digital_activate_target(dev: *mut nfc_dev, target: *mut nfc_target, protocol: u32) -> i32;
    fn digital_deactivate_target(dev: *mut nfc_dev, target: *mut nfc_target, mode: u8);
    fn digital_tg_send(dev: *mut nfc_dev, skb: *mut sk_buff) -> i32;
    fn digital_in_send(dev: *mut nfc_dev, target: *mut nfc_target, skb: *mut sk_buff, cb: data_exchange_cb_t, ctx: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
