/* SPDX-License-Identifier: GPL-2.0 */
// Translated from asm/desc.h. C includes and configuration dependencies are
// supplied by the surrounding kernel translation unit.

#[inline]
pub unsafe fn fill_ldt(desc: *mut desc_struct, info: *const user_desc) {
    (*desc).limit0 = (*info).limit & 0x0ffff;
    (*desc).base0 = (*info).base_addr & 0x0000ffff;
    (*desc).base1 = ((*info).base_addr & 0x00ff0000) >> 16;
    (*desc).type_ = ((*info).read_exec_only ^ 1) << 1;
    (*desc).type_ |= (*info).contents << 2;
    /* Set the ACCESS bit so it can be mapped RO */
    (*desc).type_ |= 1;
    (*desc).s = 1;
    (*desc).dpl = 0x3;
    (*desc).p = (*info).seg_not_present ^ 1;
    (*desc).limit1 = ((*info).limit & 0xf0000) >> 16;
    (*desc).avl = (*info).useable;
    (*desc).d = (*info).seg_32bit;
    (*desc).g = (*info).limit_in_pages;
    (*desc).base2 = ((*info).base_addr & 0xff000000) >> 24;
    /* Don't allow setting of the lm bit. */
    (*desc).l = 0;
}

#[repr(C, align(4096))]
pub struct gdt_page { pub gdt: [desc_struct; GDT_ENTRIES] }

extern "C" {
    pub static mut gdt_page_per_cpu: gdt_page;
}

#[inline] pub unsafe fn get_cpu_gdt_rw(cpu: c_uint) -> *mut desc_struct { per_cpu!(gdt_page_per_cpu, cpu).gdt.as_mut_ptr() }
#[inline] pub unsafe fn get_current_gdt_rw() -> *mut desc_struct { this_cpu_ptr!(&mut gdt_page_per_cpu).gdt.as_mut_ptr() }
#[inline] pub unsafe fn get_cpu_gdt_ro(cpu: c_int) -> *mut desc_struct { (&mut (*get_cpu_entry_area(cpu)).gdt as *mut _) as *mut desc_struct }
#[inline] pub unsafe fn get_current_gdt_ro() -> *mut desc_struct { get_cpu_gdt_ro(smp_processor_id()) }
#[inline] pub unsafe fn get_cpu_gdt_paddr(cpu: c_uint) -> phys_addr_t { per_cpu_ptr_to_phys(get_cpu_gdt_rw(cpu)) }

#[inline]
pub unsafe fn pack_gate(gate: *mut gate_desc, type_: c_uint, func: c_ulong, dpl: c_uint, ist: c_uint, seg: c_uint) {
    (*gate).offset_low = func as u16; (*gate).bits.p = 1; (*gate).bits.dpl = dpl;
    (*gate).bits.zero = 0; (*gate).bits.type_ = type_; (*gate).offset_middle = (func >> 16) as u16;
    #[cfg(target_arch = "x86_64")] { (*gate).segment = __KERNEL_CS; (*gate).bits.ist = ist; (*gate).reserved = 0; (*gate).offset_high = (func >> 32) as u32; }
    #[cfg(not(target_arch = "x86_64"))] { (*gate).segment = seg; (*gate).bits.ist = 0; }
}

#[inline] pub unsafe fn desc_empty(ptr: *const c_void) -> bool { let desc = ptr as *const u32; ((*desc) | *desc.add(1)) == 0 }

#[inline] pub unsafe fn paravirt_alloc_ldt(_ldt: *mut desc_struct, _entries: c_uint) {}
#[inline] pub unsafe fn paravirt_free_ldt(_ldt: *mut desc_struct, _entries: c_uint) {}

#[inline] pub unsafe fn native_write_idt_entry(idt: *mut gate_desc, entry: c_int, gate: *const gate_desc) { core::ptr::copy_nonoverlapping(gate, idt.offset(entry as isize), 1); }
#[inline] pub unsafe fn native_write_ldt_entry(ldt: *mut desc_struct, entry: c_int, desc: *const c_void) { core::ptr::copy_nonoverlapping(desc as *const u8, ldt.offset(entry as isize) as *mut u8, 8); }

