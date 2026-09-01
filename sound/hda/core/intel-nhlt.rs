// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2015-2019 Intel Corporation

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_table_header {
    pub signature: [c_char; 4],
    pub length: u32,
}

pub type acpi_status = u32;

unsafe extern "C" {
    fn acpi_get_table(
        signature: *const c_char,
        instance: u32,
        out_table: *mut *mut acpi_table_header,
    ) -> acpi_status;
    fn acpi_put_table(table: *mut acpi_table_header);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn hweight_long(w: c_uint) -> c_uint;
}

unsafe extern "C" {
    static ACPI_SIG_NHLT: [c_char; 5];
}

const EINVAL: c_int = 22;

const NHLT_LINK_DMIC: u8 = 2;
const NHLT_LINK_SSP: u8 = 0;
const NHLT_DEVICE_BT: u8 = 0;
const NHLT_DEVICE_I2S: u8 = 4;
const NHLT_CONFIG_TYPE_MIC_ARRAY: u8 = 1;
const NHLT_MIC_ARRAY_2CH_SMALL: u8 = 0xa;
const NHLT_MIC_ARRAY_2CH_BIG: u8 = 0xb;
const NHLT_MIC_ARRAY_4CH_1ST_GEOM: u8 = 0xc;
const NHLT_MIC_ARRAY_4CH_L_SHAPED: u8 = 0xd;
const NHLT_MIC_ARRAY_4CH_2ND_GEOM: u8 = 0xe;
const NHLT_MIC_ARRAY_VENDOR_DEFINED: u8 = 0xf;
const MIC_ARRAY_2CH: c_uint = 2;
const MIC_ARRAY_4CH: c_uint = 4;

const SSP_BLOB_V1_0_SIZE: c_int = 84;
const SSP_BLOB_V1_0_MDIVC_OFFSET: c_int = 19; /* offset in u32 */

const SSP_BLOB_V1_5_SIZE: c_int = 96;
const SSP_BLOB_V1_5_MDIVC_OFFSET: c_int = 21; /* offset in u32 */
const SSP_BLOB_VER_1_5: u32 = 0xEE000105;

const SSP_BLOB_V2_0_SIZE: c_int = 88;
const SSP_BLOB_V2_0_MDIVC_OFFSET: c_int = 20; /* offset in u32 */
const SSP_BLOB_VER_2_0: u32 = 0xEE000200;

#[repr(C)]
pub struct nhlt_acpi_table {
    pub header: acpi_table_header,
    pub endpoint_count: u8,
    pub desc: *mut nhlt_endpoint,
}

#[repr(C)]
pub struct nhlt_endpoint {
    pub length: u32,
    pub linktype: u8,
    pub instance_id: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u16,
    pub subsystem_id: u32,
    pub device_type: u8,
    pub direction: u8,
    pub virtual_bus_id: u8,
    pub config: nhlt_specific_cfg,
}

#[repr(C)]
pub struct nhlt_specific_cfg {
    pub size: u32,
    pub caps: *mut u8,
}

#[repr(C)]
pub struct nhlt_dmic_array_config {
    pub device_config: nhlt_specific_cfg,
    pub array_type: u8,
}

#[repr(C)]
pub struct nhlt_vendor_dmic_array_config {
    pub device_config: nhlt_specific_cfg,
    pub array_type: u8,
    pub nb_mics: u8,
}

#[repr(C)]
pub struct nhlt_fmt {
    pub fmt_count: u8,
    pub fmt_config: *mut nhlt_fmt_cfg,
}

#[repr(C)]
pub struct nhlt_fmt_cfg {
    pub fmt_ext: wav_fmt_ext,
    pub config: nhlt_specific_cfg,
}

