/* SPDX-License-Identifier: GPL-2.0 */

// C header intent:
// #define TRACE_SYSTEM gpio
// The tracepoint declarations below depend on the Linux tracepoint
// infrastructure supplied by the including build environment.

#[repr(C)]
pub struct GpioDirectionEntry {
    pub gpio: core::ffi::c_uint,
    pub in_: core::ffi::c_int,
    pub err: core::ffi::c_int,
}

#[repr(C)]
pub struct GpioValueEntry {
    pub gpio: core::ffi::c_uint,
    pub get: core::ffi::c_int,
    pub value: core::ffi::c_int,
}

// TRACE_EVENT(gpio_direction)
// TP_PROTO(unsigned gpio, int in, int err)
// TP_ARGS(gpio, in, err)
// TP_fast_assign:
//     __entry->gpio = gpio;
//     __entry->in = in;
//     __entry->err = err;
// TP_printk("%u %3s (%d)", __entry->gpio,
//     __entry->in ? "in" : "out", __entry->err)

// TRACE_EVENT(gpio_value)
// TP_PROTO(unsigned gpio, int get, int value)
// TP_ARGS(gpio, get, value)
// TP_fast_assign:
//     __entry->gpio = gpio;
//     __entry->get = get;
//     __entry->value = value;
// TP_printk("%u %3s %d", __entry->gpio,
//     __entry->get ? "get" : "set", __entry->value)

extern "C" {
    pub fn trace_gpio_direction(
        gpio: core::ffi::c_uint,
        in_: core::ffi::c_int,
        err: core::ffi::c_int,
    );

    pub fn trace_gpio_value(
        gpio: core::ffi::c_uint,
        get: core::ffi::c_int,
        value: core::ffi::c_int,
    );
}

// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
