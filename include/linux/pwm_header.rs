/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel/Rust translation. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pwm_polarity {
    PWM_POLARITY_NORMAL,
    PWM_POLARITY_INVERSED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_args {
    pub period: u64,
    pub polarity: pwm_polarity,
}

pub const PWMF_REQUESTED: u32 = 0;
pub const PWMF_EXPORTED: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_waveform {
    pub period_length_ns: u64,
    pub duty_length_ns: u64,
    pub duty_offset_ns: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_state {
    pub period: u64,
    pub duty_cycle: u64,
    pub polarity: pwm_polarity,
    pub enabled: bool,
    pub usage_power: bool,
}

#[repr(C)]
pub struct pwm_device {
    pub label: *const core::ffi::c_char,
    pub flags: usize,
    pub hwpwm: u32,
    pub chip: *mut pwm_chip,
    pub args: pwm_args,
    pub state: pwm_state,
    pub last: pwm_state,
}

#[inline]
pub unsafe fn pwm_get_state(pwm: *const pwm_device, state: *mut pwm_state) {
    *state = (*pwm).state;
}

#[inline]
pub unsafe fn pwm_is_enabled(pwm: *const pwm_device) -> bool {
    let mut state = core::mem::MaybeUninit::<pwm_state>::uninit();
    pwm_get_state(pwm, state.as_mut_ptr());
    (*state.as_ptr()).enabled
}

#[inline]
pub unsafe fn pwm_get_period(pwm: *const pwm_device) -> u64 {
    let mut state = core::mem::MaybeUninit::<pwm_state>::uninit();
    pwm_get_state(pwm, state.as_mut_ptr());
    (*state.as_ptr()).period
}

#[inline]
pub unsafe fn pwm_get_duty_cycle(pwm: *const pwm_device) -> u64 {
    let mut state = core::mem::MaybeUninit::<pwm_state>::uninit();
    pwm_get_state(pwm, state.as_mut_ptr());
    (*state.as_ptr()).duty_cycle
}

#[inline]
pub unsafe fn pwm_get_polarity(pwm: *const pwm_device) -> pwm_polarity {
    let mut state = core::mem::MaybeUninit::<pwm_state>::uninit();
    pwm_get_state(pwm, state.as_mut_ptr());
    (*state.as_ptr()).polarity
}

#[inline]
pub unsafe fn pwm_get_args(pwm: *const pwm_device, args: *mut pwm_args) {
    *args = (*pwm).args;
}

#[inline]
pub unsafe fn pwm_init_state(pwm: *const pwm_device, state: *mut pwm_state) {
    let mut args = core::mem::MaybeUninit::<pwm_args>::uninit();
    pwm_get_state(pwm, state);
    pwm_get_args(pwm, args.as_mut_ptr());
    (*state).period = (*args.as_ptr()).period;
    (*state).polarity = (*args.as_ptr()).polarity;
    (*state).duty_cycle = 0;
    (*state).usage_power = false;
}

#[inline]
pub unsafe fn pwm_get_relative_duty_cycle(state: *const pwm_state, scale: u32) -> u32 {
    if (*state).period == 0 { return 0; }
    (((*state).duty_cycle.wrapping_mul(scale as u64) + (*state).period / 2) / (*state).period) as u32
}

#[inline]
pub unsafe fn pwm_set_relative_duty_cycle(state: *mut pwm_state, duty_cycle: u32, scale: u32) -> i32 {
    if scale == 0 || duty_cycle > scale { return -22; }
    (*state).duty_cycle = ((duty_cycle as u64).wrapping_mul((*state).period) + scale as u64 / 2) / scale as u64;
    0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_capture { pub period: u32, pub duty_cycle: u32 }

pub const PWM_WFHWSIZE: usize = 20;

#[repr(C)]
pub struct pwm_ops {
    pub request: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device) -> i32>,
    pub free: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device)>,
    pub capture: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device, *mut pwm_capture, usize) -> i32>,
    pub sizeof_wfhw: usize,
    pub round_waveform_tohw: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device, *const pwm_waveform, *mut core::ffi::c_void) -> i32>,
    pub round_waveform_fromhw: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device, *const core::ffi::c_void, *mut pwm_waveform) -> i32>,
    pub read_waveform: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device, *mut core::ffi::c_void) -> i32>,
    pub write_waveform: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device, *const core::ffi::c_void) -> i32>,
    pub apply: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device, *const pwm_state) -> i32>,
    pub get_state: Option<unsafe extern "C" fn(*mut pwm_chip, *mut pwm_device, *mut pwm_state) -> i32>,
}

