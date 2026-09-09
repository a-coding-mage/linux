/* SPDX-License-Identifier: GPL-2.0 */

/*
 * 64-bit segment selectors
 * These flat segments are in the Xen-private section of every GDT. Since these
 * are also present in the initial GDT, many OSes will be able to avoid
 * installing their own GDT.
 */

pub const FLAT_RING3_CS32: u16 = 0xe023; /* GDT index 260 */
pub const FLAT_RING3_CS64: u16 = 0xe033; /* GDT index 261 */
pub const FLAT_RING3_DS32: u16 = 0xe02b; /* GDT index 262 */
pub const FLAT_RING3_DS64: u16 = 0x0000; /* NULL selector */
pub const FLAT_RING3_SS32: u16 = 0xe02b; /* GDT index 262 */
pub const FLAT_RING3_SS64: u16 = 0xe02b; /* GDT index 262 */

pub const FLAT_KERNEL_DS64: u16 = FLAT_RING3_DS64;
pub const FLAT_KERNEL_DS32: u16 = FLAT_RING3_DS32;
pub const FLAT_KERNEL_DS: u16 = FLAT_KERNEL_DS64;
pub const FLAT_KERNEL_CS64: u16 = FLAT_RING3_CS64;
pub const FLAT_KERNEL_CS32: u16 = FLAT_RING3_CS32;
pub const FLAT_KERNEL_CS: u16 = FLAT_KERNEL_CS64;
pub const FLAT_KERNEL_SS64: u16 = FLAT_RING3_SS64;
pub const FLAT_KERNEL_SS32: u16 = FLAT_RING3_SS32;
pub const FLAT_KERNEL_SS: u16 = FLAT_KERNEL_SS64;

pub const FLAT_USER_DS64: u16 = FLAT_RING3_DS64;
pub const FLAT_USER_DS32: u16 = FLAT_RING3_DS32;
pub const FLAT_USER_DS: u16 = FLAT_USER_DS64;
pub const FLAT_USER_CS64: u16 = FLAT_RING3_CS64;
pub const FLAT_USER_CS32: u16 = FLAT_RING3_CS32;
pub const FLAT_USER_CS: u16 = FLAT_USER_CS64;
pub const FLAT_USER_SS64: u16 = FLAT_RING3_SS64;
pub const FLAT_USER_SS32: u16 = FLAT_RING3_SS32;
pub const FLAT_USER_SS: u16 = FLAT_USER_SS64;

pub const __HYPERVISOR_VIRT_START: u64 = 0xFFFF800000000000;
pub const __HYPERVISOR_VIRT_END: u64 = 0xFFFF880000000000;
pub const __MACH2PHYS_VIRT_START: u64 = 0xFFFF800000000000;
pub const __MACH2PHYS_VIRT_END: u64 = 0xFFFF804000000000;
pub const __MACH2PHYS_SHIFT: u32 = 3;

/*
 * int HYPERVISOR_set_segment_base(unsigned int which, unsigned long base)
 *  @which == SEGBASE_*  ;  @base == 64-bit base address
 * Returns 0 on success.
 */
pub const SEGBASE_FS: u32 = 0;
pub const SEGBASE_GS_USER: u32 = 1;
pub const SEGBASE_GS_KERNEL: u32 = 2;
pub const SEGBASE_GS_USER_SEL: u32 = 3; /* Set user %gs specified in base[15:0] */

/*
 * int HYPERVISOR_iret(void)
 * All arguments are on the kernel stack, in the following format.
 * Never returns if successful. Current kernel context is lost.
 * The saved CS is mapped as follows:
 *   RING0 -> RING3 kernel mode.
 *   RING1 -> RING3 kernel mode.
 *   RING2 -> RING3 kernel mode.
 *   RING3 -> RING3 user mode.
 * However RING0 indicates that the guest kernel should return to itself
 * directly with
 *      orb   $3,1*8(%rsp)
 *      iretq
 * If flags contains VGCF_in_syscall:
 *   Restore RAX, RIP, RFLAGS, RSP.
 *   Discard R11, RCX, CS, SS.
 * Otherwise:
 *   Restore RAX, R11, RCX, CS:RIP, RFLAGS, SS:RSP.
 * All other registers are saved on hypercall entry and restored to user.
 */
