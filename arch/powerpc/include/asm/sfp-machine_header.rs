/* Machine-dependent software floating-point definitions. PPC version. */
/* This is a source-level Rust translation of the original C header. */

pub const _FP_W_TYPE_SIZE: u32 = 32;
pub type _FP_W_TYPE = u32;
pub type _FP_WS_TYPE = i32;
pub type _FP_I_TYPE = i32;

/* The original macros use UWtype and W_TYPE_SIZE supplied by other headers. */
#[macro_export]
macro_rules! __ll_B { () => { ((1 as UWtype) << (W_TYPE_SIZE / 2)) }; }
#[macro_export]
macro_rules! __ll_lowpart { ($t:expr) => { (($t as UWtype) & (__ll_B!() - 1)) }; }
#[macro_export]
macro_rules! __ll_highpart { ($t:expr) => { (($t as UWtype) >> (W_TYPE_SIZE / 2)) }; }

/* Multiplication and division meat are supplied by the soft-float operation headers. */
#[macro_export]
macro_rules! _FP_MUL_MEAT_S { ($R:ident,$X:ident,$Y:ident) => { _FP_MUL_MEAT_1_wide!(_FP_WFRACBITS_S,$R,$X,$Y,umul_ppmm); }; }
#[macro_export]
macro_rules! _FP_MUL_MEAT_D { ($R:ident,$X:ident,$Y:ident) => { _FP_MUL_MEAT_2_wide!(_FP_WFRACBITS_D,$R,$X,$Y,umul_ppmm); }; }
#[macro_export]
macro_rules! _FP_DIV_MEAT_S { ($R:ident,$X:ident,$Y:ident) => { _FP_DIV_MEAT_1_udiv_norm!(S,$R,$X,$Y); }; }
#[macro_export]
macro_rules! _FP_DIV_MEAT_D { ($R:ident,$X:ident,$Y:ident) => { _FP_DIV_MEAT_2_udiv!(D,$R,$X,$Y); }; }

#[macro_export] macro_rules! _FP_NANFRAC_S { () => { ((_FP_QNANBIT_S << 1) - 1) }; }
#[macro_export] macro_rules! _FP_NANFRAC_D { () => { ((_FP_QNANBIT_D << 1) - 1), -1 }; }
#[macro_export] macro_rules! _FP_NANFRAC_Q { () => { ((_FP_QNANBIT_Q << 1) - 1), -1, -1, -1 }; }
pub const _FP_NANSIGN_S: i32 = 0;
pub const _FP_NANSIGN_D: i32 = 0;
pub const _FP_NANSIGN_Q: i32 = 0;
pub const _FP_KEEPNANFRACP: i32 = 1;

/* Build-time FP_EX_BOOKE_E500_SPE branch preserved as conditional intent. */
#[cfg(feature = "FP_EX_BOOKE_E500_SPE")]
pub const FP_EX_INEXACT: u32 = 1 << 21;
#[cfg(feature = "FP_EX_BOOKE_E500_SPE")]
pub const FP_EX_INVALID: u32 = 1 << 20;
#[cfg(feature = "FP_EX_BOOKE_E500_SPE")]
pub const FP_EX_DIVZERO: u32 = 1 << 19;
#[cfg(feature = "FP_EX_BOOKE_E500_SPE")]
pub const FP_EX_UNDERFLOW: u32 = 1 << 18;
#[cfg(feature = "FP_EX_BOOKE_E500_SPE")]
pub const FP_EX_OVERFLOW: u32 = 1 << 17;
#[cfg(feature = "FP_EX_BOOKE_E500_SPE")]
pub const FP_INHIBIT_RESULTS: u32 = 0;

#[cfg(not(feature = "FP_EX_BOOKE_E500_SPE"))]
pub const FP_EX_INVALID: u32 = 1 << (31 - 2);
#[cfg(not(feature = "FP_EX_BOOKE_E500_SPE"))]
pub const FP_EX_OVERFLOW: u32 = 1 << (31 - 3);
#[cfg(not(feature = "FP_EX_BOOKE_E500_SPE"))]
pub const FP_EX_UNDERFLOW: u32 = 1 << (31 - 4);
#[cfg(not(feature = "FP_EX_BOOKE_E500_SPE"))]
pub const FP_EX_DIVZERO: u32 = 1 << (31 - 5);
#[cfg(not(feature = "FP_EX_BOOKE_E500_SPE"))]
pub const FP_EX_INEXACT: u32 = 1 << (31 - 6);

