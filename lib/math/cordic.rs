/*
 * Copyright (c) 2011 Broadcom Corporation
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS.
 */

// The Linux module and cordic headers supply the CORDIC_* definitions and
// the cordic_iq type in the surrounding translation unit.

static ARCTAN_TABLE: [i32; 18] = [
    2949120, 1740967, 919879, 466945, 234379, 117304,
    58666, 29335, 14668, 7334, 3667, 1833, 917, 458,
    229, 115, 57, 29,
];

/*
 * cordic_calc_iq() - calculates the i/q coordinate for given angle
 *
 * theta: angle in degrees for which i/q coordinate is to be calculated
 * coord: function output parameter holding the i/q coordinate
 */
pub unsafe fn cordic_calc_iq(mut theta: i32) -> cordic_iq {
    let mut coord: cordic_iq;
    let mut angle: i32;
    let mut valtmp: i32;
    let mut signx: i32 = 1;
    let signtheta: i32;

    coord.i = CORDIC_ANGLE_GEN;
    coord.q = 0;
    angle = 0;

    theta = CORDIC_FIXED(theta);
    signtheta = if theta < 0 { -1 } else { 1 };
    theta = ((theta + CORDIC_FIXED(180) * signtheta) % CORDIC_FIXED(360))
        - CORDIC_FIXED(180) * signtheta;

    if CORDIC_FLOAT(theta) > 90 {
        theta -= CORDIC_FIXED(180);
        signx = -1;
    } else if CORDIC_FLOAT(theta) < -90 {
        theta += CORDIC_FIXED(180);
        signx = -1;
    }

    let mut iter: u32 = 0;
    while iter < CORDIC_NUM_ITER {
        if theta > angle {
            valtmp = coord.i - (coord.q >> iter);
            coord.q += coord.i >> iter;
            angle += ARCTAN_TABLE[iter as usize];
        } else {
            valtmp = coord.i + (coord.q >> iter);
            coord.q -= coord.i >> iter;
            angle -= ARCTAN_TABLE[iter as usize];
        }
        coord.i = valtmp;
        iter += 1;
    }

    coord.i *= signx;
    coord.q *= signx;
    coord
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
