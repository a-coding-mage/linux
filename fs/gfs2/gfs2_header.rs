/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

#[repr(i32)]
pub enum Create {
    NO_CREATE = 0,
    CREATE = 1,
}

#[repr(i32)]
pub enum Force {
    NO_FORCE = 0,
    FORCE = 1,
}

pub const GFS2_FAST_NAME_SIZE: usize = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
