// SPDX-License-Identifier: GPL-2.0
/*
 * STM32 Timer Encoder and Counter driver
 *
 * Copyright (C) STMicroelectronics 2018
 *
 * Author: Benjamin Gaignard <benjamin.gaignard@st.com>
 */

// Dependencies corresponding to the original Linux kernel includes are supplied externally.

const TIM_CCMR_CCXS: u32 = (1 << 8) | (1 << 0);
const TIM_CCMR_MASK: u32 = TIM_CCMR_CC1S | TIM_CCMR_CC2S | TIM_CCMR_IC1F | TIM_CCMR_IC2F;
const TIM_CCER_MASK: u32 = TIM_CCER_CC1P | TIM_CCER_CC1NP | TIM_CCER_CC2P | TIM_CCER_CC2NP;

const STM32_CH1_SIG: u32 = 0;
const STM32_CH2_SIG: u32 = 1;
const STM32_CLOCK_SIG: u32 = 2;
const STM32_CH3_SIG: u32 = 3;
const STM32_CH4_SIG: u32 = 4;

#[repr(C)]
struct stm32_timer_regs {
    cr1: u32,
    cnt: u32,
    smcr: u32,
    arr: u32,
}

#[repr(C)]
struct stm32_timer_cnt {
    regmap: *mut regmap,
    clk: *mut clk,
    max_arr: u32,
    enabled: bool,
    bak: stm32_timer_regs,
    has_encoder: bool,
    nchannels: u32,
    nr_irqs: u32,
    lock: spinlock_t, // protects nb_ovf
    nb_ovf: u64,
}

static STM32_COUNT_FUNCTIONS: [counter_function; 4] = [
    COUNTER_FUNCTION_INCREASE,
    COUNTER_FUNCTION_QUADRATURE_X2_A,
    COUNTER_FUNCTION_QUADRATURE_X2_B,
    COUNTER_FUNCTION_QUADRATURE_X4,
];

unsafe fn stm32_count_read(counter: *mut counter_device, _count: *mut counter_count, val: *mut u64) -> i32 {
    let priv_ = counter_priv(counter);
    let mut cnt = 0u32;
    regmap_read((*priv_).regmap, TIM_CNT, &mut cnt);
    *val = cnt as u64;
    0
}

unsafe fn stm32_count_write(counter: *mut counter_device, _count: *mut counter_count, val: u64) -> i32 {
    let priv_ = counter_priv(counter);
    let mut ceiling = 0u32;
    regmap_read((*priv_).regmap, TIM_ARR, &mut ceiling);
    if val > ceiling as u64 { return -EINVAL; }
    regmap_write((*priv_).regmap, TIM_CNT, val as u32)
}

unsafe fn stm32_count_function_read(counter: *mut counter_device, _count: *mut counter_count, function: *mut counter_function) -> i32 {
    let priv_ = counter_priv(counter);
    let mut smcr = 0u32;
    regmap_read((*priv_).regmap, TIM_SMCR, &mut smcr);
    match smcr & TIM_SMCR_SMS {
        TIM_SMCR_SMS_SLAVE_MODE_DISABLED => { *function = COUNTER_FUNCTION_INCREASE; 0 }
        TIM_SMCR_SMS_ENCODER_MODE_1 => { *function = COUNTER_FUNCTION_QUADRATURE_X2_A; 0 }
        TIM_SMCR_SMS_ENCODER_MODE_2 => { *function = COUNTER_FUNCTION_QUADRATURE_X2_B; 0 }
        TIM_SMCR_SMS_ENCODER_MODE_3 => { *function = COUNTER_FUNCTION_QUADRATURE_X4; 0 }
        _ => -EINVAL,
    }
}

unsafe fn stm32_count_function_write(counter: *mut counter_device, _count: *mut counter_count, function: counter_function) -> i32 {
    let priv_ = counter_priv(counter);
    let sms = match function {
        COUNTER_FUNCTION_INCREASE => TIM_SMCR_SMS_SLAVE_MODE_DISABLED,
        COUNTER_FUNCTION_QUADRATURE_X2_A => { if !(*priv_).has_encoder { return -EOPNOTSUPP; } TIM_SMCR_SMS_ENCODER_MODE_1 }
        COUNTER_FUNCTION_QUADRATURE_X2_B => { if !(*priv_).has_encoder { return -EOPNOTSUPP; } TIM_SMCR_SMS_ENCODER_MODE_2 }
        COUNTER_FUNCTION_QUADRATURE_X4 => { if !(*priv_).has_encoder { return -EOPNOTSUPP; } TIM_SMCR_SMS_ENCODER_MODE_3 }
        _ => return -EINVAL,
    };
    let mut cr1 = 0u32;
    regmap_read((*priv_).regmap, TIM_CR1, &mut cr1);
    regmap_update_bits((*priv_).regmap, TIM_CR1, TIM_CR1_CEN, 0);
    regmap_update_bits((*priv_).regmap, TIM_SMCR, TIM_SMCR_SMS, sms);
    regmap_update_bits((*priv_).regmap, TIM_EGR, TIM_EGR_UG, TIM_EGR_UG);
    regmap_update_bits((*priv_).regmap, TIM_CR1, TIM_CR1_CEN, cr1);
    0
}

