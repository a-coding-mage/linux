/*
 * tie.h -- compile-time HAL definitions dependent on CORE & TIE configuration
 *
 * NOTE: This header file is not meant to be included directly.
 *
 * This Rust file is a source-level translation of the Xtensa TIE header.
 */

// The original header guard and include context are intentionally omitted.

pub const XCHAL_CP_NUM: u32 = 2; // number of coprocessors
pub const XCHAL_CP_MAX: u32 = 8; // max CP ID + 1 (0 if none)
pub const XCHAL_CP_MASK: u32 = 0x82; // bitmask of all CPs by ID
pub const XCHAL_CP_PORT_MASK: u32 = 0x80; // bitmask of only port CPs

// Basic parameters of each coprocessor:
pub const XCHAL_CP1_NAME: &str = "AudioEngineLX";
macro_rules! XCHAL_CP1_IDENT { () => { AudioEngineLX }; }
pub const XCHAL_CP1_SA_SIZE: u32 = 184; // size of state save area
pub const XCHAL_CP1_SA_ALIGN: u32 = 8; // min alignment of save area
pub const XCHAL_CP_ID_AUDIOENGINELX: u32 = 1; // coprocessor ID (0..7)
pub const XCHAL_CP7_NAME: &str = "XTIOP";
macro_rules! XCHAL_CP7_IDENT { () => { XTIOP }; }
pub const XCHAL_CP7_SA_SIZE: u32 = 0; // size of state save area
pub const XCHAL_CP7_SA_ALIGN: u32 = 1; // min alignment of save area
pub const XCHAL_CP_ID_XTIOP: u32 = 7; // coprocessor ID (0..7)

// Filler info for unassigned coprocessors, to simplify arrays etc:
pub const XCHAL_CP0_SA_SIZE: u32 = 0; pub const XCHAL_CP0_SA_ALIGN: u32 = 1;
pub const XCHAL_CP2_SA_SIZE: u32 = 0; pub const XCHAL_CP2_SA_ALIGN: u32 = 1;
pub const XCHAL_CP3_SA_SIZE: u32 = 0; pub const XCHAL_CP3_SA_ALIGN: u32 = 1;
pub const XCHAL_CP4_SA_SIZE: u32 = 0; pub const XCHAL_CP4_SA_ALIGN: u32 = 1;
pub const XCHAL_CP5_SA_SIZE: u32 = 0; pub const XCHAL_CP5_SA_ALIGN: u32 = 1;
pub const XCHAL_CP6_SA_SIZE: u32 = 0; pub const XCHAL_CP6_SA_ALIGN: u32 = 1;

// Save area for non-coprocessor optional and custom (TIE) state:
pub const XCHAL_NCP_SA_SIZE: u32 = 36;
pub const XCHAL_NCP_SA_ALIGN: u32 = 4;
// Total save area for optional and custom state (NCP + CPn):
pub const XCHAL_TOTAL_SA_SIZE: u32 = 240; // with 16-byte align padding
pub const XCHAL_TOTAL_SA_ALIGN: u32 = 8; // actual minimum alignment

/*
 * Detailed contents of save areas.
 * The caller must provide XCHAL_SA_REG before expanding these macros.
 * XCHAL_SA_REG(s,ccused,abikind,kind,opt,name,galign,align,asize,
 *              dbnum,base,regnum,bitsz,gapsz,reset,x...)
 *
 * The argument meanings are preserved from the C header: s selects the
 * expansion, ccused indicates compiler use, abikind is caller/callee/thread
 * saving, kind identifies special/TIE-user/TIE-regfile registers, opt marks
 * custom versus optional state, and the remaining fields describe layout.
 */

pub const XCHAL_NCP_SA_NUM: u32 = 9;
macro_rules! XCHAL_NCP_SA_LIST {
    ($s:tt) => {
        XCHAL_SA_REG!($s,1,2,1,1,threadptr,4,4,4,0x03E7,ur,231,32,0,0,0)
        XCHAL_SA_REG!($s,1,0,0,1,acclo,4,4,4,0x0210,sr,16,32,0,0,0)
        XCHAL_SA_REG!($s,1,0,0,1,acchi,4,4,4,0x0211,sr,17,8,0,0,0)
        XCHAL_SA_REG!($s,0,0,0,1,m0,4,4,4,0x0220,sr,32,32,0,0,0)
        XCHAL_SA_REG!($s,0,0,0,1,m1,4,4,4,0x0221,sr,33,32,0,0,0)
        XCHAL_SA_REG!($s,0,0,0,1,m2,4,4,4,0x0222,sr,34,32,0,0,0)
        XCHAL_SA_REG!($s,0,0,0,1,m3,4,4,4,0x0223,sr,35,32,0,0,0)
        XCHAL_SA_REG!($s,0,0,0,1,br,4,4,4,0x0204,sr,4,16,0,0,0)
        XCHAL_SA_REG!($s,0,0,0,1,scompare1,4,4,4,0x020C,sr,12,32,0,0,0)
    };
}

