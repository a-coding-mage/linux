// SPDX-License-Identifier: GPL-2.0
/*
 * x86 single-step support code, common to 32-bit and 64-bit.
 */

// Types and symbols below are supplied by the surrounding kernel headers.
#[repr(C)]
pub struct pt_regs {
    pub ip: c_ulong,
    pub cs: c_ulong,
    pub flags: c_ulong,
}
#[repr(C)]
pub struct desc_struct {
    pub d: bool,
}
#[repr(C)]
pub struct ldt_struct {
    pub nr_entries: c_ulong,
    pub entries: *mut desc_struct,
}
#[repr(C)]
pub struct mm_context {
    pub lock: opaque_mutex,
    pub ldt: *mut ldt_struct,
}
#[repr(C)]
pub struct mm_struct {
    pub context: mm_context,
}
#[repr(C)]
pub struct task_struct {
    pub mm: *mut mm_struct,
}
#[repr(C)]
pub struct opaque_mutex {
    _private: [u8; 0],
}

type c_ulong = usize;
type c_int = i32;
type c_uchar = u8;

const SEGMENT_TI_MASK: c_ulong = 0x4;
const SEGMENT_LDT: c_ulong = 0x4;
const FOLL_FORCE: c_ulong = 0;
const X86_EFLAGS_TF: c_ulong = 1 << 8;
const DEBUGCTLMSR_BTF: c_ulong = 1 << 1;
const SYSCALL_EXIT_TRAP: c_ulong = 1;
const TIF_SINGLESTEP: c_int = 0;
const TIF_FORCED_TF: c_int = 1;
const TIF_BLOCKSTEP: c_int = 2;

extern "C" {
    fn v8086_mode(regs: *mut pt_regs) -> bool;
    fn user_64bit_mode(regs: *mut pt_regs) -> bool;
    fn get_desc_base(desc: *mut desc_struct) -> c_ulong;
    fn mutex_lock(lock: *mut opaque_mutex);
    fn mutex_unlock(lock: *mut opaque_mutex);
    fn access_process_vm(task: *mut task_struct, addr: c_ulong, buf: *mut c_uchar,
                         len: c_ulong, flags: c_ulong) -> c_int;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn unlikely(value: bool) -> bool;
    fn test_tsk_thread_flag(task: *mut task_struct, flag: c_int) -> bool;
    fn set_tsk_thread_flag(task: *mut task_struct, flag: c_int);
    fn clear_tsk_thread_flag(task: *mut task_struct, flag: c_int);
    fn test_and_clear_tsk_thread_flag(task: *mut task_struct, flag: c_int) -> bool;
    fn set_task_syscall_work(task: *mut task_struct, work: c_ulong);
    fn clear_task_syscall_work(task: *mut task_struct, work: c_ulong);
    fn get_debugctlmsr() -> c_ulong;
    fn update_debugctlmsr(value: c_ulong);
    fn local_irq_disable();
    fn local_irq_enable();
    static mut current: *mut task_struct;
}

pub unsafe fn convert_ip_to_linear(child: *mut task_struct, regs: *mut pt_regs) -> c_ulong {
    let mut addr = (*regs).ip;
    let mut seg = (*regs).cs;
    if v8086_mode(regs) {
        addr = (addr & 0xffff).wrapping_add(seg << 4);
        return addr;
    }

    // CONFIG_MODIFY_LDT_SYSCALL conditionally supplies this LDT handling.
    if (seg & SEGMENT_TI_MASK) == SEGMENT_LDT {
        let mut desc: *mut desc_struct;
        let mut base: c_ulong;
        seg >>= 3;
        let mm = (*child).mm;
        mutex_lock(&mut (*mm).context.lock);
        if unlikely((*mm).context.ldt.is_null()
            || seg >= (*(*mm).context.ldt).nr_entries) {
            addr = (!0usize); // bogus selector, access would fault
        } else {
            desc = (*mm).context.ldt.add(0).read().entries.add(seg) as *mut desc_struct;
            base = get_desc_base(desc);
            // 16-bit code segment?
            if !(*desc).d {
                addr &= 0xffff;
            }
            addr = addr.wrapping_add(base);
        }
        mutex_unlock(&mut (*mm).context.lock);
    }
    addr
}

