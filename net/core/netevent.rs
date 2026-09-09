// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Network event notifiers
 *
 *	Authors:
 *      Tom Tucker             <tom@opengridcomputing.com>
 *      Steve Wise             <swise@opengridcomputing.com>
 *
 *	Fixes:
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

extern "C" {
    fn atomic_notifier_chain_register(
        nh: *mut atomic_notifier_head,
        nb: *mut notifier_block,
    ) -> ::std::ffi::c_int;
    fn atomic_notifier_chain_unregister(
        nh: *mut atomic_notifier_head,
        nb: *mut notifier_block,
    ) -> ::std::ffi::c_int;
    fn atomic_notifier_call_chain(
        nh: *mut atomic_notifier_head,
        val: ::std::ffi::c_ulong,
        v: *mut ::std::ffi::c_void,
    ) -> ::std::ffi::c_int;
}

#[repr(C)]
pub struct atomic_notifier_head {
    _private: [u8; 0],
}

static mut netevent_notif_chain: atomic_notifier_head = atomic_notifier_head {
    _private: [],
};

/**
 *	register_netevent_notifier - register a netevent notifier block
 *	@nb: notifier
 *
 *	Register a notifier to be called when a netevent occurs.
 *	The notifier passed is linked into the kernel structures and must
 *	not be reused until it has been unregistered. A negative errno code
 *	is returned on a failure.
 */
#[no_mangle]
pub unsafe extern "C" fn register_netevent_notifier(nb: *mut notifier_block) -> ::std::ffi::c_int {
    atomic_notifier_chain_register(&mut netevent_notif_chain, nb)
}

/**
 *	unregister_netevent_notifier - unregister a netevent notifier block
 *	@nb: notifier
 *
 *	Unregister a notifier previously registered by
 *	register_neigh_notifier(). The notifier is unlinked into the
 *	kernel structures and may then be reused. A negative errno code
 *	is returned on a failure.
 */
#[no_mangle]
pub unsafe extern "C" fn unregister_netevent_notifier(nb: *mut notifier_block) -> ::std::ffi::c_int {
    atomic_notifier_chain_unregister(&mut netevent_notif_chain, nb)
}

/**
 *	call_netevent_notifiers - call all netevent notifier blocks
 *      @val: value passed unmodified to notifier function
 *      @v:   pointer passed unmodified to notifier function
 *
 *	Call all neighbour notifier blocks.  Parameters and return value
 *	are as for notifier_call_chain().
 */
#[no_mangle]
pub unsafe extern "C" fn call_netevent_notifiers(
    val: ::std::ffi::c_ulong,
    v: *mut ::std::ffi::c_void,
) -> ::std::ffi::c_int {
    atomic_notifier_call_chain(&mut netevent_notif_chain, val, v)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
