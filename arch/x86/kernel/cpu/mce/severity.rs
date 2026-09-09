// SPDX-License-Identifier: GPL-2.0-only
/*
 * MCE grading rules.
 * Copyright 2008, 2009 Intel Corporation.
 *
 * Author: Andi Kleen
 */

#[repr(i32)]
enum Context { IN_KERNEL = 1, IN_USER = 2, IN_KERNEL_RECOV = 3 }
#[repr(i32)]
enum Ser { SER_REQUIRED = 1, NO_SER = 2 }
#[repr(i32)]
enum Exception { EXCP_CONTEXT = 1, NO_EXCP = 2 }

#[repr(C)]
struct Severity {
    mask: u64,
    result: u64,
    sev: u8,
    mcgmask: u16,
    mcgres: u16,
    ser: u8,
    context: u8,
    excp: u8,
    covered: u8,
    cpu_vfm: u32,
    cpu_minstepping: u8,
    bank_lo: u8,
    bank_hi: u8,
    msg: *mut i8,
}

/* The constants below are supplied by the kernel headers. */

static mut SEVERITIES: [Severity; 31] = [
    Severity { mask: MCI_STATUS_VAL, result: 0, sev: MCE_NO_SEVERITY, mcgmask: 0, mcgres: 0, ser: 0, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Invalid\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_EN, result: 0, sev: MCE_NO_SEVERITY, mcgmask: 0, mcgres: 0, ser: 0, context: 0, excp: EXCP_CONTEXT as u8, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Not enabled\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_PCC, result: MCI_STATUS_PCC, sev: MCE_PANIC_SEVERITY, mcgmask: 0, mcgres: 0, ser: 0, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Processor context corrupt\0" as *const u8 as *mut i8 },
    Severity { mask: 0, result: 0, sev: MCE_PANIC_SEVERITY, mcgmask: MCG_STATUS_MCIP as u16, mcgres: 0, ser: 0, context: 0, excp: EXCP_CONTEXT as u8, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"MCIP not set in MCA handler\0" as *const u8 as *mut i8 },
    Severity { mask: 0, result: 0, sev: MCE_PANIC_SEVERITY, mcgmask: (MCG_STATUS_RIPV | MCG_STATUS_EIPV) as u16, mcgres: 0, ser: 0, context: 0, excp: EXCP_CONTEXT as u8, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Neither restart nor error IP\0" as *const u8 as *mut i8 },
    Severity { mask: 0, result: 0, sev: MCE_PANIC_SEVERITY, mcgmask: MCG_STATUS_RIPV as u16, mcgres: 0, ser: 0, context: IN_KERNEL as u8, excp: EXCP_CONTEXT as u8, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"In kernel and no restart IP\0" as *const u8 as *mut i8 },
    Severity { mask: 0, result: 0, sev: MCE_PANIC_SEVERITY, mcgmask: MCG_STATUS_RIPV as u16, mcgres: 0, ser: 0, context: IN_KERNEL_RECOV as u8, excp: EXCP_CONTEXT as u8, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"In kernel and no restart IP\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_UC, result: 0, sev: MCE_KEEP_SEVERITY, mcgmask: 0, mcgres: 0, ser: NO_SER as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Corrected error\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_UC_AR | MCACOD_SCRUBMSK, result: MCI_STATUS_UC | MCACOD_SCRUB, sev: MCE_AO_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Action optional: memory scrubbing error\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_UC_AR | MCACOD, result: MCI_STATUS_UC | MCACOD_L3WB, sev: MCE_AO_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Action optional: last level cache writeback error\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_UC | MCI_ADDR | 0xffffeff0, result: MCI_ADDR | 0x001000c0, sev: MCE_AO_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: INTEL_SKYLAKE_X, cpu_minstepping: 4, bank_lo: 13, bank_hi: 18, msg: b"Uncorrected Patrol Scrub Error\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_UC_SAR, result: MCI_STATUS_UC, sev: MCE_UCNA_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Uncorrected no action required\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_OVER | MCI_UC_SAR, result: MCI_STATUS_UC | MCI_STATUS_AR, sev: MCE_PANIC_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Illegal combination (UCNA with AR=1)\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_S, result: 0, sev: MCE_KEEP_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Non signaled machine check\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_OVER | MCI_UC_SAR, result: MCI_STATUS_OVER | MCI_UC_SAR, sev: MCE_PANIC_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Action required with lost events\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_OVER | MCI_UC_SAR, result: MCI_UC_SAR, sev: MCE_PANIC_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Action required: unknown MCACOD\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_OVER | MCI_UC_SAR, result: MCI_UC_S, sev: MCE_SOME_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Action optional: unknown MCACOD\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_OVER | MCI_UC_SAR, result: MCI_STATUS_OVER | MCI_UC_S, sev: MCE_SOME_SEVERITY, mcgmask: 0, mcgres: 0, ser: SER_REQUIRED as u8, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Action optional with lost events\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_OVER | MCI_STATUS_UC, result: MCI_STATUS_OVER | MCI_STATUS_UC, sev: MCE_PANIC_SEVERITY, mcgmask: 0, mcgres: 0, ser: 0, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Overflowed uncorrected\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_UC, result: MCI_STATUS_UC, sev: MCE_PANIC_SEVERITY, mcgmask: 0, mcgres: 0, ser: 0, context: IN_KERNEL as u8, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Uncorrected in kernel\0" as *const u8 as *mut i8 },
    Severity { mask: MCI_STATUS_UC, result: MCI_STATUS_UC, sev: MCE_UC_SEVERITY, mcgmask: 0, mcgres: 0, ser: 0, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"Uncorrected\0" as *const u8 as *mut i8 },
    Severity { mask: 0, result: 0, sev: MCE_SOME_SEVERITY, mcgmask: 0, mcgres: 0, ser: 0, context: 0, excp: 0, covered: 0, cpu_vfm: 0, cpu_minstepping: 0, bank_lo: 0, bank_hi: 0, msg: b"No match\0" as *const u8 as *mut i8 },
];

#[inline]
fn mc_recoverable(mcg: u64) -> bool {
    (mcg & (MCG_STATUS_RIPV | MCG_STATUS_EIPV)) == (MCG_STATUS_RIPV | MCG_STATUS_EIPV)
}

unsafe fn is_copy_from_user(regs: *mut pt_regs) -> bool {
    if regs.is_null() { return false; }
    let mut insn_buf = [0u8; MAX_INSN_SIZE];
    if copy_from_kernel_nofault(insn_buf.as_mut_ptr() as *mut _, (*regs).ip as *const _, MAX_INSN_SIZE) != 0 { return false; }
    let mut insn = core::mem::zeroed::<insn>();
    if insn_decode_kernel(&mut insn, insn_buf.as_ptr()) < 0 { return false; }
    let addr = match insn.opcode.value {
        0x8A | 0x8B | 0xB60F | 0xB70F => insn_get_addr_ref(&insn, regs) as usize,
        0xA4 | 0xA5 => (*regs).si as usize,
        _ => return false,
    };
    if fault_in_kernel_space(addr as u64) { return false; }
    (*current).mce_vaddr = addr as *mut _;
    true
}

unsafe fn error_context(m: *mut mce, regs: *mut pt_regs) -> i32 {
    if ((*m).cs & 3) == 3 { return IN_USER as i32; }
    if !mc_recoverable((*m).mcgstatus) { return IN_KERNEL as i32; }
    instrumentation_begin();
    let fixup_type = ex_get_fixup_type((*m).ip);
    let copy_user = is_copy_from_user(regs);
    instrumentation_end();
    if copy_user { (*m).kflags |= MCE_IN_KERNEL_COPYIN | MCE_IN_KERNEL_RECOV; return IN_KERNEL_RECOV as i32; }
    match fixup_type {
        EX_TYPE_FAULT_MCE_SAFE | EX_TYPE_DEFAULT_MCE_SAFE => { (*m).kflags |= MCE_IN_KERNEL_RECOV; IN_KERNEL_RECOV as i32 }
        _ => IN_KERNEL as i32,
    }
}

unsafe fn mce_severity_amd(m: *mut mce, regs: *mut pt_regs, msg: *mut *mut i8, _is_excp: bool) -> i32 {
    let mut panic_msg: *mut i8 = core::ptr::null_mut();
    let mut ret = MCE_AR_SEVERITY;
    if (*m).status & MCI_STATUS_PCC != 0 { panic_msg = b"Processor Context Corrupt\0" as *const u8 as *mut i8; ret = MCE_PANIC_SEVERITY; }
    else if (*m).status & MCI_STATUS_DEFERRED != 0 { ret = MCE_DEFERRED_SEVERITY; }
    else if (*m).status & MCI_STATUS_UC == 0 { ret = MCE_KEEP_SEVERITY; }
    else if (*m).status & MCI_STATUS_OVER != 0 && !mce_flags.overflow_recov { panic_msg = b"Overflowed uncorrected error without MCA Overflow Recovery\0" as *const u8 as *mut i8; ret = MCE_PANIC_SEVERITY; }
    else if !mce_flags.succor { panic_msg = b"Uncorrected error without MCA Recovery\0" as *const u8 as *mut i8; ret = MCE_PANIC_SEVERITY; }
    else if error_context(m, regs) == IN_KERNEL as i32 { panic_msg = b"Uncorrected unrecoverable error in kernel context\0" as *const u8 as *mut i8; ret = MCE_PANIC_SEVERITY; }
    if !msg.is_null() && !panic_msg.is_null() { *msg = panic_msg; }
    ret
}

unsafe fn mce_severity_intel(m: *mut mce, regs: *mut pt_regs, msg: *mut *mut i8, is_excp: bool) -> i32 {
    let excp = if is_excp { EXCP_CONTEXT as u8 } else { NO_EXCP as u8 };
    let ctx = error_context(m, regs) as u8;
    let base = SEVERITIES.as_mut_ptr();
    for i in 0..SEVERITIES.len() {
        let s = &mut *base.add(i);
        if ((*m).status & s.mask) != s.result || ((*m).mcgstatus & s.mcgmask as u64) != s.mcgres as u64 { continue; }
        if s.ser == SER_REQUIRED as u8 && !mca_cfg.ser || s.ser == NO_SER as u8 && mca_cfg.ser { continue; }
        if s.context != 0 && ctx != s.context || s.excp != 0 && excp != s.excp || s.cpu_vfm != 0 && boot_cpu_data.x86_vfm != s.cpu_vfm || s.cpu_minstepping != 0 && boot_cpu_data.x86_stepping < s.cpu_minstepping || s.bank_lo != 0 && ((*m).bank < s.bank_lo || (*m).bank > s.bank_hi) { continue; }
        if !msg.is_null() { *msg = s.msg; }
        s.covered = 1;
        return s.sev as i32;
    }
    0
}

pub unsafe fn mce_severity(m: *mut mce, regs: *mut pt_regs, msg: *mut *mut i8, is_excp: bool) -> i32 {
    if boot_cpu_data.x86_vendor == X86_VENDOR_AMD || boot_cpu_data.x86_vendor == X86_VENDOR_HYGON { mce_severity_amd(m, regs, msg, is_excp) } else { mce_severity_intel(m, regs, msg, is_excp) }
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn s_start(_f: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    if *pos >= SEVERITIES.len() as loff_t { return core::ptr::null_mut(); }
    SEVERITIES.as_mut_ptr().add(*pos as usize) as *mut _
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn s_next(_f: *mut seq_file, _data: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    if *pos >= SEVERITIES.len() as loff_t { return core::ptr::null_mut(); }
    SEVERITIES.as_mut_ptr().add(*pos as usize) as *mut _
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn s_stop(_f: *mut seq_file, _data: *mut core::ffi::c_void) {}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn s_show(f: *mut seq_file, data: *mut core::ffi::c_void) -> i32 {
    let ser = data as *mut Severity;
    seq_printf(f, b"%d\t%s\n\0" as *const u8 as *const i8, (*ser).covered, (*ser).msg);
    0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn severities_coverage_open(inode: *mut inode, file: *mut file) -> i32 {
    seq_open(file, &SEVERITIES_SEQ_OPS)
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn severities_coverage_write(_file: *mut file, _ubuf: *const u8, count: usize, _ppos: *mut loff_t) -> isize {
    for s in SEVERITIES.iter_mut() { s.covered = 0; }
    count as isize
}

#[cfg(CONFIG_DEBUG_FS)]
static SEVERITIES_SEQ_OPS: seq_operations = seq_operations {
    start: Some(s_start), next: Some(s_next), stop: Some(s_stop), show: Some(s_show),
};

#[cfg(CONFIG_DEBUG_FS)]
static SEVERITIES_COVERAGE_FOPS: file_operations = file_operations {
    open: Some(severities_coverage_open), release: Some(seq_release), read: Some(seq_read),
    write: Some(severities_coverage_write), llseek: Some(seq_lseek),
};

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn severities_debugfs_init() -> i32 {
    let dmce = mce_get_debugfs_dir();
    debugfs_create_file(b"severities-coverage\0" as *const u8 as *const i8, 0o444, dmce, core::ptr::null_mut(), &SEVERITIES_COVERAGE_FOPS);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
