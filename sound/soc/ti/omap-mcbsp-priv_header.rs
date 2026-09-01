/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OMAP Multi-Channel Buffered Serial Port
 *
 * Contact: Jarkko Nikula <jarkko.nikula@bitmer.com>
 *          Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

/* Depends on linux/platform_data/asoc-ti-mcbsp.h. */

use core::ffi::{c_int, c_ulong, c_void};

pub type u8 = crate::u8;
pub type u16 = crate::u16;
pub type u32 = crate::u32;
pub type bool = crate::bool;
pub type spinlock_t = crate::spinlock_t;
pub type device = crate::device;
pub type clk = crate::clk;
pub type omap_mcbsp_platform_data = crate::omap_mcbsp_platform_data;
pub type snd_dmaengine_dai_dma_data = crate::snd_dmaengine_dai_dma_data;
pub type pm_qos_request = crate::pm_qos_request;
pub type platform_device = crate::platform_device;

/* CONFIG_ARCH_OMAP1 selects whether mcbsp_omap1() returns 1 or 0 in C. */
#[cfg(CONFIG_ARCH_OMAP1)]
pub const fn mcbsp_omap1() -> c_int {
    1
}

#[cfg(not(CONFIG_ARCH_OMAP1))]
pub const fn mcbsp_omap1() -> c_int {
    0
}

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

/* McBSP register numbers. Register address offset = num * reg_step */
pub const OMAP_MCBSP_REG_SPCR2: u16 = 4;
pub const OMAP_MCBSP_REG_SPCR1: u16 = 5;
pub const OMAP_MCBSP_REG_RCR2: u16 = 6;
pub const OMAP_MCBSP_REG_RCR1: u16 = 7;
pub const OMAP_MCBSP_REG_XCR2: u16 = 8;
pub const OMAP_MCBSP_REG_XCR1: u16 = 9;
pub const OMAP_MCBSP_REG_SRGR2: u16 = 10;
pub const OMAP_MCBSP_REG_SRGR1: u16 = 11;
pub const OMAP_MCBSP_REG_MCR2: u16 = 12;
pub const OMAP_MCBSP_REG_MCR1: u16 = 13;
pub const OMAP_MCBSP_REG_RCERA: u16 = 14;
pub const OMAP_MCBSP_REG_RCERB: u16 = 15;
pub const OMAP_MCBSP_REG_XCERA: u16 = 16;
pub const OMAP_MCBSP_REG_XCERB: u16 = 17;
pub const OMAP_MCBSP_REG_PCR0: u16 = 18;
pub const OMAP_MCBSP_REG_RCERC: u16 = 19;
pub const OMAP_MCBSP_REG_RCERD: u16 = 20;
pub const OMAP_MCBSP_REG_XCERC: u16 = 21;
pub const OMAP_MCBSP_REG_XCERD: u16 = 22;
pub const OMAP_MCBSP_REG_RCERE: u16 = 23;
pub const OMAP_MCBSP_REG_RCERF: u16 = 24;
pub const OMAP_MCBSP_REG_XCERE: u16 = 25;
pub const OMAP_MCBSP_REG_XCERF: u16 = 26;
pub const OMAP_MCBSP_REG_RCERG: u16 = 27;
pub const OMAP_MCBSP_REG_RCERH: u16 = 28;
pub const OMAP_MCBSP_REG_XCERG: u16 = 29;
pub const OMAP_MCBSP_REG_XCERH: u16 = 30;

/* OMAP1-OMAP2420 registers */
pub const OMAP_MCBSP_REG_DRR2: u16 = 0;
pub const OMAP_MCBSP_REG_DRR1: u16 = 1;
pub const OMAP_MCBSP_REG_DXR2: u16 = 2;
pub const OMAP_MCBSP_REG_DXR1: u16 = 3;

/* OMAP2430 and onwards */
pub const OMAP_MCBSP_REG_DRR: u16 = 0;
pub const OMAP_MCBSP_REG_DXR: u16 = 2;
pub const OMAP_MCBSP_REG_SYSCON: u16 = 35;
pub const OMAP_MCBSP_REG_THRSH2: u16 = 36;
pub const OMAP_MCBSP_REG_THRSH1: u16 = 37;
pub const OMAP_MCBSP_REG_IRQST: u16 = 40;
pub const OMAP_MCBSP_REG_IRQEN: u16 = 41;
pub const OMAP_MCBSP_REG_WAKEUPEN: u16 = 42;
pub const OMAP_MCBSP_REG_XCCR: u16 = 43;
pub const OMAP_MCBSP_REG_RCCR: u16 = 44;
pub const OMAP_MCBSP_REG_XBUFFSTAT: u16 = 45;
pub const OMAP_MCBSP_REG_RBUFFSTAT: u16 = 46;
pub const OMAP_MCBSP_REG_SSELCR: u16 = 47;

