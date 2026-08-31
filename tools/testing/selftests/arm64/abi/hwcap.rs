// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 ARM Limited.
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const TESTS_PER_HWCAP: usize = 3;

// From linux/auxvec.h when unavailable in the build headers.
const AT_HWCAP3: c_ulong = 29;

type bool_ = bool;
type u64 = u64;
type size_t = usize;
type FILE = c_void;
type sigset_t = c_void;

#[repr(C)]
struct mcontext_t {
    pc: c_ulong,
}

#[repr(C)]
struct ucontext_t {
    uc_mcontext: mcontext_t,
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

type sighandler_fn = unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void);

#[repr(C)]
struct sigaction {
    sa_sigaction: Option<sighandler_fn>,
    sa_mask: sigset_t,
    sa_flags: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;

    static AT_HWCAP: c_ulong;
    static AT_HWCAP2: c_ulong;
    static HWCAP_AES: c_ulong;
    static HWCAP_CMPBR: c_ulong;
    static HWCAP_CRC32: c_ulong;
    static HWCAP_F8MM8: c_ulong;
    static HWCAP_F8MM4: c_ulong;
    static HWCAP_FP: c_ulong;
    static HWCAP_FPRCVT: c_ulong;
    static HWCAP_GCS: c_ulong;
    static HWCAP_JSCVT: c_ulong;
    static HWCAP_LRCPC: c_ulong;
    static HWCAP_ILRCPC: c_ulong;
    static HWCAP_ATOMICS: c_ulong;
    static HWCAP_USCAT: c_ulong;
    static HWCAP_PMULL: c_ulong;
    static HWCAP_SHA1: c_ulong;
    static HWCAP_SHA2: c_ulong;
    static HWCAP_SHA512: c_ulong;
    static HWCAP_SME2P2: c_ulong;
    static HWCAP_SME_AES: c_ulong;
    static HWCAP_SME_SBITPERM: c_ulong;
    static HWCAP_SME_SFEXPA: c_ulong;
    static HWCAP_SME_SMOP4: c_ulong;
    static HWCAP_SME_STMOP: c_ulong;
    static HWCAP_SVE: c_ulong;
    static HWCAP_SVE2P2: c_ulong;
    static HWCAP_SVE_AES2: c_ulong;
    static HWCAP_SVE_BFSCALE: c_ulong;
    static HWCAP_SVE_ELTPERM: c_ulong;
    static HWCAP_SVE_F16MM: c_ulong;

    static HWCAP2_CSSC: c_ulong;
    static HWCAP2_F8CVT: c_ulong;
    static HWCAP2_F8DP4: c_ulong;
    static HWCAP2_F8DP2: c_ulong;
    static HWCAP2_F8E5M2: c_ulong;
    static HWCAP2_F8E4M3: c_ulong;
    static HWCAP2_F8FMA: c_ulong;
    static HWCAP2_FAMINMAX: c_ulong;
    static HWCAP2_FPMR: c_ulong;
    static HWCAP2_LRCPC3: c_ulong;
    static HWCAP2_LSE128: c_ulong;
    static HWCAP2_LUT: c_ulong;
    static HWCAP2_MOPS: c_ulong;
    static HWCAP2_POE: c_ulong;
    static HWCAP2_RNG: c_ulong;
    static HWCAP2_RPRFM: c_ulong;
    static HWCAP2_SME: c_ulong;
    static HWCAP2_SME2: c_ulong;
    static HWCAP2_SME2P1: c_ulong;
    static HWCAP2_SME_I16I32: c_ulong;
    static HWCAP2_SME_BI32I32: c_ulong;
    static HWCAP2_SME_B16B16: c_ulong;
    static HWCAP2_SME_F16F16: c_ulong;
    static HWCAP2_SME_F8F16: c_ulong;
    static HWCAP2_SME_F8F32: c_ulong;
    static HWCAP2_SME_LUTV2: c_ulong;
    static HWCAP2_SME_SF8FMA: c_ulong;
    static HWCAP2_SME_SF8DP2: c_ulong;
    static HWCAP2_SME_SF8DP4: c_ulong;
    static HWCAP2_SVE2: c_ulong;
    static HWCAP2_SVE2P1: c_ulong;
    static HWCAP2_SVEAES: c_ulong;
    static HWCAP2_SVE_B16B16: c_ulong;
    static HWCAP2_SVEPMULL: c_ulong;
    static HWCAP2_SVEBITPERM: c_ulong;
    static HWCAP2_SVESHA3: c_ulong;
    static HWCAP2_SVESM4: c_ulong;
    static HWCAP2_SVEI8MM: c_ulong;
    static HWCAP2_SVEF32MM: c_ulong;
    static HWCAP2_SVEF64MM: c_ulong;
    static HWCAP2_SVEBF16: c_ulong;
    static HWCAP2_SVE_EBF16: c_ulong;
    static HWCAP2_HBC: c_ulong;

