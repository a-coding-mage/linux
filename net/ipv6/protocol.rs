// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		PF_INET6 protocol dispatch tables.
 *
 * Authors:	Pedro Roque <roque@di.fc.ul.pt>
 */

/*
 *      Changes:
 *
 *      Vince Laviano (vince@cs.stanford.edu)       16 May 2001
 *      - Removed unused variable 'inet6_protocol_base'
 *      - Modified inet6_del_protocol() to correctly maintain copy bit.
 */

// The following types and synchronization primitive are supplied by the
// surrounding kernel bindings.
#[repr(C)]
pub struct inet6_protocol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_offload {
    _private: [u8; 0],
}

pub const MAX_INET_PROTOS: usize = 256;

#[cfg(feature = "CONFIG_IPV6")]
#[no_mangle]
pub static mut inet6_protos: [*const inet6_protocol; MAX_INET_PROTOS] =
    [core::ptr::null(); MAX_INET_PROTOS];

#[cfg(feature = "CONFIG_IPV6")]
#[inline]
pub unsafe fn inet6_add_protocol(
    prot: *const inet6_protocol,
    protocol: u8,
) -> i32 {
    let slot = &mut inet6_protos[protocol as usize] as *mut *const inet6_protocol;
    let atomic = &*(slot as *const core::sync::atomic::AtomicPtr<inet6_protocol>);
    if atomic
        .compare_exchange(
            core::ptr::null_mut(),
            prot as *mut inet6_protocol,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        )
        .is_err()
    {
        -1
    } else {
        0
    }
}

#[cfg(feature = "CONFIG_IPV6")]
#[inline]
pub unsafe fn inet6_del_protocol(
    prot: *const inet6_protocol,
    protocol: u8,
) -> i32 {
    let slot = &mut inet6_protos[protocol as usize] as *mut *const inet6_protocol;
    let atomic = &*(slot as *const core::sync::atomic::AtomicPtr<inet6_protocol>);
    let ret = if atomic
        .compare_exchange(
            prot as *mut inet6_protocol,
            core::ptr::null_mut(),
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        )
        .is_ok()
    {
        0
    } else {
        -1
    };

    synchronize_net();

    ret
}

#[no_mangle]
pub static mut inet6_offloads: [*const net_offload; MAX_INET_PROTOS] =
    [core::ptr::null(); MAX_INET_PROTOS];

#[inline]
pub unsafe fn inet6_add_offload(
    prot: *const net_offload,
    protocol: u8,
) -> i32 {
    let slot = &mut inet6_offloads[protocol as usize] as *mut *const net_offload;
    let atomic = &*(slot as *const core::sync::atomic::AtomicPtr<net_offload>);
    if atomic
        .compare_exchange(
            core::ptr::null_mut(),
            prot as *mut net_offload,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        )
        .is_err()
    {
        -1
    } else {
        0
    }
}

#[inline]
pub unsafe fn inet6_del_offload(
    prot: *const net_offload,
    protocol: u8,
) -> i32 {
    let slot = &mut inet6_offloads[protocol as usize] as *mut *const net_offload;
    let atomic = &*(slot as *const core::sync::atomic::AtomicPtr<net_offload>);
    let ret = if atomic
        .compare_exchange(
            prot as *mut net_offload,
            core::ptr::null_mut(),
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        )
        .is_ok()
    {
        0
    } else {
        -1
    };

    synchronize_net();

    ret
}

extern "C" {
    pub fn synchronize_net();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
