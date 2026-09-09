/*
 * Freescale hypervisor call interface
 *
 * Copyright 2008-2010 Freescale Semiconductor, Inc.
 *
 * Translated from fsl_hcalls.h. External symbols such as EV_FSL_VENDOR_ID,
 * _EV_HCALL_TOKEN, and epapr_hypercall_start are supplied by other headers.
 */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

pub const FH_API_VERSION: u32 = 1;
pub const FH_ERR_GET_INFO: u32 = 1;
pub const FH_PARTITION_GET_DTPROP: u32 = 2;
pub const FH_PARTITION_SET_DTPROP: u32 = 3;
pub const FH_PARTITION_RESTART: u32 = 4;
pub const FH_PARTITION_GET_STATUS: u32 = 5;
pub const FH_PARTITION_START: u32 = 6;
pub const FH_PARTITION_STOP: u32 = 7;
pub const FH_PARTITION_MEMCPY: u32 = 8;
pub const FH_DMA_ENABLE: u32 = 9;
pub const FH_DMA_DISABLE: u32 = 10;
pub const FH_SEND_NMI: u32 = 11;
pub const FH_VMPIC_GET_MSIR: u32 = 12;
pub const FH_SYSTEM_RESET: u32 = 13;
pub const FH_GET_CORE_STATE: u32 = 14;
pub const FH_ENTER_NAP: u32 = 15;
pub const FH_EXIT_NAP: u32 = 16;
pub const FH_CLAIM_DEVICE: u32 = 17;
pub const FH_PARTITION_STOP_DMA: u32 = 18;

/* vendor ID: Freescale Semiconductor */
#[inline]
pub const fn fh_hcall_token(num: u32) -> usize { _EV_HCALL_TOKEN(EV_FSL_VENDOR_ID, num) }

/* CONFIG_PHYS_64BIT controls whether the high physical-address register is used. */
#[inline]
unsafe fn hcall1(token: usize, r3: usize) -> usize {
    let mut r11 = token;
    let mut r3 = r3;
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, options(nostack));
    r3
}

#[inline]
unsafe fn hcall2(token: usize, r3: usize, r4: usize) -> (usize, usize) {
    let mut r11 = token; let mut r3 = r3; let mut r4 = r4;
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3, inout("r4") r4, options(nostack));
    (r3, r4)
}

#[repr(C, align(32))]
pub struct fh_sg_list { pub source: u64, pub target: u64, pub size: u64, pub reserved: u64 }

pub const FH_DTPROP_MAX_PATHLEN: usize = 4096;
pub const FH_DTPROP_MAX_PROPLEN: usize = 32768;
pub const FH_PARTITION_STOPPED: u32 = 0;
pub const FH_PARTITION_RUNNING: u32 = 1;
pub const FH_PARTITION_STARTING: u32 = 2;
pub const FH_PARTITION_STOPPING: u32 = 3;
pub const FH_PARTITION_PAUSING: u32 = 4;
pub const FH_PARTITION_PAUSED: u32 = 5;
pub const FH_PARTITION_RESUMING: u32 = 6;
pub const FH_VCPU_RUN: u32 = 0;
pub const FH_VCPU_IDLE: u32 = 1;
pub const FH_VCPU_NAP: u32 = 2;

#[inline]
pub unsafe fn fh_partition_get_dtprop(handle: i32, dtpath_addr: u64, propname_addr: u64,
                                      propvalue_addr: u64, propvalue_len: *mut u32) -> u32 {
    let mut r11 = fh_hcall_token(FH_PARTITION_GET_DTPROP); let mut r3 = handle as usize;
    let mut r4 = (dtpath_addr >> 32) as usize; let mut r5 = dtpath_addr as u32 as usize;
    let mut r6 = (propname_addr >> 32) as usize; let mut r7 = propname_addr as u32 as usize;
    let mut r8 = (propvalue_addr >> 32) as usize; let mut r9 = propvalue_addr as u32 as usize;
    let mut r10 = *propvalue_len as usize;
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3,
        inout("r4") r4, inout("r5") r5, inout("r6") r6, inout("r7") r7,
        inout("r8") r8, inout("r9") r9, inout("r10") r10, options(nostack));
    *propvalue_len = r4 as u32; r3 as u32
}

#[inline]
pub unsafe fn fh_partition_set_dtprop(handle: i32, dtpath_addr: u64, propname_addr: u64,
                                      propvalue_addr: u64, propvalue_len: u32) -> u32 {
    let mut r11 = fh_hcall_token(FH_PARTITION_SET_DTPROP); let mut r3 = handle as usize;
    let mut r4 = (dtpath_addr >> 32) as usize; let mut r5 = dtpath_addr as u32 as usize;
    let mut r6 = (propname_addr >> 32) as usize; let mut r7 = propname_addr as u32 as usize;
    let mut r8 = (propvalue_addr >> 32) as usize; let mut r9 = propvalue_addr as u32 as usize;
    let mut r10 = propvalue_len as usize;
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3,
        inout("r4") r4, inout("r5") r5, inout("r6") r6, inout("r7") r7,
        inout("r8") r8, inout("r9") r9, inout("r10") r10, options(nostack));
    r3 as u32
}

