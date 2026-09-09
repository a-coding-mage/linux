// Definitions for SGI Memory Controller.
// Translated from mc.h; volatile register fields retain C layout and require
// volatile access by callers.

#[repr(C)]
pub struct sgimc_regs {
    pub _unused0: u32,
    pub cpuctrl0: u32,
    pub _unused1: u32,
    pub cpuctrl1: u32,
    pub _unused2: u32,
    pub watchdogt: u32,
    pub _unused3: u32,
    pub systemid: u32,
    pub _unused4: [u32; 3],
    pub divider: u32,
    pub _unused5: u32,
    pub eeprom: u32,
    pub _unused6: [u32; 3],
    pub rcntpre: u32,
    pub _unused7: u32,
    pub rcounter: u32,
    pub _unused8: [u32; 13],
    pub giopar: u32,
    pub _unused9: u32,
    pub cputp: u32,
    pub _unused10: [u32; 3],
    pub lbursttp: u32,
    pub _unused11: [u32; 9],
    pub mconfig0: u32,
    pub _unused12: u32,
    pub mconfig1: u32,
    pub _unused13: u32,
    pub cmacc: u32,
    pub _unused14: u32,
    pub gmacc: u32,
    pub _unused15: u32,
    pub cerr: u32,
    pub _unused16: u32,
    pub cstat: u32,
    pub _unused17: u32,
    pub gerr: u32,
    pub _unused18: u32,
    pub gstat: u32,
    pub _unused19: u32,
    pub syssembit: u32,
    pub _unused20: u32,
    pub mlock: u32,
    pub _unused21: u32,
    pub elock: u32,
    pub _unused22: [u32; 15],
    pub gio_dma_trans: u32,
    pub _unused23: u32,
    pub gio_dma_sbits: u32,
    pub _unused24: u32,
    pub dma_intr_cause: u32,
    pub _unused25: u32,
    pub dma_ctrl: u32,
    pub _unused26: [u32; 5],
    pub dtlb_hi0: u32,
    pub _unused27: u32,
    pub dtlb_lo0: u32,
    pub _unused28: u32,
    pub dtlb_hi1: u32,
    pub _unused29: u32,
    pub dtlb_lo1: u32,
    pub _unused30: u32,
    pub dtlb_hi2: u32,
    pub _unused31: u32,
    pub dtlb_lo2: u32,
    pub _unused32: u32,
    pub dtlb_hi3: u32,
    pub _unused33: u32,
    pub dtlb_lo3: u32,
    pub _unused34: [u32; 0x0392],
    pub _unused35: u32,
    pub rpsscounter: u32,
    pub _unused36: [u32; 0x1000 / 4 - 2 * 4],
    pub _unused37: u32,
    pub maddronly: u32,
    pub _unused38: u32,
    pub maddrpdeflts: u32,
    pub _unused39: u32,
    pub dmasz: u32,
    pub _unused40: u32,
    pub ssize: u32,
    pub _unused41: u32,
    pub gmaddronly: u32,
    pub _unused42: u32,
    pub dmaddnpgo: u32,
    pub _unused43: u32,
    pub dmamode: u32,
    pub _unused44: u32,
    pub dmaccount: u32,
    pub _unused45: u32,
    pub dmastart: u32,
    pub _unused46: u32,
    pub dmarunning: u32,
    pub _unused47: u32,
    pub maddrdefstart: u32,
}

