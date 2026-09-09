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
{
    let rs = (*rsc).codec;
    let mut i: i32;
    let mut j: i32;
    let mut pad: i32;
    let nn = (*rs).nn;
    let nroots = (*rs).nroots;
    let alpha_to = (*rs).alpha_to;
    let index_of = (*rs).index_of;
    let genpoly = (*rs).genpoly;
    let mut fb: u16;
    let msk = (*rs).nn as u16;

    /* Check length parameter for validity */
    pad = nn - nroots - len;
    if pad < 0 || pad >= nn {
        return -ERANGE;
    }

    i = 0;
    while i < len {
        fb = unsafe {
            *index_of.offset(
                (((((*data.offset(i as isize)) as u16) ^ invmsk) & msk)
                    ^ *par) as isize,
            )
        };
        /* feedback term is non-zero */
        if fb != nn as u16 {
            j = 1;
            while j < nroots {
                unsafe {
                    *par.offset(j as isize) ^= *alpha_to.offset(
                        rs_modnn(
                            rs,
                            fb as i32 + *genpoly.offset((nroots - j) as isize) as i32,
                        ) as isize,
                    );
                }
                j += 1;
            }
        }
        /* Shift */
        unsafe {
            std::ptr::copy(
                par.offset(1),
                par,
                (nroots - 1) as usize,
            );
        }
        if fb != nn as u16 {
            unsafe {
                *par.offset((nroots - 1) as isize) = *alpha_to.offset(
                    rs_modnn(
                        rs,
                        fb as i32 + *genpoly as i32,
                    ) as isize,
                );
            }
        } else {
            unsafe {
                *par.offset((nroots - 1) as isize) = 0;
            }
        }
        i += 1;
    }
    return 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
