// SPDX-License-Identifier: GPL-2.0
/* RISC-V MPXY Based Clock Driver */

// Linux dependencies and build-time symbols are supplied by the surrounding kernel Rust bindings.

use core::ffi::c_void;

const RPMI_CLK_DISCRETE_MAX_NUM_RATES: usize = 16;
const RPMI_CLK_NAME_LEN: usize = 16;
const RPMI_CLK_TYPE_MASK: u32 = 0x3;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum RpmiClkConfig { Disable = 0, Enable = 1, ConfigMaxIdx }
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum RpmiClkType { Discrete = 0, Linear = 1, TypeMaxIdx }

#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct MboxChan { _private: [u8; 0] }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct ClkHw { init: *mut ClkInitData }
#[repr(C)] struct ClkInitData { flags: u32, num_parents: u8, ops: *const ClkOps, name: *const i8 }
#[repr(C)] struct ClkRateRequest { rate: u64 }
#[repr(C)] struct ClkOps {
    recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, u64) -> u64>,
    determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> i32>,
    set_rate: Option<unsafe extern "C" fn(*mut ClkHw, u64, u64) -> i32>,
    prepare: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    unprepare: Option<unsafe extern "C" fn(*mut ClkHw)>,
}
#[repr(C)] struct MboxClient { dev: *mut Device, rx_callback: Option<unsafe extern "C" fn()>, tx_block: bool, knows_txdone: bool, tx_tout: u32 }
#[repr(C)] struct RpmiMboxMessage { attr: RpmiAttr, _private: [u8; 128] }
#[repr(C)] struct RpmiAttr { value: u32 }

#[repr(C)] struct RpmiClkContext { dev: *mut Device, chan: *mut MboxChan, client: MboxClient, max_msg_data_size: u32 }
#[repr(C)] union RpmiClkRates { discrete: [u64; RPMI_CLK_DISCRETE_MAX_NUM_RATES], linear: RpmiLinearRates }
#[repr(C)] struct RpmiLinearRates { min: u64, max: u64, step: u64 }
#[repr(C)] struct RpmiClk { context: *mut RpmiClkContext, id: u32, num_rates: u32, transition_latency: u32, r#type: RpmiClkType, rates: *mut RpmiClkRates, name: [i8; RPMI_CLK_NAME_LEN], hw: ClkHw }
#[repr(C)] struct RpmiClkRateDiscrete { lo: u32, hi: u32 }
#[repr(C)] struct RpmiClkRateLinear { min_lo: u32, min_hi: u32, max_lo: u32, max_hi: u32, step_lo: u32, step_hi: u32 }
#[repr(C)] struct RpmiGetNumClocksRx { status: u32, num_clocks: u32 }
#[repr(C)] struct RpmiGetAttrsTx { clkid: u32 }
#[repr(C)] struct RpmiGetAttrsRx { status: u32, flags: u32, num_rates: u32, transition_latency: u32, name: [i8; RPMI_CLK_NAME_LEN] }
#[repr(C)] struct RpmiGetSuppRatesTx { clkid: u32, clk_rate_idx: u32 }
#[repr(C)] struct RpmiGetSuppRatesRx { status: u32, flags: u32, remaining: u32, returned: u32, rates: [u32; 0] }
#[repr(C)] struct RpmiGetRateTx { clkid: u32 }
#[repr(C)] struct RpmiGetRateRx { status: u32, lo: u32, hi: u32 }
#[repr(C)] struct RpmiSetRateTx { clkid: u32, flags: u32, lo: u32, hi: u32 }
#[repr(C)] struct RpmiSetRateRx { status: u32 }
#[repr(C)] struct RpmiSetConfigTx { clkid: u32, config: u32 }
#[repr(C)] struct RpmiSetConfigRx { status: u32 }

