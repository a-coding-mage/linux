/* SPDX-License-Identifier: GPL-2.0-or-later */

// Translation of trace/events/pwm.h.  The Linux tracepoint-generation macros
// and the types declared by the included headers are supplied externally.

#[allow(non_camel_case_types)]
pub type u64_t = u64;

#[repr(C)]
pub struct pwm_device {
    pub chip: *mut pwm_chip,
    pub hwpwm: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct pwm_chip {
    pub id: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct pwm_waveform {
    pub period_length_ns: u64,
    pub duty_length_ns: u64,
    pub duty_offset_ns: u64,
}

#[repr(C)]
pub struct pwm_state {
    pub period: u64,
    pub duty_cycle: u64,
    pub polarity: pwm_polarity,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pwm_polarity {
    Normal = 0,
    Inversed = 1,
}

#[repr(C)]
pub struct pwm_round_waveform_tohw_entry {
    pub chipid: ::core::ffi::c_uint,
    pub hwpwm: ::core::ffi::c_uint,
    pub wf_period_length_ns: u64,
    pub wf_duty_length_ns: u64,
    pub wf_duty_offset_ns: u64,
    pub wfhw: *mut ::core::ffi::c_void,
    pub err: ::core::ffi::c_int,
}

#[repr(C)]
pub struct pwm_round_waveform_fromhw_entry {
    pub chipid: ::core::ffi::c_uint,
    pub hwpwm: ::core::ffi::c_uint,
    pub wfhw: *const ::core::ffi::c_void,
    pub wf_period_length_ns: u64,
    pub wf_duty_length_ns: u64,
    pub wf_duty_offset_ns: u64,
    pub err: ::core::ffi::c_int,
}

#[repr(C)]
pub struct pwm_read_waveform_entry {
    pub chipid: ::core::ffi::c_uint,
    pub hwpwm: ::core::ffi::c_uint,
    pub wfhw: *mut ::core::ffi::c_void,
    pub err: ::core::ffi::c_int,
}

#[repr(C)]
pub struct pwm_write_waveform_entry {
    pub chipid: ::core::ffi::c_uint,
    pub hwpwm: ::core::ffi::c_uint,
    pub wfhw: *const ::core::ffi::c_void,
    pub err: ::core::ffi::c_int,
}

#[repr(C)]
pub struct pwm_entry {
    pub chipid: ::core::ffi::c_uint,
    pub hwpwm: ::core::ffi::c_uint,
    pub period: u64,
    pub duty_cycle: u64,
    pub polarity: pwm_polarity,
    pub enabled: bool,
    pub err: ::core::ffi::c_int,
}

// TP_PROTO_pwm(args...) expands to:
// TP_PROTO(struct pwm_device *pwm, args)
// TP_ARGS_pwm(args...) expands to: TP_ARGS(pwm, args)
// TP_STRUCT__entry_pwm(args...) adds chipid and hwpwm fields.
// TP_fast_assign_pwm(args...) assigns pwm->chip->id and pwm->hwpwm first.
// TP_printk_pwm(fmt, args...) prefixes output with "pwmchip%u.%u: ".
// __field_pwmwf(wf), fast_assign_pwmwf(wf), printk_pwmwf_format(wf), and
// printk_pwmwf_formatargs(wf) are represented by the waveform fields above.

// TRACE_EVENT(pwm_round_waveform_tohw)
//   TP_PROTO_pwm(const struct pwm_waveform *wf, void *wfhw, int err)
//   printk: "%lld/%lld [+%lld] > %p err=%d"

// TRACE_EVENT(pwm_round_waveform_fromhw)
//   TP_PROTO_pwm(const void *wfhw, struct pwm_waveform *wf, int err)
//   printk: "%p > %lld/%lld [+%lld] err=%d"

// TRACE_EVENT(pwm_read_waveform)
//   TP_PROTO_pwm(void *wfhw, int err)
//   printk: "%p err=%d"

// TRACE_EVENT(pwm_write_waveform)
//   TP_PROTO_pwm(const void *wfhw, int err)
//   printk: "%p err=%d"

// DECLARE_EVENT_CLASS(pwm)
//   TP_PROTO(struct pwm_device *pwm, const struct pwm_state *state, int err)
//   printk: "period=%llu duty_cycle=%llu polarity=%d enabled=%d err=%d"

// DEFINE_EVENT(pwm, pwm_apply)
// DEFINE_EVENT(pwm, pwm_get)


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
