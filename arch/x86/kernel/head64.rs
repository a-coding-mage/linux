// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2000 Andrea Arcangeli <andrea@suse.de> SuSE
// Rust production owner reconstructed from the unchanged head64.c at 0008179a1.
// The source object's no-instrumentation policy is in head64-rust.mk.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/head64_generated.rs"));
}
use bindings as b;
use core::arch::{asm, global_asm};
use core::mem::{size_of, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping, read_unaligned, write_bytes};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[path = "../../../rust/ffi_export.rs"]
mod ffi_export;

#[no_mangle]
#[link_section = ".init.data"]
pub static mut next_early_pgt: c_uint = 0;
#[no_mangle]
pub static mut early_pmd_flags: b::pmdval_t = b::RUST_HEAD64_EARLY_PMD_FLAGS;
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut __pgtable_l5_enabled: c_uint = 0;
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut pgdir_shift: c_uint = 39;
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut ptrs_per_p4d: c_uint = 1;
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut page_offset_base: c_ulong = b::RUST_HEAD64_PAGE_OFFSET_L4;
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut vmalloc_base: c_ulong = b::RUST_HEAD64_VMALLOC_L4;
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut vmemmap_base: c_ulong = b::RUST_HEAD64_VMEMMAP_L4;

// SYM_PIC_ALIAS: strong global aliases, not separate storage or weak definitions.
global_asm!(
    ".globl __pi_next_early_pgt\n.set __pi_next_early_pgt, {next}",
    ".globl __pi___pgtable_l5_enabled\n.set __pi___pgtable_l5_enabled, {l5}",
    ".globl __pi_pgdir_shift\n.set __pi_pgdir_shift, {shift}",
    ".globl __pi_ptrs_per_p4d\n.set __pi_ptrs_per_p4d, {ptrs}",
    next = sym next_early_pgt, l5 = sym __pgtable_l5_enabled,
    shift = sym pgdir_shift, ptrs = sym ptrs_per_p4d,
);
ffi_export::export_symbol!(pgdir_shift, pgdir_shift, "", "");
ffi_export::export_symbol!(ptrs_per_p4d, ptrs_per_p4d, "", "");
ffi_export::export_symbol!(page_offset_base, page_offset_base, "", "");
ffi_export::export_symbol!(vmalloc_base, vmalloc_base, "", "");
ffi_export::export_symbol!(vmemmap_base, vmemmap_base, "", "");

// All seven unconditional BUILD_BUG_ON expressions, from configured headers.
const _: () = {
    assert!(b::RUST_HEAD64_MODULES_VADDR >= b::RUST_HEAD64_START_KERNEL_MAP);
    assert!(b::RUST_HEAD64_MODULES_VADDR - b::RUST_HEAD64_START_KERNEL_MAP
        >= b::RUST_HEAD64_KERNEL_IMAGE_SIZE);
    assert!(b::RUST_HEAD64_MODULES_LEN + b::RUST_HEAD64_KERNEL_IMAGE_SIZE
        <= 2 * b::RUST_HEAD64_PUD_SIZE);
    assert!(b::RUST_HEAD64_START_KERNEL_MAP & !b::RUST_HEAD64_PMD_MASK == 0);
    assert!(b::RUST_HEAD64_MODULES_VADDR & !b::RUST_HEAD64_PMD_MASK == 0);
    assert!(b::RUST_HEAD64_MODULES_VADDR > b::RUST_HEAD64_START_KERNEL);
    assert!(b::RUST_HEAD64_FIXMAP_END > b::RUST_HEAD64_MODULES_END);
};

