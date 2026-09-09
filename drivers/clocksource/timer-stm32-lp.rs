// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) STMicroelectronics 2019 - All Rights Reserved
 * Authors: Benjamin Gaignard <benjamin.gaignard@st.com> for STMicroelectronics.
 *          Pascal Paillet <p.paillet@st.com> for STMicroelectronics.
 */

const CFGR_PSC_OFFSET: u32 = 9;
const STM32_LP_RATING: i32 = 1000;
const STM32_TARGET_CLKRATE: u64 = 32000 * HZ as u64;
const STM32_LP_MAX_PSC: i32 = 7;

#[repr(C)]
struct Stm32LpPrivate {
    reg: *mut Regmap,
    clkevt: ClockEventDevice,
    period: usize,
    psc: u32,
    dev: *mut Device,
    clk: *mut Clk,
    version: u32,
}

unsafe fn to_priv(clkevt: *mut ClockEventDevice) -> *mut Stm32LpPrivate {
    (clkevt as *mut u8).sub(core::mem::offset_of!(Stm32LpPrivate, clkevt)) as *mut Stm32LpPrivate
}

unsafe fn stm32_clkevent_lp_shutdown(clkevt: *mut ClockEventDevice) -> i32 {
    let priv_ = &mut *to_priv(clkevt);
    regmap_write(priv_.reg, STM32_LPTIM_CR, 0);
    regmap_write(priv_.reg, STM32_LPTIM_IER, 0);
    // clear pending flags
    regmap_write(priv_.reg, STM32_LPTIM_ICR, STM32_LPTIM_ARRMCF);
    0
}

unsafe fn stm32mp25_clkevent_lp_set_evt(priv_: *mut Stm32LpPrivate, evt: usize) -> i32 {
    let p = &mut *priv_;
    let mut val = 0u32;
    regmap_read(p.reg, STM32_LPTIM_CR, &mut val);
    if (FIELD_GET(STM32_LPTIM_ENABLE, val)) == 0 {
        // Enable LPTIMER to be able to write into IER and ARR registers
        regmap_write(p.reg, STM32_LPTIM_CR, STM32_LPTIM_ENABLE);
        // After setting ENABLE, wait two counter clock cycles (rounded up).
        udelay(63);
    }
    regmap_write(p.reg, STM32_LPTIM_ARR, evt as u32);
    regmap_write(p.reg, STM32_LPTIM_IER, STM32_LPTIM_ARRMIE);
    let ret = regmap_read_poll_timeout_atomic(p.reg, STM32_LPTIM_ISR, &mut val,
        (val & STM32_LPTIM_DIEROK_ARROK) == STM32_LPTIM_DIEROK_ARROK, 10, 500);
    if ret != 0 {
        dev_err(p.dev, "access to LPTIM timed out\n");
        regmap_write(p.reg, STM32_LPTIM_CR, 0);
        return ret;
    }
    regmap_write(p.reg, STM32_LPTIM_ICR, STM32_LPTIM_DIEROKCF_ARROKCF);
    0
}

unsafe fn stm32_clkevent_lp_set_evt(priv_: *mut Stm32LpPrivate, evt: usize) {
    let p = &mut *priv_;
    regmap_write(p.reg, STM32_LPTIM_CR, 0);
    regmap_write(p.reg, STM32_LPTIM_IER, STM32_LPTIM_ARRMIE);
    regmap_write(p.reg, STM32_LPTIM_CR, STM32_LPTIM_ENABLE);
    regmap_write(p.reg, STM32_LPTIM_ARR, evt as u32);
}

unsafe fn stm32_clkevent_lp_set_timer(evt: usize, clkevt: *mut ClockEventDevice, is_periodic: i32) -> i32 {
    let priv_ = to_priv(clkevt);
    if (*priv_).version == STM32_LPTIM_VERR_23 {
        let ret = stm32mp25_clkevent_lp_set_evt(priv_, evt);
        if ret != 0 { return ret; }
    } else {
        stm32_clkevent_lp_set_evt(priv_, evt);
    }
    if is_periodic != 0 {
        regmap_write((*priv_).reg, STM32_LPTIM_CR, STM32_LPTIM_CNTSTRT | STM32_LPTIM_ENABLE);
    } else {
        regmap_write((*priv_).reg, STM32_LPTIM_CR, STM32_LPTIM_SNGSTRT | STM32_LPTIM_ENABLE);
    }
    0
}

unsafe fn stm32_clkevent_lp_set_next_event(evt: usize, clkevt: *mut ClockEventDevice) -> i32 {
    stm32_clkevent_lp_set_timer(evt, clkevt, clockevent_state_periodic(clkevt))
}

unsafe fn stm32_clkevent_lp_set_periodic(clkevt: *mut ClockEventDevice) -> i32 {
    let p = to_priv(clkevt);
    stm32_clkevent_lp_set_timer((*p).period, clkevt, 1)
}

unsafe fn stm32_clkevent_lp_set_oneshot(clkevt: *mut ClockEventDevice) -> i32 {
    let p = to_priv(clkevt);
    stm32_clkevent_lp_set_timer((*p).period, clkevt, 0)
}

unsafe extern "C" fn stm32_clkevent_lp_irq_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    let clkevt = dev_id as *mut ClockEventDevice;
    let p = &mut *to_priv(clkevt);
    regmap_write(p.reg, STM32_LPTIM_ICR, STM32_LPTIM_ARRMCF);
    if let Some(handler) = (*clkevt).event_handler { handler(clkevt); }
    IRQ_HANDLED
}

