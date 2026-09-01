// SPDX-License-Identifier: GPL-2.0
//
// rt700-sdw.c -- rt700 ALSA SoC audio driver
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
//
//

// C includes removed: linux/cleanup.h, linux/delay.h, linux/device.h,
// linux/soundwire/sdw.h, linux/soundwire/sdw_type.h,
// linux/soundwire/sdw_registers.h, linux/module.h, linux/pm_runtime.h,
// linux/regmap.h, sound/soc.h, rt700.h, rt700-sdw.h.
// The referenced kernel, SoundWire, regmap, and RT700 symbols are expected
// to be supplied by surrounding bindings.

unsafe fn rt700_readable_register(dev: *mut device, reg: ::core::ffi::c_uint) -> bool {
    match reg {
        0x00e0
        | 0x00f0
        | 0x2000..=0x200e
        | 0x2012..=0x2016
        | 0x201a..=0x2027
        | 0x2029..=0x202a
        | 0x202d..=0x2034
        | 0x2200..=0x2204
        | 0x2206..=0x2212
        | 0x2220..=0x2223
        | 0x2230..=0x2231
        | 0x3000..=0x3fff
        | 0x7000..=0x7fff
        | 0x8300..=0x83ff
        | 0x9c00..=0x9cff
        | 0xb900..=0xb9ff
        | 0x75201a
        | 0x752045
        | 0x752046
        | 0x752048
        | 0x75204a
        | 0x75206b
        | 0x752080
        | 0x752081 => true,
        _ => false,
    }
}

unsafe fn rt700_volatile_register(dev: *mut device, reg: ::core::ffi::c_uint) -> bool {
    match reg {
        0x2009
        | 0x2016
        | 0x201b
        | 0x201c
        | 0x201d
        | 0x201f
        | 0x2021
        | 0x2023
        | 0x2230
        | 0x200b..=0x200e /* i2c read */
        | 0x2012..=0x2015 /* HD-A read */
        | 0x202d..=0x202f /* BRA */
        | 0x2201..=0x2212 /* i2c debug */
        | 0x2220..=0x2223 /* decoded HD-A */
        | 0x9c00..=0x9cff
        | 0xb900..=0xb9ff
        | 0xff01
        | 0x75201a
        | 0x752046
        | 0x752080
        | 0x752081 => true,
        _ => false,
    }
}

