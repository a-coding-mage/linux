// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Philipp Zabel, Pengutronix
 *
 * PWM (mis)used as clock output
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct clk_pwm {
    pub hw: clk_hw,
    pub pwm: *mut pwm_device,
    pub state: pwm_state,
    pub fixed_rate: u32,
}

#[inline]
unsafe fn to_clk_pwm(hw: *mut clk_hw) -> *mut clk_pwm {
    container_of!(hw, clk_pwm, hw)
}

unsafe extern "C" {
    fn pwm_apply_atomic(pwm: *mut pwm_device, state: *const pwm_state) -> i32;
    fn pwm_apply_might_sleep(pwm: *mut pwm_device, state: *const pwm_state) -> i32;
    fn pwm_disable(pwm: *mut pwm_device);
    fn pwm_get_state_hw(pwm: *mut pwm_device, state: *mut pwm_state) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn devm_pwm_get(dev: *mut device, con_id: *const core::ffi::c_char) -> *mut pwm_device;
    fn pwm_get_args(pwm: *mut pwm_device, args: *mut pwm_args);
    fn of_property_read_u32(node: *mut device_node, propname: *const core::ffi::c_char, out: *mut u32) -> i32;
    fn div64_u64(dividend: u64, divisor: u64) -> u64;
    fn pwm_init_state(pwm: *mut pwm_device, state: *mut pwm_state);
    fn pwm_set_relative_duty_cycle(state: *mut pwm_state, duty_cycle: u32, scale: u32);
    fn pwm_might_sleep(pwm: *mut pwm_device) -> bool;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn of_clk_add_hw_provider(node: *mut device_node, get: *const core::ffi::c_void, data: *mut clk_hw) -> i32;
    fn of_clk_del_provider(node: *mut device_node);
    fn of_property_read_string(node: *mut device_node, propname: *const core::ffi::c_char, out: *mut *const core::ffi::c_char) -> i32;
    fn of_clk_hw_simple_get(_: *mut device_node, _: *const core::ffi::c_void) -> *mut clk_hw;
}

unsafe fn clk_pwm_enable(hw: *mut clk_hw) -> i32 {
    let clk_pwm = to_clk_pwm(hw);
    pwm_apply_atomic((*clk_pwm).pwm, &(*clk_pwm).state)
}

unsafe fn clk_pwm_disable(hw: *mut clk_hw) {
    let clk_pwm = to_clk_pwm(hw);
    let mut state = (*clk_pwm).state;
    state.enabled = false;
    pwm_apply_atomic((*clk_pwm).pwm, &state);
}

unsafe fn clk_pwm_prepare(hw: *mut clk_hw) -> i32 {
    let clk_pwm = to_clk_pwm(hw);
    pwm_apply_might_sleep((*clk_pwm).pwm, &(*clk_pwm).state)
}

unsafe fn clk_pwm_unprepare(hw: *mut clk_hw) {
    let clk_pwm = to_clk_pwm(hw);
    pwm_disable((*clk_pwm).pwm);
}

unsafe fn clk_pwm_recalc_rate(hw: *mut clk_hw, _parent_rate: u64) -> u64 {
    (*to_clk_pwm(hw)).fixed_rate as u64
}

unsafe fn clk_pwm_get_duty_cycle(hw: *mut clk_hw, duty: *mut clk_duty) -> i32 {
    let clk_pwm = to_clk_pwm(hw);
    let mut state = core::mem::zeroed::<pwm_state>();
    let ret = pwm_get_state_hw((*clk_pwm).pwm, &mut state);
    if ret != 0 { return ret; }
    (*duty).num = state.duty_cycle;
    (*duty).den = state.period;
    0
}

static CLK_PWM_OPS_ATOMIC: clk_ops = clk_ops {
    enable: Some(clk_pwm_enable), disable: Some(clk_pwm_disable), recalc_rate: Some(clk_pwm_recalc_rate),
    get_duty_cycle: Some(clk_pwm_get_duty_cycle), ..unsafe { core::mem::zeroed() }
};

static CLK_PWM_OPS: clk_ops = clk_ops {
    prepare: Some(clk_pwm_prepare), unprepare: Some(clk_pwm_unprepare), recalc_rate: Some(clk_pwm_recalc_rate),
    get_duty_cycle: Some(clk_pwm_get_duty_cycle), ..unsafe { core::mem::zeroed() }
};

unsafe fn clk_pwm_probe(pdev: *mut platform_device) -> i32 {
    let node = (*pdev).dev.of_node;
    let mut init = core::mem::zeroed::<clk_init_data>();
    let clk_pwm = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<clk_pwm>(), GFP_KERNEL) as *mut clk_pwm;
    if clk_pwm.is_null() { return -ENOMEM; }
    let pwm = devm_pwm_get(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR!(pwm) { return PTR_ERR!(pwm); }
    let mut pargs = core::mem::zeroed::<pwm_args>();
    pwm_get_args(pwm, &mut pargs);
    if pargs.period == 0 { dev_err!(&mut (*pdev).dev, "invalid PWM period\n"); return -EINVAL; }
    if of_property_read_u32(node, c"clock-frequency".as_ptr(), &mut (*clk_pwm).fixed_rate) != 0 {
        (*clk_pwm).fixed_rate = div64_u64(NSEC_PER_SEC, pargs.period as u64) as u32;
    }
    if (*clk_pwm).fixed_rate == 0 { dev_err!(&mut (*pdev).dev, "fixed_rate cannot be zero\n"); return -EINVAL; }
    let rate = (*clk_pwm).fixed_rate as u64;
    if pargs.period != NSEC_PER_SEC / rate && pargs.period != DIV_ROUND_UP!(NSEC_PER_SEC, rate) { return -EINVAL; }
    pwm_init_state(pwm, &mut (*clk_pwm).state);
    pwm_set_relative_duty_cycle(&mut (*clk_pwm).state, 1, 2);
    (*clk_pwm).state.enabled = true;
    let mut clk_name = (*node).name;
    of_property_read_string(node, c"clock-output-names".as_ptr(), &mut clk_name);
    init.name = clk_name;
    init.ops = if pwm_might_sleep(pwm) { &CLK_PWM_OPS } else { &CLK_PWM_OPS_ATOMIC };
    init.flags = 0;
    init.num_parents = 0;
    (*clk_pwm).pwm = pwm;
    (*clk_pwm).hw.init = &init;
    let ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*clk_pwm).hw);
    if ret != 0 { return ret; }
    of_clk_add_hw_provider(node, of_clk_hw_simple_get as *const _, &mut (*clk_pwm).hw)
}

unsafe fn clk_pwm_remove(pdev: *mut platform_device) { of_clk_del_provider((*pdev).dev.of_node); }

static CLK_PWM_DT_IDS: [of_device_id; 2] = [
    of_device_id { compatible: c"pwm-clock".as_ptr(), ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];

static mut CLK_PWM_DRIVER: platform_driver = platform_driver {
    probe: Some(clk_pwm_probe), remove: Some(clk_pwm_remove),
    driver: driver { name: c"pwm-clock".as_ptr(), of_match_table: CLK_PWM_DT_IDS.as_ptr(), ..unsafe { core::mem::zeroed() } },
};

module_platform_driver!(CLK_PWM_DRIVER);
module_author!("Philipp Zabel <p.zabel@pengutronix.de>");
module_description!("PWM clock driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
