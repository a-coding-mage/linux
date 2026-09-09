/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding UAPI header: asm/ptrace.h.

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
mod sparc64 {
    use super::*;

    #[inline]
    pub unsafe fn pt_regs_trap_type(regs: *mut pt_regs) -> i32 {
        ((*regs).magic & 0x1ff) as i32
    }

    #[inline]
    pub unsafe fn pt_regs_is_syscall(regs: *mut pt_regs) -> bool {
        ((*regs).tstate & TSTATE_SYSCALL) != 0
    }

    #[inline]
    pub unsafe fn pt_regs_clear_syscall(regs: *mut pt_regs) -> bool {
        (*regs).tstate &= !TSTATE_SYSCALL;
        (*regs).tstate != 0
    }

    // arch_ptrace_stop_needed(): flush_user_windows(); get_thread_wsaved() != 0
    // arch_ptrace_stop(): synchronize_user_stack()

    #[inline]
    pub unsafe fn current_pt_regs() -> *mut pt_regs {
        ((current_thread_info() as usize + THREAD_SIZE) as *mut pt_regs).offset(-1)
    }

    #[repr(C)]
    pub struct global_reg_snapshot {
        pub tstate: c_ulong,
        pub tpc: c_ulong,
        pub tnpc: c_ulong,
        pub o7: c_ulong,
        pub i7: c_ulong,
        pub rpc: c_ulong,
        pub thread: *mut thread_info,
        pub pad1: c_ulong,
    }

    #[repr(C)]
    pub struct global_pmu_snapshot {
        pub pcr: [c_ulong; 4],
        pub pic: [c_ulong; 4],
    }

    #[repr(C)]
    pub union global_cpu_snapshot {
        pub reg: global_reg_snapshot,
        pub pmu: global_pmu_snapshot,
    }

    extern "C" {
        pub static mut global_cpu_snapshot: [global_cpu_snapshot; NR_CPUS as usize];
        pub fn set_thread_noerror(value: i32);
        pub fn regs_query_register_offset(name: *const c_char) -> i32;
        pub fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: c_uint) -> c_ulong;
        pub fn profile_pc(regs: *mut pt_regs) -> c_ulong;
    }

    #[inline]
    pub unsafe fn force_successful_syscall_return() {
        set_thread_noerror(1);
    }

    #[inline]
    pub unsafe fn user_mode(regs: *mut pt_regs) -> bool {
        ((*regs).tstate & TSTATE_PRIV) == 0
    }

    #[inline]
    pub unsafe fn instruction_pointer(regs: *mut pt_regs) -> c_ulong {
        (*regs).tpc
    }

    #[inline]
    pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: c_ulong) {
        (*regs).tpc = val;
        (*regs).tnpc = val.wrapping_add(4);
    }

    #[inline]
    pub unsafe fn user_stack_pointer(regs: *mut pt_regs) -> c_ulong {
        (*regs).u_regs[UREG_FP as usize]
    }

    #[inline]
    pub unsafe fn is_syscall_success(regs: *mut pt_regs) -> bool {
        ((*regs).tstate & (TSTATE_XCARRY | TSTATE_ICARRY)) == 0
    }

    #[inline]
    pub unsafe fn regs_return_value(regs: *mut pt_regs) -> c_long {
        (*regs).u_regs[UREG_I0 as usize] as c_long
    }

    pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, magic);

    #[inline]
    pub unsafe fn regs_get_register(regs: *mut pt_regs, offset: c_ulong) -> c_ulong {
        if offset as usize >= MAX_REG_OFFSET {
            return 0;
        }
        let address = (regs as usize + offset as usize) as *const u8;
        if offset as usize == PT_V9_Y as usize {
            *(address as *const c_uint) as c_ulong
        } else {
            *(address as *const c_ulong)
        }
    }

    #[inline]
    pub unsafe fn kernel_stack_pointer(regs: *mut pt_regs) -> c_ulong {
        (*regs).u_regs[UREG_I6 as usize]
    }
}

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
mod sparc32 {
    use super::*;

    #[inline]
    pub unsafe fn pt_regs_is_syscall(regs: *mut pt_regs) -> bool {
        ((*regs).psr & PSR_SYSCALL) != 0
    }

    #[inline]
    pub unsafe fn pt_regs_clear_syscall(regs: *mut pt_regs) -> bool {
        (*regs).psr &= !PSR_SYSCALL;
        (*regs).psr != 0
    }

    // arch_ptrace_stop_needed(): flush_user_windows(); current_thread_info()->w_saved != 0
    // arch_ptrace_stop(): synchronize_user_stack()

    #[inline]
    pub unsafe fn current_pt_regs() -> *mut pt_regs {
        ((current_thread_info() as usize + THREAD_SIZE) as *mut pt_regs).offset(-1)
    }

    #[inline]
    pub unsafe fn user_mode(regs: *mut pt_regs) -> bool {
        ((*regs).psr & PSR_PS) == 0
    }

    #[inline]
    pub unsafe fn instruction_pointer(regs: *mut pt_regs) -> c_ulong {
        (*regs).pc
    }

    #[inline]
    pub unsafe fn user_stack_pointer(regs: *mut pt_regs) -> c_ulong {
        (*regs).u_regs[UREG_FP as usize]
    }

    extern "C" {
        pub fn profile_pc(regs: *mut pt_regs) -> c_ulong;
    }
}

pub const STACK_BIAS: usize = 2047;
pub const GR_SNAP_TSTATE: usize = 0x00;
pub const GR_SNAP_TPC: usize = 0x08;
pub const GR_SNAP_TNPC: usize = 0x10;
pub const GR_SNAP_O7: usize = 0x18;
pub const GR_SNAP_I7: usize = 0x20;
pub const GR_SNAP_RPC: usize = 0x28;
pub const GR_SNAP_THREAD: usize = 0x30;
pub const GR_SNAP_PAD1: usize = 0x38;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
