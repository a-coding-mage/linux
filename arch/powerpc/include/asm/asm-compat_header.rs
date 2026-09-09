// Translation of asm-compat.h.
// The original include dependencies provide assembler constants, types, and
// opcode helpers; they remain external dependencies in this translation.

#[cfg(target_arch = "powerpc64")]
pub const PPC_LL: &str = "ld";
#[cfg(target_arch = "powerpc64")]
pub const PPC_STL: &str = "std";
#[cfg(target_arch = "powerpc64")]
pub const PPC_STLU: &str = "stdu";
#[cfg(target_arch = "powerpc64")]
pub const PPC_LCMPI: &str = "cmpdi";
#[cfg(target_arch = "powerpc64")]
pub const PPC_LCMPLI: &str = "cmpldi";
#[cfg(target_arch = "powerpc64")]
pub const PPC_LCMP: &str = "cmpd";
#[cfg(target_arch = "powerpc64")]
pub const PPC_LONG: &str = ".8byte";
#[cfg(target_arch = "powerpc64")]
pub const PPC_LONG_ALIGN: &str = ".balign 8";
#[cfg(target_arch = "powerpc64")]
pub const PPC_TLNEI: &str = "tdnei";
#[cfg(target_arch = "powerpc64")]
pub const PPC_LLARX: &str = "ldarx";
#[cfg(target_arch = "powerpc64")]
pub const PPC_STLCX: &str = "stdcx.";
#[cfg(target_arch = "powerpc64")]
pub const PPC_CNTLZL: &str = "cntlzd";
#[cfg(target_arch = "powerpc64")]
pub const PPC_SRL: &str = "srd";
#[cfg(target_arch = "powerpc64")]
pub const PPC_LR_STKOFF: i32 = 16;
#[cfg(target_arch = "powerpc64")]
pub const PPC_MIN_STKFRM: i32 = 112;

#[cfg(all(target_arch = "powerpc64", target_endian = "big"))]
pub const LWZX_BE: &str = "lwzx";
#[cfg(all(target_arch = "powerpc64", target_endian = "big"))]
pub const LDX_BE: &str = "ldx";
#[cfg(all(target_arch = "powerpc64", target_endian = "big"))]
pub const STWX_BE: &str = "stwx";
#[cfg(all(target_arch = "powerpc64", target_endian = "big"))]
pub const STDX_BE: &str = "stdx";

#[cfg(all(target_arch = "powerpc64", target_endian = "little"))]
pub const LWZX_BE: &str = "lwbrx";
#[cfg(all(target_arch = "powerpc64", target_endian = "little"))]
pub const LDX_BE: &str = "ldbrx";
#[cfg(all(target_arch = "powerpc64", target_endian = "little"))]
pub const STWX_BE: &str = "stwbrx";
#[cfg(all(target_arch = "powerpc64", target_endian = "little"))]
pub const STDX_BE: &str = "stdbrx";

// CONFIG_CC_IS_CLANG selects "Z<>"; other compilers select "YZ<>".
#[cfg(feature = "config_cc_is_clang")]
pub const DS_FORM_CONSTRAINT: &str = "Z<>";
#[cfg(not(feature = "config_cc_is_clang"))]
pub const DS_FORM_CONSTRAINT: &str = "YZ<>";

#[cfg(target_arch = "powerpc64")]
#[macro_export]
macro_rules! PPC_MTOCRF {
    ($fxm:expr, $rs:expr) => {
        MTOCRF(($fxm), $rs)
    };
}

#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_LL: &str = "lwz";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_STL: &str = "stw";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_STLU: &str = "stwu";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_LCMPI: &str = "cmpwi";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_LCMPLI: &str = "cmplwi";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_LCMP: &str = "cmpw";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_LONG: &str = ".long";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_LONG_ALIGN: &str = ".balign 4";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_TLNEI: &str = "twnei";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_LLARX: &str = "lwarx";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_STLCX: &str = "stwcx.";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_CNTLZL: &str = "cntlzw";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_SRL: &str = "srw";
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_LR_STKOFF: i32 = 4;
#[cfg(not(target_arch = "powerpc64"))]
pub const PPC_MIN_STKFRM: i32 = 16;

#[cfg(not(target_arch = "powerpc64"))]
#[macro_export]
macro_rules! PPC_MTOCRF {
    ($fxm:expr, $rs:expr) => {
        "mtcrf"
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
