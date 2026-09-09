// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of pcmuio.c. */

const PCMUIO_PORT_REG: fn(i32) -> i32 = |x| 0x00 + x;
const PCMUIO_INT_PENDING_REG: i32 = 0x06;
const PCMUIO_PAGE_LOCK_REG: i32 = 0x07;
const PCMUIO_LOCK_PORT: fn(i32) -> i32 = |x| ((1 << x) & 0x3f);
const PCMUIO_PAGE: fn(i32) -> i32 = |x| ((x & 3) << 6);
const PCMUIO_PAGE_MASK: i32 = 0xc0;
const PCMUIO_PAGE_POL: i32 = 1;
const PCMUIO_PAGE_ENAB: i32 = 2;
const PCMUIO_PAGE_INT_ID: i32 = 3;
const PCMUIO_PAGE_REG: fn(i32) -> i32 = |x| 0x08 + x;
const PCMUIO_ASIC_IOSIZE: i32 = 0x10;
const PCMUIO_MAX_ASICS: usize = 2;

#[repr(C)]
struct pcmuio_board { name: *const i8, num_asics: i32 }

static pcmuio_boards: [pcmuio_board; 2] = [
    pcmuio_board { name: b"pcmuio48\0".as_ptr() as *const i8, num_asics: 1 },
    pcmuio_board { name: b"pcmuio96\0".as_ptr() as *const i8, num_asics: 2 },
];

#[repr(C)]
struct pcmuio_asic { pagelock: spinlock_t, spinlock: spinlock_t, enabled_mask: u32, active: u32 }
#[repr(C)]
struct pcmuio_private { asics: [pcmuio_asic; PCMUIO_MAX_ASICS], irq2: u32 }

unsafe fn pcmuio_asic_iobase(dev: *mut comedi_device, asic: i32) -> usize {
    (*dev).iobase + (asic as usize * PCMUIO_ASIC_IOSIZE as usize)
}
unsafe fn pcmuio_subdevice_to_asic(s: *mut comedi_subdevice) -> i32 { (*s).index / 2 }
unsafe fn pcmuio_subdevice_to_port(s: *mut comedi_subdevice) -> i32 { if (*s).index % 2 != 0 { 3 } else { 0 } }

unsafe fn pcmuio_write(dev: *mut comedi_device, val: u32, asic: i32, page: i32, port: i32) {
    let p = (*dev).private as *mut pcmuio_private; let chip = &mut (*p).asics[asic as usize];
    let base = pcmuio_asic_iobase(dev, asic); let mut flags: usize = 0;
    spin_lock_irqsave(&mut chip.pagelock, &mut flags);
    if page == 0 { outb((val & 0xff) as u8, base + PCMUIO_PORT_REG(port) as usize); outb(((val >> 8) & 0xff) as u8, base + PCMUIO_PORT_REG(port+1) as usize); outb(((val >> 16) & 0xff) as u8, base + PCMUIO_PORT_REG(port+2) as usize); }
    else { outb(PCMUIO_PAGE(page) as u8, base + PCMUIO_PAGE_LOCK_REG as usize); outb(val as u8, base + PCMUIO_PAGE_REG(0) as usize); outb((val >> 8) as u8, base + PCMUIO_PAGE_REG(1) as usize); outb((val >> 16) as u8, base + PCMUIO_PAGE_REG(2) as usize); }
    spin_unlock_irqrestore(&mut chip.pagelock, flags);
}
unsafe fn pcmuio_read(dev: *mut comedi_device, asic: i32, page: i32, port: i32) -> u32 {
    let p = (*dev).private as *mut pcmuio_private; let chip = &mut (*p).asics[asic as usize]; let base = pcmuio_asic_iobase(dev, asic); let mut flags=0; let mut val;
    spin_lock_irqsave(&mut chip.pagelock, &mut flags);
    if page == 0 { val=inb(base+PCMUIO_PORT_REG(port) as usize) as u32; val|=(inb(base+PCMUIO_PORT_REG(port+1) as usize) as u32)<<8; val|=(inb(base+PCMUIO_PORT_REG(port+2) as usize) as u32)<<16; }
    else { outb(PCMUIO_PAGE(page) as u8,base+PCMUIO_PAGE_LOCK_REG as usize); val=inb(base+PCMUIO_PAGE_REG(0) as usize) as u32; val|=(inb(base+PCMUIO_PAGE_REG(1) as usize) as u32)<<8; val|=(inb(base+PCMUIO_PAGE_REG(2) as usize) as u32)<<16; }
    spin_unlock_irqrestore(&mut chip.pagelock,flags); val
}

