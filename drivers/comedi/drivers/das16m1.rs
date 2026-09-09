// SPDX-License-Identifier: GPL-2.0+
/* Comedi driver for CIO-DAS16/M1. Faithful low-level Rust translation. */

// Linux/Comedi dependencies are supplied by the surrounding repository.
use crate::*;

const DAS16M1_AI_REG: u16 = 0x00;
const DAS16M1_CS_REG: u16 = 0x02;
const DAS16M1_CS_EXT_TRIG: u8 = 1 << 0;
const DAS16M1_CS_OVRUN: u8 = 1 << 5;
const DAS16M1_CS_IRQDATA: u8 = 1 << 7;
const DAS16M1_DI_REG: u16 = 0x03;
const DAS16M1_DO_REG: u16 = 0x03;
const DAS16M1_CLR_INTR_REG: u16 = 0x04;
const DAS16M1_INTR_CTRL_REG: u16 = 0x05;
const DAS16M1_INTR_CTRL_PACER_EXT: u32 = 2;
const DAS16M1_INTR_CTRL_PACER_INT: u32 = 3;
const DAS16M1_INTR_CTRL_PACER_MASK: u32 = 3;
const DAS16M1_INTR_CTRL_INTE: u32 = 1 << 7;
const DAS16M1_Q_ADDR_REG: u16 = 0x06;
const DAS16M1_Q_REG: u16 = 0x07;
const DAS16M1_8254_IOBASE1: u16 = 0x08;
const DAS16M1_8254_IOBASE2: u16 = 0x0c;
const DAS16M1_8255_IOBASE: u16 = 0x400;
const DAS16M1_8254_IOBASE3: u16 = 0x404;
const DAS16M1_SIZE2: usize = 0x08;
const DAS16M1_AI_FIFO_SZ: usize = 1024;

#[inline] fn das16m1_ai_to_chan(x: u16) -> u16 { (x >> 0) & 0xf }
#[inline] fn das16m1_ai_to_sample(x: u16) -> u16 { (x >> 4) & 0xfff }
#[inline] fn das16m1_intr_ctrl_pacer(x: u32) -> u32 { (x & 0x3) << 0 }
#[inline] fn das16m1_intr_ctrl_irq(x: u32) -> u32 { (x & 0x7) << 4 }
#[inline] fn das16m1_q_chan(x: u32) -> u8 { ((x & 0x7) << 0) as u8 }
#[inline] fn das16m1_q_range(x: u32) -> u8 { ((x & 0xf) << 4) as u8 }

static RANGE_DAS16M1: comedi_lrange = comedi_lrange { length: 9, range: [
    BIP_RANGE!(5), BIP_RANGE!(2.5), BIP_RANGE!(1.25), BIP_RANGE!(0.625),
    UNI_RANGE!(10), UNI_RANGE!(5), UNI_RANGE!(2.5), UNI_RANGE!(1.25), BIP_RANGE!(10)
] };

#[repr(C)]
struct das16m1_private {
    counter: *mut comedi_8254,
    intr_ctrl: u32,
    adc_count: u32,
    initial_hw_count: u16,
    ai_buffer: [u16; DAS16M1_AI_FIFO_SZ],
    extra_iobase: usize,
}

unsafe fn das16m1_ai_set_queue(dev: *mut comedi_device, chanspec: *mut u32, len: u32) {
    for i in 0..len {
        let cs = *chanspec.add(i as usize);
        outb(i as u8, (*dev).iobase + DAS16M1_Q_ADDR_REG as usize);
        outb(das16m1_q_chan(CR_CHAN!(cs)) | das16m1_q_range(CR_RANGE!(cs)),
             (*dev).iobase + DAS16M1_Q_REG as usize);
    }
}

unsafe fn das16m1_ai_munge(_dev: *mut comedi_device, s: *mut comedi_subdevice,
                           data: *mut u8, num_bytes: u32, _start_chan_index: u32) {
    let array = data as *mut u16;
    let nsamples = comedi_bytes_to_samples(s, num_bytes);
    for i in 0..nsamples { *array.add(i as usize) = das16m1_ai_to_sample(*array.add(i as usize)); }
}

unsafe fn das16m1_ai_check_chanlist(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                                    cmd: *mut comedi_cmd) -> i32 {
    if (*cmd).chanlist_len == 1 { return 0; }
    if (*cmd).chanlist_len % 2 != 0 { dev_dbg!((*dev).class_dev, "chanlist must be of even length or length 1\n"); return -EINVAL; }
    for i in 0..(*cmd).chanlist_len {
        let chan = CR_CHAN!(*(*cmd).chanlist.add(i as usize));
        if (i % 2) != (chan % 2) { dev_dbg!((*dev).class_dev, "even/odd channels must go have even/odd chanlist indices\n"); return -EINVAL; }
    }
    0
}

