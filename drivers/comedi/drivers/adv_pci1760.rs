// SPDX-License-Identifier: GPL-2.0+
/* COMEDI driver for the Advantech PCI-1760. */

// Dependencies supplied by the surrounding kernel/Comedi bindings are intentionally external.

const PCI1760_CMD_TIMEOUT: u64 = 250;
const PCI1760_CMD_RETRIES: i32 = 3;
const PCI1760_PWM_TIMEBASE: u32 = 100000;

const PCI1760_INTCSR1_IRQ_ENA: u8 = 1 << 5;
const PCI1760_INTCSR2_OMB_IRQ: u8 = 1 << 0;
const PCI1760_INTCSR2_IMB_IRQ: u8 = 1 << 1;
const PCI1760_INTCSR2_IRQ_STATUS: u8 = 1 << 6;
const PCI1760_INTCSR2_IRQ_ASSERTED: u8 = 1 << 7;

const PCI1760_CMD_CLR_IMB2: u8 = 0x00;
const PCI1760_CMD_SET_DO: u8 = 0x01;
const PCI1760_CMD_GET_DO: u8 = 0x02;
const PCI1760_CMD_GET_STATUS: u8 = 0x07;
const PCI1760_CMD_GET_FW_VER: u8 = 0x0e;
const PCI1760_CMD_GET_HW_VER: u8 = 0x0f;
const PCI1760_CMD_ENA_PWM: u8 = 0x1f;
const PCI1760_CMD_ENA_FILT: u8 = 0x20;
const PCI1760_CMD_ENA_PAT_MATCH: u8 = 0x21;
const PCI1760_CMD_SET_PAT_MATCH: u8 = 0x22;
const PCI1760_CMD_ENA_RISE_EDGE: u8 = 0x23;
const PCI1760_CMD_ENA_FALL_EDGE: u8 = 0x24;
const PCI1760_CMD_ENA_CNT: u8 = 0x28;
const PCI1760_CMD_RST_CNT: u8 = 0x29;
const PCI1760_CMD_ENA_CNT_OFLOW: u8 = 0x2a;
const PCI1760_CMD_ENA_CNT_MATCH: u8 = 0x2b;
const PCI1760_CMD_SET_CNT_EDGE: u8 = 0x2c;
const PCI1760_CMD_GET_CNT: u8 = 0x2f;
const PCI1760_CMD_GET_INT_FLAGS: u8 = 0x60;
const PCI1760_CMD_GET_INT_FLAGS_MATCH: u8 = 1 << 0;
const PCI1760_CMD_GET_INT_FLAGS_COS: u8 = 1 << 1;
const PCI1760_CMD_GET_INT_FLAGS_OFLOW: u8 = 1 << 2;
const PCI1760_CMD_GET_OS: u8 = 0x61;
const PCI1760_CMD_GET_CNT_STATUS: u8 = 0x62;

#[inline] const fn pci1760_omb_reg(x: u8) -> u32 { 0x0c + x as u32 }
#[inline] const fn pci1760_imb_reg(x: u8) -> u32 { 0x1c + x as u32 }
#[inline] const fn pci1760_intcsr_reg(x: u8) -> u32 { 0x38 + x as u32 }
#[inline] const fn pci1760_cmd_set_pwm_hi(x: u8) -> u8 { 0x10 + x * 2 }
#[inline] const fn pci1760_cmd_set_pwm_lo(x: u8) -> u8 { 0x11 + x * 2 }
#[inline] const fn pci1760_cmd_set_pwm_cnt(x: u8) -> u8 { 0x14 + x }
#[inline] const fn pci1760_cmd_set_hi_samp(x: u8) -> u8 { 0x30 + x }
#[inline] const fn pci1760_cmd_set_lo_samp(x: u8) -> u8 { 0x38 + x }
#[inline] const fn pci1760_cmd_set_cnt(x: u8) -> u8 { 0x40 + x }
#[inline] const fn pci1760_cmd_set_cnt_match(x: u8) -> u8 { 0x48 + x }

