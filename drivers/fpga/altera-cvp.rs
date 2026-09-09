// SPDX-License-Identifier: GPL-2.0-only
/* FPGA Manager Driver for Altera Arria/Cyclone/Stratix CvP */

const CVP_BAR: u32 = 0;
const CVP_DUMMY_WR: u32 = 244;
const TIMEOUT_US: i32 = 2000;
const VSE_CVP_STATUS: i32 = 0x1c;
const VSE_CVP_STATUS_CFG_RDY: u32 = 1 << 18;
const VSE_CVP_STATUS_CFG_ERR: u32 = 1 << 19;
const VSE_CVP_STATUS_CVP_EN: u32 = 1 << 20;
const VSE_CVP_STATUS_USERMODE: u32 = 1 << 21;
const VSE_CVP_STATUS_CFG_DONE: u32 = 1 << 23;
const VSE_CVP_STATUS_PLD_CLK_IN_USE: u32 = 1 << 24;
const VSE_CVP_MODE_CTRL: i32 = 0x20;
const VSE_CVP_MODE_CTRL_CVP_MODE: u32 = 1 << 0;
const VSE_CVP_MODE_CTRL_HIP_CLK_SEL: u32 = 1 << 1;
const VSE_CVP_MODE_CTRL_NUMCLKS_OFF: u32 = 8;
const VSE_CVP_MODE_CTRL_NUMCLKS_MASK: u32 = 0xff << 8;
const VSE_CVP_DATA: i32 = 0x28;
const VSE_CVP_PROG_CTRL: i32 = 0x2c;
const VSE_CVP_PROG_CTRL_CONFIG: u32 = 1;
const VSE_CVP_PROG_CTRL_START_XFER: u32 = 1 << 1;
const VSE_CVP_PROG_CTRL_MASK: u32 = 3;
const VSE_UNCOR_ERR_STATUS: i32 = 0x34;
const VSE_UNCOR_ERR_CVP_CFG_ERR: u32 = 1 << 5;
const V1_VSEC_OFFSET: u32 = 0x200;
const VSE_CVP_TX_CREDITS: i32 = 0x49;
const V2_CREDIT_TIMEOUT_US: u32 = 40000;
const V2_CHECK_CREDIT_US: u32 = 10;
const V2_POLL_TIMEOUT_US: i32 = 1000000;
const V2_USER_TIMEOUT_US: i32 = 500000;
const V1_POLL_TIMEOUT_US: i32 = 10;
const DRV_NAME: &str = "altera-cvp";
const ALTERA_CVP_MGR_NAME: &str = "Altera CvP FPGA Manager";
const ALTERA_CVP_V1_SIZE: usize = 4;
const ALTERA_CVP_V2_SIZE: usize = 4096;

static mut altera_cvp_chkcfg: bool = false;

#[repr(C)]
struct cvp_priv {
    switch_clk: Option<unsafe fn(*mut altera_cvp_conf)>,
    clear_state: Option<unsafe fn(*mut altera_cvp_conf) -> i32>,
    wait_credit: Option<unsafe fn(*mut fpga_manager, u32) -> i32>,
    block_size: usize,
    poll_time_us: i32,
    user_time_us: i32,
}

#[repr(C)]
struct altera_cvp_conf {
    pci_dev: *mut pci_dev,
    map: *mut core::ffi::c_void,
    write_data: Option<unsafe fn(*mut altera_cvp_conf, u32)>,
    mgr_name: [u8; 64],
    numclks: u8,
    sent_packets: u32,
    vsec_offset: u32,
    priv_: *const cvp_priv,
}

#[repr(C)] struct pci_dev { _private: [u8; 0] }
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct device_driver { _private: [u8; 0] }
#[repr(C)] struct fpga_manager { priv_: *mut altera_cvp_conf, dev: device }
#[repr(C)] struct fpga_image_info { flags: u32 }
#[repr(C)] struct pci_device_id { _private: [u8; 0] }

#[allow(non_camel_case_types)] type fpga_mgr_states = i32;
const FPGA_MGR_STATE_UNKNOWN: fpga_mgr_states = 0;
const FPGA_MGR_STATE_OPERATING: fpga_mgr_states = 1;
const FPGA_MGR_STATE_POWER_UP: fpga_mgr_states = 2;
const FPGA_MGR_PARTIAL_RECONFIG: u32 = 1 << 0;
const FPGA_MGR_COMPRESSED_BITSTREAM: u32 = 1 << 1;
const FPGA_MGR_ENCRYPTED_BITSTREAM: u32 = 1 << 2;

extern "C" {
    fn pci_read_config_byte(_: *mut pci_dev, _: u32, _: *mut u8) -> i32;
    fn pci_read_config_dword(_: *mut pci_dev, _: u32, _: *mut u32) -> i32;
    fn pci_write_config_dword(_: *mut pci_dev, _: u32, _: u32) -> i32;
    fn readl(_: *mut core::ffi::c_void) -> u32;
    fn writel(_: u32, _: *mut core::ffi::c_void);
    fn usleep_range(_: u32, _: u32);
    fn memcpy(_: *mut core::ffi::c_void, _: *const core::ffi::c_void, _: usize) -> *mut core::ffi::c_void;
    fn dev_err(_: *mut device, _: *const u8, ...);
}

