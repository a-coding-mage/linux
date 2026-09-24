# SPDX-License-Identifier: GPL-2.0-only
"""Disposable Base64 callers; shared C code supplies data/oracle, not Rust calls."""
from pathlib import Path

SPECIAL = (b"", b"f", b"fo", b"foo", b"Zg", b"Zg==", b"Zm8", b"Zm8=", b"Zm9v",
           b"Zm9vYg==", b"Zm9vZ===", b"Zm9v====", b"Zm9vZg=", b"Zm9vZg==!",
           b"Zm\0v", b"////", b"//8", b"__8", b",,8", b"+/8", b"-_8", b"+,8",
           b"AAA", b"AAB", b"AA=", b"====")
PAIRS = 65536 * 2
INVALID = 6 * 256 * 8
NEGATIVES = 7 * 6 * 9 * 2
OVERLAPS = 6 * 25 * 33 * 2
COUNT = PAIRS + INVALID + NEGATIVES + OVERLAPS + len(SPECIAL) * 12


def marker(caller):
    if caller not in ("c", "rust"):
        raise ValueError("unknown Base64 caller")
    return f"LUPOS_BASE64_{'RUST_ABI' if caller == 'rust' else 'ABI'}_OK cases={COUNT}".encode()


REFERENCE = r'''
#include <linux/base64.h>
#include <linux/export.h>
#include <linux/kernel.h>
#include <linux/string.h>
int base64_reference_encode(const u8 *, int, char *, bool, enum base64_variant);
int base64_reference_decode(const char *, int, u8 *, bool, enum base64_variant);
int base64_case(unsigned int, unsigned int *, int *, int *, bool *, enum base64_variant *, u8 *);
#undef EXPORT_SYMBOL_GPL
#define EXPORT_SYMBOL_GPL(symbol)
#define base64_encode base64_reference_encode
#define base64_decode base64_reference_decode
#include "@ORIGINAL@"
#undef base64_encode
#undef base64_decode

/* No native selected function is called here: this helper provides only data
 * and the unchanged original C reference. Each caller owns its actual loop. */
int base64_case(unsigned int index, unsigned int *operation, int *length,
                int *delta, bool *padding, enum base64_variant *variant, u8 *data)
{
    static const int negative[] = { (-2147483647 - 1), -2147483647, -65536, -4, -3, -2, -1 };
    static const u8 tails[9][3] = { "Zm8", "Zm9", "AAA", "//8", "__8", ",,8", {255,'A','A'}, {'A',0,'A'}, "AA=" };
    static const u8 special[][32] = { @SPECIAL@ };
    static const u8 sizes[] = { @SIZES@ };
    unsigned int mode, value, size, i;
    *delta = 0;
    if (index < @PAIRS@) {
        *operation = index & 1; value = index >> 1;
        mode = value % 6; data[0] = value; data[1] = value >> 8; size = 2;
    } else if ((index -= @PAIRS@) < @INVALID@) {
        *operation = 1; mode = index / (256 * 8); value = index % (256 * 8);
        memcpy(data, "Zm9vYmFy", 8); data[value % 8] = value / 8; size = 8;
    } else if ((index -= @INVALID@) < @NEGATIVES@) {
        *operation = index % 2; index /= 2;
        value = index % 9; index /= 9; mode = index % 6; index /= 6;
        memcpy(data, tails[value], 3); *length = negative[index];
        *padding = mode & 1; *variant = mode / 2; return 3;
    } else if ((index -= @NEGATIVES@) < @OVERLAPS@) {
        u8 plain[33];
        *operation = index % 2; index /= 2;
        size = index % 33; index /= 33; *delta = (int)(index % 25) - 12; mode = index / 25;
        for (i = 0; i < size; i++) plain[i] = i * 73 + size * 19;
        if (*operation) size = base64_reference_encode(plain, size, (char *)data, mode & 1, mode / 2);
        else memcpy(data, plain, size);
    } else {
        index -= @OVERLAPS@;
        if (index >= ARRAY_SIZE(special) * 12) return -1;
        *operation = index % 2; index /= 2; mode = index % 6; value = index / 6;
        size = sizes[value]; memcpy(data, special[value], size);
    }
    *length = size; *padding = mode & 1; *variant = mode / 2;
    return size;
}
'''

