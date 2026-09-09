// SPDX-License-Identifier: GPL-2.0
/*
 * Ingenic SoCs TCU IRQ driver
 * Copyright (C) 2019 Paul Cercueil <paul@crapouillou.net>
 * Copyright (C) 2020 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
 */

// Linux kernel dependencies supplied by the surrounding tree.

static DEFINE_PER_CPU!(call_single_data_t, ingenic_cevt_csd);

#[repr(C)]
struct ingenic_soc_info {
    num_channels: c_uint,
}

#[repr(C)]
struct ingenic_tcu_timer {
    cpu: c_uint,
    channel: c_uint,
    cevt: clock_event_device,
    clk: *mut clk,
    name: [c_char; 8],
}

#[repr(C)]
struct ingenic_tcu {
    map: *mut regmap,
    np: *mut device_node,
    cs_clk: *mut clk,
    cs_channel: c_uint,
    cs: clocksource,
    pwm_channels_mask: c_ulong,
    timers: [ingenic_tcu_timer; 0],
}

static mut ingenic_tcu: *mut ingenic_tcu = core::ptr::null_mut();

unsafe fn ingenic_tcu_timer_read() -> u64 {
    let tcu = ingenic_tcu;
    let mut count: c_uint = 0;

    regmap_read((*tcu).map, TCU_REG_TCNTc((*tcu).cs_channel), &mut count);

    count as u64
}

unsafe extern "C" fn ingenic_tcu_timer_cs_read(_cs: *mut clocksource) -> u64 {
    ingenic_tcu_timer_read()
}

unsafe fn to_ingenic_tcu(timer: *mut ingenic_tcu_timer) -> *mut ingenic_tcu {
    container_of!(timer, ingenic_tcu, timers[(*timer).cpu])
}

unsafe fn to_ingenic_tcu_timer(evt: *mut clock_event_device) -> *mut ingenic_tcu_timer {
    container_of!(evt, ingenic_tcu_timer, cevt)
}

unsafe extern "C" fn ingenic_tcu_cevt_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    let timer = to_ingenic_tcu_timer(evt);
    let tcu = to_ingenic_tcu(timer);

    regmap_write((*tcu).map, TCU_REG_TECR, BIT((*timer).channel));
    0
}

unsafe extern "C" fn ingenic_tcu_cevt_set_next(
    next: c_ulong,
    evt: *mut clock_event_device,
) -> c_int {
    let timer = to_ingenic_tcu_timer(evt);
    let tcu = to_ingenic_tcu(timer);

    if next > 0xffff {
        return -EINVAL;
    }

    regmap_write((*tcu).map, TCU_REG_TDFRc((*timer).channel), next);
    regmap_write((*tcu).map, TCU_REG_TCNTc((*timer).channel), 0);
    regmap_write((*tcu).map, TCU_REG_TESR, BIT((*timer).channel));
    0
}

unsafe extern "C" fn ingenic_per_cpu_event_handler(info: *mut c_void) {
    let cevt = info as *mut clock_event_device;
    ((*cevt).event_handler)(cevt);
}

unsafe extern "C" fn ingenic_tcu_cevt_cb(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let timer = dev_id as *mut ingenic_tcu_timer;
    let tcu = to_ingenic_tcu(timer);
    let csd: *mut call_single_data_t;

    regmap_write((*tcu).map, TCU_REG_TECR, BIT((*timer).channel));

    if !(*timer).cevt.event_handler.is_none() {
        csd = &mut per_cpu!(ingenic_cevt_csd, (*timer).cpu);
        (*csd).info = &mut (*timer).cevt as *mut clock_event_device as *mut c_void;
        (*csd).func = Some(ingenic_per_cpu_event_handler);
        smp_call_function_single_async((*timer).cpu, csd);
    }

    IRQ_HANDLED
}

unsafe fn ingenic_tcu_get_clock(np: *mut device_node, id: c_int) -> *mut clk {
    let mut args: of_phandle_args = core::mem::zeroed();
    args.np = np;
    args.args_count = 1;
    args.args[0] = id;
    of_clk_get_from_provider(&mut args)
}

