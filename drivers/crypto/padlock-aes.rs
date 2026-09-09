// SPDX-License-Identifier: GPL-2.0-only
/* Cryptographic API. Support for VIA PadLock hardware crypto engine. */

// External Linux kernel headers and symbols are intentionally represented by
// their required Rust names; they are supplied by the surrounding kernel.

const mut ECB_FETCH_BLOCKS: u32 = 2;
const MAX_ECB_FETCH_BLOCKS: usize = 8;
const MAX_CBC_FETCH_BLOCKS: usize = 4;
const AES_BLOCK_SIZE: usize = 16;
const AES_MAX_KEYLENGTH_U32: usize = 60 / 4;
const AES_MAX_KEYLENGTH: usize = 60;
const PADLOCK_ALIGNMENT: usize = 16;

static mut CBC_FETCH_BLOCKS: u32 = 1;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
struct Cword {
    bits: u16,
}

#[repr(C, align(16))]
struct AesCtx {
    e: [u32; AES_MAX_KEYLENGTH_U32],
    d_data: [u32; AES_MAX_KEYLENGTH_U32],
    cword: CwordPair,
    d: *mut u32,
}

#[repr(C)]
struct CwordPair { encrypt: Cword, decrypt: Cword }

static mut PAES_LAST_CWORD: *mut Cword = core::ptr::null_mut();

#[inline]
fn aes_hw_extkey_available(key_len: u8) -> i32 {
    if key_len == 16 { 1 } else { 0 }
}

#[inline]
unsafe fn aes_ctx_common(ctx: *mut core::ffi::c_void) -> *mut AesCtx {
    let addr = ctx as usize;
    let align = if PADLOCK_ALIGNMENT <= crypto_tfm_ctx_alignment() { 1 } else { PADLOCK_ALIGNMENT };
    (align_up(addr, align)) as *mut AesCtx
}

#[inline] unsafe fn aes_ctx(tfm: *mut CryptoTfm) -> *mut AesCtx { aes_ctx_common(crypto_tfm_ctx(tfm)) }
#[inline] unsafe fn skcipher_aes_ctx(tfm: *mut CryptoSkcipher) -> *mut AesCtx { aes_ctx_common(crypto_skcipher_ctx(tfm)) }

unsafe fn aes_set_key(tfm: *mut CryptoTfm, in_key: *const u8, key_len: u32) -> i32 {
    let ctx = aes_ctx(tfm);
    if key_len % 8 != 0 { return -22; }
    (*ctx).d = (*ctx).e.as_mut_ptr();
    let key = in_key as *const u32;
    (*ctx).e[0] = u32::from_le(core::ptr::read_unaligned(key));
    (*ctx).e[1] = u32::from_le(core::ptr::read_unaligned(key.add(1)));
    (*ctx).e[2] = u32::from_le(core::ptr::read_unaligned(key.add(2)));
    (*ctx).e[3] = u32::from_le(core::ptr::read_unaligned(key.add(3)));
    (*ctx).cword = CwordPair::default();
    (*ctx).cword.decrypt.bits |= 1 << 9;
    let rounds = 10 + (key_len - 16) / 4;
    (*ctx).cword.encrypt.bits |= (rounds as u16) & 0xf;
    (*ctx).cword.decrypt.bits |= (rounds as u16) & 0xf;
    let ksize = ((key_len - 16) / 8) as u16;
    (*ctx).cword.encrypt.bits |= ksize << 10;
    (*ctx).cword.decrypt.bits |= ksize << 10;
    if aes_hw_extkey_available(key_len as u8) == 0 {
        (*ctx).d = (*ctx).d_data.as_mut_ptr();
        if aes_expandkey(core::ptr::null_mut(), in_key, key_len) != 0 { return -22; }
    }
    PAES_LAST_CWORD = core::ptr::null_mut();
    0
}

unsafe fn aes_set_key_skcipher(tfm: *mut CryptoSkcipher, key: *const u8, len: u32) -> i32 {
    aes_set_key(crypto_skcipher_tfm(tfm), key, len)
}

#[inline] unsafe fn padlock_reset_key(cword: *mut Cword) {
    if cword != PAES_LAST_CWORD {
        core::arch::asm!("pushfq; popfq", options(nostack, preserves_flags));
    }
}
#[inline] unsafe fn padlock_store_cword(cword: *mut Cword) { PAES_LAST_CWORD = cword; }

#[inline] unsafe fn rep_xcrypt_ecb(input: *const u8, output: *mut u8, key: *mut u32, cw: *mut Cword, count: i32) {
    core::arch::asm!(".byte 0xf3,0x0f,0xa7,0xc8", inout("rsi") input => _, inout("rdi") output => _, in("rdx") cw, in("rbx") key, in("rcx") count);
}
#[inline] unsafe fn rep_xcrypt_cbc(input: *const u8, output: *mut u8, key: *mut u32, iv: *mut u8, cw: *mut Cword, count: i32) -> *mut u8 {
    core::arch::asm!(".byte 0xf3,0x0f,0xa7,0xd0", inout("rsi") input => _, inout("rdi") output => _, inout("rax") iv => iv, in("rdx") cw, in("rbx") key, in("rcx") count);
    iv
}

unsafe fn ecb_crypt(in_: *const u8, out: *mut u8, key: *mut u32, cw: *mut Cword, count: i32) {
    rep_xcrypt_ecb(in_, out, key, cw, count);
}
unsafe fn cbc_crypt(in_: *const u8, out: *mut u8, key: *mut u32, iv: *mut u8, cw: *mut Cword, count: i32) -> *mut u8 {
    rep_xcrypt_cbc(in_, out, key, iv, cw, count)
}