C_CALLER = r'''
#include <linux/base64.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/string.h>
int base64_reference_encode(const u8 *, int, char *, bool, enum base64_variant);
int base64_reference_decode(const char *, int, u8 *, bool, enum base64_variant);
int base64_case(unsigned int, unsigned int *, int *, int *, bool *, enum base64_variant *, u8 *);
static noinline int base64_call_encode(const u8 *s, int n, char *d, bool p, enum base64_variant v)
{
    int (*volatile actual)(const u8 *, int, char *, bool, enum base64_variant) = base64_encode;
    return actual(s, n, d, p, v);
}
static noinline int base64_call_decode(const char *s, int n, u8 *d, bool p, enum base64_variant v)
{
    int (*volatile actual)(const char *, int, u8 *, bool, enum base64_variant) = base64_decode;
    return actual(s, n, d, p, v);
}
static int __init base64_abi_init(void)
{
    unsigned int index, operation;
    u8 input[64], expected[256], got[256];
    enum base64_variant variant;
    int size, length, delta, old, result;
    bool padding;
    for (index = 0; index < @COUNT@; index++) {
        size = base64_case(index, &operation, &length, &delta, &padding, &variant, input);
        if (size < 0 || size > sizeof(input) || delta < -12 || delta > 12) return -EINVAL;
        memset(expected, 0xa5, sizeof(expected)); memset(got, 0xa5, sizeof(got));
        memcpy(expected + 64, input, size); memcpy(got + 64, input, size);
        if (operation) {
            old = base64_reference_decode((char *)expected + 64, length, expected + 64 + delta, padding, variant);
            result = base64_call_decode((char *)got + 64, length, got + 64 + delta, padding, variant);
        } else {
            old = base64_reference_encode(expected + 64, length, (char *)expected + 64 + delta, padding, variant);
            result = base64_call_encode(got + 64, length, (char *)got + 64 + delta, padding, variant);
        }
        if (old != result || memcmp(expected, got, sizeof(got))) {
            pr_err("LUPOS_BASE64_FAILED case=%u expected=%d got=%d\n", index, old, result);
            return -EINVAL;
        }
        cond_resched();
    }
    if (base64_case(index, &operation, &length, &delta, &padding, &variant, input) != -1) return -EINVAL;
    pr_info("@MARKER@\n"); return 0;
}
static void __exit base64_abi_exit(void) {}
module_init(base64_abi_init);
module_exit(base64_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Independent Base64 native ABI and overlap test");
'''

RUST_CALLER = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Independent Rust caller of the original nominal Base64 interface.
use kernel::{bindings, ffi};
use kernel::prelude::*;
#[allow(dead_code, unreachable_pub, unused_imports)]
#[path = "@HEADER@"]
mod base64;
module! { type: Base64RustAbi, name: "base64_rust_abi", authors: ["Lupos"],
          description: "Independent Base64 native ABI and overlap test", license: "GPL", }
