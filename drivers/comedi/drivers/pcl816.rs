// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of pcl816.c. Kernel/Comedi symbols are external dependencies. */

const PCL816_DO_DI_LSB_REG: usize = 0x00;
const PCL816_DO_DI_MSB_REG: usize = 0x01;
const PCL816_TIMER_BASE: usize = 0x04;
const PCL816_AI_LSB_REG: usize = 0x08;
const PCL816_AI_MSB_REG: usize = 0x09;
const PCL816_RANGE_REG: usize = 0x09;
const PCL816_CLRINT_REG: usize = 0x0a;
const PCL816_MUX_REG: usize = 0x0b;
const PCL816_CTRL_REG: usize = 0x0c;
const PCL816_STATUS_REG: usize = 0x0d;
const PCL816_CTRL_SOFT_TRIG: u32 = 1 << 0;
const PCL816_CTRL_PACER_TRIG: u32 = 1 << 1;
const PCL816_CTRL_EXT_TRIG: u32 = 1 << 2;
const PCL816_CTRL_POE: u32 = 1 << 3;
const PCL816_CTRL_DMAEN: u32 = 1 << 4;
const PCL816_CTRL_INTEN: u32 = 1 << 5;
const PCL816_STATUS_DRDY: u32 = 1 << 7;
const MAGIC_DMA_WORD: u32 = 0x5a5a;

#[repr(C)]
pub struct ComediDevice { pub private: *mut pcl816_private, pub iobase: usize, pub irq: u32, pub attached: bool, pub read_subdev: *mut ComediSubdevice, pub pacer: *mut core::ffi::c_void, pub board_ptr: *const pcl816_board, pub board_name: *const i8, pub subdevices: *mut ComediSubdevice, pub spinlock: core::ffi::c_void, pub class_dev: *mut core::ffi::c_void }
#[repr(C)] pub struct ComediSubdevice { pub async_: *mut ComediAsync, pub maxdata: u32, pub state: u32 }
#[repr(C)] pub struct ComediAsync { pub cmd: ComediCmd, pub scans_done: u32, pub events: u32 }
#[repr(C)] pub struct ComediCmd { pub stop_src: u32, pub stop_arg: u32, pub convert_src: u32, pub convert_arg: u32, pub chanlist_len: u32, pub chanlist: *mut u32, pub flags: u32 }
#[repr(C)] pub struct ComediInsn { pub chanspec: u32, pub n: u32 }
#[repr(C)] pub struct ComediDevconfig { pub options: [u32; 4] }
#[repr(C)] pub struct ComediIsadma { pub chan: u32, pub cur_dma: usize, pub desc: *mut ComediIsadmaDesc }
#[repr(C)] pub struct ComediIsadmaDesc { pub maxsize: usize, pub size: usize, pub virt_addr: *mut u16 }
#[repr(C)] pub struct Pcl816Range { pub length: u32, pub ranges: [u32; 8] }
#[repr(C)] pub struct pcl816_board { pub name: *const i8, pub ai_maxdata: i32, pub ai_chanlist: i32 }
#[repr(C)] pub struct pcl816_private { pub dma: *mut ComediIsadma, pub ai_poll_ptr: u32, pub ai_cmd_running: u32, pub ai_cmd_canceled: u32 }

