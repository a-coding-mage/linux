// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Microchip
 *
 * Author: Kamel Bouhara <kamel.bouhara@bootlin.com>
 */
// External Linux kernel headers and symbols are supplied by other dependencies.

const ATMEL_TC_CMR_MASK: u32 = ATMEL_TC_LDRA_RISING | ATMEL_TC_LDRB_FALLING |
    ATMEL_TC_ETRGEDG_RISING | ATMEL_TC_LDBDIS | ATMEL_TC_LDBSTOP;
const ATMEL_TC_DEF_IRQS: u32 = ATMEL_TC_ETRGS | ATMEL_TC_COVFS |
    ATMEL_TC_LDRAS | ATMEL_TC_LDRBS | ATMEL_TC_CPCS;
const ATMEL_TC_QDEN: u32 = BIT(8);
const ATMEL_TC_POSEN: u32 = BIT(9);

#[repr(C)]
struct MchpTcData {
    tc_cfg: *const AtmelTcbConfig,
    regmap: *mut Regmap,
    qdec_mode: i32,
    num_channels: i32,
    channel: [i32; 2],
}

static MCHP_TC_COUNT_FUNCTIONS: [CounterFunction; 2] = [
    COUNTER_FUNCTION_INCREASE,
    COUNTER_FUNCTION_QUADRATURE_X4,
];

static MCHP_TC_SYNAPSE_ACTIONS: [CounterSynapseAction; 4] = [
    COUNTER_SYNAPSE_ACTION_NONE,
    COUNTER_SYNAPSE_ACTION_RISING_EDGE,
    COUNTER_SYNAPSE_ACTION_FALLING_EDGE,
    COUNTER_SYNAPSE_ACTION_BOTH_EDGES,
];

static mut MCHP_TC_COUNT_SIGNALS: [CounterSignal; 2] = [
    CounterSignal { id: 0, name: b"Channel A\0".as_ptr() as *const i8 },
    CounterSignal { id: 1, name: b"Channel B\0".as_ptr() as *const i8 },
];

static mut MCHP_TC_COUNT_SYNAPSES: [CounterSynapse; 2] = [
    CounterSynapse {
        actions_list: MCHP_TC_SYNAPSE_ACTIONS.as_ptr(),
        num_actions: MCHP_TC_SYNAPSE_ACTIONS.len(),
        signal: unsafe { &MCHP_TC_COUNT_SIGNALS[0] as *const _ },
    },
    CounterSynapse {
        actions_list: MCHP_TC_SYNAPSE_ACTIONS.as_ptr(),
        num_actions: MCHP_TC_SYNAPSE_ACTIONS.len(),
        signal: unsafe { &MCHP_TC_COUNT_SIGNALS[1] as *const _ },
    },
];

unsafe fn mchp_tc_count_function_read(counter: *mut CounterDevice, _count: *mut CounterCount, function: *mut CounterFunction) -> i32 {
    let priv_ = counter_priv(counter);
    *function = if (*priv_).qdec_mode != 0 { COUNTER_FUNCTION_QUADRATURE_X4 } else { COUNTER_FUNCTION_INCREASE };
    0
}

unsafe fn mchp_tc_count_function_write(counter: *mut CounterDevice, _count: *mut CounterCount, function: CounterFunction) -> i32 {
    let priv_ = counter_priv(counter);
    let mut bmr = 0u32;
    let mut cmr = 0u32;
    regmap_read((*priv_).regmap, ATMEL_TC_BMR, &mut bmr);
    regmap_read((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], CMR), &mut cmr);
    cmr &= !ATMEL_TC_WAVE;
    match function {
        COUNTER_FUNCTION_INCREASE => {
            (*priv_).qdec_mode = 0;
            bmr &= !(ATMEL_TC_QDEN | ATMEL_TC_POSEN);
            cmr |= if (*(*priv_).tc_cfg).has_gclk { ATMEL_TC_TIMER_CLOCK1 } else { ATMEL_TC_TIMER_CLOCK2 };
            cmr |= ATMEL_TC_CMR_MASK;
            cmr &= !(ATMEL_TC_ABETRG | ATMEL_TC_XC0);
        }
        COUNTER_FUNCTION_QUADRATURE_X4 => {
            if !(*(*priv_).tc_cfg).has_qdec { return -EINVAL; }
            if (*priv_).num_channels < 2 || (*priv_).channel[0] != 0 || (*priv_).channel[1] != 1 { pr_err!("Invalid channels number or id for quadrature mode\n"); return -EINVAL; }
            (*priv_).qdec_mode = 1;
            bmr |= ATMEL_TC_QDEN | ATMEL_TC_POSEN;
            cmr |= ATMEL_TC_ETRGEDG_RISING | ATMEL_TC_ABETRG | ATMEL_TC_XC0;
        }
        _ => return -EINVAL,
    }
    regmap_write((*priv_).regmap, ATMEL_TC_BMR, bmr);
    regmap_write((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], CMR), cmr);
    regmap_write((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], CCR), ATMEL_TC_CLKEN | ATMEL_TC_SWTRG);
    if (*priv_).qdec_mode != 0 {
        regmap_write((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[1], CMR), cmr);
        regmap_write((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[1], CCR), ATMEL_TC_CLKEN | ATMEL_TC_SWTRG);
    }
    0
}