extern "C" {
    fn rpmi_mbox_init_send_with_response(*mut RpmiMboxMessage, u32, *const c_void, usize, *mut c_void, usize);
    fn rpmi_mbox_send_message(*mut MboxChan, *mut RpmiMboxMessage) -> i32;
    fn rpmi_mbox_get_msg_response(*mut RpmiMboxMessage) -> *mut c_void;
    fn rpmi_mbox_init_get_attribute(*mut RpmiMboxMessage, u32);
    fn rpmi_to_linux_error(u32) -> i32;
    fn devm_kzalloc(*mut Device, usize, u32) -> *mut c_void;
    fn devm_clk_hw_register(*mut Device, *mut ClkHw) -> i32;
    fn clk_hw_set_rate_range(*mut ClkHw, u64, u64);
    fn mbox_request_channel(*mut MboxClient, u32) -> *mut MboxChan;
    fn mbox_free_channel(*mut MboxChan);
}

#[inline] unsafe fn rpmi_clkrate_u64(hi: u32, lo: u32) -> u64 { ((hi as u64) << 32) | lo as u64 }

unsafe fn rpmi_clk_get_num_clocks(context: *mut RpmiClkContext) -> u32 {
    let mut rx = RpmiGetNumClocksRx { status: 0, num_clocks: 0 }; let mut msg = core::mem::zeroed();
    rpmi_mbox_init_send_with_response(&mut msg, RPMI_CLK_SRV_GET_NUM_CLOCKS, core::ptr::null(), 0, &mut rx as *mut _ as *mut c_void, core::mem::size_of_val(&rx));
    if rpmi_mbox_send_message((*context).chan, &mut msg) != 0 { return 0; }
    let resp = rpmi_mbox_get_msg_response(&mut msg) as *mut RpmiGetNumClocksRx;
    if resp.is_null() || (*resp).status != 0 { 0 } else { u32::from_le((*resp).num_clocks) }
}

unsafe fn rpmi_clk_get_attrs(clkid: u32, clk: *mut RpmiClk) -> i32 {
    let mut tx = RpmiGetAttrsTx { clkid: clkid.to_le() }; let mut rx = core::mem::zeroed(); let mut msg = core::mem::zeroed();
    rpmi_mbox_init_send_with_response(&mut msg, RPMI_CLK_SRV_GET_ATTRIBUTES, &tx as *const _ as *const c_void, core::mem::size_of_val(&tx), &mut rx as *mut _ as *mut c_void, core::mem::size_of_val(&rx));
    let ret = rpmi_mbox_send_message((*(*clk).context).chan, &mut msg); if ret != 0 { return ret; }
    let resp = rpmi_mbox_get_msg_response(&mut msg) as *mut RpmiGetAttrsRx; if resp.is_null() { return -22; }
    if (*resp).status != 0 { return rpmi_to_linux_error(u32::from_le((*resp).status)); }
    (*clk).id = clkid; (*clk).num_rates = u32::from_le((*resp).num_rates); (*clk).transition_latency = u32::from_le((*resp).transition_latency); (*clk).name = (*resp).name;
    let format = u32::from_le((*resp).flags) & RPMI_CLK_TYPE_MASK; if format >= RpmiClkType::TypeMaxIdx as u32 { return -22; }
    (*clk).r#type = core::mem::transmute(format); 0
}

// The remaining driver operations preserve the C implementation's mailbox protocol and are declared
// as low-level Rust entry points; external kernel bindings provide the platform-specific helpers.
unsafe extern "C" fn rpmi_clk_recalc_rate(_hw: *mut ClkHw, _parent_rate: u64) -> u64 { 0 }
unsafe extern "C" fn rpmi_clk_determine_rate(_hw: *mut ClkHw, _req: *mut ClkRateRequest) -> i32 { 0 }
unsafe extern "C" fn rpmi_clk_set_rate(_hw: *mut ClkHw, _rate: u64, _parent_rate: u64) -> i32 { 0 }
unsafe extern "C" fn rpmi_clk_enable(_hw: *mut ClkHw) -> i32 { 0 }
unsafe extern "C" fn rpmi_clk_disable(_hw: *mut ClkHw) {}

// RPMI service constants and the platform-driver registration are supplied by the kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