macro_rules! XCHAL_CP0_SA_NUM { () => { 0 }; }
macro_rules! XCHAL_CP0_SA_LIST { ($s:tt) => {}; }

pub const XCHAL_CP1_SA_NUM: u32 = 26;
macro_rules! XCHAL_CP1_SA_LIST {
    ($s:tt) => {
        XCHAL_SA_REG!($s,0,0,1,0,ae_ovf_sar,8,4,4,0x03F0,ur,240,8,0,0,0)
        XCHAL_SA_REG!($s,0,0,1,0,ae_bithead,4,4,4,0x03F1,ur,241,32,0,0,0)
        XCHAL_SA_REG!($s,0,0,1,0,ae_ts_fts_bu_bp,4,4,4,0x03F2,ur,242,16,0,0,0)
        XCHAL_SA_REG!($s,0,0,1,0,ae_cw_sd_no,4,4,4,0x03F3,ur,243,29,0,0,0)
        XCHAL_SA_REG!($s,0,0,1,0,ae_cbegin0,4,4,4,0x03F6,ur,246,32,0,0,0)
        XCHAL_SA_REG!($s,0,0,1,0,ae_cend0,4,4,4,0x03F7,ur,247,32,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed0,8,8,8,0x1010,aed,0,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed1,8,8,8,0x1011,aed,1,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed2,8,8,8,0x1012,aed,2,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed3,8,8,8,0x1013,aed,3,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed4,8,8,8,0x1014,aed,4,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed5,8,8,8,0x1015,aed,5,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed6,8,8,8,0x1016,aed,6,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed7,8,8,8,0x1017,aed,7,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed8,8,8,8,0x1018,aed,8,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed9,8,8,8,0x1019,aed,9,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed10,8,8,8,0x101A,aed,10,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed11,8,8,8,0x101B,aed,11,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed12,8,8,8,0x101C,aed,12,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed13,8,8,8,0x101D,aed,13,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed14,8,8,8,0x101E,aed,14,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,aed15,8,8,8,0x101F,aed,15,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,u0,8,8,8,0x1020,u,0,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,u1,8,8,8,0x1021,u,1,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,u2,8,8,8,0x1022,u,2,64,0,0,0)
        XCHAL_SA_REG!($s,0,0,2,0,u3,8,8,8,0x1023,u,3,64,0,0,0)
    };
}

// The following coprocessor save areas are empty in the source header.
pub const XCHAL_CP2_SA_NUM: u32 = 0;
pub const XCHAL_CP3_SA_NUM: u32 = 0;
pub const XCHAL_CP4_SA_NUM: u32 = 0;
pub const XCHAL_CP5_SA_NUM: u32 = 0;
pub const XCHAL_CP6_SA_NUM: u32 = 0;
pub const XCHAL_CP7_SA_NUM: u32 = 0;
macro_rules! XCHAL_CP2_SA_LIST { ($s:tt) => {}; }
macro_rules! XCHAL_CP3_SA_LIST { ($s:tt) => {}; }
macro_rules! XCHAL_CP4_SA_LIST { ($s:tt) => {}; }
macro_rules! XCHAL_CP5_SA_LIST { ($s:tt) => {}; }
macro_rules! XCHAL_CP6_SA_LIST { ($s:tt) => {}; }
macro_rules! XCHAL_CP7_SA_LIST { ($s:tt) => {}; }

// Byte length of instruction from its first nibble (op0 field), per FLIX.
macro_rules! XCHAL_OP0_FORMAT_LENGTHS { () => { 3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8 }; }
// Byte length of instruction from its first byte, per FLIX.
macro_rules! XCHAL_BYTE0_FORMAT_LENGTHS { () => {
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,8,8
}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
