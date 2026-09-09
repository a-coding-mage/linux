/* SPDX-License-Identifier: MIT */

// Dependency: declarations supplied by xen.h are referenced below.

pub const XENPMU_VER_MAJ: u32 = 0;
pub const XENPMU_VER_MIN: u32 = 1;

/*
 * ` enum neg_errnoval
 * ` HYPERVISOR_xenpmu_op(enum xenpmu_op cmd, struct xenpmu_params *args);
 *
 * @cmd  == XENPMU_* (PMU operation)
 * @args == struct xenpmu_params
 */
/* ` enum xenpmu_op { */
pub const XENPMU_mode_get: u32 = 0; /* Also used for getting PMU version */
pub const XENPMU_mode_set: u32 = 1;
pub const XENPMU_feature_get: u32 = 2;
pub const XENPMU_feature_set: u32 = 3;
pub const XENPMU_init: u32 = 4;
pub const XENPMU_finish: u32 = 5;
pub const XENPMU_lvtpc_set: u32 = 6;
pub const XENPMU_flush: u32 = 7;

/* ` } */

/* Parameters structure for HYPERVISOR_xenpmu_op call */
#[repr(C)]
pub struct xen_pmu_params {
    /* IN/OUT parameters */
    pub version: xen_pmu_params_version,
    pub val: u64,

    /* IN parameters */
    pub vcpu: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct xen_pmu_params_version {
    pub maj: u32,
    pub min: u32,
}

/* PMU modes:
 * - XENPMU_MODE_OFF:   No PMU virtualization
 * - XENPMU_MODE_SELF:  Guests can profile themselves
 * - XENPMU_MODE_HV:    Guests can profile themselves, dom0 profiles
 *                      itself and Xen
 * - XENPMU_MODE_ALL:   Only dom0 has access to VPMU and it profiles
 *                      everyone: itself, the hypervisor and the guests.
 */
pub const XENPMU_MODE_OFF: u32 = 0;
pub const XENPMU_MODE_SELF: u32 = 1 << 0;
pub const XENPMU_MODE_HV: u32 = 1 << 1;
pub const XENPMU_MODE_ALL: u32 = 1 << 2;

/*
 * PMU features:
 * - XENPMU_FEATURE_INTEL_BTS: Intel BTS support (ignored on AMD)
 */
pub const XENPMU_FEATURE_INTEL_BTS: u32 = 1;

/*
 * Shared PMU data between hypervisor and PV(H) domains.
 *
 * The hypervisor fills out this structure during PMU interrupt and sends an
 * interrupt to appropriate VCPU.
 * Architecture-independent fields of xen_pmu_data are WO for the hypervisor
 * and RO for the guest but some fields in xen_pmu_arch can be writable
 * by both the hypervisor and the guest (see arch-$arch/pmu.h).
 */
#[repr(C)]
pub struct xen_pmu_data {
    /* Interrupted VCPU */
    pub vcpu_id: u32,

    /*
     * Physical processor on which the interrupt occurred. On non-privileged
     * guests set to vcpu_id;
     */
    pub pcpu_id: u32,

    /*
     * Domain that was interrupted. On non-privileged guests set to
     * DOMID_SELF.
     * On privileged guests can be DOMID_SELF, DOMID_XEN, or, when in
     * XENPMU_MODE_ALL mode, domain ID of another domain.
     */
    pub domain_id: domid_t,

    pub pad: [u8; 6],

    /* Architecture-specific information */
    pub pmu: xen_pmu_arch,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