unsafe fn pcmuio_dio_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let a=pcmuio_subdevice_to_asic(s); let p=pcmuio_subdevice_to_port(s); let mask=(1u32<<(*s).n_chan)-1; let m=comedi_dio_update_state(s,data); if m!=0 { let mut v=!(*s).state&mask; v&=(*s).io_bits; pcmuio_write(dev,v,a,0,p); } *data.add(1)=!pcmuio_read(dev,a,0,p)&mask; (*insn).n }
unsafe fn pcmuio_dio_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let a=pcmuio_subdevice_to_asic(s); let p=pcmuio_subdevice_to_port(s); let r=comedi_dio_insn_config(dev,s,insn,data,0); if r!=0{return r} if *data==INSN_CONFIG_DIO_INPUT {pcmuio_write(dev,(*s).io_bits,a,0,p)} (*insn).n }
unsafe fn pcmuio_reset(dev:*mut comedi_device) { let b=&*((*dev).board_ptr); for a in 0..b.num_asics { pcmuio_write(dev,0,a,0,0); pcmuio_write(dev,0,a,0,3); pcmuio_write(dev,0,a,1,0); pcmuio_write(dev,0,a,2,0); pcmuio_write(dev,0,a,3,0); } }

unsafe fn pcmuio_stop_intr(dev:*mut comedi_device,s:*mut comedi_subdevice) { let p=(*dev).private as *mut pcmuio_private; let c=&mut (*p).asics[pcmuio_subdevice_to_asic(s) as usize]; c.enabled_mask=0;c.active=0;(*s).async_.inttrig=None;pcmuio_write(dev,0,pcmuio_subdevice_to_asic(s),2,0); }
unsafe fn pcmuio_handle_intr_subdev(dev:*mut comedi_device,s:*mut comedi_subdevice,triggered:u32) { let p=(*dev).private as *mut pcmuio_private; let c=&mut (*p).asics[pcmuio_subdevice_to_asic(s) as usize]; let cmd=&(*s).async_.cmd; let mut val=0; let mut flags=0; spin_lock_irqsave(&mut c.spinlock,&mut flags); if c.active && triggered&c.enabled_mask!=0 { for i in 0..cmd.chanlist_len { let ch=CR_CHAN(*cmd.chanlist.add(i as usize)); if triggered&(1<<ch)!=0 {val|=1<<i;} } comedi_buf_write_samples(s,&val,1); if cmd.stop_src==TRIG_COUNT && (*s).async_.scans_done>=cmd.stop_arg {(*s).async_.events|=COMEDI_CB_EOA;} } spin_unlock_irqrestore(&mut c.spinlock,flags); comedi_handle_events(dev,s); }
unsafe fn pcmuio_handle_asic_interrupt(dev:*mut comedi_device,asic:i32)->i32 { let s=(*dev).subdevices.add((asic*2) as usize); let b=pcmuio_asic_iobase(dev,asic); if inb(b+PCMUIO_INT_PENDING_REG as usize)&7==0{return 0} let v=pcmuio_read(dev,asic,3,0);pcmuio_write(dev,0,asic,3,0);pcmuio_handle_intr_subdev(dev,s,v);1 }
unsafe extern "C" fn pcmuio_interrupt(irq:i32,d:*mut core::ffi::c_void)->irqreturn_t { let dev=d as *mut comedi_device;if !(*dev).attached{return IRQ_NONE} let p=(*dev).private as *mut pcmuio_private;let mut h=0;if irq==(*dev).irq{h+=pcmuio_handle_asic_interrupt(dev,0)}if irq==(*p).irq2{h+=pcmuio_handle_asic_interrupt(dev,1)}IRQ_RETVAL(h) }
unsafe fn pcmuio_start_intr(dev:*mut comedi_device,s:*mut comedi_subdevice) { let p=(*dev).private as *mut pcmuio_private;let c=&mut (*p).asics[pcmuio_subdevice_to_asic(s) as usize];let cmd=&(*s).async_.cmd;let mut bits=0;let mut pol=0;if !cmd.chanlist.is_null(){for i in 0..cmd.chanlist_len{let x=*cmd.chanlist.add(i as usize);let ch=CR_CHAN(x);bits|=1<<ch;pol|=((CR_AREF(x)!=0||CR_RANGE(x)!=0) as u32)<<ch;}}bits&=(1<<(*s).n_chan)-1;c.enabled_mask=bits;pcmuio_write(dev,pol,pcmuio_subdevice_to_asic(s),1,0);pcmuio_write(dev,bits,pcmuio_subdevice_to_asic(s),2,0);}
unsafe fn pcmuio_cancel(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32 {let p=(*dev).private as *mut pcmuio_private;let c=&mut (*p).asics[pcmuio_subdevice_to_asic(s) as usize];let mut f=0;spin_lock_irqsave(&mut c.spinlock,&mut f);if c.active!=0{pcmuio_stop_intr(dev,s)}spin_unlock_irqrestore(&mut c.spinlock,f);0}
unsafe fn pcmuio_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32 {let p=(*dev).private as *mut pcmuio_private;let c=&mut (*p).asics[pcmuio_subdevice_to_asic(s) as usize];let mut f=0;spin_lock_irqsave(&mut c.spinlock,&mut f);c.active=1;if (*s).async_.cmd.start_src==TRIG_INT{(*s).async_.inttrig=Some(pcmuio_inttrig_start_intr)}else{pcmuio_start_intr(dev,s)}spin_unlock_irqrestore(&mut c.spinlock,f);0}
unsafe fn pcmuio_inttrig_start_intr(dev:*mut comedi_device,s:*mut comedi_subdevice,n:u32)->i32 {if n!=(*s).async_.cmd.start_arg{return -EINVAL}(*s).async_.inttrig=None;pcmuio_start_intr(dev,s);1}
unsafe fn pcmuio_cmdtest(_: *mut comedi_device,_:*mut comedi_subdevice,c:*mut comedi_cmd)->i32 {let mut e=0;e|=comedi_check_trigger_src(&mut (*c).start_src,TRIG_NOW|TRIG_INT);e|=comedi_check_trigger_src(&mut (*c).scan_begin_src,TRIG_EXT);e|=comedi_check_trigger_src(&mut (*c).convert_src,TRIG_NOW);e|=comedi_check_trigger_src(&mut (*c).scan_end_src,TRIG_COUNT);e|=comedi_check_trigger_src(&mut (*c).stop_src,TRIG_COUNT|TRIG_NONE);if e!=0{return 1}e|=comedi_check_trigger_is_unique((*c).start_src);e|=comedi_check_trigger_is_unique((*c).stop_src);if e!=0{return 2}e|=comedi_check_trigger_arg_is(&mut (*c).start_arg,0);e|=comedi_check_trigger_arg_is(&mut (*c).scan_begin_arg,0);e|=comedi_check_trigger_arg_is(&mut (*c).convert_arg,0);e|=comedi_check_trigger_arg_is(&mut (*c).scan_end_arg,(*c).chanlist_len);if (*c).stop_src==TRIG_COUNT{e|=comedi_check_trigger_arg_min(&mut (*c).stop_arg,1)}else{e|=comedi_check_trigger_arg_is(&mut (*c).stop_arg,0)}if e!=0{3}else{0}}

// External kernel/comedi declarations and the attach/detach registration are
// represented by the target tree's ABI; no dependency implementations are
// introduced in this translation unit.
unsafe fn pcmuio_attach(dev:*mut comedi_device,it:*mut comedi_devconfig)->i32 { let b=&*((*dev).board_ptr); let len=b.num_asics as usize*PCMUIO_ASIC_IOSIZE as usize; let r=comedi_check_request_region(dev,(*it).options[0],len,0,0xffff,len);if r!=0{return r}let p=comedi_alloc_devpriv(dev,core::mem::size_of::<pcmuio_private>()) as *mut pcmuio_private;if p.is_null(){return -ENOMEM}pcmuio_reset(dev);comedi_alloc_subdevices(dev,(b.num_asics*2) as usize)}
unsafe fn pcmuio_detach(dev:*mut comedi_device){let p=(*dev).private as *mut pcmuio_private;if !p.is_null(){pcmuio_reset(dev)}comedi_legacy_detach(dev)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
