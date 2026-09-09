/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding tracing implementation:
// linux/trace_events.h, linux/trace_remote_event.h, linux/trace_seq.h,
// and linux/stringify.h.

// #define REMOTE_EVENT_INCLUDE(__file) __stringify(../../__file)
#[macro_export]
macro_rules! remote_event_include {
    ($file:ident) => { concat!("../../", stringify!($file)) };
}

// If REMOTE_EVENT_SECTION is defined by the build, the generated event is
// placed in REMOTE_EVENT_SECTION.<name> and marked used. Rust section and
// used attributes are supplied by the surrounding build when applicable.

// #define REMOTE_PRINTK_COUNT_ARGS(__args...)
// The original uses the kernel's __COUNT_ARGS helper to select 0, 1, or 2.
#[macro_export]
macro_rules! remote_printk_count_args {
    () => { 0 };
    ($fmt:expr) => { 1 };
    ($fmt:expr, $($args:tt)+) => { 2 };
}

#[macro_export]
macro_rules! __remote_printk0 {
    ($seq:expr) => {{
        trace_seq_putc($seq, b'\n' as _);
    }};
}

#[macro_export]
macro_rules! __remote_printk1 {
    ($seq:expr, $fmt:expr) => {{
        trace_seq_puts($seq, concat!(" ", $fmt, "\n"));
    }};
}

#[macro_export]
macro_rules! __remote_printk2 {
    ($seq:expr, $fmt:expr $(, $args:expr)*) => {{
        trace_seq_putc($seq, b' ' as _);
        trace_seq_printf($seq, $fmt $(, $args)*);
        trace_seq_putc($seq, b'\n' as _);
    }};
}

// Apply the appropriate trace_seq sequence according to the number of arguments.
#[macro_export]
macro_rules! remote_printk {
    ($seq:expr) => { $crate::__remote_printk0!($seq) };
    ($seq:expr, $fmt:expr) => { $crate::__remote_printk1!($seq, $fmt) };
    ($seq:expr, $fmt:expr, $($args:expr),+) => {
        $crate::__remote_printk2!($seq, $fmt, $($args),+)
    };
}

#[macro_export]
macro_rules! re_printk {
    ($($args:tt)*) => { $($args)* };
}

// First inclusion pass: event-format declarations and print functions are
// emitted by the externally supplied REMOTE_EVENT_INCLUDE_FILE definitions.
#[macro_export]
macro_rules! remote_event {
    ($name:ident, $id:expr, $structure:tt, $printk:tt) => {
        // struct remote_event_format_$name is supplied by the event format
        // implementation; the C source creates its print function here.
    };
}

// Second inclusion pass: field descriptors, print format strings, and remote
// event registrations are emitted by the externally supplied definitions.
#[macro_export]
macro_rules! re_field {
    ($type:ty, $field:ident) => {
        trace_event_fields {
            type_name: stringify!($type),
            name: stringify!($field),
            size: core::mem::size_of::<$type>(),
            align: core::mem::align_of::<$type>(),
            is_signed: is_signed_type::<$type>(),
        }
    };
}

// C's __entry alias used by event structure declarations.
// The surrounding event-definition translation supplies the corresponding REC value.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