unsafe fn stm32_count_direction_read(counter: *mut counter_device, _count: *mut counter_count, direction: *mut counter_count_direction) -> i32 {
    let priv_ = counter_priv(counter);
    let mut cr1 = 0u32;
    regmap_read((*priv_).regmap, TIM_CR1, &mut cr1);
    *direction = if cr1 & TIM_CR1_DIR != 0 { COUNTER_COUNT_DIRECTION_BACKWARD } else { COUNTER_COUNT_DIRECTION_FORWARD };
    0
}

unsafe fn stm32_count_ceiling_read(counter: *mut counter_device, _count: *mut counter_count, ceiling: *mut u64) -> i32 {
    let priv_ = counter_priv(counter);
    let mut arr = 0u32;
    regmap_read((*priv_).regmap, TIM_ARR, &mut arr);
    *ceiling = arr as u64;
    0
}

unsafe fn stm32_count_ceiling_write(counter: *mut counter_device, _count: *mut counter_count, ceiling: u64) -> i32 {
    let priv_ = counter_priv(counter);
    if ceiling > (*priv_).max_arr as u64 { return -ERANGE; }
    regmap_update_bits((*priv_).regmap, TIM_CR1, TIM_CR1_ARPE, 0);
    regmap_write((*priv_).regmap, TIM_ARR, ceiling as u32);
    0
}

unsafe fn stm32_count_enable_read(counter: *mut counter_device, _count: *mut counter_count, enable: *mut u8) -> i32 {
    let priv_ = counter_priv(counter);
    let mut cr1 = 0u32;
    regmap_read((*priv_).regmap, TIM_CR1, &mut cr1);
    *enable = (cr1 & TIM_CR1_CEN) as u8;
    0
}

unsafe fn stm32_count_enable_write(counter: *mut counter_device, _count: *mut counter_count, enable: u8) -> i32 {
    let priv_ = counter_priv(counter);
    let mut cr1 = 0u32;
    if enable != 0 {
        regmap_read((*priv_).regmap, TIM_CR1, &mut cr1);
        if cr1 & TIM_CR1_CEN == 0 {
            let ret = clk_enable((*priv_).clk);
            if ret != 0 { dev_err((*counter).parent, "Cannot enable clock %d\n", ret); return ret; }
        }
        regmap_update_bits((*priv_).regmap, TIM_CR1, TIM_CR1_CEN, TIM_CR1_CEN);
    } else {
        regmap_read((*priv_).regmap, TIM_CR1, &mut cr1);
        regmap_update_bits((*priv_).regmap, TIM_CR1, TIM_CR1_CEN, 0);
        if cr1 & TIM_CR1_CEN != 0 { clk_disable((*priv_).clk); }
    }
    (*priv_).enabled = enable != 0;
    0
}

unsafe fn stm32_count_prescaler_read(counter: *mut counter_device, _count: *mut counter_count, prescaler: *mut u64) -> i32 {
    let priv_ = counter_priv(counter);
    let mut psc = 0u32;
    regmap_read((*priv_).regmap, TIM_PSC, &mut psc);
    *prescaler = psc as u64 + 1;
    0
}

unsafe fn stm32_count_prescaler_write(counter: *mut counter_device, _count: *mut counter_count, prescaler: u64) -> i32 {
    let priv_ = counter_priv(counter);
    if prescaler == 0 || prescaler > (MAX_TIM_PSC as u64 + 1) { return -ERANGE; }
    regmap_write((*priv_).regmap, TIM_PSC, (prescaler - 1) as u32)
}

