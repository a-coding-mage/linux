// SPDX-License-Identifier: GPL-2.0-only
//
// rt722-sdca-sdw.rs -- rt722 SDCA ALSA SoC audio driver
//
// Copyright(c) 2023 Realtek Semiconductor Corp.
//
//

// C dependencies translated as external Rust dependencies expected from the
// surrounding kernel driver bindings:
// linux/cleanup.h, linux/delay.h, linux/device.h, linux/module.h,
// linux/pm_runtime.h, linux/soundwire/sdw_registers.h,
// "rt722-sdca.h", "rt722-sdca-sdw.h".

unsafe fn rt722_sdca_mbq_size(dev: *mut device, reg: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let _ = dev;

    match reg {
        0x22f0..=0x22f1
        | 0x2f01..=0x2f0c
        | 0x2f21..=0x2f24
        | 0x2f35..=0x2f36
        | 0x2f50..=0x2f52
        | 0x2f54
        | 0x2f58..=0x2f5d
        | RT722_BUF_ADDR_HID1..=RT722_BUF_ADDR_HID2
        | 0x44011000..=0x440115ff
        | 0x44012000
        | 0x44012021
        | 0x44012022
        | 0x44012025
        | 0x44021000..=0x440211ff
        | 0x44022000
        | 0x44022019
        | 0x4402201a
        | 0x4402201d
        | 0x44041000..=0x440415ff
        | 0x44042000
        | 0x44042019
        | 0x4404201a
        | 0x4404201d => 1,
        _ if reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_GE49, RT722_SDCA_CTL_SELECTED_MODE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_GE49, RT722_SDCA_CTL_DETECTED_MODE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_XU03, RT722_SDCA_CTL_SELECTED_MODE, 0)
            || (reg >= SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_MUTE, CH_L)
                && reg <= SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_MUTE, CH_R))
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_XU0D, RT722_SDCA_CTL_SELECTED_MODE, 0)
            || (reg >= SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_MUTE, CH_L)
                && reg <= SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_MUTE, CH_R))
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE40, RT722_SDCA_CTL_REQ_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE40, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE12, RT722_SDCA_CTL_REQ_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE12, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_CS01, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_CS11, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0)
            || (reg >= SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_MUTE, CH_01)
                && reg <= SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_MUTE, CH_04))
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_IT26, RT722_SDCA_CTL_VENDOR_DEF, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_PDE2A, RT722_SDCA_CTL_REQ_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_PDE2A, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_CS1F, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0)
            || (reg >= SDW_SDCA_CTL(FUNC_NUM_HID, RT722_SDCA_ENT_HID01, RT722_SDCA_CTL_HIDTX_CURRENT_OWNER, 0)
                && reg <= SDW_SDCA_CTL(FUNC_NUM_HID, RT722_SDCA_ENT_HID01, RT722_SDCA_CTL_HIDTX_MESSAGE_LENGTH, 0))
            || (reg >= SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_MUTE, CH_L)
                && reg <= SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_MUTE, CH_R))
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_OT23, RT722_SDCA_CTL_VENDOR_DEF, CH_08)
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_PDE23, RT722_SDCA_CTL_REQ_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_PDE23, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_CS31, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0) => 1,
        0x2000000..=0x2000024
        | 0x2000029..=0x200004a
        | 0x2000051..=0x2000052
        | 0x200005a..=0x200005b
        | 0x2000061..=0x2000069
        | 0x200006b
        | 0x2000070
        | 0x200007f
        | 0x2000082..=0x200008e
        | 0x2000090..=0x2000094
        | 0x20000b1
        | 0x20000b4
        | 0x3010000
        | 0x3110000
        | 0x5300000..=0x5300300
        | 0x5400002
        | 0x5600000..=0x5600007
        | 0x5700000..=0x5700004
        | 0x5800000..=0x5800004
        | 0x5810000
        | 0x5b00003
        | 0x5c00011
        | 0x5d00006
        | 0x5f00000..=0x5f0000d
        | 0x5f00030
        | 0x6100000..=0x6100051
        | 0x6100055..=0x6100057
        | 0x6100060
        | 0x6100062
        | 0x6100064..=0x6100065
        | 0x6100067
        | 0x6100070..=0x610007c
        | 0x6100080 => 2,
        _ if (reg >= SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_FU15, RT722_SDCA_CTL_FU_CH_GAIN, CH_01)
                && reg <= SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_FU15, RT722_SDCA_CTL_FU_CH_GAIN, CH_04))
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_VOLUME, CH_01)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_VOLUME, CH_02)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_VOLUME, CH_03)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_VOLUME, CH_04)
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_VOLUME, CH_L)
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_VOLUME, CH_R)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_VOLUME, CH_L)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_VOLUME, CH_R)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_VOLUME, CH_L)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_VOLUME, CH_R)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PLATFORM_FU44, RT722_SDCA_CTL_FU_CH_GAIN, CH_L)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PLATFORM_FU44, RT722_SDCA_CTL_FU_CH_GAIN, CH_R) => 2,
        _ => 0,
    }
}

