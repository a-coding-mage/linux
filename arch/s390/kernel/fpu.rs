// SPDX-License-Identifier: GPL-2.0
/*
 * In-kernel vector facility support functions
 *
 * Copyright IBM Corp. 2015
 * Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

// Kernel and architecture dependencies are supplied by the surrounding build.

extern "C" {
    fn cpu_has_vx() -> bool;
    fn fpu_stfpc(fpc: *mut u32);
    fn fpu_lfpc(fpc: *const u32);
    fn fpu_lfpc_safe(fpc: *const u32);
    fn save_fp_regs_vx(vxrs: *mut __vector128);
    fn load_fp_regs_vx(vxrs: *mut __vector128);
    fn fpu_vstm(first: i32, last: i32, vxrs: *mut __vector128) -> *mut __vector128;
    fn fpu_vlm(first: i32, last: i32, vxrs: *mut __vector128) -> *mut __vector128;
}

pub unsafe fn __kernel_fpu_begin(state: *mut kernel_fpu, mut flags: i32) {
    let mut vxrs = (*state).vxrs;
    let mut mask: i32;

    /*
     * Limit the save to the FPU/vector registers already
     * in use by the previous context.
     */
    flags &= (*state).hdr.mask;
    if flags & KERNEL_FPC != 0 {
        fpu_stfpc(&mut (*state).hdr.fpc);
    }
    if !cpu_has_vx() {
        if flags & KERNEL_VXR_LOW != 0 {
            save_fp_regs_vx(vxrs);
        }
        return;
    }
    mask = flags & KERNEL_VXR;
    if mask == KERNEL_VXR {
        vxrs = fpu_vstm(0, 15, vxrs);
        vxrs = fpu_vstm(16, 31, vxrs);
        return;
    }
    if mask == KERNEL_VXR_MID {
        vxrs = fpu_vstm(8, 23, vxrs);
        return;
    }
    mask = flags & KERNEL_VXR_LOW;
    if mask != 0 {
        if mask == KERNEL_VXR_LOW {
            vxrs = fpu_vstm(0, 15, vxrs);
        } else if mask == KERNEL_VXR_V0V7 {
            vxrs = fpu_vstm(0, 7, vxrs);
        } else {
            vxrs = fpu_vstm(8, 15, vxrs);
        }
    }
    mask = flags & KERNEL_VXR_HIGH;
    if mask != 0 {
        if mask == KERNEL_VXR_HIGH {
            vxrs = fpu_vstm(16, 31, vxrs);
        } else if mask == KERNEL_VXR_V16V23 {
            vxrs = fpu_vstm(16, 23, vxrs);
        } else {
            vxrs = fpu_vstm(24, 31, vxrs);
        }
    }
}

pub unsafe fn __kernel_fpu_end(state: *mut kernel_fpu, mut flags: i32) {
    let mut vxrs = (*state).vxrs;
    let mut mask: i32;

    /*
     * Limit the restore to the FPU/vector registers of the
     * previous context that have been overwritten by the
     * current context.
     */
    flags &= (*state).hdr.mask;
    if flags & KERNEL_FPC != 0 {
        fpu_lfpc(&(*state).hdr.fpc);
    }
    if !cpu_has_vx() {
        if flags & KERNEL_VXR_LOW != 0 {
            load_fp_regs_vx(vxrs);
        }
        return;
    }
    mask = flags & KERNEL_VXR;
    if mask == KERNEL_VXR {
        vxrs = fpu_vlm(0, 15, vxrs);
        vxrs = fpu_vlm(16, 31, vxrs);
        return;
    }
    if mask == KERNEL_VXR_MID {
        vxrs = fpu_vlm(8, 23, vxrs);
        return;
    }
    mask = flags & KERNEL_VXR_LOW;
    if mask != 0 {
        if mask == KERNEL_VXR_LOW {
            vxrs = fpu_vlm(0, 15, vxrs);
        } else if mask == KERNEL_VXR_V0V7 {
            vxrs = fpu_vlm(0, 7, vxrs);
        } else {
            vxrs = fpu_vlm(8, 15, vxrs);
        }
    }
    mask = flags & KERNEL_VXR_HIGH;
    if mask != 0 {
        if mask == KERNEL_VXR_HIGH {
            vxrs = fpu_vlm(16, 31, vxrs);
        } else if mask == KERNEL_VXR_V16V23 {
            vxrs = fpu_vlm(16, 23, vxrs);
        } else {
            vxrs = fpu_vlm(24, 31, vxrs);
        }
    }
}