unsafe fn pci1760_send_cmd(dev: *mut comedi_device, cmd: u8, val: u16) -> i32 {
    let base = (*dev).iobase;
    outb((val & 0xff) as u8, base + pci1760_omb_reg(0));
    outb((val >> 8) as u8, base + pci1760_omb_reg(1));
    outb(cmd, base + pci1760_omb_reg(2));
    outb(0, base + pci1760_omb_reg(3));
    let timeout = jiffies() + usecs_to_jiffies(PCI1760_CMD_TIMEOUT);
    loop {
        if inb(base + pci1760_imb_reg(2)) == cmd {
            return (inb(base + pci1760_imb_reg(0)) as i32) |
                ((inb(base + pci1760_imb_reg(1)) as i32) << 8);
        }
        cpu_relax();
        if !time_before(jiffies(), timeout) { break; }
    }
    -EBUSY
}

unsafe fn pci1760_cmd(dev: *mut comedi_device, cmd: u8, val: u16) -> i32 {
    let base = (*dev).iobase;
    if inb(base + pci1760_imb_reg(2)) == cmd {
        let mut ret = pci1760_send_cmd(dev, PCI1760_CMD_CLR_IMB2, 0);
        if ret < 0 {
            ret = pci1760_send_cmd(dev, PCI1760_CMD_CLR_IMB2, 0);
            if ret < 0 { return -ETIMEDOUT; }
        }
    }
    for _ in 0..PCI1760_CMD_RETRIES {
        let ret = pci1760_send_cmd(dev, cmd, val);
        if ret >= 0 { return ret; }
    }
    -ETIMEDOUT
}

unsafe fn pci1760_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    (*data.add(1)) = inb((*dev).iobase + pci1760_imb_reg(3)) as u32;
    (*insn).n as i32
}

unsafe fn pci1760_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    if comedi_dio_update_state(s, data) {
        let ret = pci1760_cmd(dev, PCI1760_CMD_SET_DO, (*s).state as u16);
        if ret < 0 { return ret; }
    }
    *data.add(1) = (*s).state as u32;
    (*insn).n as i32
}

unsafe fn pci1760_pwm_ns_to_div(flags: u32, ns: u32) -> i32 {
    let mut divisor = match flags {
        CMDF_ROUND_NEAREST => (ns + PCI1760_PWM_TIMEBASE / 2) / PCI1760_PWM_TIMEBASE,
        CMDF_ROUND_UP => (ns + PCI1760_PWM_TIMEBASE - 1) / PCI1760_PWM_TIMEBASE,
        CMDF_ROUND_DOWN => ns / PCI1760_PWM_TIMEBASE,
        _ => return -EINVAL,
    };
    if divisor < 1 { divisor = 1; }
    if divisor > 0xffff { divisor = 0xffff; }
    divisor as i32
}

unsafe fn pci1760_pwm_enable(dev: *mut comedi_device, chan: u32, enable: bool) -> i32 {
    let mut ret = pci1760_cmd(dev, PCI1760_CMD_GET_STATUS, PCI1760_CMD_ENA_PWM as u16);
    if ret < 0 { return ret; }
    if enable { ret |= 1 << chan; } else { ret &= !(1 << chan); }
    pci1760_cmd(dev, PCI1760_CMD_ENA_PWM, ret as u16)
}

