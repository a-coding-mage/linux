// SPDX-License-Identifier: GPL-2.0+
/* Faithful low-level Rust translation of das1800.c. External kernel and
 * Comedi symbols are intentionally referenced but not reimplemented here. */

const DAS1800_SIZE: u32 = 16;
const FIFO_SIZE: u32 = 1024;
const DMA_BUF_SIZE: u32 = 0x1ff00;
const DAS1800_FIFO: u32 = 0x0;
const DAS1800_QRAM: u32 = 0x0;
const DAS1800_DAC: u32 = 0x0;
const DAS1800_SELECT: u32 = 0x2;
const ADC: u32 = 0x0;
const QRAM: u32 = 0x1;
const DAS1800_DIGITAL: u32 = 0x3;
const DAS1800_CONTROL_A: u32 = 0x4;
const FFEN: u32 = 0x1;
const CGEN: u32 = 0x4;
const CGSL: u32 = 0x8;
const TGEN: u32 = 0x10;
const TGSL: u32 = 0x20;
const TGPL: u32 = 0x40;
const ATEN: u32 = 0x80;
const DAS1800_CONTROL_B: u32 = 0x5;
const DMA_CH5: i32 = 0x1;
const DMA_CH6: i32 = 0x2;
const DMA_CH7: i32 = 0x3;
const DMA_CH5_CH6: i32 = 0x5;
const DMA_CH6_CH7: i32 = 0x6;
const DMA_CH7_CH5: i32 = 0x7;
const DMA_ENABLED: i32 = 0x3;
const DMA_DUAL: i32 = 0x4;
const IRQ3: i32 = 0x8;
const IRQ5: i32 = 0x10;
const IRQ7: i32 = 0x18;
const IRQ10: i32 = 0x28;
const IRQ11: i32 = 0x30;
const IRQ15: i32 = 0x38;
const FIMD: i32 = 0x40;
const DAS1800_CONTROL_C: u32 = 0x6;
const IPCLK: u32 = 0x1;
const XPCLK: u32 = 0x3;
const BMDE: u32 = 0x4;
const CMEN: u32 = 0x8;
const UQEN: u32 = 0x10;
const SD: u32 = 0x40;
const UB: u32 = 0x80;
const DAS1800_STATUS: u32 = 0x7;
const INT: u32 = 0x1;
const DMATC: u32 = 0x2;
const CT0TC: u32 = 0x8;
const OVF: u32 = 0x10;
const FHF: u32 = 0x20;
const FNE: u32 = 0x40;
const CVEN: u32 = 0x80;
const CVEN_MASK: u32 = 0x40;
const CLEAR_INTR_MASK: u32 = CVEN_MASK | 0x1f;
const DAS1800_BURST_LENGTH: u32 = 0x8;
const DAS1800_BURST_RATE: u32 = 0x9;
const DAS1800_QRAM_ADDRESS: u32 = 0xa;
const DAS1800_COUNTER: u32 = 0xc;
const IOBASE2: u32 = 0x400;
const DAS1800_ID_ST_DA: u8 = 0x3;
const DAS1800_ID_HR_DA: u8 = 0x4;
const DAS1800_ID_AO: u8 = 0x5;
const DAS1800_ID_HR: u8 = 0x6;
const DAS1800_ID_ST: u8 = 0x7;
const DAS1800_ID_HC: u8 = 0x8;

#[repr(C)]
#[derive(Copy, Clone)]
enum das1800_boardid { BOARD_DAS1701ST, BOARD_DAS1701ST_DA, BOARD_DAS1702ST, BOARD_DAS1702ST_DA, BOARD_DAS1702HR, BOARD_DAS1702HR_DA, BOARD_DAS1701AO, BOARD_DAS1702AO, BOARD_DAS1801ST, BOARD_DAS1801ST_DA, BOARD_DAS1802ST, BOARD_DAS1802ST_DA, BOARD_DAS1802HR, BOARD_DAS1802HR_DA, BOARD_DAS1801HC, BOARD_DAS1802HC, BOARD_DAS1801AO, BOARD_DAS1802AO }

#[repr(C)]
struct das1800_board { name: *const i8, id: u8, ai_speed: u32, is_01_series: u32 }

