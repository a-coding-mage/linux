// SPDX-License-Identifier: ISC
/*
 * Copyright (c) 2017 Redpine Signals Inc.
 */

// Linux kernel and RSI driver headers provide the external types and symbols
// referenced below.

pub const RSI_DMA_ALIGN: usize = 8;
pub const RSI_FRAME_DESC_SIZE: usize = 16;
pub const RSI_HEADROOM_FOR_BT_HAL: usize = RSI_FRAME_DESC_SIZE + RSI_DMA_ALIGN;

#[repr(C)]
pub struct RsiHciAdapter {
    pub priv_: *mut core::ffi::c_void,
    pub proto_ops: *mut RsiProtoOps,
    pub hdev: *mut HciDev,
}

#[repr(C)]
pub struct RsiProtoOps {
    pub get_host_intf: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
    pub set_bt_context: unsafe extern "C" fn(*mut core::ffi::c_void, *mut RsiHciAdapter),
    pub coex_send_pkt: unsafe extern "C" fn(*mut core::ffi::c_void, *mut SkBuff, i32) -> i32,
}

#[repr(C)]
pub struct HciStats {
    pub cmd_tx: u64,
    pub acl_tx: u64,
    pub sco_tx: u64,
    pub byte_rx: u64,
}

#[repr(C)]
pub struct HciDev {
    pub stat: HciStats,
    pub bus: i32,
    pub open: Option<unsafe extern "C" fn(*mut HciDev) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut HciDev) -> i32>,
    pub flush: Option<unsafe extern "C" fn(*mut HciDev) -> i32>,
    pub send: Option<unsafe extern "C" fn(*mut HciDev, *mut SkBuff) -> i32>,
}

#[repr(C)]
pub struct SkBuff {
    pub data: *mut u8,
    pub len: usize,
    pub pkt_type: u8,
}

#[repr(C)]
pub struct RsiModOps {
    pub attach: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut RsiProtoOps) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub recv_pkt: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const u8) -> i32>,
}

pub const RSI_BT_Q: i32 = 0;
pub const RSI_HOST_INTF_SDIO: i32 = 0;
pub const HCI_SDIO: i32 = 0;
pub const HCI_USB: i32 = 1;
pub const HCI_COMMAND_PKT: u8 = 1;
pub const HCI_ACLDATA_PKT: u8 = 2;
pub const HCI_SCODATA_PKT: u8 = 3;

unsafe extern "C" {
    fn hci_get_drvdata(hdev: *mut HciDev) -> *mut RsiHciAdapter;
    fn hci_skb_pkt_type(skb: *mut SkBuff) -> u8;
    fn skb_headroom(skb: *mut SkBuff) -> usize;
    fn skb_realloc_headroom(skb: *mut SkBuff, headroom: usize) -> *mut SkBuff;
    fn kfree_skb(skb: *mut SkBuff);
    fn skb_push(skb: *mut SkBuff, len: usize);
    fn skb_pull(skb: *mut SkBuff, len: usize);
    fn skb_trim(skb: *mut SkBuff, len: usize);
    fn hci_alloc_dev() -> *mut HciDev;
    fn hci_set_drvdata(hdev: *mut HciDev, data: *mut RsiHciAdapter);
    fn hci_register_dev(hdev: *mut HciDev) -> i32;
    fn hci_free_dev(hdev: *mut HciDev);
    fn hci_unregister_dev(hdev: *mut HciDev);
    fn dev_alloc_skb(len: usize) -> *mut SkBuff;
    fn skb_put(skb: *mut SkBuff, len: usize);
    fn hci_recv_frame(hdev: *mut HciDev, skb: *mut SkBuff) -> i32;
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn get_unaligned_le16(ptr: *const u8) -> u16;
    fn memmove(dst: *mut u8, src: *const u8, len: usize) -> *mut u8;
}

unsafe extern "C" fn rsi_hci_open(_hdev: *mut HciDev) -> i32 { 0 }

unsafe extern "C" fn rsi_hci_close(_hdev: *mut HciDev) -> i32 { 0 }

unsafe extern "C" fn rsi_hci_flush(_hdev: *mut HciDev) -> i32 { 0 }