unsafe fn das16m1_ai_cmdtest(dev: *mut comedi_device, s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> i32 {
    let mut err = 0;
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW | TRIG_EXT);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_FOLLOW);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_TIMER | TRIG_EXT);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_COUNT | TRIG_NONE);
    if err != 0 { return 1; }
    err |= comedi_check_trigger_is_unique((*cmd).start_src);
    err |= comedi_check_trigger_is_unique((*cmd).convert_src);
    err |= comedi_check_trigger_is_unique((*cmd).stop_src);
    if err != 0 { return 2; }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0);
    if (*cmd).scan_begin_src == TRIG_FOLLOW { err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0); }
    if (*cmd).convert_src == TRIG_TIMER { err |= comedi_check_trigger_arg_min(&mut (*cmd).convert_arg, 1000); }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len);
    if (*cmd).stop_src == TRIG_COUNT { err |= comedi_check_trigger_arg_min(&mut (*cmd).stop_arg, 1); }
    else { err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0); }
    if err != 0 { return 3; }
    if (*cmd).convert_src == TRIG_TIMER {
        let mut arg = (*cmd).convert_arg;
        comedi_8254_cascade_ns_to_timer((*dev).pacer, &mut arg, (*cmd).flags);
        err |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, arg);
    }
    if err != 0 { return 4; }
    if !(*cmd).chanlist.is_null() && (*cmd).chanlist_len > 0 { err |= das16m1_ai_check_chanlist(dev, s, cmd); }
    if err != 0 { return 5; } 0
}

unsafe fn das16m1_ai_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 {
    let p = (*dev).private as *mut das16m1_private; let async_ = (*s).async_; let cmd = &mut (*async_).cmd;
    (*p).adc_count = 0;
    comedi_8254_set_mode((*p).counter, 1, I8254_MODE2 | I8254_BINARY); comedi_8254_write((*p).counter, 1, 0);
    (*p).initial_hw_count = comedi_8254_read((*p).counter, 1);
    das16m1_ai_set_queue(dev, cmd.chanlist, cmd.chanlist_len);
    (*p).intr_ctrl &= !DAS16M1_INTR_CTRL_PACER_MASK;
    if cmd.convert_src == TRIG_TIMER { comedi_8254_update_divisors((*dev).pacer); comedi_8254_pacer_enable((*dev).pacer, 1, 2, true); (*p).intr_ctrl |= das16m1_intr_ctrl_pacer(DAS16M1_INTR_CTRL_PACER_INT); }
    else { (*p).intr_ctrl |= das16m1_intr_ctrl_pacer(DAS16M1_INTR_CTRL_PACER_EXT); }
    let mut byte = 0u8; if cmd.start_src == TRIG_EXT && cmd.convert_src != TRIG_EXT { byte |= DAS16M1_CS_EXT_TRIG; }
    outb(byte, (*dev).iobase + DAS16M1_CS_REG as usize); outb(0, (*dev).iobase + DAS16M1_CLR_INTR_REG as usize);
    (*p).intr_ctrl |= DAS16M1_INTR_CTRL_INTE; outb((*p).intr_ctrl as u8, (*dev).iobase + DAS16M1_INTR_CTRL_REG as usize); 0
}

unsafe fn das16m1_ai_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 { let p=(*dev).private as *mut das16m1_private; (*p).intr_ctrl &= !(DAS16M1_INTR_CTRL_INTE|DAS16M1_INTR_CTRL_PACER_MASK); outb((*p).intr_ctrl as u8,(*dev).iobase+DAS16M1_INTR_CTRL_REG as usize); 0 }
unsafe fn das16m1_ai_eoc(dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:usize)->i32 { if inb((*dev).iobase+DAS16M1_CS_REG as usize)&DAS16M1_CS_IRQDATA != 0 {0} else {-EBUSY} }

