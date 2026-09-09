// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2020 Arm Limited
 */

// pr_fmt(fmt) = "SMCCC: SOC_ID: " fmt
// C dependencies supplied by the surrounding kernel translation.

const SMCCC_SOC_ID_JEP106_BANK_IDX_MASK: u32 = 0x7f00_0000;
/*
 * As per the SMC Calling Convention specification v1.2 (ARM DEN 0028C)
 * Section 7.4 SMCCC_ARCH_SOC_ID bits[23:16] are JEP-106 identification
 * code with parity bit for the SiP. We can drop the parity bit.
 */
const SMCCC_SOC_ID_JEP106_ID_CODE_MASK: u32 = 0x007f_0000;
const SMCCC_SOC_ID_IMP_DEF_SOC_ID_MASK: u32 = 0x0000_ffff;

#[inline]
fn jep106_bank_cont_code(x: u32) -> u8 {
    ((x & SMCCC_SOC_ID_JEP106_BANK_IDX_MASK) >> 24) as u8
}

#[inline]
fn jep106_id_code(x: u32) -> u8 {
    ((x & SMCCC_SOC_ID_JEP106_ID_CODE_MASK) >> 16) as u8
}

#[inline]
fn imp_def_soc_id(x: u32) -> u16 {
    (x & SMCCC_SOC_ID_IMP_DEF_SOC_ID_MASK) as u16
}

static mut soc_dev: *mut soc_device = core::ptr::null_mut();
static mut soc_dev_attr: *mut soc_device_attribute = core::ptr::null_mut();

#[cfg(target_arch = "aarch64")]
static mut smccc_soc_id_name: [u8; 136] = [0; 136];

#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn str_fragment_from_reg(dst: *mut u8, reg: usize) {
    *dst.add(0) = (reg >> 0) as u8;
    *dst.add(1) = (reg >> 8) as u8;
    *dst.add(2) = (reg >> 16) as u8;
    *dst.add(3) = (reg >> 24) as u8;
    *dst.add(4) = (reg >> 32) as u8;
    *dst.add(5) = (reg >> 40) as u8;
    *dst.add(6) = (reg >> 48) as u8;
    *dst.add(7) = (reg >> 56) as u8;
}

#[cfg(target_arch = "aarch64")]
unsafe fn smccc_soc_name_init() -> *mut u8 {
    let mut args: arm_smccc_1_2_regs = core::mem::zeroed();
    let mut res: arm_smccc_1_2_regs = core::mem::zeroed();

    /*
     * Issue Number 1.6 of the Arm SMC Calling Convention
     * specification introduces an optional "name" string
     * to the ARM_SMCCC_ARCH_SOC_ID function. Fetch it if
     * available.
     */
    args.a0 = ARM_SMCCC_ARCH_SOC_ID64;
    args.a1 = 2; // SOC_ID name
    arm_smccc_1_2_invoke(&args, &mut res);

    if (res.a0 as u32) == 0 {
        /*
         * Copy res.a1..res.a17 to the smccc_soc_id_name string
         * 8 bytes at a time. As per Issue 1.6 of the Arm SMC
         * Calling Convention, the string will be NUL terminated
         * and padded, from the end of the string to the end of the
         * 136 byte buffer, with NULs.
         */
        let regs = [
            res.a1, res.a2, res.a3, res.a4, res.a5, res.a6, res.a7, res.a8,
            res.a9, res.a10, res.a11, res.a12, res.a13, res.a14, res.a15,
            res.a16, res.a17,
        ];
        for (i, reg) in regs.iter().enumerate() {
            str_fragment_from_reg(smccc_soc_id_name.as_mut_ptr().add(8 * i), *reg as usize);
        }

        let len = smccc_soc_id_name.iter().position(|&c| c == 0).unwrap_or(136);
        if len != 0 {
            if len == 136 {
                pr_warn!("FW_BUG Ignoring improperly formatted name\n");
            } else {
                return smccc_soc_id_name.as_mut_ptr();
            }
        }
    }

    core::ptr::null_mut()
}

#[cfg(not(target_arch = "aarch64"))]
unsafe fn smccc_soc_name_init() -> *mut u8 {
    core::ptr::null_mut()
}

unsafe fn smccc_soc_init() -> i32 {
    let mut soc_id_rev: i32;
    let mut soc_id_version: i32;
    static mut soc_id_str: [u8; 20] = [0; 20];
    static mut soc_id_rev_str: [u8; 12] = [0; 12];
    static mut soc_id_jep106_id_str: [u8; 12] = [0; 12];

    if arm_smccc_get_version() < ARM_SMCCC_VERSION_1_2 {
        return 0;
    }

    soc_id_version = arm_smccc_get_soc_id_version();
    if soc_id_version == SMCCC_RET_NOT_SUPPORTED {
        pr_info!("ARCH_SOC_ID not implemented, skipping ....\n");
        return 0;
    }
    if soc_id_version < 0 {
        pr_err!("Invalid SoC Version: {:x}\n", soc_id_version);
        return -EINVAL;
    }

    soc_id_rev = arm_smccc_get_soc_id_revision();
    if soc_id_rev < 0 {
        pr_err!("Invalid SoC Revision: {:x}\n", soc_id_rev);
        return -EINVAL;
    }

    soc_dev_attr = kzalloc_obj::<soc_device_attribute>();
    if soc_dev_attr.is_null() {
        return -ENOMEM;
    }

    sprintf!(soc_id_rev_str, "0x{:08x}", soc_id_rev);
    sprintf!(soc_id_jep106_id_str, "jep106:{:02x}{:02x}",
        jep106_bank_cont_code(soc_id_version as u32),
        jep106_id_code(soc_id_version as u32));
    sprintf!(soc_id_str, "{}:{:04x}", soc_id_jep106_id_str,
        imp_def_soc_id(soc_id_version as u32));

    (*soc_dev_attr).soc_id = soc_id_str.as_mut_ptr();
    (*soc_dev_attr).revision = soc_id_rev_str.as_mut_ptr();
    (*soc_dev_attr).family = soc_id_jep106_id_str.as_mut_ptr();
    (*soc_dev_attr).machine = smccc_soc_name_init();

    soc_dev = soc_device_register(soc_dev_attr);
    if IS_ERR(soc_dev) {
        kfree(soc_dev_attr);
        return PTR_ERR(soc_dev);
    }

    pr_info!("ID = {} Revision = {}\n", (*soc_dev_attr).soc_id, (*soc_dev_attr).revision);
    0
}

module_init!(smccc_soc_init);

unsafe fn smccc_soc_exit() {
    if !soc_dev.is_null() {
        soc_device_unregister(soc_dev);
    }
    kfree(soc_dev_attr);
}

module_exit!(smccc_soc_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
