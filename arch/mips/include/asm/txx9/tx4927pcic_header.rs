/*
 * include/asm-mips/txx9/tx4927pcic.h
 * TX4927 PCI controller definitions.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation:
// `u32`, `u64`, `pci_controller`, and `irqreturn_t`.

#[repr(C)]
pub struct tx4927_pcic_reg {
    pub pciid: u32,
    pub pcistatus: u32,
    pub pciccrev: u32,
    pub pcicfg1: u32,
    pub p2gm0plbase: u32, // +10
    pub p2gm0pubase: u32,
    pub p2gm1plbase: u32,
    pub p2gm1pubase: u32,
    pub p2gm2pbase: u32, // +20
    pub p2giopbase: u32,
    pub unused0: u32,
    pub pcisid: u32,
    pub unused1: u32, // +30
    pub pcicapptr: u32,
    pub unused2: u32,
    pub pcicfg2: u32,
    pub g2ptocnt: u32, // +40
    pub unused3: [u32; 15],
    pub g2pstatus: u32, // +80
    pub g2pmask: u32,
    pub pcisstatus: u32,
    pub pcimask: u32,
    pub p2gcfg: u32, // +90
    pub p2gstatus: u32,
    pub p2gmask: u32,
    pub p2gccmd: u32,
    pub unused4: [u32; 24], // +a0
    pub pbareqport: u32, // +100
    pub pbacfg: u32,
    pub pbastatus: u32,
    pub pbamask: u32,
    pub pbabm: u32, // +110
    pub pbacreq: u32,
    pub pbacgnt: u32,
    pub pbacstate: u32,
    pub g2pmgbase: [u64; 3], // +120
    pub g2piogbase: u64,
    pub g2pmmask: [u32; 3], // +140
    pub g2piomask: u32,
    pub g2pmpbase: [u64; 3], // +150
    pub g2piopbase: u64,
    pub pciccfg: u32, // +170
    pub pcicstatus: u32,
    pub pcicmask: u32,
    pub unused5: u32,
    pub p2gmgbase: [u64; 3], // +180
    pub p2giogbase: u64,
    pub g2pcfgadrs: u32, // +1a0
    pub g2pcfgdata: u32,
    pub unused6: [u32; 8],
    pub g2pintack: u32,
    pub g2pspc: u32,
    pub unused7: [u32; 12], // +1d0
    pub pdmca: u64, // +200
    pub pdmga: u64,
    pub pdmpa: u64,
    pub pdmctr: u64,
    pub pdmcfg: u64, // +220
    pub pdmsts: u64,
}

pub const TX4927_PCIC_G2PSTATUS_ALL: u32 = 0x00000003;
pub const TX4927_PCIC_G2PSTATUS_TTOE: u32 = 0x00000002;
pub const TX4927_PCIC_G2PSTATUS_RTOE: u32 = 0x00000001;
pub const TX4927_PCIC_PCISTATUS_ALL: u32 = 0x0000f900;
pub const TX4927_PCIC_PBACFG_FIXPA: u32 = 0x00000008;
pub const TX4927_PCIC_PBACFG_RPBA: u32 = 0x00000004;
pub const TX4927_PCIC_PBACFG_PBAEN: u32 = 0x00000002;
pub const TX4927_PCIC_PBACFG_BMCEN: u32 = 0x00000001;
pub const TX4927_PCIC_PBASTATUS_ALL: u32 = 0x00000001;
pub const TX4927_PCIC_PBASTATUS_BM: u32 = 0x00000001;
pub const TX4927_PCIC_G2PMnGBASE_BSDIS: u64 = 0x0000002000000000;
pub const TX4927_PCIC_G2PMnGBASE_ECHG: u64 = 0x0000001000000000;
pub const TX4927_PCIC_G2PIOGBASE_BSDIS: u64 = 0x0000002000000000;
pub const TX4927_PCIC_G2PIOGBASE_ECHG: u64 = 0x0000001000000000;
pub const TX4927_PCIC_PCICSTATUS_ALL: u32 = 0x000007b8;
pub const TX4927_PCIC_PCICSTATUS_PME: u32 = 0x00000400;
pub const TX4927_PCIC_PCICSTATUS_TLB: u32 = 0x00000200;
pub const TX4927_PCIC_PCICSTATUS_NIB: u32 = 0x00000100;
pub const TX4927_PCIC_PCICSTATUS_ZIB: u32 = 0x00000080;
pub const TX4927_PCIC_PCICSTATUS_PERR: u32 = 0x00000020;
pub const TX4927_PCIC_PCICSTATUS_SERR: u32 = 0x00000010;
pub const TX4927_PCIC_PCICSTATUS_GBE: u32 = 0x00000008;
pub const TX4927_PCIC_PCICSTATUS_IWB: u32 = 0x00000002;
pub const TX4927_PCIC_PCICSTATUS_E2PDONE: u32 = 0x00000001;
pub const TX4927_PCIC_PCICCFG_GBWC_MASK: u32 = 0x0fff0000;
pub const TX4927_PCIC_PCICCFG_HRST: u32 = 0x00000800;
pub const TX4927_PCIC_PCICCFG_SRST: u32 = 0x00000400;
pub const TX4927_PCIC_PCICCFG_IRBER: u32 = 0x00000200;
#[inline]
pub const fn TX4927_PCIC_PCICCFG_G2PMEN(ch: u32) -> u32 { 0x00000100 >> ch }
pub const TX4927_PCIC_PCICCFG_G2PM0EN: u32 = 0x00000100;
pub const TX4927_PCIC_PCICCFG_G2PM1EN: u32 = 0x00000080;
pub const TX4927_PCIC_PCICCFG_G2PM2EN: u32 = 0x00000040;
pub const TX4927_PCIC_PCICCFG_G2PIOEN: u32 = 0x00000020;
pub const TX4927_PCIC_PCICCFG_TCAR: u32 = 0x00000010;
pub const TX4927_PCIC_PCICCFG_ICAEN: u32 = 0x00000008;
pub const TX4927_PCIC_P2GMnGBASE_TMEMEN: u64 = 0x0000004000000000;
pub const TX4927_PCIC_P2GMnGBASE_TBSDIS: u64 = 0x0000002000000000;
pub const TX4927_PCIC_P2GMnGBASE_TECHG: u64 = 0x0000001000000000;
pub const TX4927_PCIC_P2GIOGBASE_TIOEN: u64 = 0x0000004000000000;
pub const TX4927_PCIC_P2GIOGBASE_TBSDIS: u64 = 0x0000002000000000;
pub const TX4927_PCIC_P2GIOGBASE_TECHG: u64 = 0x0000001000000000;

#[inline]
pub const fn TX4927_PCIC_IDSEL_AD_TO_SLOT(ad: u32) -> u32 { ad - 11 }
pub const TX4927_PCIC_MAX_DEVNU: u32 = TX4927_PCIC_IDSEL_AD_TO_SLOT(32);

pub const TX4927_PCIC_PDMCFG_RSTFIFO: u32 = 0x00200000;
pub const TX4927_PCIC_PDMCFG_EXFER: u32 = 0x00100000;
pub const TX4927_PCIC_PDMCFG_REQDLY_MASK: u32 = 0x00003800;
pub const TX4927_PCIC_PDMCFG_REQDLY_NONE: u32 = 0 << 11;
pub const TX4927_PCIC_PDMCFG_REQDLY_16: u32 = 1 << 11;
pub const TX4927_PCIC_PDMCFG_REQDLY_32: u32 = 2 << 11;
pub const TX4927_PCIC_PDMCFG_REQDLY_64: u32 = 3 << 11;
pub const TX4927_PCIC_PDMCFG_REQDLY_128: u32 = 4 << 11;
pub const TX4927_PCIC_PDMCFG_REQDLY_256: u32 = 5 << 11;
pub const TX4927_PCIC_PDMCFG_REQDLY_512: u32 = 6 << 11;
pub const TX4927_PCIC_PDMCFG_REQDLY_1024: u32 = 7 << 11;
pub const TX4927_PCIC_PDMCFG_ERRIE: u32 = 0x00000400;
pub const TX4927_PCIC_PDMCFG_NCCMPIE: u32 = 0x00000200;
pub const TX4927_PCIC_PDMCFG_NTCMPIE: u32 = 0x00000100;
pub const TX4927_PCIC_PDMCFG_CHNEN: u32 = 0x00000080;
pub const TX4927_PCIC_PDMCFG_XFRACT: u32 = 0x00000040;
pub const TX4927_PCIC_PDMCFG_BSWAP: u32 = 0x00000020;
pub const TX4927_PCIC_PDMCFG_XFRSIZE_MASK: u32 = 0x0000000c;
pub const TX4927_PCIC_PDMCFG_XFRSIZE_1DW: u32 = 0x00000000;
pub const TX4927_PCIC_PDMCFG_XFRSIZE_1QW: u32 = 0x00000004;
pub const TX4927_PCIC_PDMCFG_XFRSIZE_4QW: u32 = 0x00000008;
pub const TX4927_PCIC_PDMCFG_XFRDIRC: u32 = 0x00000002;
pub const TX4927_PCIC_PDMCFG_CHRST: u32 = 0x00000001;

pub const TX4927_PCIC_PDMSTS_REQCNT_MASK: u32 = 0x3f000000;
pub const TX4927_PCIC_PDMSTS_FIFOCNT_MASK: u32 = 0x00f00000;
pub const TX4927_PCIC_PDMSTS_FIFOWP_MASK: u32 = 0x000c0000;
pub const TX4927_PCIC_PDMSTS_FIFORP_MASK: u32 = 0x00030000;
pub const TX4927_PCIC_PDMSTS_ERRINT: u32 = 0x00000800;
pub const TX4927_PCIC_PDMSTS_DONEINT: u32 = 0x00000400;
pub const TX4927_PCIC_PDMSTS_CHNEN: u32 = 0x00000200;
pub const TX4927_PCIC_PDMSTS_XFRACT: u32 = 0x00000100;
pub const TX4927_PCIC_PDMSTS_ACCMP: u32 = 0x00000080;
pub const TX4927_PCIC_PDMSTS_NCCMP: u32 = 0x00000040;
pub const TX4927_PCIC_PDMSTS_NTCMP: u32 = 0x00000020;
pub const TX4927_PCIC_PDMSTS_CFGERR: u32 = 0x00000008;
pub const TX4927_PCIC_PDMSTS_PCIERR: u32 = 0x00000004;
pub const TX4927_PCIC_PDMSTS_CHNERR: u32 = 0x00000002;
pub const TX4927_PCIC_PDMSTS_DATAERR: u32 = 0x00000001;
pub const TX4927_PCIC_PDMSTS_ALL_CMP: u32 = 0x000000e0;
pub const TX4927_PCIC_PDMSTS_ALL_ERR: u32 = 0x0000000f;

extern "C" {
    pub fn get_tx4927_pcicptr(channel: *mut pci_controller) -> *mut tx4927_pcic_reg;
    pub fn tx4927_pcic_setup(pcicptr: *mut tx4927_pcic_reg, channel: *mut pci_controller, extarb: i32);
    pub fn tx4927_report_pcic_status();
    pub fn tx4927_pcibios_setup(str_: *mut i8) -> *mut i8;
    pub fn tx4927_dump_pcic_settings();
    pub fn tx4927_pcierr_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