pub unsafe fn load_fpu_state(state: *mut fpu, flags: i32) {
    let vxrs = (*state).vxrs.as_mut_ptr();
    let mut mask: i32;

    if flags & KERNEL_FPC != 0 {
        fpu_lfpc_safe(&(*state).fpc);
    }
    if !cpu_has_vx() {
        if flags & KERNEL_VXR_V0V7 != 0 {
            load_fp_regs_vx((*state).vxrs.as_mut_ptr());
        }
        return;
    }
    mask = flags & KERNEL_VXR;
    if mask == KERNEL_VXR {
        fpu_vlm(0, 15, vxrs);
        fpu_vlm(16, 31, vxrs.add(16));
        return;
    }
    if mask == KERNEL_VXR_MID {
        fpu_vlm(8, 23, vxrs.add(8));
        return;
    }
    mask = flags & KERNEL_VXR_LOW;
    if mask != 0 {
        if mask == KERNEL_VXR_LOW {
            fpu_vlm(0, 15, vxrs);
        } else if mask == KERNEL_VXR_V0V7 {
            fpu_vlm(0, 7, vxrs);
        } else {
            fpu_vlm(8, 15, vxrs.add(8));
        }
    }
    mask = flags & KERNEL_VXR_HIGH;
    if mask != 0 {
        if mask == KERNEL_VXR_HIGH {
            fpu_vlm(16, 31, vxrs.add(16));
        } else if mask == KERNEL_VXR_V16V23 {
            fpu_vlm(16, 23, vxrs.add(16));
        } else {
            fpu_vlm(24, 31, vxrs.add(24));
        }
    }
}

pub unsafe fn save_fpu_state(state: *mut fpu, flags: i32) {
    let vxrs = (*state).vxrs.as_mut_ptr();
    let mut mask: i32;

    if flags & KERNEL_FPC != 0 {
        fpu_stfpc(&mut (*state).fpc);
    }
    if !cpu_has_vx() {
        if flags & KERNEL_VXR_LOW != 0 {
            save_fp_regs_vx((*state).vxrs.as_mut_ptr());
        }
        return;
    }
    mask = flags & KERNEL_VXR;
    if mask == KERNEL_VXR {
        fpu_vstm(0, 15, vxrs);
        fpu_vstm(16, 31, vxrs.add(16));
        return;
    }
    if mask == KERNEL_VXR_MID {
        fpu_vstm(8, 23, vxrs.add(8));
        return;
    }
    mask = flags & KERNEL_VXR_LOW;
    if mask != 0 {
        if mask == KERNEL_VXR_LOW {
            fpu_vstm(0, 15, vxrs);
        } else if mask == KERNEL_VXR_V0V7 {
            fpu_vstm(0, 7, vxrs);
        } else {
            fpu_vstm(8, 15, vxrs.add(8));
        }
    }
    mask = flags & KERNEL_VXR_HIGH;
    if mask != 0 {
        if mask == KERNEL_VXR_HIGH {
            fpu_vstm(16, 31, vxrs.add(16));
        } else if mask == KERNEL_VXR_V16V23 {
            fpu_vstm(16, 23, vxrs.add(16));
        } else {
            fpu_vstm(24, 31, vxrs.add(24));
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
