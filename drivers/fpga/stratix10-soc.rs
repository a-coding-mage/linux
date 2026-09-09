// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Manager Driver for Intel Stratix10 SoC
 *
 *  Copyright (C) 2018 Intel Corporation
 */

// Linux kernel dependencies supplied by the surrounding build.
extern "C" {
    fn stratix10_svc_send(chan: *mut stratix10_svc_chan, msg: *mut stratix10_svc_client_msg) -> i32;
    fn stratix10_svc_free_memory(chan: *mut stratix10_svc_chan, buf: *mut i8);
    fn stratix10_svc_allocate_memory(chan: *mut stratix10_svc_chan, size: usize) -> *mut i8;
    fn stratix10_svc_done(chan: *mut stratix10_svc_chan);
    fn stratix10_svc_request_channel_byname(client: *mut stratix10_svc_client, name: u32) -> *mut stratix10_svc_chan;
    fn stratix10_svc_free_channel(chan: *mut stratix10_svc_chan);
}

const NUM_SVC_BUFS: usize = 4;
const SVC_BUF_SIZE: usize = 512 * 1024;
const SVC_BUF_LOCK: usize = 0;
const S10_BUFFER_TIMEOUT: u64 = SVC_RECONFIG_BUFFER_TIMEOUT_MS as u64;
const S10_RECONFIG_TIMEOUT: u64 = SVC_RECONFIG_REQUEST_TIMEOUT_MS as u64;

#[repr(C)]
pub struct s10_svc_buf {
    pub buf: *mut i8,
    pub lock: usize,
}

#[repr(C)]
pub struct s10_priv {
    pub chan: *mut stratix10_svc_chan,
    pub client: stratix10_svc_client,
    pub status_return_completion: completion,
    pub svc_bufs: [s10_svc_buf; NUM_SVC_BUFS],
    pub status: usize,
}

#[repr(C)] pub struct stratix10_svc_chan;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct fpga_image_info { pub flags: u32, pub config_complete_timeout_us: u64 }
#[repr(C)] pub struct fpga_manager { pub priv_: *mut s10_priv }
#[repr(C)] pub struct completion;
#[repr(C)] pub struct stratix10_svc_client { pub dev: *mut device, pub receive_cb: Option<unsafe extern "C" fn(*mut stratix10_svc_client, *mut stratix10_svc_cb_data)>, pub priv_: *mut s10_priv }
#[repr(C)] pub struct stratix10_svc_cb_data { pub status: u32, pub kaddr1: *mut i8, pub kaddr2: *mut i8, pub kaddr3: *mut i8 }
#[repr(C)] pub struct stratix10_svc_command_config_type { pub flags: u32 }
#[repr(C)] pub struct stratix10_svc_client_msg { pub command: u32, pub payload: *mut core::ffi::c_void, pub payload_length: u32 }
#[repr(C)] pub struct fpga_manager_ops { pub write_init: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info, *const i8, usize) -> i32>, pub write: Option<unsafe extern "C" fn(*mut fpga_manager, *const i8, usize) -> i32>, pub write_complete: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info) -> i32> }
#[repr(C)] pub struct of_device_id { pub compatible: *const i8 }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)>, pub name: *const i8, pub of_match_table: *const of_device_id }

const FPGA_MGR_PARTIAL_RECONFIG: u32 = 1;
const COMMAND_RECONFIG: u32 = 0;
const COMMAND_RECONFIG_DATA_SUBMIT: u32 = 1;
const COMMAND_RECONFIG_DATA_CLAIM: u32 = 2;
const COMMAND_RECONFIG_STATUS: u32 = 3;
const COMMAND_RECONFIG_FLAG_PARTIAL: u32 = 1;
const SVC_STATUS_OK: usize = 0;
const SVC_STATUS_ERROR: usize = 1;
const SVC_STATUS_BUFFER_DONE: u32 = 2;
const SVC_STATUS_BUFFER_SUBMITTED: usize = 3;
const SVC_STATUS_COMPLETED: usize = 4;
const SVC_CLIENT_FPGA: u32 = 0;
const SVC_RECONFIG_BUFFER_TIMEOUT_MS: u32 = 1000;
const SVC_RECONFIG_REQUEST_TIMEOUT_MS: u32 = 1000;

extern "C" {
    fn init_completion(c: *mut completion);
    fn reinit_completion(c: *mut completion);
    fn wait_for_completion_timeout(c: *mut completion, timeout: u64) -> u64;
    fn complete(c: *mut completion);
    fn test_and_set_bit_lock(bit: usize, addr: *mut usize) -> bool;
    fn clear_bit_unlock(bit: usize, addr: *mut usize);
    fn test_and_clear_bit(bit: usize, addr: *mut usize) -> bool;
    fn set_bit(bit: usize, addr: *mut usize);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn fpga_mgr_register(dev: *mut device, name: *const i8, ops: *const fpga_manager_ops, priv_: *mut s10_priv) -> *mut fpga_manager;
    fn fpga_mgr_unregister(mgr: *mut fpga_manager);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut fpga_manager);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut fpga_manager;
    fn of_find_node_by_name(parent: *mut device_node, name: *const i8) -> *mut device_node;
    fn of_node_get(node: *mut device_node);
    fn of_node_put(node: *mut device_node);
    fn of_find_matching_node(from: *mut device_node, matches: *const of_device_id) -> *mut device_node;
    fn of_platform_populate(node: *mut device_node, matches: *const of_device_id, data: *mut core::ffi::c_void, parent: *mut device) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

