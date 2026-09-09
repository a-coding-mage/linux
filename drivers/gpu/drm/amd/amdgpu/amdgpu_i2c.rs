/*
 * Copyright 2007-8 Advanced Micro Devices, Inc.
 * Copyright 2008 Red Hat Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 */

// External kernel, DRM, and AMDGPU declarations are supplied by dependencies.

unsafe fn amdgpu_i2c_pre_xfer(i2c_adap: *mut i2c_adapter) -> i32 {
    let i2c = i2c_get_adapdata(i2c_adap);
    let adev = drm_to_adev((*i2c).dev);
    let rec = &mut (*i2c).rec;
    let mut temp: u32;

    mutex_lock(&mut (*i2c).mutex);
    if rec.hw_capable {
        temp = RREG32(adev, rec.mask_clk_reg);
        temp &= !(1u32 << 16);
        WREG32(adev, rec.mask_clk_reg, temp);
    }
    temp = RREG32(adev, rec.a_clk_reg) & !rec.a_clk_mask;
    WREG32(adev, rec.a_clk_reg, temp);
    temp = RREG32(adev, rec.a_data_reg) & !rec.a_data_mask;
    WREG32(adev, rec.a_data_reg, temp);
    temp = RREG32(adev, rec.en_clk_reg) & !rec.en_clk_mask;
    WREG32(adev, rec.en_clk_reg, temp);
    temp = RREG32(adev, rec.en_data_reg) & !rec.en_data_mask;
    WREG32(adev, rec.en_data_reg, temp);
    temp = RREG32(adev, rec.mask_clk_reg) | rec.mask_clk_mask;
    WREG32(adev, rec.mask_clk_reg, temp);
    temp = RREG32(adev, rec.mask_clk_reg);
    temp = RREG32(adev, rec.mask_data_reg) | rec.mask_data_mask;
    WREG32(adev, rec.mask_data_reg, temp);
    temp = RREG32(adev, rec.mask_data_reg);
    0
}

unsafe fn amdgpu_i2c_post_xfer(i2c_adap: *mut i2c_adapter) {
    let i2c = i2c_get_adapdata(i2c_adap);
    let adev = drm_to_adev((*i2c).dev);
    let rec = &mut (*i2c).rec;
    let mut temp: u32;
    temp = RREG32(adev, rec.mask_clk_reg) & !rec.mask_clk_mask;
    WREG32(adev, rec.mask_clk_reg, temp);
    temp = RREG32(adev, rec.mask_clk_reg);
    temp = RREG32(adev, rec.mask_data_reg) & !rec.mask_data_mask;
    WREG32(adev, rec.mask_data_reg, temp);
    temp = RREG32(adev, rec.mask_data_reg);
    mutex_unlock(&mut (*i2c).mutex);
}

unsafe fn amdgpu_i2c_get_clock(i2c_priv: *mut core::ffi::c_void) -> i32 {
    let i2c = i2c_priv as *mut amdgpu_i2c_chan;
    let adev = drm_to_adev((*i2c).dev);
    let rec = &(*i2c).rec;
    let val = RREG32(adev, rec.y_clk_reg) & rec.y_clk_mask;
    (val != 0) as i32
}

unsafe fn amdgpu_i2c_get_data(i2c_priv: *mut core::ffi::c_void) -> i32 {
    let i2c = i2c_priv as *mut amdgpu_i2c_chan;
    let adev = drm_to_adev((*i2c).dev);
    let rec = &(*i2c).rec;
    let val = RREG32(adev, rec.y_data_reg) & rec.y_data_mask;
    (val != 0) as i32
}

unsafe fn amdgpu_i2c_set_clock(i2c_priv: *mut core::ffi::c_void, clock: i32) {
    let i2c = i2c_priv as *mut amdgpu_i2c_chan;
    let adev = drm_to_adev((*i2c).dev);
    let rec = &(*i2c).rec;
    let mut val = RREG32(adev, rec.en_clk_reg) & !rec.en_clk_mask;
    val |= if clock != 0 { 0 } else { rec.en_clk_mask };
    WREG32(adev, rec.en_clk_reg, val);
}

