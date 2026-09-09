// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2002-2007 H. Peter Anvin - All Rights Reserved
 *
 * Make RAID-6 tables.  This is a host user space program to be run at compile
 * time.
 */

// C dependencies: <stdio.h>, <string.h>, <inttypes.h>, <stdlib.h>, <time.h>

unsafe fn gfmul(mut a: u8, mut b: u8) -> u8 {
    let mut v: u8 = 0;

    while b != 0 {
        if b & 1 != 0 {
            v ^= a;
        }
        a = (a.wrapping_shl(1)) ^ if a & 0x80 != 0 { 0x1d } else { 0 };
        b >>= 1;
    }

    v
}

unsafe fn gfpow(mut a: u8, mut b: i32) -> u8 {
    let mut v: u8 = 1;

    b %= 255;
    if b < 0 {
        b += 255;
    }

    while b != 0 {
        if b & 1 != 0 {
            v = gfmul(v, a);
        }
        a = gfmul(a, a);
        b >>= 1;
    }

    v
}

pub unsafe fn main(_argc: i32, _argv: *mut *mut i8) -> i32 {
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut v: u8;
    let mut exptbl = [0u8; 256];
    let mut invtbl = [0u8; 256];

    print!("#include <linux/export.h>\n");
    print!("#include \"algos.h\"\n");

    /* Compute multiplication table */
    print!("\nconst u8  __attribute__((aligned(256)))\nraid6_gfmul[256][256] =\n{{\n");
    i = 0;
    while i < 256 {
        print!("\t{{\n");
        j = 0;
        while j < 256 {
            print!("\t\t");
            k = 0;
            while k < 8 {
                print!("0x{:02x},{}", gfmul(i as u8, (j + k) as u8), if k == 7 { '\n' } else { ' ' });
                k += 1;
            }
            j += 8;
        }
        print!("\t}},\n");
        i += 1;
    }
    print!("}};\n");
    print!("EXPORT_SYMBOL(raid6_gfmul);\n");

    /* Compute vector multiplication table */
    print!("\nconst u8  __attribute__((aligned(256)))\nraid6_vgfmul[256][32] =\n{{\n");
    i = 0;
    while i < 256 {
        print!("\t{{\n");
        j = 0;
        while j < 16 {
            print!("\t\t");
            k = 0;
            while k < 8 {
                print!("0x{:02x},{}", gfmul(i as u8, (j + k) as u8), if k == 7 { '\n' } else { ' ' });
                k += 1;
            }
            j += 8;
        }
        j = 0;
        while j < 16 {
            print!("\t\t");
            k = 0;
            while k < 8 {
                print!("0x{:02x},{}", gfmul(i as u8, ((j + k) << 4) as u8), if k == 7 { '\n' } else { ' ' });
                k += 1;
            }
            j += 8;
        }
        print!("\t}},\n");
        i += 1;
    }
    print!("}};\n");
    print!("EXPORT_SYMBOL(raid6_vgfmul);\n");

    /* Compute power-of-2 table (exponent) */
    v = 1;
    print!("\nconst u8 __attribute__((aligned(256)))\nraid6_gfexp[256] =\n{{\n");
    i = 0;
    while i < 256 {
        print!("\t");
        j = 0;
        while j < 8 {
            exptbl[(i + j) as usize] = v;
            print!("0x{:02x},{}", v, if j == 7 { '\n' } else { ' ' });
            v = gfmul(v, 2);
            if v == 1 { v = 0; }
            j += 1;
        }
        i += 8;
    }
    print!("}};\n");
    print!("EXPORT_SYMBOL(raid6_gfexp);\n");

    /* Compute log-of-2 table */
    print!("\nconst u8 __attribute__((aligned(256)))\nraid6_gflog[256] =\n{{\n");
    i = 0;
    while i < 256 {
        print!("\t");
        j = 0;
        while j < 8 {
            v = 255;
            k = 0;
            while k < 256 {
                if exptbl[k as usize] == (i + j) as u8 { v = k as u8; break; }
                k += 1;
            }
            print!("0x{:02x},{}", v, if j == 7 { '\n' } else { ' ' });
            j += 1;
        }
        i += 8;
    }
    print!("}};\n");
    print!("EXPORT_SYMBOL(raid6_gflog);\n");

    /* Compute inverse table x^-1 == x^254 */
    print!("\nconst u8 __attribute__((aligned(256)))\nraid6_gfinv[256] =\n{{\n");
    i = 0;
    while i < 256 {
        print!("\t");
        j = 0;
        while j < 8 {
            invtbl[(i + j) as usize] = gfpow((i + j) as u8, 254);
            v = invtbl[(i + j) as usize];
            print!("0x{:02x},{}", v, if j == 7 { '\n' } else { ' ' });
            j += 1;
        }
        i += 8;
    }
    print!("}};\n");
    print!("EXPORT_SYMBOL(raid6_gfinv);\n");

    /* Compute inv(2^x + 1) (exponent-xor-inverse) table */
    print!("\nconst u8 __attribute__((aligned(256)))\nraid6_gfexi[256] =\n{{\n");
    i = 0;
    while i < 256 {
        print!("\t");
        j = 0;
        while j < 8 {
            print!("0x{:02x},{}", invtbl[(exptbl[(i + j) as usize] ^ 1) as usize], if j == 7 { '\n' } else { ' ' });
            j += 1;
        }
        i += 8;
    }
    print!("}};\n");
    print!("EXPORT_SYMBOL(raid6_gfexi);\n");

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
