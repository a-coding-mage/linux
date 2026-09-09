/* SPDX-License-Identifier: GPL-2.0 */
/* S390 processor declarations translated from processor.h. */

pub const CIF_ENABLED_WAIT: i32 = 5;
pub const CIF_MCCK_GUEST: i32 = 6;
pub const CIF_DEDICATED_CPU: i32 = 7;
pub const _CIF_ENABLED_WAIT: usize = 1usize << CIF_ENABLED_WAIT;
pub const _CIF_MCCK_GUEST: usize = 1usize << CIF_MCCK_GUEST;
pub const _CIF_DEDICATED_CPU: usize = 1usize << CIF_DEDICATED_CPU;
pub const RESTART_FLAG_CTLREGS: u32 = 1u32 << 0;

#[repr(C)]
pub struct pcpu {
    pub ec_mask: ::core::ffi::c_ulong,
    pub ec_clk: ::core::ffi::c_ulong,
    pub flags: ::core::ffi::c_ulong,
    pub capacity: ::core::ffi::c_ulong,
    pub state: i8,
    pub polarization: i8,
    pub address: u16,
}

extern "C" {
    pub static mut pcpu_devices: pcpu;
    pub fn get_lowcore() -> *mut lowcore;
    pub fn set_bit(flag: i32, addr: *mut ::core::ffi::c_ulong);
    pub fn clear_bit(flag: i32, addr: *mut ::core::ffi::c_ulong);
    pub fn test_bit(flag: i32, addr: *const ::core::ffi::c_ulong) -> bool;
    pub fn test_and_set_bit(flag: i32, addr: *mut ::core::ffi::c_ulong) -> bool;
    pub fn test_and_clear_bit(flag: i32, addr: *mut ::core::ffi::c_ulong) -> bool;
    pub fn execve_tail();
    pub fn vdso_text_size() -> ::core::ffi::c_ulong;
    pub fn vdso_size() -> ::core::ffi::c_ulong;
}

pub type sys_call_ptr_t = unsafe extern "C" fn(*mut pt_regs) -> ::core::ffi::c_long;

#[inline(always)]
pub unsafe fn this_pcpu() -> *mut pcpu { (*get_lowcore()).pcpu as *mut pcpu }
#[inline(always)]
pub unsafe fn set_cpu_flag(flag: i32) { set_bit(flag, &mut (*this_pcpu()).flags); }
#[inline(always)]
pub unsafe fn clear_cpu_flag(flag: i32) { clear_bit(flag, &mut (*this_pcpu()).flags); }
#[inline(always)]
pub unsafe fn test_cpu_flag(flag: i32) -> bool { test_bit(flag, &(*this_pcpu()).flags) }
#[inline(always)]
pub unsafe fn test_and_set_cpu_flag(flag: i32) -> bool { test_and_set_bit(flag, &mut (*this_pcpu()).flags) }
#[inline(always)]
pub unsafe fn test_and_clear_cpu_flag(flag: i32) -> bool { test_and_clear_bit(flag, &mut (*this_pcpu()).flags) }
#[inline(always)]
pub unsafe fn test_cpu_flag_of(flag: i32, cpu: usize) -> bool {
    test_bit(flag, &pcpu_devices.add(cpu).flags)
}
#[inline] pub unsafe fn get_cpu_id(ptr: *mut cpuid) { core::arch::asm!("stidp 0({0})", in(reg) ptr); }
#[inline] pub unsafe fn get_cpu_timer() -> u64 { let timer: u64; core::arch::asm!("stpt {0}", out(reg) timer); timer }

extern "C" {
    pub fn s390_adjust_jiffies();
    pub fn s390_update_cpu_mhz();
    pub fn cpu_detect_mhz_feature();
    pub static cpuinfo_op: seq_operations;
    pub fn show_registers(regs: *mut pt_regs);
    pub fn show_cacheinfo(m: *mut seq_file);
    pub fn guarded_storage_release(tsk: *mut task_struct);
    pub fn gs_load_bc_cb(regs: *mut pt_regs);
    pub fn __get_wchan(p: *mut task_struct) -> ::core::ffi::c_ulong;
}

pub const TASK_SIZE: usize = TASK_SIZE_MAX;
pub const TASK_UNMAPPED_BASE: usize = _REGION2_SIZE >> 1;
pub const TASK_SIZE_MAX: usize = (!PAGE_SIZE.wrapping_add(0)).wrapping_add(1);
pub const VDSO_BASE: usize = STACK_TOP + PAGE_SIZE;
pub const VDSO_LIMIT: usize = _REGION2_SIZE;
pub const ARCH_MIN_TASKALIGN: usize = 8;
pub const PER_FLAG_NO_TE: ::core::ffi::c_ulong = 1;
pub const PER_FLAG_TE_ABORT_RAND: ::core::ffi::c_ulong = 2;
pub const PER_FLAG_TE_ABORT_RAND_TEND: ::core::ffi::c_ulong = 4;
pub const ARCH_LOW_ADDRESS_LIMIT: ::core::ffi::c_ulong = 0x7fffffff;

