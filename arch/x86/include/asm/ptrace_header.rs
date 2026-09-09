/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from x86/include/asm/ptrace.h. C includes and build conditions
 * are represented by the external names and cfg branches used below. */

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct pt_regs {
    pub bx: c_ulong, pub cx: c_ulong, pub dx: c_ulong, pub si: c_ulong,
    pub di: c_ulong, pub bp: c_ulong, pub ax: c_ulong,
    pub ds: u16, pub __dsh: u16, pub es: u16, pub __esh: u16,
    pub fs: u16, pub __fsh: u16, pub gs: u16, pub __gsh: u16,
    pub orig_ax: c_ulong, pub ip: c_ulong, pub cs: u16, pub __csh: u16,
    pub flags: c_ulong, pub sp: c_ulong, pub ss: u16, pub __ssh: u16,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct fred_cs { pub bits: u64 }
#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct fred_ss { pub bits: u64 }

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub union pt_regs_cs { pub cs: u16, pub csx: u64, pub fred_cs: fred_cs }
#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub union pt_regs_ss { pub ss: u16, pub ssx: u64, pub fred_ss: fred_ss }
#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct pt_regs {
    pub r15: c_ulong, pub r14: c_ulong, pub r13: c_ulong, pub r12: c_ulong,
    pub bp: c_ulong, pub bx: c_ulong, pub r11: c_ulong, pub r10: c_ulong,
    pub r9: c_ulong, pub r8: c_ulong, pub ax: c_ulong, pub cx: c_ulong,
    pub dx: c_ulong, pub si: c_ulong, pub di: c_ulong, pub orig_ax: c_ulong,
    pub ip: c_ulong, pub cs: pt_regs_cs, pub flags: c_ulong, pub sp: c_ulong,
    pub ss: pt_regs_ss,
}

extern "C" {
    pub fn profile_pc(regs: *mut pt_regs) -> c_ulong;
    pub fn convert_ip_to_linear(child: *mut task_struct, regs: *mut pt_regs) -> c_ulong;
    pub fn send_sigtrap(regs: *mut pt_regs, error_code: c_int, si_code: c_int);
    pub fn regs_query_register_offset(name: *const c_char) -> c_int;
    pub fn regs_query_register_name(offset: c_uint) -> *const c_char;
    pub fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_long;
}

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
extern "C" { pub static boot_cpu_data: cpuinfo_x86; }
#[repr(C)] pub struct cpuinfo_x86 { pub x86: u8 }

#[inline(always)] pub unsafe fn regs_return_value(r: *mut pt_regs) -> c_ulong { (*r).ax }
#[inline(always)] pub unsafe fn regs_set_return_value(r: *mut pt_regs, rc: c_ulong) { (*r).ax = rc; }
#[inline(always)] pub unsafe fn user_mode(r: *mut pt_regs) -> c_int {
    #[cfg(target_arch = "x86")] { (((*r).cs as c_ulong & SEGMENT_RPL_MASK) | ((*r).flags & X86_VM_MASK) >= USER_RPL) as c_int }
    #[cfg(target_arch = "x86_64")] { (((*r).cs.cs & 3) != 0) as c_int }
}
#[inline(always)] pub unsafe fn v8086_mode(r: *mut pt_regs) -> c_int {
    #[cfg(target_arch = "x86")] { ((*r).flags & X86_VM_MASK) as c_int }
    #[cfg(target_arch = "x86_64")] { 0 }
}
#[inline] pub unsafe fn any_64bit_mode(r: *mut pt_regs) -> bool { #[cfg(target_arch="x86_64")] { user_mode(r)==0 || user_64bit_mode(r) } #[cfg(target_arch="x86")] { false } }
#[cfg(target_arch="x86_64")]
#[inline] pub unsafe fn user_64bit_mode(r: *mut pt_regs) -> bool { (*r).cs.cs == __USER_CS }