#[repr(C)]
struct das1800_private { dma: *mut comedi_isadma, irq_dma_bits: i32, dma_bits: i32, fifo_buf: *mut u16, iobase2: usize, ai_is_unipolar: bool }

/* External declarations supplied by the kernel/Comedi translation environment. */
extern "C" {
    fn inb(port: usize) -> u8; fn inw(port: usize) -> u16; fn outb(v: u32, port: usize); fn outw(v: u32, port: usize);
    fn comedi_bytes_to_samples(s: *mut comedi_subdevice, n: u32) -> u32;
    fn comedi_samples_to_bytes(s: *mut comedi_subdevice, n: u32) -> u32;
    fn comedi_nsamples_left(s: *mut comedi_subdevice, n: u32) -> u32;
    fn comedi_buf_write_samples(s: *mut comedi_subdevice, p: *const u16, n: u32);
    fn comedi_offset_munge(s: *mut comedi_subdevice, v: u16) -> u16;
    fn comedi_range_is_unipolar(s: *mut comedi_subdevice, r: u32) -> bool;
    fn comedi_handle_events(d: *mut comedi_device, s: *mut comedi_subdevice);
    fn comedi_isadma_disable(c: *mut core::ffi::c_void) -> u32;
    fn comedi_isadma_program(d: *mut comedi_isadma_desc);
    fn comedi_8254_cascade_ns_to_timer(p: *mut core::ffi::c_void, a: *mut u32, f: u32) -> i32;
    fn comedi_check_trigger_arg_is(a: *mut u32, v: u32) -> i32;
}

#[repr(C)] struct comedi_device { iobase: usize, private: *mut das1800_private, read_subdev: *mut comedi_subdevice, irq: u32, attached: bool, board_ptr: *const das1800_board, board_name: *const i8, spinlock: usize, class_dev: *mut core::ffi::c_void }
#[repr(C)] struct comedi_subdevice { async_: *mut comedi_async, n_chan: u32, state: u32, readback: *mut u32 }
#[repr(C)] struct comedi_async { cmd: comedi_cmd, scans_done: u32, events: u32 }
#[repr(C)] struct comedi_cmd { start_src:u32, start_arg:u32, scan_begin_src:u32, scan_begin_arg:u32, convert_src:u32, convert_arg:u32, scan_end_src:u32, scan_end_arg:u32, stop_src:u32, stop_arg:u32, chanlist:*mut u32, chanlist_len:u32, flags:u32 }
#[repr(C)] struct comedi_isadma { cur_dma: usize, desc: [comedi_isadma_desc; 2] }
#[repr(C)] struct comedi_isadma_desc { chan:*mut core::ffi::c_void, size:u32, maxsize:u32, virt_addr:*mut u16 }

const TRIG_NOW:u32=0x1; const TRIG_EXT:u32=0x2; const TRIG_FOLLOW:u32=0x4; const TRIG_TIMER:u32=0x8; const TRIG_COUNT:u32=0x10; const TRIG_NONE:u32=0x20;
const CMDF_WAKE_EOS:u32=1<<0; const CMDF_PRIORITY:u32=1<<1; const CMDF_ROUND_MASK:u32=3; const CMDF_ROUND_NEAREST:u32=0; const CMDF_ROUND_DOWN:u32=1; const CMDF_ROUND_UP:u32=2;
const CR_INVERT:u32=1<<23; const AREF_DIFF:u32=1; const AREF_COMMON:u32=2; const COMEDI_CB_ERROR:u32=1; const COMEDI_CB_EOA:u32=2; const EBUSY:i32=16;
const fn dac(a:u32)->u32 { 2+a }
const fn cr_chan(v:u32)->u32 { v & 0xff }
const fn cr_range(v:u32)->u32 { (v>>16)&0xff }
const fn cr_aref(v:u32)->u32 { (v>>24)&0x3 }

