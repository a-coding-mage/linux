/*
 * Read/Write Loongson Extension Registers
 */

// External kernel types, register helpers, assembler configuration, and
// address conversion are supplied by other translation units.

#[inline]
pub unsafe fn cpu_has_cfg() -> bool {
    (read_c0_prid() & PRID_IMP_MASK) == PRID_IMP_LOONGSON_64G
}

#[inline]
pub unsafe fn read_cpucfg(reg: u32) -> u32 {
    let res: u32;
    core::arch::asm!(
        "/* _ASM_SET_PARSE_R */",
        "/* parse_r res,{res} */",
        "/* parse_r reg,{reg} */",
        "/* _ASM_UNSET_PARSE_R */",
        ".insn",
        ".word (0xc8080118 | ({reg} << 21) | ({res} << 11))",
        res = lateout(reg) res,
        reg = in(reg) reg,
        options(nostack, preserves_flags)
    );
    res
}

/* Bit Domains for CFG registers */
pub const LOONGSON_CFG0: u32 = 0x0;
pub const LOONGSON_CFG0_PRID: u32 = 0xffff_ffff;

pub const LOONGSON_CFG1: u32 = 0x1;
pub const LOONGSON_CFG1_FP: u32 = 1 << 0;
pub const LOONGSON_CFG1_FPREV: u32 = 0b111 << 1;
pub const LOONGSON_CFG1_MMI: u32 = 1 << 4;
pub const LOONGSON_CFG1_MSA1: u32 = 1 << 5;
pub const LOONGSON_CFG1_MSA2: u32 = 1 << 6;
pub const LOONGSON_CFG1_CGP: u32 = 1 << 7;
pub const LOONGSON_CFG1_WRP: u32 = 1 << 8;
pub const LOONGSON_CFG1_LSX1: u32 = 1 << 9;
pub const LOONGSON_CFG1_LSX2: u32 = 1 << 10;
pub const LOONGSON_CFG1_LASX: u32 = 1 << 11;
pub const LOONGSON_CFG1_R6FXP: u32 = 1 << 12;
pub const LOONGSON_CFG1_R6CRCP: u32 = 1 << 13;
pub const LOONGSON_CFG1_R6FPP: u32 = 1 << 14;
pub const LOONGSON_CFG1_CNT64: u32 = 1 << 15;
pub const LOONGSON_CFG1_LSLDR0: u32 = 1 << 16;
pub const LOONGSON_CFG1_LSPREF: u32 = 1 << 17;
pub const LOONGSON_CFG1_LSPREFX: u32 = 1 << 18;
pub const LOONGSON_CFG1_LSSYNCI: u32 = 1 << 19;
pub const LOONGSON_CFG1_LSUCA: u32 = 1 << 20;
pub const LOONGSON_CFG1_LLSYNC: u32 = 1 << 21;
pub const LOONGSON_CFG1_TGTSYNC: u32 = 1 << 22;
pub const LOONGSON_CFG1_LLEXC: u32 = 1 << 23;
pub const LOONGSON_CFG1_SCRAND: u32 = 1 << 24;
pub const LOONGSON_CFG1_MUALP: u32 = 1 << 25;
pub const LOONGSON_CFG1_KMUALEN: u32 = 1 << 26;
pub const LOONGSON_CFG1_ITLBT: u32 = 1 << 27;
pub const LOONGSON_CFG1_LSUPERF: u32 = 1 << 28;
pub const LOONGSON_CFG1_SFBP: u32 = 1 << 29;
pub const LOONGSON_CFG1_CDMAP: u32 = 1 << 30;
pub const LOONGSON_CFG1_FPREV_OFFSET: u32 = 1;

