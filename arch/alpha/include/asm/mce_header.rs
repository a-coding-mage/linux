/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This is the logout header that should be common to all platforms
 * (assuming they are running OSF/1 PALcode, I guess).
 */
#[repr(C)]
pub struct el_common {
    pub size: u32,             /* size in bytes of logout area */
    /* C bit-fields sbz1:30, err2:1, retry:1, represented in declaration order. */
    pub flags: u32,
    pub proc_offset: u32,      /* processor-specific offset */
    pub sys_offset: u32,       /* system-specific offset */
    pub code: u32,             /* machine check code */
    pub frame_rev: u32,        /* frame revision */
}

/* Machine Check Frame for uncorrectable errors (Large format)
 *      --- This is used to log uncorrectable errors such as
 *          double bit ECC errors.
 *      --- These errors are detected by both processor and systems.
 */
#[repr(C)]
pub struct el_common_EV5_uncorrectable_mcheck {
    pub shadow: [u64; 8],        /* Shadow reg. 8-14, 25           */
    pub paltemp: [u64; 24],      /* PAL TEMP REGS.                 */
    pub exc_addr: u64,           /* Address of excepting instruction*/
    pub exc_sum: u64,            /* Summary of arithmetic traps.   */
    pub exc_mask: u64,           /* Exception mask (from exc_sum). */
    pub pal_base: u64,           /* Base address for PALcode.      */
    pub isr: u64,                /* Interrupt Status Reg.          */
    pub icsr: u64,               /* CURRENT SETUP OF EV5 IBOX      */
    pub ic_perr_stat: u64,       /* I-CACHE Reg. <11> set Data parity
                                     <12> set TAG parity*/
    pub dc_perr_stat: u64,       /* D-CACHE error Reg. Bits set to 1:
                                     <2> Data error in bank 0
                                     <3> Data error in bank 1
                                     <4> Tag error in bank 0
                                     <5> Tag error in bank 1 */
    pub va: u64,                 /* Effective VA of fault or miss. */
    pub mm_stat: u64,            /* Holds the reason for D-stream
                                     fault or D-cache parity errors */
    pub sc_addr: u64,            /* Address that was being accessed
                                     when EV5 detected Secondary cache
                                     failure.                 */
    pub sc_stat: u64,            /* Helps determine if the error was
                                     TAG/Data parity(Secondary Cache)*/
    pub bc_tag_addr: u64,        /* Contents of EV5 BC_TAG_ADDR    */
    pub ei_addr: u64,            /* Physical address of any transfer
                                     that is logged in EV5 EI_STAT */
    pub fill_syndrome: u64,      /* For correcting ECC errors.     */
    pub ei_stat: u64,            /* Helps identify reason of any
                                     processor uncorrectable error
                                     at its external interface.     */
    pub ld_lock: u64,            /* Contents of EV5 LD_LOCK register*/
}

#[repr(C)]
pub struct el_common_EV6_mcheck {
    pub FrameSize: u32,          /* Bytes, including this field */
    pub FrameFlags: u32,         /* <31> = Retry, <30> = Second Error */
    pub CpuOffset: u32,          /* Offset to CPU-specific info */
    pub SystemOffset: u32,       /* Offset to system-specific info */
    pub MCHK_Code: u32,
    pub MCHK_Frame_Rev: u32,
    pub I_STAT: u64,             /* EV6 Internal Processor Registers */
    pub DC_STAT: u64,            /* (See the 21264 Spec) */
    pub C_ADDR: u64,
    pub DC1_SYNDROME: u64,
    pub DC0_SYNDROME: u64,
    pub C_STAT: u64,
    pub C_STS: u64,
    pub MM_STAT: u64,
    pub EXC_ADDR: u64,
    pub IER_CM: u64,
    pub ISUM: u64,
    pub RESERVED0: u64,
    pub PAL_BASE: u64,
    pub I_CTL: u64,
    pub PCTX: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
