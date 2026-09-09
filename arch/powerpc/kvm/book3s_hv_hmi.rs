// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hypervisor Maintenance Interrupt (HMI) handling.
 *
 * Copyright 2015 IBM Corporation
 * Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
 */

// C source had DEBUG undefined.

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    static mut local_paca: *mut Paca;
    fn cpu_relax();
    fn test_bit(nr: i32, addr: *const u64) -> bool;
}

#[repr(C)]
pub struct SubcoreState {
    pub in_guest: [bool; MAX_SUBCORE_PER_CORE],
    pub flags: u64,
}

#[repr(C)]
pub struct Paca {
    pub sibling_subcore_state: *mut SubcoreState,
}

// Supplied by the included architecture headers in the original C source.
extern "C" {
    static MAX_SUBCORE_PER_CORE: usize;
    static CORE_TB_RESYNC_REQ_BIT: i32;
}

pub unsafe fn wait_for_subcore_guest_exit() {
    let mut i: i32;

    /*
     * NULL bitmap pointer indicates that KVM module hasn't
     * been loaded yet and hence no guests are running, or running
     * on POWER9 or newer CPU.
     *
     * If no KVM is in use, no need to co-ordinate among threads
     * as all of them will always be in host and no one is going
     * to modify TB other than the opal hmi handler.
     *
     * POWER9 and newer don't need this synchronisation.
     *
     * Hence, just return from here.
     */
    if (*local_paca).sibling_subcore_state.is_null() {
        return;
    }

    i = 0;
    while i < MAX_SUBCORE_PER_CORE as i32 {
        while (*(*local_paca).sibling_subcore_state).in_guest[i as usize] {
            cpu_relax();
        }
        i += 1;
    }
}

pub unsafe fn wait_for_tb_resync() {
    if (*local_paca).sibling_subcore_state.is_null() {
        return;
    }

    while test_bit(
        CORE_TB_RESYNC_REQ_BIT,
        &(*(*local_paca).sibling_subcore_state).flags,
    ) {
        cpu_relax();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
