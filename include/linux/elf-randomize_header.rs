/* SPDX-License-Identifier: GPL-2.0 */

// struct mm_struct;
pub struct mm_struct;

// CONFIG_ARCH_HAS_ELF_RANDOMIZE selects the architecture-specific declarations.
#[cfg(not(feature = "CONFIG_ARCH_HAS_ELF_RANDOMIZE"))]
#[inline]
pub fn arch_mmap_rnd() -> libc::c_ulong {
    0
}

// If arch_randomize_brk and CONFIG_COMPAT_BRK are defined, compat_brk_randomized
// is defined as a build-time marker.
#[cfg(all(
    not(feature = "CONFIG_ARCH_HAS_ELF_RANDOMIZE"),
    feature = "arch_randomize_brk",
    feature = "CONFIG_COMPAT_BRK"
))]
pub const compat_brk_randomized: () = ();

// When no architecture-specific arch_randomize_brk exists, the macro expands
// to the brk member of its mm argument.
#[cfg(all(
    not(feature = "CONFIG_ARCH_HAS_ELF_RANDOMIZE"),
    not(feature = "arch_randomize_brk")
))]
#[macro_export]
macro_rules! arch_randomize_brk {
    ($mm:expr) => {
        ($mm).brk
    };
}

#[cfg(feature = "CONFIG_ARCH_HAS_ELF_RANDOMIZE")]
unsafe extern "C" {
    pub fn arch_mmap_rnd() -> libc::c_ulong;
    pub fn arch_randomize_brk(mm: *mut mm_struct) -> libc::c_ulong;
}

#[cfg(all(
    feature = "CONFIG_ARCH_HAS_ELF_RANDOMIZE",
    feature = "CONFIG_COMPAT_BRK"
))]
pub const compat_brk_randomized: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
