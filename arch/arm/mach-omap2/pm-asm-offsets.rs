// SPDX-License-Identifier: GPL-2.0
/*
 * TI AM33XX and AM43XX PM Assembly Offsets
 *
 * Copyright (C) 2017-2018 Texas Instruments Inc.
 */

// Dependencies supplied by the Linux build environment:
// <linux/kbuild.h>, <linux/platform_data/pm33xx.h>, and <linux/ti-emif-sram.h>

unsafe extern "C" {
    fn ti_emif_asm_offsets();
}

// DEFINE!, BLANK!, and the AM33XX structure definitions are supplied by the
// corresponding kernel offset-generation environment.

fn main() -> i32 {
    unsafe {
        ti_emif_asm_offsets();
    }

    DEFINE!(
        AMX3_PM_WFI_FLAGS_OFFSET,
        core::mem::offset_of!(am33xx_pm_sram_data, wfi_flags)
    );
    DEFINE!(
        AMX3_PM_L2_AUX_CTRL_VAL_OFFSET,
        core::mem::offset_of!(am33xx_pm_sram_data, l2_aux_ctrl_val)
    );
    DEFINE!(
        AMX3_PM_L2_PREFETCH_CTRL_VAL_OFFSET,
        core::mem::offset_of!(am33xx_pm_sram_data, l2_prefetch_ctrl_val)
    );
    DEFINE!(
        AMX3_PM_SRAM_DATA_SIZE,
        core::mem::size_of::<am33xx_pm_sram_data>()
    );

    BLANK!();

    DEFINE!(
        AMX3_PM_RO_SRAM_DATA_VIRT_OFFSET,
        core::mem::offset_of!(am33xx_pm_ro_sram_data, amx3_pm_sram_data_virt)
    );
    DEFINE!(
        AMX3_PM_RO_SRAM_DATA_PHYS_OFFSET,
        core::mem::offset_of!(am33xx_pm_ro_sram_data, amx3_pm_sram_data_phys)
    );
    DEFINE!(
        AMX3_PM_RTC_BASE_VIRT_OFFSET,
        core::mem::offset_of!(am33xx_pm_ro_sram_data, rtc_base_virt)
    );
    DEFINE!(
        AMX3_PM_RO_SRAM_DATA_SIZE,
        core::mem::size_of::<am33xx_pm_ro_sram_data>()
    );

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
