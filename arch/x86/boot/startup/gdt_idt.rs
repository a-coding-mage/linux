// SPDX-License-Identifier: GPL-2.0
//! Descriptor tables used before the permanent kernel IDT is available.

use crate::startup_gdt_bindings::*;
use core::arch::{asm, global_asm};
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::addr_of_mut;

// The native array is 512 bytes, aligned to a page, in initialized data. A
// repr(align(PAGE_SIZE)) Rust wrapper would also pad its size to a whole page.
// Emit just the native array and its alignment, retaining its local linkage.
global_asm!(
    ".pushsection .data..page_aligned,\"aw\",@progbits",
    ".balign {page_size}",
    ".local bringup_idt_table",
    ".type bringup_idt_table, @object",
    "bringup_idt_table:",
    ".zero {table_size}",
    ".size bringup_idt_table, . - bringup_idt_table",
    ".popsection",
    page_size = const LUPOS_STARTUP_GDT_PAGE_SIZE,
    table_size = const size_of::<[gate_desc; NUM_EXCEPTION_VECTORS as usize]>(),
);

unsafe extern "C" {
    static mut bringup_idt_table: [gate_desc; NUM_EXCEPTION_VECTORS as usize];
}

// Reject a mismatched native target before emitting any boot instructions.
const _: () = {
    assert!(size_of::<gate_desc>() == 16);
    assert!(size_of::<desc_ptr>() == 10);
    assert!(size_of::<[gate_desc; NUM_EXCEPTION_VECTORS as usize]>() - 1 <= u16::MAX as usize);
    assert!(X86_TRAP_VC < NUM_EXCEPTION_VECTORS);
    assert!(X86_TRAP_VC <= 0xff);
    assert!(GDT_SIZE - 1 <= u16::MAX as u32);
};

/* This may run while still in the direct mapping. */
#[no_mangle]
pub(crate) unsafe extern "C" fn startup_64_load_idt(vc_handler: *mut c_void) {
    let table: *mut gate_desc;
    // SAFETY: exactly like rip_rel_ptr(), compute the address in the current
    // mapping without a GOT load or an absolute link-time pointer. The table
    // remains in .data throughout BSP/AP bringup, including before BSS clear.
    unsafe {
        asm!(
            "lea {table}, [rip + {symbol}]",
            table = out(reg) table,
            symbol = sym bringup_idt_table,
            options(nostack, nomem, preserves_flags),
        );
    }
    let desc = desc_ptr {
        size: (size_of::<[gate_desc; NUM_EXCEPTION_VECTORS as usize]>() - 1) as u16,
        address: table as core::ffi::c_ulong,
    };

    // A null handler leaves the existing table untouched, including any #VC
    // entry installed previously. init_idt_data() builds an interrupt gate,
    // unlike the trap gates used by the separate compressed-kernel IDT.
    if !vc_handler.is_null() {
        let address = vc_handler as usize;
        // SAFETY: native gate_desc contains only integers and integer bitfields.
        // This is init_idt_data()'s zeroing, including IST, DPL and reserved bits.
        let mut entry: gate_desc = unsafe { core::mem::zeroed() };
        entry.offset_low = address as u16;
        entry.segment = LUPOS_STARTUP_GDT_KERNEL_CS as u16;
        entry.bits.set_type(GATE_INTERRUPT as u16);
        entry.bits.set_p(1);
        entry.offset_middle = (address >> 16) as u16;
        entry.offset_high = (address >> 32) as u32;

        // SAFETY: the native trap number was checked above. Copy the complete
        // packed gate with its native stride, as native_write_idt_entry() does.
        unsafe { table.add(X86_TRAP_VC as usize).write(entry) };
    }

    // SAFETY: early bringup runs at CPL0. The packed ten-byte descriptor lives
    // through LIDT; the memory effect orders the optional gate write before it.
    unsafe { asm!("lidt [{}]", in(reg) &desc, options(nostack, preserves_flags)) };
}

/* Setup boot CPU state before switching to virtual addresses. */
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub(crate) unsafe extern "C" fn startup_64_setup_gdt_idt() {
    let gp: *mut gdt_page;
    // SAFETY: gdt_page is the original per-CPU GDT reached through its boot
    // linker alias after __pi_ prefixing. No per-CPU segment access is valid yet.
    unsafe {
        asm!(
            "lea {gp}, [rip + {symbol}]",
            gp = out(reg) gp,
            symbol = sym gdt_page,
            options(nostack, nomem, preserves_flags),
        );
    }
    let desc = desc_ptr {
        size: (GDT_SIZE - 1) as u16,
        // SAFETY: only form a raw field address; do not create a reference to
        // the per-CPU object while it is accessed through its early boot alias.
        address: unsafe { addr_of_mut!((*gp).gdt) as core::ffi::c_ulong },
    };

    // SAFETY: load the original GDT before reloading DS, SS and ES, preserving
    // the native order and EAX selector operand. Keep the memory clobber.
    unsafe {
        asm!("lgdt [{}]", in(reg) &desc, options(nostack, preserves_flags));
        asm!(
            "mov ds, ax",
            "mov ss, ax",
            "mov es, ax",
            in("eax") LUPOS_STARTUP_GDT_KERNEL_DS as u32,
            options(nostack, preserves_flags),
        );
    }

    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    let handler: *mut c_void = {
        let handler;
        // SAFETY: the assembly #VC entry exists only with AMD_MEM_ENCRYPT.
        // Match rip_rel_ptr(vc_no_ghcb), including the __pi_ alias contract.
        unsafe {
            asm!(
                "lea {handler}, [rip + {symbol}]",
                handler = out(reg) handler,
                symbol = sym vc_no_ghcb,
                options(nostack, nomem, preserves_flags),
            );
        }
        handler
    };
    #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
    let handler = core::ptr::null_mut();

    // SAFETY: the GDT and data segments have just been established at CPL0.
    unsafe { startup_64_load_idt(handler) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
