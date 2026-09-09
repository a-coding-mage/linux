/* SPDX-License-Identifier: MIT */
/******************************************************************************
 * callback.h
 *
 * Register guest OS callbacks with Xen.
 *
 * Copyright (c) 2006, Ian Campbell
 */

// C header guard: __XEN_PUBLIC_CALLBACK_H__
// Dependency: xen/interface/xen.h

/*
 * Prototype for this hypercall is:
 *   long callback_op(int cmd, void *extra_args)
 * @cmd        == CALLBACKOP_??? (callback operation).
 * @extra_args == Operation-specific extra arguments (NULL if none).
 */

/* x86: Callback for event delivery. */
pub const CALLBACKTYPE_event: u32 = 0;

/* x86: Failsafe callback when guest state cannot be restored by Xen. */
pub const CALLBACKTYPE_failsafe: u32 = 1;

/* x86/64 hypervisor: Syscall by 64-bit guest app ('64-on-64-on-64'). */
pub const CALLBACKTYPE_syscall: u32 = 2;

/*
 * x86/32 hypervisor: Only available on x86/32 when supervisor_mode_kernel
 *     feature is enabled. Do not use this callback type in new code.
 */
pub const CALLBACKTYPE_sysenter_deprecated: u32 = 3;

/* x86: Callback for NMI delivery. */
pub const CALLBACKTYPE_nmi: u32 = 4;

/*
 * x86: sysenter is only available as follows:
 * - 32-bit hypervisor: with the supervisor_mode_kernel feature enabled
 * - 64-bit hypervisor: 32-bit guest applications on Intel CPUs
 *                      ('32-on-32-on-64', '32-on-64-on-64')
 *                      [nb. also 64-bit guest applications on Intel CPUs
 *                           ('64-on-64-on-64'), but syscall is preferred]
 */
pub const CALLBACKTYPE_sysenter: u32 = 5;

/*
 * x86/64 hypervisor: Syscall by 32-bit guest app on AMD CPUs
 *                    ('32-on-32-on-64', '32-on-64-on-64')
 */
pub const CALLBACKTYPE_syscall32: u32 = 7;

/*
 * Disable event deliver during callback? This flag is ignored for event and
 * NMI callbacks: event delivery is unconditionally disabled.
 */
pub const _CALLBACKF_mask_events: u32 = 0;
pub const CALLBACKF_mask_events: u32 = 1u32 << _CALLBACKF_mask_events;

/*
 * Register a callback.
 */
pub const CALLBACKOP_register: u32 = 0;
#[repr(C)]
pub struct callback_register {
    pub type_: u16,
    pub flags: u16,
    pub address: xen_callback_t,
}

/*
 * Unregister a callback.
 *
 * Not all callbacks can be unregistered. -EINVAL will be returned if
 * you attempt to unregister such a callback.
 */
pub const CALLBACKOP_unregister: u32 = 1;
#[repr(C)]
pub struct callback_unregister {
    pub type_: u16,
    pub _unused: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