/************************** McBSP SPCR1 bit definitions ***********************/
pub const RRST: u32 = BIT(0);
pub const RRDY: u32 = BIT(1);
pub const RFULL: u32 = BIT(2);
pub const RSYNC_ERR: u32 = BIT(3);
pub const fn RINTM(value: u32) -> u32 {
    ((value) & 0x3) << 4
} /* bits 4:5 */
pub const ABIS: u32 = BIT(6);
pub const DXENA: u32 = BIT(7);
pub const fn CLKSTP(value: u32) -> u32 {
    ((value) & 0x3) << 11
} /* bits 11:12 */
pub const fn RJUST(value: u32) -> u32 {
    ((value) & 0x3) << 13
} /* bits 13:14 */
pub const ALB: u32 = BIT(15);
pub const DLB: u32 = BIT(15);

/************************** McBSP SPCR2 bit definitions ***********************/
pub const XRST: u32 = BIT(0);
pub const XRDY: u32 = BIT(1);
pub const XEMPTY: u32 = BIT(2);
pub const XSYNC_ERR: u32 = BIT(3);
pub const fn XINTM(value: u32) -> u32 {
    ((value) & 0x3) << 4
} /* bits 4:5 */
pub const GRST: u32 = BIT(6);
pub const FRST: u32 = BIT(7);
pub const SOFT: u32 = BIT(8);
pub const FREE: u32 = BIT(9);

/************************** McBSP PCR bit definitions *************************/
pub const CLKRP: u32 = BIT(0);
pub const CLKXP: u32 = BIT(1);
pub const FSRP: u32 = BIT(2);
pub const FSXP: u32 = BIT(3);
pub const DR_STAT: u32 = BIT(4);
pub const DX_STAT: u32 = BIT(5);
pub const CLKS_STAT: u32 = BIT(6);
pub const SCLKME: u32 = BIT(7);
pub const CLKRM: u32 = BIT(8);
pub const CLKXM: u32 = BIT(9);
pub const FSRM: u32 = BIT(10);
pub const FSXM: u32 = BIT(11);
pub const RIOEN: u32 = BIT(12);
pub const XIOEN: u32 = BIT(13);
pub const IDLE_EN: u32 = BIT(14);

/************************** McBSP RCR1 bit definitions ************************/
pub const fn RWDLEN1(value: u32) -> u32 {
    ((value) & 0x7) << 5
} /* Bits 5:7 */
pub const fn RFRLEN1(value: u32) -> u32 {
    ((value) & 0x7f) << 8
} /* Bits 8:14 */

/************************** McBSP XCR1 bit definitions ************************/
pub const fn XWDLEN1(value: u32) -> u32 {
    ((value) & 0x7) << 5
} /* Bits 5:7 */
pub const fn XFRLEN1(value: u32) -> u32 {
    ((value) & 0x7f) << 8
} /* Bits 8:14 */

/*************************** McBSP RCR2 bit definitions ***********************/
pub const fn RDATDLY(value: u32) -> u32 {
    (value) & 0x3
} /* Bits 0:1 */
pub const RFIG: u32 = BIT(2);
pub const fn RCOMPAND(value: u32) -> u32 {
    ((value) & 0x3) << 3
} /* Bits 3:4 */
pub const fn RWDLEN2(value: u32) -> u32 {
    ((value) & 0x7) << 5
} /* Bits 5:7 */
pub const fn RFRLEN2(value: u32) -> u32 {
    ((value) & 0x7f) << 8
} /* Bits 8:14 */
pub const RPHASE: u32 = BIT(15);

/*************************** McBSP XCR2 bit definitions ***********************/
pub const fn XDATDLY(value: u32) -> u32 {
    (value) & 0x3
} /* Bits 0:1 */
pub const XFIG: u32 = BIT(2);
pub const fn XCOMPAND(value: u32) -> u32 {
    ((value) & 0x3) << 3
} /* Bits 3:4 */
pub const fn XWDLEN2(value: u32) -> u32 {
    ((value) & 0x7) << 5
} /* Bits 5:7 */
pub const fn XFRLEN2(value: u32) -> u32 {
    ((value) & 0x7f) << 8
} /* Bits 8:14 */
pub const XPHASE: u32 = BIT(15);

/************************* McBSP SRGR1 bit definitions ************************/
pub const fn CLKGDV(value: u32) -> u32 {
    (value) & 0x7f
} /* Bits 0:7 */
pub const fn FWID(value: u32) -> u32 {
    ((value) & 0xff) << 8
} /* Bits 8:15 */