static rt722_mbq_config: regmap_sdw_mbq_cfg = regmap_sdw_mbq_cfg {
    mbq_size: Some(rt722_sdca_mbq_size),
};

unsafe fn rt722_sdca_readable_register(dev: *mut device, reg: ::core::ffi::c_uint) -> bool {
    rt722_sdca_mbq_size(dev, reg) > 0
}

unsafe fn rt722_sdca_volatile_register(dev: *mut device, reg: ::core::ffi::c_uint) -> bool {
    let _ = dev;

    match reg {
        0x2f01
        | 0x2f54
        | RT722_BUF_ADDR_HID1..=RT722_BUF_ADDR_HID2
        | 0x2000000
        | 0x2000007
        | 0x200000d
        | 0x2000019
        | 0x200001a
        | 0x2000020
        | 0x2000030
        | 0x2000046
        | 0x2000067
        | 0x2000084
        | 0x2000086
        | 0x3010000
        | 0x3110000
        | 0x5800003
        | 0x5810000
        | 0x6100008
        | 0x44011000..=0x440115ff
        | 0x44012000
        | 0x44012021
        | 0x44012022
        | 0x44012025
        | 0x44021000..=0x440211ff
        | 0x44022000
        | 0x44022019
        | 0x4402201a
        | 0x4402201d
        | 0x44041000..=0x440415ff
        | 0x44042000
        | 0x44042019
        | 0x4404201a
        | 0x4404201d => true,
        _ if reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE12, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE40, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_GE49, RT722_SDCA_CTL_DETECTED_MODE, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_PDE2A, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0)
            || (reg >= SDW_SDCA_CTL(FUNC_NUM_HID, RT722_SDCA_ENT_HID01, RT722_SDCA_CTL_HIDTX_CURRENT_OWNER, 0)
                && reg <= SDW_SDCA_CTL(FUNC_NUM_HID, RT722_SDCA_ENT_HID01, RT722_SDCA_CTL_HIDTX_MESSAGE_LENGTH, 0))
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0)
            || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_PDE23, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0) => true,
        _ => false,
    }
}

static rt722_sdca_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 16,
    readable_reg: Some(rt722_sdca_readable_register),
    volatile_reg: Some(rt722_sdca_volatile_register),
    max_register: 0x44ffffff,
    reg_defaults: rt722_sdca_reg_defaults,
    num_reg_defaults: ARRAY_SIZE(rt722_sdca_reg_defaults),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe fn rt722_sdca_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> ::core::ffi::c_int {
    let rt722 = dev_get_drvdata(&mut (*slave).dev) as *mut rt722_sdca_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt722).hw_init = false;
    }

    if status == SDW_SLAVE_ATTACHED {
        if (*rt722).hs_jack {
            /*
             * Due to the SCP_SDCA_INTMASK will be cleared by any reset, and then
             * if the device attached again, we will need to set the setting back.
             * It could avoid losing the jack detection interrupt.
             * This also could sync with the cache value as the rt722_sdca_jack_init set.
             */
            sdw_write_no_pm((*rt722).slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0);
            sdw_write_no_pm((*rt722).slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8);
        }
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt722).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt722_sdca_io_init(&mut (*slave).dev, slave)
}

