// SPDX-License-Identifier: GPL-2.0-only
//
// rt766-sdca-sdw.c -- rt766 SDCA ALSA SoC audio driver
//
// Copyright(c) 2026 Realtek Semiconductor Corp.
//
//

// C include dependencies translated as external Rust dependencies:
// linux/delay.h, linux/device.h, linux/module.h, linux/pm_runtime.h,
// linux/soundwire/sdw_registers.h, sound/sdca.h, sound/sdca_function.h,
// "rt766-sdca.h", and "rt766-sdca-sdw.h".
use crate::*;

pub const RT766_PROBE_TIMEOUT: i32 = 5000;

unsafe extern "C" fn rt766_sdca_readable_register(
    dev: *mut device,
    reg: ::core::ffi::c_uint,
) -> bool {
    match reg {
        SDW_SCP_SDCA_INT1..=SDW_SCP_SDCA_INTMASK4
        | RT766_VERSION_ID..=RT766_BOND_LATCH_ID
        | 0xc344..=0xc345
        | 0xc900
        | 0xc920
        | 0xd540..=0xd542
        | 0xf01e
        | RT766_HP_POWER_STATE..=RT766_HP_FSM_CTL2_1
        | 0x310100
        | RT766_MCU_PATCH_ADDR1_START..=RT766_MCU_PATCH_ADDR1_END
        | RT766_MCU_PATCH_ADDR2_START..=RT766_MCU_PATCH_ADDR2_END
        | RT766_BUF_ADDR_HID1..=RT766_BUF_ADDR_HID2 => true,
        _ if reg == RT766_MUTE_REG(UAJ, USER_FU41, 1)
            || reg == RT766_MUTE_REG(UAJ, USER_FU41, 2)
            || reg == RT766_VOLUME_REG(UAJ, USER_FU41, 1)
            || reg == RT766_VOLUME_REG(UAJ, USER_FU41, 2)
            || reg == RT766_MUTE_REG(UAJ, USER_FU36, 1)
            || reg == RT766_MUTE_REG(UAJ, USER_FU36, 2)
            || reg == RT766_VOLUME_REG(UAJ, USER_FU36, 1)
            || reg == RT766_VOLUME_REG(UAJ, USER_FU36, 2)
            || reg == RT766_PDE_REQ_REG(UAJ, PDE47)
            || reg == RT766_PDE_REQ_REG(UAJ, PDE34)
            || reg == RT766_SDCA_CTL(UAJ, CS41, SDCA_CTL_CS_SAMPLERATEINDEX)
            || reg == RT766_SDCA_CTL(UAJ, CS36, SDCA_CTL_CS_SAMPLERATEINDEX)
            || reg == RT766_FUNC_STATUS_REG(UAJ) /* 0x40480000 */
            || reg == RT766_PDE_ACTUAL_REG(UAJ, PDE47) /* 0x40481400 */
            || reg == RT766_PDE_ACTUAL_REG(UAJ, PDE34) /* 0x40481480 */
            || reg == RT766_GAIN_REG(UAJ, PLATFORM_FU33, 1)
            || reg == RT766_GAIN_REG(UAJ, PLATFORM_FU33, 2)
            || reg == RT766_SDCA_CTL(UAJ, GE49, SDCA_CTL_GE_SELECTED_MODE)
            || reg == RT766_SDCA_CTL(UAJ, GE49, SDCA_CTL_GE_DETECTED_MODE) /* 0x40600490 */
            || reg == RT766_PDE_REQ_REG(MIC, PDE11)
            || reg == RT766_MUTE_REG(MIC, USER_FU113, 1)
            || reg == RT766_MUTE_REG(MIC, USER_FU113, 2)
            || reg == RT766_MUTE_REG(MIC, USER_FU113, 3)
            || reg == RT766_MUTE_REG(MIC, USER_FU113, 4)
            || reg == RT766_VOLUME_REG(MIC, USER_FU113, 1)
            || reg == RT766_VOLUME_REG(MIC, USER_FU113, 2)
            || reg == RT766_VOLUME_REG(MIC, USER_FU113, 3)
            || reg == RT766_VOLUME_REG(MIC, USER_FU113, 4)
            || reg == RT766_FUNC_STATUS_REG(MIC) /* 0x40880000 */
            || reg == RT766_SDCA_CTL(MIC, CS113, SDCA_CTL_CS_SAMPLERATEINDEX)
            || reg == RT766_PDE_ACTUAL_REG(MIC, PDE11) /* 0x40881500 */
            || reg == RT766_FUNC_STATUS_REG(HID) /* 0x40c80000 */
            /* 0x40c80080 - 0x40c80098 */
            || (reg >= RT766_SDCA_CTL(HID, HID101, SDCA_CTL_HIDE_HIDTX_CURRENTOWNER)
                && reg <= RT766_SDCA_CTL(HID, HID101, SDCA_CTL_HIDE_HIDTX_MESSAGELENGTH))
            || reg == RT766_MUTE_REG(AMP, USER_FU21, 1)
            || reg == RT766_MUTE_REG(AMP, USER_FU21, 2)
            || reg == RT766_VOLUME_REG(AMP, USER_FU21, 1)
            || reg == RT766_VOLUME_REG(AMP, USER_FU21, 2)
            || reg == RT766_PDE_REQ_REG(AMP, PDE23)
            || reg == RT766_FUNC_STATUS_REG(AMP) /* 0x41080000 */
            || reg == RT766_SDCA_CTL(AMP, PPU21, SDCA_CTL_PPU_POSTURENUMBER)
            || reg == RT766_SDCA_CTL(AMP, CS21, SDCA_CTL_CS_SAMPLERATEINDEX)
            || reg == RT766_PDE_ACTUAL_REG(AMP, PDE23) /* 0x41081980 */ =>
        {
            true
        }
        _ => false,
    }
}