#[inline(always)] pub unsafe fn kernel_stack_pointer(r: *mut pt_regs) -> c_ulong { (*r).sp }
#[inline(always)] pub unsafe fn instruction_pointer(r: *mut pt_regs) -> c_ulong { (*r).ip }
#[inline(always)] pub unsafe fn instruction_pointer_set(r: *mut pt_regs, v: c_ulong) { (*r).ip = v; }
#[inline(always)] pub unsafe fn frame_pointer(r: *mut pt_regs) -> c_ulong { (*r).bp }
#[inline(always)] pub unsafe fn user_stack_pointer(r: *mut pt_regs) -> c_ulong { (*r).sp }
#[inline(always)] pub unsafe fn user_stack_pointer_set(r: *mut pt_regs, v: c_ulong) { (*r).sp = v; }
#[inline(always)] pub unsafe fn regs_irqs_disabled(r: *mut pt_regs) -> bool { (*r).flags & X86_EFLAGS_IF == 0 }

pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, ss);
#[inline] pub unsafe fn regs_get_register(r: *mut pt_regs, offset: c_uint) -> c_ulong {
    if offset as usize > MAX_REG_OFFSET { return 0; }
    #[cfg(target_arch="x86")]
    { if [core::mem::offset_of!(pt_regs,cs),core::mem::offset_of!(pt_regs,ss),core::mem::offset_of!(pt_regs,ds),core::mem::offset_of!(pt_regs,es),core::mem::offset_of!(pt_regs,fs),core::mem::offset_of!(pt_regs,gs)].contains(&(offset as usize)) { return *((r as *mut u8).add(offset as usize) as *const u16) as c_ulong; } }
    *((r as *mut u8).add(offset as usize) as *const c_ulong)
}
#[inline] pub unsafe fn regs_within_kernel_stack(r: *mut pt_regs, addr: c_ulong) -> bool { (addr & !(THREAD_SIZE - 1)) == ((*r).sp & !(THREAD_SIZE - 1)) }
#[inline] pub unsafe fn regs_get_kernel_stack_nth_addr(r: *mut pt_regs, n: c_uint) -> *mut c_ulong { let p = ((*r).sp as *mut c_ulong).add(n as usize); if regs_within_kernel_stack(r,p as c_ulong) { p } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn regs_get_kernel_stack_nth(r: *mut pt_regs, n: c_uint) -> c_ulong { let p=regs_get_kernel_stack_nth_addr(r,n); let mut v=0; if !p.is_null() && copy_from_kernel_nofault((&mut v as *mut _).cast(),p.cast(),core::mem::size_of_val(&v))==0 {v} else {0} }

pub const fn arch_has_single_step() -> bool { true }
#[cfg(feature="CONFIG_X86_DEBUGCTLMSR")] pub const fn arch_has_block_step() -> bool { true }
#[cfg(not(feature="CONFIG_X86_DEBUGCTLMSR"))] pub unsafe fn arch_has_block_step() -> bool { boot_cpu_data.x86 >= 6 }
pub const ARCH_HAS_USER_SINGLE_STEP_REPORT: bool = true;
#[inline] pub unsafe fn regs_get_kernel_argument(r: *mut pt_regs, n: c_uint) -> c_ulong {
    #[cfg(target_arch="x86")] let a = [core::mem::offset_of!(pt_regs,ax),core::mem::offset_of!(pt_regs,dx),core::mem::offset_of!(pt_regs,cx)];
    #[cfg(target_arch="x86_64")] let a = [core::mem::offset_of!(pt_regs,di),core::mem::offset_of!(pt_regs,si),core::mem::offset_of!(pt_regs,dx),core::mem::offset_of!(pt_regs,cx),core::mem::offset_of!(pt_regs,r8),core::mem::offset_of!(pt_regs,r9)];
    if (n as usize) >= a.len() { regs_get_kernel_stack_nth(r, n - (a.len() as u32 - 1)) } else { regs_get_register(r,a[n as usize] as c_uint) }
}
pub struct user_desc;
extern "C" { pub fn do_get_thread_area(p:*mut task_struct, idx:c_int, info:*mut user_desc)->c_int; pub fn do_set_thread_area(p:*mut task_struct, idx:c_int, info:*mut user_desc, can_allocate:c_int)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
