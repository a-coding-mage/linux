/*
 * BRIEF MODULE DESCRIPTION
 *	Defines for using and allocating DMA channels on the Alchemy
 *      Au1x00 MIPS processors.
 *
 * Copyright 2000, 2008 MontaVista Software Inc.
 * Author: MontaVista Software, Inc. <source@mvista.com>
 *
 * This is a source-level Rust translation of the original header.
 */

// Original dependencies: linux/io.h, linux/spinlock.h, linux/delay.h.

pub const NUM_AU1000_DMA_CHANNELS: usize = 8;

pub const DMA_MODE_SET: usize = 0x00000000;
pub const DMA_MODE_READ: usize = DMA_MODE_SET;
pub const DMA_MODE_CLEAR: usize = 0x00000004;
pub const DMA_DAH_MASK: u32 = 0x0f << 20;
pub const DMA_DID_BIT: u32 = 16;
pub const DMA_DID_MASK: u32 = 0x0f << DMA_DID_BIT;
pub const DMA_DS: u32 = 1 << 15;
pub const DMA_BE: u32 = 1 << 13;
pub const DMA_DR: u32 = 1 << 12;
pub const DMA_TS8: u32 = 1 << 11;
pub const DMA_DW_BIT: u32 = 9;
pub const DMA_DW_MASK: u32 = 0x03 << DMA_DW_BIT;
pub const DMA_DW8: u32 = 0 << DMA_DW_BIT;
pub const DMA_DW16: u32 = 1 << DMA_DW_BIT;
pub const DMA_DW32: u32 = 2 << DMA_DW_BIT;
pub const DMA_NC: u32 = 1 << 8;
pub const DMA_IE: u32 = 1 << 7;
pub const DMA_HALT: u32 = 1 << 6;
pub const DMA_GO: u32 = 1 << 5;
pub const DMA_AB: u32 = 1 << 4;
pub const DMA_D1: u32 = 1 << 3;
pub const DMA_BE1: u32 = 1 << 2;
pub const DMA_D0: u32 = 1 << 1;
pub const DMA_BE0: u32 = 1 << 0;
pub const DMA_PERIPHERAL_ADDR: usize = 0x00000008;
pub const DMA_BUFFER0_START: usize = 0x0000000c;
pub const DMA_BUFFER1_START: usize = 0x00000014;
pub const DMA_BUFFER0_COUNT: usize = 0x00000010;
pub const DMA_BUFFER1_COUNT: usize = 0x00000018;
pub const DMA_BAH_BIT: u32 = 16;
pub const DMA_BAH_MASK: u32 = 0x0f << DMA_BAH_BIT;
pub const DMA_COUNT_BIT: u32 = 0;
pub const DMA_COUNT_MASK: u32 = 0xffff << DMA_COUNT_BIT;

#[repr(i32)]
pub enum DmaDeviceId {
    DMA_ID_UART0_TX = 0, DMA_ID_UART0_RX, DMA_ID_GP04, DMA_ID_GP05,
    DMA_ID_AC97C_TX, DMA_ID_AC97C_RX, DMA_ID_UART3_TX, DMA_ID_UART3_RX,
    DMA_ID_USBDEV_EP0_RX, DMA_ID_USBDEV_EP0_TX, DMA_ID_USBDEV_EP2_TX,
    DMA_ID_USBDEV_EP3_TX, DMA_ID_USBDEV_EP4_RX, DMA_ID_USBDEV_EP5_RX,
    DMA_ID_I2S_TX, DMA_ID_I2S_RX, DMA_NUM_DEV,
}

#[repr(i32)]
pub enum DmaDeviceIdBank2 {
    DMA_ID_SD0_TX = 0, DMA_ID_SD0_RX, DMA_ID_SD1_TX, DMA_ID_SD1_RX,
    DMA_NUM_DEV_BANK2,
}

#[repr(C)]
pub struct dma_chan {
    pub dev_id: i32,
    pub io: *mut core::ffi::c_void,
    pub dev_str: *const core::ffi::c_char,
    pub irq: i32,
    pub irq_dev: *mut core::ffi::c_void,
    pub fifo_addr: u32,
    pub mode: u32,
}