extern "C" {
    fn outb(v: u32, port: usize); fn inb(port: usize) -> u32; fn udelay(us: u32);
    fn comedi_bytes_to_samples(s: *mut ComediSubdevice, n: usize) -> u32;
    fn comedi_samples_to_bytes(s: *mut ComediSubdevice, n: u32) -> usize;
    fn comedi_nsamples_left(s: *mut ComediSubdevice, n: u32) -> u32;
    fn comedi_isadma_disable(chan: u32); fn comedi_isadma_program(d: *mut ComediIsadmaDesc);
    fn comedi_buf_write_samples(s: *mut ComediSubdevice, p: *const u16, n: u32);
    fn comedi_handle_events(d: *mut ComediDevice, s: *mut ComediSubdevice);
    fn comedi_check_trigger_src(a: *mut u32, v: u32) -> i32; fn comedi_check_trigger_is_unique(v: u32) -> i32;
    fn comedi_check_trigger_arg_is(a: *mut u32, v: u32) -> i32; fn comedi_check_trigger_arg_min(a: *mut u32, v: u32) -> i32;
    fn comedi_8254_cascade_ns_to_timer(p: *mut core::ffi::c_void, a: *mut u32, f: u32);
    fn comedi_8254_set_mode(p: *mut core::ffi::c_void, c: u32, m: u32); fn comedi_8254_write(p: *mut core::ffi::c_void, c: u32, v: u32);
    fn comedi_8254_update_divisors(p: *mut core::ffi::c_void); fn comedi_8254_pacer_enable(p: *mut core::ffi::c_void, a: u32, b: u32, e: bool);
    fn comedi_isadma_poll(d: *mut ComediIsadma) -> usize; fn comedi_buf_n_bytes_ready(s: *mut ComediSubdevice) -> i32;
    fn comedi_timeout(d: *mut ComediDevice, s: *mut ComediSubdevice, i: *mut ComediInsn, f: unsafe extern "C" fn(*mut ComediDevice,*mut ComediSubdevice,*mut ComediInsn,usize)->i32, c: usize) -> i32;
    fn comedi_dio_update_state(s: *mut ComediSubdevice, data: *mut u32) -> bool;
    fn request_irq(i: u32, f: unsafe extern "C" fn(i32,*mut core::ffi::c_void)->i32, fl: u32, n: *const i8, d: *mut ComediDevice) -> i32;
    fn free_irq(i: u32, d: *mut ComediDevice); fn comedi_isadma_alloc(d:*mut ComediDevice,n:u32,c:u32,x:u32,z:usize,f:u32)->*mut ComediIsadma; fn comedi_isadma_free(d:*mut ComediIsadma);
    fn comedi_alloc_devpriv(d:*mut ComediDevice,z:usize)->*mut pcl816_private; fn comedi_check_request_region(d:*mut ComediDevice,b:u32,s:u32,a:u32,e:u32,w:u32)->i32;
    fn comedi_8254_io_alloc(b:usize,o:u32,w:u32,x:u32)->*mut core::ffi::c_void; fn comedi_alloc_subdevices(d:*mut ComediDevice,n:u32)->i32; fn comedi_legacy_detach(d:*mut ComediDevice);
}

const TRIG_COUNT:u32=1; const TRIG_FOLLOW:u32=2; const TRIG_EXT:u32=4; const TRIG_TIMER:u32=8; const TRIG_NOW:u32=16; const TRIG_NONE:u32=32;
const COMEDI_CB_EOA:u32=1; const I8254_MODE1:u32=1; const I8254_BINARY:u32=2; const SDF_CMD_READ:u32=1; const SDF_DIFF:u32=2;
const SDF_READABLE:u32=4; const SDF_WRITABLE:u32=8; const COMEDI_SUBD_AI:u32=1; const COMEDI_SUBD_UNUSED:u32=0; const COMEDI_SUBD_DI:u32=2; const COMEDI_SUBD_DO:u32=3;

static BOARDTYPES: [pcl816_board; 2] = [pcl816_board{name:b"pcl816\0".as_ptr() as *const i8,ai_maxdata:0xffff,ai_chanlist:1024},pcl816_board{name:b"pcl814b\0".as_ptr() as *const i8,ai_maxdata:0x3fff,ai_chanlist:1024}];

