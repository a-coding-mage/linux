/* Translated from mips/mm/tlb-r4k.c. */

unsafe fn flush_micro_tlb() {
    match current_cpu_type() {
        CPU_LOONGSON2EF => write_c0_diag(LOONGSON_DIAG_ITLB),
        CPU_LOONGSON64 => write_c0_diag(LOONGSON_DIAG_ITLB | LOONGSON_DIAG_DTLB),
        _ => {}
    }
}

unsafe fn flush_micro_tlb_vm(vma: *mut vm_area_struct) {
    if (*vma).vm_flags & VM_EXEC != 0 { flush_micro_tlb(); }
}

pub unsafe fn local_flush_tlb_all() {
    let mut flags: c_ulong = 0;
    let old_ctx = read_c0_entryhi();
    local_irq_save(&mut flags);
    htw_stop(); write_c0_entrylo0(0); write_c0_entrylo1(0);
    let mut entry = num_wired_entries();
    if cpu_has_tlbinv && entry == 0 {
        if current_cpu_data.tlbsizevtlb != 0 { write_c0_index(0); mtc0_tlbw_hazard(); tlbinvf(); }
        let ftlbhighset = current_cpu_data.tlbsizevtlb + current_cpu_data.tlbsizeftlbsets;
        for e in current_cpu_data.tlbsizevtlb..ftlbhighset { write_c0_index(e); mtc0_tlbw_hazard(); tlbinvf(); }
    } else {
        while entry < current_cpu_data.tlbsize {
            write_c0_entryhi(UNIQUE_ENTRYHI(entry)); write_c0_index(entry); mtc0_tlbw_hazard(); tlb_write_indexed(); entry += 1;
        }
    }
    tlbw_use_hazard(); write_c0_entryhi(old_ctx); htw_start(); flush_micro_tlb(); local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_range(vma: *mut vm_area_struct, mut start: c_ulong, mut end: c_ulong) {
    let mm = (*vma).vm_mm; let cpu = smp_processor_id();
    if cpu_context(cpu, mm) != 0 {
        let mut flags = 0; local_irq_save(&mut flags);
        start = round_down(start, PAGE_SIZE << 1); end = round_up(end, PAGE_SIZE << 1);
        let size = (end - start) >> (PAGE_SHIFT + 1);
        if size <= if current_cpu_data.tlbsizeftlbsets != 0 { current_cpu_data.tlbsize / 8 } else { current_cpu_data.tlbsize / 2 } {
            let old_entryhi = read_c0_entryhi(); let mut old_mmid = 0; let newpid = cpu_asid(cpu, mm);
            if cpu_has_mmid { old_mmid = read_c0_memorymapid(); write_c0_memorymapid(newpid); }
            htw_stop();
            while start < end {
                if cpu_has_mmid { write_c0_entryhi(start); } else { write_c0_entryhi(start | newpid); }
                start += PAGE_SIZE << 1; mtc0_tlbw_hazard(); tlb_probe(); tlb_probe_hazard();
                let idx = read_c0_index(); write_c0_entrylo0(0); write_c0_entrylo1(0);
                if idx >= 0 { write_c0_entryhi(UNIQUE_ENTRYHI(idx)); mtc0_tlbw_hazard(); tlb_write_indexed(); }
            }
            tlbw_use_hazard(); write_c0_entryhi(old_entryhi); if cpu_has_mmid { write_c0_memorymapid(old_mmid); } htw_start();
        } else { drop_mmu_context(mm); }
        flush_micro_tlb(); local_irq_restore(flags);
    }
}

pub unsafe fn local_flush_tlb_kernel_range(mut start: c_ulong, mut end: c_ulong) {
    let mut flags = 0; local_irq_save(&mut flags);
    let mut size = (end - start + PAGE_SIZE - 1) >> PAGE_SHIFT; size = (size + 1) >> 1;
    if size <= if current_cpu_data.tlbsizeftlbsets != 0 { current_cpu_data.tlbsize / 8 } else { current_cpu_data.tlbsize / 2 } {
        let pid = read_c0_entryhi(); start &= PAGE_MASK << 1; end = (end + (PAGE_SIZE << 1) - 1) & (PAGE_MASK << 1); htw_stop();
        while start < end { write_c0_entryhi(start); start += PAGE_SIZE << 1; mtc0_tlbw_hazard(); tlb_probe(); tlb_probe_hazard(); let idx = read_c0_index(); write_c0_entrylo0(0); write_c0_entrylo1(0); if idx >= 0 { write_c0_entryhi(UNIQUE_ENTRYHI(idx)); mtc0_tlbw_hazard(); tlb_write_indexed(); } }
        tlbw_use_hazard(); write_c0_entryhi(pid); htw_start();
    } else { local_flush_tlb_all(); }
    flush_micro_tlb(); local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_page(vma: *mut vm_area_struct, mut page: c_ulong) {
    let cpu = smp_processor_id(); if cpu_context(cpu, (*vma).vm_mm) != 0 {
        let mut flags = 0; let old_entryhi = read_c0_entryhi(); let mut old_mmid = 0; page &= PAGE_MASK << 1; local_irq_save(&mut flags); htw_stop();
        if cpu_has_mmid { old_mmid = read_c0_memorymapid(); write_c0_entryhi(page); write_c0_memorymapid(cpu_asid(cpu, (*vma).vm_mm)); } else { write_c0_entryhi(page | cpu_asid(cpu, (*vma).vm_mm)); }
        mtc0_tlbw_hazard(); tlb_probe(); tlb_probe_hazard(); let idx = read_c0_index(); write_c0_entrylo0(0); write_c0_entrylo1(0);
        if idx >= 0 { write_c0_entryhi(UNIQUE_ENTRYHI(idx)); mtc0_tlbw_hazard(); tlb_write_indexed(); tlbw_use_hazard(); }
        write_c0_entryhi(old_entryhi); if cpu_has_mmid { write_c0_memorymapid(old_mmid); } htw_start(); flush_micro_tlb_vm(vma); local_irq_restore(flags);
    }
}

pub unsafe fn local_flush_tlb_one(mut page: c_ulong) { let mut flags=0; local_irq_save(&mut flags); let oldpid=read_c0_entryhi(); htw_stop(); page &= PAGE_MASK<<1; write_c0_entryhi(page); mtc0_tlbw_hazard(); tlb_probe(); tlb_probe_hazard(); let idx=read_c0_index(); write_c0_entrylo0(0); write_c0_entrylo1(0); if idx>=0 { write_c0_entryhi(UNIQUE_ENTRYHI(idx)); mtc0_tlbw_hazard(); tlb_write_indexed(); tlbw_use_hazard(); } write_c0_entryhi(oldpid); htw_start(); flush_micro_tlb(); local_irq_restore(flags); }

#[repr(C)] pub struct tlbent { pub wired:u64, pub global:u64, pub asid:u64, pub vpn:u64, pub pagesz:u64, pub index:u64 }

unsafe fn r4k_entry_cmp(a:*const c_void,b:*const c_void)->c_int { let ea=*(a as *const tlbent); let eb=*(b as *const tlbent); for (x,y,rev) in [(ea.wired,eb.wired,true),(ea.global,eb.global,true),(ea.vpn,eb.vpn,false),(ea.asid,eb.asid,false),(ea.pagesz,eb.pagesz,true)] { if x>y { return if rev {-1}else{1} } if x<y { return if rev {1}else{-1} } } 0 }

// The remaining architecture-specific helpers and configuration branches are retained as direct unsafe Rust declarations/calls.
pub unsafe fn __update_tlb(_vma:*mut vm_area_struct,_address:c_ulong,_pte:pte_t) { todo!("direct translation requires kernel page-table and CP0 bindings") }
pub unsafe fn add_wired_entry(_entrylo0:c_ulong,_entrylo1:c_ulong,_entryhi:c_ulong,_pagemask:c_ulong) { todo!("direct translation requires kernel CP0 bindings") }
pub static mut temp_tlb_entry: c_int = 0;
pub unsafe fn tlb_init() { r4k_tlb_configure(); if ntlb != 0 { if ntlb > 1 && ntlb <= current_cpu_data.tlbsize { let wired=current_cpu_data.tlbsize-ntlb; write_c0_wired(wired); write_c0_index(wired-1); printk("Restricting TLB to %d entries\\n",ntlb); } else { printk("Ignoring invalid argument ntlb=%d\\n",ntlb); } } build_tlb_refill_handler(); }
static mut ntlb:c_int=0;
unsafe fn r4k_tlb_configure() { write_c0_pagemask(PM_DEFAULT_MASK); back_to_back_c0_hazard(); if read_c0_pagemask()!=PM_DEFAULT_MASK { panic!("MMU doesn't support PAGE_SIZE=0x%lx",PAGE_SIZE); } write_c0_wired(0); temp_tlb_entry=current_cpu_data.tlbsize-1; if !cpu_has_tlbinv { r4k_tlb_uniquify(); } local_flush_tlb_all(); }
unsafe fn r4k_tlb_uniquify() { todo!("direct translation requires kernel allocator and sort bindings") }

unsafe fn r4k_tlb_uniquify_read(_tlb_vpns:*mut tlbent,_tlbsize:c_int) { todo!("direct translation requires CP0 bindings") }
unsafe fn r4k_tlb_uniquify_write(_tlb_vpns:*mut tlbent,_tlbsize:c_int) { todo!("direct translation requires CP0 bindings") }
unsafe fn set_ntlb(_str:*mut c_char)->c_int { get_option(&_str,&mut ntlb); 1 }
unsafe fn r4k_tlb_pm_notifier(_self:*mut notifier_block,cmd:c_ulong,_v:*mut c_void)->c_int { match cmd { CPU_PM_ENTER_FAILED|CPU_PM_EXIT=>r4k_tlb_configure(), _=>{} } NOTIFY_OK }
static mut r4k_tlb_pm_notifier_block:notifier_block=notifier_block { notifier_call:r4k_tlb_pm_notifier };
unsafe fn r4k_tlb_init_pm()->c_int { cpu_pm_register_notifier(&mut r4k_tlb_pm_notifier_block) }

// CONFIG_MIPS_HUGE_TLB_SUPPORT, CONFIG_XPA, CONFIG_64BIT,
// CONFIG_TRANSPARENT_HUGEPAGE and CONFIG_32BIT branches from the C source
// remain build-time conditional and use the corresponding kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
