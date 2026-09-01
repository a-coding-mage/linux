/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2017 Intel Corporation
 *
 * Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sof_intel_hw_ip_version {
    SOF_INTEL_TANGIER,
    SOF_INTEL_BAYTRAIL,
    SOF_INTEL_BROADWELL,
    SOF_INTEL_CAVS_1_5,      /* SkyLake, KabyLake, AmberLake */
    SOF_INTEL_CAVS_1_5_PLUS, /* ApolloLake, GeminiLake */
    SOF_INTEL_CAVS_1_8,      /* CannonLake, CometLake, CoffeeLake */
    SOF_INTEL_CAVS_2_0,      /* IceLake, JasperLake */
    SOF_INTEL_CAVS_2_5,      /* TigerLake, AlderLake */
    SOF_INTEL_ACE_1_0,       /* MeteorLake */
    SOF_INTEL_ACE_2_0,       /* LunarLake */
    SOF_INTEL_ACE_3_0,       /* PantherLake */
    SOF_INTEL_ACE_4_0,       /* NovaLake */
}

/*
 * SHIM registers for BYT, BSW, CHT, BDW
 *
 * SHIM_OFFSET is supplied by another translated header.
 */

pub const SHIM_CSR: u32 = SHIM_OFFSET + 0x00;
pub const SHIM_PISR: u32 = SHIM_OFFSET + 0x08;
pub const SHIM_PIMR: u32 = SHIM_OFFSET + 0x10;
pub const SHIM_ISRX: u32 = SHIM_OFFSET + 0x18;
pub const SHIM_ISRD: u32 = SHIM_OFFSET + 0x20;
pub const SHIM_IMRX: u32 = SHIM_OFFSET + 0x28;
pub const SHIM_IMRD: u32 = SHIM_OFFSET + 0x30;
pub const SHIM_IPCX: u32 = SHIM_OFFSET + 0x38;
pub const SHIM_IPCD: u32 = SHIM_OFFSET + 0x40;
pub const SHIM_ISRSC: u32 = SHIM_OFFSET + 0x48;
pub const SHIM_ISRLPESC: u32 = SHIM_OFFSET + 0x50;
pub const SHIM_IMRSC: u32 = SHIM_OFFSET + 0x58;
pub const SHIM_IMRLPESC: u32 = SHIM_OFFSET + 0x60;
pub const SHIM_IPCSC: u32 = SHIM_OFFSET + 0x68;
pub const SHIM_IPCLPESC: u32 = SHIM_OFFSET + 0x70;
pub const SHIM_CLKCTL: u32 = SHIM_OFFSET + 0x78;
pub const SHIM_CSR2: u32 = SHIM_OFFSET + 0x80;
pub const SHIM_LTRC: u32 = SHIM_OFFSET + 0xE0;
pub const SHIM_HMDC: u32 = SHIM_OFFSET + 0xE8;

pub const SHIM_PWMCTRL: u32 = 0x1000;

