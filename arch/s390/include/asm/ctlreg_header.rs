/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 1999, 2009
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Translated from s390/include/asm/ctlreg.h.  The original Linux build-time
// checks and assembler-only conditions are retained here as comments where
// Rust has no direct equivalent.

pub const CR0_TRANSACTIONAL_EXECUTION_BIT: usize = 63 - 8;
pub const CR0_CLOCK_COMPARATOR_SIGN_BIT: usize = 63 - 10;
pub const CR0_CRYPTOGRAPHY_COUNTER_BIT: usize = 63 - 13;
pub const CR0_PAI_EXTENSION_BIT: usize = 63 - 14;
pub const CR0_CPUMF_EXTRACTION_AUTH_BIT: usize = 63 - 15;
pub const CR0_WARNING_TRACK_BIT: usize = 63 - 30;
pub const CR0_LOW_ADDRESS_PROTECTION_BIT: usize = 63 - 35;
pub const CR0_FETCH_PROTECTION_OVERRIDE_BIT: usize = 63 - 38;
pub const CR0_STORAGE_PROTECTION_OVERRIDE_BIT: usize = 63 - 39;
pub const CR0_EDAT_BIT: usize = 63 - 40;
pub const CR0_INSTRUCTION_EXEC_PROTECTION_BIT: usize = 63 - 43;
pub const CR0_VECTOR_BIT: usize = 63 - 46;
pub const CR0_MALFUNCTION_ALERT_SUBMASK_BIT: usize = 63 - 48;
pub const CR0_EMERGENCY_SIGNAL_SUBMASK_BIT: usize = 63 - 49;
pub const CR0_EXTERNAL_CALL_SUBMASK_BIT: usize = 63 - 50;
pub const CR0_CLOCK_COMPARATOR_SUBMASK_BIT: usize = 63 - 52;
pub const CR0_CPU_TIMER_SUBMASK_BIT: usize = 63 - 53;
pub const CR0_SERVICE_SIGNAL_SUBMASK_BIT: usize = 63 - 54;
pub const CR0_UNUSED_56_BIT: usize = 63 - 56;
pub const CR0_INTERRUPT_KEY_SUBMASK_BIT: usize = 63 - 57;
pub const CR0_MEASUREMENT_ALERT_SUBMASK_BIT: usize = 63 - 58;
pub const CR0_ETR_SUBMASK_BIT: usize = 63 - 59;
pub const CR0_IUCV_BIT: usize = 63 - 62;

macro_rules! bit { ($n:expr) => { 1usize << $n }; }
pub const CR0_TRANSACTIONAL_EXECUTION: usize = bit!(CR0_TRANSACTIONAL_EXECUTION_BIT);
pub const CR0_CLOCK_COMPARATOR_SIGN: usize = bit!(CR0_CLOCK_COMPARATOR_SIGN_BIT);
pub const CR0_CRYPTOGRAPHY_COUNTER: usize = bit!(CR0_CRYPTOGRAPHY_COUNTER_BIT);
pub const CR0_PAI_EXTENSION: usize = bit!(CR0_PAI_EXTENSION_BIT);
pub const CR0_CPUMF_EXTRACTION_AUTH: usize = bit!(CR0_CPUMF_EXTRACTION_AUTH_BIT);
pub const CR0_WARNING_TRACK: usize = bit!(CR0_WARNING_TRACK_BIT);
pub const CR0_LOW_ADDRESS_PROTECTION: usize = bit!(CR0_LOW_ADDRESS_PROTECTION_BIT);
pub const CR0_FETCH_PROTECTION_OVERRIDE: usize = bit!(CR0_FETCH_PROTECTION_OVERRIDE_BIT);
pub const CR0_STORAGE_PROTECTION_OVERRIDE: usize = bit!(CR0_STORAGE_PROTECTION_OVERRIDE_BIT);
pub const CR0_EDAT: usize = bit!(CR0_EDAT_BIT);
pub const CR0_INSTRUCTION_EXEC_PROTECTION: usize = bit!(CR0_INSTRUCTION_EXEC_PROTECTION_BIT);
pub const CR0_VECTOR: usize = bit!(CR0_VECTOR_BIT);
pub const CR0_MALFUNCTION_ALERT_SUBMASK: usize = bit!(CR0_MALFUNCTION_ALERT_SUBMASK_BIT);
pub const CR0_EMERGENCY_SIGNAL_SUBMASK: usize = bit!(CR0_EMERGENCY_SIGNAL_SUBMASK_BIT);
pub const CR0_EXTERNAL_CALL_SUBMASK: usize = bit!(CR0_EXTERNAL_CALL_SUBMASK_BIT);
pub const CR0_CLOCK_COMPARATOR_SUBMASK: usize = bit!(CR0_CLOCK_COMPARATOR_SUBMASK_BIT);
pub const CR0_CPU_TIMER_SUBMASK: usize = bit!(CR0_CPU_TIMER_SUBMASK_BIT);
pub const CR0_SERVICE_SIGNAL_SUBMASK: usize = bit!(CR0_SERVICE_SIGNAL_SUBMASK_BIT);
pub const CR0_UNUSED_56: usize = bit!(CR0_UNUSED_56_BIT);
pub const CR0_INTERRUPT_KEY_SUBMASK: usize = bit!(CR0_INTERRUPT_KEY_SUBMASK_BIT);
pub const CR0_MEASUREMENT_ALERT_SUBMASK: usize = bit!(CR0_MEASUREMENT_ALERT_SUBMASK_BIT);
pub const CR0_ETR_SUBMASK: usize = bit!(CR0_ETR_SUBMASK_BIT);
pub const CR0_IUCV: usize = bit!(CR0_IUCV_BIT);