unsafe extern "C" fn rt766_sdca_volatile_register(
    dev: *mut device,
    reg: ::core::ffi::c_uint,
) -> bool {
    match reg {
        SDW_SCP_SDCA_INT1..=SDW_SCP_SDCA_INTMASK4
        | RT766_VERSION_ID..=RT766_BOND_LATCH_ID
        | 0xc344..=0xc345
        | 0xc900
        | 0xc920
        | 0xd540..=0xd542
        | 0xf01e
        | RT766_HP_POWER_STATE..=RT766_HP_FSM_CTL2_1
        | 0x310100
        | RT766_MCU_PATCH_ADDR1_START..=RT766_MCU_PATCH_ADDR1_END
        | RT766_MCU_PATCH_ADDR2_START..=RT766_MCU_PATCH_ADDR2_END
        | RT766_BUF_ADDR_HID1..=RT766_BUF_ADDR_HID2 => true,
        _ if reg == RT766_FUNC_STATUS_REG(UAJ)
            || reg == RT766_PDE_ACTUAL_REG(UAJ, PDE47)
            || reg == RT766_PDE_ACTUAL_REG(UAJ, PDE34)
            || reg == RT766_SDCA_CTL(UAJ, GE49, SDCA_CTL_GE_DETECTED_MODE)
            || reg == RT766_FUNC_STATUS_REG(MIC)
            || reg == RT766_PDE_ACTUAL_REG(MIC, PDE11)
            || reg == RT766_FUNC_STATUS_REG(HID)
            || (reg >= RT766_SDCA_CTL(HID, HID101, SDCA_CTL_HIDE_HIDTX_CURRENTOWNER)
                && reg <= RT766_SDCA_CTL(HID, HID101, SDCA_CTL_HIDE_HIDTX_MESSAGELENGTH))
            || reg == RT766_FUNC_STATUS_REG(AMP)
            || reg == RT766_PDE_ACTUAL_REG(AMP, PDE23) =>
        {
            true
        }
        _ => false,
    }
}

