// SPDX-License-Identifier: GPL-2.0
// Pressure stall information for CPU, memory and IO.
// Original C owner: Copyright (c) 2018 Facebook, Inc.
// Author: Johannes Weiner <hannes@cmpxchg.org>
// Polling support: Suren Baghdasaryan <surenb@google.com>
// Copyright (c) 2018 Google, Inc.
// Existing psi.rs owner completed against psi.c at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. SOURCE ONLY, unqualified.
//! Pressure stall information using native configured types and primitives.
//! Native layouts, synchronization, instrumentation and callback ABI still need
//! qualification. All PSI state decisions and accounting are owned by Rust.
#![no_std]
compile_error!("Lupos PSI source proposal is not build-admitted; qualification is incomplete");

use core::cmp::{max, min};
use core::mem::MaybeUninit;
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use kernel::bindings::sched_psi_native as b;

const STATES: usize = b::RUST_PSI_NR_STATES as usize;
const COUNTS: usize = b::RUST_PSI_NR_TASK_COUNTS as usize;
const AVGS: usize = b::RUST_PSI_AVGS as usize;
const POLL: usize = b::RUST_PSI_POLL as usize;
const NONIDLE: usize = b::RUST_PSI_NONIDLE as usize;
const ONCPU: u32 = b::RUST_PSI_ONCPU as u32;
const TSK_ONCPU: c_uint = b::RUST_PSI_TSK_ONCPU as c_uint;
const FREQ: c_ulong = b::RUST_PSI_FREQ as c_ulong;
const UPDATES_PER_WINDOW: u32 = b::RUST_PSI_UPDATES_PER_WINDOW as u32;

macro_rules! container_of {
    ($ptr:expr, $ty:ty, $field:ident) => {
        ($ptr as *mut u8).sub(core::mem::offset_of!($ty, $field)) as *mut $ty
    };
}

/// # Safety
/// Called only by the native boot parser with its writable NUL-terminated value.
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_psi_setup(value: *mut c_char) -> c_int {
    unsafe { (b::rust_psi_kstrtobool(value, addr_of_mut!(b::rust_psi_enable)) == 0) as c_int }
}

/// # Safety
/// Group storage is zero-initialized and unpublished, with valid native pcpu.
unsafe fn group_init(group: *mut b::psi_group) {
    unsafe {
        (*group).enabled = true;
        (*group).avg_last_update = b::rust_psi_sched_clock();
        (*group).avg_next_update = (*group).avg_last_update.wrapping_add(b::rust_psi_period);
        b::rust_psi_mutex_init_avgs(group);
        b::rust_psi_init_list(addr_of_mut!((*group).avg_triggers));
        for s in 0..STATES - 1 { (*group).avg_nr_triggers[s] = 0; }
        b::rust_psi_init_avgs_work(group);
        b::rust_psi_atomic_set(addr_of_mut!((*group).rtpoll_scheduled), 0);
        b::rust_psi_mutex_init_rtpoll(group);
        b::rust_psi_init_list(addr_of_mut!((*group).rtpoll_triggers));
        (*group).rtpoll_min_period = b::RUST_PSI_U32_MAX as u64;
        (*group).rtpoll_next_update = b::RUST_PSI_U64_MAX as u64;
        b::rust_psi_init_rtpoll_wait(group);
        b::rust_psi_init_timer(group);
        b::rust_psi_assign_rtpoll_task(group, null_mut());
    }
}

/// # Safety
/// Boot-only initialization before PSI accounting starts.
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn psi_init() {
    unsafe {
        if !b::rust_psi_enable {
            b::rust_psi_disable();
            b::rust_psi_disable_cgroups();
            return;
        }
        if !b::rust_psi_cgroup_psi_enabled() { b::rust_psi_disable_cgroups(); }
        b::rust_psi_period = b::rust_psi_jiffies_to_nsecs(FREQ);
        group_init(addr_of_mut!(b::psi_system));
    }
}

/// # Safety
/// Mask storage and the callback's objects stay live for the whole iteration.
unsafe fn each_possible_cpu(mut visit: impl FnMut(c_int)) {
    unsafe {
        // include/linux/cpumask.h: NR_CPUS == 1 intentionally ignores the mask.
        if b::RUST_PSI_NR_CPUS == 1 {
            visit(0);
            return;
        }
        let mut cpu = b::rust_psi_first_possible_cpu();
        while cpu < b::rust_psi_nr_cpu_ids() {
            visit(cpu as c_int);
            cpu = b::rust_psi_next_possible_cpu(cpu);
        }
    }
}

/// # Safety
/// The caller holds the aggregator mutex that stabilizes the complete list.
unsafe fn each_trigger(head: *mut b::list_head, mut visit: impl FnMut(*mut b::psi_trigger)) {
    unsafe {
        let mut node = (*head).next;
        while node != head {
            let trigger = container_of!(node, b::psi_trigger, node);
            visit(trigger);
            node = (*node).next;
        }
    }
}

include!("psi_aggregation.rs");
include!("psi_tasks.rs");
include!("psi_triggers.rs");
#[cfg(CONFIG_PROC_FS)]
include!("psi_proc.rs");
