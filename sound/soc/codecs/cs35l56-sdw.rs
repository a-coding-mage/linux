// SPDX-License-Identifier: GPL-2.0-only
//
// CS35L56 ALSA SoC audio driver SoundWire binding
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// Rust translation of dependencies originally included from Linux kernel headers
// and "cs35l56.h"; their definitions are supplied by the surrounding tree.

const CS35L56_SDW_ADDR_OFFSET: u32 = 0x8000;

/* Cirrus bus bridge registers */
const CS35L56_SDW_MEM_ACCESS_STATUS: u32 = 0xd0;
const CS35L56_SDW_MEM_READ_DATA: u32 = 0xd8;

const CS35L56_SDW_LAST_LATE: u32 = BIT(3);
const CS35L56_SDW_CMD_IN_PROGRESS: u32 = BIT(2);
const CS35L56_SDW_RDATA_RDY: u32 = BIT(0);

const CS35L56_LATE_READ_POLL_US: u32 = 10;
const CS35L56_LATE_READ_TIMEOUT_US: u32 = 1000;

unsafe fn cs35l56_sdw_poll_mem_status(
    peripheral: *mut sdw_slave,
    mask: c_uint,
    match_: c_uint,
) -> c_int {
    let mut ret: c_int;
    let mut val: c_int = 0;

    ret = read_poll_timeout(
        sdw_read_no_pm,
        &mut val,
        |val| (val < 0) || (((val as c_uint) & mask) == match_),
        CS35L56_LATE_READ_POLL_US,
        CS35L56_LATE_READ_TIMEOUT_US,
        false,
        peripheral,
        CS35L56_SDW_MEM_ACCESS_STATUS,
    );
    if ret < 0 {
        return ret;
    }

    if val < 0 {
        return val;
    }

    0
}

unsafe fn cs35l56_sdw_slow_read(
    peripheral: *mut sdw_slave,
    reg: c_uint,
    buf: *mut u8,
    val_size: size_t,
) -> c_int {
    let mut ret: c_int;
    let mut i: size_t = 0;

    while i < val_size {
        /* Poll for bus bridge idle */
        ret = cs35l56_sdw_poll_mem_status(peripheral, CS35L56_SDW_CMD_IN_PROGRESS, 0);
        if ret < 0 {
            dev_err(
                &mut (*peripheral).dev,
                c_str!("!CMD_IN_PROGRESS fail: %d\n"),
                ret,
            );
            return ret;
        }

        /* Reading LSByte triggers read of register to holding buffer */
        sdw_read_no_pm(peripheral, reg.wrapping_add(i as c_uint));

        /* Wait for data available */
        ret = cs35l56_sdw_poll_mem_status(
            peripheral,
            CS35L56_SDW_RDATA_RDY,
            CS35L56_SDW_RDATA_RDY,
        );
        if ret < 0 {
            dev_err(&mut (*peripheral).dev, c_str!("RDATA_RDY fail: %d\n"), ret);
            return ret;
        }

        /* Read data from buffer */
        ret = sdw_nread_no_pm(
            peripheral,
            CS35L56_SDW_MEM_READ_DATA,
            core::mem::size_of::<u32>(),
            buf.add(i) as *mut c_void,
        );
        if ret != 0 {
            dev_err(
                &mut (*peripheral).dev,
                c_str!("Late read @%#x failed: %d\n"),
                reg.wrapping_add(i as c_uint),
                ret,
            );
            return ret;
        }

        swab32s(buf.add(i) as *mut u32);
        i += core::mem::size_of::<u32>();
    }

    0
}

unsafe fn cs35l56_sdw_read(
    context: *mut c_void,
    reg_buf: *const c_void,
    reg_size: size_t,
    val_buf: *mut c_void,
    val_size: size_t,
) -> c_int {
    let peripheral: *mut sdw_slave = context as *mut sdw_slave;
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(&mut (*peripheral).dev) as *mut cs35l56_private;
    let reg_addr: c_uint = get_unaligned_le32(reg_buf);
    let mut ret: c_int;

    if cs35l56_is_otp_register(reg_addr.wrapping_sub(CS35L56_SDW_ADDR_OFFSET)) {
        return cs35l56_sdw_slow_read(peripheral, reg_addr, val_buf as *mut u8, val_size);
    }

    ret = regmap_raw_read((*cs35l56).sdw_bus_regmap, reg_addr, val_buf, val_size);
    if ret != 0 {
        return ret;
    }

    swab32_array(val_buf as *mut u32, val_size / core::mem::size_of::<u32>());

    0
}

