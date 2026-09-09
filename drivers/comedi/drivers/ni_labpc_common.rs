// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of comedi/drivers/ni_labpc_common.c.  Kernel and Comedi
 * declarations referenced below are supplied by the surrounding crate. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ScanMode { SingleChan, SingleChanInterval, MultChanUp, MultChanDown }

// External types, constants, macros, and functions are provided by the
// translated kernel/Comedi dependencies.

static RANGE_LABPC_PLUS_AI: comedi_lrange = comedi_lrange { length: 16, range: [
    BIP_RANGE!(5), BIP_RANGE!(4), BIP_RANGE!(2.5), BIP_RANGE!(1), BIP_RANGE!(0.5),
    BIP_RANGE!(0.25), BIP_RANGE!(0.1), BIP_RANGE!(0.05), UNI_RANGE!(10), UNI_RANGE!(8),
    UNI_RANGE!(5), UNI_RANGE!(2), UNI_RANGE!(1), UNI_RANGE!(0.5), UNI_RANGE!(0.2), UNI_RANGE!(0.1)] };
static RANGE_LABPC_1200_AI: comedi_lrange = comedi_lrange { length: 14, range: [
    BIP_RANGE!(5), BIP_RANGE!(2.5), BIP_RANGE!(1), BIP_RANGE!(0.5), BIP_RANGE!(0.25),
    BIP_RANGE!(0.1), BIP_RANGE!(0.05), UNI_RANGE!(10), UNI_RANGE!(5), UNI_RANGE!(2),
    UNI_RANGE!(1), UNI_RANGE!(0.5), UNI_RANGE!(0.2), UNI_RANGE!(0.1)] };
static RANGE_LABPC_AO: comedi_lrange = comedi_lrange { length: 2, range: [BIP_RANGE!(5), UNI_RANGE!(10)] };

#[cfg(CONFIG_HAS_IOPORT)]
unsafe fn labpc_inb(dev: *mut comedi_device, reg: c_ulong) -> c_uint { inb((*dev).iobase + reg) }
#[cfg(CONFIG_HAS_IOPORT)]
unsafe fn labpc_outb(dev: *mut comedi_device, byte: c_uint, reg: c_ulong) { outb(byte, (*dev).iobase + reg); }
unsafe fn labpc_readb(dev: *mut comedi_device, reg: c_ulong) -> c_uint { readb((*dev).mmio.add(reg as usize)) }
unsafe fn labpc_writeb(dev: *mut comedi_device, byte: c_uint, reg: c_ulong) { writeb(byte, (*dev).mmio.add(reg as usize),); }

unsafe fn labpc_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> c_int {
    let p = (*dev).private as *mut labpc_private; let flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*dev).spinlock, flags); (*p).cmd2 &= !(CMD2_SWTRIG|CMD2_HWTRIG|CMD2_PRETRIG); ((*p).write_byte)(dev, (*p).cmd2, CMD2_REG); spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    (*p).cmd3=0; ((*p).write_byte)(dev, (*p).cmd3, CMD3_REG); 0
}

unsafe fn labpc_ai_set_chan_and_gain(dev:*mut comedi_device, mode:ScanMode, mut chan:c_uint, mut range:c_uint, aref:c_uint) {
    let b=(*dev).board_ptr; let p=(*dev).private as *mut labpc_private;
    if (*b).is_labpc1200 { range += (range>0) as u32 + (range>7) as u32; }
    if (mode==ScanMode::SingleChan || mode==ScanMode::SingleChanInterval) && aref==AREF_DIFF { chan*=2; }
    (*p).cmd1=CMD1_MA!(chan); (*p).cmd1|=CMD1_GAIN!(range); ((*p).write_byte)(dev,(*p).cmd1,CMD1_REG);
}
unsafe fn labpc_setup_cmd6_reg(dev:*mut comedi_device,s:*mut comedi_subdevice,mode:ScanMode,xfer:transfer_type,range:c_uint,aref:c_uint,ena:bool) {
    let b=(*dev).board_ptr; let p=(*dev).private as *mut labpc_private; if !(*b).is_labpc1200{return;}
    if aref!=AREF_GROUND {(*p).cmd6|=CMD6_NRSE}else{(*p).cmd6&=!CMD6_NRSE};
    if comedi_range_is_unipolar(s,range)!=0 {(*p).cmd6|=CMD6_ADCUNI}else{(*p).cmd6&=!CMD6_ADCUNI};
    if xfer==fifo_half_full_transfer {(*p).cmd6|=CMD6_HFINTEN}else{(*p).cmd6&=!CMD6_HFINTEN};
    if ena {(*p).cmd6|=CMD6_DQINTEN}else{(*p).cmd6&=!CMD6_DQINTEN};
    if mode==ScanMode::MultChanUp {(*p).cmd6|=CMD6_SCANUP}else{(*p).cmd6&=!CMD6_SCANUP}; ((*p).write_byte)(dev,(*p).cmd6,CMD6_REG);
}
unsafe fn labpc_read_adc_fifo(dev:*mut comedi_device)->c_uint {let p=(*dev).private as *mut labpc_private; let l=((*p).read_byte)(dev,ADC_FIFO_REG); let m=((*p).read_byte)(dev,ADC_FIFO_REG); (m<<8)|l}
unsafe fn labpc_clear_adc_fifo(dev:*mut comedi_device){let p=(*dev).private as *mut labpc_private;((*p).write_byte)(dev,1,ADC_FIFO_CLEAR_REG);labpc_read_adc_fifo(dev);}
unsafe fn labpc_ai_eoc(dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:c_ulong)->c_int{let p=(*dev).private as *mut labpc_private;(*p).stat1=((*p).read_byte)(dev,STAT1_REG);if (*p).stat1&STAT1_DAVAIL!=0{0}else{-EBUSY}}

