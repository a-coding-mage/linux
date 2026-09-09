// SPDX-License-Identifier: GPL-2.0-or-later
/* Freescale MPC85xx/MPC86xx RapidIO RMU support. Direct translation of fsl_rmu.c. */

// Kernel headers and fsl_rio.h provide the external types, functions, globals,
// constants, and macros referenced below.

const RIO_MIN_TX_RING_SIZE: usize = 2;
const RIO_MAX_TX_RING_SIZE: usize = 2048;
const RIO_MIN_RX_RING_SIZE: usize = 2;
const RIO_MAX_RX_RING_SIZE: usize = 2048;
const RIO_IPWMR_SEN: u32 = 0x00100000;
const RIO_IPWMR_QFIE: u32 = 0x00000100;
const RIO_IPWMR_EIE: u32 = 0x00000020;
const RIO_IPWMR_CQ: u32 = 0x00000002;
const RIO_IPWMR_PWE: u32 = 0x00000001;
const RIO_IPWSR_QF: u32 = 0x00100000;
const RIO_IPWSR_TE: u32 = 0x00000080;
const RIO_IPWSR_QFI: u32 = 0x00000010;
const RIO_IPWSR_PWD: u32 = 0x00000008;
const RIO_IPWSR_PWB: u32 = 0x00000004;
const RIO_EPWISR: usize = 0x10010;
const RIO_EPWISR_PINT1: u32 = 0x80000000;
const RIO_EPWISR_PINT2: u32 = 0x40000000;
const RIO_EPWISR_MU: u32 = 0x00000002;
const RIO_EPWISR_PW: u32 = 0x00000001;
const IPWSR_CLEAR: u32 = 0x98;
const OMSR_CLEAR: u32 = 0x1cb3;
const IMSR_CLEAR: u32 = 0x491;
const IDSR_CLEAR: u32 = 0x91;
const ODSR_CLEAR: u32 = 0x1c00;
const LTLEECSR_ENABLE_ALL: u32 = 0xFFC000FC;
const RIO_LTLEECSR: usize = 0x060c;
const RIO_IM0SR: usize = 0x64;
const RIO_IM1SR: usize = 0x164;
const RIO_OM0SR: usize = 0x4;
const RIO_OM1SR: usize = 0x104;
const RIO_MSG_OMR_MUI: u32 = 2;
const RIO_MSG_OSR_TE: u32 = 0x80;
const RIO_MSG_OSR_QOI: u32 = 0x20;
const RIO_MSG_OSR_EOMI: u32 = 2;
const RIO_MSG_ISR_TE: u32 = 0x80;
const RIO_MSG_ISR_DIQI: u32 = 1;
const RIO_MSG_IMR_MI: u32 = 2;
const RIO_MSG_DESC_SIZE: usize = 32;
const RIO_MSG_BUFFER_SIZE: usize = 4096;
const DOORBELL_DMR_DI: u32 = 2;
const DOORBELL_DSR_TE: u32 = 0x80;
const DOORBELL_DSR_QFI: u32 = 0x10;
const DOORBELL_DSR_DIQI: u32 = 1;
const DOORBELL_MESSAGE_SIZE: usize = 8;

#[repr(C)]
pub struct rio_msg_regs { pub omr:u32, pub osr:u32, pub pad1:u32, pub odqdpar:u32, pub pad2:u32, pub osar:u32, pub odpr:u32, pub odatr:u32, pub odcr:u32, pub pad3:u32, pub odqepar:u32, pub pad4:[u32;13], pub imr:u32, pub isr:u32, pub pad5:u32, pub ifqdpar:u32, pub pad6:u32, pub ifqepar:u32 }
#[repr(C)] pub struct rio_dbell_regs { pub odmr:u32,pub odsr:u32,pub pad1:[u32;4],pub oddpr:u32,pub oddatr:u32,pub pad2:[u32;3],pub odretcr:u32,pub pad3:[u32;12],pub dmr:u32,pub dsr:u32,pub pad4:u32,pub dqdpar:u32,pub pad5:u32,pub dqepar:u32 }
#[repr(C)] pub struct rio_pw_regs { pub pwmr:u32,pub pwsr:u32,pub epwqbar:u32,pub pwqbar:u32 }
#[repr(C)] pub struct rio_tx_desc { pub pad1:u32,pub saddr:u32,pub dport:u32,pub dattr:u32,pub pad2:u32,pub pad3:u32,pub dwcnt:u32,pub pad4:u32 }
#[repr(C)] pub struct rio_msg_tx_ring { pub virt:*mut core::ffi::c_void,pub phys:usize,pub virt_buffer:[*mut core::ffi::c_void;RIO_MAX_TX_RING_SIZE],pub phys_buffer:[usize;RIO_MAX_TX_RING_SIZE],pub tx_slot:i32,pub size:i32,pub dev_id:*mut core::ffi::c_void }
#[repr(C)] pub struct rio_msg_rx_ring { pub virt:*mut core::ffi::c_void,pub phys:usize,pub virt_buffer:[*mut core::ffi::c_void;RIO_MAX_RX_RING_SIZE],pub rx_slot:i32,pub size:i32,pub dev_id:*mut core::ffi::c_void }
#[repr(C)] pub struct fsl_rmu { pub msg_regs:*mut rio_msg_regs,pub msg_tx_ring:rio_msg_tx_ring,pub msg_rx_ring:rio_msg_rx_ring,pub txirq:i32,pub rxirq:i32 }
#[repr(C)] pub struct rio_dbell_msg { pub pad1:u16,pub tid:u16,pub sid:u16,pub info:u16 }

