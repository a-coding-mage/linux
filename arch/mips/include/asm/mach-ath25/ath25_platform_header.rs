/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

/*
 * This is board-specific data that is stored in a "fixed" location in flash.
 * It is shared across operating systems, so it should not be changed lightly.
 * The main reason we need it is in order to extract the ethernet MAC
 * address(es).
 */
#[repr(C)]
pub struct ath25_boarddata {
    pub magic: u32, /* board data is valid */
    pub cksum: u16, /* checksum (starting with BD_REV 2) */
    pub rev: u16, /* revision of this struct */
    pub board_name: [i8; 64], /* Name of board */
    pub major: u16, /* Board major number */
    pub minor: u16, /* Board minor number */
    pub flags: u32, /* Board configuration */
    pub reset_config_gpio: u16, /* Reset factory GPIO pin */
    pub sys_led_gpio: u16, /* System LED GPIO pin */
    pub cpu_freq: u32, /* CPU core frequency in Hz */
    pub sys_freq: u32, /* System frequency in Hz */
    pub cnt_freq: u32, /* Calculated C0_COUNT frequency */
    pub wlan0_mac: [u8; ETH_ALEN],
    pub enet0_mac: [u8; ETH_ALEN],
    pub enet1_mac: [u8; ETH_ALEN],
    pub pci_id: u16, /* Pseudo PCIID for common code */
    pub mem_cap: u16, /* cap bank1 in MB */
    /* version 3 */
    pub wlan1_mac: [u8; ETH_ALEN], /* (ar5212) */
}

pub const ATH25_BD_MAGIC: u32 = 0x35333131; /* "5311", for all 531x/231x platforms */
pub const BD_REV: u16 = 4;
pub const BD_ENET0: u32 = 0x00000001; /* ENET0 is stuffed */
pub const BD_ENET1: u32 = 0x00000002; /* ENET1 is stuffed */
pub const BD_UART1: u32 = 0x00000004; /* UART1 is stuffed */
pub const BD_UART0: u32 = 0x00000008; /* UART0 is stuffed (dma) */
pub const BD_RSTFACTORY: u32 = 0x00000010; /* Reset factory defaults stuffed */
pub const BD_SYSLED: u32 = 0x00000020; /* System LED stuffed */
pub const BD_EXTUARTCLK: u32 = 0x00000040; /* External UART clock */
pub const BD_CPUFREQ: u32 = 0x00000080; /* cpu freq is valid in nvram */
pub const BD_SYSFREQ: u32 = 0x00000100; /* sys freq is set in nvram */
pub const BD_WLAN0: u32 = 0x00000200; /* Enable WLAN0 */
pub const BD_MEMCAP: u32 = 0x00000400; /* CAP SDRAM @ mem_cap for testing */
pub const BD_DISWATCHDOG: u32 = 0x00000800; /* disable system watchdog */
pub const BD_WLAN1: u32 = 0x00001000; /* Enable WLAN1 (ar5212) */
pub const BD_ISCASPER: u32 = 0x00002000; /* FLAG for AR2312 */
pub const BD_WLAN0_2G_EN: u32 = 0x00004000; /* FLAG for radio0_2G */
pub const BD_WLAN0_5G_EN: u32 = 0x00008000; /* FLAG for radio0_2G */
pub const BD_WLAN1_2G_EN: u32 = 0x00020000; /* FLAG for radio0_2G */
pub const BD_WLAN1_5G_EN: u32 = 0x00040000; /* FLAG for radio0_2G */

pub const BOARD_CONFIG_BUFSZ: usize = 0x1000;

/*
 * Platform device information for the Wireless MAC
 */
#[repr(C)]
pub struct ar231x_board_config {
    pub devid: u16,
    /* board config data */
    pub config: *mut ath25_boarddata,
    /* radio calibration data */
    pub radio: *const c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
