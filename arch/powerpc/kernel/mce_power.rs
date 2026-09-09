// SPDX-License-Identifier: GPL-2.0-or-later
/* Machine check exception handling CPU-side for power7 and power8. */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct MceIerrorTable {
    pub srr1_mask: u64, pub srr1_value: u64, pub nip_valid: bool,
    pub error_type: u32, pub error_subtype: u32, pub error_class: u32,
    pub initiator: u32, pub severity: u32, pub sync_error: bool,
}
#[repr(C)]
pub struct MceDerrorTable {
    pub dsisr_value: u64, pub dar_valid: bool, pub error_type: u32,
    pub error_subtype: u32, pub error_class: u32, pub initiator: u32,
    pub severity: u32, pub sync_error: bool,
}

extern "C" {
    fn user_mode(regs: *const PtRegs) -> bool;
    fn mce_common_process_ue(regs: *mut PtRegs, err: *mut MceErrorInfo);
    fn save_mce_event(regs: *mut PtRegs, handled: i64, err: *mut MceErrorInfo,
                      nip: u64, addr: u64, phys_addr: u64);
    fn current_mm_pgd() -> *mut core::ffi::c_void;
    fn init_mm_pgd() -> *mut core::ffi::c_void;
    fn local_irq_save(flags: *mut u64);
    fn local_irq_restore(flags: u64);
    fn find_linux_pte(pgd: *mut core::ffi::c_void, addr: u64, shift: *mut u32) -> *mut u64;
    fn mce_find_instr_ea_and_phys(regs: *mut PtRegs, addr: *mut u64, phys: *mut u64) -> i32;
}

#[repr(C)] pub struct PtRegs { pub nip: u64, pub msr: u64, pub dsisr: u64, pub dar: u64 }
#[repr(C)] pub struct MceErrorInfo {
    pub error_type: u32, pub error_class: u32, pub sync_error: bool,
    pub severity: u32, pub initiator: u32, pub ignore_event: bool,
    pub u: MceErrorUnion,
}
#[repr(C)] pub union MceErrorUnion {
    pub ue_error_type: u32, pub slb_error_type: u32, pub erat_error_type: u32,
    pub tlb_error_type: u32, pub user_error_type: u32, pub ra_error_type: u32,
    pub link_error_type: u32,
}

const MCE_FLUSH_SLB: i32 = 1;
const MCE_FLUSH_TLB: i32 = 2;
const MCE_FLUSH_ERAT: i32 = 3;
const PAGE_SHIFT: u64 = 12;
const ULONG_MAX: u64 = u64::MAX;
const PPC_BIT_42: u64 = 1u64 << (63 - 42);
#[inline] fn srr1_mc_loadstore(srr1: u64) -> bool { (srr1 & PPC_BIT_42) != 0 }

/* The constants below are supplied by asm/mce.h in the kernel build. */
extern "C" {
    static mut ppc_md_mce_check_early_recovery: Option<unsafe extern "C" fn(*mut PtRegs) -> i32>;
}

pub unsafe fn addr_to_pfn(regs: *mut PtRegs, addr: u64) -> u64 {
    let mut flags=0; let pgd=if user_mode(regs){current_mm_pgd()}else{init_mm_pgd()}; local_irq_save(&mut flags);
    let mut shift=0; let p=find_linux_pte(pgd,addr,&mut shift); let result=if p.is_null(){ULONG_MAX}else{(*p >> PAGE_SHIFT) | ((addr & ((1u64<<shift)-4096)) >> PAGE_SHIFT)}; local_irq_restore(flags); result
}

pub unsafe fn flush_and_reload_slb() { /* CONFIG_PPC_64S_HASH_MMU implementation is supplied by the platform. */ }
pub unsafe fn flush_erat() { /* platform ERAT invalidation instruction */ }
pub unsafe fn mce_flush(what:i32)->i32 { match what { MCE_FLUSH_SLB=>{flush_and_reload_slb();1}, MCE_FLUSH_ERAT=>{flush_erat();1}, MCE_FLUSH_TLB=>{1}, _=>0 } }
pub unsafe fn mce_in_guest()->bool { false }

unsafe fn mce_handle_ue_error(regs:*mut PtRegs, err:*mut MceErrorInfo)->i64 {
    if mce_in_guest(){return 0;} mce_common_process_ue(regs,err); if (*err).ignore_event{return 1;}
    if let Some(f)=ppc_md_mce_check_early_recovery { if f(regs)!=0{return 1;} } 0
}

// Tables are kept in their source order; symbolic MCE_* constants are external dependencies.
extern "C" {
    static mce_p7_ierror_table: [MceIerrorTable; 8];
    static mce_p8_ierror_table: [MceIerrorTable; 10];
    static mce_p9_ierror_table: [MceIerrorTable; 13];
    static mce_p10_ierror_table: [MceIerrorTable; 13];
    static mce_p7_derror_table: [MceDerrorTable; 8];
    static mce_p8_derror_table: [MceDerrorTable; 10];
    static mce_p9_derror_table: [MceDerrorTable; 14];
    static mce_p10_derror_table: [MceDerrorTable; 12];
}