/************************* McBSP SRGR2 bit definitions ************************/
pub const fn FPER(value: u32) -> u32 {
    (value) & 0x0fff
} /* Bits 0:11 */
pub const FSGM: u32 = BIT(12);
pub const CLKSM: u32 = BIT(13);
pub const CLKSP: u32 = BIT(14);
pub const GSYNC: u32 = BIT(15);

/************************* McBSP MCR1 bit definitions *************************/
pub const RMCM: u32 = BIT(0);
pub const fn RCBLK(value: u32) -> u32 {
    ((value) & 0x7) << 2
} /* Bits 2:4 */
pub const fn RPABLK(value: u32) -> u32 {
    ((value) & 0x3) << 5
} /* Bits 5:6 */
pub const fn RPBBLK(value: u32) -> u32 {
    ((value) & 0x3) << 7
} /* Bits 7:8 */

/************************* McBSP MCR2 bit definitions *************************/
pub const fn XMCM(value: u32) -> u32 {
    (value) & 0x3
} /* Bits 0:1 */
pub const fn XCBLK(value: u32) -> u32 {
    ((value) & 0x7) << 2
} /* Bits 2:4 */
pub const fn XPABLK(value: u32) -> u32 {
    ((value) & 0x3) << 5
} /* Bits 5:6 */
pub const fn XPBBLK(value: u32) -> u32 {
    ((value) & 0x3) << 7
} /* Bits 7:8 */

/*********************** McBSP XCCR bit definitions *************************/
pub const XDISABLE: u32 = BIT(0);
pub const XDMAEN: u32 = BIT(3);
pub const DILB: u32 = BIT(5);
pub const XFULL_CYCLE: u32 = BIT(11);
pub const fn DXENDLY(value: u32) -> u32 {
    ((value) & 0x3) << 12
} /* Bits 12:13 */
pub const PPCONNECT: u32 = BIT(14);
pub const EXTCLKGATE: u32 = BIT(15);

/********************** McBSP RCCR bit definitions *************************/
pub const RDISABLE: u32 = BIT(0);
pub const RDMAEN: u32 = BIT(3);
pub const RFULL_CYCLE: u32 = BIT(11);

/********************** McBSP SYSCONFIG bit definitions ********************/
pub const SOFTRST: u32 = BIT(1);
pub const ENAWAKEUP: u32 = BIT(2);
pub const fn SIDLEMODE(value: u32) -> u32 {
    ((value) & 0x3) << 3
}
pub const fn CLOCKACTIVITY(value: u32) -> u32 {
    ((value) & 0x3) << 8
}

/********************** McBSP DMA operating modes **************************/
pub const MCBSP_DMA_MODE_ELEMENT: c_int = 0;
pub const MCBSP_DMA_MODE_THRESHOLD: c_int = 1;

/********************** McBSP WAKEUPEN/IRQST/IRQEN bit definitions *********/
pub const RSYNCERREN: u32 = BIT(0);
pub const RFSREN: u32 = BIT(1);
pub const REOFEN: u32 = BIT(2);
pub const RRDYEN: u32 = BIT(3);
pub const RUNDFLEN: u32 = BIT(4);
pub const ROVFLEN: u32 = BIT(5);
pub const XSYNCERREN: u32 = BIT(7);
pub const XFSXEN: u32 = BIT(8);
pub const XEOFEN: u32 = BIT(9);
pub const XRDYEN: u32 = BIT(10);
pub const XUNDFLEN: u32 = BIT(11);
pub const XOVFLEN: u32 = BIT(12);
pub const XEMPTYEOFEN: u32 = BIT(14);

/* Clock signal muxing options */
pub const CLKR_SRC_CLKR: c_int = 0; /* CLKR signal is from the CLKR pin */
pub const CLKR_SRC_CLKX: c_int = 1; /* CLKR signal is from the CLKX pin */
pub const FSR_SRC_FSR: c_int = 2; /* FSR signal is from the FSR pin */
pub const FSR_SRC_FSX: c_int = 3; /* FSR signal is from the FSX pin */

/* McBSP functional clock sources */
pub const MCBSP_CLKS_PRCM_SRC: c_int = 0;
pub const MCBSP_CLKS_PAD_SRC: c_int = 1;

