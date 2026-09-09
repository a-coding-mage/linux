/* SPDX-License-Identifier: GPL-2.0 */

/* IA32 compatible user structures for ptrace.
 * These should be used for 32bit coredumps too. */

#[repr(C)]
pub struct user_i387_ia32_struct {
    pub cwd: u32,
    pub swd: u32,
    pub twd: u32,
    pub fip: u32,
    pub fcs: u32,
    pub foo: u32,
    pub fos: u32,
    pub st_space: [u32; 20], /* 8*10 bytes for each FP-reg = 80 bytes */
}

/* FSAVE frame with extensions */
#[repr(C)]
pub struct user32_fxsr_struct {
    pub cwd: u16,
    pub swd: u16,
    pub twd: u16, /* not compatible to 64bit twd */
    pub fop: u16,
    pub fip: i32,
    pub fcs: i32,
    pub foo: i32,
    pub fos: i32,
    pub mxcsr: i32,
    pub reserved: i32,
    pub st_space: [i32; 32], /* 8*16 bytes for each FP-reg = 128 bytes */
    pub xmm_space: [i32; 32], /* 8*16 bytes for each XMM-reg = 128 bytes */
    pub padding: [i32; 56],
}

#[repr(C)]
pub struct user_regs_struct32 {
    pub ebx: __u32,
    pub ecx: __u32,
    pub edx: __u32,
    pub esi: __u32,
    pub edi: __u32,
    pub ebp: __u32,
    pub eax: __u32,
    pub ds: u16,
    pub __ds: u16,
    pub es: u16,
    pub __es: u16,
    pub fs: u16,
    pub __fs: u16,
    pub gs: u16,
    pub __gs: u16,
    pub orig_eax: __u32,
    pub eip: __u32,
    pub cs: u16,
    pub __cs: u16,
    pub eflags: __u32,
    pub esp: __u32,
    pub ss: u16,
    pub __ss: u16,
}

#[repr(C)]
pub struct user32 {
    pub regs: user_regs_struct32, /* Where the registers are actually stored */
    pub u_fpvalid: i32, /* True if math co-processor being used. */
    /* for this mess. Not yet used. */
    pub i387: user_i387_ia32_struct, /* Math Co-processor registers. */
    /* The rest of this junk is to help gdb figure out what goes where */
    pub u_tsize: __u32, /* Text segment size (pages). */
    pub u_dsize: __u32, /* Data segment size (pages). */
    pub u_ssize: __u32, /* Stack segment size (pages). */
    pub start_code: __u32, /* Starting virtual address of text. */
    pub start_stack: __u32, /* Starting virtual address of stack area.
                               This is actually the bottom of the stack,
                               the top of the stack is always found in the
                               esp register. */
    pub signal: __u32, /* Signal that caused the core dump. */
    pub reserved: i32, /* No __u32er used */
    pub u_ar0: __u32, /* Used by gdb to help find the values for */
    /* the registers. */
    pub u_fpstate: __u32, /* Math Co-processor pointer. */
    pub magic: __u32, /* To uniquely identify a core file */
    pub u_comm: [i8; 32], /* User command that was responsible */
    pub u_debugreg: [i32; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