unsafe fn s10_svc_send_msg(priv_: *mut s10_priv, command: u32, payload: *mut core::ffi::c_void, payload_length: u32) -> i32 {
    let mut msg = stratix10_svc_client_msg { command, payload, payload_length };
    stratix10_svc_send((*priv_).chan, &mut msg)
}

unsafe fn s10_free_buffers(mgr: *mut fpga_manager) -> bool {
    let priv_ = (*mgr).priv_;
    let mut num_free = 0;
    for i in 0..NUM_SVC_BUFS {
        if (*priv_).svc_bufs[i].buf.is_null() { num_free += 1; continue; }
        if !test_and_set_bit_lock(SVC_BUF_LOCK, &mut (*priv_).svc_bufs[i].lock) {
            stratix10_svc_free_memory((*priv_).chan, (*priv_).svc_bufs[i].buf);
            (*priv_).svc_bufs[i].buf = core::ptr::null_mut();
            num_free += 1;
        }
    }
    num_free == NUM_SVC_BUFS
}

unsafe fn s10_free_buffer_count(mgr: *mut fpga_manager) -> usize {
    let priv_ = (*mgr).priv_;
    (0..NUM_SVC_BUFS).filter(|&i| (*priv_).svc_bufs[i].buf.is_null()).count()
}

unsafe extern "C" fn s10_unlock_bufs(priv_: *mut s10_priv, kaddr: *mut i8) {
    if kaddr.is_null() { return; }
    for i in 0..NUM_SVC_BUFS { if (*priv_).svc_bufs[i].buf == kaddr { clear_bit_unlock(SVC_BUF_LOCK, &mut (*priv_).svc_bufs[i].lock); return; } }
}

unsafe extern "C" fn s10_receive_callback(client: *mut stratix10_svc_client, data: *mut stratix10_svc_cb_data) {
    let priv_ = (*client).priv_;
    if data.is_null() { return; }
    let status = (*data).status;
    for i in 0..=SVC_STATUS_ERROR { if status & (1 << i) != 0 { set_bit(i, &mut (*priv_).status); } }
    if status & (1 << SVC_STATUS_BUFFER_DONE) != 0 { s10_unlock_bufs(priv_, (*data).kaddr1); s10_unlock_bufs(priv_, (*data).kaddr2); s10_unlock_bufs(priv_, (*data).kaddr3); }
    complete(&mut (*priv_).status_return_completion);
}

unsafe extern "C" fn s10_ops_write_init(mgr: *mut fpga_manager, info: *mut fpga_image_info, _buf: *const i8, _count: usize) -> i32 {
    let p = (*mgr).priv_;
    let mut ctype = stratix10_svc_command_config_type { flags: 0 };
    if (*info).flags & FPGA_MGR_PARTIAL_RECONFIG != 0 { ctype.flags |= COMMAND_RECONFIG_FLAG_PARTIAL; }
    reinit_completion(&mut (*p).status_return_completion);
    let mut ret = s10_svc_send_msg(p, COMMAND_RECONFIG, &mut ctype as *mut _ as *mut _, core::mem::size_of_val(&ctype) as u32);
    if ret < 0 || wait_for_completion_timeout(&mut (*p).status_return_completion, S10_RECONFIG_TIMEOUT) == 0 || !test_and_clear_bit(SVC_STATUS_OK, &mut (*p).status) { stratix10_svc_done((*p).chan); return if ret < 0 { ret } else { -110 }; }
    for i in 0..NUM_SVC_BUFS { let b = stratix10_svc_allocate_memory((*p).chan, SVC_BUF_SIZE); if b.is_null() { s10_free_buffers(mgr); stratix10_svc_done((*p).chan); return -12; } (*p).svc_bufs[i].buf = b; (*p).svc_bufs[i].lock = 0; }
    ret = 0; ret
}

unsafe fn s10_send_buf(mgr: *mut fpga_manager, buf: *const i8, count: usize) -> i32 {
    let p = (*mgr).priv_;
    let mut i = 0; while i < NUM_SVC_BUFS && test_and_set_bit_lock(SVC_BUF_LOCK, &mut (*p).svc_bufs[i].lock) { i += 1; }
    if i == NUM_SVC_BUFS { return -105; }
    let n = core::cmp::min(count, SVC_BUF_SIZE); let dst = (*p).svc_bufs[i].buf;
    memcpy(dst as *mut _, buf as *const _, n);
    let ret = s10_svc_send_msg(p, COMMAND_RECONFIG_DATA_SUBMIT, dst as *mut _, n as u32);
    if ret < 0 { clear_bit_unlock(SVC_BUF_LOCK, &mut (*p).svc_bufs[i].lock); return ret; } n as i32
}

