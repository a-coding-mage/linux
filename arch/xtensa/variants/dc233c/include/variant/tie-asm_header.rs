/*
 * tie-asm.h -- compile-time HAL assembler definitions dependent on CORE & TIE
 *
 * Rust translation of the target-specific assembler header.
 * This header is not intended to be included directly.
 */

/* Selection parameter values for save-area save/restore macros. */
pub const XTHAL_SAS_TIE: u32 = 0x0001; // custom extension or coprocessor
pub const XTHAL_SAS_OPT: u32 = 0x0002; // optional (and not a coprocessor)
pub const XTHAL_SAS_ANYOT: u32 = 0x0003; // both of the above

pub const XTHAL_SAS_NOCC: u32 = 0x0004; // not used by compiler without special opts/code
pub const XTHAL_SAS_CC: u32 = 0x0008; // used by compiler without special opts/code
pub const XTHAL_SAS_ANYCC: u32 = 0x000C; // both of the above

pub const XTHAL_SAS_CALR: u32 = 0x0010; // caller-saved
pub const XTHAL_SAS_CALE: u32 = 0x0020; // callee-saved
pub const XTHAL_SAS_GLOB: u32 = 0x0040; // global across function calls (in thread)
pub const XTHAL_SAS_ANYABI: u32 = 0x0070; // all of the above three

pub const XTHAL_SAS_ALL: u32 = 0xFFFF; // include all default NCP contents

#[inline]
pub const fn xthal_sas3(optie: u32, ccuse: u32, abi: u32) -> u32 {
    (optie & XTHAL_SAS_ANYOT) | (ccuse & XTHAL_SAS_ANYCC) | (abi & XTHAL_SAS_ANYABI)
}

/*
 * The following macros are Xtensa assembler macros. Rust has no equivalent
 * for directives such as xchal_sa_start, xchal_sa_align, rsr, s32i, l32i,
 * wsr, and wur.THREADPTR; their complete source-level operation is retained
 * here as macro input for the target-specific assembler integration.
 */

/// Save all non-coprocessor custom TIE and optional state.
#[macro_export]
macro_rules! xchal_ncp_store {
    ($ptr:tt $at1:tt $at2:tt $at3:tt $at4:tt
     $(continue = $continue:tt)? $(ofs = $ofs:tt)?
     $(select = $select:tt)? $(alloc = $alloc:tt)?) => {
        /*
         * xchal_sa_start(continue, ofs)
         * If selected: align(ptr, 0, 1020, 4, 4); rur.THREADPTR at1;
         *   s32i at1, ptr, .Lxchal_ofs_+0; advance offset by 4.
         * If allocated: align(ptr, 0, 1020, 4, 4); advance by 4.
         * If selected: align(ptr, 0, 1016, 4, 4); rsr at1, ACCLO;
         *   s32i at1, ptr, ofs+0; rsr at1, ACCHI;
         *   s32i at1, ptr, ofs+4; advance offset by 8.
         * If allocated: align(ptr, 0, 1016, 4, 4); advance by 8.
         * If selected: align(ptr, 0, 1004, 4, 4); rsr/s32i M0, M1, M2, M3
         *   at offsets 0,4,8,12, then rsr/s32i SCOMPARE1 at offset 16;
         *   advance offset by 20.
         * If allocated: align(ptr, 0, 1004, 4, 4); advance by 20.
         */
    };
}

/// Restore all non-coprocessor custom TIE and optional state.
#[macro_export]
macro_rules! xchal_ncp_load {
    ($ptr:tt $at1:tt $at2:tt $at3:tt $at4:tt
     $(continue = $continue:tt)? $(ofs = $ofs:tt)?
     $(select = $select:tt)? $(alloc = $alloc:tt)?) => {
        /*
         * xchal_sa_start(continue, ofs)
         * If selected: align(ptr, 0, 1020, 4, 4); l32i at1, ptr, ofs+0;
         *   wur.THREADPTR at1; advance offset by 4.
         * If allocated: align(ptr, 0, 1020, 4, 4); advance by 4.
         * If selected: align(ptr, 0, 1016, 4, 4); l32i/wsr ACCLO at 0;
         *   l32i/wsr ACCHI at 4; advance offset by 8.
         * If allocated: align(ptr, 0, 1016, 4, 4); advance by 8.
         * If selected: align(ptr, 0, 1004, 4, 4); l32i/wsr M0, M1, M2, M3
         *   at offsets 0,4,8,12, then l32i/wsr SCOMPARE1 at offset 16;
         *   advance offset by 20.
         * If allocated: align(ptr, 0, 1004, 4, 4); advance by 20.
         */
    };
}

pub const XCHAL_NCP_NUM_ATMPS: u32 = 1;
pub const XCHAL_SA_NUM_ATMPS: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
