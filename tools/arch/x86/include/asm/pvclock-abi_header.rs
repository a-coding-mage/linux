/* SPDX-License-Identifier: GPL-2.0 */

/*
 * These structs MUST NOT be changed.
 * They are the ABI between hypervisor and guest OS.
 * Both Xen and KVM are using this.
 *
 * pvclock_vcpu_time_info holds the system time and the tsc timestamp
 * of the last update. So the guest can use the tsc delta to get a
 * more precise system time.  There is one per virtual cpu.
 *
 * pvclock_wall_clock references the point in time when the system
 * time was zero (usually boot time), thus the guest calculates the
 * current wall clock by adding the system time.
 *
 * Protocol for the "version" fields is: hypervisor raises it (making
 * it uneven) before it starts updating the fields and raises it again
 * (making it even) when it is done.  Thus the guest can make sure the
 * time values it got are consistent by checking the version before
 * and after reading them.
 */

#[repr(C, packed)]
pub struct pvclock_vcpu_time_info {
    pub version: u32,
    pub pad0: u32,
    pub tsc_timestamp: u64,
    pub system_time: u64,
    pub tsc_to_system_mul: u32,
    pub tsc_shift: i8,
    pub flags: u8,
    pub pad: [u8; 2],
} /* 32 bytes */

#[repr(C, packed)]
pub struct pvclock_wall_clock {
    pub version: u32,
    pub sec: u32,
    pub nsec: u32,
}

pub const PVCLOCK_TSC_STABLE_BIT: u32 = 1 << 0;
pub const PVCLOCK_GUEST_STOPPED: u32 = 1 << 1;
/* PVCLOCK_COUNTS_FROM_ZERO broke ABI and can't be used anymore. */
pub const PVCLOCK_COUNTS_FROM_ZERO: u32 = 1 << 2;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
