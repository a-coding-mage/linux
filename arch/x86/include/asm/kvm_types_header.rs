/* SPDX-License-Identifier: GPL-2.0 */

// Build-time module conditions from the C header.  The exact IS_MODULE and
// CONFIG_* configuration system is supplied by the surrounding build.
#[cfg(all(feature = "CONFIG_KVM_AMD", feature = "CONFIG_KVM_INTEL"))]
pub const KVM_SUB_MODULES: &str = "kvm-amd,kvm-intel";

#[cfg(all(
    feature = "CONFIG_KVM_AMD",
    not(feature = "CONFIG_KVM_INTEL")
))]
pub const KVM_SUB_MODULES: &str = "kvm-amd";

#[cfg(all(
    not(feature = "CONFIG_KVM_AMD"),
    feature = "CONFIG_KVM_INTEL"
))]
pub const KVM_SUB_MODULES: &str = "kvm-intel";

// With neither vendor module enabled, KVM_SUB_MODULES is undefined in C.
// EXPORT_SYMBOL_FOR_KVM(symbol) is intentionally not defined in that case.

pub const KVM_ARCH_NR_OBJS_PER_MEMORY_CACHE: usize = 40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
