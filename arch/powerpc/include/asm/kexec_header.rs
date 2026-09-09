/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard and __KERNEL__ / __ASSEMBLER__ conditions omitted. */
/* CONFIG_PPC_85xx || CONFIG_44x: use the first 2 GiB memory mapping. */
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = 2 * 1024 * 1024 * 1024usize - 1;
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = 2 * 1024 * 1024 * 1024usize - 1;
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = 2 * 1024 * 1024 * 1024usize - 1;

/* Other architectures use (-1UL) for the source, destination, and (on
 * powerpc64) control memory limits; on 32-bit powerpc the latter is TASK_SIZE. */
pub const KEXEC_SOURCE_MEMORY_LIMIT_OTHER: usize = usize::MAX;
pub const KEXEC_DESTINATION_MEMORY_LIMIT_OTHER: usize = usize::MAX;
pub const KEXEC_CONTROL_MEMORY_LIMIT_POWERPC64: usize = usize::MAX;
/* TASK_SIZE is supplied by the target environment for 32-bit powerpc. */
pub const KEXEC_CONTROL_MEMORY_LIMIT_POWERPC: usize = TASK_SIZE;

pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;

/* The native architecture. */
pub const KEXEC_ARCH_PPC64_NATIVE: u32 = KEXEC_ARCH_PPC64;
pub const KEXEC_ARCH_PPC_NATIVE: u32 = KEXEC_ARCH_PPC;

pub const KEXEC_STATE_NONE: u32 = 0;
pub const KEXEC_STATE_IRQS_OFF: u32 = 1;
pub const KEXEC_STATE_REAL_MODE: u32 = 2;

pub type CrashShutdownT = unsafe extern "C" fn();

pub enum Kimage {}
pub enum PtRegs {}
pub enum CrashMem {}
pub enum KexecFileOps {}
pub enum KexecBuf {}
pub enum Elf64Ehdr {}

#[cfg(CONFIG_KEXEC_CORE)]
extern "C" {
    pub fn kexec_smp_wait();
    pub fn default_machine_kexec(image: *mut Kimage);
    pub fn relocate_new_kernel(
        indirection_page: usize,
        reboot_code_buffer: usize,
        start_address: usize,
    ) -> !;
    pub fn kexec_copy_flush(image: *mut Kimage);
}

#[cfg(any(CONFIG_KEXEC_FILE, CONFIG_CRASH_DUMP))]
pub struct KimageArch {
    pub exclude_ranges: *mut CrashMem,
    pub backup_start: usize,
    pub backup_buf: *mut core::ffi::c_void,
    pub fdt: *mut core::ffi::c_void,
}

#[cfg(CONFIG_KEXEC_FILE)]
extern "C" {
    pub static kexec_elf64_ops: KexecFileOps;
    pub fn setup_kdump_cmdline(image: *mut Kimage, cmdline: *mut i8, cmdline_len: usize) -> *mut i8;
    pub fn setup_purgatory(
        image: *mut Kimage,
        slave_code: *const core::ffi::c_void,
        fdt: *const core::ffi::c_void,
        kernel_load_addr: usize,
        fdt_load_addr: usize,
    ) -> i32;
}

#[cfg(all(CONFIG_KEXEC_FILE, CONFIG_PPC64))]
extern "C" {
    pub fn arch_kexec_kernel_image_probe(image: *mut Kimage, buf: *mut core::ffi::c_void, buf_len: usize) -> i32;
    pub fn arch_kimage_file_post_load_cleanup(image: *mut Kimage) -> i32;
    pub fn arch_check_excluded_range(image: *mut Kimage, start: usize, end: usize) -> i32;
    pub fn load_crashdump_segments_ppc64(image: *mut Kimage, kbuf: *mut KexecBuf) -> i32;
    pub fn setup_purgatory_ppc64(image: *mut Kimage, slave_code: *const core::ffi::c_void, fdt: *const core::ffi::c_void, kernel_load_addr: usize, fdt_load_addr: usize) -> i32;
    pub fn kexec_extra_fdt_size_ppc64(image: *mut Kimage, rmem: *mut CrashMem) -> u32;
    pub fn setup_new_fdt_ppc64(image: *const Kimage, fdt: *mut core::ffi::c_void, rmem: *mut CrashMem) -> i32;
}

