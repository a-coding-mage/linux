/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * SGI specific setup.
 *
 * Copyright (C) 1995 - 1997, 1999 Silcon Graphics, Inc.
 * Copyright (C) 1999 Ralf Baechle (ralf@gnu.org)
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, asm/sn/types.h
// Under CONFIG_SGI_IP27, asm/sn/sn0/arch.h is also included.

macro_rules! cputonasid {
    ($cpu:expr) => {
        sn_cpu_info[$cpu].p_nasid
    };
}

macro_rules! cputoslice {
    ($cpu:expr) => {
        sn_cpu_info[$cpu].p_slice
    };
}

pub const INVALID_NASID: nasid_t = (-1i32) as nasid_t;
pub const INVALID_PNODEID: pnodeid_t = (-1i32) as pnodeid_t;
pub const INVALID_MODULE: moduleid_t = (-1i32) as moduleid_t;
pub const INVALID_PARTID: partid_t = (-1i32) as partid_t;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