#[inline]
unsafe fn cs35l56_swab_copy(dest: *mut c_void, src: *const c_void, mut nbytes: size_t) {
    let mut dest32: *mut u32 = dest as *mut u32;
    let mut src32: *const u32 = src as *const u32;

    while nbytes > 0 {
        *dest32 = swab32(*src32);
        dest32 = dest32.add(1);
        src32 = src32.add(1);
        nbytes -= 4;
    }
}

unsafe fn cs35l56_sdw_gather_write(
    context: *mut c_void,
    reg_buf: *const c_void,
    reg_size: size_t,
    mut val_buf: *const c_void,
    mut val_size: size_t,
) -> c_int {
    let peripheral: *mut sdw_slave = context as *mut sdw_slave;
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(&mut (*peripheral).dev) as *mut cs35l56_private;
    let mut reg_addr: c_uint = get_unaligned_le32(reg_buf);
    let mut swab_buf: [u32; 64] = [0; 64]; /* Define u32 so it is 32-bit aligned */
    let mut ret: c_int;

    while val_size > core::mem::size_of_val(&swab_buf) {
        cs35l56_swab_copy(
            swab_buf.as_mut_ptr() as *mut c_void,
            val_buf,
            core::mem::size_of_val(&swab_buf),
        );
        ret = regmap_raw_write(
            (*cs35l56).sdw_bus_regmap,
            reg_addr,
            swab_buf.as_ptr() as *const c_void,
            core::mem::size_of_val(&swab_buf),
        );
        if ret != 0 {
            return ret;
        }

        val_size -= core::mem::size_of_val(&swab_buf);
        reg_addr = reg_addr.wrapping_add(core::mem::size_of_val(&swab_buf) as c_uint);
        val_buf = (val_buf as *const u8).add(core::mem::size_of_val(&swab_buf)) as *const c_void;
    }

    if val_size == 0 {
        return 0;
    }

    cs35l56_swab_copy(swab_buf.as_mut_ptr() as *mut c_void, val_buf, val_size);

    regmap_raw_write(
        (*cs35l56).sdw_bus_regmap,
        reg_addr,
        swab_buf.as_ptr() as *const c_void,
        val_size,
    )
}

unsafe fn cs35l56_sdw_write(
    context: *mut c_void,
    val_buf: *const c_void,
    val_size: size_t,
) -> c_int {
    let src_buf: *const u8 = val_buf as *const u8;

    /* First word of val_buf contains the destination address */
    cs35l56_sdw_gather_write(
        context,
        src_buf.add(0) as *const c_void,
        4,
        src_buf.add(4) as *const c_void,
        val_size - 4,
    )
}

/*
 * Registers are big-endian on I2C and SPI but little-endian on SoundWire.
 * Exported firmware controls are big-endian on I2C/SPI but little-endian on
 * SoundWire. Firmware files are always big-endian and are opaque blobs.
 * Present a big-endian regmap and hide the endianness swap, so that the ALSA
 * byte controls always have the same byte order, and firmware file blobs
 * can be written verbatim.
 */
static cs35l56_regmap_swab_bus_sdw: regmap_bus = regmap_bus {
    read: Some(cs35l56_sdw_read),
    write: Some(cs35l56_sdw_write),
    gather_write: Some(cs35l56_sdw_gather_write),
    reg_format_endian_default: REGMAP_ENDIAN_LITTLE,
    val_format_endian_default: REGMAP_ENDIAN_BIG,
};

/* Low-level SoundWire regmap to transfer the data over the bus */
static cs35l56_sdw_bus_regmap: regmap_config = regmap_config {
    name: c_str!("sdw-le32"),
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    reg_format_endian: REGMAP_ENDIAN_LITTLE,
    val_format_endian: REGMAP_ENDIAN_LITTLE,
    max_register: CS35L56_DSP1_PMEM_5114 + 0x8000,
    cache_type: REGCACHE_NONE,
};

