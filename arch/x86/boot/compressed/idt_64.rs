// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the surrounding kernel translation:
// asm/trap_pf.h, asm/segment.h, asm/trapnr.h, and misc.h.

use core::ffi::c_void;

extern "C" {
    static mut boot_idt: [gate_desc; _];
    static mut boot_idt_desc: desc_ptr;
    static sev_status: u64;

    fn boot_stage1_vc();
    fn boot_page_fault();
    fn boot_nmi_trap();
    fn boot_stage2_vc();
    fn sev_es_shutdown_ghcb();
}

#[repr(C)]
struct gate_bits {
    type_: u8,
    p: u8,
}

#[repr(C)]
struct gate_desc {
    offset_low: u16,
    segment: u16,
    bits: gate_bits,
    offset_middle: u16,
    offset_high: u32,
}

#[repr(C)]
struct desc_ptr {
    size: u16,
    address: usize,
}

const __KERNEL_CS: u16 = 0;
const GATE_TRAP: u8 = 0;
const X86_TRAP_PF: i32 = 14;
const X86_TRAP_NMI: i32 = 2;
const X86_TRAP_VC: i32 = 29;

unsafe fn set_idt_entry(vector: i32, handler: Option<unsafe extern "C" fn()>) {
    let address = handler.map_or(0usize, |f| f as usize);
    let mut entry = core::mem::MaybeUninit::<gate_desc>::zeroed().assume_init();

    entry.offset_low = (address & 0xffff) as u16;
    entry.segment = __KERNEL_CS;
    entry.bits.type_ = GATE_TRAP;
    entry.bits.p = 1;
    entry.offset_middle = ((address >> 16) & 0xffff) as u16;
    entry.offset_high = (address >> 32) as u32;

    core::ptr::copy_nonoverlapping(
        &entry as *const gate_desc,
        boot_idt.as_mut_ptr().add(vector as usize),
        1,
    );
}

/* Have this here so we don't need to include <asm/desc.h> */
unsafe fn load_boot_idt(dtr: *const desc_ptr) {
    core::arch::asm!("lidt [{}]", in(reg) dtr, options(nostack, preserves_flags));
}

/* Setup IDT before kernel jumping to  .Lrelocated */
#[no_mangle]
pub unsafe extern "C" fn load_stage1_idt() {
    boot_idt_desc.address = boot_idt.as_mut_ptr() as usize;

    // Equivalent to IS_ENABLED(CONFIG_AMD_MEM_ENCRYPT).
    set_idt_entry(X86_TRAP_VC, Some(boot_stage1_vc));

    load_boot_idt(&boot_idt_desc);
}

/*
 * Setup IDT after kernel jumping to  .Lrelocated.
 *
 * initialize_identity_maps() needs a #PF handler to be setup
 * in order to be able to fault-in identity mapping ranges; see
 * do_boot_page_fault().
 *
 * This #PF handler setup needs to happen in load_stage2_idt() where the
 * IDT is loaded and there the #VC IDT entry gets setup too.
 *
 * In order to be able to handle #VCs, one needs a GHCB which
 * gets setup with an already set up pagetable, which is done in
 * initialize_identity_maps(). And there's the catch 22: the boot #VC
 * handler do_boot_stage2_vc() needs to call early_setup_ghcb() itself
 * (and, especially set_page_decrypted()) because the SEV-ES setup code
 * cannot initialize a GHCB as there's no #PF handler yet...
 */
#[no_mangle]
pub unsafe extern "C" fn load_stage2_idt() {
    boot_idt_desc.address = boot_idt.as_mut_ptr() as usize;

    set_idt_entry(X86_TRAP_PF, Some(boot_page_fault));
    set_idt_entry(X86_TRAP_NMI, Some(boot_nmi_trap));

    // #ifdef CONFIG_AMD_MEM_ENCRYPT
    /*
     * Clear the second stage #VC handler in case guest types
     * needing #VC have not been detected.
     */
    if sev_status & (1u64 << 1) != 0 {
        set_idt_entry(X86_TRAP_VC, Some(boot_stage2_vc));
    } else {
        set_idt_entry(X86_TRAP_VC, None);
    }
    // #endif

    load_boot_idt(&boot_idt_desc);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_exception_handling() {
    /*
     * Flush GHCB from cache and map it encrypted again when running as
     * SEV-ES guest.
     */
    sev_es_shutdown_ghcb();

    /* Set a null-idt, disabling #PF and #VC handling */
    boot_idt_desc.size = 0;
    boot_idt_desc.address = 0;
    load_boot_idt(&boot_idt_desc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
