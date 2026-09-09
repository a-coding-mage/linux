/* SPDX-License-Identifier: GPL-2.0-only */

/* Hardware driver for NI Mite PCI interface chip. */

use core::ffi::c_void;

/* C dependency: linux/pci.h. */
#[repr(C)]
pub struct pci_dev {
    pub irq: u32,
    pub device: u16,
}

pub const PCI_VENDOR_ID_NATINST: u32 = 0x1093;
pub const MITE_RING_SIZE: usize = 3000;

const fn bit(n: u32) -> u32 { 1u32 << n }

#[repr(C)]
pub struct mite_dma_chain {
    pub count: u32,
    pub addr: u32,
    pub next: u32,
}

#[repr(C)]
pub struct mite_struct {
    pub next: *mut mite_struct,
    pub used: i32,
    pub pcidev: *mut pci_dev,
    pub mite_phys_addr: usize,
    pub mite_io_addr: *mut c_void,
    pub daq_phys_addr: usize,
    pub daq_io_addr: *mut c_void,
    pub DMA_CheckNearEnd: i32,
    pub ring: [mite_dma_chain; MITE_RING_SIZE],
}

extern "C" {
    pub static mut mite_devices: *mut mite_struct;
    pub fn mite_init();
    pub fn mite_cleanup();
    pub fn mite_setup(mite: *mut mite_struct) -> i32;
    pub fn mite_unsetup(mite: *mut mite_struct);
    pub fn mite_list_devices();
}

#[inline]
pub unsafe fn mite_irq(mite: *mut mite_struct) -> u32 { (*(*mite).pcidev).irq }

#[inline]
pub unsafe fn mite_device_id(mite: *mut mite_struct) -> u32 { (*(*mite).pcidev).device as u32 }

pub const fn CHAN_OFFSET(x: u32) -> u32 { 0x100 * x }
pub const MITE_CHOR: u32 = 0x500;
pub const CHOR_DMARESET: u32 = bit(31);
pub const CHOR_SET_SEND_TC: u32 = bit(11);
pub const CHOR_CLR_SEND_TC: u32 = bit(10);
pub const CHOR_SET_LPAUSE: u32 = bit(9);
pub const CHOR_CLR_LPAUSE: u32 = bit(8);
pub const CHOR_CLRDONE: u32 = bit(7);
pub const CHOR_CLRRB: u32 = bit(6);
pub const CHOR_CLRLC: u32 = bit(5);
pub const CHOR_FRESET: u32 = bit(4);
pub const CHOR_ABORT: u32 = bit(3);
pub const CHOR_STOP: u32 = bit(2);
pub const CHOR_CONT: u32 = bit(1);
pub const CHOR_START: u32 = bit(0);
pub const CHOR_PON: u32 = CHOR_CLR_SEND_TC | CHOR_CLR_LPAUSE;

pub const MITE_CHCR: u32 = 0x504;
pub const CHCR_SET_DMA_IE: u32 = bit(31);
pub const CHCR_CLR_DMA_IE: u32 = bit(30);
pub const CHCR_SET_LINKP_IE: u32 = bit(29);
pub const CHCR_CLR_LINKP_IE: u32 = bit(28);
pub const CHCR_SET_SAR_IE: u32 = bit(27);
pub const CHCR_CLR_SAR_IE: u32 = bit(26);
pub const CHCR_SET_DONE_IE: u32 = bit(25);
pub const CHCR_CLR_DONE_IE: u32 = bit(24);
pub const CHCR_SET_MRDY_IE: u32 = bit(23);
pub const CHCR_CLR_MRDY_IE: u32 = bit(22);
pub const CHCR_SET_DRDY_IE: u32 = bit(21);
pub const CHCR_CLR_DRDY_IE: u32 = bit(20);
pub const CHCR_SET_LC_IE: u32 = bit(19);
pub const CHCR_CLR_LC_IE: u32 = bit(18);
pub const CHCR_SET_CONT_RB_IE: u32 = bit(17);
pub const CHCR_CLR_CONT_RB_IE: u32 = bit(16);
pub const CHCR_FIFODIS: u32 = bit(15);
pub const CHCR_FIFO_ON: u32 = 0;
pub const CHCR_BURSTEN: u32 = bit(14);
pub const CHCR_NO_BURSTEN: u32 = 0;
pub const fn CHCR_NFTP(x: u32) -> u32 { x << 11 }
pub const CHCR_NFTP0: u32 = CHCR_NFTP(0); pub const CHCR_NFTP1: u32 = CHCR_NFTP(1); pub const CHCR_NFTP2: u32 = CHCR_NFTP(2); pub const CHCR_NFTP4: u32 = CHCR_NFTP(3); pub const CHCR_NFTP8: u32 = CHCR_NFTP(4); pub const CHCR_NFTP16: u32 = CHCR_NFTP(5);
pub const fn CHCR_NETP(x: u32) -> u32 { x << 11 }
pub const CHCR_NETP0: u32 = CHCR_NETP(0); pub const CHCR_NETP1: u32 = CHCR_NETP(1); pub const CHCR_NETP2: u32 = CHCR_NETP(2); pub const CHCR_NETP4: u32 = CHCR_NETP(3); pub const CHCR_NETP8: u32 = CHCR_NETP(4);
pub const CHCR_CHEND1: u32 = bit(5); pub const CHCR_CHEND0: u32 = bit(4); pub const CHCR_DIR: u32 = bit(3); pub const CHCR_DEV_TO_MEM: u32 = CHCR_DIR; pub const CHCR_MEM_TO_DEV: u32 = 0;
pub const CHCR_NORMAL: u32 = 0; pub const CHCR_CONTINUE: u32 = 1; pub const CHCR_RINGBUFF: u32 = 2; pub const CHCR_LINKSHORT: u32 = 4; pub const CHCR_LINKLONG: u32 = 5;
/* The source refers to CHCR_CLR_CONT_IE, which is not defined in this header. */
pub const CHCRPON: u32 = CHCR_CLR_DMA_IE | CHCR_CLR_LINKP_IE | CHCR_CLR_SAR_IE | CHCR_CLR_DONE_IE | CHCR_CLR_MRDY_IE | CHCR_CLR_DRDY_IE | CHCR_CLR_LC_IE | 0;

