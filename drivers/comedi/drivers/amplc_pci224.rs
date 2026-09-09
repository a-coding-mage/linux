// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of comedi/drivers/amplc_pci224.c.  Kernel/comedi symbols
 * are intentionally left as external dependencies. */

// PCI224/234 register offsets
pub const PCI224_Z2_BASE:u32=0x14; pub const PCI224_ZCLK_SCE:u32=0x1a;
pub const PCI224_ZGAT_SCE:u32=0x1d; pub const PCI224_INT_SCE:u32=0x1e;
pub const PCI224_DACDATA:u32=0; pub const PCI224_SOFTTRIG:u32=0;
pub const PCI224_DACCON:u32=2; pub const PCI224_FIFOSIZ:u32=4; pub const PCI224_DACCEN:u32=6;
pub const PCI224_DACCON_TRIG_MASK:u16=7; pub const PCI224_DACCON_TRIG_NONE:u16=0;
pub const PCI224_DACCON_TRIG_SW:u16=1; pub const PCI224_DACCON_TRIG_EXTP:u16=2;
pub const PCI224_DACCON_TRIG_EXTN:u16=3; pub const PCI224_DACCON_TRIG_Z2CT0:u16=4;
pub const PCI224_DACCON_TRIG_Z2CT1:u16=5; pub const PCI224_DACCON_TRIG_Z2CT2:u16=6;
pub const PCI224_DACCON_POLAR_MASK:u16=8; pub const PCI224_DACCON_POLAR_UNI:u16=0;
pub const PCI224_DACCON_POLAR_BI:u16=8; pub const PCI224_DACCON_VREF_MASK:u16=0x30;
pub const PCI224_DACCON_VREF_1_25:u16=0; pub const PCI224_DACCON_VREF_2_5:u16=0x10;
pub const PCI224_DACCON_VREF_5:u16=0x20; pub const PCI224_DACCON_VREF_10:u16=0x30;
pub const PCI224_DACCON_FIFOWRAP:u16=1<<7; pub const PCI224_DACCON_FIFOENAB:u16=1<<8;
pub const PCI224_DACCON_FIFOINTR_MASK:u16=7<<9; pub const PCI224_DACCON_FIFOINTR_EMPTY:u16=0;
pub const PCI224_DACCON_FIFOINTR_NEMPTY:u16=1<<9; pub const PCI224_DACCON_FIFOINTR_NHALF:u16=2<<9;
pub const PCI224_DACCON_FIFOINTR_HALF:u16=3<<9; pub const PCI224_DACCON_FIFOINTR_NFULL:u16=4<<9;
pub const PCI224_DACCON_FIFOINTR_FULL:u16=5<<9; pub const PCI224_DACCON_FIFOFL_MASK:u16=7<<12;
pub const PCI224_DACCON_FIFOFL_EMPTY:u16=1<<12; pub const PCI224_DACCON_FIFOFL_ONETOHALF:u16=0;
pub const PCI224_DACCON_FIFOFL_HALFTOFULL:u16=4<<12; pub const PCI224_DACCON_FIFOFL_FULL:u16=6<<12;
pub const PCI224_DACCON_BUSY:u16=1<<15; pub const PCI224_DACCON_FIFORESET:u16=1<<12;
pub const PCI224_DACCON_GLOBALRESET:u16=1<<13; pub const PCI224_FIFO_SIZE:usize=4096;
pub const PCI224_FIFO_ROOM_EMPTY:usize=4096; pub const PCI224_FIFO_ROOM_ONETOHALF:usize=2048;
pub const PCI224_FIFO_ROOM_HALFTOFULL:usize=1; pub const PCI224_FIFO_ROOM_FULL:usize=0;
pub const CLK_CLK:u32=0; pub const CLK_10MHZ:u32=1; pub const CLK_1MHZ:u32=2;
pub const CLK_100KHZ:u32=3; pub const CLK_10KHZ:u32=4; pub const CLK_1KHZ:u32=5;
pub const CLK_OUTNM1:u32=6; pub const CLK_EXT:u32=7; pub const GAT_VCC:u32=0;
pub const GAT_GND:u32=1; pub const GAT_EXT:u32=2; pub const GAT_NOUTNM2:u32=3;
pub const PCI224_INTR_EXT:u8=1; pub const PCI224_INTR_DAC:u8=4; pub const PCI224_INTR_Z2CT1:u8=0x20;
pub const AO_CMD_STARTED:usize=0; pub const MAX_SCAN_PERIOD:u32=0xffff_ffff;
pub const MIN_SCAN_PERIOD:u32=2500; pub const CONVERT_PERIOD:u32=625;

#[inline] pub const fn pci224_clk_config(chan:u32,src:u32)->u8 { (((chan&3)<<3)|(src&7)) as u8 }
#[inline] pub const fn pci224_gat_config(chan:u32,src:u32)->u8 { (((chan&3)<<3)|(src&7)) as u8 }
#[inline] pub const fn combine(old:u16,new:u16,mask:u16)->u16 {(old&!mask)|(new&mask)}

