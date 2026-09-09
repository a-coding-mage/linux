// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto library utility functions
 *
 * Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
 */

/* Kernel configuration conditions represented as Rust feature conditions. */

/*
 * XOR @len bytes from @src1 and @src2 together, writing the result to @dst
 * (which may alias one of the sources).  Don't call this directly; call
 * crypto_xor() or crypto_xor_cpy() instead.
 */
pub unsafe fn __crypto_xor(
    mut dst: *mut u8,
    mut src1: *const u8,
    mut src2: *const u8,
    mut len: u32,
) {
    let mut relalign: usize = 0;

    if !cfg!(feature = "config_have_efficient_unaligned_access") {
        let size = core::mem::size_of::<usize>();
        let d = ((((dst as usize) ^ (src1 as usize)) |
            ((dst as usize) ^ (src2 as usize))) & (size - 1));

        relalign = if d != 0 {
            1usize << d.trailing_zeros()
        } else {
            size
        };

        /*
         * If we care about alignment, process as many bytes as
         * needed to advance dst and src to values whose alignments
         * equal their relative alignment. This will allow us to
         * process the remainder of the input using optimal strides.
         */
        while ((dst as usize) & (relalign - 1)) != 0 && len > 0 {
            *dst = *src1 ^ *src2;
            dst = dst.add(1);
            src1 = src1.add(1);
            src2 = src2.add(1);
            len -= 1;
        }
    }

    while cfg!(feature = "config_64bit") && len >= 8 && (relalign & 7) == 0 {
        if cfg!(feature = "config_have_efficient_unaligned_access") {
            let l = (src1 as *const u64).read_unaligned() ^
                (src2 as *const u64).read_unaligned();
            (dst as *mut u64).write_unaligned(l);
        } else {
            *(dst as *mut u64) = *(src1 as *const u64) ^ *(src2 as *const u64);
        }
        dst = dst.add(8);
        src1 = src1.add(8);
        src2 = src2.add(8);
        len -= 8;
    }

    while len >= 4 && (relalign & 3) == 0 {
        if cfg!(feature = "config_have_efficient_unaligned_access") {
            let l = (src1 as *const u32).read_unaligned() ^
                (src2 as *const u32).read_unaligned();
            (dst as *mut u32).write_unaligned(l);
        } else {
            *(dst as *mut u32) = *(src1 as *const u32) ^ *(src2 as *const u32);
        }
        dst = dst.add(4);
        src1 = src1.add(4);
        src2 = src2.add(4);
        len -= 4;
    }

    while len >= 2 && (relalign & 1) == 0 {
        if cfg!(feature = "config_have_efficient_unaligned_access") {
            let l = (src1 as *const u16).read_unaligned() ^
                (src2 as *const u16).read_unaligned();
            (dst as *mut u16).write_unaligned(l);
        } else {
            *(dst as *mut u16) = *(src1 as *const u16) ^ *(src2 as *const u16);
        }
        dst = dst.add(2);
        src1 = src1.add(2);
        src2 = src2.add(2);
        len -= 2;
    }

    while len != 0 {
        *dst = *src1 ^ *src2;
        dst = dst.add(1);
        src1 = src1.add(1);
        src2 = src2.add(1);
        len -= 1;
    }
}

// EXPORT_SYMBOL_GPL(__crypto_xor);
// MODULE_DESCRIPTION("Crypto library utility functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
