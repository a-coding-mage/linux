/* SPDX-License-Identifier: GPL-2.0 */

/* ioctls. `_IO` and `_IOR` are supplied by the kernel ioctl definitions. */
pub const PA_PERF_ON: u32 = _IO(b'p' as u32, 1);
pub const PA_PERF_OFF: u32 = _IOR(b'p' as u32, 2, core::ffi::c_uint);
pub const PA_PERF_VERSION: u32 = _IOR(b'p' as u32, 3, core::ffi::c_int);

pub const PA_PERF_DEV: &str = "perf";
pub const PA_PERF_MINOR: u32 = 146;

/* Interface types */
pub const UNKNOWN_INTF: u32 = 255;
pub const ONYX_INTF: u32 = 0;
pub const CUDA_INTF: u32 = 1;

/* Common Onyx and Cuda images */
pub const CPI: u32 = 0;
pub const BUSUTIL: u32 = 1;
pub const TLBMISS: u32 = 2;
pub const TLBHANDMISS: u32 = 3;
pub const PTKN: u32 = 4;
pub const PNTKN: u32 = 5;
pub const IMISS: u32 = 6;
pub const DMISS: u32 = 7;
pub const DMISS_ACCESS: u32 = 8;
pub const BIG_CPI: u32 = 9;
pub const BIG_LS: u32 = 10;
pub const BR_ABORT: u32 = 11;
pub const ISNT: u32 = 12;
pub const QUADRANT: u32 = 13;
pub const RW_PDFET: u32 = 14;
pub const RW_WDFET: u32 = 15;
pub const SHLIB_CPI: u32 = 16;

/* Cuda only Images */
pub const FLOPS: u32 = 17;
pub const CACHEMISS: u32 = 18;
pub const BRANCHES: u32 = 19;
pub const CRSTACK: u32 = 20;
pub const I_CACHE_SPEC: u32 = 21;
pub const MAX_CUDA_IMAGES: u32 = 22;

/* Onyx only Images */
pub const ADDR_INV_ABORT_ALU: u32 = 17;
pub const BRAD_STALL: u32 = 18;
pub const CNTL_IN_PIPEL: u32 = 19;
pub const DSNT_XFH: u32 = 20;
pub const FET_SIG1: u32 = 21;
pub const FET_SIG2: u32 = 22;
pub const G7_1: u32 = 23;
pub const G7_2: u32 = 24;
pub const G7_3: u32 = 25;
pub const G7_4: u32 = 26;
pub const MPB_LABORT: u32 = 27;
pub const PANIC: u32 = 28;
pub const RARE_INST: u32 = 29;
pub const RW_DFET: u32 = 30;
pub const RW_IFET: u32 = 31;
pub const RW_SDFET: u32 = 32;
pub const SPEC_IFET: u32 = 33;
pub const ST_COND0: u32 = 34;
pub const ST_COND1: u32 = 35;
pub const ST_COND2: u32 = 36;
pub const ST_COND3: u32 = 37;
pub const ST_COND4: u32 = 38;
pub const ST_UNPRED0: u32 = 39;
pub const ST_UNPRED1: u32 = 40;
pub const UNPRED: u32 = 41;
pub const GO_STORE: u32 = 42;
pub const SHLIB_CALL: u32 = 43;
pub const MAX_ONYX_IMAGES: u32 = 44;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
