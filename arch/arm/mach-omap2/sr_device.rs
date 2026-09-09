// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3/OMAP4 smartreflex device file
 *
 * Author: Thara Gopinath <thara@ti.com>
 *
 * Based originally on code from smartreflex.c
 * Copyright (C) 2010 Texas Instruments, Inc.
 * Thara Gopinath <thara@ti.com>
 *
 * Copyright (C) 2008 Nokia Corporation
 * Kalle Jokiniemi
 *
 * Copyright (C) 2007 Texas Instruments, Inc.
 * Lesly A M <x0080970@ti.com>
 */
// Dependencies are supplied by the surrounding kernel translation.

unsafe fn sr_set_nvalues(
    volt_data: *mut omap_volt_data,
    sr_data: *mut omap_sr_data,
) {
    let mut nvalue_table: *mut omap_sr_nvalue_table;
    let mut i: i32;
    let mut j: i32;
    let mut count: i32 = 0;

    (*sr_data).nvalue_count = 0;
    (*sr_data).nvalue_table = core::ptr::null_mut();

    while (*volt_data.add(count as usize)).volt_nominal != 0 {
        count += 1;
    }

    nvalue_table = kzalloc_objs::<omap_sr_nvalue_table>(count as usize);
    if nvalue_table.is_null() {
        return;
    }

    i = 0;
    j = 0;
    while i < count {
        let v: u32;

        /*
         * In OMAP4 the efuse registers are 24 bit aligned.
         * A readl_relaxed will fail for non-32 bit aligned address
         * and hence the 8-bit read and shift.
         */
        if cpu_is_omap44xx() {
            let offset: u16 = (*volt_data.add(i as usize)).sr_efuse_offs;
            v = omap_ctrl_readb(offset) as u32
                | ((omap_ctrl_readb(offset + 1) as u32) << 8)
                | ((omap_ctrl_readb(offset + 2) as u32) << 16);
        } else {
            v = omap_ctrl_readl((*volt_data.add(i as usize)).sr_efuse_offs);
        }

        /*
         * Many OMAP SoCs don't have the eFuse values set.
         * For example, pretty much all OMAP3xxx before
         * ES3.something.
         *
         * XXX There needs to be some way for board files or
         * userspace to add these in.
         */
        if v == 0 {
            i += 1;
            continue;
        }

        (*nvalue_table.add(j as usize)).nvalue = v;
        (*nvalue_table.add(j as usize)).efuse_offs = (*volt_data.add(i as usize)).sr_efuse_offs;
        (*nvalue_table.add(j as usize)).errminlimit = (*volt_data.add(i as usize)).sr_errminlimit;
        (*nvalue_table.add(j as usize)).volt_nominal = (*volt_data.add(i as usize)).volt_nominal;

        j += 1;
        i += 1;
    }

    (*sr_data).nvalue_table = nvalue_table;
    (*sr_data).nvalue_count = j;
}

extern "C" {
    static mut omap_sr_pdata: omap_sr_data;
}

