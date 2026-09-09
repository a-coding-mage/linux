// SPDX-License-Identifier: GPL-2.0
/*
 * Generic Reed Solomon encoder / decoder library
 *
 * Copyright 2002, Phil Karn, KA9Q
 * May be used under the terms of the GNU General Public License (GPL)
 *
 * Adaption to the kernel by Thomas Gleixner (tglx@kernel.org)
 *
 * Generic data width independent code which is included by the wrappers.
 */
unsafe {
    let rs = (*rsc).codec;
    let mut deg_lambda: i32;
    let mut el: i32;
    let mut deg_omega: i32;
    let mut i: i32;
    let mut j: i32;
    let mut r: i32;
    let mut k: i32;
    let mut pad: i32;
    let nn = (*rs).nn;
    let nroots = (*rs).nroots;
    let fcr = (*rs).fcr;
    let prim = (*rs).prim;
    let iprim = (*rs).iprim;
    let alpha_to = (*rs).alpha_to;
    let index_of = (*rs).index_of;
    let mut u: u16;
    let mut q: u16;
    let mut tmp: u16;
    let mut num1: u16;
    let mut num2: u16;
    let mut den: u16;
    let mut discr_r: u16;
    let mut syn_error: u16;
    let mut count: i32 = 0;
    let mut num_corrected: i32;
    let msk = nn as u16;

    /*
     * The decoder buffers are in the rs control struct. They are
     * arrays sized [nroots + 1]
     */
    let lambda = (*rsc).buffers.add(RS_DECODE_LAMBDA as usize * (nroots + 1) as usize);
    let syn = (*rsc).buffers.add(RS_DECODE_SYN as usize * (nroots + 1) as usize);
    let b = (*rsc).buffers.add(RS_DECODE_B as usize * (nroots + 1) as usize);
    let t = (*rsc).buffers.add(RS_DECODE_T as usize * (nroots + 1) as usize);
    let omega = (*rsc).buffers.add(RS_DECODE_OMEGA as usize * (nroots + 1) as usize);
    let root = (*rsc).buffers.add(RS_DECODE_ROOT as usize * (nroots + 1) as usize);
    let reg = (*rsc).buffers.add(RS_DECODE_REG as usize * (nroots + 1) as usize);
    let loc = (*rsc).buffers.add(RS_DECODE_LOC as usize * (nroots + 1) as usize);

    /* Check length parameter for validity */
    pad = nn - nroots - len;
    BUG_ON(pad < 0 || pad >= nn - nroots);

    /* Does the caller provide the syndrome ? */
    if !s.is_null() {
        i = 0;
        while i < nroots {
            /* The syndrome is in index form, so nn represents zero */
            if *s.add(i as usize) != nn as u16 {
                break;
            }
            i += 1;
        }
        if i == nroots {
            /* syndrome is zero, no errors to correct */
            return 0;
        }
    }

    /* form the syndromes; i.e., evaluate data(x) at roots of g(x) */
    i = 0;
    while i < nroots {
        *syn.add(i as usize) = ((*data as u16) ^ invmsk) & msk;
        i += 1;
    }
    j = 1;
    while j < len {
        i = 0;
        while i < nroots {
            if *syn.add(i as usize) == 0 {
                *syn.add(i as usize) = ((*data.add(j as usize) as u16) ^ invmsk) & msk;
            } else {
                *syn.add(i as usize) = (((*data.add(j as usize) as u16) ^ invmsk) & msk)
                    ^ *alpha_to.add(rs_modnn(rs, (*index_of.add(*syn.add(i as usize) as usize) as i32) + (fcr + i) * prim) as usize);
            }
            i += 1;
        }
        j += 1;
    }
    j = 0;
    while j < nroots {
        i = 0;
        while i < nroots {
            if *syn.add(i as usize) == 0 {
                *syn.add(i as usize) = (*par.add(j as usize) as u16) & msk;
            } else {
                *syn.add(i as usize) = ((*par.add(j as usize) as u16) & msk)
                    ^ *alpha_to.add(rs_modnn(rs, (*index_of.add(*syn.add(i as usize) as usize) as i32) + (fcr + i) * prim) as usize);
            }
            i += 1;
        }
        j += 1;
    }
    s = syn;

    /* Convert syndromes to index form, checking for nonzero condition */
    syn_error = 0;
    i = 0;
    while i < nroots {
        syn_error |= *s.add(i as usize);
        *s.add(i as usize) = *index_of.add(*s.add(i as usize) as usize);
        i += 1;
    }
    if syn_error == 0 {
        /* if syndrome is zero, data[] is a codeword and there are no errors to correct */
        return 0;
    }

    core::ptr::write_bytes(lambda.add(1), 0, nroots as usize);
    *lambda = 1;

    if no_eras > 0 {
        /* Init lambda to be the erasure locator polynomial */
        *lambda.add(1) = *alpha_to.add(rs_modnn(rs, prim * (nn - 1 - (*eras_pos as i32 + pad))) as usize);
        i = 1;
        while i < no_eras {
            u = rs_modnn(rs, prim * (nn - 1 - (*eras_pos.add(i as usize) as i32 + pad))) as u16;
            j = i + 1;
            while j > 0 {
                tmp = *index_of.add(*lambda.add((j - 1) as usize) as usize);
                if tmp != nn as u16 {
                    let v = rs_modnn(rs, u as i32 + tmp as i32) as usize;
                    *lambda.add(j as usize) ^= *alpha_to.add(v);
                }
                j -= 1;
            }
            i += 1;
        }
    }

    i = 0;
    while i < nroots + 1 {
        *b.add(i as usize) = *index_of.add(*lambda.add(i as usize) as usize);
        i += 1;
    }

    /* Begin Berlekamp-Massey algorithm to determine error+erasure locator polynomial */
    r = no_eras;
    el = no_eras;
    while { r += 1; r } <= nroots {
        discr_r = 0;
        i = 0;
        while i < r {
            if *lambda.add(i as usize) != 0 && *s.add((r - i - 1) as usize) != nn as u16 {
                discr_r ^= *alpha_to.add(rs_modnn(rs, *index_of.add(*lambda.add(i as usize) as usize) as i32 + *s.add((r - i - 1) as usize) as i32) as usize);
            }
            i += 1;
        }
        discr_r = *index_of.add(discr_r as usize);
        if discr_r == nn as u16 {
            core::ptr::copy(b, b.add(1), nroots as usize);
            *b = nn as u16;
        } else {
            *t = *lambda;
            i = 0;
            while i < nroots {
                *t.add((i + 1) as usize) = if *b.add(i as usize) != nn as u16 {
                    *lambda.add((i + 1) as usize) ^ *alpha_to.add(rs_modnn(rs, discr_r as i32 + *b.add(i as usize) as i32) as usize)
                } else { *lambda.add((i + 1) as usize) };
                i += 1;
            }
            if 2 * el <= r + no_eras - 1 {
                el = r + no_eras - el;
                i = 0;
                while i <= nroots {
                    *b.add(i as usize) = if *lambda.add(i as usize) == 0 { nn as u16 } else {
                        rs_modnn(rs, *index_of.add(*lambda.add(i as usize) as usize) as i32 - discr_r as i32 + nn) as u16
                    };
                    i += 1;
                }
            } else {
                core::ptr::copy(b, b.add(1), nroots as usize);
                *b = nn as u16;
            }
            core::ptr::copy_nonoverlapping(t, lambda, (nroots + 1) as usize);
        }
    }

    deg_lambda = 0;
    i = 0;
    while i < nroots + 1 {
        *lambda.add(i as usize) = *index_of.add(*lambda.add(i as usize) as usize);
        if *lambda.add(i as usize) != nn as u16 { deg_lambda = i; }
        i += 1;
    }
    if deg_lambda == 0 { return -EBADMSG; }

    /* Find roots of error+erasure locator polynomial by Chien search */
    core::ptr::copy_nonoverlapping(lambda.add(1), reg.add(1), nroots as usize);
    count = 0;
    i = 1;
    k = iprim - 1;
    while i <= nn {
        q = 1;
        j = deg_lambda;
        while j > 0 {
            if *reg.add(j as usize) != nn as u16 {
                *reg.add(j as usize) = rs_modnn(rs, *reg.add(j as usize) as i32 + j) as u16;
                q ^= *alpha_to.add(*reg.add(j as usize) as usize);
            }
            j -= 1;
        }
        if q == 0 {
            if k < pad { return -EBADMSG; }
            *root.add(count as usize) = i as u16;
            *loc.add(count as usize) = k as u16;
            count += 1;
            if count == deg_lambda { break; }
        }
        i += 1;
        k = rs_modnn(rs, k + iprim);
    }
    if deg_lambda != count { return -EBADMSG; }

    /* Compute err+eras evaluator poly omega(x) = s(x)*lambda(x) (modulo x**nroots). */
    deg_omega = deg_lambda - 1;
    i = 0;
    while i <= deg_omega {
        tmp = 0;
        j = i;
        while j >= 0 {
            if *s.add((i - j) as usize) != nn as u16 && *lambda.add(j as usize) != nn as u16 {
                tmp ^= *alpha_to.add(rs_modnn(rs, *s.add((i - j) as usize) as i32 + *lambda.add(j as usize) as i32) as usize);
            }
            j -= 1;
        }
        *omega.add(i as usize) = *index_of.add(tmp as usize);
        i += 1;
    }

    /* Compute error values in poly-form. */
    num_corrected = 0;
    j = count - 1;
    while j >= 0 {
        num1 = 0;
        i = deg_omega;
        while i >= 0 {
            if *omega.add(i as usize) != nn as u16 {
                num1 ^= *alpha_to.add(rs_modnn(rs, *omega.add(i as usize) as i32 + i * *root.add(j as usize) as i32) as usize);
            }
            i -= 1;
        }
        if num1 == 0 { *b.add(j as usize) = 0; j -= 1; continue; }
        num2 = *alpha_to.add(rs_modnn(rs, *root.add(j as usize) as i32 * (fcr - 1) + nn) as usize);
        den = 0;
        i = (core::cmp::min(deg_lambda, nroots - 1)) & !1;
        while i >= 0 {
            if *lambda.add((i + 1) as usize) != nn as u16 {
                den ^= *alpha_to.add(rs_modnn(rs, *lambda.add((i + 1) as usize) as i32 + i * *root.add(j as usize) as i32) as usize);
            }
            i -= 2;
        }
        *b.add(j as usize) = *alpha_to.add(rs_modnn(rs, *index_of.add(num1 as usize) as i32 + *index_of.add(num2 as usize) as i32 + nn - *index_of.add(den as usize) as i32) as usize);
        num_corrected += 1;
        j -= 1;
    }

    /* Check that the computed error syndrome matches the received syndrome. */
    i = 0;
    while i < nroots {
        tmp = 0;
        j = 0;
        while j < count {
            if *b.add(j as usize) != 0 {
                k = (fcr + i) * prim * (nn - *loc.add(j as usize) as i32 - 1);
                tmp ^= *alpha_to.add(rs_modnn(rs, *index_of.add(*b.add(j as usize) as usize) as i32 + k) as usize);
            }
            j += 1;
        }
        if tmp != *alpha_to.add(*s.add(i as usize) as usize) { return -EBADMSG; }
        i += 1;
    }

    /* Store the error correction pattern, if a correction buffer is available */
    if !corr.is_null() && !eras_pos.is_null() {
        j = 0;
        i = 0;
        while i < count {
            if *b.add(i as usize) != 0 {
                *corr.add(j as usize) = *b.add(i as usize);
                *eras_pos.add(j as usize) = *loc.add(i as usize) as i32 - pad;
                j += 1;
            }
            i += 1;
        }
    } else if !data.is_null() && !par.is_null() {
        i = 0;
        while i < count {
            if *loc.add(i as usize) as i32 < nn - nroots {
                *data.add((*loc.add(i as usize) as i32 - pad) as usize) ^= *b.add(i as usize) as _;
            } else {
                *par.add((*loc.add(i as usize) as i32 - pad - len) as usize) ^= *b.add(i as usize) as _;
            }
            i += 1;
        }
    }

    num_corrected
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
