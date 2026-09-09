/* SPDX-License-Identifier: GPL-2.0 */
// Translated from proto.h. Linux kernel dependencies are supplied externally.

pub type vucp = *mut core::ffi::c_uchar;
pub type vusp = *mut core::ffi::c_ushort;
pub type vip = *mut core::ffi::c_int;
pub type vuip = *mut core::ffi::c_uint;
pub type vulp = *mut core::ffi::c_ulong;

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
    pub fn cia_kill_arch(arg: core::ffi::c_int);
    pub fn cia_machine_check(vector: core::ffi::c_ulong, la_ptr: core::ffi::c_ulong);
    pub fn cia_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub static mut irongate_pci_ops: pci_ops;
    pub fn irongate_pci_clr_err() -> core::ffi::c_int;
    pub fn irongate_init_arch();

    pub static mut marvel_pci_ops: pci_ops;
    pub fn marvel_init_arch();
    pub fn marvel_kill_arch(arg: core::ffi::c_int);
    pub fn marvel_machine_check(arg1: core::ffi::c_ulong, arg2: core::ffi::c_ulong);
    pub fn marvel_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);
    pub fn marvel_agp_info() -> *mut _alpha_agp_info;
    pub fn marvel_find_io7(pe: core::ffi::c_int) -> *mut io7;
    pub fn marvel_next_io7(prev: *mut io7) -> *mut io7;
    pub fn io7_clear_errors(io7: *mut io7);

    pub static mut mcpcia_pci_ops: pci_ops;
    pub fn mcpcia_init_arch();
    pub fn mcpcia_init_hoses();
    pub fn mcpcia_machine_check(vector: core::ffi::c_ulong, la_ptr: core::ffi::c_ulong);
    pub fn mcpcia_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub static mut polaris_pci_ops: pci_ops;
    pub fn polaris_read_config_dword(dev: *mut pci_dev, where_: core::ffi::c_int, value: *mut u32) -> core::ffi::c_int;
    pub fn polaris_write_config_dword(dev: *mut pci_dev, where_: core::ffi::c_int, value: u32) -> core::ffi::c_int;
    pub fn polaris_init_arch();
    pub fn polaris_machine_check(vector: core::ffi::c_ulong, la_ptr: core::ffi::c_ulong);

    pub static mut t2_pci_ops: pci_ops;
    pub fn t2_init_arch();
    pub fn t2_kill_arch(arg: core::ffi::c_int);
    pub fn t2_machine_check(vector: core::ffi::c_ulong, la_ptr: core::ffi::c_ulong);
    pub fn t2_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub static mut titan_pci_ops: pci_ops;
    pub fn titan_init_arch();
    pub fn titan_kill_arch(arg: core::ffi::c_int);
    pub fn titan_machine_check(arg1: core::ffi::c_ulong, arg2: core::ffi::c_ulong);
    pub fn titan_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);
    pub fn titan_agp_info() -> *mut _alpha_agp_info;

    pub static mut tsunami_pci_ops: pci_ops;
    pub fn tsunami_init_arch();
    pub fn tsunami_kill_arch(arg: core::ffi::c_int);
    pub fn tsunami_machine_check(vector: core::ffi::c_ulong, la_ptr: core::ffi::c_ulong);
    pub fn tsunami_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub static mut wildfire_pci_ops: pci_ops;
    pub fn wildfire_init_arch();
    pub fn wildfire_kill_arch(arg: core::ffi::c_int);
    pub fn wildfire_machine_check(vector: core::ffi::c_ulong, la_ptr: core::ffi::c_ulong);
    pub fn wildfire_pci_tbi(controller: *mut pci_controller, start: u64, end: u64);

    pub fn find_console_vga_hose();
    pub fn locate_and_init_vga(sel_func: *mut core::ffi::c_void);
    pub static mut srm_hae: core::ffi::c_ulong;
    pub static mut boot_cpuid: core::ffi::c_int;
    pub static mut vgacon_screen_info: screen_info;
    pub fn register_srm_console();
    pub fn unregister_srm_console();
    pub fn setup_smp();
    pub fn handle_ipi(regs: *mut pt_regs);
    pub fn smp_callin();
    pub fn rtc_timer_interrupt(irq: core::ffi::c_int, dev: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn init_clockevent();
    pub fn common_init_rtc();
    pub static mut est_cycle_freq: core::ffi::c_ulong;
    pub fn SMC93x_Init() -> core::ffi::c_int;
    pub fn SMC669_Init(arg: core::ffi::c_int);
    pub fn es1888_init();
    pub fn alpha_write_fp_reg(reg: core::ffi::c_ulong, val: core::ffi::c_ulong);
    pub fn alpha_read_fp_reg(reg: core::ffi::c_ulong) -> core::ffi::c_ulong;
    pub fn wrmces(mces: core::ffi::c_ulong);
    pub fn cserve_ena(arg: core::ffi::c_ulong);
    pub fn cserve_dis(arg: core::ffi::c_ulong);
    pub fn __smp_callin(arg: core::ffi::c_ulong);
    pub fn entArith(); pub fn entIF(); pub fn entInt(); pub fn entMM(); pub fn entSys(); pub fn entUna(); pub fn entDbg();
    pub fn pcibios_claim_one_bus(bus: *mut pci_bus);
    pub fn ptrace_set_bpt(child: *mut task_struct) -> core::ffi::c_int;
    pub fn ptrace_cancel_bpt(child: *mut task_struct) -> core::ffi::c_int;
    pub fn syscall_trace_leave();
    pub fn syscall_trace_enter() -> core::ffi::c_ulong;
    pub fn do_sigreturn(context: *mut sigcontext);
    pub fn do_rt_sigreturn(frame: *mut rt_sigframe);
    pub fn do_work_pending(regs: *mut pt_regs, a: core::ffi::c_ulong, b: core::ffi::c_ulong, c: core::ffi::c_ulong);
    pub fn alpha_schedule_user_work();
    pub fn dik_show_regs(regs: *mut pt_regs, r9_15: *mut core::ffi::c_ulong);
    pub fn die_if_kernel(str_: *mut core::ffi::c_char, regs: *mut pt_regs, arg: isize, regs2: *mut core::ffi::c_ulong);
    pub fn do_entInt(a: core::ffi::c_ulong, b: core::ffi::c_ulong, c: core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_entArith(a: core::ffi::c_ulong, b: core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_entIF(a: core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_entDbg(regs: *mut pt_regs);
    pub fn do_entUna(a: *mut core::ffi::c_void, b: core::ffi::c_ulong, c: core::ffi::c_ulong, regs: *mut allregs);
    pub fn do_entUnaUser(a: *mut core::ffi::c_void, b: core::ffi::c_ulong, c: core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn lockdep_on_restore(ps: core::ffi::c_ulong, ip: core::ffi::c_ulong);
    pub fn titan_dispatch_irqs(arg: u64);
    pub fn srm_paging_stop();
    pub fn ioremap_page_range(address: core::ffi::c_ulong, end: core::ffi::c_ulong, phys_addr: core::ffi::c_ulong, prot: usize) -> core::ffi::c_int;
    pub fn process_mcheck_info(vector: core::ffi::c_ulong, la_ptr: core::ffi::c_ulong, machine: *const core::ffi::c_char, expected: core::ffi::c_int);
}

#[inline]
pub unsafe fn __alpha_remap_area_pages(address: u64, phys_addr: u64, size: u64, flags: usize) -> i32 {
    // __pgprot(_PAGE_VALID | _PAGE_ASM | _PAGE_KRE | _PAGE_KWE | flags)
    ioremap_page_range(address, address.wrapping_add(size), phys_addr, flags)
}

pub const irongate_pci_tbi: *mut core::ffi::c_void = core::ptr::null_mut();
pub const polaris_pci_tbi: *mut core::ffi::c_void = core::ptr::null_mut();

#[cfg(not(feature = "CONFIG_SMP"))]
#[repr(C, align(8))]
pub struct mcheck_info {
    pub expected: u8,
    pub taken: u8,
    pub extra: u8,
}

#[cfg(not(feature = "CONFIG_SMP"))]
extern "C" {
    pub static mut __mcheck_info: mcheck_info;
}

#[cfg(feature = "CONFIG_SMP")]
// mcheck_expected(cpu), mcheck_taken(cpu), and mcheck_extra(cpu) refer to
// cpu_data[cpu].mcheck_expected, cpu_data[cpu].mcheck_taken, and
// cpu_data[cpu].mcheck_extra in the kernel build.
pub type mcheck_cpu_macros = ();

#[cfg(not(feature = "CONFIG_ALPHA_GENERIC"))]
#[cfg(not(feature = "CONFIG_ALPHA_SRM"))]
#[inline]
pub fn register_srm_console_noop() {}

#[cfg(not(feature = "CONFIG_ALPHA_GENERIC"))]
#[cfg(not(feature = "CONFIG_ALPHA_SRM"))]
#[inline]
pub fn unregister_srm_console_noop() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
