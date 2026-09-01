// SPDX-License-Identifier: GPL-2.0-only
//
// rt5682.c  --  RT5682 ALSA SoC audio component driver
//
// Copyright 2018 Realtek Semiconductor Corp.
// Author: Bard Liao <bardliao@realtek.com>
//

// C dependencies translated from:
// linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
// linux/pm.h, linux/i2c.h, linux/platform_device.h, linux/spi/spi.h,
// linux/acpi.h, linux/gpio/consumer.h, linux/mutex.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/jack.h,
// sound/soc.h, sound/soc-dapm.h, sound/initval.h, sound/tlv.h,
// sound/rt5682.h, "rl6231.h", and "rt5682.h".

static i2s_default_platform_data: rt5682_platform_data = rt5682_platform_data {
    dmic1_data_pin: RT5682_DMIC1_DATA_GPIO2,
    dmic1_clk_pin: RT5682_DMIC1_CLK_GPIO3,
    jd_src: RT5682_JD1,
    btndet_delay: 16,
    dai_clk_names: {
        let mut dai_clk_names = [core::ptr::null(); RT5682_DAI_NUM_CLKS];
        dai_clk_names[RT5682_DAI_WCLK_IDX] = c"rt5682-dai-wclk".as_ptr();
        dai_clk_names[RT5682_DAI_BCLK_IDX] = c"rt5682-dai-bclk".as_ptr();
        dai_clk_names
    },
    ..unsafe { core::mem::zeroed() }
};

static rt5682_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 16,
    max_register: RT5682_I2C_MODE,
    volatile_reg: Some(rt5682_volatile_register),
    readable_reg: Some(rt5682_readable_register),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: rt5682_reg.as_ptr(),
    num_reg_defaults: RT5682_REG_NUM,
    use_single_read: true,
    use_single_write: true,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn rt5682_jd_check_handler(work: *mut work_struct) {
    let rt5682: *mut rt5682_priv = container_of!(
        work,
        rt5682_priv,
        jd_check_work.work
    );

    if snd_soc_component_read((*rt5682).component, RT5682_AJD1_CTRL) & RT5682_JDH_RS_MASK != 0 {
        /* jack out */
        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt5682).jack_detect_work,
            0,
        );
    } else {
        schedule_delayed_work(&mut (*rt5682).jd_check_work, 500);
    }
}

unsafe extern "C" fn rt5682_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let rt5682: *mut rt5682_priv = data as *mut rt5682_priv;

    mod_delayed_work(
        system_power_efficient_wq,
        &mut (*rt5682).jack_detect_work,
        msecs_to_jiffies((*rt5682).irq_work_delay_time),
    );

    IRQ_HANDLED
}