#[inline(always)]
unsafe fn pgtable_l5_enabled() -> bool {
    // USE_EARLY_PGTABLE_L5: never consult cpu_feature_enabled/static keys here.
    __pgtable_l5_enabled != 0
}
#[inline(always)]
unsafe fn encryption_mask() -> c_ulong {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    { b::sme_me_mask as c_ulong }
    #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
    { 0 }
}
#[inline(always)]
unsafe fn pte_pfn_mask() -> c_ulong {
    #[cfg(CONFIG_DYNAMIC_PHYSICAL_MASK)]
    let physical = b::physical_mask as c_ulong;
    #[cfg(not(CONFIG_DYNAMIC_PHYSICAL_MASK))]
    let physical = (1usize << b::RUST_HEAD64_PHYSICAL_MASK_SHIFT) - 1;
    b::RUST_HEAD64_PAGE_MASK & physical
}
#[inline(always)]
unsafe fn pa_nodebug(address: c_ulong) -> c_ulong {
    let y = address.wrapping_sub(b::RUST_HEAD64_START_KERNEL_MAP);
    y.wrapping_add(if address > y { b::phys_base } else {
        b::RUST_HEAD64_START_KERNEL_MAP.wrapping_sub(page_offset_base)
    })
}
#[inline(always)]
unsafe fn va(address: c_ulong) -> *mut c_char {
    address.wrapping_add(page_offset_base) as *mut c_char
}
#[inline(always)]
unsafe fn native_read_cr2() -> c_ulong {
    let value;
    asm!("mov {}, cr2", out(reg) value, options(nomem, nostack, preserves_flags));
    value
}
#[inline(always)]
unsafe fn native_read_cr4() -> c_ulong {
    let value;
    asm!("mov {}, cr4", out(reg) value, options(nomem, nostack, preserves_flags));
    value
}
#[inline(always)]
unsafe fn pgt_virtual(entry: c_ulong) -> c_ulong {
    (entry & pte_pfn_mask()).wrapping_add(b::RUST_HEAD64_START_KERNEL_MAP)
        .wrapping_sub(b::phys_base)
}
#[inline(always)]
unsafe fn pgt_entry(table: c_ulong) -> c_ulong {
    table.wrapping_sub(b::RUST_HEAD64_START_KERNEL_MAP).wrapping_add(b::phys_base)
        .wrapping_add(b::RUST_HEAD64_KERNPG_TABLE_NOENC | encryption_mask())
}
#[inline(always)]
unsafe fn allocate_early_pgt<T>() -> *mut T {
    let table = addr_of_mut!(b::early_dynamic_pgts).cast::<b::pmd_t>()
        .add(next_early_pgt as usize * b::RUST_HEAD64_PTRS_PER_PMD as usize).cast();
    // Match the original post-increment, before clearing/publishing the page.
    next_early_pgt = next_early_pgt.wrapping_add(1);
    table
}

#[cold]
#[link_section = ".init.text"]
unsafe fn reset_early_page_tables() {
    write_bytes(addr_of_mut!(b::early_top_pgt).cast::<b::pgd_t>(), 0,
        b::RUST_HEAD64_PTRS_PER_PGD as usize - 1);
    next_early_pgt = 0;
    b::rust_head64_write_cr3(pa_nodebug(addr_of!(b::early_top_pgt) as c_ulong)
        | encryption_mask());
}

#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn __early_make_pgtable(address: c_ulong, pmd: b::pmdval_t) -> bool {
    let physaddr = address.wrapping_sub(page_offset_base);
    let maxmem = if pgtable_l5_enabled() { b::RUST_HEAD64_MAXMEM_L5 }
        else { b::RUST_HEAD64_MAXMEM_L4 };
    if physaddr >= maxmem { return false; }
    let cr3_pa = b::rust_head64_read_cr3() & (pte_pfn_mask() & !encryption_mask());
    if cr3_pa != pa_nodebug(addr_of!(b::early_top_pgt) as c_ulong) {
        return false;
    }
    loop {
        let pgd_index = (address >> pgdir_shift) & (b::RUST_HEAD64_PTRS_PER_PGD - 1);
        let pgd_p = addr_of_mut!((*addr_of_mut!(b::early_top_pgt)
            .cast::<b::pgd_t>().add(pgd_index as usize)).pgd);
        let pgd = *pgd_p;
        // Reconstruct tables through __START_KERNEL_map, never the dynamic map.
        let p4d_p: *mut b::p4dval_t;
        if !pgtable_l5_enabled() {
            p4d_p = pgd_p.cast();
        } else if pgd != 0 {
            p4d_p = pgt_virtual(pgd) as *mut _;
        } else {
            if next_early_pgt as c_ulong >= b::RUST_HEAD64_EARLY_DYNAMIC_PAGE_TABLES {
                reset_early_page_tables();
                continue;
            }
            p4d_p = allocate_early_pgt();
            write_bytes(p4d_p, 0, ptrs_per_p4d as usize);
            *pgd_p = pgt_entry(p4d_p as c_ulong);
        }
        let p4d_index = (address >> b::RUST_HEAD64_P4D_SHIFT)
            & (ptrs_per_p4d as c_ulong).wrapping_sub(1);
        let p4d_p = p4d_p.add(p4d_index as usize);
        let p4d = *p4d_p;
        let pud_p: *mut b::pudval_t;
        if p4d != 0 {
            pud_p = pgt_virtual(p4d) as *mut _;
        } else {
            if next_early_pgt as c_ulong >= b::RUST_HEAD64_EARLY_DYNAMIC_PAGE_TABLES {
                reset_early_page_tables();
                continue;
            }
            pud_p = allocate_early_pgt();
            write_bytes(pud_p, 0, b::RUST_HEAD64_PTRS_PER_PUD as usize);
            *p4d_p = pgt_entry(pud_p as c_ulong);
        }
        let pud_index = (address >> b::RUST_HEAD64_PUD_SHIFT) & (b::RUST_HEAD64_PTRS_PER_PUD - 1);
        let pud_p = pud_p.add(pud_index as usize);
        let pud = *pud_p;
        let pmd_p: *mut b::pmdval_t;
        if pud != 0 {
            pmd_p = pgt_virtual(pud) as *mut _;
        } else {
            if next_early_pgt as c_ulong >= b::RUST_HEAD64_EARLY_DYNAMIC_PAGE_TABLES {
                reset_early_page_tables();
                continue;
            }
            pmd_p = allocate_early_pgt();
            write_bytes(pmd_p, 0, b::RUST_HEAD64_PTRS_PER_PMD as usize);
            *pud_p = pgt_entry(pmd_p as c_ulong);
        }
        let pmd_index = (address >> b::RUST_HEAD64_PMD_SHIFT) & (b::RUST_HEAD64_PTRS_PER_PMD - 1);
        *pmd_p.add(pmd_index as usize) = pmd;
        return true;
    }
}

