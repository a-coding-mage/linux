// SPDX-License-Identifier: GPL-2.0-only

// Linux kernel and sibling-module dependencies are supplied externally.

const ZL_RANGE_OFFSET: u32 = 0x80;
const ZL_PAGE_SIZE: u32 = 0x80;
const ZL_NUM_PAGES: u32 = 256;
const ZL_PAGE_SEL: u32 = 0x7f;
const ZL_PAGE_SEL_MASK: u32 = genmask(7, 0);
const ZL_NUM_REGS: u32 = ZL_NUM_PAGES * ZL_PAGE_SIZE;

static ZL3073X_CHIP_IDS: [zl3073x_chip_info; 19] = [
    zl3073x_chip_info { id: 0x0e30, num_channels: 2, flags: ZL3073X_FLAG_REF_PHASE_COMP_32 },
    zl3073x_chip_info { id: 0x0e3b, num_channels: 3, flags: ZL3073X_FLAG_REF_PHASE_COMP_32 },
    zl3073x_chip_info { id: 0x0e93, num_channels: 1, flags: ZL3073X_FLAG_REF_PHASE_COMP_32 },
    zl3073x_chip_info { id: 0x0e94, num_channels: 2, flags: ZL3073X_FLAG_REF_PHASE_COMP_32 },
    zl3073x_chip_info { id: 0x0e95, num_channels: 3, flags: ZL3073X_FLAG_REF_PHASE_COMP_32 },
    zl3073x_chip_info { id: 0x0e96, num_channels: 4, flags: ZL3073X_FLAG_REF_PHASE_COMP_32 },
    zl3073x_chip_info { id: 0x0e97, num_channels: 5, flags: ZL3073X_FLAG_REF_PHASE_COMP_32 },
    zl3073x_chip_info { id: 0x1e93, num_channels: 1, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x1e94, num_channels: 2, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x1e95, num_channels: 3, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x1e96, num_channels: 4, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x1e97, num_channels: 5, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x1f60, num_channels: 2, flags: ZL3073X_FLAG_REF_PHASE_COMP_32 },
    zl3073x_chip_info { id: 0x2e93, num_channels: 1, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x2e94, num_channels: 2, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x2e95, num_channels: 3, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x2e96, num_channels: 4, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x2e97, num_channels: 5, flags: ZL3073X_FLAG_DIE_TEMP },
    zl3073x_chip_info { id: 0x3fc4, num_channels: 2, flags: ZL3073X_FLAG_DIE_TEMP },
];

// Regmap range configuration.
static ZL3073X_REGMAP_RANGE: regmap_range_cfg = regmap_range_cfg {
    range_min: ZL_RANGE_OFFSET,
    range_max: ZL_RANGE_OFFSET + ZL_NUM_REGS - 1,
    selector_reg: ZL_PAGE_SEL,
    selector_mask: ZL_PAGE_SEL_MASK,
    selector_shift: 0,
    window_start: 0,
    window_len: ZL_PAGE_SIZE,
};

unsafe extern "C" fn zl3073x_is_volatile_reg(_dev: *mut device, reg: u32) -> bool {
    reg != ZL_PAGE_SEL
}

static mut ZL3073X_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: ZL_RANGE_OFFSET + ZL_NUM_REGS - 1,
    ranges: unsafe { &raw mut ZL3073X_REGMAP_RANGE },
    num_ranges: 1,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(zl3073x_is_volatile_reg),
};

pub static mut zl3073x_regmap_config: regmap_config = ZL3073X_REGMAP_CONFIG;

unsafe fn zl3073x_check_reg(zldev: *mut zl3073x_dev, reg: u32, size: usize) -> bool {
    if ZL_REG_PAGE(reg) >= 10 && ZL_REG_PAGE(reg) < 255 {
        lockdep_assert_held(&(*zldev).multiop_lock);
    }
    if ZL_REG_OFFSET(reg) > ZL_REG_MAX_OFFSET(reg) {
        dev_err((*zldev).dev, "Index out of range for reg 0x%04lx\\n", ZL_REG_ADDR(reg));
        return false;
    }
    if ZL_REG_SIZE(reg) != size {
        dev_err((*zldev).dev, "Invalid size %zu for reg 0x%04lx\\n", size, ZL_REG_ADDR(reg));
        return false;
    }
    true
}

