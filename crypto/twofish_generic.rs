// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Twofish for CryptoAPI
 *
 * Originally Twofish for GPG
 * By Matthew Skala <mskala@ansuz.sooke.bc.ca>, July 26, 1998
 * 256-bit key length added March 20, 1999
 * Some modifications to reduce the text size by Werner Koch, April, 1998
 * Ported to the kerneli patch by Marc Mutz <Marc@Mutz.com>
 * Ported to CryptoAPI by Colin Slater <hoho@tacomeat.net>
 *
 * The original author has disclaimed all copyright interest in this
 * code and thus put it in the public domain. The subsequent authors
 * have put this under the GNU General Public License.
 *
 * This code is a "clean room" implementation, written from the paper
 * _Twofish: A 128-Bit Block Cipher_ by Bruce Schneier, John Kelsey,
 * Doug Whiting, David Wagner, Chris Hall, and Niels Ferguson, available
 * through http://www.counterpane.com/twofish.html
 *
 * For background information on multiplication in finite fields, used
 * for the matrix operations in the key schedule, see the book _Contemporary
 * Abstract Algebra_ by Joseph A. Gallian, especially chapter 22 in the
 * Third Edition.
 */

// Dependencies supplied by the surrounding kernel translation.

macro_rules! g1 { ($ctx:expr, $a:expr) => {
    $ctx.s[0][(($a) & 0xff) as usize] ^ $ctx.s[1][((($a) >> 8) & 0xff) as usize]
        ^ $ctx.s[2][((($a) >> 16) & 0xff) as usize] ^ $ctx.s[3][(($a) >> 24) as usize]
} }
macro_rules! g2 { ($ctx:expr, $b:expr) => {
    $ctx.s[1][(($b) & 0xff) as usize] ^ $ctx.s[2][((($b) >> 8) & 0xff) as usize]
        ^ $ctx.s[3][((($b) >> 16) & 0xff) as usize] ^ $ctx.s[0][(($b) >> 24) as usize]
} }

/* Encrypt one block.  in and out may be the same. */
unsafe fn twofish_encrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let ctx: *mut twofish_ctx = crypto_tfm_ctx(tfm);
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut d: u32;
    let mut x: u32;
    let mut y: u32;

    a = get_unaligned_le32(input.add(0)) ^ (*ctx).w[0];
    b = get_unaligned_le32(input.add(4)) ^ (*ctx).w[1];
    c = get_unaligned_le32(input.add(8)) ^ (*ctx).w[2];
    d = get_unaligned_le32(input.add(12)) ^ (*ctx).w[3];

    macro_rules! enc_round { ($n:expr, $aa:expr, $bb:expr, $cc:expr, $dd:expr) => {{
        x = g1!(&*ctx, $aa); y = g2!(&*ctx, $bb);
        x = x.wrapping_add(y); y = y.wrapping_add(x).wrapping_add((*ctx).k[2 * $n + 1]);
        $cc ^= x.wrapping_add((*ctx).k[2 * $n]);
        $cc = ror32($cc, 1); $dd = rol32($dd, 1) ^ y;
    }} }
    macro_rules! enc_cycle { ($n:expr) => {{ enc_round!(2 * $n, a, b, c, d); enc_round!(2 * $n + 1, c, d, a, b); }} }
    enc_cycle!(0); enc_cycle!(1); enc_cycle!(2); enc_cycle!(3);
    enc_cycle!(4); enc_cycle!(5); enc_cycle!(6); enc_cycle!(7);

    c ^= (*ctx).w[4]; put_unaligned_le32(c, out.add(0));
    d ^= (*ctx).w[5]; put_unaligned_le32(d, out.add(4));
    a ^= (*ctx).w[6]; put_unaligned_le32(a, out.add(8));
    b ^= (*ctx).w[7]; put_unaligned_le32(b, out.add(12));
}

/* Decrypt one block.  in and out may be the same. */
unsafe fn twofish_decrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let ctx: *mut twofish_ctx = crypto_tfm_ctx(tfm);
    let mut a = get_unaligned_le32(input.add(8)) ^ (*ctx).w[6];
    let mut b = get_unaligned_le32(input.add(12)) ^ (*ctx).w[7];
    let mut c = get_unaligned_le32(input.add(0)) ^ (*ctx).w[4];
    let mut d = get_unaligned_le32(input.add(4)) ^ (*ctx).w[5];
    let mut x: u32; let mut y: u32;
    macro_rules! dec_round { ($n:expr, $aa:expr, $bb:expr, $cc:expr, $dd:expr) => {{
        x = g1!(&*ctx, $aa); y = g2!(&*ctx, $bb); x = x.wrapping_add(y); y = y.wrapping_add(x);
        $dd ^= y.wrapping_add((*ctx).k[2 * $n + 1]); $dd = ror32($dd, 1);
        $cc = rol32($cc, 1); $cc ^= x.wrapping_add((*ctx).k[2 * $n]);
    }} }
    macro_rules! dec_cycle { ($n:expr) => {{ dec_round!(2 * $n + 1, c, d, a, b); dec_round!(2 * $n, a, b, c, d); }} }
    dec_cycle!(7); dec_cycle!(6); dec_cycle!(5); dec_cycle!(4);
    dec_cycle!(3); dec_cycle!(2); dec_cycle!(1); dec_cycle!(0);
    a ^= (*ctx).w[0]; put_unaligned_le32(a, out.add(0));
    b ^= (*ctx).w[1]; put_unaligned_le32(b, out.add(4));
    c ^= (*ctx).w[2]; put_unaligned_le32(c, out.add(8));
    d ^= (*ctx).w[3]; put_unaligned_le32(d, out.add(12));
}

static mut alg: crypto_alg = crypto_alg {
    cra_name: "twofish", cra_driver_name: "twofish-generic", cra_priority: 100,
    cra_flags: CRYPTO_ALG_TYPE_CIPHER, cra_blocksize: TF_BLOCK_SIZE,
    cra_ctxsize: core::mem::size_of::<twofish_ctx>(), cra_module: THIS_MODULE,
    cra_u: crypto_alg_union { cipher: cipher_alg { cia_min_keysize: TF_MIN_KEY_SIZE,
        cia_max_keysize: TF_MAX_KEY_SIZE, cia_setkey: twofish_setkey,
        cia_encrypt: twofish_encrypt, cia_decrypt: twofish_decrypt } },
};

unsafe fn twofish_mod_init() -> i32 { crypto_register_alg(&raw mut alg) }
unsafe fn twofish_mod_fini() { crypto_unregister_alg(&raw mut alg); }

// module_init(twofish_mod_init); module_exit(twofish_mod_fini);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("Twofish Cipher Algorithm");
// MODULE_ALIAS_CRYPTO("twofish"); MODULE_ALIAS_CRYPTO("twofish-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
