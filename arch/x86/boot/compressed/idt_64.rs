// SPDX-License-Identifier: GPL-2.0-only
//! IDT setup before and after relocating the x86-64 compressed kernel.

use crate::boot_idt_bindings::*;
use core::ptr::{addr_of, addr_of_mut};

unsafe fn set_idt_entry(vector: usize, handler: Option<unsafe extern "C" fn()>) {
    let address = handler.map_or(0, |handler| handler as usize);

    // SAFETY: the native packed descriptor contains only integer fields. Keep
    // all reserved fields zero, including the trailing x86-64 reserved dword.
    let mut entry: gate_desc = unsafe { core::mem::zeroed() };
    entry.offset_low = address as u16;
    entry.segment = LUPOS_BOOT_IDT_KERNEL_CS as u16;
    entry.bits.set_type(GATE_TRAP as u16);
    entry.bits.set_p(1);
    entry.offset_middle = (address >> 16) as u16;
    entry.offset_high = (address >> 32) as u32;

    // SAFETY: callers use native trap vectors within BOOT_IDT_ENTRIES. The
    // assembly-owned table has the native gate_desc stride and packed alignment.
    unsafe {
        addr_of_mut!(boot_idt)
            .cast::<gate_desc>()
            .add(vector)
            .write(entry)
    };
}

/* Have this here so we don't need to include <asm/desc.h> */
unsafe fn load_boot_idt(dtr: *const desc_ptr) {
    // SAFETY: early boot runs at CPL0; dtr addresses a native packed descriptor
    // whose ten bytes stay valid for this instruction. LIDT preserves flags.
    unsafe { core::arch::asm!("lidt [{}]", in(reg) dtr, options(nostack, preserves_flags)) };
}

/* Setup IDT before kernel jumping to .Lrelocated. */
#[no_mangle]
pub(crate) unsafe extern "C" fn load_stage1_idt() {
    // SAFETY: head_64.S owns this descriptor and table. Set the base at runtime
    // on every stage so no link-time absolute address survives relocation.
    unsafe {
        addr_of_mut!(boot_idt_desc.address)
            .write_unaligned(addr_of!(boot_idt) as core::ffi::c_ulong);

        #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
        set_idt_entry(X86_TRAP_VC as usize, Some(boot_stage1_vc));

        load_boot_idt(addr_of!(boot_idt_desc));
    }
}

/*
 * Setup IDT after kernel jumping to .Lrelocated.
 *
 * initialize_identity_maps() needs a #PF handler in order to fault-in identity
 * mapping ranges. The second-stage #VC handler needs a GHCB but must set it up
 * itself, since early_setup_ghcb()/set_page_decrypted() require this #PF entry.
 */
#[no_mangle]
pub(crate) unsafe extern "C" fn load_stage2_idt() {
    // SAFETY: native table storage is valid through compressed-kernel boot;
    // the assembly entry points implement the original exception-entry ABI.
    unsafe {
        addr_of_mut!(boot_idt_desc.address)
            .write_unaligned(addr_of!(boot_idt) as core::ffi::c_ulong);

        set_idt_entry(X86_TRAP_PF as usize, Some(boot_page_fault));
        set_idt_entry(X86_TRAP_NMI as usize, Some(boot_nmi_trap));

        #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
        {
            // Preserve the native present trap gate with a zero target when
            // SEV-ES was not detected, clearing the prior-stage #VC target.
            let handler = if sev_status & LUPOS_BOOT_IDT_SEV_ES_ENABLED as u64 != 0 {
                Some(boot_stage2_vc as unsafe extern "C" fn())
            } else {
                None
            };
            set_idt_entry(X86_TRAP_VC as usize, handler);
        }

        load_boot_idt(addr_of!(boot_idt_desc));
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn cleanup_exception_handling() {
    // SAFETY: preserve native ordering: flush/re-encrypt the GHCB before
    // disabling exception handling. Without AMD_MEM_ENCRYPT, the native header
    // supplies an empty inline and there is no external shutdown call.
    unsafe {
        #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
        sev_es_shutdown_ghcb();

        addr_of_mut!(boot_idt_desc.size).write_unaligned(0);
        addr_of_mut!(boot_idt_desc.address).write_unaligned(0);
        load_boot_idt(addr_of!(boot_idt_desc));
    }
}