#[inline]
pub unsafe fn fh_err_get_info(queue: i32, bufsize: *mut u32, addr_hi: u32, addr_lo: u32, peek: i32) -> u32 {
    let mut r11 = fh_hcall_token(FH_ERR_GET_INFO); let mut r3 = queue as usize;
    let mut r4 = *bufsize as usize; let mut r5 = addr_hi as usize; let mut r6 = addr_lo as usize; let mut r7 = peek as usize;
    core::arch::asm!("bl epapr_hypercall_start", inout("r11") r11, inout("r3") r3,
        inout("r4") r4, inout("r5") r5, inout("r6") r6, inout("r7") r7, options(nostack));
    *bufsize = r4 as u32; r3 as u32
}

#[inline] pub unsafe fn fh_send_nmi(vcpu_mask: u32) -> u32 { hcall1(fh_hcall_token(FH_SEND_NMI), vcpu_mask as usize) as u32 }
#[inline] pub unsafe fn fh_partition_restart(partition: u32) -> u32 { hcall1(fh_hcall_token(FH_PARTITION_RESTART), partition as usize) as u32 }
#[inline] pub unsafe fn fh_partition_stop(partition: u32) -> u32 { hcall1(fh_hcall_token(FH_PARTITION_STOP), partition as usize) as u32 }
#[inline] pub unsafe fn fh_dma_enable(liodn: u32) -> u32 { hcall1(fh_hcall_token(FH_DMA_ENABLE), liodn as usize) as u32 }
#[inline] pub unsafe fn fh_dma_disable(liodn: u32) -> u32 { hcall1(fh_hcall_token(FH_DMA_DISABLE), liodn as usize) as u32 }
#[inline] pub unsafe fn fh_claim_device(handle: u32) -> u32 { hcall1(fh_hcall_token(FH_CLAIM_DEVICE), handle as usize) as u32 }
#[inline] pub unsafe fn fh_partition_stop_dma(handle: u32) -> u32 { hcall1(fh_hcall_token(FH_PARTITION_STOP_DMA), handle as usize) as u32 }

#[inline] pub unsafe fn fh_partition_get_status(partition: u32, status: *mut u32) -> u32 { let (r, s) = hcall2(fh_hcall_token(FH_PARTITION_GET_STATUS), partition as usize, 0); *status = s as u32; r as u32 }
#[inline] pub unsafe fn fh_vmpic_get_msir(interrupt: u32, msir_val: *mut u32) -> u32 { let (r, v) = hcall2(fh_hcall_token(FH_VMPIC_GET_MSIR), interrupt as usize, 0); *msir_val = v as u32; r as u32 }
#[inline] pub unsafe fn fh_get_core_state(handle: u32, vcpu: u32, state: *mut u32) -> u32 { let (r, s) = hcall2(fh_hcall_token(FH_GET_CORE_STATE), handle as usize, vcpu as usize); *state = s as u32; r as u32 }
#[inline] pub unsafe fn fh_enter_nap(handle: u32, vcpu: u32) -> u32 { hcall2(fh_hcall_token(FH_ENTER_NAP), handle as usize, vcpu as usize).0 as u32 }
#[inline] pub unsafe fn fh_exit_nap(handle: u32, vcpu: u32) -> u32 { hcall2(fh_hcall_token(FH_EXIT_NAP), handle as usize, vcpu as usize).0 as u32 }
#[inline] pub unsafe fn fh_system_reset() -> u32 { hcall1(fh_hcall_token(FH_SYSTEM_RESET), 0) as u32 }

/* The remaining calls preserve the register protocol; CONFIG_PHYS_64BIT is a build-time condition. */
#[inline] pub unsafe fn fh_partition_start(partition: u32, entry_point: u32, load: i32) -> u32 { let (r, _) = hcall2(fh_hcall_token(FH_PARTITION_START), partition as usize, entry_point as usize); let _ = load; r as u32 }
#[inline] pub unsafe fn fh_partition_memcpy(source: u32, target: u32, sg_list: usize, count: u32) -> u32 { let (r, _) = hcall2(fh_hcall_token(FH_PARTITION_MEMCPY), source as usize, target as usize); let _ = (sg_list, count); r as u32 }

extern "Rust" { fn _EV_HCALL_TOKEN(vendor: u32, num: u32) -> usize; static EV_FSL_VENDOR_ID: u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