unsafe fn zl3073x_read_reg(zldev: *mut zl3073x_dev, mut reg: u32, val: *mut core::ffi::c_void, size: usize) -> i32 {
    if !zl3073x_check_reg(zldev, reg, size) { return -EINVAL; }
    reg = ZL_REG_ADDR(reg) + ZL_RANGE_OFFSET;
    let rc = regmap_bulk_read((*zldev).regmap, reg, val, size);
    if rc != 0 {
        dev_err((*zldev).dev, "Failed to read reg 0x%04x: %pe\\n", reg, ERR_PTR(rc));
        return rc;
    }
    0
}

unsafe fn zl3073x_write_reg(zldev: *mut zl3073x_dev, mut reg: u32, val: *const core::ffi::c_void, size: usize) -> i32 {
    if !zl3073x_check_reg(zldev, reg, size) { return -EINVAL; }
    reg = ZL_REG_ADDR(reg) + ZL_RANGE_OFFSET;
    let rc = regmap_bulk_write((*zldev).regmap, reg, val, size);
    if rc != 0 {
        dev_err((*zldev).dev, "Failed to write reg 0x%04x: %pe\\n", reg, ERR_PTR(rc));
        return rc;
    }
    0
}

pub unsafe fn zl3073x_read_u8(zldev: *mut zl3073x_dev, reg: u32, val: *mut u8) -> i32 { zl3073x_read_reg(zldev, reg, val.cast(), core::mem::size_of::<u8>()) }
pub unsafe fn zl3073x_write_u8(zldev: *mut zl3073x_dev, reg: u32, val: u8) -> i32 { zl3073x_write_reg(zldev, reg, &val as *const u8 as *const _, 1) }

pub unsafe fn zl3073x_read_u16(zldev: *mut zl3073x_dev, reg: u32, val: *mut u16) -> i32 {
    let rc = zl3073x_read_reg(zldev, reg, val.cast(), 2);
    if rc == 0 { *val = u16::from_be(*val); }
    rc
}
pub unsafe fn zl3073x_write_u16(zldev: *mut zl3073x_dev, reg: u32, mut val: u16) -> i32 { val = val.to_be(); zl3073x_write_reg(zldev, reg, &val as *const u16 as *const _, 2) }
pub unsafe fn zl3073x_read_u32(zldev: *mut zl3073x_dev, reg: u32, val: *mut u32) -> i32 {
    let rc = zl3073x_read_reg(zldev, reg, val.cast(), 4);
    if rc == 0 { *val = u32::from_be(*val); }
    rc
}
pub unsafe fn zl3073x_write_u32(zldev: *mut zl3073x_dev, reg: u32, mut val: u32) -> i32 { val = val.to_be(); zl3073x_write_reg(zldev, reg, &val as *const u32 as *const _, 4) }

pub unsafe fn zl3073x_read_u48(zldev: *mut zl3073x_dev, reg: u32, val: *mut u64) -> i32 {
    let mut buf = [0u8; 6];
    let rc = zl3073x_read_reg(zldev, reg, buf.as_mut_ptr().cast(), 6);
    if rc == 0 { *val = ((buf[0] as u64) << 40) | ((buf[1] as u64) << 32) | ((buf[2] as u64) << 24) | ((buf[3] as u64) << 16) | ((buf[4] as u64) << 8) | buf[5] as u64; }
    rc
}
pub unsafe fn zl3073x_write_u48(zldev: *mut zl3073x_dev, reg: u32, val: u64) -> i32 {
    if val > genmask_ull(47, 0) && val < genmask_ull(63, 47) { dev_err((*zldev).dev, "Value 0x%0llx out of range\\n", val); return -EINVAL; }
    let buf = [(val >> 40) as u8, (val >> 32) as u8, (val >> 24) as u8, (val >> 16) as u8, (val >> 8) as u8, val as u8];
    zl3073x_write_reg(zldev, reg, buf.as_ptr().cast(), 6)
}

