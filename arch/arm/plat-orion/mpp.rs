/*
 * arch/arm/plat-orion/mpp.c
 *
 * MPP functions for Marvell orion SoCs
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

use core::ffi::c_void;

// Symbols and MPP_NUM/MPP_SEL constants/macros are supplied by the platform
// headers and other translation units.
extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn printk(fmt: *const i8, ...);
    fn orion_gpio_set_valid(num: u32, gpio_mode: i32);
    fn MPP_NUM(value: u32) -> u32;
    fn MPP_SEL(value: u32) -> u32;
}

// Address of the ith MPP control register
#[inline]
unsafe fn mpp_ctrl_addr(i: u32, dev_bus: *mut c_void) -> *mut c_void {
    (dev_bus as *mut u8).add((i * 4) as usize) as *mut c_void
}

pub unsafe fn orion_mpp_conf(
    mut mpp_list: *mut u32,
    variant_mask: u32,
    mpp_max: u32,
    dev_bus: *mut c_void,
) {
    let mpp_nr_regs = 1 + mpp_max / 8;
    let mut mpp_ctrl = [0u32; 8];
    let mut i: i32;

    printk(b"initial MPP regs:\0".as_ptr() as *const i8);
    if mpp_nr_regs > mpp_ctrl.len() as u32 {
        printk(b"orion_mpp_conf: invalid mpp_max\n\0".as_ptr() as *const i8);
        return;
    }

    i = 0;
    while i < mpp_nr_regs as i32 {
        mpp_ctrl[i as usize] = readl(mpp_ctrl_addr(i as u32, dev_bus));
        printk(b" %08x\0".as_ptr() as *const i8, mpp_ctrl[i as usize]);
        i += 1;
    }
    printk(b"\n\0".as_ptr() as *const i8);

    while *mpp_list != 0 {
        let num = MPP_NUM(*mpp_list);
        let sel = MPP_SEL(*mpp_list);
        let mut shift: i32;
        let mut gpio_mode: i32;

        if num > mpp_max {
            printk(
                b"orion_mpp_conf: invalid MPP number (%u)\n\0".as_ptr() as *const i8,
                num,
            );
            mpp_list = mpp_list.add(1);
            continue;
        }
        if variant_mask != 0 && (*mpp_list & variant_mask) == 0 {
            printk(
                b"orion_mpp_conf: requested MPP%u config unavailable on this hardware\n\0"
                    .as_ptr() as *const i8,
                num,
            );
            mpp_list = mpp_list.add(1);
            continue;
        }

        shift = ((num & 7) << 2) as i32;
        mpp_ctrl[(num / 8) as usize] &= !(0xfu32 << shift);
        mpp_ctrl[(num / 8) as usize] |= sel << shift;

        gpio_mode = 0;
        if (*mpp_list & MPP_INPUT_MASK) != 0 {
            gpio_mode |= GPIO_INPUT_OK;
        }
        if (*mpp_list & MPP_OUTPUT_MASK) != 0 {
            gpio_mode |= GPIO_OUTPUT_OK;
        }

        orion_gpio_set_valid(num, gpio_mode);
        mpp_list = mpp_list.add(1);
    }

    printk(b"  final MPP regs:\0".as_ptr() as *const i8);
    i = 0;
    while i < mpp_nr_regs as i32 {
        writel(mpp_ctrl[i as usize], mpp_ctrl_addr(i as u32, dev_bus));
        printk(b" %08x\0".as_ptr() as *const i8, mpp_ctrl[i as usize]);
        i += 1;
    }
    printk(b"\n\0".as_ptr() as *const i8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