unsafe extern "C" fn ingenic_tcu_setup_cevt(cpu: c_uint) -> c_int {
    let tcu = ingenic_tcu;
    let timer = &mut (*tcu).timers[cpu as usize];
    let mut timer_virq: c_uint;
    let domain: *mut irq_domain;
    let rate: c_ulong;
    let mut err: c_int;

    timer.clk = ingenic_tcu_get_clock((*tcu).np, timer.channel as c_int);
    if IS_ERR!(timer.clk) { return PTR_ERR!(timer.clk); }
    err = clk_prepare_enable(timer.clk);
    if err != 0 { goto!(err_clk_put); }
    rate = clk_get_rate(timer.clk);
    if rate == 0 { err = -EINVAL; goto!(err_clk_disable); }
    domain = irq_find_host((*tcu).np);
    if domain.is_null() { err = -ENODEV; goto!(err_clk_disable); }
    timer_virq = irq_create_mapping(domain, timer.channel);
    if timer_virq == 0 { err = -EINVAL; goto!(err_clk_disable); }
    snprintf!(timer.name.as_mut_ptr(), timer.name.len(), "TCU%u", timer.channel);
    err = request_irq(timer_virq, Some(ingenic_tcu_cevt_cb), IRQF_TIMER,
                      timer.name.as_ptr(), timer);
    if err != 0 { goto!(err_irq_dispose_mapping); }
    timer.cpu = smp_processor_id();
    timer.cevt.cpumask = cpumask_of(smp_processor_id());
    timer.cevt.features = CLOCK_EVT_FEAT_ONESHOT;
    timer.cevt.name = timer.name.as_ptr();
    timer.cevt.rating = 200;
    timer.cevt.set_state_shutdown = Some(ingenic_tcu_cevt_set_state_shutdown);
    timer.cevt.set_next_event = Some(ingenic_tcu_cevt_set_next);
    clockevents_config_and_register(&mut timer.cevt, rate, 10, 0xffff);
    return 0;

err_irq_dispose_mapping:
    irq_dispose_mapping(timer_virq);
err_clk_disable:
    clk_disable_unprepare(timer.clk);
err_clk_put:
    clk_put(timer.clk);
    err
}

static jz4740_soc_info: ingenic_soc_info = ingenic_soc_info { num_channels: 8 };
static jz4725b_soc_info: ingenic_soc_info = ingenic_soc_info { num_channels: 6 };

