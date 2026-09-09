/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor configuration macros retained as Rust macro equivalents.
macro_rules! REMOTE_EVENT_INCLUDE_FILE {
    () => { "arch/arm64/include/asm/kvm_hypevents.h" };
}

macro_rules! REMOTE_EVENT_SECTION {
    () => { "_hyp_events" };
}

macro_rules! HE_STRUCT {
    ($($args:tt)*) => { $($args)* };
}

macro_rules! HE_PRINTK {
    ($($args:tt)*) => { $($args)* };
}

macro_rules! he_field {
    () => { re_field };
}

macro_rules! HYP_EVENT {
    ($name:ident, $proto:tt, $struct:tt, $assign:tt, $printk:tt) => {
        REMOTE_EVENT!($name, 0, RE_STRUCT!($struct), RE_PRINTK!($printk));
    };
}

// C marker macro: HYP_EVENT_MULTI_READ
// The following include expands the remote-event declarations in the source
// build; its dependency is supplied externally.
// #include <trace/define_remote_events.h>
// #undef HYP_EVENT_MULTI_READ

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
