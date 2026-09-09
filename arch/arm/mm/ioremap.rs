// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/ioremap.c
 *
 * Re-map IO memory to kernel address space so that we can access it.
 *
 * (C) Copyright 1995 1996 Linus Torvalds
 *
 * Hacked for ARM by Phil Blundell <philb@gnu.org>
 * Hacked to allow all architectures to build, and various cleanups
 * by Russell King
 *
 * This allows a driver to remap an arbitrary region of bus memory into
 * virtual space.  One should *only* use readl, writel, memcpy_toio and
 * so on with such remapped areas.
 */

// Kernel declarations supplied by the corresponding Linux headers.

#[repr(C)]
pub struct static_vm { pub list: list_head, pub vm: vm_struct }
#[repr(C)] pub struct vm_struct { pub addr: *mut core::ffi::c_void, pub size: usize, pub phys_addr: phys_addr_t, pub flags: u64 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mem_type { pub prot_pte: u64, pub prot_sect: u64 }
#[repr(C)] pub struct mm_struct { pub context: mm_context }
#[repr(C)] pub struct mm_context { pub vmalloc_seq: atomic_t }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct pmd_t { pub val: u64 }
#[repr(C)] pub struct pgd_t { pub val: u64 }
#[repr(C)] pub struct resource { pub start: resource_size_t, pub end: resource_size_t, pub flags: u64 }

pub type phys_addr_t = u64;
pub type resource_size_t = u64;
pub type size_t = usize;

