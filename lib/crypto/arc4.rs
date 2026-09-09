// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API
 *
 * ARC4 Cipher Algorithm
 *
 * Jon Oberheide <jon@oberheide.org>
 */

// The corresponding C implementation includes <crypto/arc4.h>, which
// supplies this context type.
#[repr(C)]
pub struct arc4_ctx {
    pub x: u32,
    pub y: u32,
    pub S: [u32; 256],
}

pub unsafe fn arc4_setkey(
    ctx: *mut arc4_ctx,
    in_key: *const u8,
    key_len: u32,
) -> i32 {
    let mut j: u32 = 0;
    let mut k: u32 = 0;

    (*ctx).x = 1;
    (*ctx).y = 0;

    for i in 0..256usize {
        (*ctx).S[i] = i as u32;
    }

    for i in 0..256usize {
        let a: u32 = (*ctx).S[i];

        j = (j + *in_key.add(k as usize) as u32 + a) & 0xff;
        (*ctx).S[i] = (*ctx).S[j as usize];
        (*ctx).S[j as usize] = a;
        k += 1;
        if k >= key_len {
            k = 0;
        }
    }

    0
}

pub unsafe fn arc4_crypt(
    ctx: *mut arc4_ctx,
    mut out: *mut u8,
    mut input: *const u8,
    mut len: u32,
) {
    if len == 0 {
        return;
    }

    let mut x: u32 = (*ctx).x;
    let mut y: u32 = (*ctx).y;

    let mut a: u32 = (*ctx).S[x as usize];
    y = (y + a) & 0xff;
    let mut b: u32 = (*ctx).S[y as usize];

    loop {
        (*ctx).S[y as usize] = a;
        a = (a + b) & 0xff;
        (*ctx).S[x as usize] = b;
        x = (x + 1) & 0xff;
        let ta: u32 = (*ctx).S[x as usize];
        let ty: u32 = (y + ta) & 0xff;
        let tb: u32 = (*ctx).S[ty as usize];
        *out = *input ^ (*ctx).S[a as usize] as u8;
        out = out.add(1);
        input = input.add(1);
        len -= 1;
        if len == 0 {
            break;
        }
        y = ty;
        a = ta;
        b = tb;
    }

    (*ctx).x = x;
    (*ctx).y = y;
}

// EXPORT_SYMBOL(arc4_setkey);
// EXPORT_SYMBOL(arc4_crypt);
// MODULE_DESCRIPTION("ARC4 Cipher Algorithm");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
