// SPDX-License-Identifier: GPL-2.0-only
/*
 * imr.c -- Intel Isolated Memory Region driver
 *
 * Copyright(c) 2013 Intel Corporation.
 * Copyright(c) 2015 Bryan O'Donoghue <pure.logic@nexus-software.ie>
 *
 * IMR registers define an isolated region of memory that can
 * be masked to prohibit certain system agents from accessing memory.
 * When a device behind a masked port performs an access - snooped or
 * not, an IMR may optionally prevent that transaction from changing the
 * state of memory or from getting correct data in response to the
 * operation.
 *
 * Write data will be dropped and reads will return 0xFFFFFFFF, the
 * system will reset and system BIOS will print out an error message to
 * inform the user that an IMR has been violated.
 *
 * This code is based on the Linux MTRR code and reference code from
 * Intel's Quark BSP EFI, Linux and grub code.
 *
 * See quark-x1000-datasheet.pdf for register definitions.
 */

// External kernel types, constants, functions, and macros are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct imr_device {
    pub init: bool,
    pub lock: mutex,
    pub max_imr: i32,
    pub reg_base: i32,
}

static mut imr_dev: imr_device = imr_device {
    init: false,
    lock: mutex {},
    max_imr: 0,
    reg_base: 0,
};

const IMR_LOCK: u32 = BIT(31);

#[repr(C)]
pub struct imr_regs {
    pub addr_lo: u32,
    pub addr_hi: u32,
    pub rmask: u32,
    pub wmask: u32,
}

const IMR_NUM_REGS: usize = core::mem::size_of::<imr_regs>() / core::mem::size_of::<u32>();
const IMR_SHIFT: u32 = 8;

#[inline]
fn imr_to_phys(x: u32) -> u32 { x << IMR_SHIFT }

#[inline]
fn phys_to_imr(x: u32) -> u32 { x >> IMR_SHIFT }

#[inline]
unsafe fn imr_is_enabled(imr: *mut imr_regs) -> i32 {
    !((*imr).rmask == IMR_READ_ACCESS_ALL &&
      (*imr).wmask == IMR_WRITE_ACCESS_ALL &&
      imr_to_phys((*imr).addr_lo) == 0 &&
      imr_to_phys((*imr).addr_hi) == 0) as i32
}

unsafe fn imr_read(idev: *mut imr_device, imr_id: u32, imr: *mut imr_regs) -> i32 {
    let mut reg = imr_id * IMR_NUM_REGS as u32 + (*idev).reg_base as u32;
    let mut ret = iosf_mbi_read(QRK_MBI_UNIT_MM, MBI_REG_READ, reg, &mut (*imr).addr_lo);
    if ret != 0 { return ret; }
    reg += 1;
    ret = iosf_mbi_read(QRK_MBI_UNIT_MM, MBI_REG_READ, reg, &mut (*imr).addr_hi);
    if ret != 0 { return ret; }
    reg += 1;
    ret = iosf_mbi_read(QRK_MBI_UNIT_MM, MBI_REG_READ, reg, &mut (*imr).rmask);
    if ret != 0 { return ret; }
    reg += 1;
    iosf_mbi_read(QRK_MBI_UNIT_MM, MBI_REG_READ, reg, &mut (*imr).wmask)
}

unsafe fn imr_write(idev: *mut imr_device, imr_id: u32, imr: *mut imr_regs) -> i32 {
    let mut flags: ulong = 0;
    let mut reg = imr_id * IMR_NUM_REGS as u32 + (*idev).reg_base as u32;
    let mut ret;
    local_irq_save(&mut flags);
    ret = iosf_mbi_write(QRK_MBI_UNIT_MM, MBI_REG_WRITE, reg, (*imr).addr_lo);
    if ret != 0 { local_irq_restore(flags); return ret; }
    reg += 1;
    ret = iosf_mbi_write(QRK_MBI_UNIT_MM, MBI_REG_WRITE, reg, (*imr).addr_hi);
    if ret != 0 { local_irq_restore(flags); return ret; }
    reg += 1;
    ret = iosf_mbi_write(QRK_MBI_UNIT_MM, MBI_REG_WRITE, reg, (*imr).rmask);
    if ret != 0 { local_irq_restore(flags); return ret; }
    reg += 1;
    ret = iosf_mbi_write(QRK_MBI_UNIT_MM, MBI_REG_WRITE, reg, (*imr).wmask);
    if ret != 0 { local_irq_restore(flags); return ret; }
    local_irq_restore(flags);
    0
}

#[inline]
fn imr_check_params(base: phys_addr_t, size: usize) -> i32 {
    if (base & IMR_MASK) != 0 || (size & IMR_MASK as usize) != 0 { return -EINVAL; }
    if size == 0 { return -EINVAL; }
    0
}

#[inline]
fn imr_raw_size(size: usize) -> usize { size - IMR_ALIGN }

#[inline]
unsafe fn imr_address_overlap(addr: phys_addr_t, imr: *mut imr_regs) -> i32 {
    (addr >= imr_to_phys((*imr).addr_lo) as phys_addr_t && addr <= imr_to_phys((*imr).addr_hi) as phys_addr_t) as i32
}