#[repr(C)]
pub struct wav_fmt {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

#[repr(C)]
pub struct wav_fmt_ext_sample {
    pub valid_bits_per_sample: u16,
}

#[repr(C)]
pub struct wav_fmt_ext {
    pub fmt: wav_fmt,
    pub samples: u16,
    pub sample: wav_fmt_ext_sample,
}

#[inline]
fn acpi_failure(status: acpi_status) -> bool {
    (status as i32) < 0
}

#[inline]
fn bit(nr: u8) -> c_int {
    1i32.wrapping_shl(nr as u32)
}

#[inline]
fn genmask(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_nhlt_init(dev: *mut device) -> *mut nhlt_acpi_table {
    let mut nhlt: *mut nhlt_acpi_table = ptr::null_mut();
    let status: acpi_status;

    status = unsafe {
        acpi_get_table(
            ACPI_SIG_NHLT.as_ptr(),
            0,
            &mut nhlt as *mut *mut nhlt_acpi_table as *mut *mut acpi_table_header,
        )
    };
    if acpi_failure(status) {
        unsafe {
            dev_warn(dev, c"NHLT table not found\n".as_ptr());
        }
        return ptr::null_mut();
    }

    nhlt
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_nhlt_free(nhlt: *mut nhlt_acpi_table) {
    unsafe {
        acpi_put_table(nhlt as *mut acpi_table_header);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_nhlt_get_dmic_geo(
    dev: *mut device,
    nhlt: *mut nhlt_acpi_table,
) -> c_int {
    let mut epnt: *mut nhlt_endpoint;
    let mut cfg: *mut nhlt_dmic_array_config;
    let mut cfg_vendor: *mut nhlt_vendor_dmic_array_config;
    let mut fmt_configs: *mut nhlt_fmt;
    let mut dmic_geo: c_uint = 0;
    let mut max_ch: u16 = 0;
    let mut i: u8;
    let mut j: u8;

    if nhlt.is_null() {
        return 0;
    }

    if unsafe { (*nhlt).header.length } <= size_of::<acpi_table_header>() as u32 {
        unsafe {
            dev_warn(dev, c"Invalid DMIC description table\n".as_ptr());
        }
        return 0;
    }

    j = 0;
    epnt = unsafe { (*nhlt).desc };
    while j < unsafe { (*nhlt).endpoint_count } {
        if unsafe { (*epnt).linktype } != NHLT_LINK_DMIC {
            j = j.wrapping_add(1);
            epnt = unsafe { (epnt as *mut u8).add((*epnt).length as usize) as *mut nhlt_endpoint };
            continue;
        }

        cfg = unsafe { (*epnt).config.caps as *mut nhlt_dmic_array_config };
        fmt_configs = unsafe { (*epnt).config.caps.add((*epnt).config.size as usize) as *mut nhlt_fmt };

        /* find max number of channels based on format_configuration */
        if unsafe { (*fmt_configs).fmt_count } != 0 {
            let mut fmt_cfg: *mut nhlt_fmt_cfg = unsafe { (*fmt_configs).fmt_config };

            unsafe {
                dev_dbg(
                    dev,
                    c"found %d format definitions\n".as_ptr(),
                    (*fmt_configs).fmt_count as c_int,
                );
            }

            i = 0;
            while i < unsafe { (*fmt_configs).fmt_count } {
                let fmt_ext: *mut wav_fmt_ext;

                fmt_ext = unsafe { &mut (*fmt_cfg).fmt_ext };

                if unsafe { (*fmt_ext).fmt.channels } > max_ch {
                    max_ch = unsafe { (*fmt_ext).fmt.channels };
                }

                /* Move to the next nhlt_fmt_cfg */
                fmt_cfg = unsafe {
                    (*fmt_cfg)
                        .config
                        .caps
                        .add((*fmt_cfg).config.size as usize) as *mut nhlt_fmt_cfg
                };
                i = i.wrapping_add(1);
            }
            unsafe {
                dev_dbg(dev, c"max channels found %d\n".as_ptr(), max_ch as c_int);
            }
        } else {
            unsafe {
                dev_dbg(dev, c"No format information found\n".as_ptr());
            }
        }

        if unsafe { (*cfg).device_config.size as u8 } != NHLT_CONFIG_TYPE_MIC_ARRAY {
            dmic_geo = max_ch as c_uint;
        } else {
            match unsafe { (*cfg).array_type } {
                NHLT_MIC_ARRAY_2CH_SMALL | NHLT_MIC_ARRAY_2CH_BIG => {
                    dmic_geo = MIC_ARRAY_2CH;
                }

                NHLT_MIC_ARRAY_4CH_1ST_GEOM
                | NHLT_MIC_ARRAY_4CH_L_SHAPED
                | NHLT_MIC_ARRAY_4CH_2ND_GEOM => {
                    dmic_geo = MIC_ARRAY_4CH;
                }
                NHLT_MIC_ARRAY_VENDOR_DEFINED => {
                    cfg_vendor = cfg as *mut nhlt_vendor_dmic_array_config;
                    dmic_geo = unsafe { (*cfg_vendor).nb_mics as c_uint };
                }
                _ => unsafe {
                    dev_warn(
                        dev,
                        c"%s: undefined DMIC array_type 0x%0x\n".as_ptr(),
                        c"intel_nhlt_get_dmic_geo".as_ptr(),
                        (*cfg).array_type as c_int,
                    );
                },
            }

            if dmic_geo > 0 {
                unsafe {
                    dev_dbg(dev, c"Array with %d dmics\n".as_ptr(), dmic_geo as c_int);
                }
            }
            if (max_ch as c_uint) > dmic_geo {
                unsafe {
                    dev_dbg(
                        dev,
                        c"max channels %d exceed dmic number %d\n".as_ptr(),
                        max_ch as c_int,
                        dmic_geo as c_int,
                    );
                }
            }
        }

        j = j.wrapping_add(1);
        epnt = unsafe { (epnt as *mut u8).add((*epnt).length as usize) as *mut nhlt_endpoint };
    }

    unsafe {
        dev_dbg(
            dev,
            c"dmic number %d max_ch %d\n".as_ptr(),
            dmic_geo as c_int,
            max_ch as c_int,
        );
    }

    dmic_geo as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_nhlt_has_endpoint_type(
    nhlt: *mut nhlt_acpi_table,
    link_type: u8,
) -> bool {
    let mut epnt: *mut nhlt_endpoint;
    let mut i: c_int;

    if nhlt.is_null() {
        return false;
    }

    epnt = unsafe { (*nhlt).desc as *mut nhlt_endpoint };
    i = 0;
    while i < unsafe { (*nhlt).endpoint_count as c_int } {
        if unsafe { (*epnt).linktype } == link_type {
            return true;
        }

        epnt = unsafe { (epnt as *mut u8).add((*epnt).length as usize) as *mut nhlt_endpoint };
        i += 1;
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_nhlt_ssp_endpoint_mask(
    nhlt: *mut nhlt_acpi_table,
    device_type: u8,
) -> c_int {
    let mut epnt: *mut nhlt_endpoint;
    let mut ssp_mask: c_int = 0;
    let mut i: c_int;

    if nhlt.is_null() || (device_type != NHLT_DEVICE_BT && device_type != NHLT_DEVICE_I2S) {
        return 0;
    }

    epnt = unsafe { (*nhlt).desc as *mut nhlt_endpoint };
    i = 0;
    while i < unsafe { (*nhlt).endpoint_count as c_int } {
        if unsafe { (*epnt).linktype } == NHLT_LINK_SSP
            && unsafe { (*epnt).device_type } == device_type
        {
            /* for SSP the virtual bus id is the SSP port */
            ssp_mask |= bit(unsafe { (*epnt).virtual_bus_id });
        }
        epnt = unsafe { (epnt as *mut u8).add((*epnt).length as usize) as *mut nhlt_endpoint };
        i += 1;
    }

    ssp_mask
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_nhlt_ssp_mclk_mask(
    nhlt: *mut nhlt_acpi_table,
    ssp_num: c_int,
) -> c_int {
    let mut epnt: *mut nhlt_endpoint;
    let mut fmt: *mut nhlt_fmt;
    let mut cfg: *mut nhlt_fmt_cfg;
    let mut mclk_mask: c_int = 0;
    let mut i: c_int;
    let mut j: c_int;

    if nhlt.is_null() {
        return 0;
    }

    epnt = unsafe { (*nhlt).desc as *mut nhlt_endpoint };
    i = 0;
    while i < unsafe { (*nhlt).endpoint_count as c_int } {
        /* we only care about endpoints connected to an audio codec over SSP */
        if unsafe { (*epnt).linktype } == NHLT_LINK_SSP
            && unsafe { (*epnt).device_type } == NHLT_DEVICE_I2S
            && unsafe { (*epnt).virtual_bus_id as c_int } == ssp_num
        {
            fmt = unsafe { (*epnt).config.caps.add((*epnt).config.size as usize) as *mut nhlt_fmt };
            cfg = unsafe { (*fmt).fmt_config };

            /*
             * In theory all formats should use the same MCLK but it doesn't hurt to
             * double-check that the configuration is consistent
             */
            j = 0;
            while j < unsafe { (*fmt).fmt_count as c_int } {
                let blob: *mut u32;
                let mdivc_offset: c_int;
                let size: c_int;

                /* first check we have enough data to read the blob type */
                if unsafe { (*cfg).config.size } < 8 {
                    return -EINVAL;
                }

                blob = unsafe { (*cfg).config.caps as *mut u32 };

                if unsafe { *blob.add(1) } == SSP_BLOB_VER_2_0 {
                    mdivc_offset = SSP_BLOB_V2_0_MDIVC_OFFSET;
                    size = SSP_BLOB_V2_0_SIZE;
                } else if unsafe { *blob.add(1) } == SSP_BLOB_VER_1_5 {
                    mdivc_offset = SSP_BLOB_V1_5_MDIVC_OFFSET;
                    size = SSP_BLOB_V1_5_SIZE;
                } else {
                    mdivc_offset = SSP_BLOB_V1_0_MDIVC_OFFSET;
                    size = SSP_BLOB_V1_0_SIZE;
                }

                /* make sure we have enough data for the fixed part of the blob */
                if unsafe { (*cfg).config.size } < size as u32 {
                    return -EINVAL;
                }

                mclk_mask |= (unsafe { *blob.add(mdivc_offset as usize) } & genmask(1, 0)) as c_int;

                cfg = unsafe { (*cfg).config.caps.add((*cfg).config.size as usize) as *mut nhlt_fmt_cfg };
                j += 1;
            }
        }
        epnt = unsafe { (epnt as *mut u8).add((*epnt).length as usize) as *mut nhlt_endpoint };
        i += 1;
    }

    /* make sure only one MCLK is used */
    if unsafe { hweight_long(mclk_mask as c_uint) } != 1 {
        return -EINVAL;
    }

    mclk_mask
}

unsafe fn nhlt_get_specific_cfg(
    dev: *mut device,
    fmt: *mut nhlt_fmt,
    num_ch: u8,
    rate: u32,
    vbps: u8,
    bps: u8,
    ignore_vbps: bool,
) -> *mut nhlt_specific_cfg {
    let mut cfg: *mut nhlt_fmt_cfg = unsafe { (*fmt).fmt_config };
    let mut wfmt: *mut wav_fmt;
    let mut _bps: u16;
    let mut _vbps: u16;
    let mut i: c_int;

    unsafe {
        dev_dbg(dev, c"Endpoint format count=%d\n".as_ptr(), (*fmt).fmt_count as c_int);
    }

    i = 0;
    while i < unsafe { (*fmt).fmt_count as c_int } {
        wfmt = unsafe { &mut (*cfg).fmt_ext.fmt };
        _bps = unsafe { (*wfmt).bits_per_sample };
        _vbps = unsafe { (*cfg).fmt_ext.sample.valid_bits_per_sample };

        unsafe {
            dev_dbg(
                dev,
                c"Endpoint format: ch=%d fmt=%d/%d rate=%d\n".as_ptr(),
                (*wfmt).channels as c_int,
                _vbps as c_int,
                _bps as c_int,
                (*wfmt).samples_per_sec,
            );
        }

        /*
         * When looking for exact match of configuration ignore the vbps
         * from NHLT table when ignore_vbps is true
         */
        if unsafe { (*wfmt).channels } == num_ch as u16
            && unsafe { (*wfmt).samples_per_sec } == rate
            && (ignore_vbps || vbps as u16 == _vbps)
            && bps as u16 == _bps
        {
            return unsafe { &mut (*cfg).config };
        }

        cfg = unsafe { (*cfg).config.caps.add((*cfg).config.size as usize) as *mut nhlt_fmt_cfg };
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn nhlt_check_ep_match(
    dev: *mut device,
    epnt: *mut nhlt_endpoint,
    bus_id: u32,
    link_type: u8,
    dir: u8,
    dev_type: u8,
) -> bool {
    unsafe {
        dev_dbg(
            dev,
            c"Endpoint: vbus_id=%d link_type=%d dir=%d dev_type = %d\n".as_ptr(),
            (*epnt).virtual_bus_id as c_int,
            (*epnt).linktype as c_int,
            (*epnt).direction as c_int,
            (*epnt).device_type as c_int,
        );
    }

    if unsafe { (*epnt).virtual_bus_id as u32 } != bus_id
        || unsafe { (*epnt).linktype } != link_type
        || unsafe { (*epnt).direction } != dir
    {
        return false;
    }

    /* link of type DMIC bypasses device_type check */
    unsafe { (*epnt).linktype } == NHLT_LINK_DMIC || unsafe { (*epnt).device_type } == dev_type
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_nhlt_get_endpoint_blob(
    dev: *mut device,
    nhlt: *mut nhlt_acpi_table,
    bus_id: u32,
    link_type: u8,
    vbps: u8,
    bps: u8,
    num_ch: u8,
    rate: u32,
    dir: u8,
    dev_type: u8,
) -> *mut nhlt_specific_cfg {
    let mut cfg: *mut nhlt_specific_cfg;
    let mut epnt: *mut nhlt_endpoint;
    let mut ignore_vbps: bool = false;
    let mut fmt: *mut nhlt_fmt;
    let mut i: c_int;

    if nhlt.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        dev_dbg(dev, c"Looking for configuration:\n".as_ptr());
        dev_dbg(
            dev,
            c"  vbus_id=%d link_type=%d dir=%d, dev_type=%d\n".as_ptr(),
            bus_id,
            link_type as c_int,
            dir as c_int,
            dev_type as c_int,
        );
    }
    if link_type == NHLT_LINK_DMIC && bps == 32 && (vbps == 24 || vbps == 32) {
        /*
         * The DMIC hardware supports only one type of 32 bits sample
         * size, which is 24 bit sampling on the MSB side and bits[1:0]
         * are used for indicating the channel number.
         * It has been observed that some NHLT tables have the vbps
         * specified as 32 while some uses 24.
         * The format these variations describe are identical, the
         * hardware is configured and behaves the same way.
         * Note: when the samples assumed to be vbps=32 then the 'noise'
         * introduced by the lower two bits (channel number) have no
         * real life implication on audio quality.
         */
        unsafe {
            dev_dbg(
                dev,
                c"  ch=%d fmt=%d rate=%d (vbps is ignored for DMIC 32bit format)\n".as_ptr(),
                num_ch as c_int,
                bps as c_int,
                rate,
            );
        }
        ignore_vbps = true;
    } else {
        unsafe {
            dev_dbg(
                dev,
                c"  ch=%d fmt=%d/%d rate=%d\n".as_ptr(),
                num_ch as c_int,
                vbps as c_int,
                bps as c_int,
                rate,
            );
        }
    }
    unsafe {
        dev_dbg(dev, c"Endpoint count=%d\n".as_ptr(), (*nhlt).endpoint_count as c_int);
    }

    epnt = unsafe { (*nhlt).desc as *mut nhlt_endpoint };

    i = 0;
    while i < unsafe { (*nhlt).endpoint_count as c_int } {
        if unsafe { nhlt_check_ep_match(dev, epnt, bus_id, link_type, dir, dev_type) } {
            fmt = unsafe { (*epnt).config.caps.add((*epnt).config.size as usize) as *mut nhlt_fmt };

            cfg = unsafe { nhlt_get_specific_cfg(dev, fmt, num_ch, rate, vbps, bps, ignore_vbps) };
            if !cfg.is_null() {
                return cfg;
            }
        }

        epnt = unsafe { (epnt as *mut u8).add((*epnt).length as usize) as *mut nhlt_endpoint };
        i += 1;
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_nhlt_ssp_device_type(
    dev: *mut device,
    nhlt: *mut nhlt_acpi_table,
    virtual_bus_id: u8,
) -> c_int {
    let mut epnt: *mut nhlt_endpoint;
    let mut i: c_int;

    if nhlt.is_null() {
        unsafe {
            dev_err(
                dev,
                c"%s: NHLT table is missing (query for SSP%d)\n".as_ptr(),
                c"intel_nhlt_ssp_device_type".as_ptr(),
                virtual_bus_id as c_int,
            );
        }
        return -EINVAL;
    }

    epnt = unsafe { (*nhlt).desc as *mut nhlt_endpoint };
    i = 0;
    while i < unsafe { (*nhlt).endpoint_count as c_int } {
        /* for SSP link the virtual bus id is the SSP port number */
        if unsafe { (*epnt).linktype } == NHLT_LINK_SSP
            && unsafe { (*epnt).virtual_bus_id } == virtual_bus_id
        {
            unsafe {
                dev_dbg(
                    dev,
                    c"SSP%d: dev_type=%d\n".as_ptr(),
                    virtual_bus_id as c_int,
                    (*epnt).device_type as c_int,
                );
            }
            return unsafe { (*epnt).device_type as c_int };
        }

        epnt = unsafe { (epnt as *mut u8).add((*epnt).length as usize) as *mut nhlt_endpoint };
        i += 1;
    }

    unsafe {
        dev_err(
            dev,
            c"%s: No match for SSP%d in NHLT table\n".as_ptr(),
            c"intel_nhlt_ssp_device_type".as_ptr(),
            virtual_bus_id as c_int,
        );

        dev_dbg(dev, c"Available endpoints:\n".as_ptr());
    }
    epnt = unsafe { (*nhlt).desc as *mut nhlt_endpoint };
    i = 0;
    while i < unsafe { (*nhlt).endpoint_count as c_int } {
        unsafe {
            dev_dbg(
                dev,
                c"%d: link_type: %d, vbus_id: %d, dir: %d, dev_type: %d\n".as_ptr(),
                i,
                (*epnt).linktype as c_int,
                (*epnt).virtual_bus_id as c_int,
                (*epnt).direction as c_int,
                (*epnt).device_type as c_int,
            );
        }

        epnt = unsafe { (epnt as *mut u8).add((*epnt).length as usize) as *mut nhlt_endpoint };
        i += 1;
    }

    -EINVAL
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
