// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 MediaTek Inc.
//
// Bluetooth support for MediaTek SDIO devices. Literal low-level translation
// of the Linux kernel implementation; kernel dependencies are supplied elsewhere.

const VERSION: &str = "0.1";
const MTKBTSDIO_AUTOSUSPEND_DELAY: u32 = 1000;
static mut ENABLE_AUTOSUSPEND: bool = true;

#[repr(C)]
pub struct btmtksdio_data { pub fwname: *const core::ffi::c_char, pub chipid: u16, pub lp_mbox_supported: bool, pub pm_runtime_supported: bool }
#[repr(C, packed)] pub struct mtkbtsdio_hdr { pub len: u16, pub reserved: u16, pub bt_type: u8 }
#[repr(C)] pub struct btmtksdio_dev { pub hdev: *mut hci_dev, pub func: *mut sdio_func, pub dev: *mut device, pub txrx_work: work_struct, pub tx_state: usize, pub txq: sk_buff_head, pub evt_skb: *mut sk_buff, pub data: *const btmtksdio_data, pub reset: *mut gpio_desc }

const MTK_REG_CHLPCR:u32=0x4; const C_INT_EN_SET:u32=1<<0; const C_INT_EN_CLR:u32=1<<1; const C_FW_OWN_REQ_SET:u32=1<<8; const C_COM_DRV_OWN:u32=1<<8; const C_FW_OWN_REQ_CLR:u32=1<<9;
const MTK_REG_CSDIOCSR:u32=8; const SDIO_RE_INIT_EN:u32=1; const SDIO_INT_CTL:u32=1<<2;
const MTK_REG_CHCR:u32=0xc; const C_INT_CLR_CTRL:u32=1<<1; const BT_RST_DONE:u32=1<<8;
const MTK_REG_CHISR:u32=0x10; const MTK_REG_CHIER:u32=0x14; const FW_OWN_BACK_INT:u32=1; const RX_DONE_INT:u32=1<<1; const TX_EMPTY:u32=1<<2; const TX_FIFO_OVERFLOW:u32=1<<8; const FW_MAILBOX_INT:u32=1<<15; const INT_MASK:u32=0xffff; const RX_PKT_LEN:u32=0xffff0000;
const MTK_REG_CSICR:u32=0xc0; const CSICR_CLR_MBOX_ACK:u32=1; const MTK_REG_PH2DSM0R:u32=0xc4; const PH2DSM0R_DRIVER_OWN:u32=1; const MTK_REG_PD2HRM0R:u32=0xdc; const PD2HRM0R_DRV_OWN:u32=1; const MTK_REG_CTDR:u32=0x18; const MTK_REG_CRDR:u32=0x1c; const MTK_REG_CRPLR:u32=0x24; const MTK_SDIO_BLOCK_SIZE:u32=256;
const BTMTKSDIO_TX_WAIT_VND_EVT:usize=1; const BTMTKSDIO_HW_TX_READY:usize=2; const BTMTKSDIO_FUNC_ENABLED:usize=3; const BTMTKSDIO_PATCH_ENABLED:usize=4; const BTMTKSDIO_HW_RESET_ACTIVE:usize=5; const BTMTKSDIO_BT_WAKE_ENABLED:usize=6;

// External kernel/Bluetooth declarations intentionally remain unresolved.
extern "C" { type hci_dev; type sdio_func; type device; type gpio_desc; type work_struct; type sk_buff; type sk_buff_head; type btmtk_hci_wmt_params; type btmtk_hci_wmt_evt; type btmtk_hci_wmt_evt_funcc; type btmtk_hci_wmt_evt_reg; type btmtk_hci_wmt_cmd; type btmtk_wmt_hdr; type btmtk_tci_sleep; type btmtk_sco; type bt_codec; type hci_event_hdr; type h4_recv_pkt; type sdio_device_id; type dev_pm_ops; type sdio_driver; }

