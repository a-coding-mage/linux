/* SPDX-License-Identifier: GPL-2.0 */

pub const TRACE_SYSTEM: &str = "hda_intel";
pub const TRACE_INCLUDE_FILE: &str = "intel_trace";

/* Original C dependency:
 * #include <linux/tracepoint.h>
 */

#[repr(C)]
pub struct hda_pm_entry {
    pub dev_index: ::core::ffi::c_int,
}

/*
 * DECLARE_EVENT_CLASS(hda_pm,
 *     TP_PROTO(struct azx *chip),
 *     TP_ARGS(chip),
 *     TP_STRUCT__entry(__field(int, dev_index)),
 *     TP_fast_assign(__entry->dev_index = (chip)->dev_index;),
 *     TP_printk("card index: %d", __entry->dev_index)
 * );
 *
 * The event class records chip->dev_index and prints it as:
 * "card index: %d".
 */

unsafe extern "C" {
    pub fn trace_azx_suspend(chip: *mut azx);
    pub fn trace_azx_resume(chip: *mut azx);
    pub fn trace_azx_runtime_suspend(chip: *mut azx);
    pub fn trace_azx_runtime_resume(chip: *mut azx);
}

/*
 * These events are DEFINE_EVENT instances of the hda_pm event class:
 * - azx_suspend
 * - azx_resume
 * - azx_runtime_suspend
 * - azx_runtime_resume
 */

pub const TRACE_INCLUDE_PATH: &str = ".";

/* Original C dependency outside the header guard:
 * #include <trace/define_trace.h>
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
