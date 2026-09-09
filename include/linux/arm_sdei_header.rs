// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2017 Arm Ltd.

// C dependencies: <uapi/linux/arm_sdei.h>, <acpi/ghes.h>, and, when
// CONFIG_ARM_SDE_INTERFACE is enabled, <asm/sdei.h>.

/* Arch code should override this to set the entry point from firmware... */
#[inline]
pub fn sdei_arch_get_entry_point<T>(_conduit: T) -> i32 {
    0
}

/*
 * When an event occurs sdei_event_handler() will call a user-provided callback
 * like this in NMI context on the CPU that received the event.
 */
pub type SdeiEventCallback = unsafe extern "C" fn(
    event: u32,
    regs: *mut PtRegs,
    arg: *mut core::ffi::c_void,
) -> i32;

/* External types supplied by the included kernel headers. */
#[repr(C)]
pub struct PtRegs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Ghes {
    _private: [u8; 0],
}

/*
 * Register your callback to claim an event. The event must be described
 * by firmware.
 */
unsafe extern "C" {
    pub fn sdei_event_register(
        event_num: u32,
        cb: Option<SdeiEventCallback>,
        arg: *mut core::ffi::c_void,
    ) -> i32;

    /*
     * Calls to sdei_event_unregister() may return EINPROGRESS. Keep calling
     * it until it succeeds.
     */
    pub fn sdei_event_unregister(event_num: u32) -> i32;
    pub fn sdei_event_enable(event_num: u32) -> i32;
    pub fn sdei_event_disable(event_num: u32) -> i32;

    /*
     * Signal the software-signalled event (event 0) to another PE, NMI-like.
     * @mpidr is the target's MPIDR affinity.
     */
    pub fn sdei_event_signal(event_num: u32, mpidr: u64) -> i32;

    /* Was SDEI firmware probed and usable? */
    pub fn sdei_is_present() -> bool;

    /* GHES register/unregister helpers */
    pub fn sdei_register_ghes(
        ghes: *mut Ghes,
        normal_cb: Option<SdeiEventCallback>,
        critical_cb: Option<SdeiEventCallback>,
    ) -> i32;
    pub fn sdei_unregister_ghes(ghes: *mut Ghes) -> i32;
}

#[cfg(feature = "CONFIG_ARM_SDE_INTERFACE")]
unsafe extern "C" {
    /* For use by arch code when CPU hotplug notifiers are not appropriate. */
    pub fn sdei_mask_local_cpu() -> i32;
    pub fn sdei_unmask_local_cpu() -> i32;
    pub fn acpi_sdei_init();
    pub fn sdei_handler_abort();
}

#[cfg(not(feature = "CONFIG_ARM_SDE_INTERFACE"))]
#[inline]
pub fn sdei_mask_local_cpu() -> i32 { 0 }

#[cfg(not(feature = "CONFIG_ARM_SDE_INTERFACE"))]
#[inline]
pub fn sdei_unmask_local_cpu() -> i32 { 0 }

#[cfg(not(feature = "CONFIG_ARM_SDE_INTERFACE"))]
#[inline]
pub fn acpi_sdei_init() {}

#[cfg(not(feature = "CONFIG_ARM_SDE_INTERFACE"))]
#[inline]
pub fn sdei_handler_abort() {}

/*
 * This struct represents an event that has been registered. The driver
 * maintains a list of all events, and which ones are registered. (Private
 * events have one entry in the list, but are registered on each CPU).
 * A pointer to this struct is passed to firmware, and back to the event
 * handler. The event handler can then use this to invoke the registered
 * callback, without having to walk the list.
 *
 * For CPU private events, this structure is per-cpu.
 */
#[repr(C)]
pub struct SdeiRegisteredEvent {
    /* For use by arch code: */
    pub interrupted_regs: PtRegs,
    pub callback: Option<SdeiEventCallback>,
    pub callback_arg: *mut core::ffi::c_void,
    pub event_num: u32,
    pub priority: u8,
}

/* The arch code entry point should then call this when an event arrives. */
unsafe extern "C" {
    pub fn sdei_event_handler(
        regs: *mut PtRegs,
        arg: *mut SdeiRegisteredEvent,
    ) -> i32;

    /* arch code may use this to retrieve the extra registers. */
    pub fn sdei_api_event_context(query: u32, result: *mut u64) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
