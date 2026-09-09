/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) 2005 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
*******************************************************************************/

// External declarations corresponding to the C header's function prototypes.
unsafe extern "C" {
    fn to_dlm_errno(err: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn from_dlm_errno(err: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