unsafe extern "C" fn rt766_sdca_mbq_size(
    dev: *mut device,
    reg: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if reg == RT766_VOLUME_REG(UAJ, USER_FU41, 1)
        || reg == RT766_VOLUME_REG(UAJ, USER_FU41, 2)
        || reg == RT766_VOLUME_REG(UAJ, USER_FU36, 1)
        || reg == RT766_VOLUME_REG(UAJ, USER_FU36, 2)
        || reg == RT766_GAIN_REG(UAJ, PLATFORM_FU33, 1)
        || reg == RT766_GAIN_REG(UAJ, PLATFORM_FU33, 2)
        || reg == RT766_VOLUME_REG(MIC, USER_FU113, 1)
        || reg == RT766_VOLUME_REG(MIC, USER_FU113, 2)
        || reg == RT766_VOLUME_REG(MIC, USER_FU113, 3)
        || reg == RT766_VOLUME_REG(MIC, USER_FU113, 4)
        || reg == RT766_VOLUME_REG(AMP, USER_FU21, 1)
        || reg == RT766_VOLUME_REG(AMP, USER_FU21, 2)
    {
        2
    } else {
        1
    }
}

static rt766_sdca_mbq_cfg: regmap_sdw_mbq_cfg = regmap_sdw_mbq_cfg {
    mbq_size: Some(rt766_sdca_mbq_size),
};

static rt766_sdca_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 16,
    readable_reg: Some(rt766_sdca_readable_register),
    volatile_reg: Some(rt766_sdca_volatile_register),
    reg_defaults: rt766_sdca_defaults,
    num_reg_defaults: ARRAY_SIZE(rt766_sdca_defaults),
    max_register: SDW_SDCA_MAX_REGISTER,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn rt766_sdca_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> ::core::ffi::c_int {
    let rt766: *mut rt766_sdca_priv = dev_get_drvdata(&mut (*slave).dev) as *mut rt766_sdca_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt766).hw_init = false;
    }

    if status == SDW_SLAVE_ATTACHED {
        if (*rt766).hs_jack {
            /*
             * Due to the SCP_SDCA_INTMASK will be cleared by any reset, and then
             * if the device attached again, we will need to set the setting back.
             * It could avoid losing the jack detection interrupt.
             * This also could sync with the cache value as the rt766_sdca_jack_init set.
             */
            sdw_write_no_pm(
                (*rt766).slave,
                SDW_SCP_SDCA_INTMASK3,
                SDW_SCP_SDCA_INTMASK_SDCA_16,
            );
            sdw_write_no_pm(
                (*rt766).slave,
                SDW_SCP_SDCA_INTMASK4,
                SDW_SCP_SDCA_INTMASK_SDCA_24,
            );
        }
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt766).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt766_sdca_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt766_sdca_read_prop(slave: *mut sdw_slave) -> ::core::ffi::c_int {
    let prop: *mut sdw_slave_prop = &mut (*slave).prop;
    let mut ret: ::core::ffi::c_int;

    ret = sdw_slave_read_prop(slave);
    if ret < 0 {
        return ret;
    }

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;
    /*
     * SDCA interrupts are routed through SoundWire domain IRQ.
     */
    (*prop).use_domain_irq = true;

    0
}

static rt766_sdca_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt766_sdca_read_prop),
    update_status: Some(rt766_sdca_update_status),
};

unsafe extern "C" fn rt766_sdca_sdw_probe(
    slave: *mut sdw_slave,
    id: *const sdw_device_id,
) -> ::core::ffi::c_int {
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw_mbq_cfg(
        &mut (*slave).dev,
        slave,
        &rt766_sdca_regmap,
        &rt766_sdca_mbq_cfg,
    );
    if IS_ERR(regmap) {
        return PTR_ERR(regmap);
    }

    rt766_sdca_init(&mut (*slave).dev, regmap, slave)
}

unsafe extern "C" fn rt766_sdca_sdw_remove(slave: *mut sdw_slave) {
    pm_runtime_disable(&mut (*slave).dev);
}

static rt766_sdca_id: [sdw_device_id; 5] = [
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x766, 0x3, 0x1, 0),
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x767, 0x3, 0x1, 0),
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x766, 0x4, 0x1, 0),
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x767, 0x4, 0x1, 0),
    sdw_device_id::default(),
];
// MODULE_DEVICE_TABLE(sdw, rt766_sdca_id);

unsafe extern "C" fn rt766_sdca_dev_suspend(dev: *mut device) -> ::core::ffi::c_int {
    let rt766: *mut rt766_sdca_priv = dev_get_drvdata(dev) as *mut rt766_sdca_priv;

    if !(*rt766).hw_init {
        return 0;
    }

    regcache_cache_only((*rt766).regmap, true);
    0
}