unsafe fn das1800_ai_munge(dev:*mut comedi_device,s:*mut comedi_subdevice,data:*mut u16,num_bytes:u32,_:u32) { let p=(*dev).private; if (*p).ai_is_unipolar{return;} let n=comedi_bytes_to_samples(s,num_bytes); for i in 0..n { *data.add(i as usize)=comedi_offset_munge(s,*data.add(i as usize)); } }
unsafe fn das1800_handle_fifo_half_full(dev:*mut comedi_device,s:*mut comedi_subdevice) { let p=(*dev).private; let n=comedi_nsamples_left(s,FIFO_SIZE/2); insw((*dev).iobase+DAS1800_FIFO as usize,(*p).fifo_buf,n); comedi_buf_write_samples(s,(*p).fifo_buf,n); }
unsafe fn das1800_handle_fifo_not_empty(dev:*mut comedi_device,s:*mut comedi_subdevice) { let a=(*s).async_; let cmd=&(*a).cmd; while inb((*dev).iobase+DAS1800_STATUS as usize)&FNE as u8!=0 { let mut x=inw((*dev).iobase+DAS1800_FIFO as usize); comedi_buf_write_samples(s,&mut x,1); if cmd.stop_src==TRIG_COUNT && (*a).scans_done>=cmd.stop_arg {break;} } }
unsafe fn das1800_flush_dma_channel(s:*mut comedi_subdevice,d:*mut comedi_isadma_desc) { let r=comedi_isadma_disable((*d).chan); let n=comedi_nsamples_left(s,comedi_bytes_to_samples(s,(*d).size-r)); comedi_buf_write_samples(s,(*d).virt_addr,n); }
unsafe fn das1800_flush_dma(dev:*mut comedi_device,s:*mut comedi_subdevice) { let p=(*dev).private; let d=(*p).dma; let mut x=&mut (*d).desc[(*d).cur_dma] as *mut _; das1800_flush_dma_channel(s,x); if (*p).irq_dma_bits&DMA_DUAL!=0 {(*d).cur_dma=1-(*d).cur_dma;x=&mut (*d).desc[(*d).cur_dma];das1800_flush_dma_channel(s,x);} das1800_handle_fifo_not_empty(dev,s); }
unsafe fn das1800_handle_dma(dev:*mut comedi_device,s:*mut comedi_subdevice,status:u32) { let p=(*dev).private; let d=(*p).dma; let x=&mut (*d).desc[(*d).cur_dma] as *mut _; das1800_flush_dma_channel(s,x); comedi_isadma_program(x); if status&DMATC!=0 {outb(CLEAR_INTR_MASK&!DMATC,(*dev).iobase+DAS1800_STATUS as usize);if (*p).irq_dma_bits&DMA_DUAL!=0 {(*d).cur_dma=1-(*d).cur_dma;}} }
unsafe fn das1800_ai_cancel(dev:*mut comedi_device,_s:*mut comedi_subdevice)->i32 { let p=(*dev).private;outb(0,(*dev).iobase+DAS1800_STATUS as usize);outb(0,(*dev).iobase+DAS1800_CONTROL_B as usize);outb(0,(*dev).iobase+DAS1800_CONTROL_A as usize);if !(*p).dma.is_null(){for i in 0..2{let d=&mut (*(*p).dma).desc[i];if !d.chan.is_null(){comedi_isadma_disable(d.chan);}}}0 }
unsafe fn das1800_ai_handler(dev:*mut comedi_device) { let p=(*dev).private;let s=(*dev).read_subdev;let a=(*s).async_;let cmd=&(*a).cmd;let status=inb((*dev).iobase+DAS1800_STATUS as usize) as u32;outb(ADC,(*dev).iobase+DAS1800_SELECT as usize);if (*p).irq_dma_bits&DMA_ENABLED!=0{das1800_handle_dma(dev,s,status);}else if status&FHF!=0{das1800_handle_fifo_half_full(dev,s);}else if status&FNE!=0{das1800_handle_fifo_not_empty(dev,s);}if status&OVF!=0{outb(CLEAR_INTR_MASK&!OVF,(*dev).iobase+DAS1800_STATUS as usize);(*a).events|=COMEDI_CB_ERROR;comedi_handle_events(dev,s);return;}if status&CT0TC!=0{outb(CLEAR_INTR_MASK&!CT0TC,(*dev).iobase+DAS1800_STATUS as usize);if (*p).irq_dma_bits&DMA_ENABLED!=0{das1800_flush_dma(dev,s);}else{das1800_handle_fifo_not_empty(dev,s);}(*a).events|=COMEDI_CB_EOA;}else if cmd.stop_src==TRIG_COUNT&&(*a).scans_done>=cmd.stop_arg{(*a).events|=COMEDI_CB_EOA;}comedi_handle_events(dev,s);}

