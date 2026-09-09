/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * This interface is used for compatibility with old U-boots *ONLY*.
 * Please do not imitate or extend this.
 */

/*
 * (C) Copyright 2000, 2001
 * Wolfgang Denk, DENX Software Engineering, wd@denx.de.
 */

/* Board information passed to kernel from PPCBoot.
 * The C header's types.h dependency is supplied by the surrounding build.
 */

#[repr(C)]
pub struct bd_info {
    pub bi_memstart: core::ffi::c_ulong,   /* start of DRAM memory */
    pub bi_memsize: core::ffi::c_ulong,    /* size of DRAM memory in bytes */
    pub bi_flashstart: core::ffi::c_ulong, /* start of FLASH memory */
    pub bi_flashsize: core::ffi::c_ulong,  /* size of FLASH memory */
    pub bi_flashoffset: core::ffi::c_ulong, /* reserved area for startup monitor */
    pub bi_sramstart: core::ffi::c_ulong,  /* start of SRAM memory */
    pub bi_sramsize: core::ffi::c_ulong,   /* size of SRAM memory */

    #[cfg(any(target_8xx, target_cpm2, target_85xx, target_83xx, target_86xx))]
    pub bi_immr_base: core::ffi::c_ulong, /* base of IMMR register */
    #[cfg(target_ppc_mpc52xx)]
    pub bi_mbar_base: core::ffi::c_ulong, /* base of internal registers */

    pub bi_bootflags: core::ffi::c_ulong, /* boot / reboot flag (for LynxOS) */
    pub bi_ip_addr: core::ffi::c_ulong,   /* IP Address */
    pub bi_enetaddr: [u8; 6],             /* Ethernet address */
    pub bi_ethspeed: u16,                 /* Ethernet speed in Mbps */
    pub bi_intfreq: core::ffi::c_ulong,   /* Internal Freq, in MHz */
    pub bi_busfreq: core::ffi::c_ulong,   /* Bus Freq, in MHz */

    #[cfg(target_cpm2)]
    pub bi_cpmfreq: core::ffi::c_ulong, /* CPM_CLK Freq, in MHz */
    #[cfg(target_cpm2)]
    pub bi_brgfreq: core::ffi::c_ulong, /* BRG_CLK Freq, in MHz */
    #[cfg(target_cpm2)]
    pub bi_sccfreq: core::ffi::c_ulong, /* SCC_CLK Freq, in MHz */
    #[cfg(target_cpm2)]
    pub bi_vco: core::ffi::c_ulong, /* VCO Out from PLL, in MHz */

    #[cfg(target_ppc_mpc52xx)]
    pub bi_ipbfreq: core::ffi::c_ulong, /* IPB Bus Freq, in MHz */
    #[cfg(target_ppc_mpc52xx)]
    pub bi_pcifreq: core::ffi::c_ulong, /* PCI Bus Freq, in MHz */

    pub bi_baudrate: core::ffi::c_ulong, /* Console Baudrate */

    #[cfg(target_4xx)]
    pub bi_s_version: [u8; 4],
    #[cfg(target_4xx)]
    pub bi_r_version: [u8; 32],
    #[cfg(target_4xx)]
    pub bi_procfreq: core::ffi::c_uint,
    #[cfg(target_4xx)]
    pub bi_plb_busfreq: core::ffi::c_uint,
    #[cfg(target_4xx)]
    pub bi_pci_busfreq: core::ffi::c_uint,
    #[cfg(target_4xx)]
    pub bi_pci_enetaddr: [u8; 6],

    #[cfg(target_hymod)]
    pub bi_hymod_conf: hymod_conf_t,

    #[cfg(any(target_evb64260, target_44x, target_85xx, target_83xx, target_has_eth1))]
    pub bi_enet1addr: [u8; 6],
    #[cfg(any(target_evb64260, target_440gx, target_85xx, target_has_eth2))]
    pub bi_enet2addr: [u8; 6],
    #[cfg(any(target_440gx, target_has_eth3))]
    pub bi_enet3addr: [u8; 6],

    #[cfg(target_4xx)]
    pub bi_opbfreq: core::ffi::c_uint,
    #[cfg(target_4xx)]
    pub bi_iic_fast: [core::ffi::c_int; 2],
    #[cfg(target_440gx)]
    pub bi_phynum: [core::ffi::c_int; 4],
    #[cfg(target_440gx)]
    pub bi_phymode: [core::ffi::c_int; 4],
}

pub type bd_t = bd_info;

/* C macro: #define bi_tbfreq bi_intfreq */
#[macro_export]
macro_rules! bi_tbfreq {
    ($value:expr) => { $value.bi_intfreq };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
