// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

// Original build condition:
// ((defined(__TARGET_ARCH_arm64) || defined(__TARGET_ARCH_x86) ||
//   (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) ||
//   defined(__TARGET_ARCH_arm) || defined(__TARGET_ARCH_s390) ||
//   defined(__TARGET_ARCH_loongarch)) &&
//  __clang_major__ >= 18)

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_ktime_get_ns() -> u64;
}

// SEC("socket")
// __description("BSWAP, 16")
// __success __success_unpriv __retval(0x23ff)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bswap_16() {
    unsafe {
        core::arch::asm!(
            "r0 = 0xff23;",
            "r0 = bswap16 r0;",
            "exit;",
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("BSWAP, 32")
// __success __success_unpriv __retval(0x23ff0000)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bswap_32() {
    unsafe {
        core::arch::asm!(
            "r0 = 0xff23;",
            "r0 = bswap32 r0;",
            "exit;",
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("BSWAP, 64")
// __success __success_unpriv __retval(0x34ff12ff)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bswap_64() {
    const U64_VAL: u64 = 0xff12ff34ff56ff78u64;

    unsafe {
        core::arch::asm!(
            "r0 = {u64_val} ll;",
            "r0 = bswap64 r0;",
            "exit;",
            u64_val = const U64_VAL,
            options(noreturn)
        );
    }
}

macro_rules! bswap_range_test {
    ($name:ident, $op:literal, $in_value:literal, $out_value:literal) => {
        // SEC("socket")
        // __success __log_level(2)
        // __msg("r0 &= {{.*}}; R0=scalar({{.*}},var_off=(0x0; " #in_value "))")
        // __msg("r0 = " op " r0 {{.*}}; R0=scalar({{.*}},var_off=(0x0; " #out_value "))")
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name() {
            unsafe {
                core::arch::asm!(
                    "call {bpf_get_prandom_u32};",
                    concat!("r0 &= ", stringify!($in_value), ";"),
                    concat!("r0 =  ", $op, " r0;"),
                    concat!("r2 =  ", stringify!($out_value), " ll;"),
                    "if r0 > r2 goto 2f;",
                    "r0 = 0;",
                    "exit;",
                    "2:",
                    "r1 = 42;",
                    "r0 = *(u64 *)(r1 + 0);",
                    "exit;",
                    bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
                    options(noreturn)
                );
            }
        }
    };
}

bswap_range_test!(bswap16_range, "bswap16", 0x3f00, 0x3f);
bswap_range_test!(bswap32_range, "bswap32", 0x3f00, 0x3f0000);
bswap_range_test!(bswap64_range, "bswap64", 0x3f00, 0x3f000000000000);

// Original C conditional: #if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
#[cfg(target_endian = "little")]
bswap_range_test!(be16_range, "be16", 0x3f00, 0x3f);
#[cfg(target_endian = "little")]
bswap_range_test!(be32_range, "be32", 0x3f00, 0x3f0000);
#[cfg(target_endian = "little")]
bswap_range_test!(be64_range, "be64", 0x3f00, 0x3f000000000000);
#[cfg(target_endian = "little")]
bswap_range_test!(le16_range, "le16", 0x3f00, 0x3f00);
#[cfg(target_endian = "little")]
bswap_range_test!(le32_range, "le32", 0x3f00, 0x3f00);
#[cfg(target_endian = "little")]
bswap_range_test!(le64_range, "le64", 0x3f00, 0x3f00);

#[cfg(not(target_endian = "little"))]
bswap_range_test!(be16_range, "be16", 0x3f00, 0x3f00);
#[cfg(not(target_endian = "little"))]
bswap_range_test!(be32_range, "be32", 0x3f00, 0x3f00);
#[cfg(not(target_endian = "little"))]
bswap_range_test!(be64_range, "be64", 0x3f00, 0x3f00);
#[cfg(not(target_endian = "little"))]
bswap_range_test!(le16_range, "le16", 0x3f00, 0x3f);
#[cfg(not(target_endian = "little"))]
bswap_range_test!(le32_range, "le32", 0x3f00, 0x3f0000);
#[cfg(not(target_endian = "little"))]
bswap_range_test!(le64_range, "le64", 0x3f00, 0x3f000000000000);

// SEC("socket")
// __description("BSWAP, reset reg id")
// __failure __msg("math between fp pointer and register with unbounded min value is not allowed")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bswap_reset_reg_id() {
    unsafe {
        core::arch::asm!(
            "call {bpf_ktime_get_ns};",
            "r1 = r0;",
            "r0 = be16 r0;",
            "if r0 != 1 goto 2f;",
            "r2 = r10;",
            "r2 += -512;",
            "r2 += r1;",
            "*(u8 *)(r2 + 0) = 0;",
            "2:",
            "r0 = 0;",
            "exit;",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            options(noreturn)
        );
    }
}

// Original #else for unsupported compiler/JIT targets:
// SEC("socket")
// __description("cpuv4 is not supported by compiler or jit, use a dummy test")
// __success
#[cfg(any())]
#[unsafe(no_mangle)]
pub extern "C" fn dummy_test() -> i32 {
    0
}

// char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
