/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * MPC85xx/86xx PCI Express structure define
 *
 * Copyright 2007,2011 Freescale Semiconductor, Inc
 */

// The original declarations are kernel-only and depend on types supplied by
// other headers. Those dependencies are intentionally left external here.

pub const PCI_FSL_BRR1: u32 = 0xbf8;
pub const PCI_FSL_BRR1_VER: u32 = 0xffff;

pub const PCIE_LTSSM: u32 = 0x0404;
pub const PCIE_LTSSM_L0: u32 = 0x16;
pub const PCIE_FSL_CSR_CLASSCODE: u32 = 0x474;
pub const PCIE_IP_REV_2_2: u32 = 0x02080202;
pub const PCIE_IP_REV_3_0: u32 = 0x02080300;
pub const PIWAR_EN: u32 = 0x80000000;
pub const PIWAR_PF: u32 = 0x20000000;
pub const PIWAR_TGI_LOCAL: u32 = 0x00f00000;
pub const PIWAR_READ_SNOOP: u32 = 0x00050000;
pub const PIWAR_WRITE_SNOOP: u32 = 0x00005000;
pub const PIWAR_SZ_MASK: u32 = 0x0000003f;

pub const PEX_PMCR_PTOMR: u32 = 0x1;
pub const PEX_PMCR_EXL2S: u32 = 0x2;

pub const PME_DISR_EN_PTOD: u32 = 0x00008000;
pub const PME_DISR_EN_ENL23D: u32 = 0x00002000;
pub const PME_DISR_EN_EXL23D: u32 = 0x00001000;

#[repr(C)]
pub struct pci_outbound_window_regs {
    pub potar: u32,
    pub potear: u32,
    pub powbar: u32,
    pub res1: [u8; 4],
    pub powar: u32,
    pub res2: [u8; 12],
}

#[repr(C)]
pub struct pci_inbound_window_regs {
    pub pitar: u32,
    pub res1: [u8; 4],
    pub piwbar: u32,
    pub piwbear: u32,
    pub piwar: u32,
    pub res2: [u8; 12],
}

#[repr(C)]
pub struct ccsr_pci {
    pub config_addr: u32,
    pub config_data: u32,
    pub int_ack: u32,
    pub pex_otb_cpl_tor: u32,
    pub pex_conf_tor: u32,
    pub pex_config: u32,
    pub pex_int_status: u32,
    pub res2: [u8; 4],
    pub pex_pme_mes_dr: u32,
    pub pex_pme_mes_disr: u32,
    pub pex_pme_mes_ier: u32,
    pub pex_pmcr: u32,
    pub res3: [u8; 3016],
    pub block_rev1: u32,
    pub block_rev2: u32,
    pub pow: [pci_outbound_window_regs; 5],
    pub res14: [u8; 96],
    pub pmit: pci_inbound_window_regs,
    pub res6: [u8; 96],
    pub piw: [pci_inbound_window_regs; 4],
    pub pex_err_dr: u32,
    pub res21: [u8; 4],
    pub pex_err_en: u32,
    pub res22: [u8; 4],
    pub pex_err_disr: u32,
    pub res23: [u8; 12],
    pub pex_err_cap_stat: u32,
    pub res24: [u8; 4],
    pub pex_err_cap_r0: u32,
    pub pex_err_cap_r1: u32,
    pub pex_err_cap_r2: u32,
    pub pex_err_cap_r3: u32,
    pub res_e38: [u8; 200],
    pub pdb_stat: u32,
    pub res_f04: [u8; 16],
    pub pex_csr0: u32,
    pub pex_csr1: u32,
    pub res_f1c: [u8; 228],
}

pub const PEX_CSR0_LTSSM_MASK: u32 = 0xFC;
pub const PEX_CSR0_LTSSM_SHIFT: u32 = 2;
pub const PEX_CSR0_LTSSM_L0: u32 = 0x11;

extern "C" {
    pub fn fsl_pcibios_fixup_bus(bus: *mut pci_bus);
    pub fn fsl_pcibios_fixup_phb(phb: *mut pci_controller);
    pub fn mpc83xx_add_bridge(dev: *mut device_node) -> i32;
    pub fn fsl_pci_immrbar_base(hose: *mut pci_controller) -> u64;
    pub static mut fsl_pci_primary: *mut device_node;
}

// CONFIG_PCI selects the external implementation; otherwise the C header
// supplied an empty inline function.
#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    pub fn fsl_pci_assign_primary();
}
#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub fn fsl_pci_assign_primary() {}

// CONFIG_FSL_PCI selects the external implementation; otherwise the C header
// supplied an inline function returning zero.
#[cfg(feature = "CONFIG_FSL_PCI")]
extern "C" {
    pub fn fsl_pci_mcheck_exception(regs: *mut pt_regs) -> i32;
}
#[cfg(not(feature = "CONFIG_FSL_PCI"))]
#[inline]
pub fn fsl_pci_mcheck_exception(_regs: *mut pt_regs) -> i32 { 0 }

#[repr(C)]
pub struct platform_device { _private: [u8; 0] }
#[repr(C)]
pub struct pci_bus { _private: [u8; 0] }
#[repr(C)]
pub struct pci_controller { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
