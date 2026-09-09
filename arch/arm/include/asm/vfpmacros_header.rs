/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of arch/arm/include/asm/vfpmacros.h.
 *
 * This source corresponds to an assembler-only header containing VFP macros
 * and register definitions.  The original asm/hwcap.h and asm/vfp.h
 * dependencies are supplied by the surrounding target.
 */

/// Read a VFP system register into an ARM register.
#[macro_export]
macro_rules! VFPFMRX {
    ($rd:tt, $sysreg:tt, $cond:tt) => {{
        unsafe {
            core::arch::asm!(concat!(
                "vmrs", stringify!($cond), " ", stringify!($rd), ", ",
                stringify!($sysreg)
            ));
        }
    }};
}

/// Write an ARM register into a VFP system register.
#[macro_export]
macro_rules! VFPFMXR {
    ($sysreg:tt, $rd:tt, $cond:tt) => {{
        unsafe {
            core::arch::asm!(concat!(
                "vmsr", stringify!($cond), " ", stringify!($sysreg), ", ",
                stringify!($rd)
            ));
        }
    }};
}

/// Read all working registers back into the VFP.
///
/// The C preprocessor selected `fldmiax` before ARMv6 and `vldmia`
/// thereafter.  CONFIG_VFPv3 and ARM architecture feature tests are retained
/// below as comments because they are target build-time conditions.
#[macro_export]
macro_rules! VFPFLDMIA {
    ($base:tt, $tmp:tt) => {{
        unsafe {
            // __LINUX_ARM_ARCH__ < 6: use `fldmiax` instead of `vldmia`.
            core::arch::asm!(concat!("vldmia ", stringify!($base), ", {d0-d15}"));

            // CONFIG_VFPv3:
            // __LINUX_ARM_ARCH__ <= 6 loads elf_hwcap, tests HWCAP_VFPD32,
            // conditionally loads d16-d31, and otherwise advances by 32*4.
            // On newer ARM targets, VFPFMRX($tmp, MVFR0, ) masks
            // MVFR0_A_SIMD_MASK, compares with 2, and performs the same
            // conditional load or 32*4 advance.
            let _ = (stringify!($tmp), stringify!($base));
        }
    }};
}

/// Write all working registers out of the VFP.
///
/// The C preprocessor selected `fstmiax` before ARMv6 and `vstmia`
/// thereafter.  CONFIG_VFPv3 and ARM architecture feature tests are retained
/// below as comments because they are target build-time conditions.
#[macro_export]
macro_rules! VFPFSTMIA {
    ($base:tt, $tmp:tt) => {{
        unsafe {
            // __LINUX_ARM_ARCH__ < 6: use `fstmiax` instead of `vstmia`.
            core::arch::asm!(concat!("vstmia ", stringify!($base), ", {d0-d15}"));

            // CONFIG_VFPv3:
            // __LINUX_ARM_ARCH__ <= 6 loads elf_hwcap, tests HWCAP_VFPD32,
            // conditionally stores d16-d31, and otherwise advances by 32*4.
            // On newer ARM targets, VFPFMRX($tmp, MVFR0, ) masks
            // MVFR0_A_SIMD_MASK, compares with 2, and performs the same
            // conditional store or 32*4 advance.
            let _ = (stringify!($tmp), stringify!($base));
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
