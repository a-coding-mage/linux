/*
 * On mips we have two 32bit ABIs - o32 and n32.  The latter
 * has bigger registers, so we use it for compat_elf_regset_t.
 * The former uses o32_elf_prstatus and PRSTATUS_SIZE/SET_PR_FPVALID
 * are used to choose the size and location of ->pr_fpvalid of
 * the layout actually used.
 */
pub type compat_elf_gregset_t = elf_gregset_t;

#[repr(C)]
pub struct o32_elf_prstatus {
    pub common: compat_elf_prstatus_common,
    pub pr_reg: [core::ffi::c_uint; ELF_NGREG],
    pub pr_fpvalid: compat_int_t,
}

/* The referenced types, constant, and thread-flag test are supplied by the
 * surrounding MIPS compatibility environment. */
#[inline]
pub unsafe fn prstatus_size() -> usize {
    if !test_thread_flag(TIF_32BIT_REGS) {
        core::mem::size_of::<compat_elf_prstatus>()
    } else {
        core::mem::size_of::<o32_elf_prstatus>()
    }
}

#[inline]
pub unsafe fn set_pr_fpvalid(s: *mut compat_elf_prstatus) {
    if !test_thread_flag(TIF_32BIT_REGS) {
        (*s).pr_fpvalid = 1;
    } else {
        (*(s as *mut o32_elf_prstatus)).pr_fpvalid = 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
