/*
 * Phase5 CybervisionPPC (TVP4020) definitions for the Permedia2 framebuffer
 * driver.
 *
 * Copyright (c) 1998-1999 Ilario Nardinocchi (nardinoc@CS.UniBO.IT)
 * --------------------------------------------------------------------------
 * $Id: cvisionppc.h,v 1.8 1999/01/28 13:18:07 illo Exp $
 * --------------------------------------------------------------------------
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// C header guard: CVISIONPPC_H
// Dependency: pm2fb.h (the original header includes it when PM2FB_H is absent).

#[repr(C)]
pub struct cvppc_par {
    pub pci_config: *mut u8,
    pub pci_bridge: *mut u8,
    pub user_flags: u32,
}

pub const CSPPC_PCI_BRIDGE: u32 = 0xfffe0000;
pub const CSPPC_BRIDGE_ENDIAN: u32 = 0x0000;
pub const CSPPC_BRIDGE_INT: u32 = 0x0010;

pub const CVPPC_PCI_CONFIG: u32 = 0xfffc0000;
pub const CVPPC_ROM_ADDRESS: u32 = 0xe2000001;
pub const CVPPC_REGS_REGION: u32 = 0xef000000;
pub const CVPPC_FB_APERTURE_ONE: u32 = 0xe0000000;
pub const CVPPC_FB_APERTURE_TWO: u32 = 0xe1000000;
pub const CVPPC_FB_SIZE: u32 = 0x00800000;
pub const CVPPC_MEM_CONFIG_OLD: u32 = 0xed61fcaa; // FIXME Fujitsu??
pub const CVPPC_MEM_CONFIG_NEW: u32 = 0xed41c532; // FIXME USA??
pub const CVPPC_MEMCLOCK: u32 = 83000; // in KHz

/* CVPPC_BRIDGE_ENDIAN */
pub const CSPPCF_BRIDGE_BIG_ENDIAN: u32 = 0x02;

/* CVPPC_BRIDGE_INT */
pub const CSPPCF_BRIDGE_ACTIVE_INT2: u32 = 0x01;

/*
 *****************************************************************************
 * That's all folks!
 *****************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
