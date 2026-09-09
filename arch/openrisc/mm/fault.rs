// SPDX-License-Identifier: GPL-2.0-or-later
/* OpenRISC fault.c */

const NUM_TLB_ENTRIES: usize = 64;
#[inline]
fn tlb_offset(add: usize, page_shift: usize) -> usize { (add >> page_shift) & (NUM_TLB_ENTRIES - 1) }

extern "C" {
    static mut current: *mut task_struct;
    static mut current_pgd: *mut *mut pgd_t;
    static mut init_mm: mm_struct;
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn local_irq_enable();
    fn in_interrupt() -> bool;
    fn perf_sw_event(event: u32, count: u64, regs: *mut pt_regs, address: usize);
    fn mmap_read_lock(mm: *mut mm_struct);
    fn mmap_read_unlock(mm: *mut mm_struct);
    fn find_vma(mm: *mut mm_struct, address: usize) -> *mut vm_area_struct;
    fn expand_stack(mm: *mut mm_struct, address: usize) -> *mut vm_area_struct;
    fn handle_mm_fault(vma: *mut vm_area_struct, address: usize, flags: u32, regs: *mut pt_regs) -> u32;
    fn fault_signal_pending(fault: u32, regs: *mut pt_regs) -> bool;
    fn force_sig_fault(sig: i32, code: i32, address: *mut core::ffi::c_void);
    fn search_exception_tables(pc: usize) -> *const exception_table_entry;
    fn pagefault_out_of_memory();
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn die(name: *const core::ffi::c_char, regs: *mut pt_regs, write_acc: i32) -> !;
    fn pgd_index(address: usize) -> isize;
    fn p4d_offset(pgd: *mut pgd_t, address: usize) -> *mut p4d_t;
    fn pud_offset(p4d: *mut p4d_t, address: usize) -> *mut pud_t;
    fn pmd_offset(pud: *mut pud_t, address: usize) -> *mut pmd_t;
    fn pte_offset_kernel(pmd: *mut pmd_t, address: usize) -> *mut pte_t;
    fn p4d_present(p4d: p4d_t) -> bool;
    fn pud_present(pud: pud_t) -> bool;
    fn pmd_present(pmd: pmd_t) -> bool;
    fn pte_present(pte: pte_t) -> bool;
    fn set_pmd(dst: *mut pmd_t, src: pmd_t);
    fn smp_processor_id() -> usize;
}

extern "C" {
    type pt_regs;
    type task_struct;
    type mm_struct;
    type vm_area_struct;
    type pgd_t;
    type p4d_t;
    type pud_t;
    type pmd_t;
    type pte_t;
    type exception_table_entry;
}

const VMALLOC_START: usize = 0;
const PAGE_SIZE: usize = 0;
const PAGE_SHIFT: usize = 0;
const FAULT_FLAG_DEFAULT: u32 = 0;
const FAULT_FLAG_USER: u32 = 1;
const FAULT_FLAG_WRITE: u32 = 2;
const FAULT_FLAG_TRIED: u32 = 4;
const VM_GROWSDOWN: usize = 0;
const VM_WRITE: usize = 0;
const VM_READ: usize = 0;
const VM_EXEC: usize = 0;
const _PAGE_EXEC: usize = 0;
const VM_FAULT_COMPLETED: u32 = 0;
const VM_FAULT_ERROR: u32 = 0;
const VM_FAULT_OOM: u32 = 0;
const VM_FAULT_SIGSEGV: u32 = 0;
const VM_FAULT_SIGBUS: u32 = 0;
const VM_FAULT_RETRY: u32 = 0;
const SEGV_MAPERR: i32 = 0;
const SEGV_ACCERR: i32 = 0;
const SIGSEGV: i32 = 0;
const SIGBUS: i32 = 0;
const BUS_ADRERR: i32 = 0;

#[repr(C)]
pub struct pt_regs_fields { pub sr: usize, pub sp: usize, pub pc: usize }

