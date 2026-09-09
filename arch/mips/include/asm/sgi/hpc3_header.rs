/* Rust translation of SGI HPC3 controller definitions. */

#[repr(C)]
pub struct hpc_dma_desc {
    pub pbuf: u32,
    pub cntinfo: u32,
    pub pnext: u32,
}

pub const HPCDMA_EOX: u32 = 0x80000000;
pub const HPCDMA_EOR: u32 = 0x80000000;
pub const HPCDMA_EOXP: u32 = 0x40000000;
pub const HPCDMA_EORP: u32 = 0x40000000;
pub const HPCDMA_XIE: u32 = 0x20000000;
pub const HPCDMA_XIU: u32 = 0x01000000;
pub const HPCDMA_EIPC: u32 = 0x00ff0000;
pub const HPCDMA_ETXD: u32 = 0x00008000;
pub const HPCDMA_OWN: u32 = 0x00004000;
pub const HPCDMA_BCNT: u32 = 0x00003fff;

#[repr(C)]
pub struct hpc3_pbus_dmacregs {
    pub pbdma_bptr: u32,
    pub pbdma_dptr: u32,
    pub _unused0: [u32; 0x1000 / 4 - 2],
    pub pbdma_ctrl: u32,
    pub _unused1: [u32; 0x1000 / 4 - 1],
}
pub const HPC3_PDMACTRL_INT: u32 = 0x00000001;
pub const HPC3_PDMACTRL_ISACT: u32 = 0x00000002;
pub const HPC3_PDMACTRL_SEL: u32 = 0x00000002;
pub const HPC3_PDMACTRL_RCV: u32 = 0x00000004;
pub const HPC3_PDMACTRL_FLSH: u32 = 0x00000008;
pub const HPC3_PDMACTRL_ACT: u32 = 0x00000010;
pub const HPC3_PDMACTRL_LD: u32 = 0x00000020;
pub const HPC3_PDMACTRL_RT: u32 = 0x00000040;
pub const HPC3_PDMACTRL_HW: u32 = 0x0000ff00;
pub const HPC3_PDMACTRL_FB: u32 = 0x003f0000;
pub const HPC3_PDMACTRL_FE: u32 = 0x3f000000;

#[repr(C)]
pub struct hpc3_scsiregs {
    pub cbptr: u32, pub ndptr: u32, pub _unused0: [u32; 0x1000 / 4 - 2],
    pub bcd: u32, pub ctrl: u32, pub gfptr: u32, pub dfptr: u32,
    pub dconfig: u32, pub pconfig: u32, pub _unused1: [u32; 0x1000 / 4 - 6],
}
pub const HPC3_SBCD_BCNTMSK:u32=0x00003fff; pub const HPC3_SBCD_XIE:u32=0x00004000; pub const HPC3_SBCD_EOX:u32=0x00008000;
pub const HPC3_SCTRL_IRQ:u32=0x01; pub const HPC3_SCTRL_ENDIAN:u32=0x02; pub const HPC3_SCTRL_DIR:u32=0x04; pub const HPC3_SCTRL_FLUSH:u32=0x08; pub const HPC3_SCTRL_ACTIVE:u32=0x10; pub const HPC3_SCTRL_AMASK:u32=0x20; pub const HPC3_SCTRL_CRESET:u32=0x40; pub const HPC3_SCTRL_PERR:u32=0x80;
pub const HPC3_SDCFG_HCLK:u32=0x00001; pub const HPC3_SDCFG_D1:u32=0x00006; pub const HPC3_SDCFG_D2:u32=0x00038; pub const HPC3_SDCFG_D3:u32=0x001c0; pub const HPC3_SDCFG_HWAT:u32=0x00e00; pub const HPC3_SDCFG_HW:u32=0x01000; pub const HPC3_SDCFG_SWAP:u32=0x02000; pub const HPC3_SDCFG_EPAR:u32=0x04000; pub const HPC3_SDCFG_POLL:u32=0x08000; pub const HPC3_SDCFG_ERLY:u32=0x30000;
pub const HPC3_SPCFG_P3:u32=0x0003; pub const HPC3_SPCFG_P2W:u32=0x001c; pub const HPC3_SPCFG_P2R:u32=0x01e0; pub const HPC3_SPCFG_P1:u32=0x0e00; pub const HPC3_SPCFG_HW:u32=0x1000; pub const HPC3_SPCFG_SWAP:u32=0x2000; pub const HPC3_SPCFG_EPAR:u32=0x4000; pub const HPC3_SPCFG_FUJI:u32=0x8000;