unsafe fn rt700_sdw_read(
    context: *mut ::core::ffi::c_void,
    mut reg: ::core::ffi::c_uint,
    val: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let dev: *mut device = context as *mut device;
    let rt700: *mut rt700_priv = dev_get_drvdata(dev) as *mut rt700_priv;
    let mut sdw_data_3: ::core::ffi::c_uint;
    let mut sdw_data_2: ::core::ffi::c_uint;
    let mut sdw_data_1: ::core::ffi::c_uint;
    let mut sdw_data_0: ::core::ffi::c_uint;
    let mut reg2: ::core::ffi::c_uint = 0;
    let mut reg3: ::core::ffi::c_uint = 0;
    let mut reg4: ::core::ffi::c_uint = 0;
    let mask: ::core::ffi::c_uint;
    let nid: ::core::ffi::c_uint;
    let val2: ::core::ffi::c_uint;
    let mut is_hda_reg: ::core::ffi::c_uint = 1;
    let mut is_index_reg: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int;

    if reg > 0xffff {
        is_index_reg = 1;
    }

    mask = reg & 0xf000;

    if is_index_reg != 0 {
        /* index registers */
        val2 = reg & 0xff;
        reg = reg >> 8;
        nid = reg & 0xff;
        ret = regmap_write((*rt700).sdw_regmap, reg, 0);
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg2, val2);
        if ret < 0 {
            return ret;
        }

        reg3 = RT700_PRIV_DATA_R_H | nid;
        ret = regmap_write((*rt700).sdw_regmap, reg3, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg4 = reg3 + 0x1000;
        reg4 |= 0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg4, (*val & 0xff));
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x3000 {
        reg += 0x8000;
        ret = regmap_write((*rt700).sdw_regmap, reg, *val);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x7000 {
        reg += 0x2000;
        reg |= 0x800;
        ret = regmap_write((*rt700).sdw_regmap, reg, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg2, (*val & 0xff));
        if ret < 0 {
            return ret;
        }
    } else if (reg & 0xff00) == 0x8300 {
        /* for R channel */
        reg2 = reg - 0x1000;
        reg2 &= !0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg2, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        ret = regmap_write((*rt700).sdw_regmap, reg, (*val & 0xff));
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x9000 {
        ret = regmap_write((*rt700).sdw_regmap, reg, ((*val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg2, (*val & 0xff));
        if ret < 0 {
            return ret;
        }
    } else if mask == 0xb000 {
        ret = regmap_write((*rt700).sdw_regmap, reg, *val);
        if ret < 0 {
            return ret;
        }
    } else {
        ret = regmap_read((*rt700).sdw_regmap, reg, val);
        if ret < 0 {
            return ret;
        }
        is_hda_reg = 0;
    }

    if is_hda_reg != 0 || is_index_reg != 0 {
        sdw_data_3 = 0;
        sdw_data_2 = 0;
        sdw_data_1 = 0;
        sdw_data_0 = 0;
        ret = regmap_read((*rt700).sdw_regmap, RT700_READ_HDA_3, &mut sdw_data_3);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt700).sdw_regmap, RT700_READ_HDA_2, &mut sdw_data_2);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt700).sdw_regmap, RT700_READ_HDA_1, &mut sdw_data_1);
        if ret < 0 {
            return ret;
        }
        ret = regmap_read((*rt700).sdw_regmap, RT700_READ_HDA_0, &mut sdw_data_0);
        if ret < 0 {
            return ret;
        }
        *val = ((sdw_data_3 & 0xff) << 24)
            | ((sdw_data_2 & 0xff) << 16)
            | ((sdw_data_1 & 0xff) << 8)
            | (sdw_data_0 & 0xff);
    }

    if is_hda_reg == 0 {
        dev_dbg(dev, c"[%s] %04x => %08x\n".as_ptr(), __func__, reg, *val);
    } else if is_index_reg != 0 {
        dev_dbg(
            dev,
            c"[%s] %04x %04x %04x %04x => %08x\n".as_ptr(),
            __func__,
            reg,
            reg2,
            reg3,
            reg4,
            *val,
        );
    } else {
        dev_dbg(
            dev,
            c"[%s] %04x %04x => %08x\n".as_ptr(),
            __func__,
            reg,
            reg2,
            *val,
        );
    }

    0
}

unsafe fn rt700_sdw_write(
    context: *mut ::core::ffi::c_void,
    mut reg: ::core::ffi::c_uint,
    val: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let dev: *mut device = context as *mut device;
    let rt700: *mut rt700_priv = dev_get_drvdata(dev) as *mut rt700_priv;
    let mut reg2: ::core::ffi::c_uint = 0;
    let reg3: ::core::ffi::c_uint;
    let reg4: ::core::ffi::c_uint;
    let nid: ::core::ffi::c_uint;
    let mask: ::core::ffi::c_uint;
    let val2: ::core::ffi::c_uint;
    let mut is_index_reg: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int;

    if reg > 0xffff {
        is_index_reg = 1;
    }

    mask = reg & 0xf000;

    if is_index_reg != 0 {
        /* index registers */
        val2 = reg & 0xff;
        reg = reg >> 8;
        nid = reg & 0xff;
        ret = regmap_write((*rt700).sdw_regmap, reg, 0);
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg2, val2);
        if ret < 0 {
            return ret;
        }

        reg3 = RT700_PRIV_DATA_W_H | nid;
        ret = regmap_write((*rt700).sdw_regmap, reg3, ((val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg4 = reg3 + 0x1000;
        reg4 |= 0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg4, (val & 0xff));
        if ret < 0 {
            return ret;
        }
        is_index_reg = 1;
    } else if reg < 0x4fff {
        ret = regmap_write((*rt700).sdw_regmap, reg, val);
        if ret < 0 {
            return ret;
        }
    } else if reg == 0xff01 {
        ret = regmap_write((*rt700).sdw_regmap, reg, val);
        if ret < 0 {
            return ret;
        }
    } else if mask == 0x7000 {
        ret = regmap_write((*rt700).sdw_regmap, reg, ((val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        reg2 = reg + 0x1000;
        reg2 |= 0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg2, (val & 0xff));
        if ret < 0 {
            return ret;
        }
    } else if (reg & 0xff00) == 0x8300 {
        /* for R channel */
        reg2 = reg - 0x1000;
        reg2 &= !0x80;
        ret = regmap_write((*rt700).sdw_regmap, reg2, ((val >> 8) & 0xff));
        if ret < 0 {
            return ret;
        }
        ret = regmap_write((*rt700).sdw_regmap, reg, (val & 0xff));
        if ret < 0 {
            return ret;
        }
    }

    if reg2 == 0 {
        dev_dbg(dev, c"[%s] %04x <= %04x\n".as_ptr(), __func__, reg, val);
    } else if is_index_reg != 0 {
        dev_dbg(
            dev,
            c"[%s] %04x %04x %04x %04x <= %04x %04x\n".as_ptr(),
            __func__,
            reg,
            reg2,
            reg3,
            reg4,
            val2,
            val,
        );
    } else {
        dev_dbg(
            dev,
            c"[%s] %04x %04x <= %04x\n".as_ptr(),
            __func__,
            reg,
            reg2,
            val,
        );
    }

    0
}

static rt700_regmap: regmap_config = regmap_config {
    reg_bits: 24,
    val_bits: 32,
    readable_reg: Some(rt700_readable_register),
    volatile_reg: Some(rt700_volatile_register),
    max_register: 0x755800,
    reg_defaults: rt700_reg_defaults,
    num_reg_defaults: ARRAY_SIZE(rt700_reg_defaults),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
    reg_read: Some(rt700_sdw_read),
    reg_write: Some(rt700_sdw_write),
};

static rt700_sdw_regmap: regmap_config = regmap_config {
    name: c"sdw".as_ptr(),
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt700_readable_register),
    max_register: 0xff01,
    cache_type: REGCACHE_NONE,
    use_single_read: true,
    use_single_write: true,
};

unsafe fn rt700_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> ::core::ffi::c_int {
    let rt700: *mut rt700_priv = dev_get_drvdata(&mut (*slave).dev) as *mut rt700_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt700).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt700).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt700_io_init(&mut (*slave).dev, slave)
}

