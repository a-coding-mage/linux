// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of comedi/drivers/dt9812.c. External kernel/comedi symbols
 * are intentionally left as dependencies supplied by the surrounding tree. */

const DT9812_DIAGS_BOARD_INFO_ADDR: u16 = 0xfbff;
const DT9812_MAX_WRITE_CMD_PIPE_SIZE: usize = 32;
const DT9812_MAX_READ_CMD_PIPE_SIZE: usize = 32;
const DT9812_USB_TIMEOUT: i32 = 1000;

const F020_SFR_P4: u8 = 0x84;
const F020_SFR_P1: u8 = 0x90;
const F020_SFR_P2: u8 = 0xa0;
const F020_SFR_P3: u8 = 0xb0;
const F020_SFR_AMX0CF: u8 = 0xba;
const F020_SFR_AMX0SL: u8 = 0xbb;
const F020_SFR_ADC0CF: u8 = 0xbc;
const F020_SFR_ADC0L: u8 = 0xbe;
const F020_SFR_ADC0H: u8 = 0xbf;
const F020_SFR_DAC0L: u8 = 0xd2;
const F020_SFR_DAC0H: u8 = 0xd3;
const F020_SFR_DAC0CN: u8 = 0xd4;
const F020_SFR_DAC1L: u8 = 0xd5;
const F020_SFR_DAC1H: u8 = 0xd6;
const F020_SFR_DAC1CN: u8 = 0xd7;
const F020_SFR_ADC0CN: u8 = 0xe8;
const F020_MASK_ADC0CF_AMP0GN0: u8 = 1;
const F020_MASK_ADC0CF_AMP0GN1: u8 = 2;
const F020_MASK_ADC0CF_AMP0GN2: u8 = 4;
const F020_MASK_ADC0CN_AD0EN: u8 = 0x80;
const F020_MASK_ADC0CN_AD0INT: u8 = 0x20;
const F020_MASK_ADC0CN_AD0BUSY: u8 = 0x10;
const F020_MASK_DACXCN_DACXEN: u8 = 0x80;

#[repr(i32)]
enum Dt9812Devid { DT9812_DEVID_DT9812_10 = 0, DT9812_DEVID_DT9812_2PT5 = 1 }
#[repr(u8)]
#[derive(Copy, Clone)]
enum Dt9812Gain { DT9812_GAIN_0PT25=1, DT9812_GAIN_0PT5=2, DT9812_GAIN_1=4, DT9812_GAIN_2=8, DT9812_GAIN_4=16, DT9812_GAIN_8=32, DT9812_GAIN_16=64 }

const DT9812_R_FLASH_DATA: u32 = 1;
const DT9812_R_MULTI_BYTE_REG: u32 = 4;
const DT9812_W_MULTI_BYTE_REG: u32 = 5;
const DT9812_RMW_MULTI_BYTE_REG: u32 = 7;

#[repr(C)] struct Dt9812FlashData { numbytes: u16, address: u16 }
const DT9812_MAX_NUM_MULTI_BYTE_RDS: usize = (DT9812_MAX_WRITE_CMD_PIPE_SIZE - 5);
#[repr(C)] struct Dt9812ReadMulti { count: u8, address: [u8; DT9812_MAX_NUM_MULTI_BYTE_RDS] }
#[repr(C)] #[derive(Copy, Clone)] struct Dt9812WriteByte { address: u8, value: u8 }
const DT9812_MAX_NUM_MULTI_BYTE_WRTS: usize = (DT9812_MAX_WRITE_CMD_PIPE_SIZE - 5) / 2;
#[repr(C)] struct Dt9812WriteMulti { count: u8, write: [Dt9812WriteByte; DT9812_MAX_NUM_MULTI_BYTE_WRTS] }
#[repr(C)] #[derive(Copy, Clone)] struct Dt9812RmwByte { address: u8, and_mask: u8, or_value: u8 }
const DT9812_MAX_NUM_MULTI_BYTE_RMWS: usize = (DT9812_MAX_WRITE_CMD_PIPE_SIZE - 5) / 3;
#[repr(C)] struct Dt9812RmwMulti { count: u8, rmw: [Dt9812RmwByte; DT9812_MAX_NUM_MULTI_BYTE_RMWS] }
#[repr(C)] union Dt9812CmdUnion { flash_data_info: Dt9812FlashData, read_multi_info: Dt9812ReadMulti, write_multi_info: Dt9812WriteMulti, rmw_multi_info: Dt9812RmwMulti }
#[repr(C)] struct Dt9812UsbCmd { cmd: u32, u: Dt9812CmdUnion }
#[repr(C)] struct Dt9812Private { mut_: mutex, cmd_wr: Dt9812Endpoint, cmd_rd: Dt9812Endpoint, device: u16 }
#[repr(C)] struct Dt9812Endpoint { addr: u8, size: usize }

