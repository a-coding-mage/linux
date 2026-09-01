/* SPDX-License-Identifier: GPL-2.0 */

/* This defines the "copy" instruction from Power ISA 3.0 Book II, section 4.4. */
pub const fn __COPY(RA: u32, RB: u32, L: u32) -> u32 {
    0x7c00060c_u32 | (RA << (31 - 15)) | (RB << (31 - 20)) | (L << (31 - 10))
}

#[macro_export]
macro_rules! COPY {
    ($RA:expr, $RB:expr, $L:expr) => {
        concat!(".long ", stringify!(__COPY(($RA), ($RB), ($L))))
    };
}

pub unsafe fn copy(i: *mut core::ffi::c_void) {
    unsafe {
        core::arch::asm!(
            ".long {inst}",
            inst = const __COPY(0, 0, 0),
            in("r") i,
            options(nostack, preserves_flags)
        );
    }
}

pub unsafe fn copy_first(i: *mut core::ffi::c_void) {
    unsafe {
        core::arch::asm!(
            ".long {inst}",
            inst = const __COPY(0, 0, 1),
            in("r") i,
            options(nostack, preserves_flags)
        );
    }
}

/* This defines the "paste" instruction from Power ISA 3.0 Book II, section 4.4. */
pub const fn __PASTE(RA: u32, RB: u32, L: u32, RC: u32) -> u32 {
    0x7c00070c_u32
        | (RA << (31 - 15))
        | (RB << (31 - 20))
        | (L << (31 - 10))
        | (RC << (31 - 31))
}

#[macro_export]
macro_rules! PASTE {
    ($RA:expr, $RB:expr, $L:expr, $RC:expr) => {
        concat!(".long ", stringify!(__PASTE(($RA), ($RB), ($L), ($RC))))
    };
}

pub unsafe fn paste(i: *mut core::ffi::c_void) -> core::ffi::c_int {
    let cr: core::ffi::c_int;

    unsafe {
        core::arch::asm!(
            ".long {inst}",
            "mfcr {cr}",
            inst = const __PASTE(0, 0, 0, 0),
            cr = out(reg) cr,
            in("r") i,
            options(nostack)
        );
    }
    cr
}

pub unsafe fn paste_last(i: *mut core::ffi::c_void) -> core::ffi::c_int {
    let cr: core::ffi::c_int;

    unsafe {
        core::arch::asm!(
            ".long {inst}",
            "mfcr {cr}",
            inst = const __PASTE(0, 0, 1, 1),
            cr = out(reg) cr,
            in("r") i,
            options(nostack)
        );
    }
    cr
}

pub const PPC_INST_COPY: u32 = __COPY(0, 0, 0);
pub const PPC_INST_COPY_FIRST: u32 = __COPY(0, 0, 1);
pub const PPC_INST_PASTE: u32 = __PASTE(0, 0, 0, 0);
pub const PPC_INST_PASTE_LAST: u32 = __PASTE(0, 0, 1, 1);

/* This defines the prefixed load/store instructions.
 * The original C header stringifies these macros differently under __ASSEMBLER__
 * versus C preprocessing; the Rust translation exposes string-producing macros
 * for inline assembly text.
 */

pub const fn __PPC_RA(a: u32) -> u32 {
    (a & 0x1f) << 16
}

pub const fn __PPC_RS(s: u32) -> u32 {
    (s & 0x1f) << 21
}

pub const fn __PPC_RT(t: u32) -> u32 {
    __PPC_RS(t)
}

pub const fn __PPC_PREFIX_R(r: u32) -> u32 {
    (r & 0x1) << 20
}

pub const PPC_PREFIX_MLS: u32 = 0x06000000;
pub const PPC_PREFIX_8LS: u32 = 0x04000000;

pub const PPC_INST_LBZ: u32 = 0x88000000;
pub const PPC_INST_LHZ: u32 = 0xa0000000;
pub const PPC_INST_LHA: u32 = 0xa8000000;
pub const PPC_INST_LWZ: u32 = 0x80000000;
pub const PPC_INST_STB: u32 = 0x98000000;
pub const PPC_INST_STH: u32 = 0xb0000000;
pub const PPC_INST_STW: u32 = 0x90000000;
pub const PPC_INST_STD: u32 = 0xf8000000;
pub const PPC_INST_LFS: u32 = 0xc0000000;
pub const PPC_INST_LFD: u32 = 0xc8000000;
pub const PPC_INST_STFS: u32 = 0xd0000000;
pub const PPC_INST_STFD: u32 = 0xd8000000;

