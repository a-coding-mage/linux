// SPDX-License-Identifier: GPL-2.0+
//
// siu.h - ALSA SoC driver for Renesas SH7343, SH7722 SIU peripheral.
//
// Copyright (C) 2009-2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
// Copyright (C) 2006 Carlos Munoz <carlos@kenati.com>

/* Common kernel and user-space firmware-building defines and types */

pub const YRAM0_SIZE: usize = 0x0040 / 4; /* 16 */
pub const YRAM1_SIZE: usize = 0x0080 / 4; /* 32 */
pub const YRAM2_SIZE: usize = 0x0040 / 4; /* 16 */
pub const YRAM3_SIZE: usize = 0x0080 / 4; /* 32 */
pub const YRAM4_SIZE: usize = 0x0080 / 4; /* 32 */
pub const YRAM_DEF_SIZE: usize =
    YRAM0_SIZE + YRAM1_SIZE + YRAM2_SIZE + YRAM3_SIZE + YRAM4_SIZE;
pub const YRAM_FIR_SIZE: usize = 0x0400 / 4; /* 256 */
pub const YRAM_IIR_SIZE: usize = 0x0200 / 4; /* 128 */

pub const XRAM0_SIZE: usize = 0x0400 / 4; /* 256 */
pub const XRAM1_SIZE: usize = 0x0200 / 4; /* 128 */
pub const XRAM2_SIZE: usize = 0x0200 / 4; /* 128 */

/* PRAM program array size */
pub const PRAM0_SIZE: usize = 0x0100 / 4; /* 64 */
pub const PRAM1_SIZE: usize = (0x2000 - 0x0100) / 4; /* 1984 */

pub type __u32 = u32;

#[repr(C)]
pub struct siu_spb_param {
    pub ab1a: __u32,  /* input FIFO address */
    pub ab0a: __u32,  /* output FIFO address */
    pub dir: __u32,   /* 0=the ather except CPUOUTPUT, 1=CPUINPUT */
    pub event: __u32, /* SPB program starting conditions */
    pub stfifo: __u32, /* STFIFO register setting value */
    pub trdat: __u32, /* TRDAT register setting value */
}

#[repr(C)]
pub struct siu_firmware {
    pub yram_fir_coeff: [__u32; YRAM_FIR_SIZE],
    pub pram0: [__u32; PRAM0_SIZE],
    pub pram1: [__u32; PRAM1_SIZE],
    pub yram0: [__u32; YRAM0_SIZE],
    pub yram1: [__u32; YRAM1_SIZE],
    pub yram2: [__u32; YRAM2_SIZE],
    pub yram3: [__u32; YRAM3_SIZE],
    pub yram4: [__u32; YRAM4_SIZE],
    pub spbpar_num: __u32,
    pub spbpar: [siu_spb_param; 32],
}

/* Original C condition: #ifdef __KERNEL__ */

pub const SIU_PERIOD_BYTES_MAX: usize = 8192; /* DMA transfer/period size */
pub const SIU_PERIOD_BYTES_MIN: usize = 256; /* DMA transfer/period size */
pub const SIU_PERIODS_MAX: usize = 64; /* Max periods in buffer */
pub const SIU_PERIODS_MIN: usize = 4; /* Min periods in buffer */
pub const SIU_BUFFER_BYTES_MAX: usize = SIU_PERIOD_BYTES_MAX * SIU_PERIODS_MAX;

/* SIU ports: only one can be used at a time */
pub const SIU_PORT_A: usize = 0;
pub const SIU_PORT_B: usize = 1;
pub const SIU_PORT_NUM: usize = 2;

/* SIU clock configuration */
pub const SIU_CLKA_PLL: usize = 0;
pub const SIU_CLKA_EXT: usize = 1;
pub const SIU_CLKB_PLL: usize = 2;
pub const SIU_CLKB_EXT: usize = 3;

#[repr(C)]
pub struct siu_info {
    pub dev: *mut device,
    pub port_id: ::core::ffi::c_int,
    pub pram: *mut u32,
    pub xram: *mut u32,
    pub yram: *mut u32,
    pub reg: *mut u32,
    pub fw: siu_firmware,
}

#[repr(C)]
pub struct siu_stream {
    pub work: work_struct,
    pub substream: *mut snd_pcm_substream,
    pub format: snd_pcm_format_t,
    pub buf_bytes: usize,
    pub period_bytes: usize,
    pub cur_period: ::core::ffi::c_int, /* Period currently in dma */
    pub volume: u32,
    pub xfer_cnt: snd_pcm_sframes_t, /* Number of frames */
    pub rw_flg: u8,                  /* transfer status */
    /* DMA status */
    pub chan: *mut dma_chan, /* DMA channel */
    pub tx_desc: *mut dma_async_tx_descriptor,
    pub cookie: dma_cookie_t,
    pub param: sh_dmae_slave,
}

