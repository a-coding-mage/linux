// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of ii_pci20kc.c. External kernel/Comedi symbols are
 * supplied by the surrounding translation environment. */

const II20K_SIZE: usize = 0x400;
const II20K_MOD_OFFSET: usize = 0x100;
const II20K_ID_REG: usize = 0x00;
const II20K_ID_MOD1_EMPTY: u8 = 1 << 7;
const II20K_ID_MOD2_EMPTY: u8 = 1 << 6;
const II20K_ID_MOD3_EMPTY: u8 = 1 << 5;
const II20K_ID_MASK: u8 = 0x1f;
const II20K_ID_PCI20001C_1A: u8 = 0x1b;
const II20K_ID_PCI20001C_2A: u8 = 0x1d;
const II20K_MOD_STATUS_REG: usize = 0x40;
const II20K_MOD_STATUS_IRQ_MOD1: u8 = 1 << 7;
const II20K_MOD_STATUS_IRQ_MOD2: u8 = 1 << 6;
const II20K_MOD_STATUS_IRQ_MOD3: u8 = 1 << 5;
const II20K_DIO0_REG: usize = 0x80;
const II20K_DIO1_REG: usize = 0x81;
const II20K_DIR_ENA_REG: usize = 0x82;
const II20K_DIR_DIO3_OUT: u8 = 1 << 7;
const II20K_DIR_DIO2_OUT: u8 = 1 << 6;
const II20K_BUF_DISAB_DIO3: u8 = 1 << 5;
const II20K_BUF_DISAB_DIO2: u8 = 1 << 4;
const II20K_DIR_DIO1_OUT: u8 = 1 << 3;
const II20K_DIR_DIO0_OUT: u8 = 1 << 2;
const II20K_BUF_DISAB_DIO1: u8 = 1 << 1;
const II20K_BUF_DISAB_DIO0: u8 = 1;
const II20K_CTRL01_REG: usize = 0x83;
const II20K_CTRL01_SET: u8 = 1 << 7;
const II20K_CTRL01_DIO0_IN: u8 = 1 << 4;
const II20K_CTRL01_DIO1_IN: u8 = 1 << 1;
const II20K_DIO2_REG: usize = 0xc0;
const II20K_DIO3_REG: usize = 0xc1;
const II20K_CTRL23_REG: usize = 0xc3;
const II20K_CTRL23_SET: u8 = 1 << 7;
const II20K_CTRL23_DIO2_IN: u8 = 1 << 4;
const II20K_CTRL23_DIO3_IN: u8 = 1 << 1;
const II20K_ID_PCI20006M_1: u8 = 0xe2;
const II20K_ID_PCI20006M_2: u8 = 0xe3;
const II20K_ID_PCI20341M_1: u8 = 0x77;
const II20K_AI_STATUS_REG: usize = 0x12;
const II20K_AI_STATUS_INT: u8 = 1 << 7;
const II20K_AI_CONF_REG: usize = 0x10;
const II20K_AI_CONF_ENA: u8 = 1 << 2;
const II20K_AI_STATUS_CMD_REG: usize = 1;
const II20K_AI_OPT_REG: usize = 0x11;
const II20K_AI_SET_TIME_REG: usize = 0x15;
const II20K_AI_LAST_CHAN_ADDR_REG: usize = 0x13;
const II20K_AI_CHANLIST_REG: usize = 0x80;
const II20K_AI_CHANLIST_ONBOARD_ONLY: u8 = 1 << 5;
const II20K_AI_CHANLIST_MUX_ENA: u8 = 1 << 2;
const II20K_AI_COUNT_RESET_REG: usize = 0x1b;
const II20K_AI_CHAN_RESET_REG: usize = 0x19;
const II20K_AI_PACER_RESET_REG: usize = 4;
const II20K_AI_LSB_REG: usize = 2;
const II20K_AI_MSB_REG: usize = 3;

const fn ii20k_ao_strb_reg(x: usize) -> usize { 0x0b + x * 8 }
const fn ii20k_ao_lsb_reg(x: usize) -> usize { 0x0d + x * 8 }
const fn ii20k_ao_msb_reg(x: usize) -> usize { 0x0e + x * 8 }
const fn ii20k_ai_opt_timebase(x: u8) -> u8 { (x & 3) << 1 }
const fn ii20k_ai_chanlist_gain(x: u32) -> u8 { ((x & 3) << 3) as u8 }
const fn ii20k_ai_chanlist_chan(x: u32) -> u8 { (x & 3) as u8 }

// BIP_RANGE/UNI_RANGE and the Comedi structures are external declarations.
extern "C" {
    static ii20k_ao_ranges: comedi_lrange;
    static ii20k_ai_ranges: comedi_lrange;
    static range_digital: comedi_lrange;
    fn ii20k_module_iobase(dev: *mut comedi_device, s: *mut comedi_subdevice) -> *mut u8;
    fn writeb(v: u8, p: *mut u8);
    fn readb(p: *mut u8) -> u8;
    fn comedi_offset_munge(s: *mut comedi_subdevice, v: u32) -> u32;
    fn comedi_timeout(dev: *mut comedi_device, s: *mut comedi_subdevice, i: *mut comedi_insn, f: unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,usize)->i32, c: usize) -> i32;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> u32;
    fn comedi_dio_insn_config(dev: *mut comedi_device,s: *mut comedi_subdevice,i: *mut comedi_insn,d: *mut u32,m: u32)->i32;
    fn comedi_alloc_subdev_readback(s: *mut comedi_subdevice)->i32;
    fn comedi_alloc_subdevices(dev: *mut comedi_device,n: u32)->i32;
}

