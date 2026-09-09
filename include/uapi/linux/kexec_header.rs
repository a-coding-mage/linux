/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* kexec system call - It loads the new kernel to boot into.
 * kexec does not sync, or unmount filesystems so if you need
 * that to happen you need to do that yourself.
 */

/* Dependency corresponding to <linux/types.h>. */

/* kexec flags for different usage scenarios */
pub const KEXEC_ON_CRASH: u32 = 0x0000_0001;
pub const KEXEC_PRESERVE_CONTEXT: u32 = 0x0000_0002;
pub const KEXEC_UPDATE_ELFCOREHDR: u32 = 0x0000_0004;
pub const KEXEC_CRASH_HOTPLUG_SUPPORT: u32 = 0x0000_0008;
pub const KEXEC_ARCH_MASK: u32 = 0xffff_0000;

/*
 * Kexec file load interface flags.
 * KEXEC_FILE_UNLOAD : Unload already loaded kexec/kdump image.
 * KEXEC_FILE_ON_CRASH : Load/unload operation belongs to kdump image.
 * KEXEC_FILE_NO_INITRAMFS : No initramfs is being loaded. Ignore the initrd
 *                           fd field.
 * KEXEC_FILE_FORCE_DTB : Force carrying over the current boot's DTB to the new
 *                        kernel on x86. This is already the default behavior on
 *                        some other architectures, like ARM64 and PowerPC.
 */
pub const KEXEC_FILE_UNLOAD: u32 = 0x0000_0001;
pub const KEXEC_FILE_ON_CRASH: u32 = 0x0000_0002;
pub const KEXEC_FILE_NO_INITRAMFS: u32 = 0x0000_0004;
pub const KEXEC_FILE_DEBUG: u32 = 0x0000_0008;
pub const KEXEC_FILE_NO_CMA: u32 = 0x0000_0010;
pub const KEXEC_FILE_FORCE_DTB: u32 = 0x0000_0020;

/* These values match the ELF architecture values.
 * Unless there is a good reason that should continue to be the case.
 */
pub const KEXEC_ARCH_DEFAULT: u32 = 0 << 16;
pub const KEXEC_ARCH_386: u32 = 3 << 16;
pub const KEXEC_ARCH_68K: u32 = 4 << 16;
pub const KEXEC_ARCH_PARISC: u32 = 15 << 16;
pub const KEXEC_ARCH_X86_64: u32 = 62 << 16;
pub const KEXEC_ARCH_PPC: u32 = 20 << 16;
pub const KEXEC_ARCH_PPC64: u32 = 21 << 16;
pub const KEXEC_ARCH_IA_64: u32 = 50 << 16;
pub const KEXEC_ARCH_ARM: u32 = 40 << 16;
pub const KEXEC_ARCH_S390: u32 = 22 << 16;
pub const KEXEC_ARCH_SH: u32 = 42 << 16;
pub const KEXEC_ARCH_MIPS_LE: u32 = 10 << 16;
pub const KEXEC_ARCH_MIPS: u32 = 8 << 16;
pub const KEXEC_ARCH_AARCH64: u32 = 183 << 16;
pub const KEXEC_ARCH_RISCV: u32 = 243 << 16;
pub const KEXEC_ARCH_LOONGARCH: u32 = 258 << 16;

/* The artificial cap on the number of segments passed to kexec_load. */
pub const KEXEC_SEGMENT_MAX: usize = 16;

/*
 * This structure is used to hold the arguments that are used when
 * loading kernel binaries.
 *
 * The C __kernel_size_t type is represented by usize here.
 */
#[repr(C)]
pub struct kexec_segment {
    pub buf: *const core::ffi::c_void,
    pub bufsz: usize,
    pub mem: *const core::ffi::c_void,
    pub memsz: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
