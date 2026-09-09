// Translated from x86/include/asm/elfcore-compat.h.
// Dependency: <asm/user32.h> and the symbols/types it provides are supplied
// by other translated headers.

/*
 * On amd64 we have two 32bit ABIs - i386 and x32.  The latter
 * has bigger registers, so we use it for compat_elf_regset_t.
 * The former uses i386_elf_prstatus and PRSTATUS_SIZE/SET_PR_FPVALID
 * are used to choose the size and location of ->pr_fpvalid of
 * the layout actually used.
 */
pub type compat_elf_gregset_t = user_regs_struct;

#[repr(C)]
pub struct i386_elf_prstatus {
    pub common: compat_elf_prstatus_common,
    pub pr_reg: user_regs_struct32,
    pub pr_fpvalid: compat_int_t,
}

#[inline]
pub unsafe fn PRSTATUS_SIZE() -> usize {
    if user_64bit_mode(task_pt_regs(current)) {
        core::mem::size_of::<compat_elf_prstatus>()
    } else {
        core::mem::size_of::<i386_elf_prstatus>()
    }
}

#[inline]
pub unsafe fn SET_PR_FPVALID(s: *mut compat_elf_prstatus) {
    if user_64bit_mode(task_pt_regs(current)) {
        (*s).pr_fpvalid = 1;
    } else {
        (*(s as *mut i386_elf_prstatus)).pr_fpvalid = 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