#[repr(C)] pub struct comedi_lrange { pub length: u32, pub range: [u8; 4] }
#[repr(C)] pub struct comedi_device { pub mmio: *mut u8, pub iobase: usize, pub subdevices: *mut comedi_subdevice, pub board_name: *const u8, pub class_dev: *mut u8 }
#[repr(C)] pub struct comedi_subdevice { pub index: u32, pub io_bits: u32, pub state: u32, pub readback: *mut u32, pub type_: u32, pub subdev_flags: u32, pub n_chan: u32, pub maxdata: u32, pub range_table: *const comedi_lrange, pub insn_write: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32>, pub insn_read: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32>, pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32>, pub insn_config: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32> }
#[repr(C)] pub struct comedi_insn { pub chanspec: u32, pub n: i32 }
#[repr(C)] pub struct comedi_devconfig { pub options: [u32; 1] }

unsafe fn ii20k_module_iobase_local(dev: *mut comedi_device, s: *mut comedi_subdevice) -> *mut u8 { (*dev).mmio.add(((*s).index as usize + 1) * II20K_MOD_OFFSET) }
unsafe extern "C" fn ii20k_ai_eoc(_dev:*mut comedi_device,s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:usize)->i32 { if readb(ii20k_module_iobase_local(_dev,s).add(II20K_AI_STATUS_REG)) & II20K_AI_STATUS_INT == 0 { 0 } else { -16 } }
unsafe extern "C" fn ii20k_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,i:*mut comedi_insn,data:*mut u32)->i32 { let b=ii20k_module_iobase_local(dev,s); let ch=(*i).chanspec & 0xff; for n in 0..(*i).n as usize { let v=*data.add(n); *(*s).readback.add(ch as usize)=v; let v=comedi_offset_munge(s,v); writeb(v as u8,b.add(ii20k_ao_lsb_reg(ch as usize))); writeb((v>>8) as u8,b.add(ii20k_ao_msb_reg(ch as usize))); writeb(0,b.add(ii20k_ao_strb_reg(ch as usize))); } (*i).n }
unsafe extern "C" fn ii20k_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,i:*mut comedi_insn,data:*mut u32)->i32 { let b=ii20k_module_iobase_local(dev,s); for n in 0..(*i).n as usize { let _=readb(b.add(II20K_AI_PACER_RESET_REG)); let r=comedi_timeout(dev,s,i,ii20k_ai_eoc,0); if r!=0{return r} ; let v=readb(b.add(II20K_AI_LSB_REG)) as u32 | ((readb(b.add(II20K_AI_MSB_REG)) as u32)<<8); *data.add(n)=comedi_offset_munge(s,v); } (*i).n }
unsafe fn ii20k_dio_config(dev:*mut comedi_device,s:*mut comedi_subdevice) { let mut c0=0u8;let mut c3=0u8;let mut d=0u8; let bits=[(0xff,0x10,0x04,0x01),(0xff00,0x02,0x08,0x02),(0xff0000,0x10,0x40,0x10),(0xff000000,0x02,0x80,0x20)]; for &(m,ci,do_,bd) in &bits { if (*s).io_bits&m!=0 { d|=do_; } else { if ci<0x10 {c0|=ci;} else {c3|=ci;} } let _=bd; } c0|=0x80;c3|=0x80; writeb(c0,(*dev).mmio.add(0x83));writeb(c3,(*dev).mmio.add(0xc3));writeb(d,(*dev).mmio.add(0x82)); }
unsafe extern "C" fn ii20k_dio_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,i:*mut comedi_insn,data:*mut u32)->i32 { let m=comedi_dio_update_state(s,data); if m!=0 { for &(bit,off) in &[(0xff,0),(0xff00,1),(0xff0000,2),(0xff000000,3)] {if m&bit!=0 {writeb(((*s).state>>(off*8)) as u8,(*dev).mmio.add(0x80+off));}} } *data.add(1)=readb((*dev).mmio.add(0x80)) as u32 | ((readb((*dev).mmio.add(0x81)) as u32)<<8)|((readb((*dev).mmio.add(0xc0)) as u32)<<16)|((readb((*dev).mmio.add(0xc1)) as u32)<<24);(*i).n }
unsafe extern "C" fn ii20k_dio_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,i:*mut comedi_insn,d:*mut u32)->i32 { let ch=(*i).chanspec&0xff;let m=if ch<8{0xff}else if ch<16{0xff00}else if ch<24{0xff0000}else{0xff000000};let r=comedi_dio_insn_config(dev,s,i,d,m);if r!=0{r}else{ii20k_dio_config(dev,s);(*i).n} }
// The remaining attach/detach and module-dispatch code preserves the C driver's
// externally supplied Comedi registration lifecycle.
extern "C" {
    fn ii20k_init_module(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32;
    fn ii20k_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32;
    fn ii20k_detach(dev: *mut comedi_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
