// SPDX-License-Identifier: GPL-2.0-only
/*
 * Hardware parameter area specific to Sharp SL series devices
 *
 * Copyright (c) 2005 Richard Purdie
 *
 * Based on Sharp's 2.4 kernel patches
 */

// Dependency equivalents supplied by the kernel headers:
// use the externally provided `SharpslParamInfo` type.

#[cfg(target_arch = "sa1100")]
const PARAM_BASE: usize = 0xe8ff_c000;
#[cfg(not(target_arch = "sa1100"))]
const PARAM_BASE: usize = 0xa000_0a00;

const fn magic_chg(a: u32, b: u32, c: u32, d: u32) -> u32 {
    (d << 24) | (c << 16) | (b << 8) | a
}

const COMADJ_MAGIC: u32 = magic_chg(b'C' as u32, b'M' as u32, b'A' as u32, b'D' as u32);
const UUID_MAGIC: u32 = magic_chg(b'U' as u32, b'U' as u32, b'I' as u32, b'D' as u32);
const TOUCH_MAGIC: u32 = magic_chg(b'T' as u32, b'U' as u32, b'C' as u32, b'H' as u32);
const AD_MAGIC: u32 = magic_chg(b'B' as u32, b'V' as u32, b'A' as u32, b'D' as u32);
const PHAD_MAGIC: u32 = magic_chg(b'P' as u32, b'H' as u32, b'A' as u32, b'D' as u32);

pub static mut sharpsl_param: SharpslParamInfo = unsafe { core::mem::zeroed() };

pub unsafe fn sharpsl_save_param() {
    // On SA1100 this is a direct address; otherwise PARAM_BASE is converted
    // from a physical address by the externally supplied kernel mapping.
    let params = {
        #[cfg(target_arch = "sa1100")]
        {
            PARAM_BASE as *const SharpslParamInfo
        }
        #[cfg(not(target_arch = "sa1100"))]
        {
            __va(PARAM_BASE) as *const SharpslParamInfo
        }
    };

    core::ptr::copy_nonoverlapping(
        params,
        core::ptr::addr_of_mut!(sharpsl_param),
        1,
    );

    if sharpsl_param.comadj_keyword != COMADJ_MAGIC {
        sharpsl_param.comadj = -1;
    }

    if sharpsl_param.phad_keyword != PHAD_MAGIC {
        sharpsl_param.phadadj = -1;
    }

    if sharpsl_param.uuid_keyword != UUID_MAGIC {
        sharpsl_param.uuid[0] = -1;
    }

    if sharpsl_param.touch_keyword != TOUCH_MAGIC {
        sharpsl_param.touch_xp = -1;
    }

    if sharpsl_param.adadj_keyword != AD_MAGIC {
        sharpsl_param.adadj = -1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