#[repr(C)]
pub struct hpc3_ethregs {
    pub rx_cbptr:u32, pub rx_ndptr:u32, pub _unused0:[u32;0x1000/4-2], pub rx_bcd:u32, pub rx_ctrl:u32, pub rx_gfptr:u32, pub rx_dfptr:u32, pub _unused1:u32, pub reset:u32, pub dconfig:u32, pub pconfig:u32, pub _unused2:[u32;0x1000/4-8],
    pub tx_cbptr:u32, pub tx_ndptr:u32, pub _unused3:[u32;0x1000/4-2], pub tx_bcd:u32, pub tx_ctrl:u32, pub tx_gfptr:u32, pub tx_dfptr:u32, pub _unused4:[u32;0x1000/4-4],
}
pub const HPC3_ERXBCD_BCNTMSK:u32=0x00003fff; pub const HPC3_ERXBCD_XIE:u32=0x20000000; pub const HPC3_ERXBCD_EOX:u32=0x80000000;
pub const HPC3_ERXCTRL_STAT50:u32=0x3f; pub const HPC3_ERXCTRL_STAT6:u32=0x40; pub const HPC3_ERXCTRL_STAT7:u32=0x80; pub const HPC3_ERXCTRL_ENDIAN:u32=0x100; pub const HPC3_ERXCTRL_ACTIVE:u32=0x200; pub const HPC3_ERXCTRL_AMASK:u32=0x400; pub const HPC3_ERXCTRL_RBO:u32=0x800;
pub const HPC3_ERST_CRESET:u32=1; pub const HPC3_ERST_CLRIRQ:u32=2; pub const HPC3_ERST_LBACK:u32=4;
pub const HPC3_EDCFG_D1:u32=0xf; pub const HPC3_EDCFG_D2:u32=0xf0; pub const HPC3_EDCFG_D3:u32=0xf00; pub const HPC3_EDCFG_WCTRL:u32=0x1000; pub const HPC3_EDCFG_FRXDC:u32=0x2000; pub const HPC3_EDCFG_FEOP:u32=0x4000; pub const HPC3_EDCFG_FIRQ:u32=0x8000; pub const HPC3_EDCFG_PTO:u32=0x30000;
pub const HPC3_EPCFG_P1:u32=0xf; pub const HPC3_EPCFG_P2:u32=0xf0; pub const HPC3_EPCFG_P3:u32=0xf00; pub const HPC3_EPCFG_TST:u32=0x1000;
pub const HPC3_ETXBCD_BCNTMSK:u32=0x3fff; pub const HPC3_ETXBCD_ESAMP:u32=0x10000000; pub const HPC3_ETXBCD_XIE:u32=0x20000000; pub const HPC3_ETXBCD_EOP:u32=0x40000000; pub const HPC3_ETXBCD_EOX:u32=0x80000000;
pub const HPC3_ETXCTRL_STAT30:u32=0xf; pub const HPC3_ETXCTRL_STAT4:u32=0x10; pub const HPC3_ETXCTRL_STAT75:u32=0xe0; pub const HPC3_ETXCTRL_ENDIAN:u32=0x100; pub const HPC3_ETXCTRL_ACTIVE:u32=0x200; pub const HPC3_ETXCTRL_AMASK:u32=0x400;

#[repr(C)]
pub struct hpc3_regs {
    pub pbdma: [hpc3_pbus_dmacregs; 8],
    pub scsi_chan0: hpc3_scsiregs,
    pub scsi_chan1: hpc3_scsiregs,
    pub ethregs: hpc3_ethregs,
    pub _unused0: [u32; 0x18000 / 4],
    pub istat0: u32,
    pub gio_misc: u32,
    pub eeprom: u32,
    pub istat1: u32,
    pub bestat: u32,
    pub _unused1: [u32; 0x14000 / 4 - 5],
    pub scsi0_ext: [u32; 256],
    pub _unused2: [u32; 0x7c00 / 4],
    pub scsi1_ext: [u32; 256],
    pub _unused3: [u32; 0x7c00 / 4],
    pub eth_ext: [u32; 320],
    pub _unused4: [u32; 0x3b00 / 4],
    pub pbus_extregs: [[u32; 256]; 16],
    pub pbus_dmacfg: [[u32; 128]; 8],
    pub pbus_piocfg: [[u32; 64]; 16],
    pub pbus_promwe: u32,
    pub _unused5: [u32; 0x0800 / 4 - 1],
    pub pbus_promswap: u32,
    pub _unused6: [u32; 0x0800 / 4 - 1],
    pub pbus_gout: u32,
    pub _unused7: [u32; 0x1000 / 4 - 1],
    pub rtcregs: [u32; 14],
    pub _unused8: [u32; 50],
    pub bbram: [u32; 8192 - 50 - 14],
}

