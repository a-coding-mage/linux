// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright Altera Corporation (C) 2016. All rights reserved.
 */

// Dependency declarations supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    static mut sys_manager_base_addr: *mut u8;

    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn of_node_put(node: *mut device_node);
    fn iounmap(addr: *mut u8);
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn udelay(usecs: u32);
    fn wmb();
    fn pr_err(fmt: *const c_char, ...);
}

const ALTR_OCRAM_CLEAR_ECC: u32 = 0x00000018;
const ALTR_OCRAM_ECC_EN: u32 = 0x00000019;

pub unsafe fn socfpga_init_ocram_ecc() {
    let np: *mut device_node;
    let mapped_ocr_edac_addr: *mut u8;

    /* Find the OCRAM EDAC device tree node */
    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"altr,socfpga-ocram-ecc\0".as_ptr() as *const c_char,
    );
    if np.is_null() {
        pr_err(b"Unable to find socfpga-ocram-ecc\n\0".as_ptr() as *const c_char);
        return;
    }

    mapped_ocr_edac_addr = of_iomap(np, 0);
    of_node_put(np);
    if mapped_ocr_edac_addr.is_null() {
        pr_err(b"Unable to map OCRAM ecc regs.\n\0".as_ptr() as *const c_char);
        return;
    }

    /* Clear any pending OCRAM ECC interrupts, then enable ECC */
    writel(ALTR_OCRAM_CLEAR_ECC, mapped_ocr_edac_addr);
    writel(ALTR_OCRAM_ECC_EN, mapped_ocr_edac_addr);

    iounmap(mapped_ocr_edac_addr);
}

/* Arria10 OCRAM Section */
const ALTR_A10_ECC_CTRL_OFST: usize = 0x08;
const ALTR_A10_OCRAM_ECC_EN_CTL: u32 = (1 << 1) | (1 << 0);
const ALTR_A10_ECC_INITA: u32 = 1 << 16;
const ALTR_A10_ECC_INITSTAT_OFST: usize = 0x0C;
const ALTR_A10_ECC_INITCOMPLETEA: u32 = 1 << 0;
const ALTR_A10_ECC_INITCOMPLETEB: u32 = 1 << 8;
const ALTR_A10_ECC_ERRINTEN_OFST: usize = 0x10;
const ALTR_A10_ECC_SERRINTEN: u32 = 1 << 0;
const ALTR_A10_ECC_INTSTAT_OFST: usize = 0x20;
const ALTR_A10_ECC_SERRPENA: u32 = 1 << 0;
const ALTR_A10_ECC_DERRPENA: u32 = 1 << 8;
const ALTR_A10_ECC_ERRPENA_MASK: u32 = ALTR_A10_ECC_SERRPENA | ALTR_A10_ECC_DERRPENA;
/* ECC Manager Defines */
const A10_SYSMGR_ECC_INTMASK_SET_OFST: usize = 0x94;
const A10_SYSMGR_ECC_INTMASK_CLR_OFST: usize = 0x98;
const A10_SYSMGR_ECC_INTMASK_OCRAM: u32 = 1 << 1;
const ALTR_A10_ECC_INIT_WATCHDOG_10US: i32 = 10000;

unsafe fn ecc_set_bits(bit_mask: u32, ioaddr: *mut u8) {
    let value = readl(ioaddr);
    writel(value | bit_mask, ioaddr);
}

unsafe fn ecc_clear_bits(bit_mask: u32, ioaddr: *mut u8) {
    let value = readl(ioaddr);
    writel(value & !bit_mask, ioaddr);
}

unsafe fn ecc_test_bits(bit_mask: u32, ioaddr: *mut u8) -> i32 {
    let value = readl(ioaddr);
    if value & bit_mask != 0 { 1 } else { 0 }
}

/*
 * This function uses the memory initialization block in the Arria10 ECC
 * controller to initialize/clear the entire memory data and ECC data.
 */
unsafe fn altr_init_memory_port(ioaddr: *mut u8) -> i32 {
    let mut limit = ALTR_A10_ECC_INIT_WATCHDOG_10US;

    ecc_set_bits(ALTR_A10_ECC_INITA, ioaddr.add(ALTR_A10_ECC_CTRL_OFST));
    while limit > 0 {
        limit -= 1;
        if ecc_test_bits(
            ALTR_A10_ECC_INITCOMPLETEA,
            ioaddr.add(ALTR_A10_ECC_INITSTAT_OFST),
        ) != 0 {
            break;
        }
        udelay(1);
    }
    if limit < 0 {
        return -16; // -EBUSY
    }

    /* Clear any pending ECC interrupts */
    writel(ALTR_A10_ECC_ERRPENA_MASK, ioaddr.add(ALTR_A10_ECC_INTSTAT_OFST));
    0
}

pub unsafe fn socfpga_init_arria10_ocram_ecc() {
    let mut ret = 0;
    let np: *mut device_node;
    let ecc_block_base: *mut u8;

    if sys_manager_base_addr.is_null() {
        pr_err(b"SOCFPGA: sys-mgr is not initialized\n\0".as_ptr() as *const c_char);
        return;
    }

    /* Find the OCRAM EDAC device tree node */
    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"altr,socfpga-a10-ocram-ecc\0".as_ptr() as *const c_char,
    );
    if np.is_null() {
        pr_err(b"Unable to find socfpga-a10-ocram-ecc\n\0".as_ptr() as *const c_char);
        return;
    }

    /* Map the ECC Block */
    ecc_block_base = of_iomap(np, 0);
    of_node_put(np);
    if ecc_block_base.is_null() {
        pr_err(b"Unable to map OCRAM ECC block\n\0".as_ptr() as *const c_char);
        return;
    }

    /* Disable ECC */
    writel(ALTR_A10_OCRAM_ECC_EN_CTL, sys_manager_base_addr.add(A10_SYSMGR_ECC_INTMASK_SET_OFST));
    ecc_clear_bits(ALTR_A10_ECC_SERRINTEN, ecc_block_base.add(ALTR_A10_ECC_ERRINTEN_OFST));
    ecc_clear_bits(ALTR_A10_OCRAM_ECC_EN_CTL, ecc_block_base.add(ALTR_A10_ECC_CTRL_OFST));

    /* Ensure all writes complete */
    wmb();

    /* Use HW initialization block to initialize memory for ECC */
    ret = altr_init_memory_port(ecc_block_base);
    if ret != 0 {
        pr_err(b"ECC: cannot init OCRAM PORTA memory\n\0".as_ptr() as *const c_char);
    } else {
        /* Enable ECC */
        ecc_set_bits(ALTR_A10_OCRAM_ECC_EN_CTL, ecc_block_base.add(ALTR_A10_ECC_CTRL_OFST));
        ecc_set_bits(ALTR_A10_ECC_SERRINTEN, ecc_block_base.add(ALTR_A10_ECC_ERRINTEN_OFST));
        writel(ALTR_A10_OCRAM_ECC_EN_CTL, sys_manager_base_addr.add(A10_SYSMGR_ECC_INTMASK_CLR_OFST));

        /* Ensure all writes complete */
        wmb();
    }

    iounmap(ecc_block_base);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