unsafe fn cs35l56_sdw_get_unique_id(cs35l56: *mut cs35l56_private) -> c_int {
    let mut ret: c_int;

    ret = sdw_read_no_pm((*cs35l56).sdw_peripheral, SDW_SCP_DEVID_0);
    if ret < 0 {
        return ret;
    }

    (*cs35l56).sdw_unique_id = ret & 0xf;

    0
}

unsafe fn cs35l56_sdw_init(peripheral: *mut sdw_slave) {
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(&mut (*peripheral).dev) as *mut cs35l56_private;
    let mut ret: c_int;

    pm_runtime_get_noresume((*cs35l56).base.dev);

    ret = cs35l56_sdw_get_unique_id(cs35l56);
    if ret != 0 {
        goto_out(cs35l56);
        return;
    }

    /* SoundWire UniqueId is used to index the calibration array */
    if (*cs35l56).base.cal_index < 0 {
        (*cs35l56).base.cal_index = (*cs35l56).sdw_unique_id;
    }

    ret = cs35l56_init(cs35l56);
    if ret < 0 {
        regcache_cache_only((*cs35l56).base.regmap, true);
        goto_out(cs35l56);
        return;
    }

    /*
     * cs35l56_init can return with !init_done if it triggered
     * a soft reset.
     */
    if (*cs35l56).base.init_done {
        cs35l56_unmask_soundwire_interrupts(cs35l56);
    }

    goto_out(cs35l56);

    unsafe fn goto_out(cs35l56: *mut cs35l56_private) {
        pm_runtime_put_autosuspend((*cs35l56).base.dev);
    }
}