/* we don't do multichannel for now */
#[repr(C)]
pub struct omap_mcbsp_reg_cfg {
    pub spcr2: u16,
    pub spcr1: u16,
    pub rcr2: u16,
    pub rcr1: u16,
    pub xcr2: u16,
    pub xcr1: u16,
    pub srgr2: u16,
    pub srgr1: u16,
    pub mcr2: u16,
    pub mcr1: u16,
    pub pcr0: u16,
    pub rcerc: u16,
    pub rcerd: u16,
    pub xcerc: u16,
    pub xcerd: u16,
    pub rcere: u16,
    pub rcerf: u16,
    pub xcere: u16,
    pub xcerf: u16,
    pub rcerg: u16,
    pub rcerh: u16,
    pub xcerg: u16,
    pub xcerh: u16,
    pub xccr: u16,
    pub rccr: u16,
}

#[repr(C)]
pub struct omap_mcbsp_st_data {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct omap_mcbsp {
    pub dev: *mut device,
    pub fclk: *mut clk,
    pub lock: spinlock_t,
    pub phys_base: c_ulong,
    pub phys_dma_base: c_ulong,
    pub io_base: *mut c_void,
    pub id: u8,
    /*
     * Flags indicating is the bus already activated and configured by
     * another substream
     */
    pub active: c_int,
    pub configured: c_int,
    pub free: u8,

    pub irq: c_int,
    pub rx_irq: c_int,
    pub tx_irq: c_int,

    /* Protect the field .free, while checking if the mcbsp is in use */
    pub pdata: *mut omap_mcbsp_platform_data,
    pub st_data: *mut omap_mcbsp_st_data,
    pub cfg_regs: omap_mcbsp_reg_cfg,
    pub dma_data: [snd_dmaengine_dai_dma_data; 2],
    pub dma_req: [u32; 2],
    pub dma_op_mode: c_int,
    pub max_tx_thres: u16,
    pub max_rx_thres: u16,
    pub reg_cache: *mut c_void,
    pub reg_cache_size: c_int,

    pub fmt: u32,
    pub in_freq: u32,
    pub latency: [u32; 2],
    pub clk_div: c_int,
    pub wlen: c_int,

    pub pm_qos_req: pm_qos_request,
}

extern "C" {
    pub fn writew_relaxed(val: u16, addr: *mut c_void);
    pub fn writel_relaxed(val: u32, addr: *mut c_void);
    pub fn readw_relaxed(addr: *mut c_void) -> u16;
    pub fn readl_relaxed(addr: *mut c_void) -> u32;
}

pub unsafe fn omap_mcbsp_write(mcbsp: *mut omap_mcbsp, reg: u16, val: u32) {
    let addr = ((*mcbsp).io_base as *mut u8).add(
        (reg as usize).wrapping_mul((*(*mcbsp).pdata).reg_step as usize),
    ) as *mut c_void;

    if (*(*mcbsp).pdata).reg_size == 2 {
        *((*mcbsp).reg_cache as *mut u16).add(reg as usize) = val as u16;
        writew_relaxed(val as u16, addr);
    } else {
        *((*mcbsp).reg_cache as *mut u32).add(reg as usize) = val;
        writel_relaxed(val, addr);
    }
}

pub unsafe fn omap_mcbsp_read(mcbsp: *mut omap_mcbsp, reg: u16, from_cache: bool) -> c_int {
    let addr = ((*mcbsp).io_base as *mut u8).add(
        (reg as usize).wrapping_mul((*(*mcbsp).pdata).reg_step as usize),
    ) as *mut c_void;

    if (*(*mcbsp).pdata).reg_size == 2 {
        if !from_cache {
            readw_relaxed(addr) as c_int
        } else {
            *((*mcbsp).reg_cache as *mut u16).add(reg as usize) as c_int
        }
    } else {
        if !from_cache {
            readl_relaxed(addr) as c_int
        } else {
            *((*mcbsp).reg_cache as *mut u32).add(reg as usize) as c_int
        }
    }
}

pub unsafe fn MCBSP_READ(mcbsp: *mut omap_mcbsp, reg: u16) -> c_int {
    omap_mcbsp_read(mcbsp, reg, false)
}

pub unsafe fn MCBSP_WRITE(mcbsp: *mut omap_mcbsp, reg: u16, val: u32) {
    omap_mcbsp_write(mcbsp, reg, val);
}

pub unsafe fn MCBSP_READ_CACHE(mcbsp: *mut omap_mcbsp, reg: u16) -> c_int {
    omap_mcbsp_read(mcbsp, reg, true)
}

/* Sidetone specific API */
extern "C" {
    pub fn omap_mcbsp_st_init(pdev: *mut platform_device) -> c_int;
    pub fn omap_mcbsp_st_start(mcbsp: *mut omap_mcbsp) -> c_int;
    pub fn omap_mcbsp_st_stop(mcbsp: *mut omap_mcbsp) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