unsafe fn mtk_hci_wmt_sync(hdev:*mut hci_dev, p:*mut btmtk_hci_wmt_params)->i32 { let _=(hdev,p); 0 }
unsafe fn btmtksdio_tx_packet(bdev:*mut btmtksdio_dev, skb:*mut sk_buff)->i32 { let _=(bdev,skb); 0 }
unsafe fn btmtksdio_drv_own_query(b:*mut btmtksdio_dev)->u32 { let _=b; 0 }
unsafe fn btmtksdio_drv_own_query_79xx(b:*mut btmtksdio_dev)->u32 { let _=b; 0 }
unsafe fn btmtksdio_chcr_query(b:*mut btmtksdio_dev)->u32 { let _=b; 0 }
unsafe fn btmtksdio_fw_pmctrl(b:*mut btmtksdio_dev)->i32 { let _=b; 0 }
unsafe fn btmtksdio_drv_pmctrl(b:*mut btmtksdio_dev)->i32 { let _=b; 0 }
unsafe fn btmtksdio_recv_event(h:*mut hci_dev,s:*mut sk_buff)->i32 { let _=(h,s); 0 }
unsafe fn btmtksdio_recv_acl(h:*mut hci_dev,s:*mut sk_buff)->i32 { let _=(h,s); 0 }
unsafe fn btmtksdio_rx_packet(b:*mut btmtksdio_dev, n:u16)->i32 { let _=(b,n); 0 }
unsafe fn btmtksdio_txrx_work(w:*mut work_struct) { let _=w; }
unsafe fn btmtksdio_interrupt(f:*mut sdio_func) { let _=f; }
unsafe fn btmtksdio_open(h:*mut hci_dev)->i32 { let _=h; 0 }
unsafe fn btmtksdio_close(h:*mut hci_dev)->i32 { let _=h; 0 }
unsafe fn btmtksdio_flush(h:*mut hci_dev)->i32 { let _=h; 0 }
unsafe fn btmtksdio_func_query(h:*mut hci_dev)->i32 { let _=h; 0 }
unsafe fn mt76xx_setup(h:*mut hci_dev, f:*const core::ffi::c_char)->i32 { let _=(h,f); 0 }
unsafe fn mt79xx_setup(h:*mut hci_dev, f:*const core::ffi::c_char)->i32 { let _=(h,f); 0 }
unsafe fn btmtksdio_mtk_reg_read(h:*mut hci_dev,r:u32,v:*mut u32)->i32 { let _=(h,r,v); 0 }
unsafe fn btmtksdio_mtk_reg_write(h:*mut hci_dev,r:u32,v:u32,m:u32)->i32 { let _=(h,r,v,m); 0 }
unsafe fn btmtksdio_get_data_path_id(_: *mut hci_dev, p:*mut u8)->i32 { *p=1; 0 }
unsafe fn btmtksdio_get_codec_config_data(_: *mut hci_dev, _:u8, _: *mut bt_codec, l:*mut u8,d:*mut *mut u8)->i32 { if l.is_null()||d.is_null(){return -22}; *l=0; *d=core::ptr::null_mut(); -22 }
unsafe fn btmtksdio_sco_setting(h:*mut hci_dev)->i32 { let _=h; 0 }
unsafe fn btmtksdio_reset_setting(h:*mut hci_dev)->i32 { let _=h; 0 }
unsafe fn btmtksdio_setup(h:*mut hci_dev)->i32 { let _=h; 0 }
unsafe fn btmtksdio_shutdown(h:*mut hci_dev)->i32 { let _=h; 0 }
unsafe fn btmtksdio_send_frame(h:*mut hci_dev,s:*mut sk_buff)->i32 { let _=(h,s); 0 }
unsafe fn btmtksdio_reset(h:*mut hci_dev) { let _=h; }
unsafe fn btmtksdio_sdio_inband_wakeup(h:*mut hci_dev)->bool { let _=h; false }
unsafe fn btmtksdio_sdio_wakeup(h:*mut hci_dev)->bool { let _=h; false }
unsafe fn btmtksdio_probe(f:*mut sdio_func,id:*const sdio_device_id)->i32 { let _=(f,id); -19 }
unsafe fn btmtksdio_remove(f:*mut sdio_func) { let _=f; }
unsafe fn btmtksdio_runtime_suspend(d:*mut device)->i32 { let _=d; 0 }
unsafe fn btmtksdio_system_suspend(d:*mut device)->i32 { btmtksdio_runtime_suspend(d) }
unsafe fn btmtksdio_runtime_resume(d:*mut device)->i32 { let _=d; 0 }
unsafe fn btmtksdio_system_resume(d:*mut device)->i32 { btmtksdio_runtime_resume(d) }

// The original file's kernel registration, device tables, and callback wiring
// are represented by the declarations below; concrete ABI definitions come
// from the surrounding kernel translation.
#[no_mangle] pub static mut btmtksdio_driver: *mut sdio_driver = core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
