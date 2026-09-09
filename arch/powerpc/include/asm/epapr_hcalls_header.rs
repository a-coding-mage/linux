/*
 * ePAPR hcall interface
 *
 * Copyright 2008-2011 Freescale Semiconductor, Inc.
 *
 * Author: Timur Tabi <timur@freescale.com>
 *
 * This file is provided under a dual BSD/GPL license.  When using or
 * redistributing this file, you may do so under either license.
 */

// The C header includes <uapi/asm/epapr_hcalls.h>, Linux types, errno, and
// byte-order support.  Those declarations are supplied by the surrounding
// translation unit.

pub const EV_HCALL_CLOBBERS: &str = "r0, r12, xer, ctr, lr, cc, memory";
pub const EV_HCALL_CLOBBERS8: &str = EV_HCALL_CLOBBERS;
pub const EV_HCALL_CLOBBERS7: &str = "r10";
pub const EV_HCALL_CLOBBERS6: &str = "r9";
pub const EV_HCALL_CLOBBERS5: &str = "r8";
pub const EV_HCALL_CLOBBERS4: &str = "r7";
pub const EV_HCALL_CLOBBERS3: &str = "r6";
pub const EV_HCALL_CLOBBERS2: &str = "r5";
pub const EV_HCALL_CLOBBERS1: &str = "r4";

extern "C" {
    pub static mut epapr_paravirt_enabled: bool;
    pub static mut epapr_hypercall_start: [u32; 0];
}

#[cfg(feature = "CONFIG_EPAPR_PARAVIRT")]
pub unsafe extern "C" fn epapr_paravirt_early_init() -> i32;
#[cfg(not(feature = "CONFIG_EPAPR_PARAVIRT"))]
pub fn epapr_paravirt_early_init() -> i32 { 0 }

#[inline]
unsafe fn hcall5(mut r11: usize, mut r3: usize, mut r4: usize, mut r5: usize, mut r6: usize) -> usize {
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, inout("r4") r4, inout("r5") r5, inout("r6") r6, options(nostack)); r3
}
#[inline]
unsafe fn hcall2(mut r11: usize, mut r3: usize) -> usize {
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, options(nostack)); r3
}

#[inline]
pub unsafe fn ev_int_set_config(interrupt: u32, config: u32, priority: u32, destination: u32) -> u32 {
    hcall5(EV_HCALL_TOKEN(EV_INT_SET_CONFIG) as usize, interrupt as usize, config as usize, priority as usize, destination as usize) as u32
}
#[inline]
pub unsafe fn ev_int_get_config(interrupt: u32, config: *mut u32, priority: *mut u32, destination: *mut u32) -> u32 {
    let mut r4: usize; let mut r5: usize; let mut r6: usize;
    let mut r11 = EV_HCALL_TOKEN(EV_INT_GET_CONFIG) as usize; let mut r3 = interrupt as usize;
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, lateout("r4") r4, lateout("r5") r5, lateout("r6") r6, options(nostack));
    *config = r4 as u32; *priority = r5 as u32; *destination = r6 as u32; r3 as u32
}
#[inline] pub unsafe fn ev_int_set_mask(interrupt: u32, mask: u32) -> u32 { hcall5(EV_HCALL_TOKEN(EV_INT_SET_MASK) as usize, interrupt as usize, mask as usize, 0, 0) as u32 }
#[inline] pub unsafe fn ev_int_get_mask(interrupt: u32, mask: *mut u32) -> u32 { let mut r4: usize; let mut r11=EV_HCALL_TOKEN(EV_INT_GET_MASK) as usize; let mut r3=interrupt as usize; core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, lateout("r4") r4, options(nostack)); *mask=r4 as u32; r3 as u32 }
#[inline] pub unsafe fn ev_int_eoi(interrupt: u32) -> u32 { hcall2(EV_HCALL_TOKEN(EV_INT_EOI) as usize, interrupt as usize) as u32 }
#[inline] pub unsafe fn ev_int_iack(handle: u32, vector: *mut u32) -> u32 { let mut r4: usize; let mut r11=EV_HCALL_TOKEN(EV_INT_IACK) as usize; let mut r3=handle as usize; core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, lateout("r4") r4, options(nostack)); *vector=r4 as u32; r3 as u32 }
#[inline] pub unsafe fn ev_doorbell_send(handle: u32) -> u32 { hcall2(EV_HCALL_TOKEN(EV_DOORBELL_SEND) as usize, handle as usize) as u32 }
#[inline] pub unsafe fn ev_idle() -> u32 { let mut r11=EV_HCALL_TOKEN(EV_IDLE) as usize; let mut r3: usize; core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, lateout("r3") r3, options(nostack)); r3 as u32 }