unsafe fn rt722_sdca_read_prop(slave: *mut sdw_slave) -> ::core::ffi::c_int {
    let prop: *mut sdw_slave_prop = &mut (*slave).prop;
    let mut nval: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut bit: u32;
    let mut addr: ::core::ffi::c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    sdw_slave_read_lane_mapping(slave);

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    (*prop).paging_support = true;

    /*
     * port = 1 for headphone playback
     * port = 2 for headset-mic capture
     * port = 3 for speaker playback
     * port = 6 for digital-mic capture
     */
    (*prop).source_ports = BIT(6) | BIT(2); /* BITMAP: 01000100 */
    (*prop).sink_ports = BIT(3) | BIT(1); /* BITMAP:  00001010 */

    nval = hweight32((*prop).source_ports);
    (*prop).src_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        ::core::mem::size_of_val(&*(*prop).src_dpn_prop),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if (*prop).src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = (*prop).src_dpn_prop;
    addr = (*prop).source_ports as ::core::ffi::c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1usize << bit)) != 0 {
            (*dpn.add(i as usize)).num = bit;
            (*dpn.add(i as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(i as usize)).simple_ch_prep_sm = true;
            (*dpn.add(i as usize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = hweight32((*prop).sink_ports);
    (*prop).sink_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        ::core::mem::size_of_val(&*(*prop).sink_dpn_prop),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if (*prop).sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    j = 0;
    dpn = (*prop).sink_dpn_prop;
    addr = (*prop).sink_ports as ::core::ffi::c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1usize << bit)) != 0 {
            (*dpn.add(j as usize)).num = bit;
            (*dpn.add(j as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(j as usize)).simple_ch_prep_sm = true;
            (*dpn.add(j as usize)).ch_prep_timeout = 10;
            j += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 900;

    /* wake-up event */
    (*prop).wake_capable = 1;

    /* Three data lanes are supported by rt722-sdca codec */
    (*prop).lane_control_support = true;

    0
}

unsafe fn rt722_sdca_interrupt_callback(
    slave: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> ::core::ffi::c_int {
    let rt722 = dev_get_drvdata(&mut (*slave).dev) as *mut rt722_sdca_priv;
    let mut ret: ::core::ffi::c_int;
    let mut stat: ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0;
    let retry: ::core::ffi::c_int = 3;
    let mut sdca_cascade: ::core::ffi::c_uint;
    let mut scp_sdca_stat1: ::core::ffi::c_uint;
    let mut scp_sdca_stat2: ::core::ffi::c_uint = 0;

    if cancel_delayed_work_sync(&mut (*rt722).jack_detect_work) {
        dev_warn(&mut (*slave).dev, c"%s the pending delayed_work was cancelled".as_ptr(), __func__);
        /* avoid the HID owner doesn't change to device */
        if (*rt722).scp_sdca_stat2 != 0 {
            scp_sdca_stat2 = (*rt722).scp_sdca_stat2;
        }
    }

    /*
     * The critical section below intentionally protects a rather large piece of code.
     * We don't want to allow the system suspend to disable an interrupt while we are
     * processing it, which could be problematic given the quirky SoundWire interrupt
     * scheme. We do want however to prevent new workqueues from being scheduled if
     * the disable_irq flag was set during system suspend.
     */
    mutex_lock(&mut (*rt722).disable_irq_lock);

    ret = sdw_read_no_pm((*rt722).slave, SDW_SCP_SDCA_INT1);
    if ret < 0 {
        goto_io_error(rt722, ret);
        return ret;
    }
    (*rt722).scp_sdca_stat1 = ret as ::core::ffi::c_uint;
    ret = sdw_read_no_pm((*rt722).slave, SDW_SCP_SDCA_INT2);
    if ret < 0 {
        goto_io_error(rt722, ret);
        return ret;
    }
    (*rt722).scp_sdca_stat2 = ret as ::core::ffi::c_uint;
    if scp_sdca_stat2 != 0 {
        (*rt722).scp_sdca_stat2 |= scp_sdca_stat2;
    }

    loop {
        /* clear flag */
        ret = sdw_read_no_pm((*rt722).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            goto_io_error(rt722, ret);
            return ret;
        }
        if (ret as ::core::ffi::c_uint & SDW_SCP_SDCA_INTMASK_SDCA_0) != 0 {
            ret = sdw_update_no_pm(
                (*rt722).slave,
                SDW_SCP_SDCA_INT1,
                SDW_SCP_SDCA_INT_SDCA_0,
                SDW_SCP_SDCA_INT_SDCA_0,
            );
            if ret < 0 {
                goto_io_error(rt722, ret);
                return ret;
            }
        }

        ret = sdw_read_no_pm((*rt722).slave, SDW_SCP_SDCA_INT2);
        if ret < 0 {
            goto_io_error(rt722, ret);
            return ret;
        }
        if (ret as ::core::ffi::c_uint & SDW_SCP_SDCA_INTMASK_SDCA_8) != 0 {
            ret = sdw_write_no_pm((*rt722).slave, SDW_SCP_SDCA_INT2, SDW_SCP_SDCA_INTMASK_SDCA_8);
            if ret < 0 {
                goto_io_error(rt722, ret);
                return ret;
            }
        }

        /* check if flag clear or not */
        ret = sdw_read_no_pm((*rt722).slave, SDW_DP0_INT);
        if ret < 0 {
            goto_io_error(rt722, ret);
            return ret;
        }
        sdca_cascade = ret as ::core::ffi::c_uint & SDW_DP0_SDCA_CASCADE;

        ret = sdw_read_no_pm((*rt722).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            goto_io_error(rt722, ret);
            return ret;
        }
        scp_sdca_stat1 = ret as ::core::ffi::c_uint & SDW_SCP_SDCA_INTMASK_SDCA_0;

        ret = sdw_read_no_pm((*rt722).slave, SDW_SCP_SDCA_INT2);
        if ret < 0 {
            goto_io_error(rt722, ret);
            return ret;
        }
        scp_sdca_stat2 = ret as ::core::ffi::c_uint & SDW_SCP_SDCA_INTMASK_SDCA_8;

        stat = (scp_sdca_stat1 != 0 || scp_sdca_stat2 != 0 || sdca_cascade != 0) as ::core::ffi::c_int;

        count += 1;
        if !(stat != 0 && count < retry) {
            break;
        }
    }

    if stat != 0 {
        dev_warn(
            &mut (*slave).dev,
            c"%s scp_sdca_stat1=0x%x, scp_sdca_stat2=0x%x\n".as_ptr(),
            __func__,
            (*rt722).scp_sdca_stat1,
            (*rt722).scp_sdca_stat2,
        );
    }

    if (*status).sdca_cascade && !(*rt722).disable_irq {
        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt722).jack_detect_work,
            msecs_to_jiffies(280),
        );
    }

    mutex_unlock(&mut (*rt722).disable_irq_lock);

    0
}

unsafe fn goto_io_error(rt722: *mut rt722_sdca_priv, ret: ::core::ffi::c_int) {
    mutex_unlock(&mut (*rt722).disable_irq_lock);
    pr_err_ratelimited(c"IO error in %s, ret %d\n".as_ptr(), __func__, ret);
}

static rt722_sdca_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt722_sdca_read_prop),
    interrupt_callback: Some(rt722_sdca_interrupt_callback),
    update_status: Some(rt722_sdca_update_status),
};