#[repr(C)]
pub struct pwm_chip {
    pub dev: device,
    pub cdev: cdev,
    pub ops: *const pwm_ops,
    pub owner: *mut module,
    pub id: u32,
    pub npwm: u32,
    pub of_xlate: Option<unsafe extern "C" fn(*mut pwm_chip, *const of_phandle_args) -> *mut pwm_device>,
    pub atomic: bool,
    pub gpio: gpio_chip,
    pub uses_pwmchip_alloc: bool,
    pub operational: bool,
    pub lock: pwm_chip_lock,
    pub pwms: [pwm_device; 0],
}

#[repr(C)]
pub union pwm_chip_lock {
    pub nonatomic_lock: mutex,
    pub atomic_lock: spinlock_t,
}

/* The following kernel-owned types are supplied by dependencies. */
pub struct device;
pub struct cdev;
pub struct module;
pub struct of_phandle_args;
pub struct gpio_chip;
pub struct mutex;
pub struct spinlock_t;
pub struct fwnode_handle;

#[inline]
pub unsafe fn pwmchip_supports_waveform(chip: *mut pwm_chip) -> bool {
    (*(*chip).ops).write_waveform.is_some()
}

extern "C" {
    pub fn pwmchip_alloc(parent: *mut device, npwm: u32, sizeof_priv: usize) -> *mut pwm_chip;
    pub fn devm_pwmchip_alloc(parent: *mut device, npwm: u32, sizeof_priv: usize) -> *mut pwm_chip;
    pub fn __pwmchip_add(chip: *mut pwm_chip, owner: *mut module) -> i32;
    pub fn pwmchip_remove(chip: *mut pwm_chip);
    pub fn __devm_pwmchip_add(dev: *mut device, chip: *mut pwm_chip, owner: *mut module) -> i32;
    pub fn of_pwm_xlate_with_flags(chip: *mut pwm_chip, args: *const of_phandle_args) -> *mut pwm_device;
    pub fn of_pwm_single_xlate(chip: *mut pwm_chip, args: *const of_phandle_args) -> *mut pwm_device;
    pub fn pwm_get(dev: *mut device, con_id: *const core::ffi::c_char) -> *mut pwm_device;
    pub fn pwm_put(pwm: *mut pwm_device);
    pub fn devm_pwm_get(dev: *mut device, con_id: *const core::ffi::c_char) -> *mut pwm_device;
    pub fn devm_fwnode_pwm_get(dev: *mut device, fwnode: *mut fwnode_handle, con_id: *const core::ffi::c_char) -> *mut pwm_device;
}

/* The remaining kernel-owned structures and APIs are declarations supplied by dependencies. */
extern "C" {
    pub fn pwm_round_waveform_might_sleep(pwm: *mut pwm_device, wf: *mut pwm_waveform) -> i32;
    pub fn pwm_get_waveform_might_sleep(pwm: *mut pwm_device, wf: *mut pwm_waveform) -> i32;
    pub fn pwm_set_waveform_might_sleep(pwm: *mut pwm_device, wf: *const pwm_waveform, exact: bool) -> i32;
    pub fn pwm_apply_might_sleep(pwm: *mut pwm_device, state: *const pwm_state) -> i32;
    pub fn pwm_apply_atomic(pwm: *mut pwm_device, state: *const pwm_state) -> i32;
    pub fn pwm_get_state_hw(pwm: *mut pwm_device, state: *mut pwm_state) -> i32;
    pub fn pwm_adjust_config(pwm: *mut pwm_device) -> i32;
    pub fn pwmchip_put(chip: *mut pwm_chip);
    pub fn pwmchip_release(dev: *mut core::ffi::c_void);
    pub fn pwm_add_table(table: *mut pwm_lookup, num: usize);
    pub fn pwm_remove_table(table: *mut pwm_lookup, num: usize);
}

#[repr(C)]
pub struct pwm_lookup {
    pub list: *mut core::ffi::c_void,
    pub provider: *const core::ffi::c_char,
    pub index: u32,
    pub dev_id: *const core::ffi::c_char,
    pub con_id: *const core::ffi::c_char,
    pub period: u32,
    pub polarity: pwm_polarity,
    pub module: *const core::ffi::c_char,
}

/* PWM_LOOKUP_WITH_MODULE and PWM_LOOKUP are C initializer macros; use struct literals. */
#[macro_export]
macro_rules! PWM_LOOKUP_WITH_MODULE {
    ($provider:expr, $index:expr, $dev_id:expr, $con_id:expr, $period:expr, $polarity:expr, $module:expr) => {
        pwm_lookup { list: core::ptr::null_mut(), provider: $provider, index: $index, dev_id: $dev_id, con_id: $con_id, period: $period, polarity: $polarity, module: $module }
    };
}
#[macro_export]
macro_rules! PWM_LOOKUP {
    ($provider:expr, $index:expr, $dev_id:expr, $con_id:expr, $period:expr, $polarity:expr) => {
        $crate::PWM_LOOKUP_WITH_MODULE!($provider, $index, $dev_id, $con_id, $period, $polarity, core::ptr::null())
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
