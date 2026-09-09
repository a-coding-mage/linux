/* SPDX-License-Identifier: GPL-2.0 */

/* Enum to give clocksources a unique identifier */
#[repr(C)]
pub enum clocksource_ids {
    CSID_GENERIC = 0,
    CSID_ARM_ARCH_COUNTER,
    CSID_S390_TOD,
    CSID_X86_TSC_EARLY,
    CSID_X86_TSC,
    CSID_X86_KVM_CLK,
    CSID_X86_ART,
    CSID_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
