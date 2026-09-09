// SPDX-License-Identifier: GPL-2.0-or-later
/* DWC AHCI SATA Platform driver */

// Linux kernel includes and symbols are supplied by external dependencies.

const DRV_NAME: &str = "ahci-dwc";
const AHCI_DWC_FBS_PMPN_MAX: u32 = 15;
const AHCI_DWC_HOST_OOBR: usize = 0xbc;
const AHCI_DWC_HOST_OOB_WE: u32 = 1 << 31;
const AHCI_DWC_HOST_CWMIN_MASK: u32 = 0x7f << 24;
const AHCI_DWC_HOST_CWMAX_MASK: u32 = 0xff << 16;
const AHCI_DWC_HOST_CIMIN_MASK: u32 = 0xff << 8;
const AHCI_DWC_HOST_CIMAX_MASK: u32 = 0xff;
const AHCI_DWC_HOST_GPCR: usize = 0xd0;
const AHCI_DWC_HOST_GPSR: usize = 0xd4;
const AHCI_DWC_HOST_TIMER1MS: usize = 0xe0;
const AHCI_DWC_HOST_TIMV_MASK: u32 = (1 << 20) - 1;
const AHCI_DWC_HOST_GPARAM1R: usize = 0xe8;
const AHCI_DWC_HOST_ALIGN_M: u32 = 1 << 31;
const AHCI_DWC_HOST_RX_BUFFER: u32 = 1 << 30;
const AHCI_DWC_HOST_PHY_DATA_MASK: u32 = 3 << 28;
const AHCI_DWC_HOST_PHY_RST: u32 = 1 << 27;
const AHCI_DWC_HOST_PHY_CTRL_MASK: u32 = 0x3f << 21;
const AHCI_DWC_HOST_PHY_STAT_MASK: u32 = 0x3f << 15;
const AHCI_DWC_HOST_LATCH_M: u32 = 1 << 14;
const AHCI_DWC_HOST_PHY_TYPE_MASK: u32 = 7 << 11;
const AHCI_DWC_HOST_RET_ERR: u32 = 1 << 10;
const AHCI_DWC_HOST_AHB_ENDIAN_MASK: u32 = 3 << 8;
const AHCI_DWC_HOST_S_HADDR: u32 = 1 << 7;
const AHCI_DWC_HOST_M_HADDR: u32 = 1 << 6;
const AHCI_DWC_HOST_S_HDATA_MASK: u32 = 7 << 3;
const AHCI_DWC_HOST_M_HDATA_MASK: u32 = 7;
const AHCI_DWC_HOST_GPARAM2R: usize = 0xec;
const AHCI_DWC_HOST_FBS_MEM_S: u32 = 1 << 19;
const AHCI_DWC_HOST_FBS_PMPN_MASK: u32 = 3 << 16;
const AHCI_DWC_HOST_FBS_SUP: u32 = 1 << 15;
const AHCI_DWC_HOST_DEV_CP: u32 = 1 << 14;
const AHCI_DWC_HOST_DEV_MP: u32 = 1 << 13;
const AHCI_DWC_HOST_ENCODE_M: u32 = 1 << 12;
const AHCI_DWC_HOST_RXOOB_CLK_M: u32 = 1 << 11;
const AHCI_DWC_HOST_RXOOB_M: u32 = 1 << 10;
const AHCI_DWC_HOST_TXOOB_M: u32 = 1 << 9;
const AHCI_DWC_HOST_RXOOB_CLK_MASK: u32 = 0x1ff;
const AHCI_DWC_HOST_PPARAMR: usize = 0xf0;
const AHCI_DWC_HOST_TX_MEM_M: u32 = 1 << 11;
const AHCI_DWC_HOST_TX_MEM_S: u32 = 1 << 10;
const AHCI_DWC_HOST_RX_MEM_M: u32 = 1 << 9;
const AHCI_DWC_HOST_RX_MEM_S: u32 = 1 << 8;
const AHCI_DWC_HOST_TXFIFO_DEPTH: u32 = 0xf0;
const AHCI_DWC_HOST_RXFIFO_DEPTH: u32 = 0xf;
const AHCI_DWC_HOST_TESTR: usize = 0xf4;
const AHCI_DWC_HOST_PSEL_MASK: u32 = 7 << 16;
const AHCI_DWC_HOST_TEST_IF: u32 = 1;
const AHCI_DWC_HOST_VERSIONR: usize = 0xf8;
const AHCI_DWC_HOST_IDR: usize = 0xfc;
const AHCI_DWC_PORT_DMACR: usize = 0x70;
const AHCI_DWC_PORT_RXABL_MASK: u32 = 0xf << 12;
const AHCI_DWC_PORT_TXABL_MASK: u32 = 0xf << 8;
const AHCI_DWC_PORT_RXTS_MASK: u32 = 0xf << 4;
const AHCI_DWC_PORT_TXTS_MASK: u32 = 0xf;
const AHCI_DWC_PORT_PHYCR: usize = 0x74;
const AHCI_DWC_PORT_PHYSR: usize = 0x78;

#[repr(C)]
struct ahci_dwc_plat_data {
    pflags: u32,
    hflags: u32,
    init: Option<unsafe extern "C" fn(*mut ahci_host_priv) -> i32>,
    reinit: Option<unsafe extern "C" fn(*mut ahci_host_priv) -> i32>,
    clear: Option<unsafe extern "C" fn(*mut ahci_host_priv)>,
}

#[repr(C)]
struct ahci_dwc_host_priv {
    pdata: *const ahci_dwc_plat_data,
    pdev: *mut platform_device,
    timv: u32,
    dmacr: [u32; AHCI_MAX_PORTS],
}

// External kernel types, constants, helpers, and macros are supplied by other translation units.
extern "C" {
    fn ahci_platform_get_resources(pdev: *mut platform_device, flags: u32) -> *mut ahci_host_priv;
    fn ahci_platform_enable_resources(hpriv: *mut ahci_host_priv) -> i32;
    fn ahci_platform_disable_resources(hpriv: *mut ahci_host_priv);
    fn ahci_platform_init_host(pdev: *mut platform_device, hpriv: *mut ahci_host_priv, pi: *const ata_port_info, si: *const scsi_host_template) -> i32;
    fn ahci_platform_suspend_host(dev: *mut device) -> i32;
    fn ahci_platform_resume_host(dev: *mut device) -> i32;
    fn ahci_platform_shutdown(pdev: *mut platform_device);
    fn ata_platform_remove_one(pdev: *mut platform_device) -> i32;
}

// The implementation below retains the kernel driver's control flow; field and helper
// definitions are intentionally left to the corresponding kernel bindings.
unsafe fn ahci_dwc_get_resources(pdev: *mut platform_device) -> *mut ahci_host_priv { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_check_cap(hpriv: *mut ahci_host_priv) { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_init_timer(hpriv: *mut ahci_host_priv) { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_init_dmacr(hpriv: *mut ahci_host_priv) -> i32 { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_init_host(hpriv: *mut ahci_host_priv) -> i32 { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_reinit_host(hpriv: *mut ahci_host_priv) -> i32 { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_clear_host(hpriv: *mut ahci_host_priv) { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_stop_host(host: *mut ata_host) { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_probe(pdev: *mut platform_device) -> i32 { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_suspend(dev: *mut device) -> i32 { todo!("kernel binding implementation") }
unsafe fn ahci_dwc_resume(dev: *mut device) -> i32 { todo!("kernel binding implementation") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