pub unsafe fn imr_add_range(base: phys_addr_t, size: usize, rmask: u32, wmask: u32) -> i32 {
    let idev = &mut imr_dev as *mut imr_device;
    if !(*idev).init { return -ENODEV; }
    let ret = imr_check_params(base, size);
    if ret != 0 { return ret; }
    let raw_size = imr_raw_size(size);
    let end = base + raw_size as phys_addr_t;
    let mut imr = imr_regs { addr_lo: phys_to_imr(base as u32), addr_hi: phys_to_imr(end as u32), rmask, wmask };
    if imr_is_enabled(&mut imr) == 0 { return -ENOTSUPP; }
    mutex_lock(&mut (*idev).lock);
    let mut reg: i32 = -1;
    for i in 0..(*idev).max_imr as u32 {
        let ret = imr_read(idev, i, &mut imr);
        if ret != 0 { mutex_unlock(&mut (*idev).lock); return ret; }
        if imr_is_enabled(&mut imr) != 0 {
            if imr_address_overlap(base, &mut imr) != 0 || imr_address_overlap(end, &mut imr) != 0 { mutex_unlock(&mut (*idev).lock); return -EINVAL; }
        } else { reg = i as i32; }
    }
    if reg == -1 { mutex_unlock(&mut (*idev).lock); return -ENOMEM; }
    imr.addr_lo = phys_to_imr(base as u32); imr.addr_hi = phys_to_imr(end as u32); imr.rmask = rmask; imr.wmask = wmask;
    let ret = imr_write(idev, reg as u32, &mut imr);
    if ret < 0 {
        imr.addr_lo = 0; imr.addr_hi = 0; imr.rmask = IMR_READ_ACCESS_ALL; imr.wmask = IMR_WRITE_ACCESS_ALL;
        imr_write(idev, reg as u32, &mut imr);
    }
    mutex_unlock(&mut (*idev).lock);
    ret
}

unsafe fn __imr_remove_range(reg: i32, base: phys_addr_t, size: usize) -> i32 {
    let idev = &mut imr_dev as *mut imr_device;
    if !(*idev).init { return -ENODEV; }
    if reg == -1 { let ret = imr_check_params(base, size); if ret != 0 { return ret; } }
    let end = base + imr_raw_size(size) as phys_addr_t;
    mutex_lock(&mut (*idev).lock);
    let mut imr = imr_regs { addr_lo: 0, addr_hi: 0, rmask: 0, wmask: 0 };
    let mut found = false;
    let mut selected = reg;
    if reg >= 0 {
        let ret = imr_read(idev, reg as u32, &mut imr); if ret != 0 { mutex_unlock(&mut (*idev).lock); return ret; }
        if imr_is_enabled(&mut imr) == 0 || imr.addr_lo & IMR_LOCK != 0 { mutex_unlock(&mut (*idev).lock); return -ENODEV; }
        found = true;
    } else {
        for i in 0..(*idev).max_imr as u32 {
            let ret = imr_read(idev, i, &mut imr); if ret != 0 { mutex_unlock(&mut (*idev).lock); return ret; }
            if imr_is_enabled(&mut imr) == 0 || imr.addr_lo & IMR_LOCK != 0 { continue; }
            if imr_to_phys(imr.addr_lo) as phys_addr_t == base && imr_to_phys(imr.addr_hi) as phys_addr_t == end { found = true; selected = i as i32; break; }
        }
    }
    if !found { mutex_unlock(&mut (*idev).lock); return -ENODEV; }
    imr.addr_lo = 0; imr.addr_hi = 0; imr.rmask = IMR_READ_ACCESS_ALL; imr.wmask = IMR_WRITE_ACCESS_ALL;
    let ret = imr_write(idev, selected as u32, &mut imr);
    mutex_unlock(&mut (*idev).lock);
    ret
}

pub unsafe fn imr_remove_range(base: phys_addr_t, size: usize) -> i32 { __imr_remove_range(-1, base, size) }

#[inline]
unsafe fn imr_clear(reg: i32) -> i32 { __imr_remove_range(reg, 0, 0) }

unsafe fn imr_fixup_memmap(idev: *mut imr_device) {
    let base = virt_to_phys(&_text);
    let size = virt_to_phys(&__end_rodata) - base;
    for i in 0..(*idev).max_imr { imr_clear(i); }
    imr_add_range(base, size, IMR_CPU, IMR_CPU);
}

static imr_ids: [x86_cpu_id; 2] = [X86_MATCH_VFM(INTEL_QUARK_X1000, core::ptr::null()), x86_cpu_id {}];

unsafe fn imr_init() -> i32 {
    let idev = &mut imr_dev as *mut imr_device;
    if x86_match_cpu(imr_ids.as_ptr()) == 0 || iosf_mbi_available() == 0 { return -ENODEV; }
    (*idev).max_imr = QUARK_X1000_IMR_MAX;
    (*idev).reg_base = QUARK_X1000_IMR_REGBASE;
    (*idev).init = true;
    mutex_init(&mut (*idev).lock);
    imr_fixup_memmap(idev);
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