#[cold]
#[link_section = ".init.text"]
unsafe fn early_make_pgtable(address: c_ulong) -> bool {
    let physaddr = address.wrapping_sub(page_offset_base);
    __early_make_pgtable(address, (physaddr & b::RUST_HEAD64_PMD_MASK).wrapping_add(early_pmd_flags))
}

#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn do_early_exception(regs: *mut b::pt_regs, trapnr: c_int) {
    if trapnr == b::X86_TRAP_PF as c_int && early_make_pgtable(native_read_cr2()) { return; }
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    if trapnr == b::X86_TRAP_VC as c_int && b::handle_vc_boot_ghcb(regs) { return; }
    #[cfg(CONFIG_INTEL_TDX_GUEST)]
    if trapnr == b::X86_TRAP_VE as c_int && b::tdx_early_handle_ve(regs) { return; }
    b::early_fixup_exception(regs, trapnr);
}

// No printk: the PDA has not been initialized.
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn clear_bss() {
    let start = addr_of_mut!(b::__bss_start).cast::<u8>();
    write_bytes(start, 0, (addr_of!(b::__bss_stop) as usize).wrapping_sub(start as usize));
    let start = addr_of_mut!(b::__brk_base).cast::<u8>();
    write_bytes(start, 0, (addr_of!(b::__brk_limit) as usize).wrapping_sub(start as usize));
}

unsafe fn get_cmd_line_ptr() -> c_ulong {
    // setup_header and boot_params are packed: never create field references.
    let bp = addr_of!(b::boot_params);
    read_unaligned(addr_of!((*bp).hdr.cmd_line_ptr)) as c_ulong
        | ((read_unaligned(addr_of!((*bp).ext_cmd_line_ptr)) as c_ulong) << 32)
}

// Exact sanitizer from asm/bootparam_utils.h, including static zeroed scratch.
// MaybeUninit owns canonical storage without forming an invalid or aligned reference.
static mut boot_params_scratch: MaybeUninit<b::boot_params> = MaybeUninit::zeroed();
unsafe fn sanitize_boot_params(bp: *mut b::boot_params) {
    if (*bp).sentinel == 0 { return; }
    let saved = addr_of_mut!(boot_params_scratch).cast::<b::boot_params>();
    write_bytes(saved.cast::<u8>(), 0, size_of::<b::boot_params>());
    macro_rules! preserve {
        ($field:ident) => {
            copy_nonoverlapping(addr_of!((*bp).$field).cast::<u8>(),
                addr_of_mut!((*saved).$field).cast::<u8>(),
                // Type inference gets the exact configured member size without a reference.
                field_size(addr_of!((*bp).$field)));
        };
    }
    preserve!(screen_info);
    preserve!(apm_bios_info);
    preserve!(tboot_addr);
    preserve!(ist_info);
    preserve!(hd0_info);
    preserve!(hd1_info);
    preserve!(sys_desc_table);
    preserve!(olpc_ofw_header);
    preserve!(efi_info);
    preserve!(alt_mem_k);
    preserve!(scratch);
    preserve!(e820_entries);
    preserve!(eddbuf_entries);
    preserve!(edd_mbr_sig_buf_entries);
    preserve!(edd_mbr_sig_buffer);
    preserve!(secure_boot);
    preserve!(hdr);
    preserve!(e820_table);
    preserve!(eddbuf);
    preserve!(cc_blob_address);
    copy_nonoverlapping(saved.cast::<u8>(), bp.cast::<u8>(), size_of::<b::boot_params>());
}
#[inline(always)]
fn field_size<T>(_: *const T) -> usize { size_of::<T>() }