/* Guest exited in SYSCALL context? Return to guest with SYSRET? */
pub const _VGCF_in_syscall: u32 = 8;
pub const VGCF_in_syscall: u32 = 1 << _VGCF_in_syscall;
pub const VGCF_IN_SYSCALL: u32 = VGCF_in_syscall;

#[repr(C)]
pub struct iret_context {
    /* Top of stack (%rsp at point of hypercall). */
    pub rax: u64,
    pub r11: u64,
    pub rcx: u64,
    pub flags: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
    /* Bottom of iret stack frame. */
}

#[repr(C)]
pub union cpu_user_regs_bp {
    pub rbp: u64,
    pub ebp: u64,
    pub _ebp: u32,
}

#[repr(C)]
pub union cpu_user_regs_bx {
    pub rbx: u64,
    pub ebx: u64,
    pub _ebx: u32,
}

#[repr(C)]
pub union cpu_user_regs_ax {
    pub rax: u64,
    pub eax: u64,
    pub _eax: u32,
}

#[repr(C)]
pub union cpu_user_regs_cx {
    pub rcx: u64,
    pub ecx: u64,
    pub _ecx: u32,
}

#[repr(C)]
pub union cpu_user_regs_dx {
    pub rdx: u64,
    pub edx: u64,
    pub _edx: u32,
}

#[repr(C)]
pub union cpu_user_regs_si {
    pub rsi: u64,
    pub esi: u64,
    pub _esi: u32,
}

#[repr(C)]
pub union cpu_user_regs_di {
    pub rdi: u64,
    pub edi: u64,
    pub _edi: u32,
}

#[repr(C)]
pub union cpu_user_regs_ip {
    pub rip: u64,
    pub eip: u64,
    pub _eip: u32,
}

#[repr(C)]
pub union cpu_user_regs_flags {
    pub rflags: u64,
    pub eflags: u64,
    pub _eflags: u32,
}

#[repr(C)]
pub union cpu_user_regs_sp {
    pub rsp: u64,
    pub esp: u64,
    pub _esp: u32,
}

#[repr(C)]
pub struct cpu_user_regs {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub bp: cpu_user_regs_bp,
    pub bx: cpu_user_regs_bx,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub ax: cpu_user_regs_ax,
    pub cx: cpu_user_regs_cx,
    pub dx: cpu_user_regs_dx,
    pub si: cpu_user_regs_si,
    pub di: cpu_user_regs_di,
    pub error_code: u32, /* private */
    pub entry_vector: u32, /* private */
    pub ip: cpu_user_regs_ip,
    pub cs: u16,
    pub _pad0: [u16; 1],
    pub saved_upcall_mask: u8,
    pub _pad1: [u8; 3],
    pub flags: cpu_user_regs_flags, /* rflags.IF == !saved_upcall_mask */
    pub sp: cpu_user_regs_sp,
    pub ss: u16,
    pub _pad2: [u16; 3],
    pub es: u16,
    pub _pad3: [u16; 3],
    pub ds: u16,
    pub _pad4: [u16; 3],
    pub fs: u16,
    pub _pad5: [u16; 3], /* Non-zero => takes precedence over fs_base. */
    pub gs: u16,
    pub _pad6: [u16; 3], /* Non-zero => takes precedence over gs_base_usr. */
}

/* DEFINE_GUEST_HANDLE_STRUCT(cpu_user_regs); supplied by the surrounding Xen interface. */

#[inline]
pub const fn xen_pfn_to_cr3(pfn: u64) -> u64 {
    pfn << 12
}

#[inline]
pub const fn xen_cr3_to_pfn(cr3: u64) -> u64 {
    cr3 >> 12
}

#[repr(C)]
pub struct arch_vcpu_info {
    pub cr2: u64,
    pub pad: u64, /* sizeof(vcpu_info_t) == 64 */
}

pub type xen_callback_t = u64;

#[inline]
pub const fn XEN_CALLBACK(__cs: u64, __rip: u64) -> u64 {
    __rip
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
