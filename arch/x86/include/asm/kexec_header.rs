/* SPDX-License-Identifier: GPL-2.0 */

#[cfg(CONFIG_X86_32)]
pub const PA_CONTROL_PAGE: usize = 0;
#[cfg(CONFIG_X86_32)]
pub const VA_CONTROL_PAGE: usize = 1;
#[cfg(CONFIG_X86_32)]
pub const PA_PGD: usize = 2;
#[cfg(CONFIG_X86_32)]
pub const PA_SWAP_PAGE: usize = 3;
#[cfg(CONFIG_X86_32)]
pub const PAGES_NR: usize = 4;

#[cfg(not(CONFIG_X86_32))]
pub const KEXEC_DEBUG_EXC_HANDLER_SIZE: usize = 6; /* PUSHI, PUSHI, 2-byte JMP */

#[cfg(CONFIG_X86_64)]
pub const RELOC_KERNEL_PRESERVE_CONTEXT: usize = 1 << 0;
#[cfg(CONFIG_X86_64)]
pub const RELOC_KERNEL_CACHE_INCOHERENT: usize = 1 << 1;

pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;
pub const KEXEC_CONTROL_CODE_MAX_SIZE: usize = 2048;

/* Capture register states after panic, or repair ss/sp for a kernel-mode exception. */
pub unsafe fn crash_setup_regs(newregs: *mut pt_regs, oldregs: *const pt_regs) {
    if !oldregs.is_null() {
        core::ptr::copy_nonoverlapping(oldregs, newregs, 1);
    } else {
        core::arch::asm!("mov {}, rbx", out(reg) (*newregs).bx);
        core::arch::asm!("mov {}, rcx", out(reg) (*newregs).cx);
        core::arch::asm!("mov {}, rdx", out(reg) (*newregs).dx);
        core::arch::asm!("mov {}, rsi", out(reg) (*newregs).si);
        core::arch::asm!("mov {}, rdi", out(reg) (*newregs).di);
        core::arch::asm!("mov {}, rbp", out(reg) (*newregs).bp);
        core::arch::asm!("mov {}, rax", out(reg) (*newregs).ax);
        core::arch::asm!("mov {}, rsp", out(reg) (*newregs).sp);
        #[cfg(CONFIG_X86_64)] {
            core::arch::asm!("mov {}, r8", out(reg) (*newregs).r8);
            core::arch::asm!("mov {}, r9", out(reg) (*newregs).r9);
            core::arch::asm!("mov {}, r10", out(reg) (*newregs).r10);
            core::arch::asm!("mov {}, r11", out(reg) (*newregs).r11);
            core::arch::asm!("mov {}, r12", out(reg) (*newregs).r12);
            core::arch::asm!("mov {}, r13", out(reg) (*newregs).r13);
            core::arch::asm!("mov {}, r14", out(reg) (*newregs).r14);
            core::arch::asm!("mov {}, r15", out(reg) (*newregs).r15);
        }
        core::arch::asm!("mov {}, ss", out(reg) (*newregs).ss);
        core::arch::asm!("mov {}, cs", out(reg) (*newregs).cs);
        #[cfg(CONFIG_X86_32)] {
            core::arch::asm!("mov {}, ds", out(reg) (*newregs).ds);
            core::arch::asm!("mov {}, es", out(reg) (*newregs).es);
        }
        core::arch::asm!("pushfq; pop {}", out(reg) (*newregs).flags);
        (*newregs).ip = _THIS_IP_;
    }
}

/* KEXEC_SOURCE_MEMORY_LIMIT is the maximum page get_free_page can return. */
#[cfg(CONFIG_X86_32)]
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = usize::MAX;
#[cfg(CONFIG_X86_32)]
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = usize::MAX;
#[cfg(CONFIG_X86_32)]
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = TASK_SIZE;
#[cfg(CONFIG_X86_32)]
pub const KEXEC_ARCH: u32 = KEXEC_ARCH_386;