unsafe fn sr_init_by_name(name: *const i8, voltdm: *const i8) -> i32 {
    let mut sr_data: *mut omap_sr_data = core::ptr::null_mut();
    let mut volt_data: *mut omap_volt_data;
    static mut I: i32 = 0;

    if !strncmp(name, c"smartreflex_mpu_iva", 20)
        || !strncmp(name, c"smartreflex_mpu", 16)
    {
        sr_data = &mut omap_sr_pdata[OMAP_SR_MPU as usize];
    } else if !strncmp(name, c"smartreflex_core", 17) {
        sr_data = &mut omap_sr_pdata[OMAP_SR_CORE as usize];
    } else if !strncmp(name, c"smartreflex_iva", 16) {
        sr_data = &mut omap_sr_pdata[OMAP_SR_IVA as usize];
    }

    if sr_data.is_null() {
        pr_err!("{}: Unknown instance {:?}\n", "sr_init_by_name", name);
        return -EINVAL;
    }

    (*sr_data).name = name;
    (*sr_data).ip_type = if cpu_is_omap343x() { 1 } else { 2 };
    (*sr_data).senn_mod = 0x1;
    (*sr_data).senp_mod = 0x1;

    if cpu_is_omap34xx() || cpu_is_omap44xx() {
        (*sr_data).err_weight = OMAP3430_SR_ERRWEIGHT;
        (*sr_data).err_maxlimit = OMAP3430_SR_ERRMAXLIMIT;
        (*sr_data).accum_data = OMAP3430_SR_ACCUMDATA;
        if strcmp((*sr_data).name, c"smartreflex_mpu") == 0 {
            (*sr_data).senn_avgweight = OMAP3430_SR1_SENNAVGWEIGHT;
            (*sr_data).senp_avgweight = OMAP3430_SR1_SENPAVGWEIGHT;
        } else {
            (*sr_data).senn_avgweight = OMAP3430_SR2_SENNAVGWEIGHT;
            (*sr_data).senp_avgweight = OMAP3430_SR2_SENPAVGWEIGHT;
        }
    }

    (*sr_data).voltdm = voltdm_lookup(voltdm);
    if (*sr_data).voltdm.is_null() {
        pr_err!("{}: Unable to get voltage domain pointer for VDD {:?}\n", "sr_init_by_name", voltdm);
        I += 1;
        return 0;
    }

    omap_voltage_get_volttable((*sr_data).voltdm, &mut volt_data);
    if volt_data.is_null() {
        pr_err!("{}: No Voltage table registered for VDD{}\n", "sr_init_by_name", I + 1);
        I += 1;
        return 0;
    }

    sr_set_nvalues(volt_data, sr_data);
    I += 1;
    0
}

#[cfg(CONFIG_OMAP_HWMOD)]
unsafe fn sr_dev_init(oh: *mut omap_hwmod, _user: *mut core::ffi::c_void) -> i32 {
    let sr_dev_attr = (*oh).dev_attr as *mut omap_smartreflex_dev_attr;
    if sr_dev_attr.is_null() || (*sr_dev_attr).sensor_voltdm_name.is_null() {
        pr_err!("{}: No voltage domain specified for {:?}. Cannot initialize\n", "sr_dev_init", (*oh).name);
        return 0;
    }
    sr_init_by_name((*oh).name, (*sr_dev_attr).sensor_voltdm_name)
}

#[cfg(not(CONFIG_OMAP_HWMOD))]
unsafe fn sr_dev_init(_oh: *mut omap_hwmod, _user: *mut core::ffi::c_void) -> i32 {
    -EINVAL
}

static OMAP4_SR_INSTANCES: [&[u8]; 3] = [b"mpu\0", b"iva\0", b"core\0"];
static DRA7_SR_INSTANCES: [&[u8]; 2] = [b"mpu\0", b"core\0"];

unsafe fn omap_devinit_smartreflex() -> i32 {
    let mut sr_inst: *const &[u8] = core::ptr::null();
    let mut nr_sr: i32 = 0;

    if soc_is_omap44xx() {
        sr_inst = OMAP4_SR_INSTANCES.as_ptr();
        nr_sr = OMAP4_SR_INSTANCES.len() as i32;
    } else if soc_is_dra7xx() {
        sr_inst = DRA7_SR_INSTANCES.as_ptr();
        nr_sr = DRA7_SR_INSTANCES.len() as i32;
    }

    if nr_sr != 0 {
        for i in 0..nr_sr {
            let name = kasprintf(GFP_KERNEL, c"smartreflex_%s", (*sr_inst.add(i as usize)).as_ptr());
            let voltdm = (*sr_inst.add(i as usize)).as_ptr();
            sr_init_by_name(name, voltdm);
        }
        return 0;
    }

    omap_hwmod_for_each_by_class(c"smartreflex", sr_dev_init, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
