/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	Just a place holder.
 */

// Dependency declarations supplied by the corresponding Linux/Sparc headers
// are intentionally not reproduced here.

use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    pub static mut reboot_command: [c_char; 0];
}

// CONFIG_SPARC32
#[cfg(CONFIG_SPARC32)]
extern "C" {
    /* The CPU that was used for booting
     * Only sun4d + leon may have boot_cpu_id != 0
     */
    pub static mut boot_cpu_id: u8;

    pub static mut serial_console: c_int;

    /* from irq_32.c */
    pub static mut fdc_status: *mut u8;
    pub static mut pdma_vaddr: *mut c_char;
    pub static mut pdma_size: c_ulong;
    pub static mut doing_pdma: c_int;

    /* This is software state */
    pub static mut pdma_base: *mut c_char;
    pub static mut pdma_areasize: c_ulong;

    pub static mut cmdline_memory_size: c_ulong;

    pub fn sparc_floppy_request_irq(irq: u32, irq_handler: irq_handler_t) -> c_int;

    /* setup_32.c */
    /* devices.c */
    pub fn device_scan();

    /* unaligned_32.c */
    pub fn safe_compute_effective_address(regs: *mut pt_regs, address: u32) -> c_ulong;
}

#[cfg(CONFIG_SPARC32)]
pub unsafe fn con_is_present() -> c_int {
    if serial_console != 0 { 0 } else { 1 }
}

// CONFIG_SPARC64
#[cfg(CONFIG_SPARC64)]
extern "C" {
    pub fn start_early_boot();

    /* unaligned_64.c */
    pub fn handle_ldf_stq(insn: u32, regs: *mut pt_regs) -> c_int;
    pub fn handle_ld_nf(insn: u32, regs: *mut pt_regs);

    /* init_64.c */
    pub static mut dcpage_flushes: atomic_t;
    pub static mut dcpage_flushes_xcall: atomic_t;

    pub static mut sysctl_tsb_ratio: c_int;

    // CONFIG_SERIAL_SUNHV
    #[cfg(CONFIG_SERIAL_SUNHV)]
    pub fn sunhv_migrate_hvcons_irq(cpu: c_int);
}

extern "C" {
    pub fn sun_do_break();
    pub static mut stop_a_enabled: c_int;
    pub static mut scons_pwroff: c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