#[repr(C)]
pub struct siu_port {
    pub play_cap: ::core::ffi::c_ulong, /* Used to track full duplex */
    pub pcm: *mut snd_pcm,
    pub playback: siu_stream,
    pub capture: siu_stream,
    pub stfifo: u32, /* STFIFO value from firmware */
    pub trdat: u32,  /* TRDAT value from firmware */
}

unsafe extern "C" {
    pub static mut siu_ports: [*mut siu_port; SIU_PORT_NUM];
}

pub unsafe fn siu_port_info(substream: *mut snd_pcm_substream) -> *mut siu_port {
    let pdev: *mut platform_device = unsafe {
        to_platform_device((*(*(*substream).pcm).card).dev)
    };
    unsafe { siu_ports[(*pdev).id as usize] }
}

/* Register access */
pub unsafe fn siu_write32(addr: *mut u32, val: u32) {
    unsafe {
        __raw_writel(val, addr);
    }
}

pub unsafe fn siu_read32(addr: *mut u32) -> u32 {
    unsafe { __raw_readl(addr) }
}

/* SIU registers */
pub const SIU_IFCTL: usize = 0x000 / ::core::mem::size_of::<u32>();
pub const SIU_SRCTL: usize = 0x004 / ::core::mem::size_of::<u32>();
pub const SIU_SFORM: usize = 0x008 / ::core::mem::size_of::<u32>();
pub const SIU_CKCTL: usize = 0x00c / ::core::mem::size_of::<u32>();
pub const SIU_TRDAT: usize = 0x010 / ::core::mem::size_of::<u32>();
pub const SIU_STFIFO: usize = 0x014 / ::core::mem::size_of::<u32>();
pub const SIU_DPAK: usize = 0x01c / ::core::mem::size_of::<u32>();
pub const SIU_CKREV: usize = 0x020 / ::core::mem::size_of::<u32>();
pub const SIU_EVNTC: usize = 0x028 / ::core::mem::size_of::<u32>();
pub const SIU_SBCTL: usize = 0x040 / ::core::mem::size_of::<u32>();
pub const SIU_SBPSET: usize = 0x044 / ::core::mem::size_of::<u32>();
pub const SIU_SBFSTS: usize = 0x068 / ::core::mem::size_of::<u32>();
pub const SIU_SBDVCA: usize = 0x06c / ::core::mem::size_of::<u32>();
pub const SIU_SBDVCB: usize = 0x070 / ::core::mem::size_of::<u32>();
pub const SIU_SBACTIV: usize = 0x074 / ::core::mem::size_of::<u32>();
pub const SIU_DMAIA: usize = 0x090 / ::core::mem::size_of::<u32>();
pub const SIU_DMAIB: usize = 0x094 / ::core::mem::size_of::<u32>();
pub const SIU_DMAOA: usize = 0x098 / ::core::mem::size_of::<u32>();
pub const SIU_DMAOB: usize = 0x09c / ::core::mem::size_of::<u32>();
pub const SIU_DMAML: usize = 0x0a0 / ::core::mem::size_of::<u32>();
pub const SIU_SPSTS: usize = 0x0cc / ::core::mem::size_of::<u32>();
pub const SIU_SPCTL: usize = 0x0d0 / ::core::mem::size_of::<u32>();
pub const SIU_BRGASEL: usize = 0x100 / ::core::mem::size_of::<u32>();
pub const SIU_BRRA: usize = 0x104 / ::core::mem::size_of::<u32>();
pub const SIU_BRGBSEL: usize = 0x108 / ::core::mem::size_of::<u32>();
pub const SIU_BRRB: usize = 0x10c / ::core::mem::size_of::<u32>();

unsafe extern "C" {
    pub static siu_component: snd_soc_component_driver;
    pub static mut siu_i2s_data: *mut siu_info;

    pub fn siu_init_port(
        port: ::core::ffi::c_int,
        port_info: *mut *mut siu_port,
        card: *mut snd_card,
    ) -> ::core::ffi::c_int;
    pub fn siu_free_port(port_info: *mut siu_port);

    pub fn to_platform_device(dev: *mut device) -> *mut platform_device;
    pub fn __raw_writel(val: u32, addr: *mut u32);
    pub fn __raw_readl(addr: *mut u32) -> u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
