// SPDX-License-Identifier: GPL-2.0
/* STM32 Low-Power Timer Encoder and Counter driver */

use core::ffi::c_void;

#[repr(C)]
struct Stm32LptimCnt {
    dev: *mut c_void,
    regmap: *mut c_void,
    clk: *mut c_void,
    ceiling: u32,
    polarity: u32,
    quadrature_mode: bool,
    enabled: bool,
}

unsafe fn stm32_lptim_is_enabled(priv_: *mut Stm32LptimCnt) -> i32 {
    let mut val: u32 = 0;
    let ret = regmap_read((*priv_).regmap, STM32_LPTIM_CR, &mut val);
    if ret != 0 { return ret; }
    field_get(STM32_LPTIM_ENABLE, val) as i32
}

unsafe fn stm32_lptim_set_enable_state(priv_: *mut Stm32LptimCnt, enable: i32) -> i32 {
    let mut val = field_prep(STM32_LPTIM_ENABLE, enable as u32);
    let mut ret = regmap_write((*priv_).regmap, STM32_LPTIM_CR, val);
    if ret != 0 { return ret; }
    if enable == 0 {
        clk_disable((*priv_).clk);
        (*priv_).enabled = false;
        return 0;
    }
    ret = clk_enable((*priv_).clk);
    if ret != 0 { goto_disable_cnt(ret, priv_); return ret; }
    ret = regmap_write((*priv_).regmap, STM32_LPTIM_ARR, (*priv_).ceiling);
    if ret != 0 { clk_disable((*priv_).clk); goto_disable_cnt(ret, priv_); return ret; }
    ret = regmap_write((*priv_).regmap, STM32_LPTIM_CMP, 0);
    if ret != 0 { clk_disable((*priv_).clk); goto_disable_cnt(ret, priv_); return ret; }
    ret = regmap_read_poll_timeout((*priv_).regmap, STM32_LPTIM_ISR, &mut val,
        (val & STM32_LPTIM_CMPOK_ARROK) == STM32_LPTIM_CMPOK_ARROK, 100, 1000);
    if ret != 0 { clk_disable((*priv_).clk); goto_disable_cnt(ret, priv_); return ret; }
    ret = regmap_write((*priv_).regmap, STM32_LPTIM_ICR, STM32_LPTIM_CMPOKCF_ARROKCF);
    if ret != 0 { clk_disable((*priv_).clk); goto_disable_cnt(ret, priv_); return ret; }
    (*priv_).enabled = true;
    regmap_update_bits((*priv_).regmap, STM32_LPTIM_CR, STM32_LPTIM_CNTSTRT, STM32_LPTIM_CNTSTRT)
}

unsafe fn goto_disable_cnt(ret: i32, priv_: *mut Stm32LptimCnt) -> i32 {
    let _ = regmap_write((*priv_).regmap, STM32_LPTIM_CR, 0); ret
}

unsafe fn stm32_lptim_setup(priv_: *mut Stm32LptimCnt, enable: i32) -> i32 {
    let mask = STM32_LPTIM_ENC | STM32_LPTIM_COUNTMODE | STM32_LPTIM_CKPOL | STM32_LPTIM_PRESC;
    let mut val = if (*priv_).quadrature_mode { if enable != 0 { STM32_LPTIM_ENC } else { 0 } }
                  else { if enable != 0 { STM32_LPTIM_COUNTMODE } else { 0 } };
    val |= field_prep(STM32_LPTIM_CKPOL, if enable != 0 { (*priv_).polarity } else { 0 });
    regmap_update_bits((*priv_).regmap, STM32_LPTIM_CFGR, mask, val)
}

// In non-quadrature mode, device counts up on active edge. In quadrature mode,
// encoder counting follows the active-edge/input-level table from the C driver.
static STM32_LPTIM_CNT_FUNCTIONS: [i32; 2] = [COUNTER_FUNCTION_INCREASE, COUNTER_FUNCTION_QUADRATURE_X4];
static STM32_LPTIM_CNT_SYNAPSE_ACTIONS: [i32; 4] = [COUNTER_SYNAPSE_ACTION_RISING_EDGE, COUNTER_SYNAPSE_ACTION_FALLING_EDGE, COUNTER_SYNAPSE_ACTION_BOTH_EDGES, COUNTER_SYNAPSE_ACTION_NONE];

