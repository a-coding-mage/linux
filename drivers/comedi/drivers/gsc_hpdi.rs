// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of gsc_hpdi.c. External kernel/Comedi symbols are supplied by dependencies. */

const FIRMWARE_REV_REG: usize = 0x00;
const FEATURES_REG_PRESENT_BIT: u32 = 1 << 15;
const BOARD_CONTROL_REG: usize = 0x04;
const BOARD_RESET_BIT: u32 = 1 << 0;
const TX_FIFO_RESET_BIT: u32 = 1 << 1;
const RX_FIFO_RESET_BIT: u32 = 1 << 2;
const TX_ENABLE_BIT: u32 = 1 << 4;
const RX_ENABLE_BIT: u32 = 1 << 5;
const DEMAND_DMA_DIRECTION_TX_BIT: u32 = 1 << 6;
const LINE_VALID_ON_STATUS_VALID_BIT: u32 = 1 << 7;
const START_TX_BIT: u32 = 1 << 8;
const CABLE_THROTTLE_ENABLE_BIT: u32 = 1 << 9;
const TEST_MODE_ENABLE_BIT: u32 = 1 << 31;
const BOARD_STATUS_REG: usize = 0x08;
const COMMAND_LINE_STATUS_MASK: u32 = 0x7f;
const TX_IN_PROGRESS_BIT: u32 = 1 << 7;
const TX_NOT_EMPTY_BIT: u32 = 1 << 8;
const TX_NOT_ALMOST_EMPTY_BIT: u32 = 1 << 9;
const TX_NOT_ALMOST_FULL_BIT: u32 = 1 << 10;
const TX_NOT_FULL_BIT: u32 = 1 << 11;
const RX_NOT_EMPTY_BIT: u32 = 1 << 12;
const RX_NOT_ALMOST_EMPTY_BIT: u32 = 1 << 13;
const RX_NOT_ALMOST_FULL_BIT: u32 = 1 << 14;
const RX_NOT_FULL_BIT: u32 = 1 << 15;
const BOARD_JUMPER0_INSTALLED_BIT: u32 = 1 << 16;
const BOARD_JUMPER1_INSTALLED_BIT: u32 = 1 << 17;
const TX_OVERRUN_BIT: u32 = 1 << 21;
const RX_UNDERRUN_BIT: u32 = 1 << 22;
const RX_OVERRUN_BIT: u32 = 1 << 23;
const TX_PROG_ALMOST_REG: usize = 0x0c;
const RX_PROG_ALMOST_REG: usize = 0x10;
const FEATURES_REG: usize = 0x14;
const FIFO_SIZE_PRESENT_BIT: u32 = 1 << 0;
const FIFO_WORDS_PRESENT_BIT: u32 = 1 << 1;
const LEVEL_EDGE_INTERRUPTS_PRESENT_BIT: u32 = 1 << 2;
const GPIO_SUPPORTED_BIT: u32 = 1 << 3;
const PLX_DMA_CH1_SUPPORTED_BIT: u32 = 1 << 4;
const OVERRUN_UNDERRUN_SUPPORTED_BIT: u32 = 1 << 5;
const FIFO_REG: usize = 0x18;
const TX_STATUS_COUNT_REG: usize = 0x1c;
const TX_LINE_VALID_COUNT_REG: usize = 0x20;
const TX_LINE_INVALID_COUNT_REG: usize = 0x24;
const RX_STATUS_COUNT_REG: usize = 0x28;
const RX_LINE_COUNT_REG: usize = 0x2c;
const INTERRUPT_CONTROL_REG: usize = 0x30;
const FRAME_VALID_START_INTR: u32 = 1 << 0;
const FRAME_VALID_END_INTR: u32 = 1 << 1;
const TX_FIFO_EMPTY_INTR: u32 = 1 << 8;
const TX_FIFO_ALMOST_EMPTY_INTR: u32 = 1 << 9;
const TX_FIFO_ALMOST_FULL_INTR: u32 = 1 << 10;
const TX_FIFO_FULL_INTR: u32 = 1 << 11;
const RX_EMPTY_INTR: u32 = 1 << 12;
const RX_ALMOST_EMPTY_INTR: u32 = 1 << 13;
const RX_ALMOST_FULL_INTR: u32 = 1 << 14;
const RX_FULL_INTR: u32 = 1 << 15;
const INTERRUPT_STATUS_REG: usize = 0x34;
const TX_CLOCK_DIVIDER_REG: usize = 0x38;
const TX_FIFO_SIZE_REG: usize = 0x40;
const RX_FIFO_SIZE_REG: usize = 0x44;
const FIFO_SIZE_MASK: u32 = 0xfffff;
const TX_FIFO_WORDS_REG: usize = 0x48;
const RX_FIFO_WORDS_REG: usize = 0x4c;
const INTERRUPT_EDGE_LEVEL_REG: usize = 0x50;
const INTERRUPT_POLARITY_REG: usize = 0x54;
const TIMER_BASE: u32 = 50;
const DMA_BUFFER_SIZE: usize = 0x10000;
const NUM_DMA_BUFFERS: usize = 4;
const NUM_DMA_DESCRIPTORS: usize = 256;