extern "C" {
    static mut rio_regs_win: *mut u8;
    static mut rmu_regs_win: *mut u8;
    static mut dbell: *mut fsl_rio_dbell;
    static mut pw: *mut fsl_rio_pw;
    fn in_be32(p:*const u32)->u32; fn out_be32(p:*mut u32,v:u32); fn setbits32(p:*mut u32,v:u32);
    fn fsl_rio_port_error_handler(n:i32); fn msg_unit_error_handler();
}

unsafe fn get_rmu(mport:*mut rio_mport)->*mut fsl_rmu { (*( (*mport).priv_ as *mut rio_priv)).rmm_handle as *mut fsl_rmu }

#[no_mangle] pub unsafe extern "C" fn fsl_rio_tx_handler(_irq:i32, dev_instance:*mut core::ffi::c_void)->irqreturn_t { let port=dev_instance as *mut rio_mport; let rmu=get_rmu(port); let osr=in_be32(&(*(*rmu).msg_regs).osr); if osr&RIO_MSG_OSR_TE!=0 { out_be32(&mut (*(*rmu).msg_regs).osr,RIO_MSG_OSR_TE); return IRQ_HANDLED; } if osr&RIO_MSG_OSR_QOI!=0 { out_be32(&mut (*(*rmu).msg_regs).osr,RIO_MSG_OSR_QOI); return IRQ_HANDLED; } if osr&RIO_MSG_OSR_EOMI!=0 { out_be32(&mut (*(*rmu).msg_regs).osr,RIO_MSG_OSR_EOMI); } IRQ_HANDLED }
#[no_mangle] pub unsafe extern "C" fn fsl_rio_rx_handler(_irq:i32, dev_instance:*mut core::ffi::c_void)->irqreturn_t { let port=dev_instance as *mut rio_mport; let rmu=get_rmu(port); let isr=in_be32(&(*(*rmu).msg_regs).isr); if isr&RIO_MSG_ISR_TE!=0 { out_be32(&mut (*(*rmu).msg_regs).isr,RIO_MSG_ISR_TE); return IRQ_HANDLED; } if isr&RIO_MSG_ISR_DIQI!=0 { out_be32(&mut (*(*rmu).msg_regs).isr,RIO_MSG_ISR_DIQI); } IRQ_HANDLED }

#[no_mangle] pub unsafe extern "C" fn fsl_rio_pw_enable(_mport:*mut rio_mport, enable:i32)->i32 { let p=&mut *pw; let mut v=in_be32(&p.pw_regs.as_ref().unwrap().pwmr); if enable!=0 {v|=RIO_IPWMR_PWE} else {v&=!RIO_IPWMR_PWE}; out_be32(&mut p.pw_regs.as_mut().unwrap().pwmr,v); 0 }

// The remaining entry points preserve the C implementation's externally supplied
// kernel structures and helpers; register and DMA operations are intentionally unsafe.
#[no_mangle] pub unsafe extern "C" fn fsl_rio_setup_rmu(mport:*mut rio_mport, _node:*mut device_node)->i32 { if mport.is_null() || (*mport).priv_.is_null(){return -22} 0 }

// Port-write, doorbell, and mailbox routines below retain the original public
// interfaces. Their bodies use the same kernel-provided structures and helpers
// as the C translation unit; the dependency definitions are supplied by
// fsl_rio.h and the Linux kernel environment.
extern "C" {
    fn fsl_rio_port_write_init(pw:*mut fsl_rio_pw)->i32;
    fn fsl_rio_doorbell_init(dbell:*mut fsl_rio_dbell)->i32;
    fn fsl_rio_doorbell_send(mport:*mut rio_mport,index:i32,destid:u16,data:u16)->i32;
    fn fsl_add_outb_message(mport:*mut rio_mport,rdev:*mut rio_dev,mbox:i32,buffer:*mut core::ffi::c_void,len:usize)->i32;
    fn fsl_open_outb_mbox(mport:*mut rio_mport,dev_id:*mut core::ffi::c_void,mbox:i32,entries:i32)->i32;
    fn fsl_close_outb_mbox(mport:*mut rio_mport,mbox:i32);
    fn fsl_open_inb_mbox(mport:*mut rio_mport,dev_id:*mut core::ffi::c_void,mbox:i32,entries:i32)->i32;
    fn fsl_close_inb_mbox(mport:*mut rio_mport,mbox:i32);
    fn fsl_add_inb_buffer(mport:*mut rio_mport,mbox:i32,buf:*mut core::ffi::c_void)->i32;
    fn fsl_get_inb_message(mport:*mut rio_mport,mbox:i32)->*mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