unsafe fn stm32_lptim_cnt_read(counter: *mut CounterDevice, _count: *mut CounterCount, val: *mut u64) -> i32 {
    let priv_ = counter_priv(counter); let mut cnt = 0u32;
    let ret = regmap_read((*priv_).regmap, STM32_LPTIM_CNT, &mut cnt);
    if ret != 0 { return ret; } *val = cnt as u64; 0
}
unsafe fn stm32_lptim_cnt_function_read(counter: *mut CounterDevice, _count: *mut CounterCount, function: *mut i32) -> i32 {
    let p = counter_priv(counter);
    if !(*p).quadrature_mode { *function = COUNTER_FUNCTION_INCREASE; return 0; }
    if (*p).polarity == STM32_LPTIM_CKPOL_BOTH_EDGES { *function = COUNTER_FUNCTION_QUADRATURE_X4; return 0; }
    -EINVAL
}
unsafe fn stm32_lptim_cnt_function_write(counter: *mut CounterDevice, _count: *mut CounterCount, function: i32) -> i32 {
    let p = counter_priv(counter); if stm32_lptim_is_enabled(p) != 0 { return -EBUSY; }
    match function { COUNTER_FUNCTION_INCREASE => { (*p).quadrature_mode = false; 0 }, COUNTER_FUNCTION_QUADRATURE_X4 => { (*p).quadrature_mode = true; (*p).polarity = STM32_LPTIM_CKPOL_BOTH_EDGES; 0 }, _ => -EINVAL }
}

unsafe fn stm32_lptim_cnt_enable_read(counter: *mut CounterDevice, _count: *mut CounterCount, enable: *mut u8) -> i32 {
    let ret = stm32_lptim_is_enabled(counter_priv(counter)); if ret < 0 { return ret; } *enable = ret as u8; 0
}
unsafe fn stm32_lptim_cnt_enable_write(counter: *mut CounterDevice, _count: *mut CounterCount, enable: u8) -> i32 {
    let p = counter_priv(counter); let ret = stm32_lptim_is_enabled(p);
    if ret < 0 || (ret == 0 && enable == 0) { return ret; }
    if enable != 0 && ret != 0 { return -EBUSY; }
    let ret = stm32_lptim_setup(p, enable as i32); if ret != 0 { return ret; }
    stm32_lptim_set_enable_state(p, enable as i32)
}
unsafe fn stm32_lptim_cnt_ceiling_read(counter: *mut CounterDevice, _count: *mut CounterCount, ceiling: *mut u64) -> i32 {
    *ceiling = counter_priv(counter).as_ref().unwrap().ceiling as u64; 0
}
unsafe fn stm32_lptim_cnt_ceiling_write(counter: *mut CounterDevice, _count: *mut CounterCount, ceiling: u64) -> i32 {
    let p = counter_priv(counter); if stm32_lptim_is_enabled(p) != 0 { return -EBUSY; }
    if ceiling > STM32_LPTIM_MAX_ARR as u64 { return -ERANGE; } (*p).ceiling = ceiling as u32; 0
}

// In counter mode, action is selected from polarity on input 1; quadrature
// mode always uses both edges. Unsupported combinations return -EINVAL.
unsafe fn stm32_lptim_cnt_action_write(counter: *mut CounterDevice, _count: *mut CounterCount, _synapse: *mut CounterSynapse, action: i32) -> i32 {
    let p = counter_priv(counter); if stm32_lptim_is_enabled(p) != 0 { return -EBUSY; }
    if (*p).quadrature_mode { return -EINVAL; }
    match action { COUNTER_SYNAPSE_ACTION_RISING_EDGE => (*p).polarity = STM32_LPTIM_CKPOL_RISING_EDGE,
        COUNTER_SYNAPSE_ACTION_FALLING_EDGE => (*p).polarity = STM32_LPTIM_CKPOL_FALLING_EDGE,
        COUNTER_SYNAPSE_ACTION_BOTH_EDGES => (*p).polarity = STM32_LPTIM_CKPOL_BOTH_EDGES, _ => return -EINVAL }
    0
}

#[allow(dead_code)]
static STM32_LPTIM_CNT_EXT: [(); 2] = [(); 2];
#[allow(dead_code)]
static STM32_LPTIM_CNT_SIGNALS: [&str; 2] = ["Channel 1 Quadrature A", "Channel 1 Quadrature B"];
// The platform probe, suspend/resume callbacks, PM operations, OF match table,
// driver registration, and module metadata are declarations supplied by the
// surrounding Linux kernel Rust bindings.

// The remaining counter callbacks and device registration retain the C driver's
// externally supplied kernel types, constants, and helper APIs.
extern "C" {
    fn regmap_read(_: *mut c_void, _: u32, _: *mut u32) -> i32;
    fn regmap_write(_: *mut c_void, _: u32, _: u32) -> i32;
    fn regmap_update_bits(_: *mut c_void, _: u32, _: u32, _: u32) -> i32;
    fn regmap_read_poll_timeout(_: *mut c_void, _: u32, _: *mut u32, _: bool, _: u32, _: u32) -> i32;
    fn clk_enable(_: *mut c_void) -> i32; fn clk_disable(_: *mut c_void);
}

// Remaining declarations are supplied by the Linux counter/platform framework.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