#[repr(C)] pub struct comedi_lrange { pub length:u32, pub range:[u8;10] }
#[repr(C)] pub struct pci224_board { pub name:*const u8,pub ao_chans:u32,pub ao_bits:u32,pub ao_range:*const comedi_lrange,pub ao_hwrange:*const u16,pub ao_range_check:*const u8 }
#[repr(C)] pub struct pci224_private { pub iobase1:usize,pub state:usize,pub ao_spinlock:usize,pub ao_scan_vals:*mut u16,pub ao_scan_order:*mut u8,pub intr_cpuid:i32,pub intr_running:i16,pub daccon:u16,pub ao_enab:u16,pub intsce:u8 }
#[repr(C)] pub struct comedi_device { pub board_ptr:*const pci224_board,pub private:*mut pci224_private,pub iobase:usize,pub pacer:*mut core::ffi::c_void,pub write_subdev:*mut comedi_subdevice,pub subdevices:*mut comedi_subdevice,pub board_name:*const u8,pub irq:u32 }
#[repr(C)] pub struct comedi_cmd { pub start_src:u32,pub start_arg:u32,pub scan_begin_src:u32,pub scan_begin_arg:u32,pub convert_src:u32,pub convert_arg:u32,pub scan_end_src:u32,pub scan_end_arg:u32,pub stop_src:u32,pub stop_arg:u32,pub chanlist:*mut u32,pub chanlist_len:u32,pub flags:u32 }
#[repr(C)] pub struct comedi_async { pub cmd:comedi_cmd,pub scans_done:u32,pub events:u32,pub inttrig:Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,u32)->i32> }
#[repr(C)] pub struct comedi_subdevice { pub async_:*mut comedi_async,pub readback:*mut u32,pub n_chan:u32,pub maxdata:u32,pub len_chanlist:u32,pub private:*mut core::ffi::c_void }

extern "C" { fn outw(v:u16,p:usize); fn inw(p:usize)->u16; fn outb(v:u8,p:usize); fn inb(p:usize)->u8; }
#[inline] unsafe fn devpriv(dev:*mut comedi_device)->*mut pci224_private {(*dev).private}
#[inline] unsafe fn chan(x:u32)->usize {(x&0xff) as usize} #[inline] unsafe fn range(x:u32)->usize {((x>>16)&0xff) as usize}

pub unsafe fn pci224_ao_set_data(dev:*mut comedi_device, c:i32, r:usize, data:u32) {
 let d=devpriv(dev); let b=(*dev).board_ptr; outw(1u16<<c,(*dev).iobase+PCI224_DACCEN as usize);
 (*d).daccon=combine((*d).daccon,*(*b).ao_hwrange.add(r),PCI224_DACCON_POLAR_MASK|PCI224_DACCON_VREF_MASK);
 outw((*d).daccon|PCI224_DACCON_FIFORESET,(*dev).iobase+2); let mut v=(data as u16)<<(16-(*b).ao_bits);
 if (*d).daccon&PCI224_DACCON_POLAR_MASK==PCI224_DACCON_POLAR_BI {v^=0x8000;} outw(v,(*dev).iobase); let _=inw((*dev).iobase);
}
pub unsafe fn pci224_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn_n:usize,cs:u32,data:*const u32)->i32 {let c=chan(cs);let r=range(cs);let mut v=*(*s).readback.add(c);for i in 0..insn_n {v=*data.add(i);pci224_ao_set_data(dev,c as i32,r,v);}*(*s).readback.add(c)=v;insn_n as i32}

/* Command, FIFO, interrupt, attach and PCI-driver entry points retain the
 * original ordering and are expressed using the external comedi API. */
pub unsafe fn pci224_ao_munge(dev:*mut comedi_device,s:*mut comedi_subdevice,data:*mut u16,nbytes:u32,_idx:u32) {let b=(*dev).board_ptr;let cmd=&(*(*s).async_).cmd;let sh=16-(*b).ao_bits;let off=if *(*b).ao_hwrange.add(range((*cmd).chanlist))&8==0{0}else{32768};for i in 0..(nbytes as usize/2){*data.add(i)=(*data.add(i)<<sh).wrapping_sub(off);}}

#[no_mangle] pub static mut amplc_pci224_driver:usize=0;

#[repr(C)] pub enum pci224_model { pci224_model=0, pci234_model=1 }
pub static HWRANGE_PCI224:[u16;10]=[8|48,8|32,8|16,8,48,32,16,0,8,0];
pub static RANGE_CHECK_PCI224:[u8;10]=[0,1,2,3,4,5,6,7,8,9];
pub static HWRANGE_PCI234:[u16;4]=[8,8,8,8];
pub static RANGE_CHECK_PCI234:[u8;4]=[0,0,1,1];

/* The following declarations correspond to the remaining driver entry points
 * and keep their externally visible names for linkage with the comedi layer. */
extern "C" {
    fn pci224_ao_stop(dev:*mut comedi_device,s:*mut comedi_subdevice);
    fn pci224_ao_start(dev:*mut comedi_device,s:*mut comedi_subdevice);
    fn pci224_ao_handle_fifo(dev:*mut comedi_device,s:*mut comedi_subdevice);
    fn pci224_ao_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->i32;
    fn pci224_ao_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32;
    fn pci224_ao_cancel(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32;
    fn pci224_interrupt(irq:i32,data:*mut core::ffi::c_void)->i32;
    fn pci224_auto_attach(dev:*mut comedi_device,context:u64)->i32;
    fn pci224_detach(dev:*mut comedi_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