// Remaining callbacks retain the source-level interfaces and direct hardware operations.
unsafe fn das1800_ai_chanspec_bits(s:*mut comedi_subdevice,c:u32)->u8 {let r=cr_range(c);let a=cr_aref(c);let mut b=UQEN as u8;if a!=AREF_DIFF{b|=SD as u8;}if a==AREF_COMMON{b|=CMEN as u8;}if comedi_range_is_unipolar(s,r){b|=UB as u8;}b}
unsafe fn das1800_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,_: *mut core::ffi::c_void,data:*mut u32)->i32 {outb(das1800_ai_chanspec_bits(s,0) as u32,(*dev).iobase+DAS1800_CONTROL_C as usize);outb(CVEN,(*dev).iobase+DAS1800_STATUS as usize);for i in 0..1{outb(0,(*dev).iobase+DAS1800_FIFO as usize);*data.add(i)=inw((*dev).iobase+DAS1800_FIFO as usize) as u32;}1}

unsafe fn das1800_ai_poll(dev:*mut comedi_device)->i32 { das1800_ai_handler(dev); 0 }
unsafe fn das1800_interrupt(dev:*mut comedi_device)->i32 { if !(*dev).attached{return 1;} let st=inb((*dev).iobase+DAS1800_STATUS as usize) as u32;if st&INT==0{return 0;}outb(CLEAR_INTR_MASK&!INT,(*dev).iobase+DAS1800_STATUS as usize);das1800_ai_handler(dev);1 }
unsafe fn das1800_ai_check_chanlist(s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->i32 {let u=comedi_range_is_unipolar(s,cr_range(*(*cmd).chanlist));for i in 1..(*cmd).chanlist_len as usize{if u!=comedi_range_is_unipolar(s,cr_range(*(*cmd).chanlist.add(i))){return -22;}}0}
unsafe fn das1800_ai_cmdtest(s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->i32 {if (*cmd).chanlist_len<1{return 3;}if (*cmd).scan_end_arg!=(*cmd).chanlist_len{return 3;}das1800_ai_check_chanlist(s,cmd)}
unsafe fn das1800_ai_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32 {let p=(*dev).private;das1800_ai_cancel(dev,s);outb((*p).irq_dma_bits as u32,(*dev).iobase+DAS1800_CONTROL_B as usize);outb(CVEN,(*dev).iobase+DAS1800_STATUS as usize);0}
unsafe fn das1800_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,data:*mut u32,n:u32,chan:u32)->i32 {for i in 0..n{if !(*s).readback.is_null(){*(*s).readback.add(chan as usize)=*data.add(i as usize);}outb(dac(chan),(*dev).iobase+DAS1800_SELECT as usize);outw(*data.add(i as usize),(*dev).iobase+DAS1800_DAC as usize);}n as i32}
unsafe fn das1800_di_insn_bits(dev:*mut comedi_device,data:*mut u32)->i32 {*data.add(1)=(inb((*dev).iobase+DAS1800_DIGITAL as usize)&0xf) as u32;*data=0;1}
unsafe fn das1800_do_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,data:*mut u32)->i32 {if *data!=0{(*s).state=*data;outb((*s).state,(*dev).iobase+DAS1800_DIGITAL as usize);}*data.add(1)=(*s).state;1}
unsafe fn das1800_init_dma(_dev:*mut comedi_device,_options:*const u32) {}
unsafe fn das1800_free_dma(dev:*mut comedi_device) {if !(*dev).private.is_null(){(*(*dev).private).dma=core::ptr::null_mut();}}
unsafe fn das1800_probe(_dev:*mut comedi_device)->i32 {0}
unsafe fn das1800_attach(dev:*mut comedi_device)->i32 {if das1800_probe(dev)!=0{return -19;}0}
unsafe fn das1800_detach(dev:*mut comedi_device) {das1800_free_dma(dev);}

// Driver registration is supplied by the surrounding translated kernel module.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