unsafe extern "C" fn rsi_hci_send_pkt(hdev: *mut HciDev, mut skb: *mut SkBuff) -> i32 {
    let h_adapter = hci_get_drvdata(hdev);
    let mut new_skb: *mut SkBuff = core::ptr::null_mut();

    match hci_skb_pkt_type(skb) {
        HCI_COMMAND_PKT => (*hdev).stat.cmd_tx += 1,
        HCI_ACLDATA_PKT => (*hdev).stat.acl_tx += 1,
        HCI_SCODATA_PKT => (*hdev).stat.sco_tx += 1,
        _ => {}
    }

    if skb_headroom(skb) < RSI_HEADROOM_FOR_BT_HAL {
        new_skb = skb_realloc_headroom(skb, RSI_HEADROOM_FOR_BT_HAL);
        if new_skb.is_null() { return -12; }
        (*new_skb).pkt_type = hci_skb_pkt_type(skb);
        kfree_skb(skb);
        skb = new_skb;
        if ((*skb).data as usize) % RSI_DMA_ALIGN != 0 {
            let skb_data = (*skb).data;
            let skb_len = (*skb).len;
            skb_push(skb, RSI_DMA_ALIGN);
            let aligned = ((*skb).data as usize + RSI_DMA_ALIGN - 1) & !(RSI_DMA_ALIGN - 1);
            skb_pull(skb, aligned - (*skb).data as usize);
            memmove((*skb).data, skb_data, skb_len);
            skb_trim(skb, skb_len);
        }
    }

    ((*(*h_adapter).proto_ops).coex_send_pkt)((*h_adapter).priv_, skb, RSI_BT_Q)
}

unsafe extern "C" fn rsi_hci_recv_pkt(priv_: *mut core::ffi::c_void, pkt: *const u8) -> i32 {
    let h_adapter = priv_ as *mut RsiHciAdapter;
    let hdev = (*h_adapter).hdev;
    let pkt_len = (get_unaligned_le16(pkt) & 0x0fff) as usize;
    let skb = dev_alloc_skb(pkt_len);
    if skb.is_null() { return -12; }
    core::ptr::copy_nonoverlapping(pkt.add(RSI_FRAME_DESC_SIZE), (*skb).data, pkt_len);
    skb_put(skb, pkt_len);
    (*hdev).stat.byte_rx += (*skb).len as u64;
    (*skb).pkt_type = *pkt.add(14);
    hci_recv_frame(hdev, skb)
}

unsafe extern "C" fn rsi_hci_attach(priv_: *mut core::ffi::c_void, ops: *mut RsiProtoOps) -> i32 {
    let h_adapter = kzalloc(core::mem::size_of::<RsiHciAdapter>()) as *mut RsiHciAdapter;
    if h_adapter.is_null() { return -12; }
    (*h_adapter).priv_ = priv_;
    (*h_adapter).proto_ops = ops;
    let hdev = hci_alloc_dev();
    if hdev.is_null() { kfree(h_adapter.cast()); return -22; }
    (*h_adapter).hdev = hdev;
    (*hdev).bus = if ((*ops).get_host_intf)(priv_) == RSI_HOST_INTF_SDIO { HCI_SDIO } else { HCI_USB };
    hci_set_drvdata(hdev, h_adapter);
    (*hdev).open = Some(rsi_hci_open);
    (*hdev).close = Some(rsi_hci_close);
    (*hdev).flush = Some(rsi_hci_flush);
    (*hdev).send = Some(rsi_hci_send_pkt);
    let err = hci_register_dev(hdev);
    if err < 0 { hci_free_dev(hdev); kfree(h_adapter.cast()); return -22; }
    ((*ops).set_bt_context)(priv_, h_adapter);
    0
}

unsafe extern "C" fn rsi_hci_detach(priv_: *mut core::ffi::c_void) {
    let h_adapter = priv_ as *mut RsiHciAdapter;
    if h_adapter.is_null() { return; }
    let hdev = (*h_adapter).hdev;
    if !hdev.is_null() { hci_unregister_dev(hdev); hci_free_dev(hdev); (*h_adapter).hdev = core::ptr::null_mut(); }
    kfree(h_adapter.cast());
}

#[no_mangle]
pub static mut rsi_bt_ops: RsiModOps = RsiModOps { attach: Some(rsi_hci_attach), detach: Some(rsi_hci_detach), recv_pkt: Some(rsi_hci_recv_pkt) };

unsafe extern "C" fn rsi_91x_bt_module_init() -> i32 { 0 }
unsafe extern "C" fn rsi_91x_bt_module_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
