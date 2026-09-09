/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency intent: the C header includes <asm/processor-flags.h>. */

/*
 * I'm guessing at the VIF/VIP flag usage, but hope that this is how
 * the Pentium uses them. Linux will return from vm86 mode when both
 * VIF and VIP is set.
 *
 * On a Pentium, we could probably optimize the virtual flags directly
 * in the eflags register instead of doing it "by hand" in vflags...
 *
 * Linus
 */

pub const BIOSSEG: u32 = 0x0f000;

pub const CPU_086: u32 = 0;
pub const CPU_186: u32 = 1;
pub const CPU_286: u32 = 2;
pub const CPU_386: u32 = 3;
pub const CPU_486: u32 = 4;
pub const CPU_586: u32 = 5;

/* Return values for the 'vm86()' system call */
#[inline]
pub const fn VM86_TYPE(retval: u32) -> u32 { retval & 0xff }
#[inline]
pub const fn VM86_ARG(retval: u32) -> u32 { retval >> 8 }

pub const VM86_SIGNAL: u32 = 0; /* return due to signal */
pub const VM86_UNKNOWN: u32 = 1; /* unhandled GP fault
                                   - IO-instruction or similar */
pub const VM86_INTx: u32 = 2; /* int3/int x instruction (ARG = x) */
pub const VM86_STI: u32 = 3; /* sti/popf/iret instruction enabled
                                virtual interrupts */

/* Additional return values when invoking new vm86() */
pub const VM86_PICRETURN: u32 = 4; /* return due to pending PIC request */
pub const VM86_TRAP: u32 = 6; /* return due to DOS-debugger request */

/* function codes when invoking new vm86() */
pub const VM86_PLUS_INSTALL_CHECK: u32 = 0;
pub const VM86_ENTER: u32 = 1;
pub const VM86_ENTER_NO_BYPASS: u32 = 2;
pub const VM86_REQUEST_IRQ: u32 = 3;
pub const VM86_FREE_IRQ: u32 = 4;
pub const VM86_GET_IRQ_BITS: u32 = 5;
pub const VM86_GET_AND_RESET_IRQ: u32 = 6;

/*
 * This is the stack-layout seen by the user space program when we have
 * done a translation of "SAVE_ALL" from vm86 mode. The real kernel layout
 * is 'kernel_vm86_regs' (see below).
 */
#[repr(C)]
pub struct vm86_regs {
    /* normal regs, with special meaning for the segment descriptors.. */
    pub ebx: i32,
    pub ecx: i32,
    pub edx: i32,
    pub esi: i32,
    pub edi: i32,
    pub ebp: i32,
    pub eax: i32,
    pub __null_ds: i32,
    pub __null_es: i32,
    pub __null_fs: i32,
    pub __null_gs: i32,
    pub orig_eax: i32,
    pub eip: i32,
    pub cs: u16,
    pub __csh: u16,
    pub eflags: i32,
    pub esp: i32,
    pub ss: u16,
    pub __ssh: u16,
    /* these are specific to v86 mode: */
    pub es: u16,
    pub __esh: u16,
    pub ds: u16,
    pub __dsh: u16,
    pub fs: u16,
    pub __fsh: u16,
    pub gs: u16,
    pub __gsh: u16,
}

#[repr(C)]
pub struct revectored_struct {
    pub __map: [u32; 8], /* 256 bits */
}

#[repr(C)]
pub struct vm86_struct {
    pub regs: vm86_regs,
    pub flags: u32,
    pub screen_bitmap: u32, /* unused, preserved by vm86() */
    pub cpu_type: u32,
    pub int_revectored: revectored_struct,
    pub int21_revectored: revectored_struct,
}

/* flags masks */
pub const VM86_SCREEN_BITMAP: u32 = 0x0001; /* no longer supported */

#[repr(C)]
pub struct vm86plus_info_struct {
    /* C unsigned-long bitfields; represented by their containing word. */
    pub bitfields: u32,
    pub vm86dbg_intxxtab: [u8; 32], /* for debugger */
}

/* Bit positions in vm86plus_info_struct::bitfields. */
pub const VM86PLUS_FORCE_RETURN_FOR_PIC: u32 = 1 << 0;
pub const VM86PLUS_VM86DBG_ACTIVE: u32 = 1 << 1;
pub const VM86PLUS_VM86DBG_TFPENDIG: u32 = 1 << 2;
pub const VM86PLUS_UNUSED_MASK: u32 = 0x0fffffff << 3;
pub const VM86PLUS_IS_VM86PUS: u32 = 1 << 31;

#[repr(C)]
pub struct vm86plus_struct {
    pub regs: vm86_regs,
    pub flags: u32,
    pub screen_bitmap: u32,
    pub cpu_type: u32,
    pub int_revectored: revectored_struct,
    pub int21_revectored: revectored_struct,
    pub vm86plus: vm86plus_info_struct,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