unsafe fn mchp_tc_count_signal_read(counter: *mut CounterDevice, signal: *mut CounterSignal, lvl: *mut CounterSignalLevel) -> i32 {
    let priv_ = counter_priv(counter); let mut sr = 0u32;
    regmap_read((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], SR), &mut sr);
    let high = if (*signal).id == 1 { sr & ATMEL_TC_MTIOB } else { sr & ATMEL_TC_MTIOA };
    *lvl = if high != 0 { COUNTER_SIGNAL_LEVEL_HIGH } else { COUNTER_SIGNAL_LEVEL_LOW }; 0
}

unsafe fn mchp_tc_count_action_read(counter: *mut CounterDevice, _count: *mut CounterCount, synapse: *mut CounterSynapse, action: *mut CounterSynapseAction) -> i32 {
    let priv_ = counter_priv(counter); let mut cmr = 0u32;
    if (*priv_).qdec_mode != 0 { *action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES; return 0; }
    if (*(*synapse).signal).id != 0 { *action = COUNTER_SYNAPSE_ACTION_NONE; return 0; }
    regmap_read((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], CMR), &mut cmr);
    *action = match cmr & ATMEL_TC_ETRGEDG { ATMEL_TC_ETRGEDG_RISING => COUNTER_SYNAPSE_ACTION_RISING_EDGE, ATMEL_TC_ETRGEDG_FALLING => COUNTER_SYNAPSE_ACTION_FALLING_EDGE, ATMEL_TC_ETRGEDG_BOTH => COUNTER_SYNAPSE_ACTION_BOTH_EDGES, _ => COUNTER_SYNAPSE_ACTION_NONE }; 0
}

unsafe fn mchp_tc_count_action_write(counter: *mut CounterDevice, _count: *mut CounterCount, synapse: *mut CounterSynapse, action: CounterSynapseAction) -> i32 {
    let priv_ = counter_priv(counter);
    if (*priv_).qdec_mode != 0 || (*(*synapse).signal).id != 0 { return -EINVAL; }
    let edge = match action { COUNTER_SYNAPSE_ACTION_NONE => ATMEL_TC_ETRGEDG_NONE, COUNTER_SYNAPSE_ACTION_RISING_EDGE => ATMEL_TC_ETRGEDG_RISING, COUNTER_SYNAPSE_ACTION_FALLING_EDGE => ATMEL_TC_ETRGEDG_FALLING, COUNTER_SYNAPSE_ACTION_BOTH_EDGES => ATMEL_TC_ETRGEDG_BOTH, _ => return -EINVAL };
    regmap_write_bits((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], CMR), ATMEL_TC_ETRGEDG, edge)
}

unsafe fn mchp_tc_count_read(counter: *mut CounterDevice, _count: *mut CounterCount, val: *mut u64) -> i32 {
    let priv_ = counter_priv(counter); let mut cnt = 0u32; regmap_read((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], CV), &mut cnt); *val = cnt as u64; 0
}

unsafe fn mchp_tc_count_cap_read(counter: *mut CounterDevice, _count: *mut CounterCount, idx: usize, val: *mut u64) -> i32 {
    let priv_ = counter_priv(counter); let mut cnt = 0u32;
    let reg = match idx { COUNTER_MCHP_EXCAP_RA => ATMEL_TC_REG((*priv_).channel[0], RA), COUNTER_MCHP_EXCAP_RB => ATMEL_TC_REG((*priv_).channel[0], RB), _ => return -EINVAL };
    let ret = regmap_read((*priv_).regmap, reg, &mut cnt); if ret < 0 { return ret; } *val = cnt as u64; 0
}