pub unsafe fn zl3073x_poll_zero_u8(zldev: *mut zl3073x_dev, mut reg: u32, mask: u8, timeout_us: u32) -> i32 {
    let sleep_us = timeout_us / 50;
    if ZL_REG_SIZE(reg) != 1 { dev_err((*zldev).dev, "Invalid reg 0x%04lx size for polling\\n", ZL_REG_ADDR(reg)); return -EINVAL; }
    reg = ZL_REG_ADDR(reg) + ZL_RANGE_OFFSET;
    regmap_read_poll_timeout((*zldev).regmap, reg, mask, sleep_us, timeout_us)
}

pub unsafe fn zl3073x_mb_op(zldev: *mut zl3073x_dev, op_reg: u32, op_val: u8, mask_reg: u32, mask_val: u16) -> i32 {
    let mut rc = zl3073x_write_u16(zldev, mask_reg, mask_val); if rc != 0 { return rc; }
    rc = zl3073x_write_u8(zldev, op_reg, op_val); if rc != 0 { return rc; }
    zl3073x_poll_zero_u8(zldev, op_reg, op_val, ZL_POLL_MB_TIMEOUT_US)
}

unsafe fn zl3073x_do_hwreg_op(zldev: *mut zl3073x_dev, op: u8) -> i32 {
    let rc = zl3073x_write_u8(zldev, ZL_REG_HWREG_OP, op | ZL_HWREG_OP_PENDING); if rc != 0 { return rc; }
    zl3073x_poll_zero_u8(zldev, ZL_REG_HWREG_OP, ZL_HWREG_OP_PENDING, ZL_POLL_HWREG_TIMEOUT_US)
}
pub unsafe fn zl3073x_read_hwreg(zldev: *mut zl3073x_dev, addr: u32, value: *mut u32) -> i32 { let rc = zl3073x_write_u32(zldev, ZL_REG_HWREG_ADDR, addr); if rc != 0 { return rc; } let rc = zl3073x_do_hwreg_op(zldev, ZL_HWREG_OP_READ); if rc != 0 { return rc; } zl3073x_read_u32(zldev, ZL_REG_HWREG_READ_DATA, value) }
pub unsafe fn zl3073x_write_hwreg(zldev: *mut zl3073x_dev, addr: u32, value: u32) -> i32 { let mut rc = zl3073x_write_u32(zldev, ZL_REG_HWREG_ADDR, addr); if rc != 0 { return rc; } rc = zl3073x_write_u32(zldev, ZL_REG_HWREG_WRITE_DATA, value); if rc != 0 { return rc; } zl3073x_do_hwreg_op(zldev, ZL_HWREG_OP_WRITE) }
pub unsafe fn zl3073x_update_hwreg(zldev: *mut zl3073x_dev, addr: u32, value: u32, mask: u32) -> i32 { let mut tmp = 0; let rc = zl3073x_read_hwreg(zldev, addr, &mut tmp); if rc != 0 { return rc; } tmp = (tmp & !mask) | (value & mask); zl3073x_write_hwreg(zldev, addr, tmp) }

pub unsafe fn zl3073x_write_hwreg_seq(zldev: *mut zl3073x_dev, seq: *const zl3073x_hwreg_seq_item, num_items: usize) -> i32 {
    let mut rc = 0;
    for i in 0..num_items { let item = &*seq.add(i); dev_dbg((*zldev).dev, "Write 0x%0x [0x%0x] to 0x%0x", item.value, item.mask, item.addr); rc = if item.mask == U32_MAX { zl3073x_write_hwreg(zldev, item.addr, item.value) } else { zl3073x_update_hwreg(zldev, item.addr, item.value, item.mask) }; if rc != 0 { return rc; } if (*seq).wait != 0 { msleep((*seq).wait); } }
    rc
}

unsafe fn zl3073x_dev_state_fetch(zldev: *mut zl3073x_dev) -> i32 {
    let mut rc = zl3073x_read_u16(zldev, ZL_REG_OUTPUT_STEP_TIME_MASK, &mut (*zldev).out_step_time_mask); if rc != 0 { return rc; }
    for i in 0..ZL3073X_NUM_REFS { rc = zl3073x_ref_state_fetch(zldev, i); if rc != 0 { dev_err((*zldev).dev, "Failed to fetch input state: %pe\\n", ERR_PTR(rc)); return rc; } }
    for i in 0..ZL3073X_NUM_SYNTHS { rc = zl3073x_synth_state_fetch(zldev, i); if rc != 0 { dev_err((*zldev).dev, "Failed to fetch synth state: %pe\\n", ERR_PTR(rc)); return rc; } }
    for i in 0..ZL3073X_NUM_OUTS { rc = zl3073x_out_state_fetch(zldev, i); if rc != 0 { dev_err((*zldev).dev, "Failed to fetch output state: %pe\\n", ERR_PTR(rc)); return rc; } }
    for i in 0..(*zldev).info.num_channels { rc = zl3073x_chan_state_fetch(zldev, i); if rc != 0 { dev_err((*zldev).dev, "Failed to fetch channel state: %pe\\n", ERR_PTR(rc)); return rc; } }
    rc
}