static ingenic_tcu_of_match: [of_device_id; 6] = [
    of_device_id { compatible: cstr!("ingenic,jz4740-tcu"), data: &jz4740_soc_info as *const _ as *const c_void },
    of_device_id { compatible: cstr!("ingenic,jz4725b-tcu"), data: &jz4725b_soc_info as *const _ as *const c_void },
    of_device_id { compatible: cstr!("ingenic,jz4760-tcu"), data: &jz4740_soc_info as *const _ as *const c_void },
    of_device_id { compatible: cstr!("ingenic,jz4770-tcu"), data: &jz4740_soc_info as *const _ as *const c_void },
    of_device_id { compatible: cstr!("ingenic,x1000-tcu"), data: &jz4740_soc_info as *const _ as *const c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn ingenic_tcu_clocksource_init(np: *mut device_node, tcu: *mut ingenic_tcu) -> c_int {
    let channel = (*tcu).cs_channel;
    let cs = &mut (*tcu).cs;
    (*tcu).cs_clk = ingenic_tcu_get_clock(np, channel as c_int);
    if IS_ERR!((*tcu).cs_clk) { return PTR_ERR!((*tcu).cs_clk); }
    let mut err = clk_prepare_enable((*tcu).cs_clk);
    if err != 0 { clk_put((*tcu).cs_clk); return err; }
    let rate = clk_get_rate((*tcu).cs_clk);
    if rate == 0 { err = -EINVAL; clk_disable_unprepare((*tcu).cs_clk); clk_put((*tcu).cs_clk); return err; }
    regmap_update_bits((*tcu).map, TCU_REG_TCSRc(channel), 0xffff & !TCU_TCSR_RESERVED_BITS, 0);
    regmap_write((*tcu).map, TCU_REG_TDFRc(channel), 0xffff);
    regmap_write((*tcu).map, TCU_REG_TCNTc(channel), 0);
    regmap_write((*tcu).map, TCU_REG_TESR, BIT(channel));
    cs.name = cstr!("ingenic-timer"); cs.rating = 200; cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    cs.mask = CLOCKSOURCE_MASK!(16); cs.read = Some(ingenic_tcu_timer_cs_read);
    err = clocksource_register_hz(cs, rate);
    if err != 0 { clk_disable_unprepare((*tcu).cs_clk); clk_put((*tcu).cs_clk); }
    err
}

unsafe extern "C" fn ingenic_tcu_init(np: *mut device_node) -> c_int {
    let id = of_match_node(ingenic_tcu_of_match.as_ptr(), np);
    let soc_info = (*id).data as *const ingenic_soc_info;
    let map = device_node_to_regmap(np);
    if IS_ERR!(map) { return PTR_ERR!(map); }
    let tcu = kzalloc_flex!(ingenic_tcu, timers, num_possible_cpus());
    if tcu.is_null() { return -ENOMEM; }
    (*tcu).pwm_channels_mask = GENMASK!((*soc_info).num_channels - 1, num_possible_cpus() + 1);
    of_property_read_u32(np, cstr!("ingenic,pwm-channels-mask"), &mut (*tcu).pwm_channels_mask as *mut _ as *mut u32);
    if hweight8((*tcu).pwm_channels_mask as u8) > (*soc_info).num_channels - num_possible_cpus() + 1 { kfree(tcu); return -EINVAL; }
    (*tcu).map = map; (*tcu).np = np; ingenic_tcu = tcu;
    let mut last_bit: c_int = -1;
    for cpu in 0..num_possible_cpus() { (*tcu).timers[cpu].cpu = cpu; (*tcu).timers[cpu].channel = find_next_zero_bit(&(*tcu).pwm_channels_mask, (*soc_info).num_channels, (last_bit + 1) as usize) as u32; last_bit = (*tcu).timers[cpu].channel as c_int; }
    (*tcu).cs_channel = find_next_zero_bit(&(*tcu).pwm_channels_mask, (*soc_info).num_channels, (last_bit + 1) as usize) as u32;
    let ret = ingenic_tcu_clocksource_init(np, tcu); if ret != 0 { kfree(tcu); return ret; }
    let ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, cstr!("Ingenic XBurst: online"), Some(ingenic_tcu_setup_cevt), None);
    if ret < 0 { clocksource_unregister(&mut (*tcu).cs); clk_disable_unprepare((*tcu).cs_clk); clk_put((*tcu).cs_clk); kfree(tcu); return ret; }
    sched_clock_register(Some(ingenic_tcu_timer_read), 16, clk_get_rate((*tcu).cs_clk)); 0
}

TIMER_OF_DECLARE!(jz4740_tcu_intc, "ingenic,jz4740-tcu", ingenic_tcu_init);
TIMER_OF_DECLARE!(jz4725b_tcu_intc, "ingenic,jz4725b-tcu", ingenic_tcu_init);
TIMER_OF_DECLARE!(jz4760_tcu_intc, "ingenic,jz4760-tcu", ingenic_tcu_init);
TIMER_OF_DECLARE!(jz4770_tcu_intc, "ingenic,jz4770-tcu", ingenic_tcu_init);
TIMER_OF_DECLARE!(x1000_tcu_intc, "ingenic,x1000-tcu", ingenic_tcu_init);

unsafe extern "C" fn ingenic_tcu_probe(pdev: *mut platform_device) -> c_int {
    platform_set_drvdata(pdev, ingenic_tcu);
    0
}

unsafe extern "C" fn ingenic_tcu_suspend(dev: *mut device) -> c_int {
    let tcu = dev_get_drvdata(dev) as *mut ingenic_tcu;
    clk_disable((*tcu).cs_clk);
    for cpu in 0..num_online_cpus() { clk_disable((*tcu).timers[cpu].clk); }
    0
}

unsafe extern "C" fn ingenic_tcu_resume(dev: *mut device) -> c_int {
    let tcu = dev_get_drvdata(dev) as *mut ingenic_tcu;
    let mut cpu = 0;
    while cpu < num_online_cpus() { let ret = clk_enable((*tcu).timers[cpu].clk); if ret != 0 { while cpu > 0 { cpu -= 1; clk_disable((*tcu).timers[cpu].clk); } return ret; } cpu += 1; }
    let ret = clk_enable((*tcu).cs_clk); if ret != 0 { while cpu > 0 { cpu -= 1; clk_disable((*tcu).timers[cpu].clk); } return ret; } 0
}

static ingenic_tcu_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend_noirq: Some(ingenic_tcu_suspend), resume_noirq: Some(ingenic_tcu_resume), ..ZEROED
};

static mut ingenic_tcu_driver: platform_driver = platform_driver {
    driver: driver { name: cstr!("ingenic-tcu-timer"), pm: pm_sleep_ptr!(&ingenic_tcu_pm_ops), of_match_table: ingenic_tcu_of_match.as_ptr(), ..ZEROED },
    ..ZEROED
};

builtin_platform_driver_probe!(ingenic_tcu_driver, ingenic_tcu_probe);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
