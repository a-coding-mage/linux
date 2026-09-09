// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-dove/mpp.c
 *
 * MPP functions for Marvell Dove SoCs
 */

// Dependencies supplied by the Linux kernel and platform headers are external.

#[repr(C)]
struct dove_mpp_grp {
    start: i32,
    end: i32,
}

/* Map a group to a range of GPIO pins in that group */
static DOVE_MPP_GRP: [dove_mpp_grp; 5] = [
    dove_mpp_grp { start: 24, end: 39 },
    dove_mpp_grp { start: 40, end: 45 },
    dove_mpp_grp { start: 46, end: 51 },
    dove_mpp_grp { start: 58, end: 61 },
    dove_mpp_grp { start: 62, end: 63 },
];

/* Enable gpio for a range of pins. mode should be a combination of
   GPIO_OUTPUT_OK | GPIO_INPUT_OK */
unsafe fn dove_mpp_gpio_mode(start: i32, end: i32, gpio_mode: i32) {
    let mut i = start;
    while i <= end {
        orion_gpio_set_valid(i, gpio_mode);
        i += 1;
    }
}

/* Dump all the extra MPP registers. The platform code will dump the
   registers for pins 0-23. */
unsafe fn dove_mpp_dump_regs() {
    pr_debug("PMU_CTRL4_CTRL: %08x\n", readl(DOVE_MPP_CTRL4_VIRT_BASE));
    pr_debug(
        "PMU_MPP_GENERAL_CTRL: %08x\n",
        readl(DOVE_PMU_MPP_GENERAL_CTRL),
    );
    pr_debug("MPP_GENERAL: %08x\n", readl(DOVE_MPP_GENERAL_VIRT_BASE));
}

unsafe fn dove_mpp_cfg_nfc(sel: i32) {
    let mut mpp_gen_cfg: u32 = readl(DOVE_MPP_GENERAL_VIRT_BASE);
    mpp_gen_cfg &= !0x1;
    mpp_gen_cfg |= sel as u32;
    writel(mpp_gen_cfg, DOVE_MPP_GENERAL_VIRT_BASE);
    dove_mpp_gpio_mode(64, 71, GPIO_OUTPUT_OK);
}

unsafe fn dove_mpp_cfg_au1(sel: i32) {
    let mut mpp_ctrl4: u32 = readl(DOVE_MPP_CTRL4_VIRT_BASE);
    let mut ssp_ctrl1: u32 = readl(DOVE_SSP_CTRL_STATUS_1);
    let mut mpp_gen_ctrl: u32 = readl(DOVE_MPP_GENERAL_VIRT_BASE);
    let mut global_cfg_2: u32 = readl(DOVE_GLOBAL_CONFIG_2);

    mpp_ctrl4 &= !(DOVE_AU1_GPIO_SEL as u32);
    ssp_ctrl1 &= !(DOVE_SSP_ON_AU1 as u32);
    mpp_gen_ctrl &= !(DOVE_AU1_SPDIFO_GPIO_EN as u32);
    global_cfg_2 &= !(DOVE_TWSI_OPTION3_GPIO as u32);

    if sel == 0 || sel == 0x2 {
        dove_mpp_gpio_mode(52, 57, 0);
    } else {
        dove_mpp_gpio_mode(52, 57, GPIO_OUTPUT_OK | GPIO_INPUT_OK);
    }
    if sel & 0x1 != 0 {
        global_cfg_2 |= DOVE_TWSI_OPTION3_GPIO as u32;
        dove_mpp_gpio_mode(56, 57, 0);
    }
    if sel & 0x2 != 0 {
        mpp_gen_ctrl |= DOVE_AU1_SPDIFO_GPIO_EN as u32;
        dove_mpp_gpio_mode(57, 57, GPIO_OUTPUT_OK | GPIO_INPUT_OK);
    }
    if sel & 0x4 != 0 {
        ssp_ctrl1 |= DOVE_SSP_ON_AU1 as u32;
        dove_mpp_gpio_mode(52, 55, 0);
    }
    if sel & 0x8 != 0 {
        mpp_ctrl4 |= DOVE_AU1_GPIO_SEL as u32;
    }

    writel(mpp_ctrl4, DOVE_MPP_CTRL4_VIRT_BASE);
    writel(ssp_ctrl1, DOVE_SSP_CTRL_STATUS_1);
    writel(mpp_gen_ctrl, DOVE_MPP_GENERAL_VIRT_BASE);
    writel(global_cfg_2, DOVE_GLOBAL_CONFIG_2);
}

/* Configure the group registers, enabling GPIO if sel indicates the
   pin is to be used for GPIO */
unsafe fn dove_mpp_conf_grp(mut mpp_grp_list: *mut u32) {
    let mut mpp_ctrl4: u32 = readl(DOVE_MPP_CTRL4_VIRT_BASE);

    while *mpp_grp_list != 0 {
        let num: usize = MPP_NUM(*mpp_grp_list) as usize;
        let sel: u32 = MPP_SEL(*mpp_grp_list);
        if num > MPP_GRP_MAX as usize {
            pr_err("dove: invalid MPP GRP number (%u)\n", num as u32);
            mpp_grp_list = mpp_grp_list.add(1);
            continue;
        }
        mpp_ctrl4 &= !(0x1u32 << num);
        mpp_ctrl4 |= sel << num;
        let gpio_mode = if sel != 0 {
            GPIO_OUTPUT_OK | GPIO_INPUT_OK
        } else {
            0
        };
        dove_mpp_gpio_mode(DOVE_MPP_GRP[num].start, DOVE_MPP_GRP[num].end, gpio_mode);
        mpp_grp_list = mpp_grp_list.add(1);
    }
    writel(mpp_ctrl4, DOVE_MPP_CTRL4_VIRT_BASE);
}

/* Configure the various MPP pins on Dove */
pub unsafe fn dove_mpp_conf(
    mpp_list: *mut u32,
    mpp_grp_list: *mut u32,
    grp_au1_52_57: u32,
    grp_nfc_64_71: u32,
) {
    dove_mpp_dump_regs();

    /* Use platform code for pins 0-23 */
    orion_mpp_conf(mpp_list, 0, MPP_MAX, DOVE_MPP_VIRT_BASE);

    dove_mpp_conf_grp(mpp_grp_list);
    dove_mpp_cfg_au1(grp_au1_52_57 as i32);
    dove_mpp_cfg_nfc(grp_nfc_64_71 as i32);

    dove_mpp_dump_regs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