unsafe fn zl3073x_dev_ref_states_update(zldev: *mut zl3073x_dev) { for i in 0..ZL3073X_NUM_REFS { let rc = zl3073x_ref_state_update(zldev, i); if rc != 0 { dev_warn((*zldev).dev, "Failed to get REF%u status: %pe\\n", i, ERR_PTR(rc)); } } }

pub unsafe fn zl3073x_ref_phase_offsets_update(zldev: *mut zl3073x_dev, channel: i32) -> i32 {
    let mut rc = zl3073x_poll_zero_u8(zldev, ZL_REG_REF_PHASE_ERR_READ_RQST, ZL_REF_PHASE_ERR_READ_RQST_RD, ZL_POLL_PHASE_ERR_TIMEOUT_US); if rc != 0 { return rc; }
    if channel != -1 { rc = zl3073x_write_u8(zldev, ZL_REG_DPLL_MEAS_IDX, channel as u8); if rc != 0 { return rc; } }
    rc = zl3073x_write_u8(zldev, ZL_REG_REF_PHASE_ERR_READ_RQST, ZL_REF_PHASE_ERR_READ_RQST_RD); if rc != 0 { return rc; }
    zl3073x_poll_zero_u8(zldev, ZL_REG_REF_PHASE_ERR_READ_RQST, ZL_REF_PHASE_ERR_READ_RQST_RD, ZL_POLL_PHASE_ERR_TIMEOUT_US)
}

unsafe fn zl3073x_ref_freq_meas_latch(zldev: *mut zl3073x_dev, typ: u8) -> i32 {
    let mut rc = zl3073x_poll_zero_u8(zldev, ZL_REG_REF_FREQ_MEAS_CTRL, ZL_REF_FREQ_MEAS_CTRL, ZL_POLL_FREQ_MEAS_TIMEOUT_US); if rc != 0 { return rc; }
    rc = zl3073x_write_u8(zldev, ZL_REG_REF_FREQ_MEAS_MASK_3_0, genmask(7, 0) as u8); if rc != 0 { return rc; }
    rc = zl3073x_write_u8(zldev, ZL_REG_REF_FREQ_MEAS_MASK_4, genmask(1, 0) as u8); if rc != 0 { return rc; }
    rc = zl3073x_write_u8(zldev, ZL_REG_REF_FREQ_MEAS_CTRL, typ); if rc != 0 { return rc; }
    zl3073x_poll_zero_u8(zldev, ZL_REG_REF_FREQ_MEAS_CTRL, ZL_REF_FREQ_MEAS_CTRL, ZL_POLL_FREQ_MEAS_TIMEOUT_US)
}

unsafe fn zl3073x_ref_freq_meas_update(zldev: *mut zl3073x_dev) -> i32 {
    let mut rc = zl3073x_ref_freq_meas_latch(zldev, ZL_REF_FREQ_MEAS_CTRL_REF_FREQ); if rc != 0 { return rc; }
    for i in 0..ZL3073X_NUM_REFS { let mut value = 0; rc = zl3073x_read_u32(zldev, ZL_REG_REF_FREQ(i), &mut value); if rc != 0 { return rc; } (*zldev).ref_[i].meas_freq = value; }
    0
}

