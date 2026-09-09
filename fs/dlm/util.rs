// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) 2005-2008 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C dependencies supplied by the surrounding DLM sources are intentionally
// omitted here; this file has no declarations from them that are needed below.

const DLM_ERRNO_EDEADLK: i32 = 35;
const DLM_ERRNO_EBADR: i32 = 53;
const DLM_ERRNO_EBADSLT: i32 = 57;
const DLM_ERRNO_EPROTO: i32 = 71;
const DLM_ERRNO_EOPNOTSUPP: i32 = 95;
const DLM_ERRNO_ETIMEDOUT: i32 = 110;
const DLM_ERRNO_EINPROGRESS: i32 = 115;

// These errno values correspond to the platform errno macros used by the C
// implementation.  Higher errno values are inconsistent across architectures.
const EDEADLK: i32 = 35;
const EBADR: i32 = 53;
const EBADSLT: i32 = 57;
const EPROTO: i32 = 71;
const EOPNOTSUPP: i32 = 95;
const ETIMEDOUT: i32 = 110;
const EINPROGRESS: i32 = 115;

/* higher errno values are inconsistent across architectures, so select
   one set of values for on the wire */
pub fn to_dlm_errno(err: i32) -> i32 {
    match err {
        -EDEADLK => -DLM_ERRNO_EDEADLK,
        -EBADR => -DLM_ERRNO_EBADR,
        -EBADSLT => -DLM_ERRNO_EBADSLT,
        -EPROTO => -DLM_ERRNO_EPROTO,
        -EOPNOTSUPP => -DLM_ERRNO_EOPNOTSUPP,
        -ETIMEDOUT => -DLM_ERRNO_ETIMEDOUT,
        -EINPROGRESS => -DLM_ERRNO_EINPROGRESS,
        _ => err,
    }
}

pub fn from_dlm_errno(err: i32) -> i32 {
    match err {
        -DLM_ERRNO_EDEADLK => -EDEADLK,
        -DLM_ERRNO_EBADR => -EBADR,
        -DLM_ERRNO_EBADSLT => -EBADSLT,
        -DLM_ERRNO_EPROTO => -EPROTO,
        -DLM_ERRNO_EOPNOTSUPP => -EOPNOTSUPP,
        -DLM_ERRNO_ETIMEDOUT => -ETIMEDOUT,
        -DLM_ERRNO_EINPROGRESS => -EINPROGRESS,
        _ => err,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
