/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/// struct extcontext - extended context header structure
/// @magic: magic value identifying the type of extended context
/// @size: the size in bytes of the enclosing structure
///
/// Extended context structures provide context which does not fit within
/// struct sigcontext. They are placed sequentially in memory at the end of
/// struct ucontext and struct sigframe, with each extended context structure
/// beginning with a header defined by this struct. The type of context
/// represented is indicated by the magic field. Userland may check each
/// extended context structure against magic values that it recognises. The
/// size field allows any unrecognised context to be skipped, allowing for
/// future expansion. The end of the extended context data is indicated by the
/// magic value END_EXTCONTEXT_MAGIC.
#[repr(C)]
pub struct extcontext {
    pub magic: u32,
    pub size: u32,
}

/// struct msa_extcontext - MSA extended context structure
/// @ext: the extended context header, with magic == MSA_EXTCONTEXT_MAGIC
/// @wr: the most significant 64 bits of each MSA vector register
/// @csr: the value of the MSA control & status register
///
/// If MSA context is live for a task at the time a signal is delivered to it,
/// this structure will hold the MSA context of the task as it was prior to the
/// signal delivery.
#[repr(C)]
pub struct msa_extcontext {
    pub ext: extcontext,
    pub wr: [u64; 32],
    pub csr: u32,
}

pub const MSA_EXTCONTEXT_MAGIC: u32 = 0x784d5341; /* xMSA */
pub const END_EXTCONTEXT_MAGIC: u32 = 0x78454e44; /* xEND */

/// struct ucontext - user context structure
/// @uc_flags:
/// @uc_link:
/// @uc_stack:
/// @uc_mcontext: holds basic processor state
/// @uc_sigmask:
/// @uc_extcontext: holds extended processor state
#[repr(C)]
pub struct ucontext {
    /* Historic fields matching asm-generic */
    pub uc_flags: usize,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: sigcontext,
    pub uc_sigmask: sigset_t,

    /* Extended context structures may follow ucontext */
    pub uc_extcontext: [u64; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