#[inline] pub unsafe fn native_write_gdt_entry(gdt: *mut desc_struct, entry: c_int, desc: *const c_void, type_: c_int) {
    let size = match type_ { DESC_TSS => core::mem::size_of::<tss_desc>(), DESC_LDT => core::mem::size_of::<ldt_desc>(), _ => core::mem::size_of::<desc_struct>() };
    core::ptr::copy_nonoverlapping(desc as *const u8, gdt.offset(entry as isize) as *mut u8, size);
}

#[inline]
pub unsafe fn set_tssldt_descriptor(d: *mut c_void, addr: c_ulong, type_: c_uint, size: c_uint) {
    let desc = d as *mut ldttss_desc; core::ptr::write_bytes(desc, 0, 1);
    (*desc).limit0 = size as u16; (*desc).base0 = addr as u16; (*desc).base1 = ((addr >> 16) & 0xff) as u8;
    (*desc).type_ = type_; (*desc).p = 1; (*desc).limit1 = ((size >> 16) & 0xf) as u8; (*desc).base2 = ((addr >> 24) & 0xff) as u8;
    #[cfg(target_arch = "x86_64")] { (*desc).base3 = (addr >> 32) as u32; }
}

#[inline] pub unsafe fn __set_tss_desc(cpu: c_uint, entry: c_uint, addr: *mut x86_hw_tss) { let d = get_cpu_gdt_rw(cpu); let mut tss: tss_desc = core::mem::zeroed(); set_tssldt_descriptor(&mut tss as *mut _ as *mut c_void, addr as c_ulong, DESC_TSS, __KERNEL_TSS_LIMIT); native_write_gdt_entry(d, entry as c_int, &tss as *const _ as *const c_void, DESC_TSS as c_int); }
#[inline] pub unsafe fn set_tss_desc(cpu: c_uint, addr: *mut x86_hw_tss) { __set_tss_desc(cpu, GDT_ENTRY_TSS, addr) }

#[inline] pub unsafe fn native_load_gdt(dtr: *const desc_ptr) { core::arch::asm!("lgdt [{}]", in(reg) dtr, options(nostack)); }
#[inline] pub unsafe fn native_load_idt(dtr: *const desc_ptr) { core::arch::asm!("lidt [{}]", in(reg) dtr, options(nostack)); }
#[inline] pub unsafe fn native_store_gdt(dtr: *mut desc_ptr) { core::arch::asm!("sgdt [{}]", in(reg) dtr, options(nostack)); }
#[inline] pub unsafe fn store_idt(dtr: *mut desc_ptr) { core::arch::asm!("sidt [{}]", in(reg) dtr, options(nostack)); }

#[inline] pub unsafe fn native_gdt_invalidate() { let invalid = desc_ptr { address: 0, size: 0 }; native_load_gdt(&invalid); }
#[inline] pub unsafe fn native_idt_invalidate() { let invalid = desc_ptr { address: 0, size: 0 }; native_load_idt(&invalid); }

#[cfg(target_arch = "x86_64")]
#[inline] pub unsafe fn native_load_tr_desc() { let mut gdt: desc_ptr = core::mem::zeroed(); let cpu = raw_smp_processor_id(); native_store_gdt(&mut gdt); let fixmap_gdt = get_cpu_gdt_ro(cpu); let mut restore = false; if gdt.address == fixmap_gdt as c_ulong { load_direct_gdt(cpu); restore = true; } core::arch::asm!("ltr {0:x}", in(reg) (GDT_ENTRY_TSS * 8)); if restore { load_fixmap_gdt(cpu); } }
#[cfg(not(target_arch = "x86_64"))]
#[inline] pub unsafe fn native_load_tr_desc() { core::arch::asm!("ltr {0:x}", in(reg) (GDT_ENTRY_TSS * 8)); }

#[inline] pub unsafe fn native_store_tr() -> c_ulong { let tr: c_ulong; core::arch::asm!("str {0}", out(reg) tr); tr }
#[inline] pub unsafe fn native_load_tls(t: *mut thread_struct, cpu: c_uint) { let gdt = get_cpu_gdt_rw(cpu); for i in 0..GDT_ENTRY_TLS_ENTRIES { *gdt.add(GDT_ENTRY_TLS_MIN + i) = (*t).tls_array[i]; } }