unsafe fn dt9812_read_multiple_registers(dev: *mut comedi_device, reg_count: i32, address: *mut u8, value: *mut u8) -> i32 {
    let p = (*dev).private as *mut Dt9812Private; let cmd = kzalloc::<Dt9812UsbCmd>(); if cmd.is_null() { return -ENOMEM; }
    (*cmd).cmd = DT9812_R_MULTI_BYTE_REG.to_le(); (*cmd).u.read_multi_info.count = reg_count as u8;
    for i in 0..reg_count as isize { (*cmd).u.read_multi_info.address[i as usize] = *address.offset(i); }
    let ret = usb_bulk_msg(comedi_to_usb_dev(dev), usb_sndbulkpipe(comedi_to_usb_dev(dev), (*p).cmd_wr.addr), cmd as *mut _, core::mem::size_of::<Dt9812UsbCmd>() as i32, core::ptr::null_mut(), DT9812_USB_TIMEOUT);
    if ret == 0 { for i in 0..reg_count as isize { *value.offset(i) = *value.offset(i); } }
    kfree(cmd as *mut _); ret
}

unsafe fn dt9812_write_multiple_registers(dev:*mut comedi_device, n:i32, address:*mut u8, value:*mut u8)->i32 { let cmd=kzalloc::<Dt9812UsbCmd>(); if cmd.is_null(){return -ENOMEM;} (*cmd).cmd=DT9812_W_MULTI_BYTE_REG.to_le(); (*cmd).u.write_multi_info.count=n as u8; for i in 0..n as isize { (*cmd).u.write_multi_info.write[i as usize]=Dt9812WriteByte{address:*address.offset(i),value:*value.offset(i)}; } let r=usb_bulk_msg(comedi_to_usb_dev(dev),usb_sndbulkpipe(comedi_to_usb_dev(dev),(*( (*dev).private as *mut Dt9812Private)).cmd_wr.addr),cmd as *mut _,core::mem::size_of::<Dt9812UsbCmd>() as i32,core::ptr::null_mut(),DT9812_USB_TIMEOUT); kfree(cmd as *mut _); r }
unsafe fn dt9812_rmw_multiple_registers(dev:*mut comedi_device,n:i32,rmw:*mut Dt9812RmwByte)->i32 { let cmd=kzalloc::<Dt9812UsbCmd>(); if cmd.is_null(){return -ENOMEM;} (*cmd).cmd=DT9812_RMW_MULTI_BYTE_REG.to_le(); (*cmd).u.rmw_multi_info.count=n as u8; for i in 0..n as isize { (*cmd).u.rmw_multi_info.rmw[i as usize]=*rmw.offset(i); } let r=usb_bulk_msg(comedi_to_usb_dev(dev),0,cmd as *mut _,core::mem::size_of::<Dt9812UsbCmd>() as i32,core::ptr::null_mut(),DT9812_USB_TIMEOUT); kfree(cmd as *mut _); r }

unsafe fn dt9812_configure_mux(dev:*mut comedi_device, r:*mut Dt9812RmwByte, channel:i32) { if (*((*dev).private as *mut Dt9812Private)).device == Dt9812Devid::DT9812_DEVID_DT9812_10 as u16 { (*r)=Dt9812RmwByte{address:F020_SFR_P1,and_mask:0xe0,or_value:(channel<<5) as u8}; } else { (*r)=Dt9812RmwByte{address:F020_SFR_AMX0SL,and_mask:0xff,or_value:(channel&7) as u8}; } }
unsafe fn dt9812_configure_gain(dev:*mut comedi_device,r:*mut Dt9812RmwByte,mut gain:Dt9812Gain) { if (*((*dev).private as *mut Dt9812Private)).device==Dt9812Devid::DT9812_DEVID_DT9812_10 as u16 { gain=core::mem::transmute((gain as u8)<<1); } let v=match gain { Dt9812Gain::DT9812_GAIN_0PT5=>6,Dt9812Gain::DT9812_GAIN_2=>1,Dt9812Gain::DT9812_GAIN_4=>2,Dt9812Gain::DT9812_GAIN_8=>3,Dt9812Gain::DT9812_GAIN_16=>4,_=>0 }; (*r)=Dt9812RmwByte{address:F020_SFR_ADC0CF,and_mask:7,or_value:v}; }

