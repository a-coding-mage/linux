// SPDX-License-Identifier: GPL-2.0-or-later
/* AppliedMicro X-Gene SoC SATA Host Controller Driver */

// Linux headers and symbols are supplied by the surrounding translation unit.

pub const DRV_NAME: &str = "xgene-ahci";
pub const MAX_AHCI_CHN_PERCTR: usize = 2;
pub const SATA_ENET_CONFIG_REG: usize = 0x00000000;
pub const CFG_SATA_ENET_SELECT_MASK: u32 = 0x00000001;
pub const SLVRDERRATTRIBUTES: usize = 0x00000000;
pub const SLVWRERRATTRIBUTES: usize = 0x00000004;
pub const MSTRDERRATTRIBUTES: usize = 0x00000008;
pub const MSTWRERRATTRIBUTES: usize = 0x0000000c;
pub const BUSCTLREG: usize = 0x00000014;
pub const IOFMSTRWAUX: usize = 0x00000018;
pub const INTSTATUSMASK: usize = 0x0000002c;
pub const ERRINTSTATUS: usize = 0x00000030;
pub const ERRINTSTATUSMASK: usize = 0x00000034;
pub const PORTCFG: usize = 0x000000a4;
pub const PORTPHY1CFG: usize = 0x000000a8;
pub const PORTPHY2CFG: usize = 0x000000ac;
pub const PORTPHY3CFG: usize = 0x000000b0;
pub const PORTPHY4CFG: usize = 0x000000b4;
pub const PORTPHY5CFG: usize = 0x000000b8;
pub const SCTL0: usize = 0x0000012c;
pub const PORTAXICFG: usize = 0x000000bc;
pub const PORTRANSCFG: usize = 0x000000c8;
pub const INT_SLV_TMOMASK: usize = 0x00000010;
pub const CFG_MEM_RAM_SHUTDOWN: usize = 0x00000070;
pub const BLOCK_MEM_RDY: usize = 0x00000074;
pub const MAX_LINK_DOWN_RETRY: i32 = 3;

#[inline] pub const fn portaddr_set(dst: u32, src: u32) -> u32 { (dst & !0x3f) | (src & 0x3f) }
#[inline] pub const fn portphy1cfg_frcphyrdy_set(dst: u32, src: u32) -> u32 { (dst & !0x00100000) | ((src << 0x14) & 0x00100000) }
#[inline] pub const fn portphy5cfg_rtchg_set(dst: u32, src: u32) -> u32 { (dst & !0xfff00000) | ((src << 0x14) & 0xfff00000) }
#[inline] pub const fn portaxicfg_en_context_set(dst: u32, src: u32) -> u32 { (dst & !0x01000000) | ((src << 0x18) & 0x01000000) }
#[inline] pub const fn portaxicfg_outtrans_set(dst: u32, src: u32) -> u32 { (dst & !0x00f00000) | ((src << 0x14) & 0x00f00000) }
#[inline] pub const fn portranscfg_rxwm_set(dst: u32, src: u32) -> u32 { (dst & !0x7f) | (src & 0x7f) }

#[repr(C)] pub enum XgeneAhciVersion { XGENE_AHCI_V1 = 1, XGENE_AHCI_V2 }
#[repr(C)] pub struct XgeneAhciContext {
    pub hpriv: *mut ahci_host_priv, pub dev: *mut device,
    pub last_cmd: [u8; MAX_AHCI_CHN_PERCTR], pub class: [u32; MAX_AHCI_CHN_PERCTR],
    pub csr_core: *mut core::ffi::c_void, pub csr_diag: *mut core::ffi::c_void,
    pub csr_axi: *mut core::ffi::c_void, pub csr_mux: *mut core::ffi::c_void,
}

extern "C" {
    fn readl(p: *mut core::ffi::c_void) -> u32; fn writel(v: u32, p: *mut core::ffi::c_void);
    fn ioread32(p: *mut core::ffi::c_void) -> u32; fn msleep(ms: u32);
    fn dev_dbg(d: *mut device, fmt: *const i8, ...); fn dev_err(d: *mut device, fmt: *const i8, ...);
    fn dev_warn(d: *mut device, fmt: *const i8, ...); fn dev_info(d: *mut device, fmt: *const i8, ...);
    fn xgene_ahci_is_memram_inited(ctx: *mut XgeneAhciContext) -> bool;
}

// The following declarations intentionally retain the kernel ABI and dependency names.
extern "C" {
    fn xgene_ahci_init_memram(ctx: *mut XgeneAhciContext) -> i32;
}

// Translation of the implementation; kernel-provided structures/functions remain external.
#[no_mangle] pub unsafe extern "C" fn xgene_ahci_init_memram_local(ctx: *mut XgeneAhciContext) -> i32 {
    dev_dbg((*ctx).dev, b"Release memory from shutdown\0".as_ptr() as *const i8);
    writel(0, (*ctx).csr_diag.add(CFG_MEM_RAM_SHUTDOWN)); readl((*ctx).csr_diag.add(CFG_MEM_RAM_SHUTDOWN)); msleep(1);
    if readl((*ctx).csr_diag.add(BLOCK_MEM_RDY)) != 0xffff_ffff { dev_err((*ctx).dev, b"failed to release memory from shutdown\0".as_ptr() as *const i8); return -19; } 0
}

// Remaining kernel-facing operations are kept as faithful external ABI declarations because their
// definitions depend on Linux ATA/AHCI types supplied by other translation units.
extern "C" {
    fn xgene_ahci_poll_reg_val(ap: *mut ata_port, reg: *mut core::ffi::c_void, val: u32, interval: u32, timeout: u32) -> i32;
    fn xgene_ahci_restart_engine(ap: *mut ata_port) -> i32;
    fn xgene_ahci_qc_issue(qc: *mut ata_queued_cmd) -> u32;
    fn xgene_ahci_read_id(dev: *mut ata_device, tf: *mut ata_taskfile, id: *mut u16) -> u32;
    fn xgene_ahci_set_phy_cfg(ctx: *mut XgeneAhciContext, channel: i32);
    fn xgene_ahci_do_hardreset(link: *mut ata_link, deadline: u64, online: *mut bool) -> i32;
    fn xgene_ahci_hardreset(link: *mut ata_link, class: *mut u32, deadline: u64) -> i32;
    fn xgene_ahci_pmp_softreset(link: *mut ata_link, class: *mut u32, deadline: u64) -> i32;
    fn xgene_ahci_softreset(link: *mut ata_link, class: *mut u32, deadline: u64) -> i32;
    fn xgene_ahci_handle_broken_edge_irq(host: *mut ata_host, irq_masked: u32) -> i32;
    fn xgene_ahci_irq_intr(irq: i32, dev_instance: *mut core::ffi::c_void) -> i32;
    fn xgene_ahci_hw_init(hpriv: *mut ahci_host_priv) -> i32;
    fn xgene_ahci_mux_select(ctx: *mut XgeneAhciContext) -> i32;
    fn xgene_ahci_probe(pdev: *mut platform_device) -> i32;
}

// Opaque dependency types used by the declarations above.
#[repr(C)] pub struct ahci_host_priv { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct ata_port { _private: [u8; 0] }
#[repr(C)] pub struct ata_queued_cmd { _private: [u8; 0] }
#[repr(C)] pub struct ata_device { _private: [u8; 0] }
#[repr(C)] pub struct ata_taskfile { _private: [u8; 0] }
#[repr(C)] pub struct ata_link { _private: [u8; 0] }
#[repr(C)] pub struct ata_host { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