unsafe fn stm32_count_cap_read(counter: *mut counter_device, _count: *mut counter_count, ch: usize, cap: *mut u64) -> i32 {
    let priv_ = counter_priv(counter);
    if ch >= (*priv_).nchannels as usize { return -EOPNOTSUPP; }
    let mut ccrx = 0u32;
    match ch { 0 => regmap_read((*priv_).regmap, TIM_CCR1, &mut ccrx), 1 => regmap_read((*priv_).regmap, TIM_CCR2, &mut ccrx), 2 => regmap_read((*priv_).regmap, TIM_CCR3, &mut ccrx), 3 => regmap_read((*priv_).regmap, TIM_CCR4, &mut ccrx), _ => return -EINVAL }
    dev_dbg((*counter).parent, "CCR%zu: 0x%08x\n", ch + 1, ccrx);
    *cap = ccrx as u64;
    0
}

unsafe fn stm32_count_nb_ovf_read(counter: *mut counter_device, _count: *mut counter_count, val: *mut u64) -> i32 {
    let priv_ = counter_priv(counter); let mut irqflags = 0ul;
    spin_lock_irqsave(&mut (*priv_).lock, &mut irqflags); *val = (*priv_).nb_ovf; spin_unlock_irqrestore(&mut (*priv_).lock, irqflags); 0
}

unsafe fn stm32_count_nb_ovf_write(counter: *mut counter_device, _count: *mut counter_count, val: u64) -> i32 {
    let priv_ = counter_priv(counter); let mut irqflags = 0ul;
    spin_lock_irqsave(&mut (*priv_).lock, &mut irqflags); (*priv_).nb_ovf = val; spin_unlock_irqrestore(&mut (*priv_).lock, irqflags); 0
}

#[repr(C)]
struct stm32_count_cc_regs { ccmr_reg: u32, ccmr_mask: u32, ccmr_bits: u32, ccer_bits: u32 }

static STM32_CC: [stm32_count_cc_regs; 4] = [
    stm32_count_cc_regs { ccmr_reg: TIM_CCMR1, ccmr_mask: TIM_CCMR_CC1S, ccmr_bits: TIM_CCMR_CC1S_TI1, ccer_bits: TIM_CCER_CC1E | TIM_CCER_CC1P | TIM_CCER_CC1NP },
    stm32_count_cc_regs { ccmr_reg: TIM_CCMR1, ccmr_mask: TIM_CCMR_CC2S, ccmr_bits: TIM_CCMR_CC2S_TI2, ccer_bits: TIM_CCER_CC2E | TIM_CCER_CC2P | TIM_CCER_CC2NP },
    stm32_count_cc_regs { ccmr_reg: TIM_CCMR2, ccmr_mask: TIM_CCMR_CC3S, ccmr_bits: TIM_CCMR_CC3S_TI3, ccer_bits: TIM_CCER_CC3E | TIM_CCER_CC3P | TIM_CCER_CC3NP },
    stm32_count_cc_regs { ccmr_reg: TIM_CCMR2, ccmr_mask: TIM_CCMR_CC4S, ccmr_bits: TIM_CCMR_CC4S_TI4, ccer_bits: TIM_CCER_CC4E | TIM_CCER_CC4P | TIM_CCER_CC4NP },
];

const STM32_TIM_ENCODER_SUPPORTED: u32 = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 7) | (1 << 19);

static STM32_TIMER_TRIGGER_COMPAT: [&str; 3] = [
    "st,stm32-timer-trigger", "st,stm32h7-timer-trigger", "st,stm32mp25-timer-trigger",
];

unsafe fn stm32_action_read(counter: *mut counter_device, count: *mut counter_count, synapse: *mut counter_synapse, action: *mut counter_synapse_action) -> i32;
unsafe fn stm32_count_capture_configure(counter: *mut counter_device, ch: u32, enable: bool) -> i32;
unsafe fn stm32_count_events_configure(counter: *mut counter_device) -> i32;
unsafe fn stm32_count_watch_validate(counter: *mut counter_device, watch: *const counter_watch) -> i32;
unsafe fn stm32_timer_cnt_isr(irq: i32, ptr: *mut core::ffi::c_void) -> irqreturn_t;
unsafe fn stm32_timer_cnt_detect_channels(dev: *mut device, priv_: *mut stm32_timer_cnt);
unsafe fn stm32_timer_cnt_probe_encoder(dev: *mut device, priv_: *mut stm32_timer_cnt) -> i32;
unsafe fn stm32_timer_cnt_probe(pdev: *mut platform_device) -> i32;
unsafe fn stm32_timer_cnt_suspend(dev: *mut device) -> i32;
unsafe fn stm32_timer_cnt_resume(dev: *mut device) -> i32;

extern "C" {
    static mut stm32_count_ext: counter_comp;
    static mut stm32_timer_cnt_driver: platform_driver;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
