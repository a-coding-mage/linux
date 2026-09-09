/*
 * Copyright (c) 2016 Broadcom Corporation
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

pub const BRCMFMAC_PDATA_NAME: &str = "brcmfmac";
pub const BRCMFMAC_COUNTRY_BUF_SZ: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum brcmf_bus_type {
    BRCMF_BUSTYPE_SDIO,
    BRCMF_BUSTYPE_USB,
    BRCMF_BUSTYPE_PCIE,
}

#[repr(C)]
pub struct brcmfmac_sdio_pd {
    pub txglomsz: i32,
    pub drive_strength: u32,
    pub oob_irq_supported: bool,
    pub oob_irq_nr: u32,
    pub oob_irq_flags: usize,
    pub broken_sg_support: bool,
    pub sd_head_align: u16,
    pub sd_sgentry_align: u16,
    pub reset: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct brcmfmac_pd_cc_entry {
    pub iso3166: [i8; BRCMFMAC_COUNTRY_BUF_SZ],
    pub cc: [i8; BRCMFMAC_COUNTRY_BUF_SZ],
    pub rev: i32,
}

#[repr(C)]
pub struct brcmfmac_pd_cc {
    pub table_size: i32,
    pub table: [brcmfmac_pd_cc_entry; 0],
}

#[repr(C)]
pub union brcmfmac_pd_device_bus {
    pub sdio: brcmfmac_sdio_pd,
}

#[repr(C)]
pub struct brcmfmac_pd_device {
    pub id: u32,
    pub rev: u32,
    pub bus_type: brcmf_bus_type,
    pub feature_disable: u32,
    pub country_codes: *mut brcmfmac_pd_cc,
    pub bus: brcmfmac_pd_device_bus,
}

#[repr(C)]
pub struct brcmfmac_platform_data {
    pub power_on: Option<unsafe extern "C" fn()>,
    pub power_off: Option<unsafe extern "C" fn()>,
    pub fw_alternative_path: *mut i8,
    pub device_count: i32,
    pub devices: [brcmfmac_pd_device; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