#[cold]
#[link_section = ".init.text"]
unsafe fn copy_bootdata(real_mode_data: *mut c_char) {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    b::sme_map_bootdata(real_mode_data);
    let bp = addr_of_mut!(b::boot_params);
    copy_nonoverlapping(real_mode_data.cast::<u8>(), bp.cast::<u8>(), size_of::<b::boot_params>());
    sanitize_boot_params(bp);
    let command_line = get_cmd_line_ptr();
    if command_line != 0 {
        copy_nonoverlapping(va(command_line), addr_of_mut!(b::boot_command_line).cast(),
            b::RUST_HEAD64_COMMAND_LINE_SIZE as usize);
    }
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    b::sme_unmap_bootdata(real_mode_data);
}

#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn x86_64_start_kernel(real_mode_data: *mut c_char) -> ! {
    // MAYBE_BUILD_BUG_ON is a runtime BUG_ON for variable PGDIR_SHIFT.
    let pgdir_mask = !((1usize << pgdir_shift).wrapping_sub(1));
    if ((b::RUST_HEAD64_MODULES_END - 1) & pgdir_mask)
        != (b::RUST_HEAD64_START_KERNEL & pgdir_mask) { b::rust_head64_bug(); }
    b::rust_head64_this_cpu_write_cr4(native_read_cr4());
    reset_early_page_tables();
    if pgtable_l5_enabled() {
        page_offset_base = b::RUST_HEAD64_PAGE_OFFSET_L5;
        vmalloc_base = b::RUST_HEAD64_VMALLOC_L5;
        vmemmap_base = b::RUST_HEAD64_VMEMMAP_L5;
    }
    clear_bss();
    b::rust_head64_clear_page(addr_of_mut!(b::init_top_pgt).cast::<c_void>());
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    b::sme_early_init();
    #[cfg(CONFIG_KASAN)]
    b::kasan_early_init();
    // native_write_cr4 may be instrumented; preserve its position after KASAN.
    let cr4 = b::rust_head64_this_cpu_cr4();
    b::native_write_cr4(cr4 ^ b::RUST_HEAD64_CR4_PGE);
    b::native_write_cr4(cr4);
    b::idt_setup_early_handler();
    #[cfg(CONFIG_INTEL_TDX_GUEST)]
    b::tdx_early_init();
    copy_bootdata(va(real_mode_data as c_ulong));
    #[cfg(CONFIG_MICROCODE)]
    b::load_ucode_bsp();
    *addr_of_mut!(b::init_top_pgt).cast::<b::pgd_t>().add(511)
        = *addr_of!(b::early_top_pgt).cast::<b::pgd_t>().add(511);
    x86_64_start_reservations(real_mode_data)
}

#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn x86_64_start_reservations(real_mode_data: *mut c_char) -> ! {
    if read_unaligned(addr_of!(b::boot_params.hdr.version)) == 0 {
        copy_bootdata(va(real_mode_data as c_ulong));
    }
    b::x86_early_init_platform_quirks();
    #[cfg(CONFIG_X86_INTEL_MID)]
    if read_unaligned(addr_of!(b::boot_params.hdr.hardware_subarch)) == b::RUST_HEAD64_SUBARCH_INTEL_MID as u32 {
        b::x86_intel_mid_early_setup();
    }
    b::start_kernel()
}

#[no_mangle]
pub unsafe extern "C" fn early_setup_idt() {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    let handler = {
        b::setup_ghcb();
        b::vc_boot_ghcb as *const () as *mut c_void
    };
    #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
    let handler = core::ptr::null_mut();
    b::__pi_startup_64_load_idt(handler);
}