pub const fn PREFIX_MLS_PREFIX_WORD(r: u32, d: u32) -> u32 {
    PPC_PREFIX_MLS | __PPC_PREFIX_R(r) | ((d >> 16) & 0x3ffff)
}

pub const fn PREFIX_MLS_INSTR_WORD(instr: u32, t: u32, a: u32, d: u32) -> u32 {
    instr | __PPC_RT(t) | __PPC_RA(a) | (d & 0xffff)
}

pub const fn PREFIX_8LS_PREFIX_WORD(r: u32, d: u32) -> u32 {
    PPC_PREFIX_8LS | __PPC_PREFIX_R(r) | ((d >> 16) & 0x3ffff)
}

pub const fn PREFIX_8LS_INSTR_WORD(instr: u32, t: u32, a: u32, d: u32) -> u32 {
    instr | __PPC_RT(t) | __PPC_RA(a) | (d & 0xffff)
}

#[macro_export]
macro_rules! PREFIX_MLS {
    ($instr:expr, $t:expr, $a:expr, $r:expr, $d:expr) => {
        concat!(
            ".balign 64, , 4;",
            ".long PPC_PREFIX_MLS | __PPC_PREFIX_R(",
            stringify!($r),
            ") | (((",
            stringify!($d),
            ") >> 16) & 0x3ffff);",
            ".long (",
            stringify!($instr),
            ") | __PPC_RT(",
            stringify!($t),
            ") | __PPC_RA(",
            stringify!($a),
            ") | ((",
            stringify!($d),
            ") & 0xffff);\n"
        )
    };
}

#[macro_export]
macro_rules! PREFIX_8LS {
    ($instr:expr, $t:expr, $a:expr, $r:expr, $d:expr) => {
        concat!(
            ".balign 64, , 4;",
            ".long PPC_PREFIX_8LS | __PPC_PREFIX_R(",
            stringify!($r),
            ") | (((",
            stringify!($d),
            ") >> 16) & 0x3ffff);",
            ".long (",
            stringify!($instr),
            ") | __PPC_RT(",
            stringify!($t),
            ") | __PPC_RA(",
            stringify!($a),
            ") | ((",
            stringify!($d),
            ") & 0xffff);\n"
        )
    };
}

/* Prefixed Integer Load/Store instructions */
#[macro_export]
macro_rules! PLBZ { ($t:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_LBZ, $t, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLHZ { ($t:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_LHZ, $t, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLHA { ($t:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_LHA, $t, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLWZ { ($t:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_LWZ, $t, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLWA { ($t:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xa4000000, $t, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLD { ($t:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xe4000000, $t, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLQ { ($t:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xe0000000, $t, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTB { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_STB, $s, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTH { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_STH, $s, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTW { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_STW, $s, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTD { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xf4000000, $s, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTQ { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xf0000000, $s, $a, $r, $d) }; }

/* Prefixed Floating-Point Load/Store Instructions */
#[macro_export]
macro_rules! PLFS { ($frt:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_LFS, $frt, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLFD { ($frt:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_LFD, $frt, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTFS { ($frs:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_STFS, $frs, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTFD { ($frs:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_MLS!(PPC_INST_STFD, $frs, $a, $r, $d) }; }

/* Prefixed VSX Load/Store Instructions */
#[macro_export]
macro_rules! PLXSD { ($vrt:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xa8000000, $vrt, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLXSSP { ($vrt:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xac000000, $vrt, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLXV0 { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xc8000000, $s, $a, $r, $d) }; }
#[macro_export]
macro_rules! PLXV1 { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xcc000000, $s, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTXSD { ($vrs:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xb8000000, $vrs, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTXSSP { ($vrs:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xbc000000, $vrs, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTXV0 { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xd8000000, $s, $a, $r, $d) }; }
#[macro_export]
macro_rules! PSTXV1 { ($s:expr, $a:expr, $r:expr, $d:expr) => { PREFIX_8LS!(0xdc000000, $s, $a, $r, $d) }; }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