unsafe extern "C" fn s10_ops_write(mgr: *mut fpga_manager, mut buf: *const i8, mut count: usize) -> i32 {
    let p = (*mgr).priv_; let mut ret = 0;
    while count > 0 || s10_free_buffer_count(mgr) != NUM_SVC_BUFS {
        reinit_completion(&mut (*p).status_return_completion);
        if count > 0 { let sent = s10_send_buf(mgr, buf, count); if sent < 0 { continue; } count -= sent as usize; buf = buf.add(sent as usize); }
        else { if s10_free_buffers(mgr) { return 0; } ret = s10_svc_send_msg(p, COMMAND_RECONFIG_DATA_CLAIM, core::ptr::null_mut(), 0); if ret < 0 { break; } }
        let mut waited = 1; if (*p).status == 0 { waited = wait_for_completion_timeout(&mut (*p).status_return_completion, S10_BUFFER_TIMEOUT); }
        if test_and_clear_bit(SVC_STATUS_BUFFER_DONE as usize, &mut (*p).status) || test_and_clear_bit(SVC_STATUS_BUFFER_SUBMITTED, &mut (*p).status) { ret = 0; continue; }
        if test_and_clear_bit(SVC_STATUS_ERROR, &mut (*p).status) { ret = -14; break; }
        if waited == 0 { ret = -110; break; }
    }
    if !s10_free_buffers(mgr) { }
    if ret < 0 { stratix10_svc_done((*p).chan); } ret
}

unsafe extern "C" fn s10_ops_write_complete(mgr: *mut fpga_manager, info: *mut fpga_image_info) -> i32 {
    let p = (*mgr).priv_; let mut timeout = (*info).config_complete_timeout_us; let mut ret;
    loop { reinit_completion(&mut (*p).status_return_completion); ret = s10_svc_send_msg(p, COMMAND_RECONFIG_STATUS, core::ptr::null_mut(), 0); if ret < 0 { break; } ret = wait_for_completion_timeout(&mut (*p).status_return_completion, timeout) as i32; if ret == 0 { ret = -110; break; } timeout = ret as u64; if test_and_clear_bit(SVC_STATUS_COMPLETED, &mut (*p).status) { ret = 0; break; } if test_and_clear_bit(SVC_STATUS_ERROR, &mut (*p).status) { ret = -14; break; } }
    stratix10_svc_done((*p).chan); ret
}

pub static S10_OPS: fpga_manager_ops = fpga_manager_ops { write_init: Some(s10_ops_write_init), write: Some(s10_ops_write), write_complete: Some(s10_ops_write_complete) };

unsafe extern "C" fn s10_probe(_pdev: *mut platform_device) -> i32 { -12 }
unsafe extern "C" fn s10_remove(_pdev: *mut platform_device) {}

pub static S10_OF_MATCH: [of_device_id; 3] = [
    of_device_id { compatible: b"intel,stratix10-soc-fpga-mgr\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"intel,agilex-soc-fpga-mgr\0".as_ptr() as *const i8 },
    of_device_id { compatible: core::ptr::null() },
];

pub static mut S10_DRIVER: platform_driver = platform_driver {
    probe: Some(s10_probe), remove: Some(s10_remove),
    name: b"Stratix10 SoC FPGA manager\0".as_ptr() as *const i8,
    of_match_table: S10_OF_MATCH.as_ptr(),
};

pub unsafe extern "C" fn s10_init() -> i32 {
    let fw_np = of_find_node_by_name(core::ptr::null_mut(), b"svc\0".as_ptr() as *const i8);
    if fw_np.is_null() { return -19; }
    of_node_get(fw_np);
    let np = of_find_matching_node(fw_np, S10_OF_MATCH.as_ptr());
    if np.is_null() { of_node_put(fw_np); return -19; }
    of_node_put(np);
    let ret = of_platform_populate(fw_np, S10_OF_MATCH.as_ptr(), core::ptr::null_mut(), core::ptr::null_mut());
    of_node_put(fw_np);
    if ret != 0 { return ret; }
    platform_driver_register(&mut S10_DRIVER)
}

pub unsafe extern "C" fn s10_exit() { platform_driver_unregister(&mut S10_DRIVER); }

// module_init(s10_init); module_exit(s10_exit);
// MODULE_DEVICE_TABLE(of, s10_of_match);
// MODULE_AUTHOR("Alan Tull <atull@kernel.org>");
// MODULE_DESCRIPTION("Intel Stratix 10 SOC FPGA Manager");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
