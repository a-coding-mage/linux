// SPDX-License-Identifier: GPL-2.0+
/* Literal Rust translation of addi_apci_1500.c. */

const APCI1500_Z8536_PORTC_REG: u32 = 0x00;
const APCI1500_Z8536_PORTB_REG: u32 = 0x01;
const APCI1500_Z8536_PORTA_REG: u32 = 0x02;
const APCI1500_Z8536_CTRL_REG: u32 = 0x03;
const APCI1500_CLK_SEL_REG: u32 = 0x00;
const APCI1500_DI_REG: u32 = 0x00;
const APCI1500_DO_REG: u32 = 0x02;

#[repr(C)]
struct apci1500_private {
    amcc: libc::c_ulong,
    addon: libc::c_ulong,
    clk_src: libc::c_uint,
    pm: [libc::c_uint; 2],
    pt: [libc::c_uint; 2],
    pp: [libc::c_uint; 2],
}

unsafe fn z8536_read(dev: *mut comedi_device, reg: libc::c_uint) -> libc::c_uint {
    let mut flags: libc::c_ulong = 0;
    spin_lock_irqsave(&mut (*dev).spinlock, &mut flags);
    outb(reg as u8, (*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    let val = inb((*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong) as libc::c_uint;
    spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    val
}

unsafe fn z8536_write(dev: *mut comedi_device, val: libc::c_uint, reg: libc::c_uint) {
    let mut flags: libc::c_ulong = 0;
    spin_lock_irqsave(&mut (*dev).spinlock, &mut flags);
    outb(reg as u8, (*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    outb(val as u8, (*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
}

unsafe fn z8536_reset(dev: *mut comedi_device) {
    let mut flags: libc::c_ulong = 0;
    spin_lock_irqsave(&mut (*dev).spinlock, &mut flags);
    inb((*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    outb(0, (*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    inb((*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    outb(0, (*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    outb(1, (*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    outb(0, (*dev).iobase + APCI1500_Z8536_CTRL_REG as libc::c_ulong);
    spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    z8536_write(dev, 0x00, Z8536_CFG_CTRL_REG);
    z8536_write(dev, Z8536_PAB_MODE_PTS_BIT | Z8536_PAB_MODE_SB | Z8536_PAB_MODE_PMS_DISABLE, Z8536_PA_MODE_REG);
    z8536_write(dev, 0xff, Z8536_PB_DPP_REG);
    z8536_write(dev, 0xff, Z8536_PA_DD_REG);
    z8536_write(dev, Z8536_PAB_MODE_PTS_BIT | Z8536_PAB_MODE_SB | Z8536_PAB_MODE_PMS_DISABLE, Z8536_PB_MODE_REG);
    z8536_write(dev, 0x7f, Z8536_PB_DPP_REG);
    z8536_write(dev, 0xff, Z8536_PB_DD_REG);
    z8536_write(dev, 0x09, Z8536_PC_DPP_REG);
    z8536_write(dev, 0x0e, Z8536_PC_DD_REG);
    for reg in [Z8536_PA_CMDSTAT_REG, Z8536_PB_CMDSTAT_REG, Z8536_CT_CMDSTAT_REG(0), Z8536_CT_CMDSTAT_REG(1), Z8536_CT_CMDSTAT_REG(2)] {
        z8536_write(dev, Z8536_CMD_CLR_IP_IUS, reg);
        z8536_write(dev, Z8536_CMD_CLR_IE, reg);
    }
    z8536_write(dev, 0x00, Z8536_INT_CTRL_REG);
}

unsafe fn apci1500_port_enable(dev: *mut comedi_device, enable: bool) {
    let mut cfg = z8536_read(dev, Z8536_CFG_CTRL_REG);
    if enable { cfg |= Z8536_CFG_CTRL_PAE | Z8536_CFG_CTRL_PBE; }
    else { cfg &= !(Z8536_CFG_CTRL_PAE | Z8536_CFG_CTRL_PBE); }
    z8536_write(dev, cfg, Z8536_CFG_CTRL_REG);
}

unsafe fn apci1500_timer_enable(dev: *mut comedi_device, chan: libc::c_uint, enable: bool) {
    let bit = if chan == 0 { Z8536_CFG_CTRL_CT1E } else if chan == 1 { Z8536_CFG_CTRL_CT2E } else { Z8536_CFG_CTRL_PCE_CT3E };
    let mut cfg = z8536_read(dev, Z8536_CFG_CTRL_REG);
    if enable { cfg |= bit; } else { cfg &= !bit; z8536_write(dev, 0, Z8536_CT_CMDSTAT_REG(chan)); }
    z8536_write(dev, cfg, Z8536_CFG_CTRL_REG);
}

unsafe fn apci1500_ack_irq(dev: *mut comedi_device, reg: libc::c_uint) -> bool {
    let mut val = z8536_read(dev, reg);
    if val & Z8536_STAT_IE_IP == Z8536_STAT_IE_IP {
        val = (val & 0x0f) | Z8536_CMD_CLR_IP_IUS;
        z8536_write(dev, val, reg);
        true
    } else { false }
}

unsafe extern "C" fn apci1500_interrupt(_irq: libc::c_int, d: *mut libc::c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    let devpriv = (*dev).private as *mut apci1500_private;
    let s = (*dev).read_subdev;
    let mut status: u16 = 0;
    let mut val = inl((*devpriv).amcc + AMCC_OP_REG_INTCSR as libc::c_ulong);
    if val & INTCSR_INTR_ASSERTED == 0 { return IRQ_NONE; }
    if apci1500_ack_irq(dev, Z8536_PA_CMDSTAT_REG) { status |= 1; }
    if apci1500_ack_irq(dev, Z8536_PB_CMDSTAT_REG) {
        val = inb((*dev).iobase + APCI1500_Z8536_PORTB_REG as libc::c_ulong) as u32 & 0xc0;
        if val != 0 { if val & 0x80 != 0 { status |= 0x40; } if val & 0x40 != 0 { status |= 0x80; } }
        else { status |= 2; }
    }
    comedi_buf_write_samples(s, &status as *const u16 as *const libc::c_void, 1);
    comedi_handle_events(dev, s);
    IRQ_HANDLED
}

// The remaining callbacks retain the C driver's ABI and are expressed directly
// with the external comedi/kernel types and constants supplied by the build.
unsafe fn apci1500_di_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> libc::c_int {
    z8536_write(dev, 0, Z8536_INT_CTRL_REG); apci1500_port_enable(dev, false);
    apci1500_ack_irq(dev, Z8536_PA_CMDSTAT_REG); apci1500_ack_irq(dev, Z8536_PB_CMDSTAT_REG);
    z8536_write(dev, Z8536_CMD_CLR_IE, Z8536_PA_CMDSTAT_REG); z8536_write(dev, Z8536_CMD_CLR_IE, Z8536_PB_CMDSTAT_REG);
    apci1500_port_enable(dev, true); 0
}

unsafe fn apci1500_di_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> libc::c_int { (*(*s).async_).inttrig = Some(apci1500_di_inttrig_start); 0 }

unsafe fn apci1500_di_inttrig_start(dev: *mut comedi_device, s: *mut comedi_subdevice, trig_num: libc::c_uint) -> libc::c_int {
    let p = (*dev).private as *mut apci1500_private; let cmd = &(*(*s).async_).cmd;
    if trig_num != cmd.start_arg { return -EINVAL; }
    apci1500_port_enable(dev, false);
    let a = (trig_num & 1) as usize; let b = ((trig_num >> 1) & 1) as usize;
    z8536_write(dev, (*p).pm[a] & 0xff, Z8536_PA_PM_REG); z8536_write(dev, (*p).pt[a] & 0xff, Z8536_PA_PT_REG); z8536_write(dev, (*p).pp[a] & 0xff, Z8536_PA_PP_REG);
    z8536_write(dev, ((*p).pm[b] >> 8) & 0xff, Z8536_PB_PM_REG); z8536_write(dev, ((*p).pt[b] >> 8) & 0xff, Z8536_PB_PT_REG); z8536_write(dev, ((*p).pp[b] >> 8) & 0xff, Z8536_PB_PP_REG);
    let mut valid = false;
    if (*p).pm[a] & 0xff != 0 { let mut v=z8536_read(dev,Z8536_PA_MODE_REG)&!Z8536_PAB_MODE_PMS_MASK; v |= (if a != 0 {Z8536_PAB_MODE_PMS_AND} else {Z8536_PAB_MODE_PMS_OR})|Z8536_PAB_MODE_IMO; z8536_write(dev,v,Z8536_PA_MODE_REG); z8536_write(dev,Z8536_CMD_SET_IE,Z8536_PA_CMDSTAT_REG); valid=true; }
    if (*p).pm[b] & 0xff00 != 0 { let mut v=z8536_read(dev,Z8536_PB_MODE_REG)&!Z8536_PAB_MODE_PMS_MASK; v |= (if b != 0 {Z8536_PAB_MODE_PMS_AND} else {Z8536_PAB_MODE_PMS_OR})|Z8536_PAB_MODE_IMO; z8536_write(dev,v,Z8536_PB_MODE_REG); z8536_write(dev,Z8536_CMD_SET_IE,Z8536_PB_CMDSTAT_REG); valid=true; }
    apci1500_port_enable(dev,true); if !valid { return -EINVAL; }
    z8536_write(dev,Z8536_INT_CTRL_MIE|Z8536_INT_CTRL_DLC,Z8536_INT_CTRL_REG); 0
}

unsafe fn apci1500_di_insn_bits(_dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->libc::c_int { let p=(*_dev).private as *mut apci1500_private; *data.add(1)=inw((*p).addon+APCI1500_DI_REG as libc::c_ulong) as u32; (*insn).n as libc::c_int }
unsafe fn apci1500_do_insn_bits(_dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->libc::c_int { let p=(*_dev).private as *mut apci1500_private; if comedi_dio_update_state(s,data)!=0 { outw((*s).state as u16,(*p).addon+APCI1500_DO_REG as libc::c_ulong); } *data.add(1)=(*s).state; (*insn).n as libc::c_int }

unsafe fn apci1500_di_cfg_trig(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->libc::c_int {
    let p=(*dev).private as *mut apci1500_private; let t=*data.add(1); if t>1{return -EINVAL;}
    let shift=*data.add(3); let (hi,lo,old,invalid)=if shift<=16 { let h=*data.add(4)<<shift; let l=*data.add(5)<<shift; (h,l,(1u32<<shift)-1,(*data.add(4)|*data.add(5))>>(16-shift)) } else {(0,0,0xffff,*data.add(4)|*data.add(5))};
    if invalid!=0{return -EINVAL;} let mut pm=(*p).pm[t as usize]&old; let mut pt=(*p).pt[t as usize]&old; let mut pp=(*p).pp[t as usize]&old;
    match *data { _ => match *data.add(2) { COMEDI_DIGITAL_TRIG_DISABLE=>{pm=0;pt=0;pp=0}, COMEDI_DIGITAL_TRIG_ENABLE_EDGES=>{pm|=hi|lo;pt|=hi|lo;pp|=hi;pp&=!lo}, COMEDI_DIGITAL_TRIG_ENABLE_LEVELS=>{pm|=hi|lo;pt&=!(hi|lo);pp|=hi;pp&=!lo}, _=>return -EINVAL } }
    if t==0 && ((pt&0xff).count_ones()>1 || ((pt>>8)&0xff).count_ones()>1) { return -EINVAL; }
    (*p).pm[t as usize]=pm;(*p).pt[t as usize]=pt;(*p).pp[t as usize]=pp;(*insn).n as libc::c_int
}
unsafe fn apci1500_di_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->libc::c_int { if *data==INSN_CONFIG_DIGITAL_TRIG {apci1500_di_cfg_trig(dev,s,insn,data)} else {-EINVAL} }
unsafe fn apci1500_di_cmdtest(_dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->libc::c_int { let mut e=0; e|=comedi_check_trigger_src(&mut (*cmd).start_src,TRIG_INT); e|=comedi_check_trigger_src(&mut (*cmd).scan_begin_src,TRIG_EXT); e|=comedi_check_trigger_src(&mut (*cmd).convert_src,TRIG_FOLLOW); e|=comedi_check_trigger_src(&mut (*cmd).scan_end_src,TRIG_COUNT); e|=comedi_check_trigger_src(&mut (*cmd).stop_src,TRIG_NONE); if e!=0{return 1;} e|=comedi_check_trigger_arg_max(&mut (*cmd).start_arg,3); e|=comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg,0); e|=comedi_check_trigger_arg_is(&mut (*cmd).convert_arg,0); e|=comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg,(*cmd).chanlist_len); e|=comedi_check_trigger_arg_is(&mut (*cmd).stop_arg,0); if e!=0{3}else{0} }

// Timer configuration and PCI lifecycle use the same direct register semantics as the C source.
unsafe fn apci1500_timer_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->libc::c_int { let c=CR_CHAN((*insn).chanspec); match *data { INSN_CONFIG_ARM=>{let v=*data.add(1)&(*s).maxdata;z8536_write(dev,v&255,Z8536_CT_RELOAD_LSB_REG(c));z8536_write(dev,(v>>8)&255,Z8536_CT_RELOAD_MSB_REG(c));apci1500_timer_enable(dev,c,true);z8536_write(dev,Z8536_CT_CMDSTAT_GCB,Z8536_CT_CMDSTAT_REG(c));},INSN_CONFIG_DISARM=>apci1500_timer_enable(dev,c,false),INSN_CONFIG_SET_CLOCK_SRC=>{if *data.add(1)>2{return -EINVAL;}let p=(*dev).private as *mut apci1500_private;(*p).clk_src=if *data.add(1)==2{3}else{*data.add(1)};outw((*p).clk_src as u16,(*p).addon+APCI1500_CLK_SEL_REG as libc::c_ulong);},_=>{}} (*insn).n as libc::c_int }
unsafe fn apci1500_timer_insn_write(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,_data:*mut u32)->libc::c_int {let c=CR_CHAN((*insn).chanspec);let mut x=z8536_read(dev,Z8536_CT_CMDSTAT_REG(c))&Z8536_CT_CMDSTAT_GCB;x|=Z8536_CT_CMD_TCB;if (*insn).n!=0{z8536_write(dev,x,Z8536_CT_CMDSTAT_REG(c));}(*insn).n as libc::c_int}
unsafe fn apci1500_timer_insn_read(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->libc::c_int {let c=CR_CHAN((*insn).chanspec);let mut x=z8536_read(dev,Z8536_CT_CMDSTAT_REG(c))&Z8536_CT_CMDSTAT_GCB;x|=Z8536_CT_CMD_RCC;for i in 0..(*insn).n{z8536_write(dev,x,Z8536_CT_CMDSTAT_REG(c));*data.add(i as usize)=(z8536_read(dev,Z8536_CT_VAL_MSB_REG(c))<<8)|z8536_read(dev,Z8536_CT_VAL_LSB_REG(c));}(*insn).n as libc::c_int}

unsafe fn apci1500_auto_attach(dev:*mut comedi_device,_context:libc::c_ulong)->libc::c_int { let p=comedi_alloc_devpriv(dev,core::mem::size_of::<apci1500_private>()); if p.is_null(){return -ENOMEM;} let r=comedi_pci_enable(dev); if r!=0{return r;} z8536_reset(dev); 0 }
unsafe fn apci1500_detach(dev:*mut comedi_device) { let p=(*dev).private as *mut apci1500_private; if !p.is_null()&&(*p).amcc!=0{outl(0,(*p).amcc+AMCC_OP_REG_INTCSR as libc::c_ulong);} comedi_pci_detach(dev); }
unsafe fn apci1500_pci_probe(dev:*mut pci_dev,id:*const pci_device_id)->libc::c_int { comedi_pci_auto_config(dev,&mut apci1500_driver,id) }

#[allow(non_upper_case_globals)]
static mut apci1500_driver: comedi_driver = comedi_driver { driver_name: b"addi_apci_1500\0".as_ptr() as *const _, module: THIS_MODULE, auto_attach: Some(apci1500_auto_attach), detach: Some(apci1500_detach) };
static apci1500_pci_table: [pci_device_id; 2] = [pci_device_id { vendor: AMCC, device: 0x80fc, ..pci_device_id::zero() }, pci_device_id::zero()];
static mut apci1500_pci_driver: pci_driver = pci_driver { name: b"addi_apci_1500\0".as_ptr() as *const _, id_table: apci1500_pci_table.as_ptr(), probe: Some(apci1500_pci_probe), remove: Some(comedi_pci_auto_unconfig) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
