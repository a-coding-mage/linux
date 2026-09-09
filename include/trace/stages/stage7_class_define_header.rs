/* SPDX-License-Identifier: GPL-2.0 */

/* Stage 7 definitions for creating trace events */

/* C preprocessor state: __entry expands to REC. */
macro_rules! __entry {
    () => { REC };
}

/*
 * The TRACE_FLAG_* are enums. Instead of using TRACE_DEFINE_ENUM(),
 * use their hardcoded values. These values are parsed by user space
 * tooling elsewhere so they will never change.
 *
 * See "enum trace_flag_type" in linux/trace_events.h:
 *   TRACE_FLAG_HARDIRQ
 *   TRACE_FLAG_SOFTIRQ
 */

/* This is what is displayed in the format files */
macro_rules! __event_in_hardirq {
    () => { (REC.common_flags & 0x8) };
}

macro_rules! __event_in_softirq {
    () => { (REC.common_flags & 0x10) };
}

macro_rules! __event_in_irq {
    () => { (REC.common_flags & 0x18) };
}

/*
 * The below is not executed in the kernel. It is only what is
 * displayed in the print format for userspace to parse.
 */
macro_rules! __print_ns_to_secs {
    ($val:expr) => { ($val) / 1_000_000_000u32 };
}

macro_rules! __print_ns_without_secs {
    ($val:expr) => { ($val) % 1_000_000_000u32 };
}

/* TP_printk(fmt, args...) expands to a quoted format followed by stringified args. */
macro_rules! TP_printk {
    ($fmt:expr $(, $args:expr)*) => {
        (concat!("\"", $fmt, "\", "), stringify!($($args),*))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