#[inline]
pub unsafe fn ev_byte_channel_send(handle: u32, count: *mut u32, buffer: *const u8) -> u32 {
    let p = buffer as *const u32; let mut r11=EV_HCALL_TOKEN(EV_BYTE_CHANNEL_SEND) as usize; let mut r3=handle as usize; let mut r4=*count as usize; let mut r5=u32::from_be(*p) as usize; let mut r6=u32::from_be(*p.add(1)) as usize; let mut r7=u32::from_be(*p.add(2)) as usize; let mut r8=u32::from_be(*p.add(3)) as usize;
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, inout("r4") r4, inout("r5") r5, inout("r6") r6, inout("r7") r7, inout("r8") r8, options(nostack)); *count=r4 as u32; r3 as u32
}
#[inline]
pub unsafe fn ev_byte_channel_receive(handle: u32, count: *mut u32, buffer: *mut u8) -> u32 {
    let p=buffer as *mut u32; let mut r11=EV_HCALL_TOKEN(EV_BYTE_CHANNEL_RECEIVE) as usize; let mut r3=handle as usize; let mut r4=*count as usize; let mut r5: usize; let mut r6: usize; let mut r7: usize; let mut r8: usize;
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, inout("r4") r4, lateout("r5") r5, lateout("r6") r6, lateout("r7") r7, lateout("r8") r8, options(nostack)); *count=r4 as u32; *p=u32::to_be(r5 as u32); *p.add(1)=u32::to_be(r6 as u32); *p.add(2)=u32::to_be(r7 as u32); *p.add(3)=u32::to_be(r8 as u32); r3 as u32
}
#[inline] pub unsafe fn ev_byte_channel_poll(handle: u32, rx_count: *mut u32, tx_count: *mut u32) -> u32 { let mut r4:usize; let mut r5:usize; let mut r11=EV_HCALL_TOKEN(EV_BYTE_CHANNEL_POLL) as usize; let mut r3=handle as usize; core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, lateout("r4") r4, lateout("r5") r5, options(nostack)); *rx_count=r4 as u32; *tx_count=r5 as u32; r3 as u32 }

#[cfg(feature = "CONFIG_EPAPR_PARAVIRT")]
pub unsafe fn epapr_hypercall(in_: *const usize, out: *mut usize, nr: usize) -> isize {
    let mut r3=*in_; let mut r4=*in_.add(1); let mut r5=*in_.add(2); let mut r6=*in_.add(3); let mut r7=*in_.add(4); let mut r8=*in_.add(5); let mut r9=*in_.add(6); let mut r10=*in_.add(7); let mut r11=nr; let mut r0:usize; let mut r12:usize;
    core::arch::asm!("bl epapr_hypercall_start", lateout("r0") r0, inout("r3") r3, inout("r4") r4, inout("r5") r5, inout("r6") r6, inout("r7") r7, inout("r8") r8, inout("r9") r9, inout("r10") r10, inout("r11") r11, lateout("r12") r12, options(nostack));
    *out.add(0)=r4; *out.add(1)=r5; *out.add(2)=r6; *out.add(3)=r7; *out.add(4)=r8; *out.add(5)=r9; *out.add(6)=r10; *out.add(7)=r11; r3 as isize
}
#[cfg(not(feature = "CONFIG_EPAPR_PARAVIRT"))]
pub unsafe fn epapr_hypercall(_in: *const usize, _out: *mut usize, _nr: usize) -> isize { EV_UNIMPLEMENTED as isize }

#[inline] pub unsafe fn epapr_hypercall0_1(nr:u32,r2:*mut usize)->isize { let i=[0usize;8]; let mut o=[0usize;8]; let r=epapr_hypercall(i.as_ptr(),o.as_mut_ptr(),nr as usize); *r2=o[0]; r }
#[inline] pub unsafe fn epapr_hypercall0(nr:u32)->isize { let i=[0usize;8]; let mut o=[0usize;8]; epapr_hypercall(i.as_ptr(),o.as_mut_ptr(),nr as usize) }
#[inline] pub unsafe fn epapr_hypercall1(nr:u32,p1:usize)->isize { let mut i=[0usize;8]; let mut o=[0usize;8]; i[0]=p1; epapr_hypercall(i.as_ptr(),o.as_mut_ptr(),nr as usize) }
#[inline] pub unsafe fn epapr_hypercall2(nr:u32,p1:usize,p2:usize)->isize { let mut i=[0usize;8]; let mut o=[0usize;8]; i[0]=p1;i[1]=p2;epapr_hypercall(i.as_ptr(),o.as_mut_ptr(),nr as usize) }
#[inline] pub unsafe fn epapr_hypercall3(nr:u32,p1:usize,p2:usize,p3:usize)->isize { let mut i=[0usize;8]; let mut o=[0usize;8]; i[0]=p1;i[1]=p2;i[2]=p3;epapr_hypercall(i.as_ptr(),o.as_mut_ptr(),nr as usize) }
#[inline] pub unsafe fn epapr_hypercall4(nr:u32,p1:usize,p2:usize,p3:usize,p4:usize)->isize { let mut i=[0usize;8]; let mut o=[0usize;8]; i[0]=p1;i[1]=p2;i[2]=p3;i[3]=p4;epapr_hypercall(i.as_ptr(),o.as_mut_ptr(),nr as usize) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
