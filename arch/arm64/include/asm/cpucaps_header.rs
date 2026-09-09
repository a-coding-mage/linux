/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency: <asm/cpucap-defs.h>
// Dependency: <linux/types.h>

/*
 * Check whether a cpucap is possible at compiletime.
 *
 * The C implementation uses compiletime_assert() and IS_ENABLED().  Rust's
 * cfg! expresses the corresponding build-time configuration intent.
 */
#[inline(always)]
pub fn cpucap_is_possible(cap: u32) -> bool {
    // C: compiletime_assert(__builtin_constant_p(cap), "cap must be a constant");
    // C: compiletime_assert(cap < ARM64_NCAPS, "cap must be < ARM64_NCAPS");
    match cap {
        ARM64_HAS_EPAN => cfg!(feature = "CONFIG_ARM64_EPAN"),
        ARM64_SVE => cfg!(feature = "CONFIG_ARM64_SVE"),
        ARM64_SME | ARM64_SME2 | ARM64_SME_FA64 => cfg!(feature = "CONFIG_ARM64_SME"),
        ARM64_HAS_CNP => cfg!(feature = "CONFIG_ARM64_CNP"),
        ARM64_HAS_ADDRESS_AUTH | ARM64_HAS_GENERIC_AUTH => {
            cfg!(feature = "CONFIG_ARM64_PTR_AUTH")
        }
        ARM64_HAS_GIC_PRIO_MASKING => cfg!(feature = "CONFIG_ARM64_PSEUDO_NMI"),
        ARM64_MTE => cfg!(feature = "CONFIG_ARM64_MTE"),
        ARM64_BTI => cfg!(feature = "CONFIG_ARM64_BTI"),
        ARM64_HAS_TLB_RANGE => cfg!(feature = "CONFIG_ARM64_TLB_RANGE"),
        ARM64_HAS_S1POE => cfg!(feature = "CONFIG_ARM64_POE"),
        ARM64_HAS_GCS => cfg!(feature = "CONFIG_ARM64_GCS"),
        ARM64_HAFT => cfg!(feature = "CONFIG_ARM64_HAFT"),
        ARM64_UNMAP_KERNEL_AT_EL0 => cfg!(feature = "CONFIG_UNMAP_KERNEL_AT_EL0"),
        ARM64_WORKAROUND_843419 => cfg!(feature = "CONFIG_ARM64_ERRATUM_843419"),
        ARM64_WORKAROUND_1742098 => cfg!(feature = "CONFIG_ARM64_ERRATUM_1742098"),
        ARM64_WORKAROUND_2645198 => cfg!(feature = "CONFIG_ARM64_ERRATUM_2645198"),
        ARM64_WORKAROUND_2658417 => cfg!(feature = "CONFIG_ARM64_ERRATUM_2658417"),
        ARM64_WORKAROUND_CAVIUM_23154 => cfg!(feature = "CONFIG_CAVIUM_ERRATUM_23154"),
        ARM64_WORKAROUND_DISABLE_CNP => cfg!(feature = "CONFIG_ARM64_WORKAROUND_DISABLE_CNP"),
        ARM64_WORKAROUND_REPEAT_TLBI_SYNC => {
            cfg!(feature = "CONFIG_ARM64_WORKAROUND_REPEAT_TLBI_SYNC")
        }
        ARM64_WORKAROUND_SPECULATIVE_SSBS => cfg!(feature = "CONFIG_ARM64_ERRATUM_3194386"),
        ARM64_WORKAROUND_4193714 => cfg!(feature = "CONFIG_ARM64_ERRATUM_4193714"),
        ARM64_MPAM => {
            /* KVM MPAM support doesn't rely on the host kernel supporting MPAM. */
            true
        }
        ARM64_HAS_PMUV3 => cfg!(feature = "CONFIG_HW_PERF_EVENTS"),
        ARM64_HAS_LSUI => cfg!(feature = "CONFIG_ARM64_LSUI"),
        _ => true,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
