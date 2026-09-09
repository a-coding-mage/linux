/*
 * tie.h -- compile-time HAL definitions dependent on CORE & TIE configuration
 *
 * NOTE: This header file is not meant to be included directly.
 *
 * Rust translation of the Xtensa processor TIE configuration header.
 */

pub const XCHAL_CP_NUM: usize = 1;
pub const XCHAL_CP_MAX: usize = 8;
pub const XCHAL_CP_MASK: u32 = 0x80;
pub const XCHAL_CP_PORT_MASK: u32 = 0x80;

pub const XCHAL_CP7_NAME: &str = "XTIOP";
/* XCHAL_CP7_IDENT is the target-specific identifier XTIOP. */
pub const XCHAL_CP7_SA_SIZE: usize = 0;
pub const XCHAL_CP7_SA_ALIGN: usize = 1;
pub const XCHAL_CP_ID_XTIOP: usize = 7;

pub const XCHAL_CP0_SA_SIZE: usize = 0;
pub const XCHAL_CP0_SA_ALIGN: usize = 1;
pub const XCHAL_CP1_SA_SIZE: usize = 0;
pub const XCHAL_CP1_SA_ALIGN: usize = 1;
pub const XCHAL_CP2_SA_SIZE: usize = 0;
pub const XCHAL_CP2_SA_ALIGN: usize = 1;
pub const XCHAL_CP3_SA_SIZE: usize = 0;
pub const XCHAL_CP3_SA_ALIGN: usize = 1;
pub const XCHAL_CP4_SA_SIZE: usize = 0;
pub const XCHAL_CP4_SA_ALIGN: usize = 1;
pub const XCHAL_CP5_SA_SIZE: usize = 0;
pub const XCHAL_CP5_SA_ALIGN: usize = 1;
pub const XCHAL_CP6_SA_SIZE: usize = 0;
pub const XCHAL_CP6_SA_ALIGN: usize = 1;

pub const XCHAL_NCP_SA_SIZE: usize = 36;
pub const XCHAL_NCP_SA_ALIGN: usize = 4;
pub const XCHAL_TOTAL_SA_SIZE: usize = 48;
pub const XCHAL_TOTAL_SA_ALIGN: usize = 4;

pub const XCHAL_NCP_SA_NUM: usize = 9;

/*
 * The caller must define XCHAL_SA_REG before expanding this macro, matching
 * the original C preprocessor interface.
 */
#[macro_export]
macro_rules! XCHAL_NCP_SA_LIST {
    ($s:expr) => {
        XCHAL_SA_REG!($s, 1, 2, 1, 1, threadptr, 4, 4, 4, 0x03E7, ur, 231, 32, 0, 0, 0);
        XCHAL_SA_REG!($s, 1, 0, 0, 1, acclo, 4, 4, 4, 0x0210, sr, 16, 32, 0, 0, 0);
        XCHAL_SA_REG!($s, 1, 0, 0, 1, acchi, 4, 4, 4, 0x0211, sr, 17, 8, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 0, 1, br, 4, 4, 4, 0x0204, sr, 4, 16, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 0, 1, scompare1, 4, 4, 4, 0x020C, sr, 12, 32, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 0, 1, m0, 4, 4, 4, 0x0220, sr, 32, 32, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 0, 1, m1, 4, 4, 4, 0x0221, sr, 33, 32, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 0, 1, m2, 4, 4, 4, 0x0222, sr, 34, 32, 0, 0, 0);
        XCHAL_SA_REG!($s, 0, 0, 0, 1, m3, 4, 4, 4, 0x0223, sr, 35, 32, 0, 0, 0);
    };
}

pub const XCHAL_CP0_SA_NUM: usize = 0;
pub const XCHAL_CP1_SA_NUM: usize = 0;
pub const XCHAL_CP2_SA_NUM: usize = 0;
pub const XCHAL_CP3_SA_NUM: usize = 0;
pub const XCHAL_CP4_SA_NUM: usize = 0;
pub const XCHAL_CP5_SA_NUM: usize = 0;
pub const XCHAL_CP6_SA_NUM: usize = 0;
pub const XCHAL_CP7_SA_NUM: usize = 0;

/* The coprocessor save-area list macros are intentionally empty. */
#[macro_export] macro_rules! XCHAL_CP0_SA_LIST { ($s:expr) => {}; }
#[macro_export] macro_rules! XCHAL_CP1_SA_LIST { ($s:expr) => {}; }
#[macro_export] macro_rules! XCHAL_CP2_SA_LIST { ($s:expr) => {}; }
#[macro_export] macro_rules! XCHAL_CP3_SA_LIST { ($s:expr) => {}; }
#[macro_export] macro_rules! XCHAL_CP4_SA_LIST { ($s:expr) => {}; }
#[macro_export] macro_rules! XCHAL_CP5_SA_LIST { ($s:expr) => {}; }
#[macro_export] macro_rules! XCHAL_CP6_SA_LIST { ($s:expr) => {}; }
#[macro_export] macro_rules! XCHAL_CP7_SA_LIST { ($s:expr) => {}; }

pub const XCHAL_OP0_FORMAT_LENGTHS: [u8; 16] = [3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 3, 3];

pub const XCHAL_BYTE0_FORMAT_LENGTHS: [u8; 256] = [
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
