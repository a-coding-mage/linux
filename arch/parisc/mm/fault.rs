/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Dependencies are supplied by the surrounding kernel translation. */

const DEBUG_NATLB: i32 = 0;
const BITSSET: u32 = 0x1c0;

pub static mut show_unhandled_signals: i32 = 1;

#[inline]
fn bit22set(x: u32) -> bool { (x & 0x00000200) != 0 }
#[inline]
fn bits23_25set(x: u32) -> u32 { x & 0x000001c0 }
#[inline]
fn is_graphics_flush_read(x: u32) -> bool { (x & 0xfc003fdf) == 0x04001a80 }

pub unsafe fn parisc_acctyp(code: u64, inst: u32) -> u64 {
    if code == 6 || code == 16 { return VM_EXEC as u64; }
    match inst & 0xf0000000 {
        0x40000000 | 0x50000000 => VM_READ as u64,
        0x60000000 | 0x70000000 => VM_WRITE as u64,
        0x20000000 | 0x30000000 => {
            if bit22set(inst) { return VM_WRITE as u64; }
            if bit22set(inst) {
                if is_graphics_flush_read(inst) { return VM_READ as u64; }
                return VM_WRITE as u64;
            }
            if bits23_25set(inst) == BITSSET { return VM_WRITE as u64; }
            VM_READ as u64
        }
        0 => {
            if bit22set(inst) {
                if is_graphics_flush_read(inst) { return VM_READ as u64; }
                return VM_WRITE as u64;
            }
            if bits23_25set(inst) == BITSSET { return VM_WRITE as u64; }
            VM_READ as u64
        }
        _ => VM_READ as u64,
    }
}

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> i32 {
    let fix = search_exception_tables((*regs).iaoq[0]);
    if !fix.is_null() {
        if (*fix).fixup & 1 != 0 {
            let fault_error_reg = ((*fix).err_opcode & 0x1f) as usize;
            if !WARN_ON(fault_error_reg == 0) { (*regs).gr[fault_error_reg] = (-EFAULT) as _; }
            pr_debug!("Unalignment fixup of register {} at {:p}\n", fault_error_reg, (*regs).iaoq[0] as *const u8);
            if parisc_acctyp(0, (*regs).iir) == VM_READ as u64 {
                let treg = ((*regs).iir & 0x1f) as usize;
                BUG_ON!(treg == 0);
                (*regs).gr[treg] = 0;
            }
        }
        (*regs).iaoq[0] = ((fix as usize + core::mem::offset_of!(exception_table_entry, fixup)) as u64).wrapping_add((*fix).fixup);
        (*regs).iaoq[0] &= !3;
        (*regs).iaoq[1] = (*regs).iaoq[0] + 4;
        (*regs).gr[0] &= !PSW_B;
        return 1;
    }
    0
}

static trap_description: [Option<&'static str>; 29] = [
    None, Some("High-priority machine check (HPMC)"), Some("Power failure interrupt"), Some("Recovery counter trap"), None,
    Some("Low-priority machine check"), Some("Instruction TLB miss fault"), Some("Instruction access rights / protection trap"), Some("Illegal instruction trap"), Some("Break instruction trap"), Some("Privileged operation trap"), Some("Privileged register trap"), Some("Overflow trap"), Some("Conditional trap"), Some("FP Assist Exception trap"), Some("Data TLB miss fault"), Some("Non-access ITLB miss fault"), Some("Non-access DTLB miss fault"), Some("Data memory protection/unaligned access trap"), Some("Data memory break trap"), Some("TLB dirty bit trap"), Some("Page reference trap"), Some("Assist emulation trap"), None, None, Some("Taken branch trap"), Some("Data memory access rights trap"), Some("Data memory protection ID trap"), Some("Unaligned data reference trap")
];

pub fn trap_name(code: usize) -> &'static str { trap_description.get(code).and_then(|x| *x).unwrap_or("Unknown trap") }

