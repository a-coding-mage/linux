/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace-events-sample.h.
// Linux tracepoint headers and their TRACE_* DSL are external dependencies.

pub const TRACE_SYSTEM: &str = "sample-trace";
pub const TRACE_SYSTEM_VAR: &str = "sample_trace";

/// Equivalent of the C helper `__length_of`.
pub unsafe fn __length_of(list: *const core::ffi::c_int) -> core::ffi::c_int {
    if list.is_null() {
        return 0;
    }
    let mut i: core::ffi::c_int = 0;
    while *list.add(i as usize) != 0 {
        i = i.wrapping_add(1);
    }
    i
}

pub const TRACE_SAMPLE_FOO: i32 = 2;
pub const TRACE_SAMPLE_BAR: i32 = 4;
pub const TRACE_SAMPLE_ZOO: i32 = 8;

// `struct timer_list` and the percpu pointer are supplied by Linux headers.
#[repr(C)]
pub struct foo_timer_data {
    pub name: *const core::ffi::c_char,
    pub timer: timer_list,
    pub counter: *mut core::ffi::c_int,
}

extern "C" {
    pub fn foo_bar_reg();
    pub fn foo_bar_unreg();
}

// The following declarations are Linux tracepoint DSL definitions.  They are
// retained verbatim in semantic form; TRACE_DEFINE_ENUM, TRACE_EVENT,
// TRACE_EVENT_CONDITION, TRACE_EVENT_FN, DECLARE_EVENT_CLASS, DEFINE_EVENT,
// DEFINE_EVENT_CONDITION, DEFINE_EVENT_FN, and DEFINE_EVENT_PRINT are provided
// by the external tracepoint implementation.
//
// TRACE_DEFINE_ENUM(TRACE_SAMPLE_FOO);
// TRACE_DEFINE_ENUM(TRACE_SAMPLE_BAR);
// TRACE_DEFINE_ENUM(TRACE_SAMPLE_ZOO);
//
// TRACE_EVENT(foo_bar,
//     TP_PROTO(const char *foo, int bar, const int *lst,
//              const char *string, const struct cpumask *mask,
//              const char *fmt, va_list *va),
//     TP_ARGS(foo, bar, lst, string, mask, fmt, va),
//     TP_STRUCT__entry(
//         __array(char, foo, 10)
//         __field(int, bar)
//         __dynamic_array(int, list, __length_of(lst))
//         __string(str, string)
//         __bitmask(cpus, num_possible_cpus())
//         __cpumask(cpum)
//         __vstring(vstr, fmt, va)
//         __string_len(lstr, foo, bar / 2 < strlen(foo) ? bar / 2 : strlen(foo))
//     ),
//     TP_fast_assign(
//         strscpy(__entry->foo, foo, 10);
//         __entry->bar = bar;
//         memcpy(__get_dynamic_array(list), lst, __length_of(lst) * sizeof(int));
//         __assign_str(str);
//         __assign_str(lstr);
//         __assign_vstr(vstr, fmt, va);
//         __assign_bitmask(cpus, cpumask_bits(mask), num_possible_cpus());
//         __assign_cpumask(cpum, cpumask_bits(mask));
//     ),
//     TP_printk("foo %s %d %s %s %s %s %s %s (%s) (%s) %s [%d] %*pbl", ...)
// );
//
// TRACE_EVENT_CONDITION(foo_bar_with_cond, ... TP_CONDITION(!(bar % 10)) ...);
// TRACE_EVENT_FN(foo_bar_with_fn, ... foo_bar_reg, foo_bar_unreg);
// DECLARE_EVENT_CLASS(foo_template, ...);
// DEFINE_EVENT(foo_template, foo_with_template_simple, ...);
// DEFINE_EVENT_CONDITION(foo_template, foo_with_template_cond, ... TP_CONDITION(!(bar % 8)));
// DEFINE_EVENT_FN(foo_template, foo_with_template_fn, ... foo_bar_reg, foo_bar_unreg);
// DEFINE_EVENT_PRINT(foo_template, foo_with_template_print, ...);
// TRACE_EVENT(foo_rel_loc, ...);
// TRACE_EVENT(foo_timer_fn, ...);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