unsafe fn das16m1_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { das16m1_ai_set_queue(dev,&mut (*insn).chanspec,1); for i in 0..(*insn).n { outb(0,(*dev).iobase+DAS16M1_CLR_INTR_REG as usize); outb(0,(*dev).iobase+DAS16M1_AI_REG as usize); let r=comedi_timeout(dev,s,insn,das16m1_ai_eoc,0); if r!=0{return r;} *data.add(i as usize)=das16m1_ai_to_sample(inw((*dev).iobase+DAS16M1_AI_REG as usize)) as u32;} (*insn).n as i32 }
unsafe fn das16m1_di_insn_bits(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { *data.add(1)=(inb((*dev).iobase+DAS16M1_DI_REG as usize)&0xf) as u32; (*insn).n as i32 }
unsafe fn das16m1_do_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { if comedi_dio_update_state(s,data)!=0 {outb((*s).state as u8,(*dev).iobase+DAS16M1_DO_REG as usize);} *data.add(1)=(*s).state; (*insn).n as i32 }

unsafe fn das16m1_handler(dev:*mut comedi_device,status:u32) {
    let p=(*dev).private as *mut das16m1_private; let s=(*dev).read_subdev; let a=(*s).async_; let c=&(*a).cmd;
    let hw=comedi_8254_read((*p).counter,1); let mut n:u16;
    if (*p).adc_count==0 && hw==(*p).initial_hw_count { n=0; } else { n=0u16.wrapping_sub(hw).wrapping_sub((*p).adc_count as u16); }
    if c.stop_src==TRIG_COUNT { let lim=c.stop_arg.wrapping_mul(c.chanlist_len); if n as u32>lim {n=lim as u16;} }
    if n as usize>DAS16M1_AI_FIFO_SZ {n=DAS16M1_AI_FIFO_SZ as u16;}
    insw((*dev).iobase,(*p).ai_buffer.as_mut_ptr(),n); comedi_buf_write_samples(s,(*p).ai_buffer.as_mut_ptr(),n as u32); (*p).adc_count=(*p).adc_count.wrapping_add(n as u32);
    if c.stop_src==TRIG_COUNT && (*p).adc_count>=c.stop_arg*c.chanlist_len {(*a).events|=COMEDI_CB_EOA;}
    if status & DAS16M1_CS_OVRUN as u32 != 0 {(*a).events|=COMEDI_CB_ERROR; dev_err!((*dev).class_dev,"fifo overflow\n");} comedi_handle_events(dev,s);
}
unsafe fn das16m1_ai_poll(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32 { let mut f=0usize; spin_lock_irqsave(&mut (*dev).spinlock,&mut f); let st=inb((*dev).iobase+DAS16M1_CS_REG as usize) as u32; das16m1_handler(dev,st); spin_unlock_irqrestore(&mut (*dev).spinlock,f); comedi_buf_n_bytes_ready(s) }
unsafe extern "C" fn das16m1_interrupt(_irq:i32,d:*mut core::ffi::c_void)->irqreturn_t { let dev=d as *mut comedi_device; if !(*dev).attached {dev_err!((*dev).class_dev,"premature interrupt\n");return IRQ_HANDLED;} spin_lock(&mut (*dev).spinlock); let st=inb((*dev).iobase+DAS16M1_CS_REG as usize) as u32; if st&(DAS16M1_CS_IRQDATA as u32|DAS16M1_CS_OVRUN as u32)==0 {dev_err!((*dev).class_dev,"spurious interrupt\n");spin_unlock(&mut (*dev).spinlock);return IRQ_NONE;} das16m1_handler(dev,st);outb(0,(*dev).iobase+DAS16M1_CLR_INTR_REG as usize);spin_unlock(&mut (*dev).spinlock);IRQ_HANDLED }

unsafe fn das16m1_irq_bits(irq:u32)->i32 { match irq {10=>0,11=>1,12=>2,15=>3,2=>4,3=>5,5=>6,7=>7,_=>0} }

#[no_mangle] pub static mut das16m1_driver: comedi_driver = comedi_driver { driver_name: "das16m1", module: THIS_MODULE, attach: Some(das16m1_attach), detach: Some(das16m1_detach) };

unsafe extern "C" fn das16m1_attach(dev:*mut comedi_device,it:*mut comedi_devconfig)->i32 {
    let p=comedi_alloc_devpriv(dev,core::mem::size_of::<das16m1_private>()) as *mut das16m1_private; if p.is_null(){return -ENOMEM;} let r=comedi_check_request_region(dev,(*it).options[0],0x10,0,0x3ff,16); if r!=0{return r;}
    (*p).extra_iobase=(*dev).iobase+DAS16M1_8255_IOBASE as usize; let irq=(*it).options[1]; if irq>=2&&irq<=15&&((1u32<<irq)&0xdcfc)!=0 { if request_irq(irq,das16m1_interrupt,0,(*dev).board_name,dev)==0 {(*dev).irq=irq;} }
    (*dev).pacer=comedi_8254_io_alloc((*dev).iobase+DAS16M1_8254_IOBASE2 as usize,I8254_OSC_BASE_10MHZ,I8254_IO8,0); (*p).counter=comedi_8254_io_alloc((*dev).iobase+DAS16M1_8254_IOBASE1 as usize,0,I8254_IO8,0); if comedi_alloc_subdevices(dev,4)!=0{return -ENOMEM;}
    let s=&mut *(*dev).subdevices; s.type_=COMEDI_SUBD_AI;s.subdev_flags=SDF_READABLE|SDF_DIFF;s.n_chan=8;s.maxdata=0xfff;s.range_table=&RANGE_DAS16M1 as *const _;s.insn_read=Some(das16m1_ai_insn_read); if (*dev).irq!=0 {(*dev).read_subdev=s;s.subdev_flags|=SDF_CMD_READ;s.len_chanlist=256;s.do_cmdtest=Some(das16m1_ai_cmdtest);s.do_cmd=Some(das16m1_ai_cmd);s.cancel=Some(das16m1_ai_cancel);s.poll=Some(das16m1_ai_poll);s.munge=Some(das16m1_ai_munge);} (*p).intr_ctrl=das16m1_intr_ctrl_irq(das16m1_irq_bits((*dev).irq) as u32);outb((*p).intr_ctrl as u8,(*dev).iobase+DAS16M1_INTR_CTRL_REG as usize);0
}
unsafe extern "C" fn das16m1_detach(dev:*mut comedi_device) { let p=(*dev).private as *mut das16m1_private; if !p.is_null()&&(*p).extra_iobase!=0 {release_region((*p).extra_iobase,DAS16M1_SIZE2);} comedi_legacy_detach(dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
