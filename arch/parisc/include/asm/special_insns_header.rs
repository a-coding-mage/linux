/* SPDX-License-Identifier: GPL-2.0 */
// Translated from __PARISC_SPECIAL_INSNS_H.

// ASM_EXCEPTIONTABLE_ENTRY is supplied by the surrounding architecture code.
// The original inline assembly also emits exception-table entries at these
// sites; preserve the assembly operations here while retaining that intent.

#[macro_export]
macro_rules! lpa {
    ($va:expr) => {{
        let pa: ::core::primitive::usize;
        unsafe {
            ::core::arch::asm!(
                "copy %r0, {pa}",
                "8: lpa 0({va}), {pa}",
                "9:",
                pa = lateout(reg) pa,
                va = in(reg) $va,
                options(nostack)
            );
        }
        pa
    }};
}

#[macro_export]
macro_rules! lpa_user {
    ($va:expr) => {{
        let pa: ::core::primitive::usize;
        unsafe {
            ::core::arch::asm!(
                "copy %r0, {pa}",
                "8: lpa 0(%sr3,{va}), {pa}",
                "9:",
                pa = lateout(reg) pa,
                va = in(reg) $va,
                options(nostack)
            );
        }
        pa
    }};
}

/**
 * prober_user() - Probe user read access
 * @sr: Space regster.
 * @va: Virtual address.
 *
 * Return: Non-zero if address is accessible.
 *
 * Due to the way _PAGE_READ is handled in TLB entries, we need
 * a special check to determine whether a user address is accessible.
 * The ldb instruction does the initial access check. If it is
 * successful, the probe instruction checks user access rights.
 */
#[macro_export]
macro_rules! prober_user {
    ($sr:expr, $va:expr) => {{
        let read_allowed: ::core::primitive::usize;
        unsafe {
            ::core::arch::asm!(
                "copy %r0, {read_allowed}",
                "8: ldb 0(%sr{sr},{va}), %r0",
                "    proberi (%sr{sr},{va}), {priv_user}, {read_allowed}",
                "9:",
                sr = const $sr,
                va = in(reg) $va,
                priv_user = const PRIV_USER,
                read_allowed = lateout(reg) read_allowed,
                options(nostack)
            );
        }
        read_allowed
    }};
}

pub const CR_EIEM: usize = 15; // External Interrupt Enable Mask
pub const CR_CR16: usize = 16; // CR16 Interval Timer
pub const CR_EIRR: usize = 23; // External Interrupt Request Register

#[macro_export]
macro_rules! mfctl {
    ($reg:expr) => {{
        let cr: ::core::primitive::usize;
        unsafe {
            ::core::arch::asm!("mfctl {reg}, {cr}", reg = const $reg, cr = lateout(reg) cr);
        }
        cr
    }};
}

#[macro_export]
macro_rules! mtctl {
    ($gr:expr, $cr:expr) => {{
        unsafe {
            ::core::arch::asm!("mtctl {gr}, {cr}", gr = in(reg) $gr, cr = const $cr, options(nostack));
        }
    }};
}

#[macro_export]
macro_rules! get_eiem {
    () => { $crate::mfctl!(CR_EIEM) };
}

#[macro_export]
macro_rules! set_eiem {
    ($val:expr) => { $crate::mtctl!($val, CR_EIEM) };
}

#[macro_export]
macro_rules! mfsp {
    ($reg:expr) => {{
        let cr: ::core::primitive::usize;
        unsafe {
            ::core::arch::asm!("mfsp %sr{reg}, {cr}", reg = const $reg, cr = lateout(reg) cr);
        }
        cr
    }};
}

#[macro_export]
macro_rules! mtsp {
    ($val:expr, $cr:expr) => {{
        unsafe {
            if ($val) == 0 {
                ::core::arch::asm!("mtsp %r0, {cr}", cr = const $cr, options(nostack));
            } else {
                ::core::arch::asm!("mtsp {val}, {cr}", val = in(reg) $val, cr = const $cr, options(nostack));
            }
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