unsafe fn mchp_tc_count_cap_write(counter: *mut CounterDevice, _count: *mut CounterCount, idx: usize, val: u64) -> i32 {
    let priv_ = counter_priv(counter); if val > U32_MAX as u64 { return -ERANGE; }
    let reg = match idx { COUNTER_MCHP_EXCAP_RA => ATMEL_TC_REG((*priv_).channel[0], RA), COUNTER_MCHP_EXCAP_RB => ATMEL_TC_REG((*priv_).channel[0], RB), _ => return -EINVAL };
    regmap_write((*priv_).regmap, reg, val as u32)
}

unsafe fn mchp_tc_count_compare_read(counter: *mut CounterDevice, _count: *mut CounterCount, val: *mut u64) -> i32 {
    let priv_ = counter_priv(counter); let mut cnt = 0u32; let ret = regmap_read((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], RC), &mut cnt); if ret < 0 { return ret; } *val = cnt as u64; 0
}

unsafe fn mchp_tc_count_compare_write(counter: *mut CounterDevice, _count: *mut CounterCount, val: u64) -> i32 {
    let priv_ = counter_priv(counter); if val > U32_MAX as u64 { return -ERANGE; } regmap_write((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], RC), val as u32)
}

unsafe fn mchp_tc_watch_validate(_counter: *mut CounterDevice, watch: *const CounterWatch) -> i32 {
    if (*watch).channel == COUNTER_MCHP_EVCHN_CV || (*watch).channel == COUNTER_MCHP_EVCHN_RA { match (*watch).event { COUNTER_EVENT_CHANGE_OF_STATE | COUNTER_EVENT_OVERFLOW | COUNTER_EVENT_CAPTURE => return 0, _ => return -EINVAL } }
    if (*watch).channel == COUNTER_MCHP_EVCHN_RB && (*watch).event == COUNTER_EVENT_CAPTURE { return 0; }
    if (*watch).channel == COUNTER_MCHP_EVCHN_RC && (*watch).event == COUNTER_EVENT_THRESHOLD { return 0; }
    -EINVAL
}

unsafe fn mchp_tc_isr(_irq: i32, dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let counter = dev_id as *mut CounterDevice; let priv_ = counter_priv(counter); let mut sr = 0; let mut mask = 0;
    regmap_read((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], SR), &mut sr); regmap_read((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], IMR), &mut mask);
    sr &= mask; if sr & ATMEL_TC_ALL_IRQ == 0 { return IRQ_NONE; }
    if sr & ATMEL_TC_ETRGS != 0 { counter_push_event(counter, COUNTER_EVENT_CHANGE_OF_STATE, COUNTER_MCHP_EVCHN_CV); }
    if sr & ATMEL_TC_LDRAS != 0 { counter_push_event(counter, COUNTER_EVENT_CAPTURE, COUNTER_MCHP_EVCHN_RA); }
    if sr & ATMEL_TC_LDRBS != 0 { counter_push_event(counter, COUNTER_EVENT_CAPTURE, COUNTER_MCHP_EVCHN_RB); }
    if sr & ATMEL_TC_CPCS != 0 { counter_push_event(counter, COUNTER_EVENT_THRESHOLD, COUNTER_MCHP_EVCHN_RC); }
    if sr & ATMEL_TC_COVFS != 0 { counter_push_event(counter, COUNTER_EVENT_OVERFLOW, COUNTER_MCHP_EVCHN_CV); } IRQ_HANDLED
}

unsafe fn mchp_tc_irq_remove(ptr: *mut core::ffi::c_void) { let priv_ = ptr as *mut MchpTcData; regmap_write((*priv_).regmap, ATMEL_TC_REG((*priv_).channel[0], IDR), ATMEL_TC_DEF_IRQS); }
unsafe fn mchp_tc_clk_remove(ptr: *mut core::ffi::c_void) { clk_disable_unprepare(ptr as *mut Clk); }

// The following descriptors and platform-driver registration use dependency-provided
// kernel bindings for their field layouts and registration macros.
static MCHP_TC_COUNT_EXT: [CounterComp; 2] = [
    COUNTER_COMP_ARRAY_CAPTURE!(mchp_tc_count_cap_read, mchp_tc_count_cap_write, MCHP_TC_CNT_CAP_ARRAY),
    COUNTER_COMP_COMPARE!(mchp_tc_count_compare_read, mchp_tc_count_compare_write),
];
static mut MCHP_TC_OPS: CounterOps = CounterOps {
    signal_read: Some(mchp_tc_count_signal_read), count_read: Some(mchp_tc_count_read),
    function_read: Some(mchp_tc_count_function_read), function_write: Some(mchp_tc_count_function_write),
    action_read: Some(mchp_tc_count_action_read), action_write: Some(mchp_tc_count_action_write),
    watch_validate: Some(mchp_tc_watch_validate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
