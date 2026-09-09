/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * MEI UUID definition
 *
 * Copyright (C) 2010, Intel Corp.
 *	Huang Ying <ying.huang@intel.com>
 */

// Translated from the Linux UAPI header. `__u8` is represented by `u8` here.

#[repr(C)]
#[derive(Clone, Copy)]
pub struct uuid_le {
    pub b: [u8; 16],
}

macro_rules! UUID_LE {
    ($a:expr, $b:expr, $c:expr, $d0:expr, $d1:expr, $d2:expr, $d3:expr,
     $d4:expr, $d5:expr, $d6:expr, $d7:expr) => {
        uuid_le {
            b: [
                (($a as u32) & 0xff) as u8,
                ((($a as u32) >> 8) & 0xff) as u8,
                ((($a as u32) >> 16) & 0xff) as u8,
                ((($a as u32) >> 24) & 0xff) as u8,
                (($b as u32) & 0xff) as u8,
                ((($b as u32) >> 8) & 0xff) as u8,
                (($c as u32) & 0xff) as u8,
                ((($c as u32) >> 8) & 0xff) as u8,
                $d0 as u8,
                $d1 as u8,
                $d2 as u8,
                $d3 as u8,
                $d4 as u8,
                $d5 as u8,
                $d6 as u8,
                $d7 as u8,
            ],
        }
    };
}

pub const NULL_UUID_LE: uuid_le = UUID_LE!(
    0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
