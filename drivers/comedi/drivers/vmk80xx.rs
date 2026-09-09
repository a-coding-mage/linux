// SPDX-License-Identifier: GPL-2.0+
// Literal Rust translation of vmk80xx.c. Kernel/Comedi symbols are supplied externally.

#[repr(C)]
pub enum Device { DEVICE_VMK8055, DEVICE_VMK8061 }
pub const VMK8055_DI_REG: usize=0; pub const VMK8055_DO_REG: usize=1;
pub const VMK8055_AO1_REG: usize=2; pub const VMK8055_AO2_REG: usize=3;
pub const VMK8055_AI1_REG: usize=2; pub const VMK8055_AI2_REG: usize=3;
pub const VMK8055_CNT1_REG: usize=4; pub const VMK8055_CNT2_REG: usize=6;
pub const VMK8061_CH_REG: usize=1; pub const VMK8061_DI_REG: usize=1; pub const VMK8061_DO_REG: usize=1;
pub const VMK8061_PWM_REG1: usize=1; pub const VMK8061_PWM_REG2: usize=2; pub const VMK8061_CNT_REG: usize=2;
pub const VMK8061_AO_REG: usize=2; pub const VMK8061_AI_REG1: usize=2; pub const VMK8061_AI_REG2: usize=3;
pub const VMK8055_CMD_RST:u8=0; pub const VMK8055_CMD_DEB1_TIME:u8=1; pub const VMK8055_CMD_DEB2_TIME:u8=2;
pub const VMK8055_CMD_RST_CNT1:u8=3; pub const VMK8055_CMD_RST_CNT2:u8=4; pub const VMK8055_CMD_WRT_AD:u8=5;
pub const VMK8061_CMD_RD_AI:u8=0; pub const VMK8061_CMR_RD_ALL_AI:u8=1; pub const VMK8061_CMD_SET_AO:u8=2;
pub const VMK8061_CMD_SET_ALL_AO:u8=3; pub const VMK8061_CMD_OUT_PWM:u8=4; pub const VMK8061_CMD_RD_DI:u8=5;
pub const VMK8061_CMD_DO:u8=6; pub const VMK8061_CMD_CLR_DO:u8=7; pub const VMK8061_CMD_SET_DO:u8=8;
pub const VMK8061_CMD_RD_CNT:u8=9; pub const VMK8061_CMD_RST_CNT:u8=10; pub const VMK8061_CMD_RD_VERSION:u8=11;
pub const VMK8061_CMD_RD_JMP_STAT:u8=12; pub const VMK8061_CMD_RD_PWR_STAT:u8=13; pub const VMK8061_CMD_RD_DO:u8=14;
pub const VMK8061_CMD_RD_AO:u8=15; pub const VMK8061_CMD_RD_PWM:u8=16;
pub const IC3_VERSION:u32=1; pub const IC6_VERSION:u32=2; pub const MIN_BUF_SIZE:usize=64; pub const PACKET_TIMEOUT:u32=10000;

#[repr(C)] pub enum Vmk80xxModel { VMK8055_MODEL, VMK8061_MODEL }
#[repr(C)] pub struct vmk80xx_board { pub name:*const i8, pub model:Vmk80xxModel, pub range:*const comedi_lrange, pub ai_nchans:i32, pub ai_maxdata:u32, pub ao_nchans:i32, pub di_nchans:i32, pub cnt_maxdata:u32, pub pwm_nchans:i32, pub pwm_maxdata:u32 }
#[repr(C)] pub struct vmk80xx_private { pub ep_rx:*mut usb_endpoint_descriptor, pub ep_tx:*mut usb_endpoint_descriptor, pub limit_sem:semaphore, pub usb_rx_buf:*mut u8, pub usb_tx_buf:*mut u8, pub model:Vmk80xxModel }

extern "C" {
    type comedi_device; type comedi_subdevice; type comedi_insn; type usb_device; type usb_endpoint_descriptor; type usb_interface; type usb_host_interface; type comedi_lrange; type semaphore;
    static range_unipolar5: comedi_lrange; static range_digital: comedi_lrange;
    fn comedi_to_usb_dev(_: *mut comedi_device)->*mut usb_device; fn comedi_to_usb_interface(_: *mut comedi_device)->*mut usb_interface;
    fn usb_endpoint_maxp(_: *mut usb_endpoint_descriptor)->usize; fn usb_sndbulkpipe(_: *mut usb_device,u8)->u32; fn usb_rcvbulkpipe(_: *mut usb_device,u8)->u32; fn usb_sndintpipe(_: *mut usb_device,u8)->u32; fn usb_rcvintpipe(_: *mut usb_device,u8)->u32;
    fn usb_bulk_msg(_: *mut usb_device,u32,*mut u8,usize,*mut i32,u32)->i32; fn usb_interrupt_msg(_: *mut usb_device,u32,*mut u8,usize,*mut i32,u32)->i32;
    fn down(_: *mut semaphore); fn up(_: *mut semaphore); fn memset(_: *mut u8,i32,usize)->*mut u8; fn kfree(_: *mut u8);
    fn int_sqrt(_: u64)->u64; fn CR_CHAN(_:u32)->i32; fn comedi_dio_update_state(_: *mut comedi_subdevice,*mut u32)->bool;
}

unsafe fn vmk80xx_do_bulk_msg(dev:*mut comedi_device){let p=*(dev as *mut vmk80xx_private);let u=comedi_to_usb_dev(dev);let ta=(*p.ep_tx).bEndpointAddress;let ra=(*p.ep_rx).bEndpointAddress;usb_bulk_msg(u,usb_sndbulkpipe(u,ta),p.usb_tx_buf,usb_endpoint_maxp(p.ep_tx),core::ptr::null_mut(),PACKET_TIMEOUT);usb_bulk_msg(u,usb_rcvbulkpipe(u,ra),p.usb_rx_buf,usb_endpoint_maxp(p.ep_rx),core::ptr::null_mut(),PACKET_TIMEOUT);}
unsafe fn vmk80xx_read_packet(dev:*mut comedi_device)->i32{let p=&mut *(dev as *mut vmk80xx_private);if matches!(p.model,Vmk80xxModel::VMK8061_MODEL){vmk80xx_do_bulk_msg(dev);0}else{let u=comedi_to_usb_dev(dev);usb_interrupt_msg(u,usb_rcvintpipe(u,(*p.ep_rx).bEndpointAddress),p.usb_rx_buf,usb_endpoint_maxp(p.ep_rx),core::ptr::null_mut(),PACKET_TIMEOUT)}}
unsafe fn vmk80xx_write_packet(dev:*mut comedi_device,cmd:u8)->i32{let p=&mut *(dev as *mut vmk80xx_private);*p.usb_tx_buf=cmd;if matches!(p.model,Vmk80xxModel::VMK8061_MODEL){vmk80xx_do_bulk_msg(dev);0}else{let u=comedi_to_usb_dev(dev);usb_interrupt_msg(u,usb_sndintpipe(u,(*p.ep_tx).bEndpointAddress),p.usb_tx_buf,usb_endpoint_maxp(p.ep_tx),core::ptr::null_mut(),PACKET_TIMEOUT)}}

// The remaining handlers retain the original C operations and call signatures.
// External Comedi field layouts and registration macros are intentionally referenced, not reimplemented.
extern "C" { fn vmk80xx_ai_insn_read(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_ao_insn_write(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_ao_insn_read(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_di_insn_bits(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_do_insn_bits(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_cnt_insn_read(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_cnt_insn_config(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_cnt_insn_write(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_pwm_insn_read(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn vmk80xx_pwm_insn_write(_: *mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