pub const HPC3_ISTAT_PBIMASK:u32=0x0ff; pub const HPC3_ISTAT_SC0MASK:u32=0x100; pub const HPC3_ISTAT_SC1MASK:u32=0x200;
pub const HPC3_GIOMISC_ERTIME:u32=0x1; pub const HPC3_GIOMISC_DENDIAN:u32=0x2;
pub const HPC3_EEPROM_EPROT:u32=0x01; pub const HPC3_EEPROM_CSEL:u32=0x02; pub const HPC3_EEPROM_ECLK:u32=0x04; pub const HPC3_EEPROM_DATO:u32=0x08; pub const HPC3_EEPROM_DATI:u32=0x10;
pub const HPC3_BESTAT_BLMASK:u32=0x000ff; pub const HPC3_BESTAT_CTYPE:u32=0x00100; pub const HPC3_BESTAT_PIDSHIFT:u32=9; pub const HPC3_BESTAT_PIDMASK:u32=0x3f700;
pub const HPC3_DMACFG_D3R_MASK:u32=0x00000001; pub const HPC3_DMACFG_D3R_SHIFT:u32=0; pub const HPC3_DMACFG_D4R_MASK:u32=0x0000001e; pub const HPC3_DMACFG_D4R_SHIFT:u32=1; pub const HPC3_DMACFG_D5R_MASK:u32=0x000001e0; pub const HPC3_DMACFG_D5R_SHIFT:u32=5; pub const HPC3_DMACFG_D3W_MASK:u32=0x00000200; pub const HPC3_DMACFG_D3W_SHIFT:u32=9; pub const HPC3_DMACFG_D4W_MASK:u32=0x00003c00; pub const HPC3_DMACFG_D4W_SHIFT:u32=10; pub const HPC3_DMACFG_D5W_MASK:u32=0x0003c000; pub const HPC3_DMACFG_D5W_SHIFT:u32=14; pub const HPC3_DMACFG_DS16:u32=0x00040000; pub const HPC3_DMACFG_EVENHI:u32=0x00080000; pub const HPC3_DMACFG_RTIME:u32=0x00200000; pub const HPC3_DMACFG_BURST_MASK:u32=0x07c00000; pub const HPC3_DMACFG_BURST_SHIFT:u32=22; pub const HPC3_DMACFG_DRQLIVE:u32=0x08000000;
pub const HPC3_PIOCFG_P2R_MASK:u32=1; pub const HPC3_PIOCFG_P2R_SHIFT:u32=0; pub const HPC3_PIOCFG_P3R_MASK:u32=0x1e; pub const HPC3_PIOCFG_P3R_SHIFT:u32=1; pub const HPC3_PIOCFG_P4R_MASK:u32=0x1e0; pub const HPC3_PIOCFG_P4R_SHIFT:u32=5; pub const HPC3_PIOCFG_P2W_MASK:u32=0x200; pub const HPC3_PIOCFG_P2W_SHIFT:u32=9; pub const HPC3_PIOCFG_P3W_MASK:u32=0x3c00; pub const HPC3_PIOCFG_P3W_SHIFT:u32=10; pub const HPC3_PIOCFG_P4W_MASK:u32=0x3c000; pub const HPC3_PIOCFG_P4W_SHIFT:u32=14; pub const HPC3_PIOCFG_DS16:u32=0x40000; pub const HPC3_PIOCFG_EVENHI:u32=0x80000;
pub const HPC3_PROM_WENAB:u32=1; pub const HPC3_PROM_SWAP:u32=1; pub const HPC3_PROM_STAT:u32=1;
pub const HPC3_CHIP0_BASE:u32=0x1fb80000; pub const HPC3_CHIP1_BASE:u32=0x1fb00000;

extern "C" {
    pub static mut hpc3c0: *mut hpc3_regs;
    pub static mut hpc3c1: *mut hpc3_regs;
    pub fn sgihpc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