unsafe fn rt700_read_prop(slave: *mut sdw_slave) -> ::core::ffi::c_int {
    let prop: *mut sdw_slave_prop = &mut (*slave).prop;
    let mut nval: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut bit: u32;
    let mut addr: ::core::ffi::c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    (*prop).scp_int1_mask = SDW_SCP_INT1_IMPL_DEF | SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    (*prop).paging_support = false;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = 0x14; /* BITMAP: 00010100 */
    (*prop).sink_ports = 0xA; /* BITMAP:  00001010 */

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
        if (addr & (1 as ::core::ffi::c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
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

    i = 0;
    dpn = (*prop).sink_dpn_prop;
    addr = (*prop).sink_ports as ::core::ffi::c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as ::core::ffi::c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 20;

    /* wake-up event */
    (*prop).wake_capable = 1;

    0
}

unsafe fn rt700_bus_config(
    slave: *mut sdw_slave,
    params: *mut sdw_bus_params,
) -> ::core::ffi::c_int {
    let rt700: *mut rt700_priv = dev_get_drvdata(&mut (*slave).dev) as *mut rt700_priv;
    let mut ret: ::core::ffi::c_int;

    memcpy(
        &mut (*rt700).params as *mut _ as *mut ::core::ffi::c_void,
        params as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*params),
    );

    ret = rt700_clock_config(&mut (*slave).dev);
    if ret < 0 {
        dev_err(&mut (*slave).dev, c"Invalid clk config".as_ptr());
    }

    ret
}

unsafe fn rt700_interrupt_callback(
    slave: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> ::core::ffi::c_int {
    let rt700: *mut rt700_priv = dev_get_drvdata(&mut (*slave).dev) as *mut rt700_priv;

    dev_dbg(
        &mut (*slave).dev,
        c"%s control_port_stat=%x".as_ptr(),
        __func__,
        (*status).control_port,
    );

    /* C guard(mutex)(&rt700->disable_irq_lock) scoped lock. */
    guard_mutex(&mut (*rt700).disable_irq_lock);
    if ((*status).control_port & 0x4) != 0 && !(*rt700).disable_irq {
        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt700).jack_detect_work,
            msecs_to_jiffies(250),
        );
    }

    0
}

/*
 * slave_ops: callbacks for get_clock_stop_mode, clock_stop and
 * port_prep are not defined for now
 */
static rt700_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt700_read_prop),
    interrupt_callback: Some(rt700_interrupt_callback),
    update_status: Some(rt700_update_status),
    bus_config: Some(rt700_bus_config),
};