unsafe extern "C" {
    fn base64_reference_encode(s: *const u8, n: ffi::c_int, d: *mut ffi::c_char, p: bool, v: bindings::base64_variant) -> ffi::c_int;
    fn base64_reference_decode(s: *const ffi::c_char, n: ffi::c_int, d: *mut u8, p: bool, v: bindings::base64_variant) -> ffi::c_int;
    fn base64_case(index: u32, op: *mut u32, n: *mut i32, delta: *mut i32, p: *mut bool, v: *mut bindings::base64_variant, data: *mut u8) -> i32;
}
/// Preserve a separate actual typed native encoder call site.
/// # Safety
/// Same pointer/length/variant requirements as the original C export.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn base64_call_encode(s: *const u8, n: i32, d: *mut ffi::c_char, p: bool, v: bindings::base64_variant) -> i32 {
    let pointer: unsafe extern "C" fn(*const u8, i32, *mut ffi::c_char, bool, bindings::base64_variant) -> i32 = base64::ffi::base64_encode;
    // SAFETY: The caller provides the original valid pointer/enum domain.
    unsafe { core::ptr::read_volatile(&pointer)(s, n, d, p, v) }
}
/// Preserve a separate actual typed native decoder call site.
/// # Safety
/// Same pointer/length/variant requirements as the original C export.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn base64_call_decode(s: *const ffi::c_char, n: i32, d: *mut u8, p: bool, v: bindings::base64_variant) -> i32 {
    let pointer: unsafe extern "C" fn(*const ffi::c_char, i32, *mut u8, bool, bindings::base64_variant) -> i32 = base64::ffi::base64_decode;
    // SAFETY: Raw pointers allow valid overlap without aliased Rust slices.
    unsafe { core::ptr::read_volatile(&pointer)(s, n, d, p, v) }
}
struct Base64RustAbi;
impl kernel::Module for Base64RustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let mut input = [0u8; 64];
        let mut expected = [0xa5u8; 256];
        let mut got = [0xa5u8; 256];
        let (mut operation, mut length, mut delta, mut padding) = (0, 0, 0, false);
        let mut variant = bindings::base64_variant::BASE64_STD;
        let mut index = 0;
        while index < @COUNT@ {
            // SAFETY: Private data generator initializes these exact fields
            // and at most64 bytes; it never invokes the selected provider.
            let size = unsafe { base64_case(index, &mut operation, &mut length, &mut delta, &mut padding, &mut variant, input.as_mut_ptr()) };
            if !(0..=64).contains(&size) || !(-12..=12).contains(&delta) { return Err(EINVAL); }
            expected.fill(0xa5); got.fill(0xa5);
            expected[64..64 + size as usize].copy_from_slice(&input[..size as usize]);
            got[64..64 + size as usize].copy_from_slice(&input[..size as usize]);
            let at = (64 + delta) as usize;
            // SAFETY: Buffers contain every actual input/output byte, also
            // the original negative unpadded three-byte path. Valid enum is
            // generated by the original-header fixture. No slices alias.
            let (old, result) = unsafe {
                if operation != 0 {
                    (base64_reference_decode(expected.as_ptr().add(64).cast(), length, expected.as_mut_ptr().add(at), padding, variant),
                     base64_call_decode(got.as_ptr().add(64).cast(), length, got.as_mut_ptr().add(at), padding, variant))
                } else {
                    (base64_reference_encode(expected.as_ptr().add(64), length, expected.as_mut_ptr().add(at).cast(), padding, variant),
                     base64_call_encode(got.as_ptr().add(64), length, got.as_mut_ptr().add(at).cast(), padding, variant))
                }
            };
            if old != result || expected != got {
                pr_err!("LUPOS_BASE64_FAILED case={} expected={} got={}\n", index, old, result);
                return Err(EINVAL);
            }
            index += 1;
        }
        // SAFETY: Same initialized private output storage as above.
        if unsafe { base64_case(index, &mut operation, &mut length, &mut delta, &mut padding, &mut variant, input.as_mut_ptr()) } != -1 { return Err(EINVAL); }
        pr_info!("@MARKER@\n"); Ok(Self)
    }
}
impl Drop for Base64RustAbi { fn drop(&mut self) {} }
'''

PROPRIETARY = r'''
#include <linux/base64.h>
#include <linux/init.h>
#include <linux/module.h>
static int __init rejected_init(void) {
    u8 data[8] = {0}; char out[16];
    int (*volatile encode)(const u8 *, int, char *, bool, enum base64_variant) = base64_encode;
    int (*volatile decode)(const char *, int, u8 *, bool, enum base64_variant) = base64_decode;
    return encode(data, 1, out, true, BASE64_STD) + decode("AA==", 4, data, true, BASE64_STD);
}
module_init(rejected_init);
MODULE_LICENSE("Proprietary");
'''


def reference_source(root):
    values = {"ORIGINAL": str(Path(root) / "lib/base64.c"), "PAIRS": PAIRS,
              "INVALID": INVALID, "NEGATIVES": NEGATIVES, "OVERLAPS": OVERLAPS,
              "SPECIAL": ",".join("{" + ",".join(map(str, value)) + "}" for value in SPECIAL),
              "SIZES": ",".join(str(len(value)) for value in SPECIAL)}
    result = REFERENCE
    for key, value in values.items(): result = result.replace("@" + key + "@", str(value))
    return result


def caller_source(root, caller):
    if caller not in ("c", "rust"): raise ValueError("unknown Base64 caller")
    return (RUST_CALLER if caller == "rust" else C_CALLER).replace("@COUNT@", str(COUNT)).replace(
        "@MARKER@", marker(caller).decode()).replace("@HEADER@", str(Path(root) / "include/linux/base64_header.rs"))