extern "C" {
    pub static mut au1000_dma_table: [dma_chan; NUM_AU1000_DMA_CHANNELS];
    pub fn request_au1000_dma(dev_id: i32, dev_str: *const core::ffi::c_char,
        irqhandler: irq_handler_t, irqflags: core::ffi::c_ulong,
        irq_dev_id: *mut core::ffi::c_void) -> i32;
    pub fn free_au1000_dma(dmanr: u32);
    pub fn au1000_dma_read_proc(buf: *mut core::ffi::c_char,
        start: *mut *mut core::ffi::c_char, fpos: off_t, length: i32,
        eof: *mut i32, data: *mut core::ffi::c_void) -> i32;
    pub static mut au1000_dma_spin_lock: spinlock_t;
    pub fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    pub fn __raw_readl(addr: *mut core::ffi::c_void) -> u32;
    pub fn CPHYSADDR(addr: u32) -> u32;
    pub fn printk(fmt: *const core::ffi::c_char, ...);
}

pub type irq_handler_t = unsafe extern "C" fn();
pub type off_t = i64;
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

pub unsafe fn get_dma_chan(dmanr: u32) -> *mut dma_chan {
    if dmanr >= NUM_AU1000_DMA_CHANNELS as u32 || au1000_dma_table[dmanr as usize].dev_id < 0 {
        core::ptr::null_mut()
    } else { &mut au1000_dma_table[dmanr as usize] }
}

pub unsafe fn claim_dma_lock() -> core::ffi::c_ulong { 0 /* spin_lock_irqsave(&au1000_dma_spin_lock, flags) */ }
pub unsafe fn release_dma_lock(_flags: core::ffi::c_ulong) { /* spin_unlock_irqrestore */ }

pub const DMA_HALT_POLL: i32 = 0x5000;

unsafe fn dma_reg(chan: *mut dma_chan, offset: usize) -> *mut core::ffi::c_void {
    ((*chan).io as *mut u8).add(offset) as *mut core::ffi::c_void
}