unsafe fn cs35l56_sdw_interrupt(
    peripheral: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> c_int {
    /*
     * The IRQ itself was handled through the regmap_irq handler, this is
     * just clearing up the additional Cirrus SoundWire registers that are
     * not covered by the SoundWire framework or the IRQ handler itself.
     */
    sdw_read_no_pm(peripheral, CS35L56_SDW_GEN_INT_STAT_1);
    sdw_write_no_pm(peripheral, CS35L56_SDW_GEN_INT_STAT_1, 0xFF);

    0
}

unsafe fn cs35l56_sdw_read_prop(peripheral: *mut sdw_slave) -> c_int {
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(&mut (*peripheral).dev) as *mut cs35l56_private;
    let prop: *mut sdw_slave_prop = &mut (*peripheral).prop;
    let mut ports: *mut sdw_dpn_prop;
    let mut clock_stop_1: u8 = false as u8;
    let mut ret: c_int;

    ret = fwnode_property_read_u8(
        dev_fwnode((*cs35l56).base.dev),
        c_str!("mipi-sdw-clock-stop-mode1-supported"),
        &mut clock_stop_1,
    );
    if ret == 0 {
        (*prop).clk_stop_mode1 = clock_stop_1 != 0;
    }

    ports = devm_kcalloc(
        (*cs35l56).base.dev,
        2,
        core::mem::size_of::<sdw_dpn_prop>(),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if ports.is_null() {
        return -ENOMEM;
    }

    (*prop).source_ports = BIT(CS35L56_SDW1_CAPTURE_PORT);
    (*prop).sink_ports = BIT(CS35L56_SDW1_PLAYBACK_PORT);
    (*prop).paging_support = true;
    (*prop).use_domain_irq = true;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;
    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY | SDW_SCP_INT1_IMPL_DEF;

    /* DP1 - playback */
    (*ports.add(0)).num = CS35L56_SDW1_PLAYBACK_PORT;
    (*ports.add(0)).type_ = SDW_DPN_FULL;
    (*ports.add(0)).ch_prep_timeout = 10;
    (*prop).sink_dpn_prop = ports.add(0);

    /* DP3 - capture */
    (*ports.add(1)).num = CS35L56_SDW1_CAPTURE_PORT;
    (*ports.add(1)).type_ = SDW_DPN_FULL;
    (*ports.add(1)).ch_prep_timeout = 10;
    (*prop).src_dpn_prop = ports.add(1);

    dev_dbg(
        &mut (*peripheral).dev,
        c_str!("clock stop mode 1 supported: %s\n"),
        str_yes_no((*prop).clk_stop_mode1),
    );

    0
}

unsafe fn cs35l56_sdw_update_status(
    peripheral: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(&mut (*peripheral).dev) as *mut cs35l56_private;

    match status {
        SDW_SLAVE_ATTACHED => {
            if (*cs35l56).sdw_attached {
                return 0;
            }

            dev_dbg((*cs35l56).base.dev, c_str!("%s: ATTACHED\n"), c_str!("cs35l56_sdw_update_status"));
            if !(*cs35l56).base.init_done || (*cs35l56).soft_resetting {
                cs35l56_sdw_init(peripheral);
            }

            (*cs35l56).sdw_attached = true;
        }
        SDW_SLAVE_UNATTACHED => {
            if (*cs35l56).sdw_attached {
                dev_dbg((*cs35l56).base.dev, c_str!("%s: UNATTACHED\n"), c_str!("cs35l56_sdw_update_status"));
            }
            (*cs35l56).sdw_attached = false;
        }
        _ => {}
    }

    0
}

static cs35l56_sdw_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(cs35l56_sdw_read_prop),
    interrupt_callback: Some(cs35l56_sdw_interrupt),
    update_status: Some(cs35l56_sdw_update_status),
};

// __maybe_unused in C.
unsafe fn cs35l56_sdw_handle_unattach(cs35l56: *mut cs35l56_private) -> c_int {
    let peripheral: *mut sdw_slave = (*cs35l56).sdw_peripheral;
    let mut ret: c_int;

    dev_dbg(
        (*cs35l56).base.dev,
        c_str!("attached:%u unattach_request:%u\n"),
        (*cs35l56).sdw_attached,
        (*peripheral).unattach_request,
    );

    /* Cannot access registers until bus is re-initialized. */
    ret = sdw_slave_wait_for_init(peripheral, 5000);
    if ret != 0 {
        return ret;
    }

    /*
     * Don't call regcache_mark_dirty(), we can't be sure that the
     * Manager really did issue a Bus Reset.
     */

    0
}

// __maybe_unused in C.
unsafe fn cs35l56_sdw_runtime_suspend(dev: *mut device) -> c_int {
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(dev) as *mut cs35l56_private;

    if !(*cs35l56).base.init_done {
        return 0;
    }

    cs35l56_runtime_suspend_common(&mut (*cs35l56).base)
}

// __maybe_unused in C.
unsafe fn cs35l56_sdw_runtime_resume(dev: *mut device) -> c_int {
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(dev) as *mut cs35l56_private;
    let mut ret: c_int;

    dev_dbg(dev, c_str!("Runtime resume\n"));

    if !(*cs35l56).base.init_done {
        return 0;
    }

    ret = cs35l56_sdw_handle_unattach(cs35l56);
    if ret < 0 {
        return ret;
    }

    ret = cs35l56_runtime_resume_common(&mut (*cs35l56).base, true);
    if ret != 0 {
        return ret;
    }

    cs35l56_unmask_soundwire_interrupts(cs35l56);

    0
}

// __maybe_unused in C.
unsafe fn cs35l56_sdw_system_suspend(dev: *mut device) -> c_int {
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(dev) as *mut cs35l56_private;

    if !(*cs35l56).base.init_done {
        return 0;
    }

    /* runtime_resume unmasks the interrupt */
    cs35l56_mask_soundwire_interrupts(cs35l56);

    cs35l56_system_suspend(dev)
}

unsafe fn cs35l56_sdw_probe(
    peripheral: *mut sdw_slave,
    id: *const sdw_device_id,
) -> c_int {
    let dev: *mut device = &mut (*peripheral).dev;
    let mut cs35l56: *mut cs35l56_private;
    let regmap_config: *const regmap_config;
    let mut ret: c_int;

    cs35l56 = devm_kzalloc(dev, core::mem::size_of::<cs35l56_private>(), GFP_KERNEL) as *mut cs35l56_private;
    if cs35l56.is_null() {
        return -ENOMEM;
    }

    (*cs35l56).base.dev = dev;
    (*cs35l56).sdw_peripheral = peripheral;
    (*cs35l56).sdw_link_num = (*(*peripheral).bus).link_id;

    dev_set_drvdata(dev, cs35l56 as *mut c_void);

    match (*id).driver_data as c_uint {
        0x3556 | 0x3557 => {
            regmap_config = &cs35l56_regmap_sdw;
        }
        0x3563 | 0x3562 => {
            regmap_config = &cs35l63_regmap_sdw;
        }
        _ => {
            return -ENODEV;
        }
    }

    (*cs35l56).base.type_ = ((*id).driver_data as c_uint) & 0xff;

    /* Low-level regmap to transfer read/writes over SoundWire bus */
    (*cs35l56).sdw_bus_regmap = devm_regmap_init_sdw(peripheral, &cs35l56_sdw_bus_regmap);
    if IS_ERR((*cs35l56).sdw_bus_regmap) {
        ret = PTR_ERR((*cs35l56).sdw_bus_regmap);
        return dev_err_probe(dev, ret, c_str!("Failed to allocate bus register map\n"));
    }

    /* Wrapper regmap to simulate big-endian ordering */
    (*cs35l56).base.regmap = devm_regmap_init(
        dev,
        &cs35l56_regmap_swab_bus_sdw,
        peripheral as *mut c_void,
        regmap_config,
    );
    if IS_ERR((*cs35l56).base.regmap) {
        ret = PTR_ERR((*cs35l56).base.regmap);
        return dev_err_probe(dev, ret, c_str!("Failed to allocate register map\n"));
    }

    /* Start in cache-only until device is enumerated */
    regcache_cache_only((*cs35l56).base.regmap, true);

    cs35l56_common_probe(cs35l56, (*peripheral).irq)
}

unsafe fn cs35l56_sdw_remove(peripheral: *mut sdw_slave) {
    let cs35l56: *mut cs35l56_private = dev_get_drvdata(&mut (*peripheral).dev) as *mut cs35l56_private;

    cs35l56_mask_soundwire_interrupts(cs35l56);

    cs35l56_remove(cs35l56);
}

static cs35l56_sdw_pm: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(cs35l56_sdw_runtime_suspend),
    runtime_resume: Some(cs35l56_sdw_runtime_resume),
    runtime_idle: None,
    system_suspend: Some(cs35l56_sdw_system_suspend),
    system_resume: Some(cs35l56_system_resume),
    system_suspend_late: Some(cs35l56_system_suspend_late),
    system_resume_early: Some(cs35l56_system_resume_early),
    /* NOIRQ stage not needed, SoundWire doesn't use a hard IRQ */
};

