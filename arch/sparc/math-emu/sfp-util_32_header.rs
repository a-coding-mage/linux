/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the SPARC 32-bit C header.  The original dependencies are
// supplied by the surrounding kernel translation.

// These operations are implemented by SPARC inline assembly in the source.
// Rust has no portable file-local equivalent for that assembly; the macros
// retain the same interfaces and require the target-specific implementation.
macro_rules! add_ssaaaa {
    ($sh:expr, $sl:expr, $ah:expr, $al:expr, $bh:expr, $bl:expr) => {{
        compile_error!("SPARC add_ssaaaa inline assembly requires a target-specific implementation");
        let _ = (&$sh, &$sl, &$ah, &$al, &$bh, &$bl);
    }};
}

macro_rules! sub_ddmmss {
    ($sh:expr, $sl:expr, $ah:expr, $al:expr, $bh:expr, $bl:expr) => {{
        compile_error!("SPARC sub_ddmmss inline assembly requires a target-specific implementation");
        let _ = (&$sh, &$sl, &$ah, &$al, &$bh, &$bl);
    }};
}

macro_rules! umul_ppmm {
    ($w1:expr, $w0:expr, $u:expr, $v:expr) => {{
        compile_error!("SPARC umul_ppmm inline assembly requires a target-specific implementation");
        let _ = (&$w1, &$w0, &$u, &$v);
    }};
}

/* It is quite necessary to add this much assembler for the sparc.
   The default udiv_qrnnd (in C) is more than 10 times slower! */
macro_rules! udiv_qrnnd {
    ($q:expr, $r:expr, $n1:expr, $n0:expr, $d:expr) => {{
        compile_error!("SPARC udiv_qrnnd inline assembly requires a target-specific implementation");
        let _ = (&$q, &$r, &$n1, &$n0, &$d);
    }};
}

pub const UDIV_NEEDS_NORMALIZATION: i32 = 0;

// C's abort() macro in this header returns zero from the containing function.
macro_rules! abort {
    () => { return 0 };
}

// __BIG_ENDIAN is a build-time target condition in the original header.
#[cfg(target_endian = "big")]
pub const __BYTE_ORDER: u32 = __BIG_ENDIAN;
#[cfg(not(target_endian = "big"))]
pub const __BYTE_ORDER: u32 = __LITTLE_ENDIAN;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