unsafe fn altera_read_config_byte(c: *mut altera_cvp_conf, w: i32, v: *mut u8) -> i32 { pci_read_config_byte((*c).pci_dev, (*c).vsec_offset + w as u32, v) }
unsafe fn altera_read_config_dword(c: *mut altera_cvp_conf, w: i32, v: *mut u32) -> i32 { pci_read_config_dword((*c).pci_dev, (*c).vsec_offset + w as u32, v) }
unsafe fn altera_write_config_dword(c: *mut altera_cvp_conf, w: i32, v: u32) -> i32 { pci_write_config_dword((*c).pci_dev, (*c).vsec_offset + w as u32, v) }

unsafe fn altera_cvp_state(mgr: *mut fpga_manager) -> fpga_mgr_states {
    let c = (*mgr).priv_; let mut status = 0; altera_read_config_dword(c, VSE_CVP_STATUS, &mut status);
    if status & VSE_CVP_STATUS_CFG_DONE != 0 { FPGA_MGR_STATE_OPERATING } else if status & VSE_CVP_STATUS_CVP_EN != 0 { FPGA_MGR_STATE_POWER_UP } else { FPGA_MGR_STATE_UNKNOWN }
}
unsafe fn altera_cvp_write_data_iomem(c: *mut altera_cvp_conf, v: u32) { writel(v, (*c).map); }
unsafe fn altera_cvp_write_data_config(c: *mut altera_cvp_conf, v: u32) { pci_write_config_dword((*c).pci_dev, (*c).vsec_offset + VSE_CVP_DATA as u32, v); }

unsafe fn altera_cvp_dummy_write(c: *mut altera_cvp_conf) {
    let mut v = 0; altera_read_config_dword(c, VSE_CVP_MODE_CTRL, &mut v); v &= !VSE_CVP_MODE_CTRL_NUMCLKS_MASK; v |= 1 << VSE_CVP_MODE_CTRL_NUMCLKS_OFF; altera_write_config_dword(c, VSE_CVP_MODE_CTRL, v);
    for _ in 0..CVP_DUMMY_WR { ((*c).write_data.unwrap())(c, 0); }
}
unsafe fn altera_cvp_wait_status(c: *mut altera_cvp_conf, mask: u32, expected: u32, timeout: i32) -> i32 {
    let mut retries = (timeout / 10) + if timeout % 10 != 0 { 1 } else { 0 }; let mut v = 0;
    loop { altera_read_config_dword(c, VSE_CVP_STATUS, &mut v); if v & mask == expected { return 0; } usleep_range(10, 11); retries -= 1; if retries == 0 { break; } } -110
}
unsafe fn altera_cvp_chk_error(mgr: *mut fpga_manager, bytes: usize) -> i32 {
    let c = (*mgr).priv_; let mut v = 0; let ret = altera_read_config_dword(c, VSE_CVP_STATUS, &mut v); if ret != 0 || v & VSE_CVP_STATUS_CFG_ERR != 0 { return -71; } 0
}
unsafe fn altera_cvp_v2_clear_state(c: *mut altera_cvp_conf) -> i32 {
    let mut v = 0; let r = altera_read_config_dword(c, VSE_CVP_PROG_CTRL, &mut v); if r != 0 { return r; } v &= !VSE_CVP_PROG_CTRL_MASK; let r = altera_write_config_dword(c, VSE_CVP_PROG_CTRL, v); if r != 0 { return r; } altera_cvp_wait_status(c, VSE_CVP_STATUS_CFG_RDY, 0, (*(*c).priv_).poll_time_us)
}
unsafe fn altera_cvp_v2_wait_for_credit(mgr: *mut fpga_manager, blocks: u32) -> i32 {
    let c = (*mgr).priv_; let mut timeout = V2_CREDIT_TIMEOUT_US / V2_CHECK_CREDIT_US; let mut v = 0u8;
    loop { let r = altera_read_config_byte(c, VSE_CVP_TX_CREDITS, &mut v); if r != 0 { return r; } if v.wrapping_sub((*c).sent_packets as u8) != 0 { return 0; } if altera_cvp_chk_error(mgr, blocks as usize * ALTERA_CVP_V2_SIZE) != 0 { return -11; } usleep_range(V2_CHECK_CREDIT_US, V2_CHECK_CREDIT_US + 1); timeout -= 1; if timeout == 0 { return -110; } }
}
unsafe fn altera_cvp_send_block(c: *mut altera_cvp_conf, data: *const u32, len: usize) -> i32 {
    let words = len / 4; for i in 0..words { ((*c).write_data.unwrap())(c, *data.add(i)); } let rem = len % 4; if rem != 0 { let mut word = 0u32; memcpy(&mut word as *mut _ as *mut _, data.add(words) as *const _, rem); ((*c).write_data.unwrap())(c, word); } 0
}