unsafe fn stm32_clkevent_lp_set_prescaler(priv_: *mut Stm32LpPrivate, rate: *mut usize) {
    let p = &mut *priv_;
    let mut i = 0i32;
    while i <= STM32_LP_MAX_PSC {
        if div_round_closest(*rate as u64, (1u64 << i) as u64) < STM32_TARGET_CLKRATE { break; }
        i += 1;
    }
    regmap_write(p.reg, STM32_LPTIM_CFGR, (i as u32) << CFGR_PSC_OFFSET);
    *rate = div_round_closest(*rate as u64, (1u64 << i) as u64) as usize;
    p.period = div_round_up(*rate, HZ as usize);
    p.psc = i as u32;
}

unsafe fn stm32_clkevent_lp_suspend(clkevt: *mut ClockEventDevice) {
    let p = to_priv(clkevt);
    stm32_clkevent_lp_shutdown(clkevt);
    clk_disable_unprepare((*p).clk);
}

unsafe fn stm32_clkevent_lp_resume(clkevt: *mut ClockEventDevice) {
    let p = to_priv(clkevt);
    clk_prepare_enable((*p).clk);
    regmap_write((*p).reg, STM32_LPTIM_CFGR, (*p).psc << CFGR_PSC_OFFSET);
}

unsafe fn stm32_clkevent_lp_init(priv_: *mut Stm32LpPrivate, np: *mut DeviceNode, rate: usize) {
    let p = &mut *priv_;
    p.clkevt.name = (*np).full_name;
    p.clkevt.cpumask = cpu_possible_mask;
    p.clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    p.clkevt.set_state_shutdown = Some(stm32_clkevent_lp_shutdown);
    p.clkevt.set_state_periodic = Some(stm32_clkevent_lp_set_periodic);
    p.clkevt.set_state_oneshot = Some(stm32_clkevent_lp_set_oneshot);
    p.clkevt.set_next_event = Some(stm32_clkevent_lp_set_next_event);
    p.clkevt.rating = STM32_LP_RATING;
    p.clkevt.suspend = Some(stm32_clkevent_lp_suspend);
    p.clkevt.resume = Some(stm32_clkevent_lp_resume);
    p.clkevt.owner = THIS_MODULE;
    clockevents_config_and_register(&mut p.clkevt, rate, 0x1, STM32_LPTIM_MAX_ARR);
}

unsafe fn stm32_clkevent_lp_probe(pdev: *mut PlatformDevice) -> i32 {
    let ddata = dev_get_drvdata((*(*pdev).dev.parent));
    let priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Stm32LpPrivate>(), GFP_KERNEL)
        as *mut Stm32LpPrivate;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).reg = (*ddata).regmap;
    (*priv_).version = (*ddata).version;
    (*priv_).clk = (*ddata).clk;
    let mut ret = clk_prepare_enable((*priv_).clk);
    if ret != 0 { return -EINVAL; }
    let mut rate = clk_get_rate((*priv_).clk);
    if rate == 0 { ret = -EINVAL; goto_out_clk_disable((*priv_).clk, ret); }
    let irq = platform_get_irq(to_platform_device((*pdev).dev.parent), 0);
    if irq <= 0 { ret = irq; goto_out_clk_disable((*priv_).clk, ret); }
    if of_property_read_bool((*(*pdev).dev.parent).of_node, "wakeup-source") {
        device_set_wakeup_capable(&mut (*pdev).dev, true);
        ret = dev_pm_set_wake_irq(&mut (*pdev).dev, irq);
        if ret != 0 { goto_out_clk_disable((*priv_).clk, ret); }
    }
    ret = devm_request_irq(&mut (*pdev).dev, irq, Some(stm32_clkevent_lp_irq_handler),
                           IRQF_TIMER, (*pdev).name, &mut (*priv_).clkevt);
    if ret != 0 { goto_out_clk_disable((*priv_).clk, ret); }
    stm32_clkevent_lp_set_prescaler(priv_, &mut rate);
    stm32_clkevent_lp_init(priv_, (*(*pdev).dev.parent).of_node, rate);
    (*priv_).dev = &mut (*pdev).dev;
    0
}

unsafe fn goto_out_clk_disable(clk: *mut Clk, ret: i32) -> ! {
    clk_disable_unprepare(clk);
    // C control flow returns the saved error after the cleanup.
    panic!("kernel probe error: {}", ret)
}

#[repr(C)]
struct OfDeviceId { compatible: *const u8 }
static STM32_CLKEVENT_LP_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"st,stm32-lptimer-timer\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

#[repr(C)]
struct PlatformDriver { probe: unsafe fn(*mut PlatformDevice) -> i32, name: *const u8,
                         of_match_table: *const OfDeviceId, suppress_bind_attrs: bool }
static mut STM32_CLKEVENT_LP_DRIVER: PlatformDriver = PlatformDriver {
    probe: stm32_clkevent_lp_probe, name: b"stm32-lptimer-timer\0".as_ptr(),
    of_match_table: STM32_CLKEVENT_LP_OF_MATCH.as_ptr(), suppress_bind_attrs: true,
};

// Equivalent of module_platform_driver(stm32_clkevent_lp_driver).
// MODULE_DESCRIPTION("STMicroelectronics STM32 clockevent low power driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