extern "C" {
    static mut static_vmlist: list_head;
    static mut init_mm: mm_struct;
    static mut current: *mut task_struct;
    static mut arch_ioremap_caller: Option<unsafe extern "C" fn(phys_addr_t, size_t, u32, *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
}
#[repr(C)] pub struct task_struct { pub active_mm: *mut mm_struct }

unsafe fn find_static_vm_paddr(paddr: phys_addr_t, size: size_t, mtype: u32) -> *mut static_vm {
    let mut svm: *mut static_vm;
    let mut vm: *mut vm_struct;
    list_for_each_entry!(svm, &mut static_vmlist, list) {
        vm = &mut (*svm).vm;
        if ((*vm).flags & VM_ARM_STATIC_MAPPING) == 0 { continue; }
        if ((*vm).flags & VM_ARM_MTYPE_MASK) != VM_ARM_MTYPE(mtype) { continue; }
        if (*vm).phys_addr > paddr || paddr + size - 1 > (*vm).phys_addr + (*vm).size as u64 - 1 { continue; }
        return svm;
    }
    core::ptr::null_mut()
}

pub unsafe extern "C" fn find_static_vm_vaddr(vaddr: *mut core::ffi::c_void) -> *mut static_vm {
    let mut svm: *mut static_vm;
    let mut vm: *mut vm_struct;
    list_for_each_entry!(svm, &mut static_vmlist, list) {
        vm = &mut (*svm).vm;
        if (*vm).addr > vaddr { break; }
        if (*vm).addr <= vaddr && ((*vm).addr as usize + (*vm).size > vaddr as usize) { return svm; }
    }
    core::ptr::null_mut()
}

pub unsafe extern "C" fn add_static_vm_early(svm: *mut static_vm) {
    let mut curr_svm: *mut static_vm;
    let vm = &mut (*svm).vm;
    vm_area_add_early(vm);
    let vaddr = vm.addr;
    list_for_each_entry!(curr_svm, &mut static_vmlist, list) {
        if (*(*curr_svm).vm).addr > vaddr { break; }
    }
    list_add_tail!(&mut (*svm).list, &mut (*curr_svm).list);
}

pub unsafe extern "C" fn ioremap_page(virt: u64, phys: u64, mtype: *const mem_type) -> i32 {
    vmap_page_range(virt, virt + PAGE_SIZE, phys, __pgprot((*mtype).prot_pte))
}

#[cfg(CONFIG_KASAN)]
unsafe fn arm_kasan_mem_to_shadow(addr: u64) -> u64 { kasan_mem_to_shadow(addr as *mut core::ffi::c_void) as u64 }
#[cfg(not(CONFIG_KASAN))]
unsafe fn arm_kasan_mem_to_shadow(_addr: u64) -> u64 { 0 }

unsafe fn memcpy_pgd(mm: *mut mm_struct, start: u64, mut end: u64) {
    end = ALIGN(end, PGDIR_SIZE);
    memcpy(pgd_offset(mm, start), pgd_offset_k(start), core::mem::size_of::<pgd_t>() * (pgd_index(end) - pgd_index(start)) as usize);
}

pub unsafe extern "C" fn __check_vmalloc_seq(mm: *mut mm_struct) {
    let mut seq: i32;
    loop {
        seq = atomic_read_acquire(&(*init_mm).context.vmalloc_seq);
        memcpy_pgd(mm, VMALLOC_START, VMALLOC_END);
        if IS_ENABLED!(CONFIG_KASAN_VMALLOC) {
            memcpy_pgd(mm, arm_kasan_mem_to_shadow(VMALLOC_START), arm_kasan_mem_to_shadow(VMALLOC_END));
        }
        atomic_set_release(&mut (*mm).context.vmalloc_seq, seq);
        if seq == atomic_read(&(*init_mm).context.vmalloc_seq) { break; }
    }
}

#[cfg(all(not(CONFIG_SMP), not(CONFIG_ARM_LPAE)))]
unsafe fn unmap_area_sections(virt: u64, size: u64) {
    let mut addr = virt; let end = virt + (size & !(SZ_1M - 1)); let mut pmdp = pmd_off_k(addr);
    loop {
        let pmd = *pmdp;
        if !pmd_none(pmd) {
            pmd_clear(pmdp); atomic_inc_return_release(&mut (*init_mm).context.vmalloc_seq);
            if (pmd_val(pmd) & PMD_TYPE_MASK) == PMD_TYPE_TABLE { pte_free_kernel(init_mm_ptr(), pmd_page_vaddr(pmd)); }
        }
        addr += PMD_SIZE; pmdp = pmdp.add(2); if addr >= end { break; }
    }
    check_vmalloc_seq((*current).active_mm); flush_tlb_kernel_range(virt, end);
}

#[cfg(all(not(CONFIG_SMP), not(CONFIG_ARM_LPAE)))]
unsafe fn remap_area_sections(virt: u64, mut pfn: u64, size: usize, typ: *const mem_type) -> i32 {
    let mut addr = virt; let end = virt + size as u64; let mut pmd = pmd_off_k(addr); unmap_area_sections(virt, size as u64);
    loop { *pmd = __pmd(__pfn_to_phys(pfn) | (*typ).prot_sect); pfn += SZ_1M >> PAGE_SHIFT; *pmd.add(1) = __pmd(__pfn_to_phys(pfn) | (*typ).prot_sect); pfn += SZ_1M >> PAGE_SHIFT; flush_pmd_entry(pmd); addr += PMD_SIZE; pmd = pmd.add(2); if addr >= end { break; } } 0
}

#[cfg(all(not(CONFIG_SMP), not(CONFIG_ARM_LPAE)))]
unsafe fn remap_area_supersections(virt: u64, mut pfn: u64, size: usize, typ: *const mem_type) -> i32 {
    let mut addr = virt; let end = virt + size as u64; let mut pmd = pmd_off_k(addr); unmap_area_sections(virt, size as u64);
    loop { let mut val = __pfn_to_phys(pfn) | (*typ).prot_sect | PMD_SECT_SUPER; val |= ((pfn >> (32 - PAGE_SHIFT)) & 0xf) << 20; for _ in 0..8 { *pmd = __pmd(val); *pmd.add(1) = __pmd(val); flush_pmd_entry(pmd); addr += PMD_SIZE; pmd = pmd.add(2); } pfn += SUPERSECTION_SIZE >> PAGE_SHIFT; if addr >= end { break; } } 0
}

pub unsafe extern "C" fn __arm_ioremap_pfn_caller(pfn: u64, offset: u64, mut size: usize, mtype: u32, caller: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let paddr = __pfn_to_phys(pfn); let typ = get_mem_type(mtype); if typ.is_null() { return core::ptr::null_mut(); }
    size = PAGE_ALIGN(offset as usize + size); let mut addr: u64; 
    if size != 0 && !(core::mem::size_of::<phys_addr_t>() == 4 && pfn >= 0x100000) { let svm = find_static_vm_paddr(paddr, size, mtype); if !svm.is_null() { addr = (*svm).vm.addr as u64 + paddr - (*svm).vm.phys_addr; return (offset + addr) as *mut _; } }
    if WARN_ON!(memblock_is_map_memory(PFN_PHYS(pfn)) && mtype != MT_MEMORY_RW) { return core::ptr::null_mut(); }
    let area = get_vm_area_caller(size, VM_IOREMAP, caller); if area.is_null() { return core::ptr::null_mut(); } addr = (*area).addr as u64; (*area).phys_addr = paddr;
    let err = if !CONFIG_ARM_LPAE && DOMAIN_IO == 0 && ((cpu_architecture() >= CPU_ARCH_ARMv6 && (get_cr() & CR_XP) != 0) || cpu_is_xsc3()) && pfn >= 0x100000 && ((paddr | size as u64 | addr) & !SUPERSECTION_MASK) == 0 { (*area).flags |= VM_ARM_SECTION_MAPPING; remap_area_supersections(addr, pfn, size, typ) } else if !CONFIG_ARM_LPAE && ((paddr | size as u64 | addr) & !PMD_MASK) == 0 { (*area).flags |= VM_ARM_SECTION_MAPPING; remap_area_sections(addr, pfn, size, typ) } else { ioremap_page_range(addr, addr + size as u64, paddr, __pgprot((*typ).prot_pte)) };
    if err != 0 { vunmap(addr as *mut _); return core::ptr::null_mut(); } flush_cache_vmap(addr, addr + size as u64); (offset + addr) as *mut _
}

pub unsafe extern "C" fn __arm_ioremap_caller(phys_addr: phys_addr_t, size: usize, mtype: u32, caller: *mut core::ffi::c_void) -> *mut core::ffi::c_void { let offset = phys_addr & !PAGE_MASK; let pfn = __phys_to_pfn(phys_addr); let last = phys_addr + size as u64 - 1; if size == 0 || last < phys_addr { return core::ptr::null_mut(); } __arm_ioremap_pfn_caller(pfn, offset, size, mtype, caller) }
pub unsafe extern "C" fn __arm_ioremap_pfn(pfn: u64, offset: u64, size: usize, mtype: u32) -> *mut core::ffi::c_void { __arm_ioremap_pfn_caller(pfn, offset, size, mtype, __builtin_return_address(0)) }
pub unsafe extern "C" fn ioremap(res_cookie: resource_size_t, size: usize) -> *mut core::ffi::c_void { arch_ioremap_caller.unwrap()(res_cookie, size, MT_DEVICE, __builtin_return_address(0)) }
pub unsafe extern "C" fn ioremap_cache(res_cookie: resource_size_t, size: usize) -> *mut core::ffi::c_void { arch_ioremap_caller.unwrap()(res_cookie, size, MT_DEVICE_CACHED, __builtin_return_address(0)) }
pub unsafe extern "C" fn ioremap_wc(res_cookie: resource_size_t, size: usize) -> *mut core::ffi::c_void { arch_ioremap_caller.unwrap()(res_cookie, size, MT_DEVICE_WC, __builtin_return_address(0)) }
pub unsafe extern "C" fn __arm_ioremap_exec(phys_addr: phys_addr_t, size: usize, cached: bool) -> *mut core::ffi::c_void { __arm_ioremap_caller(phys_addr, size, if cached { MT_MEMORY_RWX } else { MT_MEMORY_RWX_NONCACHED }, __builtin_return_address(0)) }
pub unsafe extern "C" fn __arm_iomem_set_ro(ptr: *mut core::ffi::c_void, size: usize) { set_memory_ro(ptr as u64, PAGE_ALIGN(size) / PAGE_SIZE); }
pub unsafe extern "C" fn arch_memremap_wb(phys_addr: phys_addr_t, size: usize, _flags: u64) -> *mut core::ffi::c_void { arch_ioremap_caller.unwrap()(phys_addr, size, MT_MEMORY_RW, __builtin_return_address(0)) }

pub unsafe extern "C" fn iounmap(io_addr: *const core::ffi::c_void) { let addr = ((io_addr as u64) & PAGE_MASK) as *mut _; if !find_static_vm_vaddr(addr).is_null() { return; } #[cfg(all(not(CONFIG_SMP), not(CONFIG_ARM_LPAE)))] { let vm = find_vm_area(addr); if !vm.is_null() && ((*vm).flags & VM_ARM_SECTION_MAPPING) != 0 { unmap_area_sections((*vm).addr as u64, (*vm).size as u64); } } vunmap(addr); }

#[cfg(any(CONFIG_PCI, CONFIG_PCMCIA))]
static mut pci_ioremap_mem_type: i32 = MT_DEVICE;
#[cfg(any(CONFIG_PCI, CONFIG_PCMCIA))] pub unsafe extern "C" fn pci_ioremap_set_mem_type(mem_type: i32) { pci_ioremap_mem_type = mem_type; }
#[cfg(any(CONFIG_PCI, CONFIG_PCMCIA))] pub unsafe extern "C" fn pci_remap_iospace(res: *const resource, phys_addr: phys_addr_t) -> i32 { let vaddr = PCI_IOBASE + (*res).start; if (*res).flags & IORESOURCE_IO == 0 || (*res).end > IO_SPACE_LIMIT { return -EINVAL; } vmap_page_range(vaddr, vaddr + resource_size(res), phys_addr, __pgprot((*get_mem_type(pci_ioremap_mem_type as u32)).prot_pte)) }
#[cfg(any(CONFIG_PCI, CONFIG_PCMCIA))] pub unsafe extern "C" fn pci_remap_cfgspace(res_cookie: resource_size_t, size: usize) -> *mut core::ffi::c_void { arch_ioremap_caller.unwrap()(res_cookie, size, MT_UNCACHED, __builtin_return_address(0)) }

pub unsafe extern "C" fn early_ioremap_init() { early_ioremap_setup(); }
pub unsafe extern "C" fn arch_memremap_can_ram_remap(offset: resource_size_t, _size: usize, _flags: u64) -> bool { memblock_is_map_memory(offset) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