static cs35l56_sdw_id: [sdw_device_id; 5] = [
    SDW_SLAVE_ENTRY(0x01FA, 0x3556, 0x3556),
    SDW_SLAVE_ENTRY(0x01FA, 0x3557, 0x3557),
    SDW_SLAVE_ENTRY(0x01FA, 0x3562, 0x3562),
    SDW_SLAVE_ENTRY(0x01FA, 0x3563, 0x3563),
    sdw_device_id::default(),
];
MODULE_DEVICE_TABLE(sdw, cs35l56_sdw_id);

static mut cs35l56_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: c_str!("cs35l56"),
        pm: pm_ptr(&cs35l56_sdw_pm),
    },
    probe: Some(cs35l56_sdw_probe),
    remove: Some(cs35l56_sdw_remove),
    ops: &cs35l56_sdw_ops,
    id_table: cs35l56_sdw_id.as_ptr(),
};

module_sdw_driver!(cs35l56_sdw_driver);

MODULE_DESCRIPTION!("ASoC CS35L56 SoundWire driver");
MODULE_IMPORT_NS!("SND_SOC_CS35L56_CORE");
MODULE_IMPORT_NS!("SND_SOC_CS35L56_SHARED");
MODULE_AUTHOR!("Richard Fitzgerald <rf@opensource.cirrus.com>");
MODULE_AUTHOR!("Simon Trimmer <simont@opensource.cirrus.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
