/* SPDX-License-Identifier: GPL-2.0 */
// Translated from proto.h. Linux kernel dependencies are supplied externally.

pub type vucp = *mut kernel::ffi::c_uchar;
pub type vusp = *mut kernel::ffi::c_ushort;
pub type vip = *mut kernel::ffi::c_int;
pub type vuip = *mut kernel::ffi::c_uint;
pub type vulp = *mut kernel::ffi::c_ulong;

pub enum pt_regs {}
pub enum task_struct {}
pub enum pci_dev {}
pub enum pci_controller {}
pub enum pci_bus {}
pub enum pci_ops {}
pub enum _alpha_agp_info {}
pub enum io7 {}
pub enum sigcontext {}
pub enum rt_sigframe {}
pub enum allregs {}
pub enum screen_info {}

extern "C" {
    pub static mut cia_pci_ops: pci_ops;
    pub fn cia_init_pci();
    pub fn cia_init_arch();
    pub fn pyxis_init_arch();
    pub fn cia_kill_arch(arg: kernel::ffi::c_int);
    pub fn cia_machine_check(vector: kernel::ffi::c_ulong, la_ptr: kernel::ffi::c_ulong);
    pub fn cia_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub static mut irongate_pci_ops: pci_ops;
    pub fn irongate_pci_clr_err() -> kernel::ffi::c_int;
    pub fn irongate_init_arch();

    pub static mut marvel_pci_ops: pci_ops;
    pub fn marvel_init_arch();
    pub fn marvel_kill_arch(arg: kernel::ffi::c_int);
    pub fn marvel_machine_check(arg1: kernel::ffi::c_ulong, arg2: kernel::ffi::c_ulong);
    pub fn marvel_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);
    pub fn marvel_agp_info() -> *mut _alpha_agp_info;
    pub fn marvel_find_io7(pe: kernel::ffi::c_int) -> *mut io7;
    pub fn marvel_next_io7(prev: *mut io7) -> *mut io7;
    pub fn io7_clear_errors(io7: *mut io7);

    pub static mut mcpcia_pci_ops: pci_ops;
    pub fn mcpcia_init_arch();
    pub fn mcpcia_init_hoses();
    pub fn mcpcia_machine_check(vector: kernel::ffi::c_ulong, la_ptr: kernel::ffi::c_ulong);
    pub fn mcpcia_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub static mut polaris_pci_ops: pci_ops;
    pub fn polaris_read_config_dword(dev: *mut pci_dev, where_: kernel::ffi::c_int, value: *mut u32) -> kernel::ffi::c_int;
    pub fn polaris_write_config_dword(dev: *mut pci_dev, where_: kernel::ffi::c_int, value: u32) -> kernel::ffi::c_int;
    pub fn polaris_init_arch();
    pub fn polaris_machine_check(vector: kernel::ffi::c_ulong, la_ptr: kernel::ffi::c_ulong);

    pub static mut t2_pci_ops: pci_ops;
    pub fn t2_init_arch();
    pub fn t2_kill_arch(arg: kernel::ffi::c_int);
    pub fn t2_machine_check(vector: kernel::ffi::c_ulong, la_ptr: kernel::ffi::c_ulong);
    pub fn t2_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub static mut titan_pci_ops: pci_ops;
    pub fn titan_init_arch();
    pub fn titan_kill_arch(arg: kernel::ffi::c_int);
    pub fn titan_machine_check(arg1: kernel::ffi::c_ulong, arg2: kernel::ffi::c_ulong);
    pub fn titan_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);
    pub fn titan_agp_info() -> *mut _alpha_agp_info;

    pub static mut tsunami_pci_ops: pci_ops;
    pub fn tsunami_init_arch();
    pub fn tsunami_kill_arch(arg: kernel::ffi::c_int);
    pub fn tsunami_machine_check(vector: kernel::ffi::c_ulong, la_ptr: kernel::ffi::c_ulong);
    pub fn tsunami_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub static mut wildfire_pci_ops: pci_ops;
    pub fn wildfire_init_arch();
    pub fn wildfire_kill_arch(arg: kernel::ffi::c_int);
    pub fn wildfire_machine_check(vector: kernel::ffi::c_ulong, la_ptr: kernel::ffi::c_ulong);
    pub fn wildfire_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub fn find_console_vga_hose();
    pub fn locate_and_init_vga(sel_func: *mut kernel::ffi::c_void);
    pub static mut srm_hae: kernel::ffi::c_ulong;
    pub static mut boot_cpuid: kernel::ffi::c_int;
    pub static mut vgacon_screen_info: screen_info;
    pub fn register_srm_console();
    pub fn unregister_srm_console();
    pub fn setup_smp();
    pub fn handle_ipi(regs: *mut pt_regs);
    pub fn smp_callin();
    pub fn rtc_timer_interrupt(irq: kernel::ffi::c_int, dev: *mut kernel::ffi::c_void) -> kernel::ffi::c_int;
    pub fn init_clockevent();
    pub fn common_init_rtc();
    pub static mut est_cycle_freq: kernel::ffi::c_ulong;
    pub fn SMC93x_Init() -> kernel::ffi::c_int;
    pub fn SMC669_Init(arg: kernel::ffi::c_int);
    pub fn es1888_init();
    pub fn alpha_write_fp_reg(reg: kernel::ffi::c_ulong, val: kernel::ffi::c_ulong);
    pub fn alpha_read_fp_reg(reg: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong;
    pub fn wrmces(mces: kernel::ffi::c_ulong);
    pub fn cserve_ena(arg: kernel::ffi::c_ulong);
    pub fn cserve_dis(arg: kernel::ffi::c_ulong);
    pub fn __smp_callin(arg: kernel::ffi::c_ulong);
    pub fn entArith(); pub fn entIF(); pub fn entInt(); pub fn entMM(); pub fn entSys(); pub fn entUna(); pub fn entDbg();
    pub fn pcibios_claim_one_bus(bus: *mut pci_bus);
    pub fn ptrace_set_bpt(child: *mut task_struct) -> kernel::ffi::c_int;
    pub fn ptrace_cancel_bpt(child: *mut task_struct) -> kernel::ffi::c_int;
    pub fn syscall_trace_leave();
    pub fn syscall_trace_enter() -> kernel::ffi::c_ulong;
    pub fn do_sigreturn(context: *mut sigcontext);
    pub fn do_rt_sigreturn(frame: *mut rt_sigframe);
    pub fn do_work_pending(regs: *mut pt_regs, a: kernel::ffi::c_ulong, b: kernel::ffi::c_ulong, c: kernel::ffi::c_ulong);
    pub fn alpha_schedule_user_work();
    pub fn dik_show_regs(regs: *mut pt_regs, r9_15: *mut kernel::ffi::c_ulong);
    pub fn die_if_kernel(str_: *mut kernel::ffi::c_char, regs: *mut pt_regs, arg: isize, regs2: *mut kernel::ffi::c_ulong);
    pub fn do_entInt(a: kernel::ffi::c_ulong, b: kernel::ffi::c_ulong, c: kernel::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_entArith(a: kernel::ffi::c_ulong, b: kernel::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_entIF(a: kernel::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_entDbg(regs: *mut pt_regs);
    pub fn do_entUna(a: *mut kernel::ffi::c_void, b: kernel::ffi::c_ulong, c: kernel::ffi::c_ulong, regs: *mut allregs);
    pub fn do_entUnaUser(a: *mut kernel::ffi::c_void, b: kernel::ffi::c_ulong, c: kernel::ffi::c_ulong, regs: *mut pt_regs);
    pub fn lockdep_on_restore(ps: kernel::ffi::c_ulong, ip: kernel::ffi::c_ulong);
    pub fn titan_dispatch_irqs(arg: u64);
    pub fn srm_paging_stop();
    pub fn ioremap_page_range(address: kernel::ffi::c_ulong, end: kernel::ffi::c_ulong, phys_addr: kernel::ffi::c_ulong, prot: usize) -> kernel::ffi::c_int;
    pub fn process_mcheck_info(vector: kernel::ffi::c_ulong, la_ptr: kernel::ffi::c_ulong, machine: *const kernel::ffi::c_char, expected: kernel::ffi::c_int);
}

#[inline]
pub unsafe fn __alpha_remap_area_pages(address: u64, phys_addr: u64, size: u64, flags: usize) -> i32 {
    // __pgprot(_PAGE_VALID | _PAGE_ASM | _PAGE_KRE | _PAGE_KWE | flags)
    ioremap_page_range(address, address.wrapping_add(size), phys_addr, flags)
}

pub const irongate_pci_tbi: *mut kernel::ffi::c_void = core::ptr::null_mut();
pub const polaris_pci_tbi: *mut kernel::ffi::c_void = core::ptr::null_mut();

#[cfg(not(CONFIG_SMP))]
#[repr(C, align(8))]
pub struct mcheck_info {
    pub expected: u8,
    pub taken: u8,
    pub extra: u8,
}

#[cfg(not(CONFIG_SMP))]
extern "C" {
    pub static mut __mcheck_info: mcheck_info;
}

#[cfg(CONFIG_SMP)]
// mcheck_expected(cpu), mcheck_taken(cpu), and mcheck_extra(cpu) refer to
// cpu_data[cpu].mcheck_expected, cpu_data[cpu].mcheck_taken, and
// cpu_data[cpu].mcheck_extra in the kernel build.
pub type mcheck_cpu_macros = ();

#[cfg(not(CONFIG_ALPHA_GENERIC))]
#[cfg(not(CONFIG_ALPHA_SRM))]
#[inline]
pub fn register_srm_console_noop() {}

#[cfg(not(CONFIG_ALPHA_GENERIC))]
#[cfg(not(CONFIG_ALPHA_SRM))]
#[inline]
pub fn unregister_srm_console_noop() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