    static HWCAP3_F16MM: c_ulong;
    static HWCAP3_F16F32DOT: c_ulong;
    static HWCAP3_F16F32MM: c_ulong;
    static HWCAP3_LSFE: c_ulong;
    static HWCAP3_SME2P3: c_ulong;
    static HWCAP3_SME_LUT6: c_ulong;
    static HWCAP3_SVE2P3: c_ulong;
    static HWCAP3_SVE_B16MM: c_ulong;
    static HWCAP3_SVE_LUT6: c_ulong;
    static HWCAP3_MTE_FAR: c_ulong;
    static HWCAP3_MTE_STORE_ONLY: c_ulong;
    static HWCAP3_LS64: c_ulong;

    static SA_SIGINFO: c_int;
    static SA_RESTART: c_int;
    static SIGILL: c_int;
    static SIGBUS: c_int;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;

    fn ksft_print_header();
    fn ksft_set_plan(plan: usize);
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result(result: bool_, format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_exit_fail_msg(format: *const c_char, ...) -> !;
    fn ksft_print_cnts();
}

/*
 * Function expected to generate exception when the feature is not
 * supported and return when it is supported. If the specific exception
 * is generated then the handler must be able to skip over the
 * instruction safely.
 *
 * Note that it is expected that for many architecture extensions
 * there are no specific traps due to no architecture state being
 * added so we may not fault if running on a kernel which doesn't know
 * to add the hwcap.
 */
type sig_fn = unsafe fn();

unsafe fn aes_sigill() { /* AESE V0.16B, V0.16B */ asm!(".inst 0x4e284800", options(nostack, preserves_flags)); }
unsafe fn atomics_sigill() { /* STADD W0, [SP] */ asm!(".inst 0xb82003ff"); }
unsafe fn cmpbr_sigill() { asm!(".inst 0x74C00040\nudf #0", options(nostack)); }
unsafe fn crc32_sigill() { /* CRC32W W0, W0, W1 */ asm!(".inst 0x1ac14800", options(nostack, preserves_flags)); }
unsafe fn cssc_sigill() { /* CNT x0, x0 */ asm!(".inst 0xdac01c00", out("x0") _); }
unsafe fn f8cvt_sigill() { /* FSCALE V0.4H, V0.4H, V0.4H */ asm!(".inst 0x2ec03c00"); }
unsafe fn f8dp2_sigill() { /* FDOT V0.4H, V0.4H, V0.5H */ asm!(".inst 0xe40fc00"); }
unsafe fn f8dp4_sigill() { /* FDOT V0.2S, V0.2S, V0.2S */ asm!(".inst 0xe00fc00"); }
unsafe fn f8fma_sigill() { /* FMLALB V0.8H, V0.16B, V0.16B */ asm!(".inst 0xec0fc00"); }
unsafe fn f8mm4_sigill() { /* FMMLA V0.4SH, V0.16B, V0.16B */ asm!(".inst 0x6e00ec00"); }
unsafe fn f8mm8_sigill() { /* FMMLA V0.4S, V0.16B, V0.16B */ asm!(".inst 0x6e80ec00"); }
unsafe fn f16f32dot_sigill() { /* FDOT V0.2S, V0.4H, V0.2H[0] */ asm!(".inst 0xf409000"); }
unsafe fn f16f32mm_sigill() { /* FMMLA V0.4S, V0.8H, V0.8H */ asm!(".inst 0x4e40ec00"); }
unsafe fn f16mm_sigill() { /* FMMLA V0.8H, V0.8H, V0.8H */ asm!(".inst 0x4ec0ec00"); }
unsafe fn faminmax_sigill() { /* FAMIN V0.4H, V0.4H, V0.4H */ asm!(".inst 0x2ec01c00"); }
unsafe fn fp_sigill() { asm!("fmov s0, #1"); }
unsafe fn fpmr_sigill() { asm!("mrs x0, S3_3_C4_C4_2", out("x0") _); }
unsafe fn fprcvt_sigill() { /* FCVTAS S0, H0 */ asm!(".inst 0x1efa0000"); }
unsafe fn gcs_sigill() { let gcspr: *mut c_ulong; asm!("mrs {0}, S3_3_C2_C5_1", out(reg) gcspr, options(nostack)); let _ = gcspr; }
unsafe fn ilrcpc_sigill() { /* LDAPUR W0, [SP, #8] */ asm!(".inst 0x994083e0", options(nostack, preserves_flags)); }
unsafe fn jscvt_sigill() { /* FJCVTZS W0, D0 */ asm!(".inst 0x1e7e0000", options(nostack, preserves_flags)); }
unsafe fn lrcpc_sigill() { /* LDAPR W0, [SP, #0] */ asm!(".inst 0xb8bfc3e0", options(nostack, preserves_flags)); }

#[repr(align(16))]
struct Align16U64([u64; 2]);
#[repr(align(16))]
struct Align16F32(f32);
#[repr(align(64))]
struct Align64Char([c_char; 64]);

unsafe fn lse128_sigill() {
    let mut mem = Align16U64([10, 20]);
    let mut memp: *mut u64 = mem.0.as_mut_ptr();
    let mut val0: u64 = 5;
    let mut val1: u64 = 4;
    /* SWPP X1, X2, [X0] */
    asm!(".inst 0x19228001", inout("x0") memp, inout("x1") val0, inout("x2") val1, options(nostack));
    let _ = (memp, val0, val1);
}

unsafe fn lsfe_sigill() {
    let mut mem = Align16F32(0.0);
    let mut memp: *mut f32 = &mut mem.0;
    /* STFADD H0, [X0] */
    asm!(".inst 0x7c20801f", inout("x0") memp, options(nostack, preserves_flags));
    let _ = memp;
}

unsafe fn lut_sigill() { /* LUTI2 V0.16B, { V0.16B }, V[0] */ asm!(".inst 0x4e801000"); }
unsafe fn sve_lut6_sigill() { /* LUTI6 Z0.H, { Z0.H, Z1.H }, Z0[0] */ asm!(".inst 0x4560ac00"); }

unsafe fn mops_sigill() {
    let mut dst: [c_char; 1] = [0];
    let mut src: [c_char; 1] = [0];
    let mut dstp = dst.as_mut_ptr();
    let mut srcp = src.as_mut_ptr();
    let mut size: isize = 1;
    /* CPYP [x0]!, [x1]!, x2! */
    asm!(".inst 0x1d010440", inout("x0") dstp, inout("x1") srcp, inout("x2") size, options(nostack));
    let _ = (dstp, srcp, size);
}

unsafe fn pmull_sigill() { /* PMULL V0.1Q, V0.1D, V0.1D */ asm!(".inst 0x0ee0e000", options(nostack, preserves_flags)); }
unsafe fn poe_sigill() { /* mrs x0, POR_EL0 */ asm!("mrs x0, S3_3_C10_C2_4", out("x0") _); }
unsafe fn rng_sigill() { asm!("mrs x0, S3_3_C2_C4_0", out("x0") _); }
unsafe fn sha1_sigill() { /* SHA1H S0, S0 */ asm!(".inst 0x5e280800", options(nostack, preserves_flags)); }
unsafe fn sha2_sigill() { /* SHA256H Q0, Q0, V0.4S */ asm!(".inst 0x5e004000", options(nostack, preserves_flags)); }
unsafe fn sha512_sigill() { /* SHA512H Q0, Q0, V0.2D */ asm!(".inst 0xce608000", options(nostack, preserves_flags)); }
unsafe fn sme_sigill() { /* RDSVL x0, #0 */ asm!(".inst 0x04bf5800", out("x0") _); }

unsafe fn sme2_sigill() { asm!("msr S0_3_C4_C5_3, xzr"); asm!(".inst 0xc0480001"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn sme2p1_sigill() { asm!("msr S0_3_C4_C3_3, xzr"); asm!(".inst 0xc120C000"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn sme2p2_sigill() { asm!("msr S0_3_C4_C3_3, xzr"); asm!(".inst 0x4c1a000"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn sme2p3_sigill() { asm!("msr S0_3_C4_C3_3, xzr"); asm!(".inst 0x4207800", out("z0") _); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn sme_aes_sigill() { asm!("msr S0_3_C4_C3_3, xzr"); asm!(".inst 0x4522e400", out("z0") _); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn sme_sbitperm_sigill() { asm!("msr S0_3_C4_C3_3, xzr"); asm!(".inst 0x4500b400", out("z0") _); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smei16i32_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0xa0800000"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smebi32i32_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0x80800008"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smeb16b16_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0xC1E41C00"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smef16f16_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0xc1a41C00"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smef8f16_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0xc1a01020"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smef8f32_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0xc1500038"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smelut6_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0xc08a0000"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smelutv2_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0xc08b0000"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smesf8dp2_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0x64204400"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smesf8dp4_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0xc1a41C00"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smesf8fma_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0x64205000"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smesfexpa_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0x04e0b800"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smesmop4_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0x80108000"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn smestmop_sigill() { asm!("msr S0_3_C4_C7_3, xzr"); asm!(".inst 0x80408008"); asm!("msr S0_3_C4_C6_3, xzr"); }
unsafe fn sve_sigill() { /* RDVL x0, #0 */ asm!(".inst 0x04bf5000", out("x0") _); }
unsafe fn sve2_sigill() { /* SQABS Z0.b, P0/M, Z0.B */ asm!(".inst 0x4408A000", out("z0") _); }
unsafe fn sve2p1_sigill() { /* LD1Q {Z0.Q}, P0/Z, [Z0.D, X0] */ asm!(".inst 0xC400A000", out("z0") _); }
unsafe fn sve2p2_sigill() { /* NOT Z0.D, P0/Z, Z0.D */ asm!(".inst 0x4cea000", out("z0") _); }
unsafe fn sve2p3_sigill() { /* ADDQP Z0.B, Z0.B, Z0.B */ asm!(".inst 0x4207800", out("z0") _); }
unsafe fn sveaes_sigill() { /* AESD z0.b, z0.b, z0.b */ asm!(".inst 0x4522e400", out("z0") _); }
unsafe fn sveaes2_sigill() { /* AESD {Z0.B - Z1.B }, { Z0.B - Z1.B }, Z0.Q */ asm!(".inst 0x4522ec00", out("z0") _); }
unsafe fn sveb16b16_sigill() { /* BFADD Z0.H, Z0.H, Z0.H */ asm!(".inst 0x65000000", options(nostack, preserves_flags)); }
unsafe fn sveb16mm_sigill() { /* BFMMLA Z0.H, Z0.H, Z0.H */ asm!(".inst 0x64e0e000", options(nostack, preserves_flags)); }
unsafe fn svebfscale_sigill() { /* BFSCALE Z0.H, P0/M, Z0.H, Z0.H */ asm!(".inst 0x65098000", out("z0") _); }
unsafe fn svef16mm_sigill() { /* FMMLA Z0.S, Z0.H, Z0.H */ asm!(".inst 0x6420e400"); }
unsafe fn svepmull_sigill() { /* PMULLB Z0.Q, Z0.D, Z0.D */ asm!(".inst 0x45006800", out("z0") _); }
unsafe fn svebitperm_sigill() { /* BDEP Z0.B, Z0.B, Z0.B */ asm!(".inst 0x4500b400", out("z0") _); }
unsafe fn svesha3_sigill() { /* EOR3 Z0.D, Z0.D, Z0.D, Z0.D */ asm!(".inst 0x4203800", out("z0") _); }
unsafe fn sveeltperm_sigill() { /* COMPACT Z0.B, P0, Z0.B */ asm!(".inst 0x5218000", out("x0") _); }
unsafe fn svesm4_sigill() { /* SM4E Z0.S, Z0.S, Z0.S */ asm!(".inst 0x4523e000", out("z0") _); }
unsafe fn svei8mm_sigill() { /* USDOT Z0.S, Z0.B, Z0.B[0] */ asm!(".inst 0x44a01800", out("z0") _); }
unsafe fn svef32mm_sigill() { /* FMMLA Z0.S, Z0.S, Z0.S */ asm!(".inst 0x64a0e400", out("z0") _); }
unsafe fn svef64mm_sigill() { /* FMMLA Z0.D, Z0.D, Z0.D */ asm!(".inst 0x64e0e400", out("z0") _); }
unsafe fn svebf16_sigill() { /* BFCVT Z0.H, P0/M, Z0.S */ asm!(".inst 0x658aa000", out("z0") _); }
unsafe fn hbc_sigill() { /* BC.EQ +4 */ asm!("cmp xzr, xzr\n.inst 0x54000030"); }
unsafe fn uscat_sigbus() { asm!("ADD x1, sp, #2", options(nostack, preserves_flags)); asm!(".inst 0xb820003f", options(nostack, preserves_flags)); }

unsafe fn lrcpc3_sigill() {
    let data: [c_int; 2] = [1, 2];
    let src = data.as_ptr();
    let data0: c_int;
    let data1: c_int;
    /* LDIAPP w2, w3, [x0] */
    asm!(".inst 0x99431802", out("w2") data0, out("w3") data1, in("x0") src, options(nostack, preserves_flags));
    let _ = (data0, data1);
}

unsafe extern "C" fn ignore_signal(_sig: c_int, _info: *mut siginfo_t, context: *mut c_void) {
    let uc = context as *mut ucontext_t;
    (*uc).uc_mcontext.pc += 4;
}

unsafe fn ls64_sigill() {
    let mut ign: sigaction = zeroed();
    let mut old: sigaction = zeroed();
    let mut src = Align64Char([0; 64]);
    src.0[0] = 1;

    /*
     * LS64 requires target memory to be Device/Non-cacheable (if
     * FEAT_LS64WB not supported) and the completer supports these
     * instructions, otherwise we'll receive a SIGBUS. Since we are only
     * testing the ABI here, so just ignore the SIGBUS and see if we can
     * execute the instructions without receiving a SIGILL. Restore the
     * handler of SIGBUS after this test.
     */
    ign.sa_sigaction = Some(ignore_signal);
    ign.sa_flags = SA_SIGINFO | SA_RESTART;
    sigemptyset(&mut ign.sa_mask);
    sigaction(SIGBUS, &ign, &mut old);

    let xn = src.0.as_mut_ptr() as *mut c_void;
    let xt_1: u64;
    asm!(".inst 0xf83fd100", out("x0") xt_1, in("x8") xn, out("x1") _, out("x2") _, out("x3") _, out("x4") _, out("x5") _, out("x6") _, out("x7") _);
    asm!(".inst 0xf83f9100", in("x0") xt_1, in("x8") xn, out("x1") _, out("x2") _, out("x3") _, out("x4") _, out("x5") _, out("x6") _, out("x7") _);

    sigaction(SIGBUS, &old, ptr::null_mut());
}

#[repr(C)]
struct hwcap_data {
    name: *const c_char,
    at_hwcap: c_ulong,
    hwcap_bit: c_ulong,
    cpuinfo: *const c_char,
    sigill_fn: Option<sig_fn>,
    sigill_reliable: bool_,
    sigbus_fn: Option<sig_fn>,
    sigbus_reliable: bool_,
}

macro_rules! hw {
    ($name:expr, $at:expr, $bit:expr, $cpu:expr $(, sigill = $sigill:ident)? $(, sigill_reliable = $sr:expr)? $(, sigbus = $sigbus:ident)? $(, sigbus_reliable = $br:expr)? $(,)?) => {
        hwcap_data {
            name: concat!($name, "\0").as_ptr() as *const c_char,
            at_hwcap: $at,
            hwcap_bit: $bit,
            cpuinfo: concat!($cpu, "\0").as_ptr() as *const c_char,
            sigill_fn: hw!(@optfn $($sigill)?),
            sigill_reliable: hw!(@bool $($sr)?),
            sigbus_fn: hw!(@optfn $($sigbus)?),
            sigbus_reliable: hw!(@bool $($br)?),
        }
    };
    (@optfn) => { None };
    (@optfn $f:ident) => { Some($f) };
    (@bool) => { false };
    (@bool $v:expr) => { $v };
}

static mut HWCAPS: [hwcap_data; 79] = unsafe {
    [
        hw!("AES", AT_HWCAP, HWCAP_AES, "aes", sigill = aes_sigill),
        hw!("CMPBR", AT_HWCAP, HWCAP_CMPBR, "cmpbr", sigill = cmpbr_sigill),
        hw!("CRC32", AT_HWCAP, HWCAP_CRC32, "crc32", sigill = crc32_sigill),
        hw!("CSSC", AT_HWCAP2, HWCAP2_CSSC, "cssc", sigill = cssc_sigill),
        hw!("F8CVT", AT_HWCAP2, HWCAP2_F8CVT, "f8cvt", sigill = f8cvt_sigill),
        hw!("F8DP4", AT_HWCAP2, HWCAP2_F8DP4, "f8dp4", sigill = f8dp4_sigill),
        hw!("F8DP2", AT_HWCAP2, HWCAP2_F8DP2, "f8dp2", sigill = f8dp2_sigill),
        hw!("F8E5M2", AT_HWCAP2, HWCAP2_F8E5M2, "f8e5m2"),
        hw!("F8E4M3", AT_HWCAP2, HWCAP2_F8E4M3, "f8e4m3"),
        hw!("F8FMA", AT_HWCAP2, HWCAP2_F8FMA, "f8fma", sigill = f8fma_sigill),
        hw!("F8MM8", AT_HWCAP, HWCAP_F8MM8, "f8mm8", sigill = f8mm8_sigill),
        hw!("F8MM4", AT_HWCAP, HWCAP_F8MM4, "f8mm4", sigill = f8mm4_sigill),
        hw!("F16MM", AT_HWCAP3, HWCAP3_F16MM, "f16mm", sigill = f16mm_sigill),
        hw!("F16F32DOT", AT_HWCAP3, HWCAP3_F16F32DOT, "f16f32dot", sigill = f16f32dot_sigill),
        hw!("F16F32MM", AT_HWCAP3, HWCAP3_F16F32MM, "f16f32mm", sigill = f16f32mm_sigill),
        hw!("FAMINMAX", AT_HWCAP2, HWCAP2_FAMINMAX, "faminmax", sigill = faminmax_sigill),
        hw!("FP", AT_HWCAP, HWCAP_FP, "fp", sigill = fp_sigill),
        hw!("FPMR", AT_HWCAP2, HWCAP2_FPMR, "fpmr", sigill = fpmr_sigill, sigill_reliable = true),
        hw!("FPRCVT", AT_HWCAP, HWCAP_FPRCVT, "fprcvt", sigill = fprcvt_sigill),
        hw!("GCS", AT_HWCAP, HWCAP_GCS, "gcs", sigill = gcs_sigill, sigill_reliable = true),
        hw!("JSCVT", AT_HWCAP, HWCAP_JSCVT, "jscvt", sigill = jscvt_sigill),
        hw!("LRCPC", AT_HWCAP, HWCAP_LRCPC, "lrcpc", sigill = lrcpc_sigill),
        hw!("LRCPC2", AT_HWCAP, HWCAP_ILRCPC, "ilrcpc", sigill = ilrcpc_sigill),
        hw!("LRCPC3", AT_HWCAP2, HWCAP2_LRCPC3, "lrcpc3", sigill = lrcpc3_sigill),
        hw!("LSE", AT_HWCAP, HWCAP_ATOMICS, "atomics", sigill = atomics_sigill),
        hw!("LSE2", AT_HWCAP, HWCAP_USCAT, "uscat", sigill = atomics_sigill, sigbus = uscat_sigbus, sigbus_reliable = true),
        hw!("LSE128", AT_HWCAP2, HWCAP2_LSE128, "lse128", sigill = lse128_sigill),
        hw!("LSFE", AT_HWCAP3, HWCAP3_LSFE, "lsfe", sigill = lsfe_sigill),
        hw!("LUT", AT_HWCAP2, HWCAP2_LUT, "lut", sigill = lut_sigill),
        hw!("MOPS", AT_HWCAP2, HWCAP2_MOPS, "mops", sigill = mops_sigill, sigill_reliable = true),
        hw!("PMULL", AT_HWCAP, HWCAP_PMULL, "pmull", sigill = pmull_sigill),
        hw!("POE", AT_HWCAP2, HWCAP2_POE, "poe", sigill = poe_sigill, sigill_reliable = true),
        hw!("RNG", AT_HWCAP2, HWCAP2_RNG, "rng", sigill = rng_sigill),
        hw!("RPRFM", AT_HWCAP2, HWCAP2_RPRFM, "rprfm"),
        hw!("SHA1", AT_HWCAP, HWCAP_SHA1, "sha1", sigill = sha1_sigill),
        hw!("SHA2", AT_HWCAP, HWCAP_SHA2, "sha2", sigill = sha2_sigill),
        hw!("SHA512", AT_HWCAP, HWCAP_SHA512, "sha512", sigill = sha512_sigill),
        hw!("SME", AT_HWCAP2, HWCAP2_SME, "sme", sigill = sme_sigill, sigill_reliable = true),
        hw!("SME2", AT_HWCAP2, HWCAP2_SME2, "sme2", sigill = sme2_sigill, sigill_reliable = true),
        hw!("SME 2.1", AT_HWCAP2, HWCAP2_SME2P1, "sme2p1", sigill = sme2p1_sigill),
        hw!("SME 2.2", AT_HWCAP, HWCAP_SME2P2, "sme2p2", sigill = sme2p2_sigill),
        hw!("SME 2.3", AT_HWCAP3, HWCAP3_SME2P3, "sme2p3", sigill = sme2p3_sigill),
        hw!("SME AES", AT_HWCAP, HWCAP_SME_AES, "smeaes", sigill = sme_aes_sigill),
        hw!("SME I16I32", AT_HWCAP2, HWCAP2_SME_I16I32, "smei16i32", sigill = smei16i32_sigill),
        hw!("SME BI32I32", AT_HWCAP2, HWCAP2_SME_BI32I32, "smebi32i32", sigill = smebi32i32_sigill),
        hw!("SME B16B16", AT_HWCAP2, HWCAP2_SME_B16B16, "smeb16b16", sigill = smeb16b16_sigill),
        hw!("SME F16F16", AT_HWCAP2, HWCAP2_SME_F16F16, "smef16f16", sigill = smef16f16_sigill),
        hw!("SME F8F16", AT_HWCAP2, HWCAP2_SME_F8F16, "smef8f16", sigill = smef8f16_sigill),
        hw!("SME F8F32", AT_HWCAP2, HWCAP2_SME_F8F32, "smef8f32", sigill = smef8f32_sigill),
        hw!("SME LUT6", AT_HWCAP3, HWCAP3_SME_LUT6, "smelut6", sigill = smelut6_sigill),
        hw!("SME LUTV2", AT_HWCAP2, HWCAP2_SME_LUTV2, "smelutv2", sigill = smelutv2_sigill),
        hw!("SME SBITPERM", AT_HWCAP, HWCAP_SME_SBITPERM, "smesbitperm", sigill = sme_sbitperm_sigill),
        hw!("SME SF8FMA", AT_HWCAP2, HWCAP2_SME_SF8FMA, "smesf8fma", sigill = smesf8fma_sigill),
        hw!("SME SF8DP2", AT_HWCAP2, HWCAP2_SME_SF8DP2, "smesf8dp2", sigill = smesf8dp2_sigill),
        hw!("SME SF8DP4", AT_HWCAP2, HWCAP2_SME_SF8DP4, "smesf8dp4", sigill = smesf8dp4_sigill),
        hw!("SME SFEXPA", AT_HWCAP, HWCAP_SME_SFEXPA, "smesfexpa", sigill = smesfexpa_sigill),
        hw!("SME SMOP4", AT_HWCAP, HWCAP_SME_SMOP4, "smesmop4", sigill = smesmop4_sigill),
        hw!("SME STMOP", AT_HWCAP, HWCAP_SME_STMOP, "smestmop", sigill = smestmop_sigill),
        hw!("SVE", AT_HWCAP, HWCAP_SVE, "sve", sigill = sve_sigill, sigill_reliable = true),
        hw!("SVE 2", AT_HWCAP2, HWCAP2_SVE2, "sve2", sigill = sve2_sigill),
        hw!("SVE 2.1", AT_HWCAP2, HWCAP2_SVE2P1, "sve2p1", sigill = sve2p1_sigill),
        hw!("SVE 2.2", AT_HWCAP, HWCAP_SVE2P2, "sve2p2", sigill = sve2p2_sigill),
        hw!("SVE 2.3", AT_HWCAP3, HWCAP3_SVE2P3, "sve2p3", sigill = sve2p3_sigill),
        hw!("SVE AES", AT_HWCAP2, HWCAP2_SVEAES, "sveaes", sigill = sveaes_sigill),
        hw!("SVE AES2", AT_HWCAP, HWCAP_SVE_AES2, "sveaes2", sigill = sveaes2_sigill),
        hw!("SVE B16MM", AT_HWCAP3, HWCAP3_SVE_B16MM, "sveb16mm", sigill = sveb16mm_sigill),
        hw!("SVE BFSCALE", AT_HWCAP, HWCAP_SVE_BFSCALE, "svebfscale", sigill = svebfscale_sigill),
        hw!("SVE ELTPERM", AT_HWCAP, HWCAP_SVE_ELTPERM, "sveeltperm", sigill = sveeltperm_sigill),
        hw!("SVE F16MM", AT_HWCAP, HWCAP_SVE_F16MM, "svef16mm", sigill = svef16mm_sigill),
        hw!("SVE_LUT6", AT_HWCAP3, HWCAP3_SVE_LUT6, "svelut6", sigill = sve_lut6_sigill),
        hw!("SVE2 B16B16", AT_HWCAP2, HWCAP2_SVE_B16B16, "sveb16b16", sigill = sveb16b16_sigill),
        hw!("SVE2 PMULL", AT_HWCAP2, HWCAP2_SVEPMULL, "svepmull", sigill = svepmull_sigill),
        hw!("SVE2 BITPERM", AT_HWCAP2, HWCAP2_SVEBITPERM, "svebitperm", sigill = svebitperm_sigill),
        hw!("SVE2 SHA3", AT_HWCAP2, HWCAP2_SVESHA3, "svesha3", sigill = svesha3_sigill),
        hw!("SVE2 SM4", AT_HWCAP2, HWCAP2_SVESM4, "svesm4", sigill = svesm4_sigill),
        hw!("SVE2 I8MM", AT_HWCAP2, HWCAP2_SVEI8MM, "svei8mm", sigill = svei8mm_sigill),
        hw!("SVE2 F32MM", AT_HWCAP2, HWCAP2_SVEF32MM, "svef32mm", sigill = svef32mm_sigill),
        hw!("SVE2 F64MM", AT_HWCAP2, HWCAP2_SVEF64MM, "svef64mm", sigill = svef64mm_sigill),
        hw!("SVE2 BF16", AT_HWCAP2, HWCAP2_SVEBF16, "svebf16", sigill = svebf16_sigill),
        hw!("SVE2 EBF16", AT_HWCAP2, HWCAP2_SVE_EBF16, "sveebf16"),
        hw!("HBC", AT_HWCAP2, HWCAP2_HBC, "hbc", sigill = hbc_sigill, sigill_reliable = true),
        hw!("MTE_FAR", AT_HWCAP3, HWCAP3_MTE_FAR, "mtefar"),
        hw!("MTE_STOREONLY", AT_HWCAP3, HWCAP3_MTE_STORE_ONLY, "mtestoreonly"),
        hw!("LS64", AT_HWCAP3, HWCAP3_LS64, "ls64", sigill = ls64_sigill, sigill_reliable = true),
    ]
};

static mut seen_sigill: bool_ = false;
static mut seen_sigbus: bool_ = false;

unsafe extern "C" fn handle_sigill(_sig: c_int, _info: *mut siginfo_t, context: *mut c_void) {
    let uc = context as *mut ucontext_t;
    seen_sigill = true;
    /* Skip over the offending instruction */
    (*uc).uc_mcontext.pc += 4;
}

unsafe extern "C" fn handle_sigbus(_sig: c_int, _info: *mut siginfo_t, context: *mut c_void) {
    let uc = context as *mut ucontext_t;
    seen_sigbus = true;
    /* Skip over the offending instruction */
    (*uc).uc_mcontext.pc += 4;
}

unsafe fn cpuinfo_present(name: *const c_char) -> bool_ {
    let mut buf: [c_char; 2048] = [0; 2048];
    let mut name_space: [c_char; 30] = [0; 30];
    let mut name_newline: [c_char; 30] = [0; 30];

    /*
     * The feature should appear with a leading space and either a
     * trailing space or a newline.
     */
    snprintf(name_space.as_mut_ptr(), name_space.len(), c" %s ".as_ptr(), name);
    snprintf(name_newline.as_mut_ptr(), name_newline.len(), c" %s\n".as_ptr(), name);

    let f = fopen(c"/proc/cpuinfo".as_ptr(), c"r".as_ptr());
    if f.is_null() {
        ksft_print_msg(c"Failed to open /proc/cpuinfo\n".as_ptr());
        return false;
    }

    while !fgets(buf.as_mut_ptr(), buf.len() as c_int, f).is_null() {
        /* Features: line? */
        if strncmp(buf.as_ptr(), c"Features\t:".as_ptr(), strlen(c"Features\t:".as_ptr())) != 0 {
            continue;
        }

        /* All CPUs should be symmetric, don't read any more */
        fclose(f);

        let mut s = strstr(buf.as_ptr(), name_space.as_ptr());
        if !s.is_null() {
            return true;
        }
        s = strstr(buf.as_ptr(), name_newline.as_ptr());
        if !s.is_null() {
            return true;
        }

        return false;
    }

    ksft_print_msg(c"Failed to find Features in /proc/cpuinfo\n".as_ptr());
    fclose(f);
    false
}

unsafe fn install_sigaction(signum: c_int, handler: sighandler_fn) -> c_int {
    let mut sa: sigaction = zeroed();
    memset((&mut sa as *mut sigaction).cast(), 0, size_of::<sigaction>());
    sa.sa_sigaction = Some(handler);
    sa.sa_flags = SA_RESTART | SA_SIGINFO;
    sigemptyset(&mut sa.sa_mask);
    let ret = sigaction(signum, &sa, ptr::null_mut());
    if ret < 0 {
        ksft_exit_fail_msg(c"Failed to install SIGNAL handler: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }

    ret
}

unsafe fn uninstall_sigaction(signum: c_int) {
    if sigaction(signum, ptr::null(), ptr::null_mut()) < 0 {
        ksft_exit_fail_msg(c"Failed to uninstall SIGNAL handler: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }
}

unsafe fn inst_raise_sigill(hwcap: *const hwcap_data, have_hwcap: bool_) -> bool_ {
    if (*hwcap).sigill_fn.is_none() {
        ksft_test_result_skip(c"sigill_%s\n".as_ptr(), (*hwcap).name);
        /* assume that it would raise exception in default */
        return true;
    }

    install_sigaction(SIGILL, handle_sigill);

    seen_sigill = false;
    ((*hwcap).sigill_fn.unwrap())();

    if have_hwcap {
        /* Should be able to use the extension */
        ksft_test_result(!seen_sigill, c"sigill_%s\n".as_ptr(), (*hwcap).name);
    } else if (*hwcap).sigill_reliable {
        /* Guaranteed a SIGNAL */
        ksft_test_result(seen_sigill, c"sigill_%s\n".as_ptr(), (*hwcap).name);
    } else {
        /* Missing SIGNAL might be fine */
        ksft_print_msg(c"sigill_%sreported for %s\n".as_ptr(), if seen_sigill { c"".as_ptr() } else { c"not ".as_ptr() }, (*hwcap).name);
        ksft_test_result_skip(c"sigill_%s\n".as_ptr(), (*hwcap).name);
    }

    uninstall_sigaction(SIGILL);
    seen_sigill
}

unsafe fn inst_raise_sigbus(hwcap: *const hwcap_data, have_hwcap: bool_) -> bool_ {
    if (*hwcap).sigbus_fn.is_none() {
        ksft_test_result_skip(c"sigbus_%s\n".as_ptr(), (*hwcap).name);
        /* assume that it would raise exception in default */
        return true;
    }

    install_sigaction(SIGBUS, handle_sigbus);

    seen_sigbus = false;
    ((*hwcap).sigbus_fn.unwrap())();

    if have_hwcap {
        /* Should be able to use the extension */
        ksft_test_result(!seen_sigbus, c"sigbus_%s\n".as_ptr(), (*hwcap).name);
    } else if (*hwcap).sigbus_reliable {
        /* Guaranteed a SIGNAL */
        ksft_test_result(seen_sigbus, c"sigbus_%s\n".as_ptr(), (*hwcap).name);
    } else {
        /* Missing SIGNAL might be fine */
        ksft_print_msg(c"sigbus_%sreported for %s\n".as_ptr(), if seen_sigbus { c"".as_ptr() } else { c"not ".as_ptr() }, (*hwcap).name);
        ksft_test_result_skip(c"sigbus_%s\n".as_ptr(), (*hwcap).name);
    }

    uninstall_sigaction(SIGBUS);
    seen_sigbus
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut i: usize;
    let mut hwcap: *const hwcap_data;
    let mut have_cpuinfo: bool_;
    let mut have_hwcap: bool_;
    let mut raise_sigill: bool_;

    ksft_print_header();
    ksft_set_plan(HWCAPS.len() * TESTS_PER_HWCAP);

    i = 0;
    while i < HWCAPS.len() {
        hwcap = &HWCAPS[i];

        have_hwcap = (getauxval((*hwcap).at_hwcap) & (*hwcap).hwcap_bit) != 0;
        have_cpuinfo = cpuinfo_present((*hwcap).cpuinfo);

        if have_hwcap {
            ksft_print_msg(c"%s present\n".as_ptr(), (*hwcap).name);
        }

        ksft_test_result(have_hwcap == have_cpuinfo, c"cpuinfo_match_%s\n".as_ptr(), (*hwcap).name);

        /*
         * Testing for SIGBUS only makes sense after make sure
         * that the instruction does not cause a SIGILL signal.
         */
        raise_sigill = inst_raise_sigill(hwcap, have_hwcap);
        if !raise_sigill {
            inst_raise_sigbus(hwcap, have_hwcap);
        } else {
            ksft_test_result_skip(c"sigbus_%s\n".as_ptr(), (*hwcap).name);
        }

        i += 1;
    }

    ksft_print_cnts();

    0
}