unsafe extern "C" fn rt766_sdca_dev_system_suspend(dev: *mut device) -> ::core::ffi::c_int {
    let rt766: *mut rt766_sdca_priv = dev_get_drvdata(dev) as *mut rt766_sdca_priv;
    let slave: *mut sdw_slave = dev_to_sdw_dev(dev);
    let ret1: ::core::ffi::c_int;
    let ret2: ::core::ffi::c_int;

    if !(*rt766).hw_init {
        return 0;
    }

    /*
     * prevent new interrupts from being handled after the
     * deferred work completes and before the parent disables
     * interrupts on the link
     */
    mutex_lock(&mut (*rt766).disable_irq_lock);
    (*rt766).disable_irq = true;
    ret1 = sdw_update_no_pm(
        slave,
        SDW_SCP_SDCA_INTMASK3,
        SDW_SCP_SDCA_INTMASK_SDCA_16,
        0,
    );
    ret2 = sdw_update_no_pm(
        slave,
        SDW_SCP_SDCA_INTMASK4,
        SDW_SCP_SDCA_INTMASK_SDCA_24,
        0,
    );
    mutex_unlock(&mut (*rt766).disable_irq_lock);

    if ret1 < 0 || ret2 < 0 {
        /* log but don't prevent suspend from happening */
        dev_dbg(
            &mut (*slave).dev,
            c"%s: could not disable SDCA interrupts\n:",
            __func__,
        );
    }

    rt766_sdca_dev_suspend(dev)
}

unsafe extern "C" fn rt766_sdca_dev_resume(dev: *mut device) -> ::core::ffi::c_int {
    let slave: *mut sdw_slave = dev_to_sdw_dev(dev);
    let rt766: *mut rt766_sdca_priv = dev_get_drvdata(dev) as *mut rt766_sdca_priv;
    let mut ret: ::core::ffi::c_int;

    if !(*rt766).first_hw_init {
        return 0;
    }

    if !(*slave).unattach_request {
        mutex_lock(&mut (*rt766).disable_irq_lock);
        if (*rt766).disable_irq == true {
            sdw_write_no_pm(slave, SDW_SCP_SDCA_INTMASK3, SDW_SCP_SDCA_INTMASK_SDCA_16);
            sdw_write_no_pm(slave, SDW_SCP_SDCA_INTMASK4, SDW_SCP_SDCA_INTMASK_SDCA_24);
            (*rt766).disable_irq = false;
        }
        mutex_unlock(&mut (*rt766).disable_irq_lock);
    } else {
        ret = sdw_slave_wait_for_init(slave, RT766_PROBE_TIMEOUT);
        if ret != 0 {
            sdw_show_ping_status((*slave).bus, true);
            return ret;
        }
    }

    regcache_cache_only((*rt766).regmap, false);
    ret = regcache_sync((*rt766).regmap);
    if ret != 0 {
        regcache_cache_only((*rt766).regmap, true);
        regcache_mark_dirty((*rt766).regmap);
        return ret;
    }

    0
}

static rt766_sdca_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(rt766_sdca_dev_system_suspend, rt766_sdca_dev_resume)
    // RUNTIME_PM_OPS(rt766_sdca_dev_suspend, rt766_sdca_dev_resume, NULL)
    system_suspend: Some(rt766_sdca_dev_system_suspend),
    system_resume: Some(rt766_sdca_dev_resume),
    runtime_suspend: Some(rt766_sdca_dev_suspend),
    runtime_resume: Some(rt766_sdca_dev_resume),
    runtime_idle: None,
};

static mut rt766_sdca_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: c"rt766-sdca",
        pm: pm_ptr(&rt766_sdca_pm),
    },
    probe: Some(rt766_sdca_sdw_probe),
    remove: Some(rt766_sdca_sdw_remove),
    ops: &rt766_sdca_slave_ops,
    id_table: rt766_sdca_id.as_ptr(),
};
// module_sdw_driver(rt766_sdca_sdw_driver);

// MODULE_DESCRIPTION("ASoC RT766 SDCA SDW driver");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