unsafe fn handle_ierror(regs: *mut PtRegs, srr1: u64, table: *const MceIerrorTable,
                        err: *mut MceErrorInfo, addr: *mut u64, phys: *mut u64) -> i32 {
    *addr = 0;
    let mut i = 0;
    while (*table.add(i)).srr1_mask != 0 {
        let t = &*table.add(i);
        if (srr1 & t.srr1_mask) != t.srr1_value { i += 1; continue; }
        let mut handled = 0;
        if !mce_in_guest() {
            match t.error_type { 1 => handled = mce_flush(MCE_FLUSH_SLB), 3 => handled = mce_flush(MCE_FLUSH_ERAT), 4 => handled = mce_flush(MCE_FLUSH_TLB), _ => {} }
        }
        (*err).error_type = t.error_type; (*err).error_class = t.error_class;
        (*err).sync_error = t.sync_error; (*err).severity = t.severity; (*err).initiator = t.initiator;
        if t.nip_valid && !mce_in_guest() { *addr = (*regs).nip; if t.sync_error && t.error_type == 0 && !mce_in_guest() { let p = addr_to_pfn(regs, (*regs).nip); if p != ULONG_MAX { *phys = p << PAGE_SHIFT; } } }
        return handled;
    }
    (*err).error_type = 0; (*err).error_class = 0; (*err).severity = 3; (*err).initiator = 0; (*err).sync_error = true; 0
}

unsafe fn handle_derror(regs: *mut PtRegs, table: *const MceDerrorTable,
                        err: *mut MceErrorInfo, addr: *mut u64, phys: *mut u64) -> i32 {
    let dsisr = (*regs).dsisr; let mut handled = 0; let mut found = false; *addr = 0; let mut i = 0;
    while (*table.add(i)).dsisr_value != 0 {
        let t = &*table.add(i); if dsisr & t.dsisr_value == 0 { i += 1; continue; }
        if !mce_in_guest() { match t.error_type { 1 => if mce_flush(MCE_FLUSH_SLB) != 0 { handled=1 }, 3 => if mce_flush(MCE_FLUSH_ERAT) != 0 { handled=1 }, 4 => if mce_flush(MCE_FLUSH_TLB) != 0 { handled=1 }, _ => {} } }
        if !found { (*err).error_type=t.error_type; (*err).error_class=t.error_class; (*err).sync_error=t.sync_error; (*err).severity=t.severity; (*err).initiator=t.initiator; if t.dar_valid { *addr=(*regs).dar; } else if t.sync_error && !mce_in_guest() && t.error_type==0 { if (*regs).nip != 0 { let _=mce_find_instr_ea_and_phys(regs,addr,phys); } } found=true; }
        i += 1;
    }
    if found { handled } else { (*err).error_type=0; (*err).error_class=0; (*err).severity=3; (*err).initiator=0; (*err).sync_error=true; 0 }
}

unsafe fn handle_error(regs: *mut PtRegs, srr1: u64, dt: *const MceDerrorTable, it: *const MceIerrorTable) -> i64 {
    let mut err = core::mem::MaybeUninit::<MceErrorInfo>::zeroed().assume_init(); let mut addr=0; let mut phys=ULONG_MAX;
    let mut handled = if srr1_mc_loadstore(srr1) { handle_derror(regs,dt,&mut err,&mut addr,&mut phys) } else { handle_ierror(regs,srr1,it,&mut err,&mut addr,&mut phys) };
    if handled==0 && err.error_type==0 { handled=mce_handle_ue_error(regs,&mut err) as i32; }
    save_mce_event(regs,handled as i64,&mut err,(*regs).nip,addr,phys); handled as i64
}

pub unsafe fn __machine_check_early_realmode_p7(regs:*mut PtRegs)->i64 { (*regs).dsisr &= 0xffff; handle_error(regs,(*regs).msr,mce_p7_derror_table.as_ptr(),mce_p7_ierror_table.as_ptr()) }
pub unsafe fn __machine_check_early_realmode_p8(regs:*mut PtRegs)->i64 { handle_error(regs,(*regs).msr,mce_p8_derror_table.as_ptr(),mce_p8_ierror_table.as_ptr()) }
pub unsafe fn __machine_check_early_realmode_p9(regs:*mut PtRegs)->i64 { let mut s=(*regs).msr; if srr1_mc_loadstore(s)&&(*regs).dsisr==0x02000000{return 1} if srr1_mc_loadstore(s)&&((s&0x081c0000)==0x08140000||(s&0x081c0000)==0x08180000){s&=!PPC_BIT_42} handle_error(regs,s,mce_p9_derror_table.as_ptr(),mce_p9_ierror_table.as_ptr()) }
pub unsafe fn __machine_check_early_realmode_p10(regs:*mut PtRegs)->i64 { let mut s=(*regs).msr; if srr1_mc_loadstore(s)&&(s&0x081c0000)==0x08140000{s&=!PPC_BIT_42} handle_error(regs,s,mce_p10_derror_table.as_ptr(),mce_p10_ierror_table.as_ptr()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