unsafe fn amdgpu_i2c_set_data(i2c_priv: *mut core::ffi::c_void, data: i32) {
    let i2c = i2c_priv as *mut amdgpu_i2c_chan;
    let adev = drm_to_adev((*i2c).dev);
    let rec = &(*i2c).rec;
    let mut val = RREG32(adev, rec.en_data_reg) & !rec.en_data_mask;
    val |= if data != 0 { 0 } else { rec.en_data_mask };
    WREG32(adev, rec.en_data_reg, val);
}

static amdgpu_atombios_i2c_algo: i2c_algorithm = i2c_algorithm {
    master_xfer: Some(amdgpu_atombios_i2c_xfer),
    functionality: Some(amdgpu_atombios_i2c_func),
};

pub unsafe fn amdgpu_i2c_create(dev: *mut drm_device, rec: *const amdgpu_i2c_bus_rec, name: *const core::ffi::c_char) -> *mut amdgpu_i2c_chan {
    let mut i2c: *mut amdgpu_i2c_chan;
    let mut ret: i32;
    if (*rec).mm_i2c && amdgpu_hw_i2c == 0 { return core::ptr::null_mut(); }
    i2c = kzalloc_obj::<amdgpu_i2c_chan>();
    if i2c.is_null() { return core::ptr::null_mut(); }
    (*i2c).rec = *rec;
    (*i2c).adapter.owner = THIS_MODULE;
    (*i2c).adapter.dev.parent = (*dev).dev;
    (*i2c).dev = dev;
    i2c_set_adapdata(&mut (*i2c).adapter, i2c);
    mutex_init(&mut (*i2c).mutex);
    if (*rec).hw_capable && amdgpu_hw_i2c != 0 {
        snprintf((*i2c).adapter.name.as_mut_ptr(), core::mem::size_of_val(&(*i2c).adapter.name), b"AMDGPU i2c hw bus %s\0".as_ptr() as _, name);
        (*i2c).adapter.algo = &amdgpu_atombios_i2c_algo;
        ret = devm_i2c_add_adapter((*dev).dev, &mut (*i2c).adapter);
        if ret != 0 { kfree(i2c as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    } else {
        snprintf((*i2c).adapter.name.as_mut_ptr(), core::mem::size_of_val(&(*i2c).adapter.name), b"AMDGPU i2c bit bus %s\0".as_ptr() as _, name);
        (*i2c).adapter.algo_data = &mut (*i2c).bit as *mut _ as _;
        (*i2c).bit.pre_xfer = Some(amdgpu_i2c_pre_xfer);
        (*i2c).bit.post_xfer = Some(amdgpu_i2c_post_xfer);
        (*i2c).bit.setsda = Some(amdgpu_i2c_set_data);
        (*i2c).bit.setscl = Some(amdgpu_i2c_set_clock);
        (*i2c).bit.getsda = Some(amdgpu_i2c_get_data);
        (*i2c).bit.getscl = Some(amdgpu_i2c_get_clock);
        (*i2c).bit.udelay = 10;
        (*i2c).bit.timeout = usecs_to_jiffies(2200);
        (*i2c).bit.data = i2c as *mut core::ffi::c_void;
        ret = i2c_bit_add_bus(&mut (*i2c).adapter);
        if ret != 0 { DRM_ERROR!("Failed to register bit i2c %s\n", name); kfree(i2c as _); return core::ptr::null_mut(); }
    }
    i2c
}

pub unsafe fn amdgpu_i2c_init(adev: *mut amdgpu_device) {
    if !(*adev).is_atom_fw && !amdgpu_device_has_dc_support(adev) { amdgpu_atombios_i2c_init(adev); }
    else if !(*adev).is_atom_fw { match (*adev).asic_type { CHIP_POLARIS10 | CHIP_POLARIS11 | CHIP_POLARIS12 => amdgpu_atombios_oem_i2c_init(adev, 0x97), _ => {} } }
}

pub unsafe fn amdgpu_i2c_fini(adev: *mut amdgpu_device) {
    for i in 0..AMDGPU_MAX_I2C_BUS { if !(*adev).i2c_bus[i].is_null() { (*adev).i2c_bus[i] = core::ptr::null_mut(); } }
}

pub unsafe fn amdgpu_i2c_lookup(adev: *mut amdgpu_device, i2c_bus: *const amdgpu_i2c_bus_rec) -> *mut amdgpu_i2c_chan {
    for i in 0..AMDGPU_MAX_I2C_BUS { let bus = (*adev).i2c_bus[i]; if !bus.is_null() && (*bus).rec.i2c_id == (*i2c_bus).i2c_id { return bus; } }
    core::ptr::null_mut()
}

unsafe fn amdgpu_i2c_get_byte(i2c_bus: *mut amdgpu_i2c_chan, slave_addr: u8, addr: u8, val: *mut u8) -> i32 {
    let mut out_buf = [addr, 0u8]; let mut in_buf = [0u8; 2];
    let mut msgs = [i2c_msg { addr: slave_addr, flags: 0, len: 1, buf: out_buf.as_mut_ptr() }, i2c_msg { addr: slave_addr, flags: I2C_M_RD, len: 1, buf: in_buf.as_mut_ptr() }];
    if i2c_transfer(&mut (*i2c_bus).adapter, msgs.as_mut_ptr(), 2) != 2 { DRM_DEBUG!("i2c 0x%02x read failed\n", addr); return -EIO; }
    *val = in_buf[0]; DRM_DEBUG!("val = 0x%02x\n", *val); 0
}

unsafe fn amdgpu_i2c_put_byte(i2c_bus: *mut amdgpu_i2c_chan, slave_addr: u8, addr: u8, val: u8) -> i32 {
    let mut out_buf = [addr, val]; let mut msg = i2c_msg { addr: slave_addr, flags: 0, len: 2, buf: out_buf.as_mut_ptr() };
    if i2c_transfer(&mut (*i2c_bus).adapter, &mut msg, 1) != 1 { DRM_DEBUG!("i2c 0x%02x 0x%02x write failed\n", addr, val); return -EIO; } 0
}

pub unsafe fn amdgpu_i2c_router_select_ddc_port(c: *const amdgpu_connector) {
    if !(*c).router.ddc_valid || (*c).router_bus.is_null() { return; }
    let mut val = 0u8; if amdgpu_i2c_get_byte((*c).router_bus, (*c).router.i2c_addr, 3, &mut val) != 0 { return; }
    val &= !(*c).router.ddc_mux_control_pin; amdgpu_i2c_put_byte((*c).router_bus, (*c).router.i2c_addr, 3, val);
    if amdgpu_i2c_get_byte((*c).router_bus, (*c).router.i2c_addr, 1, &mut val) != 0 { return; }
    val = (val & !(*c).router.ddc_mux_control_pin) | (*c).router.ddc_mux_state; amdgpu_i2c_put_byte((*c).router_bus, (*c).router.i2c_addr, 1, val);
}

pub unsafe fn amdgpu_i2c_router_select_cd_port(c: *const amdgpu_connector) {
    if !(*c).router.cd_valid || (*c).router_bus.is_null() { return; }
    let mut val: u8; if amdgpu_i2c_get_byte((*c).router_bus, (*c).router.i2c_addr, 3, &mut val) != 0 { return; }
    val &= !(*c).router.cd_mux_control_pin; amdgpu_i2c_put_byte((*c).router_bus, (*c).router.i2c_addr, 3, val);
    if amdgpu_i2c_get_byte((*c).router_bus, (*c).router.i2c_addr, 1, &mut val) != 0 { return; }
    val = (val & !(*c).router.cd_mux_control_pin) | (*c).router.cd_mux_state; amdgpu_i2c_put_byte((*c).router_bus, (*c).router.i2c_addr, 1, val);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