pub unsafe fn enable_dma_buffer0(dmanr: u32) { let c=get_dma_chan(dmanr); if !c.is_null(){__raw_writel(DMA_BE0,dma_reg(c,DMA_MODE_SET));} }
pub unsafe fn enable_dma_buffer1(dmanr: u32) { let c=get_dma_chan(dmanr); if !c.is_null(){__raw_writel(DMA_BE1,dma_reg(c,DMA_MODE_SET));} }
pub unsafe fn enable_dma_buffers(dmanr: u32) { let c=get_dma_chan(dmanr); if !c.is_null(){__raw_writel(DMA_BE0|DMA_BE1,dma_reg(c,DMA_MODE_SET));} }
pub unsafe fn start_dma(dmanr: u32) { let c=get_dma_chan(dmanr); if !c.is_null(){__raw_writel(DMA_GO,dma_reg(c,DMA_MODE_SET));} }
pub unsafe fn halt_dma(dmanr: u32) { let c=get_dma_chan(dmanr); if c.is_null(){return;} __raw_writel(DMA_GO,dma_reg(c,DMA_MODE_CLEAR)); let mut i=0; while i<DMA_HALT_POLL { if __raw_readl(dma_reg(c,DMA_MODE_READ))&DMA_HALT != 0 {break;} i+=1; } }
pub unsafe fn disable_dma(dmanr: u32) { let c=get_dma_chan(dmanr); if c.is_null(){return;} halt_dma(dmanr); __raw_writel(!DMA_GO,dma_reg(c,DMA_MODE_CLEAR)); }
pub unsafe fn dma_halted(dmanr: u32) -> i32 { let c=get_dma_chan(dmanr); if c.is_null(){1}else{if __raw_readl(dma_reg(c,DMA_MODE_READ))&DMA_HALT!=0{1}else{0}} }
pub unsafe fn init_dma(dmanr: u32) { let c=get_dma_chan(dmanr); if c.is_null(){return;} disable_dma(dmanr); __raw_writel(CPHYSADDR((*c).fifo_addr),dma_reg(c,DMA_PERIPHERAL_ADDR)); let mut mode=(*c).mode|((*c).dev_id as u32<<DMA_DID_BIT); if (*c).irq!=0{mode|=DMA_IE;} __raw_writel(!mode,dma_reg(c,DMA_MODE_CLEAR)); __raw_writel(mode,dma_reg(c,DMA_MODE_SET)); }
pub unsafe fn set_dma_mode(dmanr:u32, mut mode:u32) { let c=get_dma_chan(dmanr); if c.is_null(){return;} mode&=DMA_BE|DMA_DR|DMA_TS8|DMA_DW_MASK|DMA_NC; (*c).mode&=!(DMA_BE|DMA_DR|DMA_TS8|DMA_DW_MASK|DMA_NC); (*c).mode|=mode; }
pub unsafe fn get_dma_mode(dmanr:u32)->u32 { let c=get_dma_chan(dmanr); if c.is_null(){0}else{(*c).mode} }
pub unsafe fn get_dma_active_buffer(dmanr:u32)->i32 { let c=get_dma_chan(dmanr); if c.is_null(){-1}else{if __raw_readl(dma_reg(c,DMA_MODE_READ))&DMA_AB!=0{1}else{0}} }
pub unsafe fn set_dma_fifo_addr(dmanr:u32,a:u32) { let c=get_dma_chan(dmanr); if c.is_null()||(*c).mode&DMA_DS!=0||((*c).dev_id!=2&&(*c).dev_id!=3){return;} __raw_writel(CPHYSADDR(a),dma_reg(c,DMA_PERIPHERAL_ADDR)); }
pub unsafe fn clear_dma_done0(dmanr:u32){let c=get_dma_chan(dmanr);if !c.is_null(){__raw_writel(DMA_D0,dma_reg(c,DMA_MODE_CLEAR));}}
pub unsafe fn clear_dma_done1(dmanr:u32){let c=get_dma_chan(dmanr);if !c.is_null(){__raw_writel(DMA_D1,dma_reg(c,DMA_MODE_CLEAR));}}
pub unsafe fn set_dma_page(_dmanr:u32,_pagenr:i8){}
pub unsafe fn set_dma_addr0(dmanr:u32,a:u32){let c=get_dma_chan(dmanr);if !c.is_null(){__raw_writel(a,dma_reg(c,DMA_BUFFER0_START));}}
pub unsafe fn set_dma_addr1(dmanr:u32,a:u32){let c=get_dma_chan(dmanr);if !c.is_null(){__raw_writel(a,dma_reg(c,DMA_BUFFER1_START));}}
pub unsafe fn set_dma_count0(dmanr:u32,mut count:u32){let c=get_dma_chan(dmanr);if !c.is_null(){count&=DMA_COUNT_MASK;__raw_writel(count,dma_reg(c,DMA_BUFFER0_COUNT));}}
pub unsafe fn set_dma_count1(dmanr:u32,mut count:u32){let c=get_dma_chan(dmanr);if !c.is_null(){count&=DMA_COUNT_MASK;__raw_writel(count,dma_reg(c,DMA_BUFFER1_COUNT));}}
pub unsafe fn set_dma_count(dmanr:u32,mut count:u32){let c=get_dma_chan(dmanr);if !c.is_null(){count&=DMA_COUNT_MASK;__raw_writel(count,dma_reg(c,DMA_BUFFER0_COUNT));__raw_writel(count,dma_reg(c,DMA_BUFFER1_COUNT));}}
pub unsafe fn get_dma_buffer_done(dmanr:u32)->u32{let c=get_dma_chan(dmanr);if c.is_null(){0}else{__raw_readl(dma_reg(c,DMA_MODE_READ))&(DMA_D0|DMA_D1)}}
pub unsafe fn get_dma_done_irq(dmanr:u32)->i32{let c=get_dma_chan(dmanr);if c.is_null(){-1}else{(*c).irq}}
pub unsafe fn get_dma_residue(dmanr:u32)->i32{let c=get_dma_chan(dmanr);if c.is_null(){return 0;}let r=if __raw_readl(dma_reg(c,DMA_MODE_READ))&DMA_AB!=0{DMA_BUFFER1_COUNT}else{DMA_BUFFER0_COUNT};let mut count=(__raw_readl(dma_reg(c,r))&DMA_COUNT_MASK) as i32;if (*c).mode&DMA_DW_MASK==DMA_DW16{count<<=1}else if (*c).mode&DMA_DW_MASK==DMA_DW32{count<<=2}count}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