pub const MITE_TCR: u32 = 0x508;

pub const fn CR_RL(x: u32) -> u32 { x << 21 }
pub const CR_RL0:u32=CR_RL(0); pub const CR_RL1:u32=CR_RL(1); pub const CR_RL2:u32=CR_RL(2); pub const CR_RL4:u32=CR_RL(3); pub const CR_RL8:u32=CR_RL(4); pub const CR_RL16:u32=CR_RL(5); pub const CR_RL32:u32=CR_RL(6); pub const CR_RL64:u32=CR_RL(7);
pub const fn CR_RD(x:u32)->u32{x<<19} pub const CR_RD0:u32=CR_RD(0); pub const CR_RD32:u32=CR_RD(1); pub const CR_RD512:u32=CR_RD(2); pub const CR_RD8192:u32=CR_RD(3);
pub const fn CR_REQS(x:u32)->u32{x<<16} pub const CR_REQSDRQ0:u32=CR_REQS(4); pub const CR_REQSDRQ1:u32=CR_REQS(5); pub const CR_REQSDRQ2:u32=CR_REQS(6); pub const CR_REQSDRQ3:u32=CR_REQS(7);
pub const fn CR_ASEQX(x:u32)->u32{x<<10} pub const CR_ASEQX0:u32=CR_ASEQX(0); pub const CR_ASEQDONT:u32=CR_ASEQX0; pub const CR_ASEQXP1:u32=CR_ASEQX(1); pub const CR_ASEQUP:u32=CR_ASEQXP1; pub const CR_ASEQXP2:u32=CR_ASEQX(2); pub const CR_ASEQDOWN:u32=CR_ASEQXP2; pub const CR_ASEQXP4:u32=CR_ASEQX(3); pub const CR_ASEQXP8:u32=CR_ASEQX(4); pub const CR_ASEQXP16:u32=CR_ASEQX(5); pub const CR_ASEQXP32:u32=CR_ASEQX(6); pub const CR_ASEQXP64:u32=CR_ASEQX(7); pub const CR_ASEQXM1:u32=CR_ASEQX(9); pub const CR_ASEQXM2:u32=CR_ASEQX(10); pub const CR_ASEQXM4:u32=CR_ASEQX(11); pub const CR_ASEQXM8:u32=CR_ASEQX(12); pub const CR_ASEQXM16:u32=CR_ASEQX(13); pub const CR_ASEQXM32:u32=CR_ASEQX(14); pub const CR_ASEQXM64:u32=CR_ASEQX(15);
pub const CR_PSIZEBYTE:u32=bit(8); pub const CR_PSIZEHALF:u32=2<<8; pub const CR_PSIZEWORD:u32=3<<8; pub const CR_PORTCPU:u32=0; pub const CR_PORTIO:u32=bit(6); pub const CR_PORTVXI:u32=2<<6; pub const CR_PORTMXI:u32=3<<6; pub const CR_AMDEVICE:u32=bit(0);

pub const CHSR_INT:u32=0x80000000; pub const CHSR_DONE:u32=0x02000000; pub const CHSR_LINKC:u32=0x00080000;
pub const MITE_MCR:u32=0x50c; pub const MCRPON:u32=0; pub const MITE_MAR:u32=0x510; pub const MITE_DCR:u32=0x514; pub const DCR_NORMAL:u32=bit(29); pub const DCRPON:u32=0; pub const MITE_DAR:u32=0x518; pub const MITE_LKCR:u32=0x51c; pub const MITE_LKAR:u32=0x520; pub const MITE_LLKAR:u32=0x524; pub const MITE_BAR:u32=0x528; pub const MITE_BCR:u32=0x52c; pub const MITE_SAR:u32=0x530; pub const MITE_WSCR:u32=0x534; pub const MITE_WSER:u32=0x538; pub const MITE_CHSR:u32=0x53c; pub const MITE_FCR:u32=0x540;
pub const MITE_FIFO:u32=0x80; pub const MITE_FIFOEND:u32=0xff;
pub const MITE_AMRAM:u32=0x00; pub const MITE_AMDEVICE:u32=0x01; pub const MITE_AMHOST_A32_SINGLE:u32=0x09; pub const MITE_AMHOST_A24_SINGLE:u32=0x39; pub const MITE_AMHOST_A16_SINGLE:u32=0x29; pub const MITE_AMHOST_A32_BLOCK:u32=0x0b; pub const MITE_AMHOST_A32D64_BLOCK:u32=0x08; pub const MITE_AMHOST_A24_BLOCK:u32=0x3b;

#[repr(u32)] pub enum mite_registers { MITE_IODWBSR=0xc0, MITE_CSIGR=0x460, MITE_IODWBSR_1=0xc4, MITE_IODWCR_1=0xf4 }
#[repr(u32)] pub enum MITE_IODWBSR_bits { WENAB=0x80, WENAB_6602=0x8c }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