unsafe fn rt700_sdw_probe(
    slave: *mut sdw_slave,
    id: *const sdw_device_id,
) -> ::core::ffi::c_int {
    let sdw_regmap: *mut regmap;
    let regmap: *mut regmap;

    /* Regmap Initialization */
    sdw_regmap = devm_regmap_init_sdw(slave, &rt700_sdw_regmap);
    if IS_ERR(sdw_regmap) {
        return PTR_ERR(sdw_regmap);
    }

    regmap = devm_regmap_init(
        &mut (*slave).dev,
        ::core::ptr::null_mut(),
        &mut (*slave).dev as *mut _ as *mut ::core::ffi::c_void,
        &rt700_regmap,
    );
    if IS_ERR(regmap) {
        return PTR_ERR(regmap);
    }

    rt700_init(&mut (*slave).dev, sdw_regmap, regmap, slave)
}

unsafe fn rt700_sdw_remove(slave: *mut sdw_slave) {
    let rt700: *mut rt700_priv = dev_get_drvdata(&mut (*slave).dev) as *mut rt700_priv;

    cancel_delayed_work_sync(&mut (*rt700).jack_detect_work);
    cancel_delayed_work_sync(&mut (*rt700).jack_btn_check_work);

    pm_runtime_disable(&mut (*slave).dev);
}

static rt700_id: [sdw_device_id; 2] = [
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x700, 0x1, 0, 0),
    sdw_device_id {},
];
// MODULE_DEVICE_TABLE(sdw, rt700_id);

unsafe fn rt700_dev_suspend(dev: *mut device) -> ::core::ffi::c_int {
    let rt700: *mut rt700_priv = dev_get_drvdata(dev) as *mut rt700_priv;

    if !(*rt700).hw_init {
        return 0;
    }

    cancel_delayed_work_sync(&mut (*rt700).jack_detect_work);
    cancel_delayed_work_sync(&mut (*rt700).jack_btn_check_work);

    regcache_cache_only((*rt700).regmap, true);

    0
}

unsafe fn rt700_dev_system_suspend(dev: *mut device) -> ::core::ffi::c_int {
    let slave: *mut sdw_slave = dev_to_sdw_dev(dev);
    let rt700: *mut rt700_priv = dev_get_drvdata(dev) as *mut rt700_priv;
    let mut ret: ::core::ffi::c_int;

    if !(*rt700).hw_init {
        return 0;
    }

    /*
     * prevent new interrupts from being handled after the
     * deferred work completes and before the parent disables
     * interrupts on the link
     */
    /* C scoped_guard(mutex, &rt700->disable_irq_lock) block. */
    {
        scoped_guard_mutex(&mut (*rt700).disable_irq_lock);
        (*rt700).disable_irq = true;
        ret = sdw_update_no_pm(slave, SDW_SCP_INTMASK1, SDW_SCP_INT1_IMPL_DEF, 0);
    }

    if ret < 0 {
        /* log but don't prevent suspend from happening */
        dev_dbg(
            &mut (*slave).dev,
            c"%s: could not disable imp-def interrupts\n:".as_ptr(),
            __func__,
        );
    }

    rt700_dev_suspend(dev)
}

const RT700_PROBE_TIMEOUT: ::core::ffi::c_int = 5000;

unsafe fn rt700_dev_resume(dev: *mut device) -> ::core::ffi::c_int {
    let slave: *mut sdw_slave = dev_to_sdw_dev(dev);
    let rt700: *mut rt700_priv = dev_get_drvdata(dev) as *mut rt700_priv;
    let mut ret: ::core::ffi::c_int;

    if !(*rt700).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, RT700_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt700).regmap, false);
    regcache_sync_region((*rt700).regmap, 0x3000, 0x8fff);
    regcache_sync_region((*rt700).regmap, 0x752010, 0x75206b);

    0
}

static rt700_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(rt700_dev_system_suspend, rt700_dev_resume)
    // RUNTIME_PM_OPS(rt700_dev_suspend, rt700_dev_resume, NULL)
    system_suspend: Some(rt700_dev_system_suspend),
    system_resume: Some(rt700_dev_resume),
    runtime_suspend: Some(rt700_dev_suspend),
    runtime_resume: Some(rt700_dev_resume),
    runtime_idle: None,
};

static mut rt700_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: c"rt700".as_ptr(),
        pm: pm_ptr(&rt700_pm),
    },
    probe: Some(rt700_sdw_probe),
    remove: Some(rt700_sdw_remove),
    ops: &rt700_slave_ops,
    id_table: rt700_id.as_ptr(),
};
// module_sdw_driver(rt700_sdw_driver);

// MODULE_DESCRIPTION("ASoC RT700 driver SDW");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
