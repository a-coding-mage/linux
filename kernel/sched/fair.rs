// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2007 Red Hat, Inc., Ingo Molnar; IBM Corporation.
// Rust source owner for kernel/sched/fair.c. Native ABI comes from the configured
// original scheduler headers and extracted private declarations, never replicas.
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// RECONCILED-SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
#![no_std]
// Generated native ABI declarations inherit only the existing bindings crate
// policy; handwritten scheduler code retains the kernel's ordinary diagnostics.
use kernel::bindings::sched_fair_native as b;
use b::fair_sched_class;
use core::cmp::{max, min};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_long, c_uint, c_ulong};
include!("fair_once.rs");
macro_rules! feat {
    ($name:ident) => {
        b::$name()
    };
}
macro_rules! read_once {
    ($value:expr) => {
        FairOnce::read_once(addr_of!($value))
    };
}
macro_rules! write_once {
    ($value:expr, $new:expr) => {
        FairOnce::write_once(addr_of_mut!($value), $new)
    };
}
include!("fair_foundation.rs");
include!("fair_tail.rs");