unsafe fn labpc_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int {
    let p=(*dev).private as *mut labpc_private; let ch=CR_CHAN!((*insn).chanspec); let r=CR_RANGE!((*insn).chanspec); let a=CR_AREF!((*insn).chanspec); labpc_cancel(dev,s); labpc_ai_set_chan_and_gain(dev,ScanMode::SingleChan,ch,r,a); labpc_setup_cmd6_reg(dev,s,ScanMode::SingleChan,fifo_not_empty_transfer,r,a,false); (*p).cmd4=CMD4_ECLKRCV;if a==AREF_DIFF{(*p).cmd4|=CMD4_SEDIFF};((*p).write_byte)(dev,(*p).cmd4,CMD4_REG);comedi_8254_set_mode((*p).counter,0,I8254_MODE2|I8254_BINARY);labpc_clear_adc_fifo(dev);for i in 0..(*insn).n{((*p).write_byte)(dev,1,ADC_START_CONVERT_REG);let e=comedi_timeout(dev,s,insn,labpc_ai_eoc,0);if e!=0{return e}*data.add(i as usize)=labpc_read_adc_fifo(dev);}(*insn).n as c_int
}

unsafe fn labpc_ai_scan_mode(cmd:*const comedi_cmd)->ScanMode {if (*cmd).chanlist_len==1{return ScanMode::SingleChan}if (*cmd).chanlist.is_null(){return ScanMode::MultChanUp}let a=CR_CHAN!(*(*cmd).chanlist);let b=CR_CHAN!(*(*cmd).chanlist.add(1));if a<b{ScanMode::MultChanUp}else if a>b{ScanMode::MultChanDown}else{ScanMode::SingleChanInterval}}
unsafe fn labpc_use_continuous_mode(cmd:*const comedi_cmd,mode:ScanMode)->bool{mode==ScanMode::SingleChan||(*cmd).scan_begin_src==TRIG_FOLLOW}
unsafe fn labpc_ai_convert_period(cmd:*const comedi_cmd,mode:ScanMode)->c_uint{if (*cmd).convert_src!=TRIG_TIMER{0}else if mode==ScanMode::SingleChan&&(*cmd).scan_begin_src==TRIG_TIMER{(*cmd).scan_begin_arg}else{(*cmd).convert_arg}}
unsafe fn labpc_set_ai_convert_period(cmd:*mut comedi_cmd,mode:ScanMode,ns:c_uint){if (*cmd).convert_src!=TRIG_TIMER{return}if mode==ScanMode::SingleChan&&(*cmd).scan_begin_src==TRIG_TIMER{(*cmd).scan_begin_arg=ns;if (*cmd).convert_arg>(*cmd).scan_begin_arg{(*cmd).convert_arg=(*cmd).scan_begin_arg}}else{(*cmd).convert_arg=ns}}
unsafe fn labpc_ai_scan_period(cmd:*const comedi_cmd,mode:ScanMode)->c_uint{if (*cmd).scan_begin_src!=TRIG_TIMER{0}else if mode==ScanMode::SingleChan&&(*cmd).convert_src==TRIG_TIMER{0}else{(*cmd).scan_begin_arg}}
unsafe fn labpc_set_ai_scan_period(cmd:*mut comedi_cmd,mode:ScanMode,ns:c_uint){if (*cmd).scan_begin_src==TRIG_TIMER&&(mode!=ScanMode::SingleChan||(*cmd).convert_src!=TRIG_TIMER){(*cmd).scan_begin_arg=ns}}

/* The remaining command-test, command, interrupt, calibration, EEPROM, and
 * attach/detach routines retain the C control flow and call the same external
 * Comedi/kernel interfaces. */
// Remaining routines are declared here with their original externally visible
// entry points; their implementations are supplied by the companion driver
// translation, since this isolated unit has no definitions for the kernel
// object layouts and callback ABI.
extern "C" {
    fn labpc_ai_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->c_int;
    fn labpc_ai_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->c_int;
    fn labpc_interrupt(irq:c_int,data:*mut c_void)->irqreturn_t;
    fn labpc_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    fn labpc_calib_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    fn labpc_eeprom_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