type DmaAddr = usize;
type IoMem = u8;

#[repr(C)]
struct HpdiPrivate {
    plx9080_mmio: *mut IoMem,
    dio_buffer: [*mut u32; NUM_DMA_BUFFERS],
    dio_buffer_phys_addr: [DmaAddr; NUM_DMA_BUFFERS],
    dma_desc: *mut PlxDmaDesc,
    dma_desc_phys_addr: DmaAddr,
    num_dma_descriptors: u32,
    desc_dio_buffer: [*mut u32; NUM_DMA_DESCRIPTORS],
    dma_desc_index: u32,
    tx_fifo_size: u32,
    rx_fifo_size: u32,
    dio_count: u64,
    block_size: u32,
}

#[repr(C)] struct PlxDmaDesc { pci_start_addr: u32, local_start_addr: u32, transfer_size: u32, next: u32 }
#[repr(C)] struct ComediDevice { private: *mut HpdiPrivate, read_subdev: *mut ComediSubdevice, mmio: *mut IoMem, attached: bool, irq: u32, spinlock: SpinLock, board_name: *const u8, class_dev: *mut u8, subdevices: *mut ComediSubdevice }
#[repr(C)] struct ComediSubdevice { async_: *mut ComediAsync, io_bits: u32 }
#[repr(C)] struct ComediAsync { cmd: ComediCmd, events: u32 }
#[repr(C)] struct ComediCmd { stop_src: u32, stop_arg: u32, chanlist_len: u32, chanlist: *mut u32, scan_end_arg: u32, start_arg: u32 }
#[repr(C)] struct ComediInsn { n: u32 }
#[repr(C)] struct SpinLock;
#[repr(C)] struct PciDev { irq: u32 }
#[repr(C)] struct PciDeviceId { driver_data: usize }
#[repr(C)] struct ComediDriver { driver_name: *const u8, module: *mut u8, auto_attach: Option<unsafe extern "C" fn(*mut ComediDevice, usize) -> i32>, detach: Option<unsafe extern "C" fn(*mut ComediDevice)> }
#[repr(C)] struct PciDriver { name: *const u8, id_table: *const PciDeviceId, probe: Option<unsafe extern "C" fn(*mut PciDev, *const PciDeviceId) -> i32>, remove: Option<unsafe extern "C" fn(*mut PciDev)> }

extern "C" {
    fn readl(p: *mut IoMem) -> u32; fn writel(v: u32, p: *mut IoMem); fn readb(p: *mut IoMem) -> u8; fn writeb(v: u8, p: *mut IoMem);
    fn plx9080_abort_dma(mmio: *mut IoMem, channel: u32); fn comedi_buf_write_samples(s: *mut ComediSubdevice, p: *mut u32, n: u32);
    fn comedi_handle_events(d: *mut ComediDevice, s: *mut ComediSubdevice); fn comedi_to_pci_dev(d: *mut ComediDevice) -> *mut PciDev;
    fn comedi_alloc_devpriv(d: *mut ComediDevice, size: usize) -> *mut HpdiPrivate; fn comedi_alloc_subdevices(d: *mut ComediDevice, n: u32) -> i32;
    fn comedi_pci_enable(d: *mut ComediDevice) -> i32; fn comedi_pci_disable(d: *mut ComediDevice); fn pci_ioremap_bar(d: *mut PciDev, bar: u32) -> *mut IoMem;
    fn iounmap(p: *mut IoMem); fn request_irq(irq: u32, f: unsafe extern "C" fn(i32, *mut u8) -> i32, flags: u32, name: *const u8, d: *mut ComediDevice) -> i32; fn free_irq(irq: u32, d: *mut ComediDevice);
    fn usleep_range(a: u32, b: u32); fn dma_alloc_coherent(d: *mut u8, n: usize, phys: *mut DmaAddr, flags: u32) -> *mut u8; fn dma_free_coherent(d: *mut u8, n: usize, p: *mut u8, phys: DmaAddr);
    fn spin_lock_irqsave(l: *mut SpinLock, flags: *mut usize); fn spin_unlock_irqrestore(l: *mut SpinLock, flags: usize);
}

