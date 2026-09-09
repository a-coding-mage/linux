// SPDX-License-Identifier: GPL-2.0
/* sstate.c: System soft state support.
 *
 * Copyright (C) 2007, 2008 David S. Miller <davem@davemloft.net>
 */

// Linux and architecture dependencies are supplied by other translated files.

static mut hv_supports_soft_state: ::core::ffi::c_int = 0;

unsafe fn do_set_sstate(state: ::core::ffi::c_ulong, msg: *const ::core::ffi::c_char) {
    let mut err: ::core::ffi::c_ulong;

    if hv_supports_soft_state == 0 {
        return;
    }

    err = sun4v_mach_set_soft_state(state, kimage_addr_to_ra(msg));
    if err != 0 {
        printk(
            KERN_WARNING.as_ptr() as *const ::core::ffi::c_char,
            state,
            msg,
            err,
        );
    }
}

#[repr(align(32))]
struct Aligned32([u8; 32]);

static booting_msg: Aligned32 = Aligned32(*b"Linux booting\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
static running_msg: Aligned32 = Aligned32(*b"Linux running\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
static halting_msg: Aligned32 = Aligned32(*b"Linux halting\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
static poweroff_msg: Aligned32 = Aligned32(*b"Linux powering off\0\0\0\0\0\0\0\0\0\0\0\0\0");
static rebooting_msg: Aligned32 = Aligned32(*b"Linux rebooting\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
static panicking_msg: Aligned32 = Aligned32(*b"Linux panicking\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");

unsafe extern "C" {
    static tlb_type: ::core::ffi::c_int;
    static hypervisor: ::core::ffi::c_int;
    static panic_notifier_list: notifier_head;
    static KERN_WARNING: [::core::ffi::c_char; 8];

    fn sun4v_mach_set_soft_state(state: ::core::ffi::c_ulong, msg: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn kimage_addr_to_ra(addr: *const ::core::ffi::c_char) -> ::core::ffi::c_ulong;
    fn printk(fmt: *const ::core::ffi::c_char, ...);
    fn sun4v_hvapi_register(group: ::core::ffi::c_ulong, major: ::core::ffi::c_ulong, minor: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn prom_sun4v_guest_soft_state();
    fn atomic_notifier_chain_register(head: *mut notifier_head, block: *mut notifier_block) -> ::core::ffi::c_int;
    fn register_reboot_notifier(block: *mut notifier_block) -> ::core::ffi::c_int;
}

#[repr(C)]
struct notifier_block {
    notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, ::core::ffi::c_ulong, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    next: *mut notifier_block,
    priority: ::core::ffi::c_int,
}

#[repr(C)]
struct notifier_head {
    head: *mut notifier_block,
}

unsafe extern "C" fn sstate_reboot_call(_np: *mut notifier_block, type_: ::core::ffi::c_ulong, _unused: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let msg: *const ::core::ffi::c_char;

    msg = match type_ {
        SYS_HALT => halting_msg.0.as_ptr() as *const _,
        SYS_POWER_OFF => poweroff_msg.0.as_ptr() as *const _,
        SYS_DOWN | _ => rebooting_msg.0.as_ptr() as *const _,
    };

    do_set_sstate(HV_SOFT_STATE_TRANSITION, msg);
    NOTIFY_OK
}

static mut sstate_reboot_notifier: notifier_block = notifier_block {
    notifier_call: Some(sstate_reboot_call),
    next: ::core::ptr::null_mut(),
    priority: 0,
};

unsafe extern "C" fn sstate_panic_event(_n: *mut notifier_block, _event: ::core::ffi::c_ulong, _ptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    do_set_sstate(HV_SOFT_STATE_TRANSITION, panicking_msg.0.as_ptr() as *const _);
    NOTIFY_DONE
}

static mut sstate_panic_block: notifier_block = notifier_block {
    notifier_call: Some(sstate_panic_event),
    next: ::core::ptr::null_mut(),
    priority: INT_MAX,
};

unsafe extern "C" fn sstate_init() -> ::core::ffi::c_int {
    let major: ::core::ffi::c_ulong = 1;
    let mut minor: ::core::ffi::c_ulong = 0;

    if tlb_type != hypervisor {
        return 0;
    }
    if sun4v_hvapi_register(HV_GRP_SOFT_STATE, major, &mut minor) != 0 {
        return 0;
    }

    hv_supports_soft_state = 1;
    prom_sun4v_guest_soft_state();
    do_set_sstate(HV_SOFT_STATE_TRANSITION, booting_msg.0.as_ptr() as *const _);
    atomic_notifier_chain_register(&mut panic_notifier_list, &mut sstate_panic_block);
    register_reboot_notifier(&mut sstate_reboot_notifier);
    0
}

unsafe extern "C" fn sstate_running() -> ::core::ffi::c_int {
    do_set_sstate(HV_SOFT_STATE_NORMAL, running_msg.0.as_ptr() as *const _);
    0
}

// core_initcall(sstate_init)
// late_initcall(sstate_running)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