#[cfg(CONFIG_CRASH_RESERVE)]
extern "C" {
    pub fn overlaps_crashkernel(start: usize, size: usize) -> i32;
    pub fn arch_reserve_crashkernel();
    pub fn kdump_cma_reserve();
}

#[cfg(not(CONFIG_CRASH_RESERVE))]
pub unsafe fn arch_reserve_crashkernel() {}
#[cfg(not(CONFIG_CRASH_RESERVE))]
pub unsafe fn overlaps_crashkernel(_start: usize, _size: usize) -> i32 { 0 }
#[cfg(not(CONFIG_CRASH_RESERVE))]
pub unsafe fn kdump_cma_reserve() {}

#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn crash_setup_regs(newregs: *mut PtRegs, oldregs: *mut PtRegs) {
    if !oldregs.is_null() {
        memcpy(newregs, oldregs, core::mem::size_of::<PtRegs>());
    } else {
        ppc_save_regs(newregs);
    }
}

#[cfg(CONFIG_CRASH_DUMP)]
extern "C" {
    pub static mut crashing_cpu: i32;
    pub fn crash_send_ipi(callback: unsafe extern "C" fn(*mut PtRegs));
    pub fn crash_ipi_callback(regs: *mut PtRegs);
    pub static mut crash_wake_offline: i32;
    pub fn crash_shutdown_register(handler: CrashShutdownT) -> i32;
    pub fn crash_shutdown_unregister(handler: CrashShutdownT) -> i32;
    pub fn default_machine_crash_shutdown(regs: *mut PtRegs);
    pub fn crash_kexec_prepare();
    pub fn crash_kexec_secondary(regs: *mut PtRegs);
    pub fn sync_backup_region_phdr(image: *mut Kimage, ehdr: *mut Elf64Ehdr, phdr_to_kimage: bool);
    pub fn is_kdump_kernel() -> bool;
}

#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn kdump_in_progress() -> bool { crashing_cpu >= 0 }

#[cfg(not(CONFIG_CRASH_DUMP))]
pub unsafe fn crash_kexec_secondary(_regs: *mut PtRegs) {}
#[cfg(not(CONFIG_CRASH_DUMP))]
pub unsafe fn crash_shutdown_register(_handler: CrashShutdownT) -> i32 { 0 }
#[cfg(not(CONFIG_CRASH_DUMP))]
pub unsafe fn crash_shutdown_unregister(_handler: CrashShutdownT) -> i32 { 0 }
#[cfg(not(CONFIG_CRASH_DUMP))]
pub unsafe fn kdump_in_progress() -> bool { false }
#[cfg(not(CONFIG_CRASH_DUMP))]
pub unsafe fn crash_ipi_callback(_regs: *mut PtRegs) {}
#[cfg(not(CONFIG_CRASH_DUMP))]
pub unsafe fn crash_send_ipi(_callback: unsafe extern "C" fn(*mut PtRegs)) {}

#[cfg(any(CONFIG_KEXEC_FILE, CONFIG_CRASH_DUMP))]
extern "C" { pub fn update_cpus_node(fdt: *mut core::ffi::c_void) -> i32; }

/* CONFIG_PPC_BOOK3S_64 includes asm/book3s/64/kexec.h. */

pub unsafe fn reset_sprs() {}

extern "C" {
    fn memcpy(dest: *mut PtRegs, src: *const PtRegs, n: usize);
    fn ppc_save_regs(regs: *mut PtRegs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