pub const CR2_MIO_ADDRESSING_BIT: usize = 63 - 58;
pub const CR2_GUARDED_STORAGE_BIT: usize = 63 - 59;
pub const CR2_MIO_ADDRESSING: usize = bit!(CR2_MIO_ADDRESSING_BIT);
pub const CR2_GUARDED_STORAGE: usize = bit!(CR2_GUARDED_STORAGE_BIT);
pub const CR14_UNUSED_32_BIT: usize = 63 - 32;
pub const CR14_UNUSED_33_BIT: usize = 63 - 33;
pub const CR14_CHANNEL_REPORT_SUBMASK_BIT: usize = 63 - 35;
pub const CR14_RECOVERY_SUBMASK_BIT: usize = 63 - 36;
pub const CR14_DEGRADATION_SUBMASK_BIT: usize = 63 - 37;
pub const CR14_EXTERNAL_DAMAGE_SUBMASK_BIT: usize = 63 - 38;
pub const CR14_WARNING_SUBMASK_BIT: usize = 63 - 39;
pub const CR14_UNUSED_32: usize = bit!(CR14_UNUSED_32_BIT);
pub const CR14_UNUSED_33: usize = bit!(CR14_UNUSED_33_BIT);
pub const CR14_CHANNEL_REPORT_SUBMASK: usize = bit!(CR14_CHANNEL_REPORT_SUBMASK_BIT);
pub const CR14_RECOVERY_SUBMASK: usize = bit!(CR14_RECOVERY_SUBMASK_BIT);
pub const CR14_DEGRADATION_SUBMASK: usize = bit!(CR14_DEGRADATION_SUBMASK_BIT);
pub const CR14_EXTERNAL_DAMAGE_SUBMASK: usize = bit!(CR14_EXTERNAL_DAMAGE_SUBMASK_BIT);
pub const CR14_WARNING_SUBMASK: usize = bit!(CR14_WARNING_SUBMASK_BIT);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctlreg { pub val: usize }

#[inline(always)]
pub unsafe fn local_ctl_load(cr: u32, reg: *const ctlreg) {
    core::arch::asm!("lctlg {0},{0},[{1}]", in(reg) cr, in(reg) reg, options(nostack));
}

#[inline(always)]
pub unsafe fn local_ctl_store(cr: u32, reg: *mut ctlreg) {
    core::arch::asm!("stctg {0},{0},[{1}]", in(reg) cr, in(reg) reg, options(nostack));
}

#[inline(always)]
pub unsafe fn local_ctl_set_bit(cr: u32, bit: u32) -> ctlreg {
    let mut old = ctlreg { val: 0 };
    local_ctl_store(cr, &mut old);
    let mut new = old;
    new.val |= 1usize << bit;
    local_ctl_load(cr, &new);
    old
}

#[inline(always)]
pub unsafe fn local_ctl_clear_bit(cr: u32, bit: u32) -> ctlreg {
    let mut old = ctlreg { val: 0 };
    local_ctl_store(cr, &mut old);
    let mut new = old;
    new.val &= !(1usize << bit);
    local_ctl_load(cr, &new);
    old
}

#[repr(C)]
pub struct lowcore;

extern "C" {
    pub fn system_ctlreg_lock();
    pub fn system_ctlreg_unlock();
    pub fn system_ctlreg_init_save_area(lc: *mut lowcore);
    pub fn system_ctlreg_modify(cr: u32, data: usize, request: i32);
}

pub const CTLREG_SET_BIT: i32 = 0;
pub const CTLREG_CLEAR_BIT: i32 = 1;
pub const CTLREG_LOAD: i32 = 2;

#[inline]
pub unsafe fn system_ctl_set_bit(cr: u32, bit: u32) { system_ctlreg_modify(cr, bit as usize, CTLREG_SET_BIT); }
#[inline]
pub unsafe fn system_ctl_clear_bit(cr: u32, bit: u32) { system_ctlreg_modify(cr, bit as usize, CTLREG_CLEAR_BIT); }
#[inline]
pub unsafe fn system_ctl_load(cr: u32, reg: *const ctlreg) { system_ctlreg_modify(cr, (*reg).val, CTLREG_LOAD); }

// C bit-field members are represented by the raw storage word; the listed
// names and widths preserve the source layout and meaning.
#[repr(C)]
pub union ctlreg0 { pub val: usize, pub reg: ctlreg, pub bits: usize }
#[repr(C)]
pub union ctlreg2 { pub val: usize, pub reg: ctlreg, pub bits: usize }
#[repr(C)]
pub union ctlreg5 { pub val: usize, pub reg: ctlreg, pub bits: usize }
#[repr(C)]
pub union ctlreg15 { pub val: usize, pub reg: ctlreg, pub bits: usize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