unsafe fn rt722_sdca_sdw_probe(
    slave: *mut sdw_slave,
    id: *const sdw_device_id,
) -> ::core::ffi::c_int {
    let _ = id;
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw_mbq_cfg(
        &mut (*slave).dev,
        slave,
        &rt722_sdca_regmap,
        &rt722_mbq_config,
    );
    if IS_ERR(regmap) {
        return PTR_ERR(regmap);
    }

    rt722_sdca_init(&mut (*slave).dev, regmap, slave)
}

unsafe fn rt722_sdca_sdw_remove(slave: *mut sdw_slave) {
    let rt722 = dev_get_drvdata(&mut (*slave).dev) as *mut rt722_sdca_priv;

    if (*rt722).hw_init {
        cancel_delayed_work_sync(&mut (*rt722).jack_detect_work);
        cancel_delayed_work_sync(&mut (*rt722).jack_btn_check_work);
    }

    if (*rt722).first_hw_init {
        pm_runtime_disable(&mut (*slave).dev);
    }

    mutex_destroy(&mut (*rt722).calibrate_mutex);
    mutex_destroy(&mut (*rt722).disable_irq_lock);
}

static rt722_sdca_id: [sdw_device_id; 2] = [
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x722, 0x3, 0x1, 0),
    sdw_device_id::default(),
];
// MODULE_DEVICE_TABLE(sdw, rt722_sdca_id);