#[inline(always)]
pub unsafe fn __stackleak_poison(erase_low: ::core::ffi::c_ulong, erase_high: ::core::ffi::c_ulong, poison: ::core::ffi::c_ulong) {
    let mut p = erase_low as *mut ::core::ffi::c_ulong;
    let end = erase_high as usize;
    while (p as usize) < end { core::ptr::write_volatile(p, poison); p = (p as *mut u8).add(8) as *mut _; }
}

#[repr(C)]
pub struct thread_struct {
    pub acrs: [u32; NUM_ACRS], pub ksp: ::core::ffi::c_ulong,
    pub user_timer: ::core::ffi::c_ulong, pub guest_timer: ::core::ffi::c_ulong,
    pub system_timer: ::core::ffi::c_ulong, pub hardirq_timer: ::core::ffi::c_ulong,
    pub softirq_timer: ::core::ffi::c_ulong, pub gmap_teid: teid,
    pub gmap_int_code: u32, pub ufpu_flags: i32, pub kfpu_flags: i32,
    pub per_user: per_regs, pub per_event: per_event, pub per_flags: ::core::ffi::c_ulong,
    pub system_call: u32, pub last_break: ::core::ffi::c_ulong,
    pub pfault_wait: ::core::ffi::c_ulong, pub list: list_head,
    pub ri_cb: *mut runtime_instr_cb, pub gs_cb: *mut gs_cb, pub gs_bc_cb: *mut gs_cb,
    pub trap_tdb: pgm_tdb, pub ufpu: fpu, pub kfpu: fpu,
}
pub type thread_struct_alias = thread_struct;

#[inline(always)]
pub unsafe fn start_thread(regs: *mut pt_regs, new_psw: u64, new_stackp: u64) {
    (*regs).psw.mask = PSW_USER_BITS | PSW_MASK_EA | PSW_MASK_BA;
    (*regs).psw.addr = new_psw; (*regs).gprs[15] = new_stackp; execve_tail();
}
#[inline(always)]
pub unsafe fn start_thread31(regs: *mut pt_regs, new_psw: u64, new_stackp: u64) {
    (*regs).psw.mask = PSW_USER_BITS | PSW_MASK_BA;
    (*regs).psw.addr = new_psw; (*regs).gprs[15] = new_stackp; execve_tail();
}
#[inline(always)] pub unsafe fn is_ri_task(tsk: *mut task_struct) -> bool { !(*tsk).thread.ri_cb.is_null() }
#[inline(always)] pub unsafe fn task_pt_regs(tsk: *mut task_struct) -> *mut pt_regs { (task_stack_page(tsk) as usize + THREAD_SIZE - core::mem::size_of::<pt_regs>()) as *mut pt_regs }
#[inline(always)] pub unsafe fn KSTK_EIP(tsk: *mut task_struct) -> u64 { (*task_pt_regs(tsk)).psw.addr }
#[inline(always)] pub unsafe fn KSTK_ESP(tsk: *mut task_struct) -> u64 { (*task_pt_regs(tsk)).gprs[15] }

#[inline(always)]
pub unsafe fn __current_stack_pointer() -> ::core::ffi::c_ulong { let sp: ::core::ffi::c_ulong; core::arch::asm!("lgr {0},15", out(reg) sp); sp }
#[inline(always)]
pub unsafe fn on_thread_stack() -> bool { let ksp = (*get_lowcore()).kernel_stack; ((ksp ^ __current_stack_pointer()) & !(THREAD_SIZE - 1)) == 0 }
#[inline(always)]
pub unsafe fn stap() -> u16 { let v: u16; core::arch::asm!("stap {0}", out(reg) v); v }
pub const ECAG_CACHE_ATTRIBUTE: u32 = 0;
pub const ECAG_CPU_ATTRIBUTE: u32 = 1;
#[inline(always)]
pub unsafe fn __rewind_psw(psw: psw_t, ilen: isize) -> ::core::ffi::c_ulong {
    let mask = if psw.mask & PSW_MASK_EA != 0 { !0 } else if psw.mask & PSW_MASK_BA != 0 { (1u64 << 31) - 1 } else { (1u64 << 24) - 1 };
    (psw.addr.wrapping_sub(ilen as u64)) & mask
}
#[inline(always)] pub unsafe fn __forward_psw(psw: psw_t, ilen: isize) -> u64 { __rewind_psw(psw, -ilen) }
#[inline(always)] pub unsafe fn disabled_wait() -> ! { let psw = psw_t { mask: PSW_MASK_BASE | PSW_MASK_WAIT | PSW_MASK_BA | PSW_MASK_EA, addr: _THIS_IP_ }; __load_psw(psw); loop {} }
#[inline(always)] pub unsafe fn __load_psw(_psw: psw_t) { core::arch::asm!("lpswe 0({0})", in(reg) &_psw); }
#[inline(always)] pub unsafe fn regs_irqs_disabled(regs: *mut pt_regs) -> bool { arch_irqs_disabled_flags((*regs).psw.mask) }
#[inline(always)] pub unsafe fn bpon() { core::arch::asm!("nop"); }

// The following declarations are supplied by the architecture and other headers.
extern "C" {
    pub fn arch_irqs_disabled_flags(flags: u64) -> bool;
    pub fn task_stack_page(tsk: *mut task_struct) -> *mut u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