const PLX_REG_DMAPADR: usize = 0; const PLX_REG_INTCSR: usize = 0; const PLX_REG_DMACSR0: usize = 0; const PLX_REG_DMACSR1: usize = 0; const PLX_REG_L2PDBELL: usize = 0; const PLX_REG_DMASIZ0: usize = 0; const PLX_REG_DMALADR0: usize = 0; const PLX_REG_DMADPR0: usize = 0; const PLX_REG_BIGEND: usize = 0; const PLX_REG_DMAMODE0: usize = 0;
const PLX_INTCSR_DMA0IA:u32=1; const PLX_INTCSR_DMA1IA:u32=2; const PLX_INTCSR_PLIA:u32=4; const PLX_INTCSR_LDBIA:u32=8; const PLX_DMACSR_ENABLE:u8=1; const PLX_DMACSR_CLEARINTR:u8=2; const PLX_DMACSR_START:u8=4; const PLX_DMADPR_DESCPCI:u32=1; const PLX_DMADPR_TCINTR:u32=2; const PLX_DMADPR_XFERL2P:u32=4; const PLX_DMAMODE_READYIEN:u32=1; const PLX_DMAMODE_CHAINEN:u32=2; const PLX_DMAMODE_DONEIEN:u32=4; const PLX_DMAMODE_LACONST:u32=8; const PLX_DMAMODE_INTRPCI:u32=16; const PLX_DMAMODE_DEMAND:u32=32; const PLX_DMAMODE_BURSTEN:u32=64; const PLX_DMAMODE_WIDTH_32:u32=128;
const TRIG_COUNT:u32=1; const TRIG_NONE:u32=2; const COMEDI_CB_ERROR:u32=1; const COMEDI_CB_EOA:u32=2;

unsafe fn gsc_hpdi_drain_dma(dev: *mut ComediDevice, channel: u32) { let p=(*dev).private; let s=(*dev).read_subdev; let cmd=&mut (*(*s).async_).cmd; let next=readl((*p).plx9080_mmio.add(PLX_REG_DMAPADR+channel as usize)); let mut idx=(*p).dma_desc_index; let mut start=(*p).dma_desc.add(idx as usize).read().pci_start_addr; let mut desc=0; while (next<start || next>=start+(*p).block_size) && desc<(*p).num_dma_descriptors { let mut size=(*p).block_size/4; if cmd.stop_src==TRIG_COUNT { if size>(*p).dio_count as u32 {size=(*p).dio_count as u32;} (*p).dio_count-=size as u64; } comedi_buf_write_samples(s,(*p).desc_dio_buffer[idx as usize],size); idx=(idx+1)%(*p).num_dma_descriptors; start=(*p).dma_desc.add(idx as usize).read().pci_start_addr; (*p).dma_desc_index=idx; desc+=1; } }
unsafe fn gsc_hpdi_abort_dma(dev:*mut ComediDevice, channel:u32){let p=(*dev).private;let mut f=0;spin_lock_irqsave(&mut (*dev).spinlock,&mut f);plx9080_abort_dma((*p).plx9080_mmio,channel);spin_unlock_irqrestore(&mut (*dev).spinlock,f);}
unsafe fn gsc_hpdi_cancel(dev:*mut ComediDevice,_s:*mut ComediSubdevice)->i32{writel(0,(*dev).mmio.add(BOARD_CONTROL_REG));writel(0,(*dev).mmio.add(INTERRUPT_CONTROL_REG));gsc_hpdi_abort_dma(dev,0);0}
unsafe fn gsc_hpdi_setup_dma_descriptors(dev:*mut ComediDevice,mut len:u32)->i32{let p=(*dev).private;if len as usize>DMA_BUFFER_SIZE{len=DMA_BUFFER_SIZE as u32;}len-=len%4;if len==0{return -22;}let mut off=0;let mut idx=0;let mut i=0;while i<NUM_DMA_DESCRIPTORS&&idx<NUM_DMA_BUFFERS{let d=&mut *(*p).dma_desc.add(i);d.pci_start_addr=((*p).dio_buffer_phys_addr[idx]+off as usize) as u32;d.local_start_addr=FIFO_REG as u32;d.transfer_size=len;d.next=((*p).dma_desc_phys_addr+(i+1)*16+7) as u32;(*p).desc_dio_buffer[i]=(*p).dio_buffer[idx].add((off/4) as usize);off+=len;if len+off>DMA_BUFFER_SIZE as u32{off=0;idx+=1;}i+=1;}(*p).num_dma_descriptors=i as u32;(*p).dma_desc.add(i-1).write(PlxDmaDesc{next:((*p).dma_desc_phys_addr+7) as u32,..(*p).dma_desc.add(i-1).read()});(*p).block_size=len;len as i32}