#[inline] unsafe fn chan(x:u32)->u32{x&0xff} #[inline] unsafe fn range(x:u32)->u32{(x>>8)&0xff} #[inline] unsafe fn aref(x:u32)->u32{(x>>16)&3}
#[inline] unsafe fn pcl816_ai_set_chan_range(d:*mut ComediDevice,c:u32,r:u32){outb(c,(*d).iobase+PCL816_MUX_REG);outb(r,(*d).iobase+PCL816_RANGE_REG)}
#[inline] unsafe fn pcl816_ai_set_chan_scan(d:*mut ComediDevice,f:u32,l:u32){outb((l<<4)|f,(*d).iobase+PCL816_MUX_REG)}
unsafe fn pcl816_ai_clear_eoc(d:*mut ComediDevice){outb(0,(*d).iobase+PCL816_CLRINT_REG)}
unsafe fn pcl816_ai_soft_trig(d:*mut ComediDevice){outb(0,(*d).iobase+PCL816_AI_LSB_REG)}
unsafe fn pcl816_ai_get_sample(d:*mut ComediDevice,s:*mut ComediSubdevice)->u32{(inb((*d).iobase+PCL816_AI_MSB_REG)<<8|inb((*d).iobase+PCL816_AI_LSB_REG))&(*s).maxdata}
unsafe extern "C" fn pcl816_ai_eoc(d:*mut ComediDevice,_s:*mut ComediSubdevice,_i:*mut ComediInsn,_c:usize)->i32{if inb((*d).iobase+PCL816_STATUS_REG)&PCL816_STATUS_DRDY==0{0}else{-16}}
unsafe fn pcl816_ai_next_chan(_d:*mut ComediDevice,s:*mut ComediSubdevice)->bool{let c=&(*s).async_;if (*c).cmd.stop_src==TRIG_COUNT&&(*c).scans_done>=(*c).cmd.stop_arg{(*c).events|=COMEDI_CB_EOA;false}else{true}}
unsafe fn pcl816_ai_setup_chanlist(d:*mut ComediDevice,l:*mut u32,n:u32){let f=chan(*l);let mut last=0;for i in 0..n{last=chan(*l.add(i));pcl816_ai_set_chan_range(d,last,range(*l.add(i)));}udelay(1);pcl816_ai_set_chan_scan(d,f,last)}
unsafe fn transfer_from_dma_buf(d:*mut ComediDevice,s:*mut ComediSubdevice,p:*mut u16,mut b:u32,n:u32){for _ in 0..n{let v=*p.add(b as usize);b+=1;comedi_buf_write_samples(s,&v,1);if !pcl816_ai_next_chan(d,s){return}}}
unsafe fn pcl816_ai_setup_dma(d:*mut ComediDevice,s:*mut ComediSubdevice,u:u32){let x=&mut *(*(*d).private).dma;let q=&mut *x.desc.add(x.cur_dma);let m=comedi_bytes_to_samples(s,q.maxsize);comedi_isadma_disable(x.chan);let n=comedi_nsamples_left(s,m+u);if n>u{q.size=comedi_samples_to_bytes(s,n-u);comedi_isadma_program(q)}}

unsafe extern "C" fn pcl816_interrupt(_irq:i32,p:*mut core::ffi::c_void)->i32{let d=p as *mut ComediDevice;let s=(*d).read_subdev;let v=&mut *(*d).private;let x=&mut *v.dma;if !(*d).attached||v.ai_cmd_running==0{pcl816_ai_clear_eoc(d);return 1}if v.ai_cmd_canceled!=0{v.ai_cmd_canceled=0;pcl816_ai_clear_eoc(d);return 1}let q=&mut *x.desc.add(x.cur_dma);let n=comedi_bytes_to_samples(s,q.size)-v.ai_poll_ptr;let b=v.ai_poll_ptr;v.ai_poll_ptr=0;x.cur_dma=1-x.cur_dma;pcl816_ai_setup_dma(d,s,n);transfer_from_dma_buf(d,s,q.virt_addr,b,n);pcl816_ai_clear_eoc(d);comedi_handle_events(d,s);1}

unsafe fn check_channel_list(_d:*mut ComediDevice,_s:*mut ComediSubdevice,l:*mut u32,n:u32)->i32{if n<1{return 0}let mut seg=[0u32;16];seg[0]=*l;if n>1{let mut sl=1;for i in 1..n{if *l==*l.add(i){break}let expected=(chan(seg[(i-1)as usize])+1)%n;if expected!=chan(*l.add(i)){return 0}seg[i as usize]=*l.add(i);sl+=1}for i in 0..n{if *l.add(i)!=seg[(i as usize)%sl as usize]{return 0}}}1}