pub const SGIMC_CCTRL0_REFS: u32 = 0x0000000f;
pub const SGIMC_CCTRL0_EREFRESH: u32 = 0x00000010;
pub const SGIMC_CCTRL0_EPERRGIO: u32 = 0x00000020;
pub const SGIMC_CCTRL0_EPERRMEM: u32 = 0x00000040;
pub const SGIMC_CCTRL0_EPERRCPU: u32 = 0x00000080;
pub const SGIMC_CCTRL0_WDOG: u32 = 0x00000100;
pub const SGIMC_CCTRL0_SYSINIT: u32 = 0x00000200;
pub const SGIMC_CCTRL0_GFXRESET: u32 = 0x00000400;
pub const SGIMC_CCTRL0_EISALOCK: u32 = 0x00000800;
pub const SGIMC_CCTRL0_EPERRSCMD: u32 = 0x00001000;
pub const SGIMC_CCTRL0_IENAB: u32 = 0x00002000;
pub const SGIMC_CCTRL0_ESNOOP: u32 = 0x00004000;
pub const SGIMC_CCTRL0_EPROMWR: u32 = 0x00008000;
pub const SGIMC_CCTRL0_WRESETPMEM: u32 = 0x00010000;
pub const SGIMC_CCTRL0_LENDIAN: u32 = 0x00020000;
pub const SGIMC_CCTRL0_WRESETDMEM: u32 = 0x00040000;
pub const SGIMC_CCTRL0_CMEMBADPAR: u32 = 0x02000000;
pub const SGIMC_CCTRL0_R4KNOCHKPARR: u32 = 0x04000000;
pub const SGIMC_CCTRL0_GIOBTOB: u32 = 0x08000000;
pub const SGIMC_CCTRL1_EGIOTIMEO: u32 = 0x00000010;
pub const SGIMC_CCTRL1_FIXEDEHPC: u32 = 0x00001000;
pub const SGIMC_CCTRL1_LITTLEHPC: u32 = 0x00002000;
pub const SGIMC_CCTRL1_FIXEDEEXP0: u32 = 0x00004000;
pub const SGIMC_CCTRL1_LITTLEEXP0: u32 = 0x00008000;
pub const SGIMC_CCTRL1_FIXEDEEXP1: u32 = 0x00010000;
pub const SGIMC_CCTRL1_LITTLEEXP1: u32 = 0x00020000;
pub const SGIMC_SYSID_MASKREV: u32 = 0x0000000f;
pub const SGIMC_SYSID_EPRESENT: u32 = 0x00000010;
pub const SGIMC_EEPROM_PRE: u32 = 0x00000001;
pub const SGIMC_EEPROM_CSEL: u32 = 0x00000002;
pub const SGIMC_EEPROM_SECLOCK: u32 = 0x00000004;
pub const SGIMC_EEPROM_SDATAO: u32 = 0x00000008;
pub const SGIMC_EEPROM_SDATAI: u32 = 0x00000010;
pub const SGIMC_GIOPAR_HPC64: u32 = 0x00000001;
pub const SGIMC_GIOPAR_GFX64: u32 = 0x00000002;
pub const SGIMC_GIOPAR_EXP064: u32 = 0x00000004;
pub const SGIMC_GIOPAR_EXP164: u32 = 0x00000008;
pub const SGIMC_GIOPAR_EISA64: u32 = 0x00000010;
pub const SGIMC_GIOPAR_HPC264: u32 = 0x00000020;
pub const SGIMC_GIOPAR_RTIMEGFX: u32 = 0x00000040;
pub const SGIMC_GIOPAR_RTIMEEXP0: u32 = 0x00000080;
pub const SGIMC_GIOPAR_RTIMEEXP1: u32 = 0x00000100;
pub const SGIMC_GIOPAR_MASTEREISA: u32 = 0x00000200;
pub const SGIMC_GIOPAR_ONEBUS: u32 = 0x00000400;
pub const SGIMC_GIOPAR_MASTERGFX: u32 = 0x00000800;
pub const SGIMC_GIOPAR_MASTEREXP0: u32 = 0x00001000;
pub const SGIMC_GIOPAR_MASTEREXP1: u32 = 0x00002000;
pub const SGIMC_GIOPAR_PLINEEXP0: u32 = 0x00004000;
pub const SGIMC_GIOPAR_PLINEEXP1: u32 = 0x00008000;
pub const SGIMC_MCONFIG_BASEADDR: u32 = 0x000000ff;
pub const SGIMC_MCONFIG_RMASK: u32 = 0x00001f00;
pub const SGIMC_MCONFIG_BVALID: u32 = 0x00002000;
pub const SGIMC_MCONFIG_SBANKS: u32 = 0x00004000;
pub const SGIMC_MACC_ALIASBIG: u32 = 0x20000000;
pub const SGIMC_CSTAT_RD: u32 = 0x00000100;
pub const SGIMC_CSTAT_PAR: u32 = 0x00000200;
pub const SGIMC_CSTAT_ADDR: u32 = 0x00000400;
pub const SGIMC_CSTAT_SYSAD_PAR: u32 = 0x00000800;
pub const SGIMC_CSTAT_SYSCMD_PAR: u32 = 0x00001000;
pub const SGIMC_CSTAT_BAD_DATA: u32 = 0x00002000;
pub const SGIMC_CSTAT_PAR_MASK: u32 = 0x00001f00;
pub const SGIMC_CSTAT_RD_PAR: u32 = SGIMC_CSTAT_RD | SGIMC_CSTAT_PAR;
pub const SGIMC_GSTAT_RD: u32 = 0x00000100;
pub const SGIMC_GSTAT_WR: u32 = 0x00000200;
pub const SGIMC_GSTAT_TIME: u32 = 0x00000400;
pub const SGIMC_GSTAT_PROM: u32 = 0x00000800;
pub const SGIMC_GSTAT_ADDR: u32 = 0x00001000;
pub const SGIMC_GSTAT_BC: u32 = 0x00002000;
pub const SGIMC_GSTAT_PIO_RD: u32 = 0x00004000;
pub const SGIMC_GSTAT_PIO_WR: u32 = 0x00008000;

pub const SGIMC_BASE: usize = 0x1fa00000;
pub const SGIMC_SEG0_BADDR: usize = 0x08000000;
pub const SGIMC_SEG1_BADDR: usize = 0x20000000;
pub const SGIMC_SEG0_SIZE_ALL: usize = 0x10000000;
pub const SGIMC_SEG1_SIZE_IP20_IP22: usize = 0x08000000;
pub const SGIMC_SEG1_SIZE_IP26_IP28: usize = 0x20000000;

extern "C" {
    pub static mut sgimc: *mut sgimc_regs;
    pub fn sgimc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