unsafe fn padlock_xcrypt_ecb(input: *const u8, output: *mut u8, key: *mut u32, cw: *mut Cword, count: u32) {
    let initial = count & (ECB_FETCH_BLOCKS - 1);
    if count < ECB_FETCH_BLOCKS { ecb_crypt(input, output, key, cw, count as i32); return; }
    if initial != 0 { rep_xcrypt_ecb(input, output, key, cw, initial as i32); }
    rep_xcrypt_ecb(input, output, key, cw, (count - initial) as i32);
}
unsafe fn padlock_xcrypt_cbc(input: *const u8, output: *mut u8, key: *mut u32, iv: *mut u8, cw: *mut Cword, count: u32) -> *mut u8 {
    let blocks = CBC_FETCH_BLOCKS;
    let initial = count & (blocks - 1);
    if count < blocks { return cbc_crypt(input, output, key, iv, cw, count as i32); }
    if initial != 0 { rep_xcrypt_cbc(input, output, key, iv, cw, initial as i32); }
    rep_xcrypt_cbc(input, output, key, iv, cw, (count - initial) as i32)
}

// The remaining registration and skcipher plumbing is kept as external kernel
// interfaces; function declarations preserve the source-level entry points.
extern "C" {
    fn crypto_tfm_ctx_alignment() -> usize;
    fn align_up(addr: usize, align: usize) -> usize;
    fn crypto_tfm_ctx(tfm: *mut CryptoTfm) -> *mut core::ffi::c_void;
    fn crypto_skcipher_ctx(tfm: *mut CryptoSkcipher) -> *mut core::ffi::c_void;
    fn crypto_skcipher_tfm(tfm: *mut CryptoSkcipher) -> *mut CryptoTfm;
    fn aes_expandkey(ctx: *mut core::ffi::c_void, key: *const u8, len: u32) -> i32;
}
#[repr(C)] struct CryptoTfm;
#[repr(C)] struct CryptoSkcipher;

unsafe fn padlock_aes_encrypt(tfm: *mut CryptoTfm, out: *mut u8, input: *const u8) {
    let ctx = aes_ctx(tfm);
    padlock_reset_key(&mut (*ctx).cword.encrypt);
    ecb_crypt(input, out, (*ctx).e.as_mut_ptr(), &mut (*ctx).cword.encrypt, 1);
    padlock_store_cword(&mut (*ctx).cword.encrypt);
}
unsafe fn padlock_aes_decrypt(tfm: *mut CryptoTfm, out: *mut u8, input: *const u8) {
    let ctx = aes_ctx(tfm);
    padlock_reset_key(&mut (*ctx).cword.encrypt);
    ecb_crypt(input, out, (*ctx).d, &mut (*ctx).cword.decrypt, 1);
    padlock_store_cword(&mut (*ctx).cword.encrypt);
}

extern "C" {
    fn ecb_aes_encrypt(req: *mut SkcipherRequest) -> i32;
    fn ecb_aes_decrypt(req: *mut SkcipherRequest) -> i32;
    fn cbc_aes_encrypt(req: *mut SkcipherRequest) -> i32;
    fn cbc_aes_decrypt(req: *mut SkcipherRequest) -> i32;
    fn crypto_register_alg(alg: *mut CryptoAlg) -> i32;
    fn crypto_register_skcipher(alg: *mut SkcipherAlg) -> i32;
    fn crypto_unregister_alg(alg: *mut CryptoAlg);
    fn crypto_unregister_skcipher(alg: *mut SkcipherAlg);
    fn x86_match_cpu(ids: *const X86CpuId) -> bool;
    fn boot_cpu_has(feature: u32) -> bool;
}
#[repr(C)] struct SkcipherRequest;
#[repr(C)] struct CryptoAlg;
#[repr(C)] struct SkcipherAlg;
#[repr(C)] struct X86CpuId;

static mut AES_ALG: *mut CryptoAlg = core::ptr::null_mut();
static mut ECB_AES_ALG: *mut SkcipherAlg = core::ptr::null_mut();
static mut CBC_AES_ALG: *mut SkcipherAlg = core::ptr::null_mut();
static PADLOCK_CPU_ID: [X86CpuId; 1] = unsafe { core::mem::zeroed() };

unsafe fn padlock_init() -> i32 {
    if !x86_match_cpu(PADLOCK_CPU_ID.as_ptr()) { return -19; }
    if !boot_cpu_has(0) { return -19; }
    let mut ret = crypto_register_alg(AES_ALG);
    if ret != 0 { return ret; }
    ret = crypto_register_skcipher(ECB_AES_ALG);
    if ret != 0 { crypto_unregister_alg(AES_ALG); return ret; }
    ret = crypto_register_skcipher(CBC_AES_ALG);
    if ret != 0 {
        crypto_unregister_skcipher(ECB_AES_ALG);
        crypto_unregister_alg(AES_ALG);
    }
    ret
}

unsafe fn padlock_fini() {
    crypto_unregister_skcipher(CBC_AES_ALG);
    crypto_unregister_skcipher(ECB_AES_ALG);
    crypto_unregister_alg(AES_ALG);
}

// module_init(padlock_init); module_exit(padlock_fini);
// MODULE_DESCRIPTION("VIA PadLock AES algorithm support");
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Michal Ludvig");
// MODULE_ALIAS_CRYPTO("aes");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