unsafe fn zl3073x_dev_periodic_work(work: *mut kthread_work) {
    let zldev = container_of!(work, zl3073x_dev, work.work);
    zl3073x_dev_ref_states_update(zldev);
    let mut rc = zl3073x_ref_phase_offsets_update(zldev, -1); if rc != 0 { dev_warn((*zldev).dev, "Failed to update phase offsets: %pe\\n", ERR_PTR(rc)); }
    if READ_ONCE((*zldev).freq_monitor) { rc = zl3073x_ref_freq_meas_update(zldev); if rc != 0 { dev_warn((*zldev).dev, "Failed to update measured frequencies: %pe\\n", ERR_PTR(rc)); } }
    list_for_each_entry!(zldpll, &(*zldev).dplls, list, { zl3073x_dpll_changes_check(zldpll); });
    kthread_queue_delayed_work((*zldev).kworker, &mut (*zldev).work, msecs_to_jiffies(500));
}

pub unsafe fn zl3073x_dev_phase_avg_factor_set(zldev: *mut zl3073x_dev, factor: u8) -> i32 {
    let mut dpll_meas_ctrl = 0; let rc = zl3073x_read_u8(zldev, ZL_REG_DPLL_MEAS_CTRL, &mut dpll_meas_ctrl); if rc != 0 { return rc; }
    let value = (factor + 1) & 0x0f; FIELD_MODIFY!(ZL_DPLL_MEAS_CTRL_AVG_FACTOR, &mut dpll_meas_ctrl, value);
    let rc = zl3073x_write_u8(zldev, ZL_REG_DPLL_MEAS_CTRL, dpll_meas_ctrl); if rc != 0 { return rc; }
    WRITE_ONCE!((*zldev).phase_avg_factor, factor); 0
}

unsafe fn zl3073x_dev_phase_meas_setup(zldev: *mut zl3073x_dev) -> i32 {
    let mut rc = zl3073x_dev_phase_avg_factor_set(zldev, (*zldev).phase_avg_factor); if rc != 0 { return rc; }
    let mut ctrl = 0; rc = zl3073x_read_u8(zldev, ZL_REG_DPLL_MEAS_CTRL, &mut ctrl); if rc != 0 { return rc; }
    ctrl |= ZL_DPLL_MEAS_CTRL_EN; rc = zl3073x_write_u8(zldev, ZL_REG_DPLL_MEAS_CTRL, ctrl); if rc != 0 { return rc; }
    let mut mask = 0; list_for_each_entry!(zldpll, &(*zldev).dplls, list, { mask |= BIT((*zldpll).id); });
    zl3073x_write_u8(zldev, ZL_REG_DPLL_PHASE_ERR_READ_MASK, mask)
}

pub unsafe fn zl3073x_dev_start(zldev: *mut zl3073x_dev, full: bool) -> i32 {
    let mut info = 0; let mut rc = zl3073x_read_u8(zldev, ZL_REG_INFO, &mut info); if rc != 0 { dev_err((*zldev).dev, "Failed to read device status info\\n"); return rc; }
    if !FIELD_GET(ZL_INFO_READY, info) { dev_info((*zldev).dev, "FW not fully ready - missing or corrupted config\\n"); return 0; }
    if full { rc = zl3073x_dev_state_fetch(zldev); if rc != 0 { return rc; } rc = zl3073x_dev_phase_meas_setup(zldev); if rc != 0 { dev_err((*zldev).dev, "Failed to setup phase measurement\\n"); return rc; } }
    list_for_each_entry!(zldpll, &(*zldev).dplls, list, { rc = zl3073x_dpll_register(zldpll); if rc != 0 { dev_err_probe((*zldev).dev, rc, "Failed to register DPLL%u\\n", (*zldpll).id); return rc; } });
    rc = zl3073x_dpll_init_fine_phase_adjust(zldev); if rc != 0 { dev_err_probe((*zldev).dev, rc, "Failed to init fine phase correction\\n"); return rc; }
    kthread_queue_delayed_work((*zldev).kworker, &mut (*zldev).work, 0); 0
}

pub unsafe fn zl3073x_dev_stop(zldev: *mut zl3073x_dev) { kthread_cancel_delayed_work_sync(&mut (*zldev).work); list_for_each_entry!(zldpll, &(*zldev).dplls, list, { if !(*zldpll).dpll_dev.is_null() { zl3073x_dpll_unregister(zldpll); } }); }