static mut rt5682_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"rt5682-aif1".as_ptr(),
        id: RT5682_AIF1,
        playback: snd_soc_pcm_stream {
            stream_name: c"AIF1 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: RT5682_STEREO_RATES,
            formats: RT5682_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF1 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: RT5682_STEREO_RATES,
            formats: RT5682_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &rt5682_aif1_dai_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c"rt5682-aif2".as_ptr(),
        id: RT5682_AIF2,
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF2 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: RT5682_STEREO_RATES,
            formats: RT5682_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &rt5682_aif2_dai_ops,
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn rt5682_i2c_disable_regulators(data: *mut c_void) {
    let rt5682: *mut rt5682_priv = data as *mut rt5682_priv;

    regulator_bulk_disable((*rt5682).supplies.len(), (*rt5682).supplies.as_mut_ptr());
}

unsafe extern "C" fn rt5682_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let pdata: *mut rt5682_platform_data =
        dev_get_platdata(&mut (*i2c).dev) as *mut rt5682_platform_data;
    let rt5682: *mut rt5682_priv;
    let mut i: c_int;
    let mut ret: c_int;
    let mut val: c_uint = 0;

    rt5682 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<rt5682_priv>(),
        GFP_KERNEL,
    ) as *mut rt5682_priv;
    if rt5682.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, rt5682 as *mut c_void);

    (*rt5682).i2c_dev = &mut (*i2c).dev;

    (*rt5682).pdata = i2s_default_platform_data;

    if !pdata.is_null() {
        (*rt5682).pdata = *pdata;
    } else {
        rt5682_parse_dt(rt5682, &mut (*i2c).dev);
    }

    (*rt5682).regmap = devm_regmap_init_i2c(i2c, &rt5682_regmap);
    if IS_ERR((*rt5682).regmap as *const c_void) {
        ret = PTR_ERR((*rt5682).regmap as *const c_void) as c_int;
        dev_err(
            &mut (*i2c).dev,
            c"Failed to allocate register map: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    i = 0;
    while (i as usize) < (*rt5682).supplies.len() {
        (*rt5682).supplies[i as usize].supply = rt5682_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        &mut (*i2c).dev,
        (*rt5682).supplies.len(),
        (*rt5682).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            c"Failed to request supplies: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = regulator_bulk_enable((*rt5682).supplies.len(), (*rt5682).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            c"Failed to enable supplies: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = devm_add_action_or_reset(
        &mut (*i2c).dev,
        Some(rt5682_i2c_disable_regulators),
        rt5682 as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    ret = rt5682_get_ldo1(rt5682, &mut (*i2c).dev);
    if ret != 0 {
        return ret;
    }

    /* Sleep for 300 ms minimum */
    usleep_range(300000, 350000);

    regmap_write((*rt5682).regmap, RT5682_I2C_MODE, 0x1);
    usleep_range(10000, 15000);

    regmap_read((*rt5682).regmap, RT5682_DEVICE_ID, &mut val);
    if val != DEVICE_ID {
        dev_err(
            &mut (*i2c).dev,
            c"Device with ID register %x is not rt5682\n".as_ptr(),
            val,
        );
        return -ENODEV;
    }

    regmap_read((*rt5682).regmap, RT5682_INT_DEVICE_ID, &mut val);
    if val == 0x6956 {
        dev_dbg(&mut (*i2c).dev, c"ALC5682I-VE device\n".as_ptr());
        (*rt5682).ve_ic = true;
    }

    mutex_init(&mut (*rt5682).calibrate_mutex);
    rt5682_calibrate(rt5682);

    rt5682_apply_patch_list(rt5682, &mut (*i2c).dev);

    regmap_write((*rt5682).regmap, RT5682_DEPOP_1, 0x0000);

    /* DMIC pin*/
    if (*rt5682).pdata.dmic1_data_pin != RT5682_DMIC1_NULL {
        match (*rt5682).pdata.dmic1_data_pin {
            RT5682_DMIC1_DATA_GPIO2 => {
                /* share with LRCK2 */
                regmap_update_bits(
                    (*rt5682).regmap,
                    RT5682_DMIC_CTRL_1,
                    RT5682_DMIC_1_DP_MASK,
                    RT5682_DMIC_1_DP_GPIO2,
                );
                regmap_update_bits(
                    (*rt5682).regmap,
                    RT5682_GPIO_CTRL_1,
                    RT5682_GP2_PIN_MASK,
                    RT5682_GP2_PIN_DMIC_SDA,
                );
            }
            RT5682_DMIC1_DATA_GPIO5 => {
                /* share with DACDAT1 */
                regmap_update_bits(
                    (*rt5682).regmap,
                    RT5682_DMIC_CTRL_1,
                    RT5682_DMIC_1_DP_MASK,
                    RT5682_DMIC_1_DP_GPIO5,
                );
                regmap_update_bits(
                    (*rt5682).regmap,
                    RT5682_GPIO_CTRL_1,
                    RT5682_GP5_PIN_MASK,
                    RT5682_GP5_PIN_DMIC_SDA,
                );
            }
            _ => {
                dev_warn(&mut (*i2c).dev, c"invalid DMIC_DAT pin\n".as_ptr());
            }
        }

        match (*rt5682).pdata.dmic1_clk_pin {
            RT5682_DMIC1_CLK_GPIO1 => {
                /* share with IRQ */
                regmap_update_bits(
                    (*rt5682).regmap,
                    RT5682_GPIO_CTRL_1,
                    RT5682_GP1_PIN_MASK,
                    RT5682_GP1_PIN_DMIC_CLK,
                );
            }
            RT5682_DMIC1_CLK_GPIO3 => {
                /* share with BCLK2 */
                regmap_update_bits(
                    (*rt5682).regmap,
                    RT5682_GPIO_CTRL_1,
                    RT5682_GP3_PIN_MASK,
                    RT5682_GP3_PIN_DMIC_CLK,
                );
                if (*rt5682).pdata.dmic_clk_driving_high {
                    regmap_update_bits(
                        (*rt5682).regmap,
                        RT5682_PAD_DRIVING_CTRL,
                        RT5682_PAD_DRV_GP3_MASK,
                        2 << RT5682_PAD_DRV_GP3_SFT,
                    );
                }
            }
            _ => {
                dev_warn(&mut (*i2c).dev, c"invalid DMIC_CLK pin\n".as_ptr());
            }
        }
    }

    regmap_update_bits(
        (*rt5682).regmap,
        RT5682_PWR_ANLG_1,
        RT5682_LDO1_DVO_MASK | RT5682_HP_DRIVER_MASK,
        RT5682_LDO1_DVO_12 | RT5682_HP_DRIVER_5X,
    );
    regmap_write((*rt5682).regmap, RT5682_MICBIAS_2, 0x0080);
    regmap_update_bits(
        (*rt5682).regmap,
        RT5682_GPIO_CTRL_1,
        RT5682_GP4_PIN_MASK | RT5682_GP5_PIN_MASK,
        RT5682_GP4_PIN_ADCDAT1 | RT5682_GP5_PIN_DACDAT1,
    );
    regmap_write((*rt5682).regmap, RT5682_TEST_MODE_CTRL_1, 0x0000);
    regmap_update_bits(
        (*rt5682).regmap,
        RT5682_BIAS_CUR_CTRL_8,
        RT5682_HPA_CP_BIAS_CTRL_MASK,
        RT5682_HPA_CP_BIAS_3UA,
    );
    regmap_update_bits(
        (*rt5682).regmap,
        RT5682_CHARGE_PUMP_1,
        RT5682_CP_CLK_HP_MASK,
        RT5682_CP_CLK_HP_300KHZ,
    );
    regmap_update_bits(
        (*rt5682).regmap,
        RT5682_HP_CHARGE_PUMP_1,
        RT5682_PM_HP_MASK,
        RT5682_PM_HP_HV,
    );
    regmap_update_bits(
        (*rt5682).regmap,
        RT5682_DMIC_CTRL_1,
        RT5682_FIFO_CLK_DIV_MASK,
        RT5682_FIFO_CLK_DIV_2,
    );

    INIT_DELAYED_WORK(
        &mut (*rt5682).jack_detect_work,
        Some(rt5682_jack_detect_handler),
    );
    INIT_DELAYED_WORK(
        &mut (*rt5682).jd_check_work,
        Some(rt5682_jd_check_handler),
    );

    if (*i2c).irq != 0 {
        ret = devm_request_threaded_irq(
            &mut (*i2c).dev,
            (*i2c).irq,
            None,
            Some(rt5682_irq),
            IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
            c"rt5682".as_ptr(),
            rt5682 as *mut c_void,
        );
        if ret == 0 {
            (*rt5682).irq = (*i2c).irq;
        } else {
            dev_err(
                &mut (*i2c).dev,
                c"Failed to request IRQ: %d\n".as_ptr(),
                ret,
            );
        }
    }

    // #ifdef CONFIG_COMMON_CLK
    // Check if MCLK provided
    #[cfg(CONFIG_COMMON_CLK)]
    {
        (*rt5682).mclk = devm_clk_get_optional(&mut (*i2c).dev, c"mclk".as_ptr());
        if IS_ERR((*rt5682).mclk as *const c_void) {
            return PTR_ERR((*rt5682).mclk as *const c_void) as c_int;
        }

        // Register CCF DAI clock control
        ret = rt5682_register_dai_clks(rt5682);
        if ret != 0 {
            return ret;
        }

        // Initial setup for CCF
        (*rt5682).lrck[RT5682_AIF1 as usize] = 48000;
    }

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &rt5682_soc_component_dev,
        rt5682_dai.as_mut_ptr(),
        rt5682_dai.len(),
    )
}

unsafe extern "C" fn rt5682_i2c_shutdown(client: *mut i2c_client) {
    let rt5682: *mut rt5682_priv = i2c_get_clientdata(client) as *mut rt5682_priv;

    disable_irq((*client).irq);
    cancel_delayed_work_sync(&mut (*rt5682).jack_detect_work);
    cancel_delayed_work_sync(&mut (*rt5682).jd_check_work);

    rt5682_reset(rt5682);
}

unsafe extern "C" fn rt5682_i2c_remove(client: *mut i2c_client) {
    rt5682_i2c_shutdown(client);
}

static rt5682_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"realtek,rt5682i".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
module_device_table!(of, rt5682_of_match);

static rt5682_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: *b"10EC5682\0",
        ..unsafe { core::mem::zeroed() }
    },
    acpi_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
module_device_table!(acpi, rt5682_acpi_match);

static rt5682_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"rt5682\0",
        ..unsafe { core::mem::zeroed() }
    },
    i2c_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
module_device_table!(i2c, rt5682_i2c_id);

static mut rt5682_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"rt5682".as_ptr(),
        of_match_table: rt5682_of_match.as_ptr(),
        acpi_match_table: rt5682_acpi_match.as_ptr(),
        probe_type: PROBE_PREFER_ASYNCHRONOUS,
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(rt5682_i2c_probe),
    remove: Some(rt5682_i2c_remove),
    shutdown: Some(rt5682_i2c_shutdown),
    id_table: rt5682_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};
module_i2c_driver!(rt5682_i2c_driver);

module_description!("ASoC RT5682 driver");
module_author!("Bard Liao <bardliao@realtek.com>");
module_license!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