#[cfg(not(CONFIG_X86_32))]
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = MAXMEM - 1;
#[cfg(not(CONFIG_X86_32))]
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = MAXMEM - 1;
#[cfg(not(CONFIG_X86_32))]
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = MAXMEM - 1;
#[cfg(not(CONFIG_X86_32))]
pub const KEXEC_ARCH: u32 = KEXEC_ARCH_X86_64;

#[cfg(not(CONFIG_X86_32))]
extern "C" {
    pub static mut kexec_va_control_page: c_ulong;
    pub static mut kexec_pa_table_page: c_ulong;
    pub static mut kexec_pa_swap_page: c_ulong;
    pub static mut kexec_debug_idt: [gate_desc; 0];
    pub static mut kexec_debug_exc_vectors: [u8; 0];
    pub static mut kexec_debug_8250_port: u16;
    pub static mut kexec_debug_8250_mmio32: c_ulong;
}

#[cfg(CONFIG_X86_32)]
pub type RelocateKernelFn = unsafe extern "C" fn(
    indirection_page: c_ulong,
    control_page: c_ulong,
    start_address: c_ulong,
    has_pae: c_uint,
    preserve_context: c_uint,
);

#[cfg(not(CONFIG_X86_32))]
pub type RelocateKernelFn = unsafe extern "C" fn(
    indirection_page: c_ulong,
    pa_control_page: c_ulong,
    start_address: c_ulong,
    flags: c_uint,
) -> c_ulong;

extern "C" {
    pub static mut relocate_kernel: RelocateKernelFn;
}

#[cfg(CONFIG_X86_32)]
#[repr(C)]
pub struct kimage_arch {
    pub pgd: *mut pgd_t,
    #[cfg(CONFIG_X86_PAE)]
    pub pmd0: *mut pmd_t,
    #[cfg(CONFIG_X86_PAE)]
    pub pmd1: *mut pmd_t,
    pub pte0: *mut pte_t,
    pub pte1: *mut pte_t,
}

#[cfg(not(CONFIG_X86_32))]
#[repr(C)]
pub struct kimage_arch {
    pub pgd: *mut pgd_t,
    pub p4d: *mut p4d_t,
    pub pud: *mut pud_t,
    pub pmd: *mut pmd_t,
    pub pte: *mut pte_t,
}

#[cfg(CONFIG_X86_64)]
#[repr(C)]
pub struct kexec_entry64_regs {
    pub rax: u64, pub rcx: u64, pub rdx: u64, pub rbx: u64,
    pub rsp: u64, pub rbp: u64, pub rsi: u64, pub rdi: u64,
    pub r8: u64, pub r9: u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64,
}

#[cfg(CONFIG_X86_64)]
extern "C" {
    pub fn arch_kexec_post_alloc_pages(vaddr: *mut c_void, pages: c_uint, gfp: gfp_t) -> c_int;
    pub fn arch_kexec_pre_free_pages(vaddr: *mut c_void, pages: c_uint);
    pub fn arch_kexec_protect_crashkres();
    pub fn arch_kexec_unprotect_crashkres();
}

#[cfg(all(CONFIG_X86_64, CONFIG_KEXEC_FILE))]
extern "C" {
    pub fn arch_kexec_apply_relocations_add(
        pi: *mut purgatory_info, section: *mut Elf_Shdr,
        relsec: *const Elf_Shdr, symtab: *const Elf_Shdr,
    ) -> c_int;
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int;
}

extern "C" {
    pub fn kdump_nmi_shootdown_cpus();
}

#[cfg(CONFIG_CRASH_HOTPLUG)]
extern "C" {
    pub fn arch_crash_handle_hotplug_event(image: *mut kimage, arg: *mut c_void);
    pub fn arch_crash_hotplug_support(image: *mut kimage, kexec_flags: c_ulong) -> c_int;
    pub fn arch_crash_get_elfcorehdr_size() -> c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
