//! Rust representation of `asm/ppc_asm.h`.
//!
//! The source is an assembler header: its substantive interface consists of
//! preprocessor/assembler macros.  They are retained below as Rust macros so
//! callers can preserve the original spellings and conditional intent.  The
//! emitted bodies are assembler fragments and therefore remain textual until
//! an architecture-specific inline-assembly integration supplies them.

#![allow(unused_macros)]

/* Dependencies supplied by the surrounding kernel translation unit:
 * linux/stringify.h, asm/asm-compat.h, asm/processor.h, asm/ppc-opcode.h,
 * asm/firmware.h, asm/feature-fixups.h, and asm/extable.h.
 */

#[macro_export]
macro_rules! ppc_asm_fragment { ($($tt:tt)*) => {{ /* PowerPC assembler: $($tt)* */ }} }

macro_rules! OP_REGS { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! ZEROIZE_REGS { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! SAVE_GPRS { ($($tt:tt)*) => { OP_REGS!($($tt)*) }; }
macro_rules! REST_GPRS { ($($tt:tt)*) => { OP_REGS!($($tt)*) }; }
macro_rules! SAVE_NVGPRS { ($($tt:tt)*) => { SAVE_GPRS!($($tt)*) }; }
macro_rules! REST_NVGPRS { ($($tt:tt)*) => { REST_GPRS!($($tt)*) }; }
macro_rules! ZEROIZE_GPRS { ($($tt:tt)*) => { ZEROIZE_REGS!($($tt)*) }; }
macro_rules! ZEROIZE_NVGPRS { () => { ZEROIZE_GPRS!(13, 31) }; }
macro_rules! ZEROIZE_GPR { ($n:tt) => { ZEROIZE_GPRS!($n, $n) }; }
macro_rules! SAVE_GPR { ($n:tt, $base:tt) => { SAVE_GPRS!($n, $n, $base) }; }
macro_rules! REST_GPR { ($n:tt, $base:tt) => { REST_GPRS!($n, $n, $base) }; }

macro_rules! SANITIZE_SYSCALL_GPRS { () => { ZEROIZE_GPR!(0); ZEROIZE_GPRS!(5, 12); ZEROIZE_NVGPRS!() }; }
macro_rules! SANITIZE_GPR { ($n:tt) => { ZEROIZE_GPR!($n) }; }
macro_rules! SANITIZE_GPRS { ($s:tt, $e:tt) => { ZEROIZE_GPRS!($s, $e) }; }
macro_rules! SANITIZE_NVGPRS { () => { ZEROIZE_NVGPRS!() }; }
macro_rules! SANITIZE_RESTORE_NVGPRS { () => { REST_NVGPRS!(r1) }; }
macro_rules! HANDLER_RESTORE_NVGPRS { () => {}; }

macro_rules! SAVE_FPR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! REST_FPR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! SAVE_2FPRS { ($($tt:tt)*) => { SAVE_FPR!($($tt)*) }; }
macro_rules! SAVE_4FPRS { ($($tt:tt)*) => { SAVE_2FPRS!($($tt)*) }; }
macro_rules! SAVE_8FPRS { ($($tt:tt)*) => { SAVE_4FPRS!($($tt)*) }; }
macro_rules! SAVE_16FPRS { ($($tt:tt)*) => { SAVE_8FPRS!($($tt)*) }; }
macro_rules! SAVE_32FPRS { ($($tt:tt)*) => { SAVE_16FPRS!($($tt)*) }; }
macro_rules! REST_2FPRS { ($($tt:tt)*) => { REST_FPR!($($tt)*) }; }
macro_rules! REST_4FPRS { ($($tt:tt)*) => { REST_2FPRS!($($tt)*) }; }
macro_rules! REST_8FPRS { ($($tt:tt)*) => { REST_4FPRS!($($tt)*) }; }
macro_rules! REST_16FPRS { ($($tt:tt)*) => { REST_8FPRS!($($tt)*) }; }
macro_rules! REST_32FPRS { ($($tt:tt)*) => { REST_16FPRS!($($tt)*) }; }

macro_rules! SAVE_VR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! REST_VR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! SAVE_2VRS { ($($tt:tt)*) => { SAVE_VR!($($tt)*) }; }
macro_rules! SAVE_4VRS { ($($tt:tt)*) => { SAVE_2VRS!($($tt)*) }; }
macro_rules! SAVE_8VRS { ($($tt:tt)*) => { SAVE_4VRS!($($tt)*) }; }
macro_rules! SAVE_16VRS { ($($tt:tt)*) => { SAVE_8VRS!($($tt)*) }; }
macro_rules! SAVE_32VRS { ($($tt:tt)*) => { SAVE_16VRS!($($tt)*) }; }
macro_rules! REST_2VRS { ($($tt:tt)*) => { REST_VR!($($tt)*) }; }
macro_rules! REST_4VRS { ($($tt:tt)*) => { REST_2VRS!($($tt)*) }; }
macro_rules! REST_8VRS { ($($tt:tt)*) => { REST_4VRS!($($tt)*) }; }
macro_rules! REST_16VRS { ($($tt:tt)*) => { REST_8VRS!($($tt)*) }; }
macro_rules! REST_32VRS { ($($tt:tt)*) => { REST_16VRS!($($tt)*) }; }

macro_rules! SAVE_VSR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! REST_VSR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! SAVE_2VSRS { ($($tt:tt)*) => { SAVE_VSR!($($tt)*) }; }
macro_rules! SAVE_4VSRS { ($($tt:tt)*) => { SAVE_2VSRS!($($tt)*) }; }
macro_rules! SAVE_8VSRS { ($($tt:tt)*) => { SAVE_4VSRS!($($tt)*) }; }
macro_rules! SAVE_16VSRS { ($($tt:tt)*) => { SAVE_8VSRS!($($tt)*) }; }
macro_rules! SAVE_32VSRS { ($($tt:tt)*) => { SAVE_16VSRS!($($tt)*) }; }
macro_rules! REST_2VSRS { ($($tt:tt)*) => { REST_VSR!($($tt)*) }; }
macro_rules! REST_4VSRS { ($($tt:tt)*) => { REST_2VSRS!($($tt)*) }; }
macro_rules! REST_8VSRS { ($($tt:tt)*) => { REST_4VSRS!($($tt)*) }; }
macro_rules! REST_16VSRS { ($($tt:tt)*) => { REST_8VSRS!($($tt)*) }; }
macro_rules! REST_32VSRS { ($($tt:tt)*) => { REST_16VSRS!($($tt)*) }; }

macro_rules! SAVE_EVR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! REST_EVR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! SAVE_2EVRS { ($($tt:tt)*) => { SAVE_EVR!($($tt)*) }; }
macro_rules! SAVE_4EVRS { ($($tt:tt)*) => { SAVE_2EVRS!($($tt)*) }; }
macro_rules! SAVE_8EVRS { ($($tt:tt)*) => { SAVE_4EVRS!($($tt)*) }; }
macro_rules! SAVE_16EVRS { ($($tt:tt)*) => { SAVE_8EVRS!($($tt)*) }; }
macro_rules! SAVE_32EVRS { ($($tt:tt)*) => { SAVE_16EVRS!($($tt)*) }; }
macro_rules! REST_2EVRS { ($($tt:tt)*) => { REST_EVR!($($tt)*) }; }
macro_rules! REST_4EVRS { ($($tt:tt)*) => { REST_2EVRS!($($tt)*) }; }
macro_rules! REST_8EVRS { ($($tt:tt)*) => { REST_4EVRS!($($tt)*) }; }
macro_rules! REST_16EVRS { ($($tt:tt)*) => { REST_8EVRS!($($tt)*) }; }
macro_rules! REST_32EVRS { ($($tt:tt)*) => { REST_16EVRS!($($tt)*) }; }

macro_rules! HMT_VERY_LOW { () => { ppc_asm_fragment!(or 31,31,31) }; }
macro_rules! HMT_LOW { () => { ppc_asm_fragment!(or 1,1,1) }; }
macro_rules! HMT_MEDIUM_LOW { () => { ppc_asm_fragment!(or 6,6,6) }; }
macro_rules! HMT_MEDIUM { () => { ppc_asm_fragment!(or 2,2,2) }; }
macro_rules! HMT_MEDIUM_HIGH { () => { ppc_asm_fragment!(or 5,5,5) }; }
macro_rules! HMT_HIGH { () => { ppc_asm_fragment!(or 3,3,3) }; }
macro_rules! HMT_EXTRA_HIGH { () => { ppc_asm_fragment!(or 7,7,7) }; }

pub const ULONG_SIZE: usize = 4;
#[inline] pub const fn __VCPU_GPR(n: usize) -> usize { VCPU_GPRS + n * ULONG_SIZE }
#[inline] pub const fn VCPU_GPR(n: usize) -> usize { __VCPU_GPR(n) }

macro_rules! CFUNC { ($name:ident) => { $name }; }
macro_rules! FUNC_START { ($name:ident) => { ppc_asm_fragment!(_GLOBAL $name) }; }
macro_rules! FUNC_END { ($name:ident) => {}; }
macro_rules! LOAD_REG_ADDR_PIC { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! LOAD_REG_IMMEDIATE { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! LOAD_REG_IMMEDIATE_SYM { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! LOAD_REG_ADDR { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! LOAD_REG_ADDRBASE { ($($tt:tt)*) => { LOAD_REG_ADDR!($($tt)*) }; }
macro_rules! PPC_CREATE_STACK_FRAME { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! MFTB { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! MFTBL { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! MFTBU { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! TLBSYNC { () => { ppc_asm_fragment!(tlbsync; sync) }; }
macro_rules! MTOCRF { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! PPC440EP_ERR42 { () => {}; }
macro_rules! DCBT_BOOK3S_STOP_ALL_STREAM_IDS { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! DCBT_SETUP_STREAMS { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! toreal { ($($tt:tt)*) => {}; }
macro_rules! fromreal { ($($tt:tt)*) => {}; }
macro_rules! tophys { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! tovirt { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! MTMSRD { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! MTMSR_EERI { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }

pub const RFSCV: u32 = 0x4c00_00a4;
macro_rules! FIXUP_ENDIAN { () => { ppc_asm_fragment!(tdi 0,0,0x48; b 191f; /* endian trampoline */) }; }
macro_rules! FIXUP_ENDIAN_HV { () => { ppc_asm_fragment!(tdi 0,0,0x48; b 191f; /* HV endian trampoline */) }; }
macro_rules! SOFT_MASK_TABLE { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! RESTART_TABLE { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }
macro_rules! BTB_FLUSH { ($($tt:tt)*) => { ppc_asm_fragment!($($tt)*) }; }

pub const STACK_FRAME_PARAMS: usize = 8;
pub const LRSAVE: usize = 4;

// CR, GPR, FPR, VPR, VSR, and EVR aliases retain their source numeric values.
pub const CR0: usize = 0; pub const CR1: usize = 1; pub const CR2: usize = 2;
pub const CR3: usize = 3; pub const CR4: usize = 4; pub const CR5: usize = 5;
pub const CR6: usize = 6; pub const CR7: usize = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