unsafe fn show_signal_msg(regs: *mut pt_regs, code: u64, address: u64, tsk: *mut task_struct, vma: *mut vm_area_struct) {
    if !unhandled_signal(tsk, SIGSEGV) || !printk_ratelimit() { return; }
    pr_warn!("\n");
    pr_warn!("do_page_fault() command='{}' type={} address=0x{:08x}", (*tsk).comm, code, address);
    print_vma_addr(KERN_CONT, " in ", (*regs).iaoq[0]);
    pr_cont!("\ntrap #{}: {}{}", code, trap_name(code as usize), if !vma.is_null() { ',' } else { '\n' });
    if !vma.is_null() { pr_cont!(" vm_start = 0x{:08x}, vm_end = 0x{:08x}\n", (*vma).vm_start, (*vma).vm_end); }
    show_regs(regs);
}

pub unsafe fn do_page_fault(regs: *mut pt_regs, code: u64, address: u64) {
    let tsk = current; let mm = (*tsk).mm;
    if mm.is_null() { parisc_terminate("Page fault: no context", regs, code, address); return; }
    let mut flags = FAULT_FLAG_DEFAULT | if user_mode(regs) { FAULT_FLAG_USER } else { 0 };
    let acc_type = parisc_acctyp(code, (*regs).iir);
    if acc_type & VM_WRITE as u64 != 0 { flags |= FAULT_FLAG_WRITE; }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);
    'retry: loop {
        mmap_read_lock(mm);
        let mut prev_vma = core::ptr::null_mut(); let mut vma = find_vma_prev(mm, address, &mut prev_vma);
        if vma.is_null() || address < (*vma).vm_start {
            if prev_vma.is_null() || (*prev_vma).vm_flags & VM_GROWSUP == 0 { mmap_read_unlock(mm); if !user_mode(regs) && fixup_exception(regs) != 0 { return; } parisc_terminate("Page fault: bad address", regs, code, address); return; }
            vma = expand_stack(mm, address); if vma.is_null() { mmap_read_unlock(mm); parisc_terminate("Page fault: bad address", regs, code, address); return; }
        }
        if (*vma).vm_flags & acc_type as _ != acc_type as _ { mmap_read_unlock(mm); if !user_mode(regs) && fixup_exception(regs) != 0 { return; } parisc_terminate("Page fault: bad address", regs, code, address); return; }
        let fault = handle_mm_fault(vma, address, flags, regs);
        if fault_signal_pending(fault, regs) { if !user_mode(regs) { mmap_read_unlock(mm); parisc_terminate("Page fault: fault signal on kernel memory", regs, code, address); } return; }
        if fault & VM_FAULT_COMPLETED != 0 { return; }
        if fault & VM_FAULT_ERROR != 0 { if fault & VM_FAULT_OOM != 0 { mmap_read_unlock(mm); if !user_mode(regs) { parisc_terminate("Page fault: out of memory", regs, code, address); } else { pagefault_out_of_memory(); } return; } BUG!(); }
        if fault & VM_FAULT_RETRY != 0 { flags |= FAULT_FLAG_TRIED; continue 'retry; }
        mmap_read_unlock(mm); return;
    }
}

pub unsafe fn handle_nadtlb_fault(regs: *mut pt_regs) -> i32 {
    let insn = (*regs).iir; let mut breg; let mut treg; let mut xreg; let mut val = 0;
    match insn & 0x380 {
        0x280 | 0x380 => { if insn & 0x20 != 0 { breg=(insn>>21)&0x1f; xreg=(insn>>16)&0x1f; if breg!=0 && xreg!=0 { (*regs).gr[breg as usize] += (*regs).gr[xreg as usize]; } } (*regs).gr[0] |= PSW_N; 1 }
        0x180 => { treg=insn&0x1f; if (*regs).isr != 0 { let mm=(*current).mm; if !mm.is_null() { mmap_read_lock(mm); let vma=vma_lookup(mm,(*regs).ior); mmap_read_unlock(mm); let a=if insn&0x40!=0 {VM_WRITE} else {VM_READ}; if !vma.is_null() && (*vma).vm_flags&a==a { val=1; } } } if treg!=0 { (*regs).gr[treg as usize]=val; } (*regs).gr[0]|=PSW_N; 1 }
        0x300 => { if insn&0x20!=0 { breg=(insn>>21)&0x1f; xreg=(insn>>16)&0x1f; if breg!=0&&xreg!=0 { (*regs).gr[breg as usize]+=(*regs).gr[xreg as usize]; } } treg=insn&0x1f; if treg!=0 { (*regs).gr[treg as usize]=0; } (*regs).gr[0]|=PSW_N; 1 }
        _ => 0,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
