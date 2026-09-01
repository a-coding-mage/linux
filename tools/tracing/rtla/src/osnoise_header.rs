// SPDX-License-Identifier: GPL-2.0

// Depends on declarations translated from "common.h".

#[repr(C)]
pub enum osnoise_mode {
    MODE_OSNOISE = 0,
    MODE_HWNOISE,
}

#[repr(C)]
pub struct osnoise_params {
    pub common: common_params,
    pub runtime: ::std::os::raw::c_ulonglong,
    pub period: ::std::os::raw::c_ulonglong,
    pub threshold: ::std::os::raw::c_longlong,
    pub mode: osnoise_mode,
}

// C macro:
// #define to_osnoise_params(ptr) container_of(ptr, struct osnoise_params, common)
// Requires the external container_of-equivalent supplied by translated dependencies.

/*
 * *_INIT_VALs are also invalid values, they are used to
 * communicate errors.
 */
pub const OSNOISE_OPTION_INIT_VAL: ::std::os::raw::c_int = -1;
pub const OSNOISE_TIME_INIT_VAL: ::std::os::raw::c_int = 0;

unsafe extern "C" {
    pub fn osnoise_context_alloc() -> *mut osnoise_context;
    pub fn osnoise_get_context(context: *mut osnoise_context) -> ::std::os::raw::c_int;
    pub fn osnoise_put_context(context: *mut osnoise_context);

    pub fn osnoise_set_runtime_period(
        context: *mut osnoise_context,
        runtime: ::std::os::raw::c_ulonglong,
        period: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
    pub fn osnoise_restore_runtime_period(context: *mut osnoise_context);

    pub fn osnoise_restore_stop_us(context: *mut osnoise_context);
    pub fn osnoise_restore_stop_total_us(context: *mut osnoise_context);

    pub fn osnoise_set_timerlat_period_us(
        context: *mut osnoise_context,
        timerlat_period_us: ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int;
    pub fn osnoise_restore_timerlat_period_us(context: *mut osnoise_context);

    pub fn osnoise_set_tracing_thresh(
        context: *mut osnoise_context,
        tracing_thresh: ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int;
    pub fn osnoise_restore_tracing_thresh(context: *mut osnoise_context);

    pub fn osnoise_restore_print_stack(context: *mut osnoise_context);
    pub fn osnoise_set_print_stack(
        context: *mut osnoise_context,
        print_stack: ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int;

    pub fn osnoise_set_timerlat_align_us(
        context: *mut osnoise_context,
        timerlat_align_us: ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int;
    pub fn osnoise_restore_timerlat_align_us(context: *mut osnoise_context);

    pub fn osnoise_set_timerlat_align(
        context: *mut osnoise_context,
        onoff: bool,
    ) -> ::std::os::raw::c_int;

    pub fn osnoise_set_irq_disable(
        context: *mut osnoise_context,
        onoff: bool,
    ) -> ::std::os::raw::c_int;
    pub fn osnoise_report_missed_events(tool: *mut osnoise_tool);
    pub fn osnoise_apply_config(
        tool: *mut osnoise_tool,
        params: *mut osnoise_params,
    ) -> ::std::os::raw::c_int;

    pub fn osnoise_enable(tool: *mut osnoise_tool) -> ::std::os::raw::c_int;
    pub fn osnoise_main(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn hwnoise_main(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub static mut timerlat_top_ops: tool_ops;
    pub static mut timerlat_hist_ops: tool_ops;
    pub static mut osnoise_top_ops: tool_ops;
    pub static mut osnoise_hist_ops: tool_ops;

    pub fn run_tool(
        ops: *mut tool_ops,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