unsafe fn pci1760_pwm_insn_config(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mut ret;
    match *data {
        INSN_CONFIG_ARM => { ret = pci1760_pwm_enable(dev, chan, false); if ret < 0 { return ret; } if *data.add(1) > 0xffff { return -EINVAL; } ret = pci1760_cmd(dev, pci1760_cmd_set_pwm_cnt(chan as u8), *data.add(1) as u16); if ret < 0 { return ret; } ret = pci1760_pwm_enable(dev, chan, true); if ret < 0 { return ret; } }
        INSN_CONFIG_DISARM => { ret = pci1760_pwm_enable(dev, chan, false); if ret < 0 { return ret; } }
        INSN_CONFIG_PWM_OUTPUT => {
            ret = pci1760_pwm_enable(dev, chan, false); if ret < 0 { return ret; }
            let hi = pci1760_pwm_ns_to_div(*data.add(1), *data.add(2)); let lo = pci1760_pwm_ns_to_div(*data.add(3), *data.add(4));
            if hi < 0 || lo < 0 { return -EINVAL; }
            if hi as u32 * PCI1760_PWM_TIMEBASE != *data.add(2) || lo as u32 * PCI1760_PWM_TIMEBASE != *data.add(4) { *data.add(2)=hi as u32*PCI1760_PWM_TIMEBASE; *data.add(4)=lo as u32*PCI1760_PWM_TIMEBASE; return -EAGAIN; }
            ret=pci1760_cmd(dev,pci1760_cmd_set_pwm_hi(chan as u8),hi as u16); if ret<0{return ret;} ret=pci1760_cmd(dev,pci1760_cmd_set_pwm_lo(chan as u8),lo as u16); if ret<0{return ret;}
        }
        INSN_CONFIG_GET_PWM_OUTPUT => { let hi=pci1760_cmd(dev,PCI1760_CMD_GET_STATUS,pci1760_cmd_set_pwm_hi(chan as u8) as u16); let lo=pci1760_cmd(dev,PCI1760_CMD_GET_STATUS,pci1760_cmd_set_pwm_lo(chan as u8) as u16); if hi<0||lo<0{return -ETIMEDOUT;} *data.add(1)=hi as u32*PCI1760_PWM_TIMEBASE; *data.add(2)=lo as u32*PCI1760_PWM_TIMEBASE; }
        INSN_CONFIG_GET_PWM_STATUS => { ret=pci1760_cmd(dev,PCI1760_CMD_GET_STATUS,PCI1760_CMD_ENA_PWM as u16); if ret<0{return ret;} *data.add(1)=if ret & (1<<chan)!=0 {1}else{0}; }
        _ => return -EINVAL,
    }
    (*insn).n as i32
}

unsafe fn pci1760_reset(dev: *mut comedi_device) {
    let base=(*dev).iobase; outb(0,base+pci1760_intcsr_reg(0)); outb(0,base+pci1760_intcsr_reg(1)); outb(0,base+pci1760_intcsr_reg(3));
    pci1760_cmd(dev,PCI1760_CMD_ENA_CNT,0); pci1760_cmd(dev,PCI1760_CMD_ENA_CNT_OFLOW,0); pci1760_cmd(dev,PCI1760_CMD_ENA_CNT_MATCH,0);
    for i in 0..8 { pci1760_cmd(dev,pci1760_cmd_set_cnt_match(i),0x8000); pci1760_cmd(dev,pci1760_cmd_set_cnt(i),0); }
    pci1760_cmd(dev,PCI1760_CMD_RST_CNT,0xff); pci1760_cmd(dev,PCI1760_CMD_SET_CNT_EDGE,0); pci1760_cmd(dev,PCI1760_CMD_ENA_FILT,0); pci1760_cmd(dev,PCI1760_CMD_ENA_PAT_MATCH,0); pci1760_cmd(dev,PCI1760_CMD_SET_PAT_MATCH,0);
}

// The remaining driver registration and subdevice setup use the native Comedi structures.
unsafe fn pci1760_auto_attach(dev: *mut comedi_device, _context: u64) -> i32 {
    let pcidev = comedi_to_pci_dev(dev); let mut ret=comedi_pci_enable(dev); if ret!=0{return ret;} (*dev).iobase=pci_resource_start(pcidev,0); pci1760_reset(dev); ret=comedi_alloc_subdevices(dev,4); if ret!=0{return ret;}
    let s=&mut (*dev).subdevices[0]; s.type_=COMEDI_SUBD_DI; s.subdev_flags=SDF_READABLE; s.n_chan=8; s.maxdata=1; s.range_table=&range_digital; s.insn_bits=Some(pci1760_di_insn_bits);
    let s=&mut (*dev).subdevices[1]; s.type_=COMEDI_SUBD_DO; s.subdev_flags=SDF_WRITABLE; s.n_chan=8; s.maxdata=1; s.range_table=&range_digital; s.insn_bits=Some(pci1760_do_insn_bits); ret=pci1760_cmd(dev,PCI1760_CMD_GET_DO,0); if ret<0{return ret;} s.state=ret;
    let s=&mut (*dev).subdevices[2]; s.type_=COMEDI_SUBD_PWM; s.subdev_flags=SDF_PWM_COUNTER; s.n_chan=2; s.insn_config=Some(pci1760_pwm_insn_config);
    (*dev).subdevices[3].type_=COMEDI_SUBD_UNUSED; 0
}

// C driver/module registration declarations are represented by the surrounding bindings.
extern "C" {
    static mut pci1760_driver: comedi_driver;
    static pci1760_pci_table: [pci_device_id; 2];
    static mut pci1760_pci_driver: pci_driver;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