unsafe fn pcl816_ai_cmdtest(d:*mut ComediDevice,s:*mut ComediSubdevice,c:*mut ComediCmd)->i32{let mut e=0;e|=comedi_check_trigger_src(&mut (*c).start_src,TRIG_NOW);e|=comedi_check_trigger_src(&mut (*c).scan_begin_src,TRIG_FOLLOW);e|=comedi_check_trigger_src(&mut (*c).convert_src,TRIG_EXT|TRIG_TIMER);e|=comedi_check_trigger_src(&mut (*c).scan_end_src,TRIG_COUNT);e|=comedi_check_trigger_src(&mut (*c).stop_src,TRIG_COUNT|TRIG_NONE);if e!=0{return 1}e|=comedi_check_trigger_is_unique((*c).convert_src);e|=comedi_check_trigger_is_unique((*c).stop_src);if e!=0{return 2}e|=comedi_check_trigger_arg_is(&mut (*c).start_arg,0);e|=comedi_check_trigger_arg_is(&mut (*c).scan_begin_arg,0);if (*c).convert_src==TRIG_TIMER{e|=comedi_check_trigger_arg_min(&mut (*c).convert_arg,10000)}else{e|=comedi_check_trigger_arg_is(&mut (*c).convert_arg,0)}e|=comedi_check_trigger_arg_is(&mut (*c).scan_end_arg,(*c).chanlist_len);if (*c).stop_src==TRIG_COUNT{e|=comedi_check_trigger_arg_min(&mut (*c).stop_arg,1)}else{e|=comedi_check_trigger_arg_is(&mut (*c).stop_arg,0)}if e!=0{return 3}if (*c).chanlist.is_null()||check_channel_list(d,s,(*c).chanlist,(*c).chanlist_len)>0{0}else{5}}

