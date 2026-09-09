/* SPDX-License-Identifier: GPL-2.0 */

pub const PGM_FLAG_GUEST_FAULT: u32 = 1;
pub const SYSCALL_FLAG_PER_TRAP: u32 = 1;

// Declarations supplied by the Linux kernel and architecture-specific headers.

unsafe extern "C" {
    pub static mut restart_stack: *mut core::ffi::c_void;

    pub fn system_call();
    pub fn pgm_check_handler();
    pub fn ext_int_handler();
    pub fn io_int_handler();
    pub fn mcck_int_handler();
    pub fn restart_int_handler();
    pub fn early_pgm_check_handler();

    pub fn __switch_to_asm(
        prev: *mut task_struct,
        next: *mut task_struct,
    ) -> *mut task_struct;
    pub fn __ret_from_fork(prev: *mut task_struct, regs: *mut pt_regs);
    pub fn __do_pgm_check(regs: *mut pt_regs, flags: core::ffi::c_ulong);
    pub fn __do_syscall(regs: *mut pt_regs, flags: core::ffi::c_ulong);
    pub fn __do_early_pgm_check(regs: *mut pt_regs);

    pub fn do_protection_exception(regs: *mut pt_regs);
    pub fn do_dat_exception(regs: *mut pt_regs);
    pub fn do_secure_storage_access(regs: *mut pt_regs);
    pub fn do_non_secure_storage_access(regs: *mut pt_regs);
    pub fn do_secure_storage_violation(regs: *mut pt_regs);
    pub fn do_report_trap(
        regs: *mut pt_regs,
        si_signo: core::ffi::c_int,
        si_code: core::ffi::c_int,
        str_: *const core::ffi::c_char,
    );
    pub fn kernel_stack_invalid(regs: *mut pt_regs);
    pub fn handle_signal32(
        ksig: *mut ksignal,
        oldset: *mut sigset_t,
        regs: *mut pt_regs,
    );

    pub fn do_io_irq(regs: *mut pt_regs);
    pub fn do_ext_irq(regs: *mut pt_regs);
    pub fn do_restart(arg: *mut core::ffi::c_void);
    pub fn startup_init();
    pub fn die(regs: *mut pt_regs, str_: *const core::ffi::c_char);
    pub fn setup_profiling_timer(multiplier: core::ffi::c_uint) -> core::ffi::c_int;

    pub fn sys_rt_sigreturn() -> core::ffi::c_long;
    pub fn sys_sigreturn() -> core::ffi::c_long;

    pub fn sys_s390_personality(personality: core::ffi::c_uint) -> core::ffi::c_long;
    pub fn sys_s390_runtime_instr(command: core::ffi::c_int, signum: core::ffi::c_int) -> core::ffi::c_long;
    pub fn sys_s390_guarded_storage(command: core::ffi::c_int, cb: *mut gs_cb) -> core::ffi::c_long;
    pub fn sys_s390_pci_mmio_write(
        offset: core::ffi::c_ulong,
        buf: *const core::ffi::c_void,
        size: usize,
    ) -> core::ffi::c_long;
    pub fn sys_s390_pci_mmio_read(
        offset: core::ffi::c_ulong,
        buf: *mut core::ffi::c_void,
        size: usize,
    ) -> core::ffi::c_long;
    pub fn sys_s390_sthyi(
        function_code: core::ffi::c_ulong,
        buffer: *mut core::ffi::c_void,
        return_code: *mut u64,
        flags: core::ffi::c_ulong,
    ) -> core::ffi::c_long;

    pub fn stack_alloc() -> core::ffi::c_ulong;
    pub fn stack_free(stack: core::ffi::c_ulong);

    pub static mut kprobes_insn_page: [core::ffi::c_char; 0];
    pub static mut _samode31: [core::ffi::c_char; 0];
    pub static mut _eamode31: [core::ffi::c_char; 0];
    pub static mut _stext_amode31: [core::ffi::c_char; 0];
    pub static mut _etext_amode31: [core::ffi::c_char; 0];
    pub static mut _start_amode31_ex_table: [exception_table_entry; 0];
    pub static mut _stop_amode31_ex_table: [exception_table_entry; 0];
    pub static mut _start_amode31_refs: [core::ffi::c_long; 0];
    pub static mut _end_amode31_refs: [core::ffi::c_long; 0];
}

// C forward declarations and externally supplied types.
pub enum task_struct {}
pub enum pt_regs {}
pub enum ksignal {}
pub enum sigset_t {}
pub enum gs_cb {}
pub enum exception_table_entry {}
pub enum s390_mmap_arg_struct {}
pub enum fadvise64_64_args {}
pub enum old_sigaction {}

// __amode31_data: __section(".amode31.data")
// __amode31_ref: __section(".amode31.refs")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