unsafe fn rt722_sdca_dev_suspend(dev: *mut device) -> ::core::ffi::c_int {
    let rt722 = dev_get_drvdata(dev) as *mut rt722_sdca_priv;

    if !(*rt722).hw_init {
        return 0;
    }

    cancel_delayed_work_sync(&mut (*rt722).jack_detect_work);
    cancel_delayed_work_sync(&mut (*rt722).jack_btn_check_work);

    regcache_cache_only((*rt722).regmap, true);

    0
}

unsafe fn rt722_sdca_dev_system_suspend(dev: *mut device) -> ::core::ffi::c_int {
    let rt722_sdca = dev_get_drvdata(dev) as *mut rt722_sdca_priv;
    let slave = dev_to_sdw_dev(dev);
    let ret1: ::core::ffi::c_int;
    let ret2: ::core::ffi::c_int;

    if !(*rt722_sdca).hw_init {
        return 0;
    }

    /*
     * prevent new interrupts from being handled after the
     * deferred work completes and before the parent disables
     * interrupts on the link
     */
    mutex_lock(&mut (*rt722_sdca).disable_irq_lock);
    (*rt722_sdca).disable_irq = true;
    ret1 = sdw_update_no_pm(slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0, 0);
    ret2 = sdw_update_no_pm(slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8, 0);
    mutex_unlock(&mut (*rt722_sdca).disable_irq_lock);

    if ret1 < 0 || ret2 < 0 {
        /* log but don't prevent suspend from happening */
        dev_dbg(&mut (*slave).dev, c"%s: could not disable SDCA interrupts\n:".as_ptr(), __func__);
    }

    rt722_sdca_dev_suspend(dev)
}

const RT722_PROBE_TIMEOUT: ::core::ffi::c_int = 5000;

unsafe fn rt722_sdca_dev_resume(dev: *mut device) -> ::core::ffi::c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt722 = dev_get_drvdata(dev) as *mut rt722_sdca_priv;
    let mut ret: ::core::ffi::c_int;

    if !(*rt722).first_hw_init {
        return 0;
    }

    if !(*slave).unattach_request {
        mutex_lock(&mut (*rt722).disable_irq_lock);
        if (*rt722).disable_irq {
            sdw_write_no_pm(slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0);
            sdw_write_no_pm(slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8);
            (*rt722).disable_irq = false;
        }
        mutex_unlock(&mut (*rt722).disable_irq_lock);
    }

    ret = sdw_slave_wait_for_init(slave, RT722_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt722).regmap, false);
    ret = regcache_sync((*rt722).regmap);
    if ret != 0 {
        regcache_cache_only((*rt722).regmap, true);
        regcache_mark_dirty((*rt722).regmap);
        return ret;
    }

    0
}

static rt722_sdca_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(rt722_sdca_dev_system_suspend, rt722_sdca_dev_resume)
    // RUNTIME_PM_OPS(rt722_sdca_dev_suspend, rt722_sdca_dev_resume, NULL)
    system_suspend: Some(rt722_sdca_dev_system_suspend),
    system_resume: Some(rt722_sdca_dev_resume),
    runtime_suspend: Some(rt722_sdca_dev_suspend),
    runtime_resume: Some(rt722_sdca_dev_resume),
    runtime_idle: None,
};

static mut rt722_sdca_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: c"rt722-sdca".as_ptr(),
        pm: pm_ptr(&rt722_sdca_pm),
    },
    probe: Some(rt722_sdca_sdw_probe),
    remove: Some(rt722_sdca_sdw_remove),
    ops: &rt722_sdca_slave_ops,
    id_table: rt722_sdca_id.as_ptr(),
};
// module_sdw_driver(rt722_sdca_sdw_driver);

// MODULE_DESCRIPTION("ASoC RT722 SDCA SDW driver");
// MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