// Remaining driver entry points preserve the original externally visible names and behavior.
#[no_mangle] pub unsafe extern "C" fn pcl816_ai_cancel(d:*mut ComediDevice,s:*mut ComediSubdevice)->i32{let v=&mut *(*d).private;if v.ai_cmd_running==0{return 0}outb(0,(*d).iobase+PCL816_CTRL_REG);pcl816_ai_clear_eoc(d);comedi_8254_pacer_enable((*d).pacer,1,2,false);v.ai_cmd_running=0;v.ai_cmd_canceled=1;0}
#[no_mangle] pub unsafe extern "C" fn pcl816_ai_insn_read(d:*mut ComediDevice,s:*mut ComediSubdevice,i:*mut ComediInsn,data:*mut u32)->i32{let c=chan((*i).chanspec);let r=range((*i).chanspec);outb(PCL816_CTRL_SOFT_TRIG,(*d).iobase+PCL816_CTRL_REG);pcl816_ai_set_chan_range(d,c,r);pcl816_ai_set_chan_scan(d,c,c);let mut ret=0;for n in 0..(*i).n{pcl816_ai_clear_eoc(d);pcl816_ai_soft_trig(d);ret=comedi_timeout(d,s,i,pcl816_ai_eoc,0);if ret!=0{break}*data.add(n as usize)=pcl816_ai_get_sample(d,s)}outb(0,(*d).iobase+PCL816_CTRL_REG);pcl816_ai_clear_eoc(d);if ret!=0{ret}else{(*i).n as i32}}
#[no_mangle] pub unsafe extern "C" fn pcl816_di_insn_bits(d:*mut ComediDevice,_s:*mut ComediSubdevice,i:*mut ComediInsn,data:*mut u32)->i32{*data.add(1)=inb((*d).iobase)|inb((*d).iobase+1)<<8;(*i).n as i32}
#[no_mangle] pub unsafe extern "C" fn pcl816_do_insn_bits(d:*mut ComediDevice,s:*mut ComediSubdevice,i:*mut ComediInsn,data:*mut u32)->i32{if comedi_dio_update_state(s,data){outb((*s).state&0xff,(*d).iobase);outb((*s).state>>8,(*d).iobase+1)}*data.add(1)=(*s).state;(*i).n as i32}
#[no_mangle] pub unsafe extern "C" fn pcl816_reset(d:*mut ComediDevice){outb(0,(*d).iobase+PCL816_CTRL_REG);pcl816_ai_set_chan_range(d,0,0);pcl816_ai_clear_eoc(d);outb(0,(*d).iobase);outb(0,(*d).iobase+1)}
#[no_mangle] pub unsafe extern "C" fn pcl816_ai_poll(d:*mut ComediDevice,s:*mut ComediSubdevice)->i32{let v=&mut *(*d).private;let x=&mut *v.dma;let p=comedi_bytes_to_samples(s,comedi_isadma_poll(x));if p>v.ai_poll_ptr{let q=&mut *x.desc.add(x.cur_dma);transfer_from_dma_buf(d,s,q.virt_addr,v.ai_poll_ptr,p-v.ai_poll_ptr);v.ai_poll_ptr=p;comedi_handle_events(d,s);comedi_buf_n_bytes_ready(s)}else{0}}
#[no_mangle] pub unsafe extern "C" fn pcl816_ai_cmd(d:*mut ComediDevice,s:*mut ComediSubdevice)->i32{let v=&mut *(*d).private;let x=&mut *v.dma;let c=&mut (*(*s).async_).cmd;if v.ai_cmd_running!=0{return -16}let n=check_channel_list(d,s,c.chanlist,c.chanlist_len);if n<1{return -22}pcl816_ai_setup_chanlist(d,c.chanlist,n as u32);udelay(1);v.ai_cmd_running=1;v.ai_poll_ptr=0;v.ai_cmd_canceled=0;x.cur_dma=0;pcl816_ai_setup_dma(d,s,0);comedi_8254_set_mode((*d).pacer,0,I8254_MODE1|I8254_BINARY);comedi_8254_write((*d).pacer,0,0x0ff);udelay(1);comedi_8254_update_divisors((*d).pacer);comedi_8254_pacer_enable((*d).pacer,1,2,true);let mut ctrl=PCL816_CTRL_INTEN|PCL816_CTRL_DMAEN;if c.convert_src==TRIG_TIMER{ctrl|=PCL816_CTRL_PACER_TRIG}else{ctrl|=PCL816_CTRL_EXT_TRIG}outb(ctrl,(*d).iobase+PCL816_CTRL_REG);outb((x.chan<<4)|(*d).irq,(*d).iobase+PCL816_STATUS_REG);0}
#[no_mangle] pub unsafe extern "C" fn pcl816_alloc_irq_and_dma(d:*mut ComediDevice,it:*mut ComediDevconfig){let v=&mut *(*d).private;let irq=(*it).options[1];let dma=(*it).options[2];if !(irq>=2&&irq<=7)||(dma!=3&&dma!=1){return}if request_irq(irq,pcl816_interrupt,0,(*d).board_name,d)!=0{return}v.dma=comedi_isadma_alloc(d,2,dma,dma,16384,1);if v.dma.is_null(){free_irq(irq,d)}else{(*d).irq=irq}}
#[no_mangle] pub unsafe extern "C" fn pcl816_free_dma(d:*mut ComediDevice){if !(*d).private.is_null(){comedi_isadma_free((*(*d).private).dma)}}
#[no_mangle] pub unsafe extern "C" fn pcl816_attach(d:*mut ComediDevice,it:*mut ComediDevconfig)->i32{let v=comedi_alloc_devpriv(d,core::mem::size_of::<pcl816_private>());if v.is_null(){return -12}let r=comedi_check_request_region(d,(*it).options[0],0x10,0,0x3ff,16);if r!=0{return r}pcl816_alloc_irq_and_dma(d,it);(*d).pacer=comedi_8254_io_alloc((*d).iobase+PCL816_TIMER_BASE,10000000,8,0);if (*d).pacer.is_null(){return -1}let r=comedi_alloc_subdevices(d,4);if r!=0{return r}pcl816_reset(d);0}
#[no_mangle] pub unsafe extern "C" fn pcl816_detach(d:*mut ComediDevice){if !(*d).private.is_null(){pcl816_ai_cancel(d,(*d).read_subdev);pcl816_reset(d)}pcl816_free_dma(d);comedi_legacy_detach(d)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