unsafe fn altera_cvp_teardown(mgr: *mut fpga_manager, _info: *mut fpga_image_info) -> i32 {
    let c = (*mgr).priv_; let mut v = 0; altera_read_config_dword(c, VSE_CVP_PROG_CTRL, &mut v); v &= !VSE_CVP_PROG_CTRL_START_XFER; altera_write_config_dword(c, VSE_CVP_PROG_CTRL, v); v &= !VSE_CVP_PROG_CTRL_CONFIG; altera_write_config_dword(c, VSE_CVP_PROG_CTRL, v);
    if let Some(f) = (*(*c).priv_).switch_clk { f(c); } altera_cvp_wait_status(c, VSE_CVP_STATUS_CFG_RDY, 0, (*(*c).priv_).poll_time_us)
}
unsafe fn altera_cvp_write_init(mgr: *mut fpga_manager, info: *mut fpga_image_info, _buf: *const u8, _count: usize) -> i32 {
    let c = (*mgr).priv_; let flags = if info.is_null() { 0 } else { (*info).flags }; if flags & FPGA_MGR_PARTIAL_RECONFIG != 0 { return -22; }
    (*c).numclks = if flags & FPGA_MGR_COMPRESSED_BITSTREAM != 0 { 8 } else if flags & FPGA_MGR_ENCRYPTED_BITSTREAM != 0 { 4 } else { 1 }; let mut v = 0; altera_read_config_dword(c, VSE_CVP_STATUS, &mut v); if v & VSE_CVP_STATUS_CVP_EN == 0 { return -19; }
    if v & VSE_CVP_STATUS_CFG_RDY != 0 { let r = altera_cvp_teardown(mgr, info); if r != 0 { return r; } }
    altera_read_config_dword(c, VSE_CVP_MODE_CTRL, &mut v); v |= VSE_CVP_MODE_CTRL_HIP_CLK_SEL; altera_write_config_dword(c, VSE_CVP_MODE_CTRL, v); altera_read_config_dword(c, VSE_CVP_MODE_CTRL, &mut v); v |= VSE_CVP_MODE_CTRL_CVP_MODE; altera_write_config_dword(c, VSE_CVP_MODE_CTRL, v);
    if let Some(f) = (*(*c).priv_).switch_clk { f(c); } if let Some(f) = (*(*c).priv_).clear_state { let r = f(c); if r != 0 { return r; } } (*c).sent_packets = 0; altera_read_config_dword(c, VSE_CVP_PROG_CTRL, &mut v); v |= VSE_CVP_PROG_CTRL_CONFIG; altera_write_config_dword(c, VSE_CVP_PROG_CTRL, v); let r = altera_cvp_wait_status(c, VSE_CVP_STATUS_CFG_RDY, VSE_CVP_STATUS_CFG_RDY, (*(*c).priv_).poll_time_us); if r != 0 { return r; }
    if let Some(f) = (*(*c).priv_).switch_clk { f(c); } altera_read_config_dword(c, VSE_CVP_PROG_CTRL, &mut v); v |= VSE_CVP_PROG_CTRL_START_XFER; altera_write_config_dword(c, VSE_CVP_PROG_CTRL, v); if (*(*c).priv_).switch_clk.is_some() { altera_read_config_dword(c, VSE_CVP_MODE_CTRL, &mut v); v = (v & !VSE_CVP_MODE_CTRL_NUMCLKS_MASK) | ((*c).numclks as u32) << VSE_CVP_MODE_CTRL_NUMCLKS_OFF; altera_write_config_dword(c, VSE_CVP_MODE_CTRL, v); } 0
}
unsafe fn altera_cvp_write(mgr: *mut fpga_manager, buf: *const u8, count: usize) -> i32 { let c = (*mgr).priv_; let mut done = 0; while done < count { if let Some(f) = (*(*c).priv_).wait_credit { let r = f(mgr, done as u32); if r != 0 { return r; } } let len = core::cmp::min((*(*c).priv_).block_size, count - done); altera_cvp_send_block(c, buf.add(done) as *const u32, len); done += len; (*c).sent_packets += 1; } 0 }
unsafe fn altera_cvp_write_complete(mgr: *mut fpga_manager, info: *mut fpga_image_info) -> i32 { let c = (*mgr).priv_; let r = altera_cvp_teardown(mgr, info); if r != 0 { return r; } let mut v = 0; altera_read_config_dword(c, VSE_UNCOR_ERR_STATUS, &mut v); if v & VSE_UNCOR_ERR_CVP_CFG_ERR != 0 { return -71; } altera_read_config_dword(c, VSE_CVP_MODE_CTRL, &mut v); v &= !(VSE_CVP_MODE_CTRL_HIP_CLK_SEL | VSE_CVP_MODE_CTRL_CVP_MODE); altera_write_config_dword(c, VSE_CVP_MODE_CTRL, v); altera_cvp_wait_status(c, VSE_CVP_STATUS_PLD_CLK_IN_USE | VSE_CVP_STATUS_USERMODE, VSE_CVP_STATUS_PLD_CLK_IN_USE | VSE_CVP_STATUS_USERMODE, (*(*c).priv_).user_time_us) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