/* __FPU_FPSCR and current-thread access are supplied by the kernel context. */
#[macro_export] macro_rules! __FPU_TRAP_P { ($bits:expr) => { ((__FPU_ENABLED_EXC & ($bits)) != 0) }; }
#[macro_export] macro_rules! FP_ROUNDMODE { () => { (__FPU_FPSCR & 0x3) }; }

/* C token-pasting macros retained as Rust macro interfaces; their operations are external. */
#[macro_export]
macro_rules! _FP_CHOOSENAN { ($($args:tt)*) => { /* _FP_CHOOSENAN source operation */ }; }
#[macro_export]
macro_rules! __FP_PACK_S { ($val:expr,$X:expr) => {{ let __exc = _FP_PACK_CANONICAL!(S,1,$X); if __exc == 0 || !__FPU_TRAP_P!(__exc) { _FP_PACK_RAW_1_P!(S,$val,$X); } __exc }}; }
#[macro_export] macro_rules! __FP_PACK_D { ($val:expr,$X:expr) => {{ _FP_PACK_CANONICAL!(D,2,$X); if !FP_CUR_EXCEPTIONS || !__FPU_TRAP_P!(FP_CUR_EXCEPTIONS) { _FP_PACK_RAW_2_P!(D,$val,$X); } }}; }
#[macro_export] macro_rules! __FP_PACK_DS { ($val:expr,$X:expr) => {{ /* conversion sequence is provided by the soft-float headers */ }}; }

#[macro_export]
macro_rules! add_ssaaaa { ($sh:expr,$sl:expr,$ah:expr,$al:expr,$bh:expr,$bl:expr) => {{
    let __sum = ($al as u64).wrapping_add($bl as u64);
    $sl = __sum as UWtype; $sh = ($ah as u64).wrapping_add($bh as u64).wrapping_add(__sum >> W_TYPE_SIZE) as UWtype;
}}; }
#[macro_export]
macro_rules! sub_ddmmss { ($sh:expr,$sl:expr,$ah:expr,$al:expr,$bh:expr,$bl:expr) => {{
    let __lo = ($al as u64).wrapping_sub($bl as u64); $sl = __lo as UWtype;
    $sh = ($ah as u64).wrapping_sub($bh as u64).wrapping_sub((($al as u64) < ($bl as u64)) as u64) as UWtype;
}}; }

#[macro_export]
macro_rules! umul_ppmm { ($ph:expr,$pl:expr,$m0:expr,$m1:expr) => {{ let __p = ($m0 as u64).wrapping_mul($m1 as u64); $ph = (__p >> 32) as UWtype; $pl = __p as UWtype; }}; }
#[macro_export]
macro_rules! udiv_qrnnd { ($q:expr,$r:expr,$n1:expr,$n0:expr,$d:expr) => {{ let __n = (($n1 as u64) << 32) | ($n0 as u64); $q = (__n / ($d as u64)) as UWtype; $r = (__n % ($d as u64)) as UWtype; }}; }
pub const UDIV_NEEDS_NORMALIZATION: i32 = 1;

/* C abort() macro returns zero from the enclosing function. */
#[macro_export] macro_rules! abort { () => { return 0 }; }

#[cfg(target_endian = "big")]
pub const __BYTE_ORDER: u32 = __BIG_ENDIAN;
#[cfg(target_endian = "little")]
pub const __BYTE_ORDER: u32 = __LITTLE_ENDIAN;

pub const EFLAG_INVALID: u32 = 1 << (31 - 2);
pub const EFLAG_OVERFLOW: u32 = 1 << (31 - 3);
pub const EFLAG_UNDERFLOW: u32 = 1 << (31 - 4);
pub const EFLAG_DIVZERO: u32 = 1 << (31 - 5);
pub const EFLAG_INEXACT: u32 = 1 << (31 - 6);
pub const EFLAG_VXSNAN: u32 = 1 << (31 - 7);
pub const EFLAG_VXISI: u32 = 1 << (31 - 8);
pub const EFLAG_VXIDI: u32 = 1 << (31 - 9);
pub const EFLAG_VXZDZ: u32 = 1 << (31 - 10);
pub const EFLAG_VXIMZ: u32 = 1 << (31 - 11);
pub const EFLAG_VXVC: u32 = 1 << (31 - 12);
pub const EFLAG_VXSOFT: u32 = 1 << (31 - 21);
pub const EFLAG_VXSQRT: u32 = 1 << (31 - 22);
pub const EFLAG_VXCVI: u32 = 1 << (31 - 23);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
