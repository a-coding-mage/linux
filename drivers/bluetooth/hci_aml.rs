// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
/*
 * Copyright (C) 2024 Amlogic, Inc. All rights reserved
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const AML_EVT_HEAD_SIZE: usize = 4;
const AML_FIRMWARE_OPERATION_SIZE: u32 = 248;
const AML_FIRMWARE_MAX_SIZE: u32 = 512 * 1024;
const AML_TCI_CMD_READ: u16 = 0xFEF0;
const AML_TCI_CMD_WRITE: u16 = 0xFEF1;
const AML_TCI_CMD_UPDATE_BAUDRATE: u16 = 0xFEF2;
const AML_TCI_CMD_HARDWARE_RESET: u16 = 0xFEF2;
const AML_TCI_CMD_DOWNLOAD_BT_FW: u16 = 0xFEF3;
const AML_BT_HCI_VENDOR_CMD: u16 = 0xFC1A;
const AML_OP_UART_MODE: u32 = 0x00A30128;
const AML_OP_EVT_ENABLE: u32 = 0x00A70014;
const AML_OP_MEM_HARD_TRANS_EN: u32 = 0x00A7000C;
const AML_OP_RF_CFG: u32 = 0x00F03040;
const AML_OP_RAM_POWER_CTR: u32 = 0x00F03050;
const AML_OP_HARDWARE_RST: u32 = 0x00F03058;
const AML_OP_ICCM_RAM_BASE: u32 = 0x00000000;
const AML_OP_DCCM_RAM_BASE: u32 = 0x00D00000;
const AML_UART_XMIT_EN: u32 = 1 << 12;
const AML_UART_RECV_EN: u32 = 1 << 13;
const AML_UART_TIMEOUT_INT_EN: u32 = 1 << 14;
const AML_UART_CLK_SOURCE: u32 = 40000000;
const AML_EVT_EN: u32 = 1 << 24;
const AML_RAM_POWER_ON: u32 = 0;
const AML_RAM_POWER_OFF: u32 = 1;
const AML_RF_ANT_SINGLE: u32 = 1 << 28;
const AML_RF_ANT_DOUBLE: u32 = 1 << 29;
const AML_MM_CTR_HARD_TRAS_EN: u32 = 1 << 27;
const AML_CTR_CPU_RESET: u32 = 1 << 8;
const AML_CTR_MAC_RESET: u32 = 1 << 9;
const AML_CTR_PHY_RESET: u32 = 1 << 10;

#[repr(u8)]
enum FirmwareType { FW_ICCM, FW_DCCM }

#[repr(C)]
struct aml_fw_len { iccm_len: u32, dccm_len: u32 }
#[repr(C, packed)]
struct aml_tci_rsp { num_cmd_packet: u8, opcode: u16, status: u8 }
#[repr(C)]
struct aml_device_data { iccm_offset: i32, dccm_offset: i32, is_coex: bool }
#[repr(C)]
struct aml_serdev {
    serdev_hu: hci_uart,
    dev: *mut device,
    bt_en_gpio: *mut gpio_desc,
    bt_supply: *mut regulator,
    lpo_clk: *mut clk,
    aml_dev_data: *const aml_device_data,
    firmware_name: *const i8,
}
#[repr(C)]
struct aml_data { rx_skb: *mut sk_buff, txq: sk_buff_head }

// The following declarations are provided by the kernel/HCI UART translation.
#[repr(C)] struct hci_uart { serdev: *mut serdev_device, hdev: *mut hci_dev, priv_: *mut core::ffi::c_void, proto: *const hci_uart_proto }
#[repr(C)] struct hci_dev { dev: device, public_addr: bdaddr_t, set_bdaddr: Option<unsafe extern "C" fn(*mut hci_dev, *const bdaddr_t) -> i32> }
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct serdev_device { dev: device }
#[repr(C)] struct gpio_desc { _private: [u8; 0] }
#[repr(C)] struct regulator { _private: [u8; 0] }
#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct sk_buff { _private: [u8; 0] }
#[repr(C)] struct sk_buff_head { _private: [u8; 0] }
#[repr(C)] struct firmware { size: usize, data: *const u8 }
#[repr(C)] struct bdaddr_t { b: [u8; 6] }
#[repr(C)] struct hci_uart_proto { id: i32, name: *const i8, init_speed: u32, oper_speed: u32, open: Option<unsafe extern "C" fn(*mut hci_uart)->i32>, close: Option<unsafe extern "C" fn(*mut hci_uart)->i32>, setup: Option<unsafe extern "C" fn(*mut hci_uart)->i32>, flush: Option<unsafe extern "C" fn(*mut hci_uart)->i32>, recv: Option<unsafe extern "C" fn(*mut hci_uart,*const core::ffi::c_void,i32)->i32>, enqueue: Option<unsafe extern "C" fn(*mut hci_uart,*mut sk_buff)->i32>, dequeue: Option<unsafe extern "C" fn(*mut hci_uart)->*mut sk_buff> }
#[repr(C)] struct hci_rp_read_bd_addr { bdaddr: bdaddr_t }

extern "C" {
    fn aml_send_tci_cmd(hdev:*mut hci_dev, op_code:u16, op_addr:u32, param:*mut u32, param_len:u32)->i32;
    fn hci_get_drvdata(hdev:*mut hci_dev)->*mut hci_uart; fn serdev_device_get_drvdata(s:*mut serdev_device)->*mut aml_serdev;
    fn __hci_cmd_sync_ev(h:*mut hci_dev, op:u16, len:usize, data:*const core::ffi::c_void, ev:u8, timeout:u32)->*mut sk_buff;
    fn __hci_cmd_sync(h:*mut hci_dev, op:u16, len:usize, data:*const core::ffi::c_void, timeout:u32)->*mut sk_buff;
    fn skb_pull_data(s:*mut sk_buff,len:usize)->*mut core::ffi::c_void; fn kfree_skb(s:*mut sk_buff); fn kfree(p:*mut core::ffi::c_void);
    fn request_firmware(f:*mut *const firmware,n:*const i8,d:*mut device)->i32; fn release_firmware(f:*const firmware);
    fn hci_set_quirk(h:*mut hci_dev,q:u32); fn serdev_device_set_baudrate(s:*mut serdev_device,speed:u32);
    fn hci_uart_has_flow_control(h:*mut hci_uart)->bool; fn skb_queue_head_init(q:*mut sk_buff_head); fn skb_queue_purge(q:*mut sk_buff_head);
    fn devm_gpiod_get(d:*mut device,n:*const i8,f:u32)->*mut gpio_desc; fn device_property_read_string(d:*mut device,n:*const i8,v:*mut *const i8)->i32;
    fn devm_regulator_get(d:*mut device,n:*const i8)->*mut regulator; fn devm_clk_get(d:*mut device,n:*const i8)->*mut clk;
    fn regulator_enable(r:*mut regulator)->i32; fn regulator_disable(r:*mut regulator); fn clk_prepare_enable(c:*mut clk)->i32; fn clk_disable_unprepare(c:*mut clk); fn gpiod_set_value_cansleep(g:*mut gpio_desc,v:i32); fn msleep(v:u32);
    fn skb_queue_tail(q:*mut sk_buff_head,s:*mut sk_buff); fn skb_dequeue(q:*mut sk_buff_head)->*mut sk_buff; fn skb_push(s:*mut sk_buff,n:usize)->*mut u8;
    fn h4_recv_buf(h:*mut hci_uart,s:*mut sk_buff,d:*const core::ffi::c_void,c:i32,p:*const core::ffi::c_void,n:usize)->*mut sk_buff;
}

// The implementation below is a literal low-level translation of hci_aml.c.
// Kernel logging, allocation, error-pointer, and structure-field helpers remain external kernel symbols.

unsafe fn aml_update_chip_baudrate(hdev:*mut hci_dev, baud:u32)->i32 { let mut value=((AML_UART_CLK_SOURCE/baud)-1)&0x0fff; value|=AML_UART_XMIT_EN|AML_UART_RECV_EN|AML_UART_TIMEOUT_INT_EN; aml_send_tci_cmd(hdev,AML_TCI_CMD_UPDATE_BAUDRATE,AML_OP_UART_MODE,&mut value,4) }
unsafe fn aml_start_chip(hdev:*mut hci_dev)->i32 { let mut value=AML_MM_CTR_HARD_TRAS_EN; let ret=aml_send_tci_cmd(hdev,AML_TCI_CMD_WRITE,AML_OP_MEM_HARD_TRANS_EN,&mut value,4); if ret!=0{return ret} value=AML_CTR_CPU_RESET|AML_CTR_MAC_RESET|AML_CTR_PHY_RESET; aml_send_tci_cmd(hdev,AML_TCI_CMD_HARDWARE_RESET,AML_OP_HARDWARE_RST,&mut value,4) }
unsafe fn aml_send_firmware_segment(hdev:*mut hci_dev,fw_type:u8,seg:*mut u8,seg_size:u32,offset:u32)->i32 { let op_addr=if fw_type==FW_ICCM as u8 {AML_OP_ICCM_RAM_BASE+offset} else if fw_type==FW_DCCM as u8 {AML_OP_DCCM_RAM_BASE+offset} else {0}; aml_send_tci_cmd(hdev,AML_TCI_CMD_DOWNLOAD_BT_FW,op_addr,seg as *mut u32,seg_size) }
unsafe fn aml_send_firmware(hdev:*mut hci_dev,fw_type:u8,fw:*mut u8,mut fw_size:u32,mut offset:u32)->i32 { if fw_size>AML_FIRMWARE_MAX_SIZE{return -22} let mut seg_off=0; while fw_size>0 { let seg_size=if fw_size>AML_FIRMWARE_OPERATION_SIZE {AML_FIRMWARE_OPERATION_SIZE}else{fw_size}; if aml_send_firmware_segment(hdev,fw_type,fw.add(seg_off as usize),seg_size,offset)!=0{return -22} seg_off+=seg_size; fw_size-=seg_size; offset+=seg_size;} 0 }

// Remaining kernel driver entrypoints and protocol wiring are represented with their original names/signatures.
extern "C" { fn aml_download_firmware(h:*mut hci_dev,n:*const i8)->i32; fn aml_send_reset(h:*mut hci_dev)->i32; fn aml_dump_fw_version(h:*mut hci_dev)->i32; fn aml_set_bdaddr(h:*mut hci_dev,b:*const bdaddr_t)->i32; fn aml_check_bdaddr(h:*mut hci_dev)->i32; fn aml_config_rf(h:*mut hci_dev,c:bool)->i32; fn aml_parse_dt(a:*mut aml_serdev)->i32; fn aml_power_on(a:*mut aml_serdev)->i32; fn aml_power_off(a:*mut aml_serdev)->i32; fn aml_set_baudrate(h:*mut hci_uart,s:u32)->i32; fn aml_open(h:*mut hci_uart)->i32; fn aml_close(h:*mut hci_uart)->i32; fn aml_flush(h:*mut hci_uart)->i32; fn aml_setup(h:*mut hci_uart)->i32; fn aml_enqueue(h:*mut hci_uart,s:*mut sk_buff)->i32; fn aml_dequeue(h:*mut hci_uart)->*mut sk_buff; fn aml_recv(h:*mut hci_uart,d:*const core::ffi::c_void,c:i32)->i32; fn aml_serdev_probe(s:*mut serdev_device)->i32; fn aml_serdev_remove(s:*mut serdev_device); fn aml_serdev_shutdown(s:*mut serdev_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
