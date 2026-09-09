// SPDX-License-Identifier: GPL-2.0
/*
 * CRC-32 implemented with the z/Architecture Vector Extension Facility.
 *
 * Copyright IBM Corp. 2015
 * Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

// C dependencies: <linux/cpufeature.h>, <asm/fpu.h>, and "crc32-vx.h".

const VX_MIN_LEN: usize = 64;
const VX_ALIGNMENT: usize = 16;
const VX_ALIGN_MASK: usize = VX_ALIGNMENT - 1;

// The following names are supplied by the corresponding kernel dependencies.
extern "C" {
    fn cpu_has_vx() -> bool;
    fn kernel_fpu_begin(vxstate: *mut core::ffi::c_void, mode: i32);
    fn kernel_fpu_end(vxstate: *mut core::ffi::c_void, mode: i32);

    fn crc32_le_vgfm_16(crc: u32, data: *const u8, datalen: usize) -> u32;
    fn crc32_le_base(crc: u32, data: *const u8, datalen: usize) -> u32;
    fn crc32_be_vgfm_16(crc: u32, data: *const u8, datalen: usize) -> u32;
    fn crc32_be_base(crc: u32, data: *const u8, datalen: usize) -> u32;
    fn crc32c_le_vgfm_16(crc: u32, data: *const u8, datalen: usize) -> u32;
    fn crc32c_base(crc: u32, data: *const u8, datalen: usize) -> u32;
}

// KERNEL_VXR_LOW and the CRC32_*_OPTIMIZATION constants are supplied by the
// corresponding kernel dependencies.

#[inline]
pub unsafe fn crc32_le_arch(mut crc: u32, mut data: *const u8, mut datalen: usize) -> u32 {
    let mut vxstate = core::mem::MaybeUninit::<[u8; 16]>::uninit();

    if datalen < VX_MIN_LEN + VX_ALIGN_MASK || !cpu_has_vx() {
        return crc32_le_base(crc, data, datalen);
    }

    if (data as usize) & VX_ALIGN_MASK != 0 {
        let prealign = VX_ALIGNMENT - ((data as usize) & VX_ALIGN_MASK);
        datalen -= prealign;
        crc = crc32_le_base(crc, data, prealign);
        data = (data as usize + prealign) as *const u8;
    }

    let aligned = datalen & !VX_ALIGN_MASK;
    let remaining = datalen & VX_ALIGN_MASK;

    kernel_fpu_begin(vxstate.as_mut_ptr() as *mut core::ffi::c_void, KERNEL_VXR_LOW);
    crc = crc32_le_vgfm_16(crc, data, aligned);
    kernel_fpu_end(vxstate.as_mut_ptr() as *mut core::ffi::c_void, KERNEL_VXR_LOW);

    if remaining != 0 {
        crc = crc32_le_base(crc, data.add(aligned), remaining);
    }
    crc
}

#[inline]
pub unsafe fn crc32_be_arch(mut crc: u32, mut data: *const u8, mut datalen: usize) -> u32 {
    let mut vxstate = core::mem::MaybeUninit::<[u8; 16]>::uninit();
    if datalen < VX_MIN_LEN + VX_ALIGN_MASK || !cpu_has_vx() { return crc32_be_base(crc, data, datalen); }
    if (data as usize) & VX_ALIGN_MASK != 0 {
        let prealign = VX_ALIGNMENT - ((data as usize) & VX_ALIGN_MASK);
        datalen -= prealign; crc = crc32_be_base(crc, data, prealign);
        data = (data as usize + prealign) as *const u8;
    }
    let aligned = datalen & !VX_ALIGN_MASK; let remaining = datalen & VX_ALIGN_MASK;
    kernel_fpu_begin(vxstate.as_mut_ptr() as *mut core::ffi::c_void, KERNEL_VXR_LOW);
    crc = crc32_be_vgfm_16(crc, data, aligned);
    kernel_fpu_end(vxstate.as_mut_ptr() as *mut core::ffi::c_void, KERNEL_VXR_LOW);
    if remaining != 0 { crc = crc32_be_base(crc, data.add(aligned), remaining); }
    crc
}

#[inline]
pub unsafe fn crc32c_arch(mut crc: u32, mut data: *const u8, mut datalen: usize) -> u32 {
    let mut vxstate = core::mem::MaybeUninit::<[u8; 16]>::uninit();
    if datalen < VX_MIN_LEN + VX_ALIGN_MASK || !cpu_has_vx() { return crc32c_base(crc, data, datalen); }
    if (data as usize) & VX_ALIGN_MASK != 0 {
        let prealign = VX_ALIGNMENT - ((data as usize) & VX_ALIGN_MASK);
        datalen -= prealign; crc = crc32c_base(crc, data, prealign);
        data = (data as usize + prealign) as *const u8;
    }
    let aligned = datalen & !VX_ALIGN_MASK; let remaining = datalen & VX_ALIGN_MASK;
    kernel_fpu_begin(vxstate.as_mut_ptr() as *mut core::ffi::c_void, KERNEL_VXR_LOW);
    crc = crc32c_le_vgfm_16(crc, data, aligned);
    kernel_fpu_end(vxstate.as_mut_ptr() as *mut core::ffi::c_void, KERNEL_VXR_LOW);
    if remaining != 0 { crc = crc32c_base(crc, data.add(aligned), remaining); }
    crc
}

#[inline]
pub unsafe fn crc32_optimizations_arch() -> u32 {
    if cpu_has_vx() {
        return CRC32_LE_OPTIMIZATION | CRC32_BE_OPTIMIZATION | CRC32C_OPTIMIZATION;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