extern "C" { pub static mut __tss_limit_invalid: bool; }
#[inline] pub unsafe fn force_reload_TR() { let d = get_current_gdt_rw(); let mut tss: tss_desc = core::mem::zeroed(); core::ptr::copy_nonoverlapping(d.add(GDT_ENTRY_TSS) as *const u8, &mut tss as *mut _ as *mut u8, core::mem::size_of::<tss_desc>()); tss.type_ = DESC_TSS; native_write_gdt_entry(d, GDT_ENTRY_TSS as c_int, &tss as *const _ as *const c_void, DESC_TSS as c_int); native_load_tr_desc(); this_cpu_write!(__tss_limit_invalid, false); }
#[inline] pub unsafe fn refresh_tss_limit() { debug_locks_warn_on!(preemptible()); if unlikely!(this_cpu_read!(__tss_limit_invalid)) { force_reload_TR(); } }
#[inline] pub unsafe fn invalidate_tss_limit() { debug_locks_warn_on!(preemptible()); if unlikely!(test_thread_flag!(TIF_IO_BITMAP)) { force_reload_TR(); } else { this_cpu_write!(__tss_limit_invalid, true); } }

#[inline] pub unsafe fn LDT_empty(info: *const user_desc) -> bool { (*info).base_addr == 0 && (*info).limit == 0 && (*info).contents == 0 && (*info).read_exec_only == 1 && (*info).seg_32bit == 0 && (*info).limit_in_pages == 0 && (*info).seg_not_present == 1 && (*info).useable == 0 }
#[inline] pub unsafe fn LDT_zero(info: *const user_desc) -> bool { (*info).base_addr == 0 && (*info).limit == 0 && (*info).contents == 0 && (*info).read_exec_only == 0 && (*info).seg_32bit == 0 && (*info).limit_in_pages == 0 && (*info).seg_not_present == 0 && (*info).useable == 0 }
#[inline] pub unsafe fn clear_LDT() { set_ldt(core::ptr::null(), 0); }
#[inline] pub unsafe fn get_desc_base(desc: *const desc_struct) -> c_ulong { ((*desc).base0 | ((*desc).base1 << 16) | ((*desc).base2 << 24)) as c_uint as c_ulong }
#[inline] pub unsafe fn set_desc_base(desc: *mut desc_struct, base: c_ulong) { (*desc).base0 = base & 0xffff; (*desc).base1 = (base >> 16) & 0xff; (*desc).base2 = (base >> 24) & 0xff; }
#[inline] pub unsafe fn get_desc_limit(desc: *const desc_struct) -> c_ulong { ((*desc).limit0 | ((*desc).limit1 << 16)) as c_ulong }
#[inline] pub unsafe fn set_desc_limit(desc: *mut desc_struct, limit: c_ulong) { (*desc).limit0 = limit & 0xffff; (*desc).limit1 = (limit >> 16) & 0xf; }

#[inline] pub unsafe fn init_idt_data(data: *mut idt_data, n: c_uint, addr: *const c_void) { BUG_ON!(n > 0xff); core::ptr::write_bytes(data, 0, 1); (*data).vector = n; (*data).addr = addr; (*data).segment = __KERNEL_CS; (*data).bits.type_ = GATE_INTERRUPT; (*data).bits.p = 1; }
#[inline] pub unsafe fn idt_init_desc(gate: *mut gate_desc, d: *const idt_data) { let addr = (*d).addr as c_ulong; (*gate).offset_low = addr as u16; (*gate).segment = (*d).segment as u16; (*gate).bits = (*d).bits; (*gate).offset_middle = (addr >> 16) as u16; #[cfg(target_arch = "x86_64")] { (*gate).offset_high = (addr >> 32) as u32; (*gate).reserved = 0; } }

extern "C" {
    pub static mut system_vectors: [c_ulong; NR_VECTORS];
    pub fn load_current_idt(); pub fn idt_setup_early_handler(); pub fn idt_setup_early_traps(); pub fn idt_setup_traps(); pub fn idt_setup_apic_and_irq_gates(); pub fn idt_is_f00f_address(address: c_ulong) -> bool;
    pub fn idt_do_interrupt_irqoff(address: c_ulong); pub fn idt_do_nmi_irqoff(); pub fn idt_entry_from_kvm(vector: c_uint); pub fn idt_invalidate();
    #[cfg(target_arch = "x86_64")] pub fn idt_setup_early_pf();
}

#[cfg(not(target_arch = "x86_64"))]
#[inline] pub unsafe fn idt_setup_early_pf() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