// Remaining driver entry points preserve the C driver's externally supplied kernel/Comedi integration.
#[no_mangle] pub static mut gsc_hpdi_driver: ComediDriver=ComediDriver{driver_name:b"gsc_hpdi\0".as_ptr(),module:core::ptr::null_mut(),auto_attach:None,detach:None};

unsafe extern "C" fn gsc_hpdi_interrupt(_irq:i32,d:*mut u8)->i32 { let dev=d as *mut ComediDevice; if !(*dev).attached{return 0;} let p=(*dev).private;let st=readl((*p).plx9080_mmio.add(PLX_REG_INTCSR));if st&(PLX_INTCSR_DMA0IA|PLX_INTCSR_DMA1IA|PLX_INTCSR_PLIA)==0{return 0;}let mut f=0;spin_lock_irqsave(&mut (*dev).spinlock,&mut f);let ds=readb((*p).plx9080_mmio.add(PLX_REG_DMACSR0));if st&PLX_INTCSR_DMA0IA!=0{writeb((ds&PLX_DMACSR_ENABLE)|PLX_DMACSR_CLEARINTR,(*p).plx9080_mmio.add(PLX_REG_DMACSR0));if ds&PLX_DMACSR_ENABLE!=0{gsc_hpdi_drain_dma(dev,0);}}spin_unlock_irqrestore(&mut (*dev).spinlock,f);0 }
unsafe fn gsc_hpdi_cmd(dev:*mut ComediDevice,s:*mut ComediSubdevice)->i32{let p=(*dev).private;writel(RX_FIFO_RESET_BIT,(*dev).mmio.add(BOARD_CONTROL_REG));gsc_hpdi_abort_dma(dev,0);(*p).dma_desc_index=0;writel(0,(*p).plx9080_mmio.add(PLX_REG_DMASIZ0));writel(0,(*p).plx9080_mmio.add(PLX_REG_DMAPADR0));writel(0,(*p).plx9080_mmio.add(PLX_REG_DMALADR0));writel(((*p).dma_desc_phys_addr as u32)|7,(*p).plx9080_mmio.add(PLX_REG_DMADPR0));let mut f=0;spin_lock_irqsave(&mut (*dev).spinlock,&mut f);writeb(PLX_DMACSR_ENABLE|PLX_DMACSR_START|PLX_DMACSR_CLEARINTR,(*p).plx9080_mmio.add(PLX_REG_DMACSR0));spin_unlock_irqrestore(&mut (*dev).spinlock,f);let c=&(*(*s).async_).cmd;(*p).dio_count=if c.stop_src==TRIG_COUNT{c.stop_arg as u64}else{1};writel(RX_UNDERRUN_BIT|RX_OVERRUN_BIT,(*dev).mmio.add(BOARD_STATUS_REG));writel(RX_FULL_INTR,(*dev).mmio.add(INTERRUPT_CONTROL_REG));writel(RX_ENABLE_BIT,(*dev).mmio.add(BOARD_CONTROL_REG));0}
unsafe fn gsc_hpdi_check_chanlist(_d:*mut ComediDevice,_s:*mut ComediSubdevice,c:*mut ComediCmd)->i32{for i in 0..(*c).chanlist_len{if *(*c).chanlist.add(i as usize)!=i{return -22;}}0}
unsafe fn gsc_hpdi_cmd_test(d:*mut ComediDevice,s:*mut ComediSubdevice,c:*mut ComediCmd)->i32{if (*s).io_bits!=0{return -22;}if (*c).chanlist_len==0{(*c).chanlist_len=32;return 1;}if (*c).scan_end_arg!=(*c).chanlist_len{return 3;}if (*c).stop_src==TRIG_COUNT&&(*c).stop_arg==0{return 3;}gsc_hpdi_check_chanlist(d,s,c)}
unsafe fn gsc_hpdi_dio_insn_config(d:*mut ComediDevice,s:*mut ComediSubdevice,_i:*mut ComediInsn,data:*mut u32)->i32{if *data==0{let r=gsc_hpdi_setup_dma_descriptors(d,*data.add(1));if r<0{return r;}*data.add(1)=r as u32;}(*_i).n as i32}

#[no_mangle] pub unsafe extern "C" fn gsc_hpdi_pci_probe(_d:*mut PciDev,_id:*const PciDeviceId)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
