/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Compact binary representation of ihex records. Some devices need their
 * firmware loaded in strange orders rather than a single big blob, but
 * actually parsing ihex-as-text within the kernel seems silly. Thus,...
 */

/* C header dependencies: <linux/types.h>, <linux/firmware.h>, and
 * <linux/device.h> are supplied by the surrounding kernel translation. */

#[repr(C, packed)]
pub struct ihex_binrec {
    pub addr: __be32,
    pub len: __be16,
    pub data: [u8; 0],
}

pub type __be32 = u32;
pub type __be16 = u16;

extern "C" {
    fn be16_to_cpu(value: __be16) -> u16;
    fn request_firmware(
        fw: *mut *const firmware,
        fw_name: *const core::ffi::c_char,
        dev: *mut device,
    ) -> i32;
    fn release_firmware(fw: *const firmware);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct firmware {
    pub data: *const u8,
    pub size: usize,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn ihex_binrec_size(p: *const ihex_binrec) -> u16 {
    be16_to_cpu(core::ptr::read_unaligned(core::ptr::addr_of!((*p).len)))
        .wrapping_add(core::mem::size_of::<ihex_binrec>() as u16)
}

#[inline]
pub unsafe fn __ihex_next_binrec(rec: *const ihex_binrec) -> *const ihex_binrec {
    let size = ihex_binrec_size(rec) as usize;
    let aligned = (size.wrapping_add(3)) & !3;
    (rec as *const u8).add(aligned) as *const ihex_binrec
}

#[inline]
pub unsafe fn ihex_next_binrec(rec: *const ihex_binrec) -> *const ihex_binrec {
    let rec = __ihex_next_binrec(rec);
    if be16_to_cpu(core::ptr::read_unaligned(core::ptr::addr_of!((*rec).len))) != 0 {
        rec
    } else {
        core::ptr::null()
    }
}

/* Check that ihex_next_binrec() won't take us off the end of the image... */
#[inline]
pub unsafe fn ihex_validate_fw(fw: *const firmware) -> i32 {
    let rec = (*fw).data as *const ihex_binrec;
    let end = (*fw).data.add((*fw).size.wrapping_sub(core::mem::size_of::<ihex_binrec>()))
        as *const ihex_binrec;

    let mut rec = rec;
    while (rec as usize) <= (end as usize) {
        /* Zero length marks end of records */
        if rec == end
            && be16_to_cpu(core::ptr::read_unaligned(core::ptr::addr_of!((*rec).len))) == 0
        {
            return 0;
        }
        rec = __ihex_next_binrec(rec);
    }
    -22
}

/* Request firmware and validate it so that we can trust we won't
 * run off the end while reading records... */
#[inline]
pub unsafe fn request_ihex_firmware(
    fw: *mut *const firmware,
    fw_name: *const core::ffi::c_char,
    dev: *mut device,
) -> i32 {
    let mut lfw: *const firmware = core::ptr::null();
    let mut ret = request_firmware(&mut lfw, fw_name, dev);
    if ret != 0 {
        return ret;
    }
    ret = ihex_validate_fw(lfw);
    if ret != 0 {
        dev_err(dev, b"Firmware \"%s\" not valid IHEX records\n\0".as_ptr() as *const _, fw_name);
        release_firmware(lfw);
        return ret;
    }
    *fw = lfw;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