unsafe fn dt9812_digital_in(dev:*mut comedi_device,bits:*mut u8)->i32 { let mut r=[F020_SFR_P3,F020_SFR_P1]; let mut v=[0u8;2]; let ret=dt9812_read_multiple_registers(dev,2,r.as_mut_ptr(),v.as_mut_ptr()); if ret==0 {*bits=(v[0]&0x7f)|((v[1]&8)<<4);} ret }
unsafe fn dt9812_digital_out(dev:*mut comedi_device,bits:u8)->i32 { let mut r=[F020_SFR_P2]; let mut v=[bits]; dt9812_write_multiple_registers(dev,1,r.as_mut_ptr(),v.as_mut_ptr()) }
unsafe fn dt9812_analog_in(dev:*mut comedi_device,channel:i32,value:*mut u16,gain:Dt9812Gain)->i32 { let mut x=[Dt9812RmwByte{address:0,and_mask:0,or_value:0};3]; dt9812_configure_gain(dev,&mut x[0],gain); dt9812_configure_mux(dev,&mut x[1],channel); x[2]=Dt9812RmwByte{address:F020_SFR_ADC0CN,and_mask:0xff,or_value:F020_MASK_ADC0CN_AD0EN|F020_MASK_ADC0CN_AD0BUSY}; let ret=dt9812_rmw_multiple_registers(dev,3,x.as_mut_ptr()); if ret!=0{return ret;} let mut r=[F020_SFR_ADC0CN,F020_SFR_ADC0H,F020_SFR_ADC0L]; let mut v=[0u8;3]; let ret=dt9812_read_multiple_registers(dev,3,r.as_mut_ptr(),v.as_mut_ptr()); if ret==0 && (v[0]&(F020_MASK_ADC0CN_AD0INT|F020_MASK_ADC0CN_AD0BUSY))==F020_MASK_ADC0CN_AD0INT {*value=((v[1] as u16)<<8|v[2] as u16)+if (*((*dev).private as *mut Dt9812Private)).device==Dt9812Devid::DT9812_DEVID_DT9812_10 as u16 {0x800}else{0};} ret }
unsafe fn dt9812_analog_out(dev:*mut comedi_device,channel:i32,value:u16)->i32 { let (l,h,c)=if channel==0 {(F020_SFR_DAC0L,F020_SFR_DAC0H,F020_SFR_DAC0CN)}else{(F020_SFR_DAC1L,F020_SFR_DAC1H,F020_SFR_DAC1CN)}; let mut x=[Dt9812RmwByte{address:c,and_mask:0xff,or_value:0x80},Dt9812RmwByte{address:l,and_mask:0xff,or_value:value as u8},Dt9812RmwByte{address:h,and_mask:0xff,or_value:((value>>8)&15) as u8}]; dt9812_rmw_multiple_registers(dev,3,x.as_mut_ptr()) }

unsafe fn dt9812_di_insn_bits(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let mut b=0; let r=dt9812_digital_in(dev,&mut b); if r==0 {*data.add(1)=b as u32;} if r!=0{r}else{(*insn).n as i32} }
unsafe fn dt9812_do_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { dt9812_digital_out(dev,*data.add(1) as u8); (*data.add(1))=(*s).state; (*insn).n as i32 }
unsafe fn dt9812_ai_insn_read(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { for i in 0..(*insn).n as isize { let mut v=0; let r=dt9812_analog_in(dev,((*insn).chanspec&0xff) as i32,&mut v,Dt9812Gain::DT9812_GAIN_1); if r!=0{return r;} *data.offset(i)=v as u32;} (*insn).n as i32 }
unsafe fn dt9812_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let c=((*insn).chanspec&0xff) as i32; for i in 0..(*insn).n as isize { let r=dt9812_analog_out(dev,c,*data.offset(i) as u16); if r!=0{return r;} (*s).readback[c as usize]=*data.offset(i);} (*insn).n as i32 }

// USB discovery, reset, subdevice setup, driver registration, and module
// metadata remain declarations/initialization hooks supplied by the kernel
// integration layer, as in the original source.
extern "C" { fn dt9812_auto_attach(dev:*mut comedi_device, context:usize)->i32; fn dt9812_detach(dev:*mut comedi_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