pub const LOONGSON_CFG2: u32 = 0x2;
pub const LOONGSON_CFG2_LEXT1: u32 = 1 << 0;
pub const LOONGSON_CFG2_LEXT2: u32 = 1 << 1;
pub const LOONGSON_CFG2_LEXT3: u32 = 1 << 2;
pub const LOONGSON_CFG2_LSPW: u32 = 1 << 3;
pub const LOONGSON_CFG2_LBT1: u32 = 1 << 4;
pub const LOONGSON_CFG2_LBT2: u32 = 1 << 5;
pub const LOONGSON_CFG2_LBT3: u32 = 1 << 6;
pub const LOONGSON_CFG2_LBTMMU: u32 = 1 << 7;
pub const LOONGSON_CFG2_LPMP: u32 = 1 << 8;
pub const LOONGSON_CFG2_LPMREV: u32 = 0b111 << 9;
pub const LOONGSON_CFG2_LAMO: u32 = 1 << 12;
pub const LOONGSON_CFG2_LPIXU: u32 = 1 << 13;
pub const LOONGSON_CFG2_LPIXNU: u32 = 1 << 14;
pub const LOONGSON_CFG2_LVZP: u32 = 1 << 15;
pub const LOONGSON_CFG2_LVZREV: u32 = 0b111 << 16;
pub const LOONGSON_CFG2_LGFTP: u32 = 1 << 19;
pub const LOONGSON_CFG2_LGFTPREV: u32 = 0b111 << 20;
pub const LOONGSON_CFG2_LLFTP: u32 = 1 << 23;
pub const LOONGSON_CFG2_LLFTPREV: u32 = 0b111 << 24;
pub const LOONGSON_CFG2_LCSRP: u32 = 1 << 27;
pub const LOONGSON_CFG2_LDISBLIKELY: u32 = 1 << 28;
pub const LOONGSON_CFG2_LPMREV_OFFSET: u32 = 9;
pub const LOONGSON_CFG2_LPM_REV1: u32 = 1 << LOONGSON_CFG2_LPMREV_OFFSET;
pub const LOONGSON_CFG2_LPM_REV2: u32 = 2 << LOONGSON_CFG2_LPMREV_OFFSET;
pub const LOONGSON_CFG2_LVZREV_OFFSET: u32 = 16;
pub const LOONGSON_CFG2_LVZ_REV1: u32 = 1 << LOONGSON_CFG2_LVZREV_OFFSET;
pub const LOONGSON_CFG2_LVZ_REV2: u32 = 2 << LOONGSON_CFG2_LVZREV_OFFSET;

pub const LOONGSON_CFG3: u32 = 0x3;
pub const LOONGSON_CFG3_LCAMP: u32 = 1 << 0;
pub const LOONGSON_CFG3_LCAMREV: u32 = 0b111 << 1;
pub const LOONGSON_CFG3_LCAMNUM: u32 = 0xff << 4;
pub const LOONGSON_CFG3_LCAMKW: u32 = 0xff << 12;
pub const LOONGSON_CFG3_LCAMVW: u32 = 0xff << 20;
pub const LOONGSON_CFG3_LCAMREV_OFFSET: u32 = 1;
pub const LOONGSON_CFG3_LCAM_REV1: u32 = 1 << LOONGSON_CFG3_LCAMREV_OFFSET;
pub const LOONGSON_CFG3_LCAM_REV2: u32 = 2 << LOONGSON_CFG3_LCAMREV_OFFSET;
pub const LOONGSON_CFG3_LCAMNUM_OFFSET: u32 = 4;
pub const LOONGSON_CFG3_LCAMNUM_REV1: u32 = 0x3f << LOONGSON_CFG3_LCAMNUM_OFFSET;
pub const LOONGSON_CFG3_LCAMKW_OFFSET: u32 = 12;
pub const LOONGSON_CFG3_LCAMKW_REV1: u32 = 0x27 << LOONGSON_CFG3_LCAMKW_OFFSET;
pub const LOONGSON_CFG3_LCAMVW_OFFSET: u32 = 20;
pub const LOONGSON_CFG3_LCAMVW_REV1: u32 = 0x3f << LOONGSON_CFG3_LCAMVW_OFFSET;
pub const LOONGSON_CFG4: u32 = 0x4;
pub const LOONGSON_CFG4_CCFREQ: u32 = 0xffff_ffff;
pub const LOONGSON_CFG5: u32 = 0x5;
pub const LOONGSON_CFG5_CFM: u32 = 0xffff;
pub const LOONGSON_CFG5_CFD: u32 = 0xffff << 16;
pub const LOONGSON_CFG6: u32 = 0x6;
pub const LOONGSON_CFG7: u32 = 0x7;
pub const LOONGSON_CFG7_GCCAEQRP: u32 = 1 << 0;
pub const LOONGSON_CFG7_UCAWINP: u32 = 1 << 1;

#[inline]
pub unsafe fn cpu_has_csr() -> bool {
    if cpu_has_cfg() { (read_cpucfg(LOONGSON_CFG2) & LOONGSON_CFG2_LCSRP) != 0 } else { false }
}

