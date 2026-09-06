// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor task related definitions and mediation
//
// Copyright 2017 Canonical Ltd.

// Dependencies: <linux/sched.h>, audit.h, label.h

// Opaque external types
pub enum aa_label {}
pub enum callback_head {}
pub enum task_struct {}
pub enum cred {}
pub enum aa_profile {}
pub enum apparmor_audit_data {}

#[repr(C)]
pub struct AppArmorBlobSizes {
    pub lbs_task: usize,
}

extern "C" {
    pub static apparmor_blob_sizes: AppArmorBlobSizes;

    pub fn aa_replace_current_label(label: *mut aa_label) -> i32;
    pub fn aa_schedule_stale_label_replacement();
    pub fn aa_set_current_onexec(label: *mut aa_label, stack: bool);
    pub fn aa_set_current_hat(label: *mut aa_label, token: u64) -> i32;
    pub fn aa_restore_previous_label(cookie: u64) -> i32;
    pub fn aa_get_task_label(task: *mut task_struct) -> *mut aa_label;
    pub fn aa_put_label(label: *mut aa_label);
    pub fn aa_get_label(label: *mut aa_label) -> *mut aa_label;
    pub fn aa_may_ptrace(
        tracer_cred: *const cred,
        tracer: *mut aa_label,
        tracee_cred: *const cred,
        tracee: *mut aa_label,
        request: u32,
    ) -> i32;
    pub fn aa_profile_ns_perm(
        profile: *mut aa_profile,
        ad: *mut apparmor_audit_data,
        request: u32,
    ) -> i32;
}

#[repr(C)]
pub struct aa_task_ctx {
    pub nnp: *mut aa_label,
    pub onexec: *mut aa_label,
    pub previous: *mut aa_label,
    pub token: u64,
    pub label_replacement_tw: callback_head,
    pub label_replacement_pending: bool,
}

#[inline]
pub fn task_ctx(task: *mut task_struct) -> *mut aa_task_ctx {
    unsafe {
        ((*task).security as *mut u8).add(apparmor_blob_sizes.lbs_task) as *mut aa_task_ctx
    }
}

/// Free a task_ctx.
///
/// # Arguments
/// * `ctx` - task_ctx to free (MAY BE NULL)
#[inline]
pub fn aa_free_task_ctx(ctx: *mut aa_task_ctx) {
    if !ctx.is_null() {
        unsafe {
            aa_put_label((*ctx).nnp);
            aa_put_label((*ctx).previous);
            aa_put_label((*ctx).onexec);
        }
    }
}

/// Duplicate a task context, incrementing reference counts.
///
/// # Arguments
/// * `new` - a blank task context (NOT NULL)
/// * `old` - the task context to copy (NOT NULL)
#[inline]
pub fn aa_dup_task_ctx(new: *mut aa_task_ctx, old: *const aa_task_ctx) {
    unsafe {
        (*new).nnp = aa_get_label((*old).nnp);
        (*new).onexec = aa_get_label((*old).onexec);
        (*new).previous = aa_get_label((*old).previous);
        (*new).token = (*old).token;
    }
}

/// Clear transition tracking info from the ctx.
///
/// # Arguments
/// * `ctx` - task context to clear (NOT NULL)
#[inline]
pub fn aa_clear_task_ctx_trans(ctx: *mut aa_task_ctx) {
    // AA_BUG(!ctx) check - ctx must not be null
    debug_assert!(!ctx.is_null());

    unsafe {
        aa_put_label((*ctx).previous);
        aa_put_label((*ctx).onexec);
        (*ctx).previous = std::ptr::null_mut();
        (*ctx).onexec = std::ptr::null_mut();
        (*ctx).token = 0;
    }
}

// Ptrace permission constants
// References: MAY_WRITE and MAY_READ from linux/fs.h, AA_MAY_APPEND and AA_MAY_CREATE from apparmor audit definitions
// These constants would need to be imported from their respective source files
// pub const AA_PTRACE_TRACE: u32 = MAY_WRITE;
// pub const AA_PTRACE_READ: u32 = MAY_READ;
// pub const AA_MAY_BE_TRACED: u32 = AA_MAY_APPEND;
// pub const AA_MAY_BE_READ: u32 = AA_MAY_CREATE;

pub const PTRACE_PERM_SHIFT: u32 = 2;

// Composed permissions requiring external constants:
// pub const AA_PTRACE_PERM_MASK: u32 = AA_PTRACE_READ | AA_PTRACE_TRACE | AA_MAY_BE_READ | AA_MAY_BE_TRACED;
// pub const AA_SIGNAL_PERM_MASK: u32 = MAY_READ | MAY_WRITE;

pub const AA_SFS_SIG_MASK: &str = "hup int quit ill trap abrt bus fpe kill usr1 \
    segv usr2 pipe alrm term stkflt chld cont stop stp ttin ttou urg \
    xcpu xfsz vtalrm prof winch io pwr sys emt lost";

pub const AA_USERNS_CREATE: u32 = 8;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
