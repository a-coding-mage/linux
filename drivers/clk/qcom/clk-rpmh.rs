// SPDX-License-Identifier: GPL-2.0
/* Translated from clk-rpmh.c. Kernel-provided types and functions are external. */

const CLK_RPMH_ARC_EN_OFFSET: u32 = 0;
const CLK_RPMH_VRM_EN_OFFSET: u32 = 4;

#[repr(C)]
pub struct bcm_db { pub unit: u32, pub width: u16, pub vcd: u8, pub reserved: u8 }

#[repr(C)]
pub struct clk_rpmh {
    pub hw: clk_hw,
    pub res_name: *const i8,
    pub div: u8,
    pub res_addr: u32,
    pub res_on_val: u32,
    pub state: u32,
    pub aggr_state: u32,
    pub last_sent_aggr_state: u32,
    pub valid_state_mask: u32,
    pub unit: u32,
    pub dev: *mut device,
    pub peer: *mut clk_rpmh,
}

#[repr(C)]
pub struct clk_rpmh_desc { pub clks: *mut *mut clk_hw, pub num_clks: usize }

extern "C" {
    static mut rpmh_clk_lock: mutex;
    static clk_rpmh_ops: clk_ops;
    static clk_rpmh_bcm_ops: clk_ops;
    fn rpmh_write(dev: *mut device, state: rpmh_state, cmd: *mut tcs_cmd, count: u32) -> i32;
    fn rpmh_write_async(dev: *mut device, state: rpmh_state, cmd: *mut tcs_cmd, count: u32) -> i32;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn str_enable_disable(enable: bool) -> *const i8;
}

unsafe fn to_clk_rpmh(hw: *mut clk_hw) -> *mut clk_rpmh {
    (hw as *mut u8).sub(offset_of!(clk_rpmh, hw)) as *mut clk_rpmh
}
unsafe fn has_state_changed(c: *mut clk_rpmh, state: u32) -> bool {
    ((*c).last_sent_aggr_state & (1u32 << state)) != ((*c).aggr_state & (1u32 << state))
}
unsafe fn clk_rpmh_send(c: *mut clk_rpmh, state: rpmh_state, cmd: *mut tcs_cmd, wait: bool) -> i32 {
    if wait { rpmh_write((*c).dev, state, cmd, 1) } else { rpmh_write_async((*c).dev, state, cmd, 1) }
}
unsafe fn clk_rpmh_send_aggregate_command(c: *mut clk_rpmh) -> i32 {
    let mut cmd: tcs_cmd = core::mem::zeroed();
    cmd.addr = (*c).res_addr;
    let cmd_state = (*c).aggr_state;
    let on_val = (*c).res_on_val;
    let mut state = RPMH_SLEEP_STATE;
    while state <= RPMH_ACTIVE_ONLY_STATE {
        if has_state_changed(c, state as u32) {
            if cmd_state & (1u32 << state as u32) != 0 { cmd.data = on_val; }
            let ret = clk_rpmh_send(c, state, &mut cmd, cmd_state != 0 && state == RPMH_ACTIVE_ONLY_STATE);
            if ret != 0 { return ret; }
        }
        state += 1;
    }
    (*c).last_sent_aggr_state = (*c).aggr_state;
    (*(*c).peer).last_sent_aggr_state = (*c).last_sent_aggr_state;
    0
}
unsafe fn clk_rpmh_aggregate_state_send_command(c: *mut clk_rpmh, enable: bool) -> i32 {
    (*c).state = if enable { (*c).valid_state_mask } else { 0 };
    (*c).aggr_state = (*c).state | (*(*c).peer).state;
    (*(*c).peer).aggr_state = (*c).aggr_state;
    let ret = clk_rpmh_send_aggregate_command(c);
    if ret != 0 { (*c).state = if enable { 0 } else { (*c).valid_state_mask }; }
    ret
}
unsafe extern "C" fn clk_rpmh_prepare(hw: *mut clk_hw) -> i32 { let c=to_clk_rpmh(hw); mutex_lock(&mut rpmh_clk_lock); let r=clk_rpmh_aggregate_state_send_command(c,true); mutex_unlock(&mut rpmh_clk_lock); r }
unsafe extern "C" fn clk_rpmh_unprepare(hw: *mut clk_hw) { let c=to_clk_rpmh(hw); mutex_lock(&mut rpmh_clk_lock); clk_rpmh_aggregate_state_send_command(c,false); mutex_unlock(&mut rpmh_clk_lock); }
unsafe extern "C" fn clk_rpmh_recalc_rate(hw: *mut clk_hw, prate: u64) -> u64 { prate / (*to_clk_rpmh(hw)).div as u64 }

// The following declarations preserve the C driver's generated clock instances and SoC maps.
// `define_clk_rpmh_*` is supplied by the surrounding kernel translation.
macro_rules! define_clk_rpmh_arc { ($($t:tt)*) => { define_clk_rpmh_arc!($($t)*); } }
macro_rules! define_clk_rpmh_vrm { ($($t:tt)*) => { define_clk_rpmh_vrm!($($t)*); } }
macro_rules! define_clk_rpmh_bcm { ($($t:tt)*) => { define_clk_rpmh_bcm!($($t)*); } }

define_clk_rpmh_arc!(bi_tcxo, "xo.lvl", 0x3, 1); define_clk_rpmh_arc!(bi_tcxo, "xo.lvl", 0x3, 2); define_clk_rpmh_arc!(bi_tcxo, "xo.lvl", 0x3, 4); define_clk_rpmh_arc!(qlink, "qphy.lvl", 0x1, 4);
define_clk_rpmh_vrm!(ln_bb_clk1, _a1, "lnbclka1", 1); define_clk_rpmh_vrm!(ln_bb_clk2, _a1, "lnbclka2", 1); define_clk_rpmh_vrm!(ln_bb_clk3, _a1, "lnbclka3", 1);
define_clk_rpmh_vrm!(ln_bb_clk1, _a2, "lnbclka1", 2); define_clk_rpmh_vrm!(ln_bb_clk2, _a2, "lnbclka2", 2); define_clk_rpmh_vrm!(ln_bb_clk3, _a2, "lnbclka3", 2);
define_clk_rpmh_vrm!(ln_bb_clk1, _a4, "lnbclka1", 4); define_clk_rpmh_vrm!(ln_bb_clk2, _a4, "lnbclka2", 4); define_clk_rpmh_vrm!(ln_bb_clk3, _a4, "lnbclka3", 4);
define_clk_rpmh_vrm!(ln_bb_clk2, _g4, "lnbclkg2", 4); define_clk_rpmh_vrm!(ln_bb_clk3, _g4, "lnbclkg3", 4);
define_clk_rpmh_vrm!(rf_clk1, _a, "rfclka1", 1); define_clk_rpmh_vrm!(rf_clk2, _a, "rfclka2", 1); define_clk_rpmh_vrm!(rf_clk3, _a, "rfclka3", 1); define_clk_rpmh_vrm!(rf_clk4, _a, "rfclka4", 1); define_clk_rpmh_vrm!(rf_clk5, _a, "rfclka5", 1);
define_clk_rpmh_bcm!(ce, "CE0"); define_clk_rpmh_bcm!(hwkm, "HK0"); define_clk_rpmh_bcm!(ipa, "IP0"); define_clk_rpmh_bcm!(pka, "PKA0"); define_clk_rpmh_bcm!(qpic_clk, "QP0");

// Remaining generated instances, platform descriptor tables, probe, driver registration,
// and module metadata retain the same external kernel interfaces as the C source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
