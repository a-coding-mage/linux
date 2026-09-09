/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2012 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, Version 2, as
 * published by the Free Software Foundation.
 ***********************license end**************************************/

// C header dependency: CVMX_ADD_IO_SEG is supplied by the surrounding bindings.
pub const CVMX_RNM_BIST_STATUS: u64 = CVMX_ADD_IO_SEG(0x0001180040000008u64);
pub const CVMX_RNM_CTL_STATUS: u64 = CVMX_ADD_IO_SEG(0x0001180040000000u64);
pub const CVMX_RNM_EER_DBG: u64 = CVMX_ADD_IO_SEG(0x0001180040000018u64);
pub const CVMX_RNM_EER_KEY: u64 = CVMX_ADD_IO_SEG(0x0001180040000010u64);
pub const CVMX_RNM_SERIAL_NUM: u64 = CVMX_ADD_IO_SEG(0x0001180040000020u64);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_rnm_bist_status_s { pub bits: u64 }
#[repr(C)]
pub union cvmx_rnm_bist_status { pub u64_: u64, pub s: cvmx_rnm_bist_status_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_rnm_ctl_status_s { pub bits: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_rnm_ctl_status_cn30xx { pub bits: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_rnm_ctl_status_cn50xx { pub bits: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_rnm_ctl_status_cn63xx { pub bits: u64 }
#[repr(C)]
pub union cvmx_rnm_ctl_status {
    pub u64_: u64,
    pub s: cvmx_rnm_ctl_status_s,
    pub cn30xx: cvmx_rnm_ctl_status_cn30xx,
    pub cn50xx: cvmx_rnm_ctl_status_cn50xx,
    pub cn63xx: cvmx_rnm_ctl_status_cn63xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_rnm_eer_dbg_s { pub dat: u64 }
#[repr(C)]
pub union cvmx_rnm_eer_dbg { pub u64_: u64, pub s: cvmx_rnm_eer_dbg_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_rnm_eer_key_s { pub key: u64 }
#[repr(C)]
pub union cvmx_rnm_eer_key { pub u64_: u64, pub s: cvmx_rnm_eer_key_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_rnm_serial_num_s { pub dat: u64 }
#[repr(C)]
pub union cvmx_rnm_serial_num { pub u64_: u64, pub s: cvmx_rnm_serial_num_s }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