unsafe fn is_setting_trap_flag(child: *mut task_struct, regs: *mut pt_regs) -> c_int {
    let mut opcode = [0u8; 15];
    let addr = convert_ip_to_linear(child, regs);
    let copied = access_process_vm(child, addr, opcode.as_mut_ptr(), opcode.len(), FOLL_FORCE);
    for i in 0..copied {
        match opcode[i as usize] {
            0x9d | 0xcf => return 1, // popf and iret
            0x66 | 0x67 => continue,
            0x26 | 0x2e | 0x36 | 0x3e | 0x64 | 0x65 | 0xf0 | 0xf2 | 0xf3 => continue,
            0x40..=0x4f => {
                if !user_64bit_mode(regs) {
                    // 32-bit mode: register increment
                    return 0;
                }
                // 64-bit mode: REX prefix
                continue;
            }
            // CHECKME: f2, f3
            // pushf: NOTE! We should probably not let the user see the TF bit being set.
            _ => return 0,
        }
    }
    0
}

unsafe fn enable_single_step(child: *mut task_struct) -> c_int {
    let regs = task_pt_regs(child);
    let oflags;
    if unlikely(test_tsk_thread_flag(child, TIF_SINGLESTEP)) {
        (*regs).flags |= X86_EFLAGS_TF;
    }
    set_tsk_thread_flag(child, TIF_SINGLESTEP);
    set_task_syscall_work(child, SYSCALL_EXIT_TRAP);
    oflags = (*regs).flags;
    (*regs).flags |= X86_EFLAGS_TF;
    if is_setting_trap_flag(child, regs) != 0 {
        clear_tsk_thread_flag(child, TIF_FORCED_TF);
        return 0;
    }
    if (oflags & X86_EFLAGS_TF) != 0 {
        return test_tsk_thread_flag(child, TIF_FORCED_TF) as c_int;
    }
    set_tsk_thread_flag(child, TIF_FORCED_TF);
    1
}

pub unsafe fn set_task_blockstep(task: *mut task_struct, on: bool) {
    local_irq_disable();
    let mut debugctl = get_debugctlmsr();
    if on {
        debugctl |= DEBUGCTLMSR_BTF;
        set_tsk_thread_flag(task, TIF_BLOCKSTEP);
    } else {
        debugctl &= !DEBUGCTLMSR_BTF;
        clear_tsk_thread_flag(task, TIF_BLOCKSTEP);
    }
    if task == current {
        update_debugctlmsr(debugctl);
    }
    local_irq_enable();
}

unsafe fn enable_step(child: *mut task_struct, block: bool) {
    if enable_single_step(child) != 0 && block {
        set_task_blockstep(child, true);
    } else if test_tsk_thread_flag(child, TIF_BLOCKSTEP) {
        set_task_blockstep(child, false);
    }
}

pub unsafe fn user_enable_single_step(child: *mut task_struct) {
    enable_step(child, false);
}

pub unsafe fn user_enable_block_step(child: *mut task_struct) {
    enable_step(child, true);
}

pub unsafe fn user_disable_single_step(child: *mut task_struct) {
    if test_tsk_thread_flag(child, TIF_BLOCKSTEP) {
        set_task_blockstep(child, false);
    }
    clear_tsk_thread_flag(child, TIF_SINGLESTEP);
    clear_task_syscall_work(child, SYSCALL_EXIT_TRAP);
    if test_and_clear_tsk_thread_flag(child, TIF_FORCED_TF) {
        (*task_pt_regs(child)).flags &= !X86_EFLAGS_TF;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