/*
 * SST SHIM register bits for BYT, BSW, CHT, BDW
 * Register bit naming and functionaility can differ between devices.
 */

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const fn BIT_ULL(n: u32) -> u64 {
    1u64 << n
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

/* CSR / CS */
pub const SHIM_CSR_RST: u32 = BIT(1);
pub const SHIM_CSR_SBCS0: u32 = BIT(2);
pub const SHIM_CSR_SBCS1: u32 = BIT(3);
pub const fn SHIM_CSR_DCS(x: u32) -> u32 {
    x << 4
}
pub const SHIM_CSR_DCS_MASK: u32 = 0x7 << 4;
pub const SHIM_CSR_STALL: u32 = BIT(10);
pub const SHIM_CSR_S0IOCS: u32 = BIT(21);
pub const SHIM_CSR_S1IOCS: u32 = BIT(23);
pub const SHIM_CSR_LPCS: u32 = BIT(31);
pub const SHIM_CSR_24MHZ_LPCS: u32 = SHIM_CSR_SBCS0 | SHIM_CSR_SBCS1 | SHIM_CSR_LPCS;
pub const SHIM_CSR_24MHZ_NO_LPCS: u32 = SHIM_CSR_SBCS0 | SHIM_CSR_SBCS1;
pub const SHIM_BYT_CSR_RST: u32 = BIT(0);
pub const SHIM_BYT_CSR_VECTOR_SEL: u32 = BIT(1);
pub const SHIM_BYT_CSR_STALL: u32 = BIT(2);
pub const SHIM_BYT_CSR_PWAITMODE: u32 = BIT(3);

/*  ISRX / ISC */
pub const SHIM_ISRX_BUSY: u32 = BIT(1);
pub const SHIM_ISRX_DONE: u32 = BIT(0);
pub const SHIM_BYT_ISRX_REQUEST: u32 = BIT(1);

/*  ISRD / ISD */
pub const SHIM_ISRD_BUSY: u32 = BIT(1);
pub const SHIM_ISRD_DONE: u32 = BIT(0);

/* IMRX / IMC */
pub const SHIM_IMRX_BUSY: u32 = BIT(1);
pub const SHIM_IMRX_DONE: u32 = BIT(0);
pub const SHIM_BYT_IMRX_REQUEST: u32 = BIT(1);

/* IMRD / IMD */
pub const SHIM_IMRD_DONE: u32 = BIT(0);
pub const SHIM_IMRD_BUSY: u32 = BIT(1);
pub const SHIM_IMRD_SSP0: u32 = BIT(16);
pub const SHIM_IMRD_DMAC0: u32 = BIT(21);
pub const SHIM_IMRD_DMAC1: u32 = BIT(22);
pub const SHIM_IMRD_DMAC: u32 = SHIM_IMRD_DMAC0 | SHIM_IMRD_DMAC1;

/*  IPCX / IPCC */
pub const SHIM_IPCX_DONE: u32 = BIT(30);
pub const SHIM_IPCX_BUSY: u32 = BIT(31);
pub const SHIM_BYT_IPCX_DONE: u64 = BIT_ULL(62);
pub const SHIM_BYT_IPCX_BUSY: u64 = BIT_ULL(63);

/*  IPCD */
pub const SHIM_IPCD_DONE: u32 = BIT(30);
pub const SHIM_IPCD_BUSY: u32 = BIT(31);
pub const SHIM_BYT_IPCD_DONE: u64 = BIT_ULL(62);
pub const SHIM_BYT_IPCD_BUSY: u64 = BIT_ULL(63);

/* CLKCTL */
pub const fn SHIM_CLKCTL_SMOS(x: u32) -> u32 {
    x << 24
}
pub const SHIM_CLKCTL_MASK: u32 = 3 << 24;
pub const SHIM_CLKCTL_DCPLCG: u32 = BIT(18);
pub const SHIM_CLKCTL_SCOE1: u32 = BIT(17);
pub const SHIM_CLKCTL_SCOE0: u32 = BIT(16);

/* CSR2 / CS2 */
pub const SHIM_CSR2_SDFD_SSP0: u32 = BIT(1);
pub const SHIM_CSR2_SDFD_SSP1: u32 = BIT(2);

/* LTRC */
pub const fn SHIM_LTRC_VAL(x: u32) -> u32 {
    x << 0
}

/* HMDC */
pub const fn SHIM_HMDC_HDDA0(x: u32) -> u32 {
    x << 0
}
pub const fn SHIM_HMDC_HDDA1(x: u32) -> u32 {
    x << 7
}
pub const SHIM_HMDC_HDDA_E0_CH0: u32 = 1;
pub const SHIM_HMDC_HDDA_E0_CH1: u32 = 2;
pub const SHIM_HMDC_HDDA_E0_CH2: u32 = 4;
pub const SHIM_HMDC_HDDA_E0_CH3: u32 = 8;
pub const SHIM_HMDC_HDDA_E1_CH0: u32 = SHIM_HMDC_HDDA1(SHIM_HMDC_HDDA_E0_CH0);
pub const SHIM_HMDC_HDDA_E1_CH1: u32 = SHIM_HMDC_HDDA1(SHIM_HMDC_HDDA_E0_CH1);
pub const SHIM_HMDC_HDDA_E1_CH2: u32 = SHIM_HMDC_HDDA1(SHIM_HMDC_HDDA_E0_CH2);
pub const SHIM_HMDC_HDDA_E1_CH3: u32 = SHIM_HMDC_HDDA1(SHIM_HMDC_HDDA_E0_CH3);
pub const SHIM_HMDC_HDDA_E0_ALLCH: u32 = SHIM_HMDC_HDDA_E0_CH0
    | SHIM_HMDC_HDDA_E0_CH1
    | SHIM_HMDC_HDDA_E0_CH2
    | SHIM_HMDC_HDDA_E0_CH3;
pub const SHIM_HMDC_HDDA_E1_ALLCH: u32 = SHIM_HMDC_HDDA_E1_CH0
    | SHIM_HMDC_HDDA_E1_CH1
    | SHIM_HMDC_HDDA_E1_CH2
    | SHIM_HMDC_HDDA_E1_CH3;

/* Audio DSP PCI registers */
pub const PCI_VDRTCTL0: u32 = 0xa0;
pub const PCI_VDRTCTL1: u32 = 0xa4;
pub const PCI_VDRTCTL2: u32 = 0xa8;
pub const PCI_VDRTCTL3: u32 = 0xaC;

/* VDRTCTL0 */
pub const PCI_VDRTCL0_D3PGD: u32 = BIT(0);
pub const PCI_VDRTCL0_D3SRAMPGD: u32 = BIT(1);
pub const PCI_VDRTCL0_DSRAMPGE_SHIFT: u32 = 12;
pub const PCI_VDRTCL0_DSRAMPGE_MASK: u32 = GENMASK(
    PCI_VDRTCL0_DSRAMPGE_SHIFT + 19,
    PCI_VDRTCL0_DSRAMPGE_SHIFT,
);
pub const PCI_VDRTCL0_ISRAMPGE_SHIFT: u32 = 2;
pub const PCI_VDRTCL0_ISRAMPGE_MASK: u32 = GENMASK(
    PCI_VDRTCL0_ISRAMPGE_SHIFT + 9,
    PCI_VDRTCL0_ISRAMPGE_SHIFT,
);

/* VDRTCTL2 */
pub const PCI_VDRTCL2_DCLCGE: u32 = BIT(1);
pub const PCI_VDRTCL2_DTCGE: u32 = BIT(10);
pub const PCI_VDRTCL2_APLLSE_MASK: u32 = BIT(31);

/* PMCS */
pub const PCI_PMCS: u32 = 0x84;
pub const PCI_PMCS_PS_MASK: u32 = 0x3;

/* Intel quirks */
pub const SOF_INTEL_PROCEN_FMT_QUIRK: u32 = BIT(0);

/* DSP hardware descriptor */
#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: ::core::ffi::c_int,
    pub host_managed_cores_mask: ::core::ffi::c_int,
    pub init_core_mask: ::core::ffi::c_int, /* cores available after fw boot */
    pub ipc_req: ::core::ffi::c_int,
    pub ipc_req_mask: ::core::ffi::c_int,
    pub ipc_ack: ::core::ffi::c_int,
    pub ipc_ack_mask: ::core::ffi::c_int,
    pub ipc_ctl: ::core::ffi::c_int,
    pub rom_status_reg: ::core::ffi::c_int,
    pub rom_init_timeout: ::core::ffi::c_int,
    pub ssp_count: ::core::ffi::c_int,       /* ssp count of the platform */
    pub ssp_base_offset: ::core::ffi::c_int, /* base address of the SSPs */
    pub sdw_shim_base: u32,
    pub sdw_alh_base: u32,
    pub d0i3_offset: u32,
    pub quirks: u32,
    pub platform: *const ::core::ffi::c_char,
    pub hw_ip_version: sof_intel_hw_ip_version,
    pub read_sdw_lcount: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int>,
    pub enable_sdw_irq: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, enable: bool)>,
    pub check_sdw_irq: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> bool>,
    pub check_sdw_wakeen_irq: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> bool>,
    pub sdw_process_wakeen: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev)>,
    pub check_ipc_irq: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> bool>,
    pub check_mic_privacy_irq: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            alt: bool,
            elid: ::core::ffi::c_int,
        ) -> bool,
    >,
    pub process_mic_privacy: Option<
        unsafe extern "C" fn(sdev: *mut snd_sof_dev, alt: bool, elid: ::core::ffi::c_int),
    >,
    pub power_down_dsp: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int>,
    pub disable_interrupts:
        Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int>,
    pub cl_init: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            stream_tag: ::core::ffi::c_int,
            imr_boot: bool,
        ) -> ::core::ffi::c_int,
    >,
}

unsafe extern "C" {
    pub static sof_tng_ops: snd_sof_dsp_ops;
    pub static tng_chip_info: sof_intel_dsp_desc;
}

#[repr(C)]
pub struct sof_intel_stream {
    pub posn_offset: usize,
}

pub unsafe fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc {
    let desc: *const sof_dev_desc = unsafe { (*pdata).desc };

    unsafe { (*desc).chip_info }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