unsafe fn zl3073x_dev_dpll_fini(ptr: *mut core::ffi::c_void) { let zldev = ptr as *mut zl3073x_dev; zl3073x_dev_stop(zldev); if !(*zldev).kworker.is_null() { kthread_destroy_worker((*zldev).kworker); (*zldev).kworker = core::ptr::null_mut(); } list_for_each_entry_safe!(zldpll, next, &(*zldev).dplls, list, { list_del(&mut (*zldpll).list); zl3073x_dpll_free(zldpll); }); }

unsafe fn zl3073x_devm_dpll_init(zldev: *mut zl3073x_dev) -> i32 {
    INIT_LIST_HEAD!(&mut (*zldev).dplls);
    for i in 0..(*zldev).info.num_channels { let zldpll = zl3073x_dpll_alloc(zldev, i); if IS_ERR(zldpll) { let rc = PTR_ERR(zldpll); dev_err_probe((*zldev).dev, rc, "Failed to alloc DPLL%u\\n", i); zl3073x_dev_dpll_fini(zldev.cast()); return rc; } list_add_tail!(&mut (*zldpll).list, &mut (*zldev).dplls); }
    kthread_init_delayed_work!(&mut (*zldev).work, zl3073x_dev_periodic_work); let kworker = kthread_run_worker(0, "zl3073x-%s", dev_name((*zldev).dev)); if IS_ERR(kworker) { let rc = PTR_ERR(kworker); zl3073x_dev_dpll_fini(zldev.cast()); return rc; } (*zldev).kworker = kworker;
    let rc = zl3073x_dev_start(zldev, true); if rc != 0 { dev_err_probe((*zldev).dev, rc, "Failed to start device\\n"); zl3073x_dev_dpll_fini(zldev.cast()); return rc; }
    devm_add_action_or_reset((*zldev).dev, zl3073x_dev_dpll_fini, zldev.cast())
}

pub unsafe fn zl3073x_dev_probe(zldev: *mut zl3073x_dev) -> i32 {
    let mut id = 0; let mut rc = zl3073x_read_u16(zldev, ZL_REG_ID, &mut id); if rc != 0 { return rc; }
    let mut i = 0; while i < ZL3073X_CHIP_IDS.len() && ZL3073X_CHIP_IDS[i].id != id { i += 1; }
    if i == ZL3073X_CHIP_IDS.len() { return dev_err_probe((*zldev).dev, -ENODEV, "Unknown chip ID: 0x%04x\\n", id); }
    (*zldev).info = &ZL3073X_CHIP_IDS[i];
    let (mut revision, mut fw_ver, mut cfg_ver) = (0u16, 0u16, 0u32); rc = zl3073x_read_u16(zldev, ZL_REG_REVISION, &mut revision); if rc != 0 { return rc; } rc = zl3073x_read_u16(zldev, ZL_REG_FW_VER, &mut fw_ver); if rc != 0 { return rc; } rc = zl3073x_read_u32(zldev, ZL_REG_CUSTOM_CONFIG_VER, &mut cfg_ver); if rc != 0 { return rc; }
    dev_dbg((*zldev).dev, "ChipID(%X), ChipRev(%X), FwVer(%u)\\n", id, revision, fw_ver); dev_dbg((*zldev).dev, "Custom config version: %lu.%lu.%lu.%lu\\n", FIELD_GET(genmask(31,24), cfg_ver), FIELD_GET(genmask(23,16), cfg_ver), FIELD_GET(genmask(15,8), cfg_ver), FIELD_GET(genmask(7,0), cfg_ver));
    (*zldev).clock_id = get_random_u64(); (*zldev).phase_avg_factor = 2;
    rc = devm_mutex_init((*zldev).dev, &mut (*zldev).multiop_lock); if rc != 0 { return dev_err_probe((*zldev).dev, rc, "Failed to initialize mutex\\n"); } rc = devm_mutex_init((*zldev).dev, &mut (*zldev).phase_step_lock); if rc != 0 { return dev_err_probe((*zldev).dev, rc, "Failed to initialize mutex\\n"); } rc = devm_mutex_init((*zldev).dev, &mut (*zldev).tie_lock); if rc != 0 { return dev_err_probe((*zldev).dev, rc, "Failed to initialize mutex\\n"); }
    rc = zl3073x_devm_dpll_init(zldev); if rc != 0 { return rc; } rc = zl3073x_devlink_register(zldev); if rc != 0 { return dev_err_probe((*zldev).dev, rc, "Failed to register devlink instance\\n"); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