#[no_mangle]
pub unsafe extern "C" fn do_page_fault(regs: *mut pt_regs, address: usize, vector: usize, write_acc: i32) {
    let tsk = current;
    let mut flags = FAULT_FLAG_DEFAULT;

    if address >= VMALLOC_START && vector != 0x300 && vector != 0x400 && !user_mode(regs) {
        let offset = pgd_index(address);
        let pgd = current_pgd.add(smp_processor_id()).read().offset(offset);
        let pgd_k = (&mut init_mm.pgd as *mut pgd_t).offset(offset);
        let p4d = p4d_offset(pgd, address);
        let p4d_k = p4d_offset(pgd_k, address);
        if !p4d_present(p4d_k.read()) { return no_context(regs, write_acc); }
        let pud = pud_offset(p4d, address);
        let pud_k = pud_offset(p4d_k, address);
        if !pud_present(pud_k.read()) { return no_context(regs, write_acc); }
        let pmd = pmd_offset(pud, address);
        let pmd_k = pmd_offset(pud_k, address);
        if !pmd_present(pmd_k.read()) { return bad_area_nosemaphore(regs, address, SEGV_MAPERR, write_acc); }
        set_pmd(pmd, pmd_k.read());
        let pte_k = pte_offset_kernel(pmd_k, address);
        if !pte_present(pte_k.read()) { return no_context(regs, write_acc); }
        return;
    }

    if user_mode(regs) { local_irq_enable(); flags |= FAULT_FLAG_USER; }
    else if (*(regs as *mut pt_regs_fields)).sr != 0 { local_irq_enable(); }
    let mm = (*tsk).mm;
    let mut si_code = SEGV_MAPERR;
    if in_interrupt() || mm.is_null() { return no_context(regs, write_acc); }
    perf_sw_event(0, 1, regs, address);

    'retry: loop {
        mmap_read_lock(mm);
        let vma = find_vma(mm, address);
        if vma.is_null() { mmap_read_unlock(mm); return bad_area_nosemaphore(regs, address, si_code, write_acc); }
        let v = &mut *vma;
        if v.vm_start > address {
            if v.vm_flags & VM_GROWSDOWN == 0 || (user_mode(regs) && address + PAGE_SIZE < (*(regs as *mut pt_regs_fields)).sp) {
                mmap_read_unlock(mm); return bad_area_nosemaphore(regs, address, si_code, write_acc);
            }
            if expand_stack(mm, address).is_null() { mmap_read_unlock(mm); return bad_area_nosemaphore(regs, address, si_code, write_acc); }
        }
        si_code = SEGV_ACCERR;
        if write_acc != 0 { if v.vm_flags & VM_WRITE == 0 { mmap_read_unlock(mm); return bad_area_nosemaphore(regs, address, si_code, write_acc); } flags |= FAULT_FLAG_WRITE; }
        else if v.vm_flags & (VM_READ | VM_EXEC) == 0 { mmap_read_unlock(mm); return bad_area_nosemaphore(regs, address, si_code, write_acc); }
        if vector == 0x400 && v.vm_page_prot.pgprot & _PAGE_EXEC == 0 { mmap_read_unlock(mm); return bad_area_nosemaphore(regs, address, si_code, write_acc); }
        let fault = handle_mm_fault(vma, address, flags, regs);
        if fault_signal_pending(fault, regs) { if !user_mode(regs) { return no_context(regs, write_acc); } return; }
        if fault & VM_FAULT_COMPLETED != 0 { return; }
        if fault & VM_FAULT_ERROR != 0 { if fault & VM_FAULT_OOM != 0 { mmap_read_unlock(mm); if !user_mode(regs) { return no_context(regs, write_acc); } pagefault_out_of_memory(); return; } if fault & VM_FAULT_SIGSEGV != 0 { mmap_read_unlock(mm); return bad_area_nosemaphore(regs, address, si_code, write_acc); } if fault & VM_FAULT_SIGBUS != 0 { mmap_read_unlock(mm); force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut _); if !user_mode(regs) { return no_context(regs, write_acc); } return; } }
        if fault & VM_FAULT_RETRY != 0 { flags |= FAULT_FLAG_TRIED; continue 'retry; }
        mmap_read_unlock(mm); return;
    }
}

unsafe fn bad_area_nosemaphore(regs: *mut pt_regs, address: usize, si_code: i32, write_acc: i32) {
    if user_mode(regs) { force_sig_fault(SIGSEGV, si_code, address as *mut _); return; }
    no_context(regs, write_acc);
}

unsafe fn no_context(regs: *mut pt_regs, write_acc: i32) -> ! {
    let entry = search_exception_tables((*(regs as *mut pt_regs_fields)).pc);
    if !entry.is_null() { (*(regs as *mut pt_regs_fields)).pc = *(entry as *const usize); return; }
    if 0 < PAGE_SIZE { printk(b"Unable to handle kernel NULL pointer dereference\0".as_ptr() as *const _); }
    else { printk(b"Unable to handle kernel access\0".as_ptr() as *const _); }
    printk(b" at virtual address 0x%08lx\n\0".as_ptr() as *const _, 0usize);
    die(b"Oops\0".as_ptr() as *const _, regs, write_acc)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