#[inline]
pub unsafe fn csr_readl(reg: u32) -> u32 {
    let res: u32;
    core::arch::asm!(".word (0xc8000118 | ({reg} << 21) | ({res} << 11))", res = lateout(reg) res, reg = in(reg) reg, options(nostack, preserves_flags));
    res
}

#[inline]
pub unsafe fn csr_readq(reg: u32) -> u64 {
    let res: u64;
    core::arch::asm!(".word (0xc8020118 | ({reg} << 21) | ({res} << 11))", res = lateout(reg) res, reg = in(reg) reg, options(nostack, preserves_flags));
    res
}

#[inline]
pub unsafe fn csr_writel(val: u32, reg: u32) {
    core::arch::asm!(".word (0xc8010118 | ({reg} << 21) | ({val} << 11))", reg = in(reg) reg, val = in(reg) val, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn csr_writeq(val: u64, reg: u32) {
    core::arch::asm!(".word (0xc8030118 | ({reg} << 21) | ({val} << 11))", reg = in(reg) reg, val = in(reg) val, options(nostack, preserves_flags));
}

/* Public CSR Register can also be accessed with regular addresses */
pub const CSR_PUBLIC_MMIO_BASE: usize = 0x1fe00000;

#[macro_export]
macro_rules! MMIO_CSR { ($x:expr) => { unsafe { TO_UNCAC(CSR_PUBLIC_MMIO_BASE + $x) as *mut core::ffi::c_void } }; }

pub const LOONGSON_CSR_FEATURES: usize = 0x8;
pub const LOONGSON_CSRF_TEMP: u32 = 1 << 0;
pub const LOONGSON_CSRF_NODECNT: u32 = 1 << 1;
pub const LOONGSON_CSRF_MSI: u32 = 1 << 2;
pub const LOONGSON_CSRF_EXTIOI: u32 = 1 << 3;
pub const LOONGSON_CSRF_IPI: u32 = 1 << 4;
pub const LOONGSON_CSRF_FREQ: u32 = 1 << 5;
pub const LOONGSON_CSR_VENDOR: usize = 0x10; /* Vendor name string, should be "Loongson" */
pub const LOONGSON_CSR_CPUNAME: usize = 0x20; /* Processor name string */
pub const LOONGSON_CSR_NODECNT: usize = 0x408;
pub const LOONGSON_CSR_CPUTEMP: usize = 0x428;

/* PerCore CSR, only accessible by local cores */
pub const LOONGSON_CSR_IPI_STATUS: usize = 0x1000;
pub const LOONGSON_CSR_IPI_EN: usize = 0x1004;
pub const LOONGSON_CSR_IPI_SET: usize = 0x1008;
pub const LOONGSON_CSR_IPI_CLEAR: usize = 0x100c;
pub const LOONGSON_CSR_IPI_SEND: usize = 0x1040;
pub const CSR_IPI_SEND_IP_SHIFT: u32 = 0;
pub const CSR_IPI_SEND_CPU_SHIFT: u32 = 16;
pub const CSR_IPI_SEND_BLOCK: u32 = 1 << 31;
pub const LOONGSON_CSR_MAIL_BUF0: usize = 0x1020;
pub const LOONGSON_CSR_MAIL_SEND: usize = 0x1048;
pub const CSR_MAIL_SEND_BLOCK: u64 = 1u64 << 31;
#[macro_export]
macro_rules! CSR_MAIL_SEND_BOX_LOW { ($box:expr) => { $box << 1 }; }
#[macro_export]
macro_rules! CSR_MAIL_SEND_BOX_HIGH { ($box:expr) => { ($box << 1) + 1 }; }
pub const CSR_MAIL_SEND_BOX_SHIFT: u32 = 2;
pub const CSR_MAIL_SEND_CPU_SHIFT: u32 = 16;
pub const CSR_MAIL_SEND_BUF_SHIFT: u32 = 32;
pub const CSR_MAIL_SEND_H32_MASK: u64 = 0xffff_ffff_0000_0000;

#[inline]
pub unsafe fn drdtime() -> u64 {
    let rid: i32 = 0;
    let val: u64;
    core::arch::asm!(".word (0xc8090118 | ({rid} << 21) | ({val} << 11))", rid = in(reg) rid, val = lateout(reg) val, options(nostack, preserves_flags));
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
