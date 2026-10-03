// SPDX-License-Identifier: GPL-2.0
//! Low-memory trampoline placement and native 4/5-level paging transitions.

use crate::bindings as b;
use crate::boot_pgtable_bindings as p;
use core::arch::asm;
use core::ffi::{c_ulong, c_void};
use core::mem::{size_of, transmute, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, null_mut, read_unaligned, write_unaligned};

const BIOS_START_MIN: usize = 0x20000; // 128K, less than this is insane.
const BIOS_START_MAX: usize = 0x9f000; // 640K, absolute maximum.
const PAGE_SIZE: usize = p::LUPOS_BOOT_PGTABLE_PAGE_SIZE as usize;
const PAGE_MASK: usize = p::LUPOS_BOOT_PGTABLE_PAGE_MASK as usize;
const TRAMPOLINE_SIZE: usize = p::LUPOS_BOOT_PGTABLE_TRAMPOLINE_SIZE as usize;
const CODE_OFFSET: usize = p::LUPOS_BOOT_PGTABLE_CODE_OFFSET as usize;
const CODE_SIZE: usize = p::LUPOS_BOOT_PGTABLE_CODE_SIZE as usize;

// These values must survive the relocation and subsequent .bss clearing in
// head_64.S, including the initially zero flag and null trampoline pointer.
#[export_name = "__pgtable_l5_enabled"]
#[link_section = ".data"]
static mut PGTABLE_L5_ENABLED: u32 = 0;
#[export_name = "pgdir_shift"]
#[link_section = ".data"]
static mut PGDIR_SHIFT: u32 = 39;
#[export_name = "ptrs_per_p4d"]
#[link_section = ".data"]
static mut PTRS_PER_P4D: u32 = 1;
#[export_name = "trampoline_32bit"]
#[link_section = ".data"]
static mut TRAMPOLINE_32BIT: *mut c_ulong = null_mut();

// Filled before use and consumed before .bss is cleared, just like the C array.
static mut TRAMPOLINE_SAVE: [u8; TRAMPOLINE_SIZE] = [0; TRAMPOLINE_SIZE];

// This translation unit originally gets its own sanitizer from
// asm/bootparam_utils.h. Preserve exactly that field list and scratch lifetime.
unsafe fn preserve_field<T>(dest: *mut T, source: *const T) {
    unsafe {
        b::memcpy(dest.cast(), source.cast(), size_of::<T>());
    }
}

unsafe fn sanitize_boot_params(params: *mut b::boot_params) {
    unsafe {
        if (*params).sentinel == 0 {
            return;
        }
        static mut SANITIZE_SCRATCH: MaybeUninit<b::boot_params> = MaybeUninit::uninit();
        let saved = addr_of_mut!(SANITIZE_SCRATCH).cast::<b::boot_params>();
        b::memset(saved.cast(), 0, size_of::<b::boot_params>());
        macro_rules! preserve {
            ($($field:ident),+ $(,)?) => {
                $(preserve_field(addr_of_mut!((*saved).$field), addr_of!((*params).$field));)+
            };
        }
        preserve!(
            screen_info,
            apm_bios_info,
            tboot_addr,
            ist_info,
            hd0_info,
            hd1_info,
            sys_desc_table,
            olpc_ofw_header,
            efi_info,
            alt_mem_k,
            scratch,
            e820_entries,
            eddbuf_entries,
            edd_mbr_sig_buf_entries,
            edd_mbr_sig_buffer,
            secure_boot,
            hdr,
            e820_table,
            eddbuf,
            cc_blob_address,
        );
        b::memcpy(params.cast(), saved.cast(), size_of::<b::boot_params>());
    }
}

unsafe fn find_trampoline_placement() -> usize {
    unsafe {
        let bp = b::boot_params_ptr;
        let mut bios_start = 0;
        let mut ebda_start = 0;

        // EFI machines need not map legacy ROM. Check its private signature
        // before accessing either BIOS data-area word. 0x413 is unaligned.
        let signature = addr_of!((*bp).efi_info.efi_loader_signature).cast();
        if b::strncmp(signature, b::EFI32_LOADER_SIGNATURE.as_ptr().cast(), 4) != 0
            && b::strncmp(signature, b::EFI64_LOADER_SIGNATURE.as_ptr().cast(), 4) != 0
        {
            ebda_start = (read_unaligned(0x40e as *const u16) as usize) << 4;
            bios_start = (read_unaligned(0x413 as *const u16) as usize) << 10;
        }

        if bios_start < BIOS_START_MIN || bios_start > BIOS_START_MAX {
            bios_start = BIOS_START_MAX;
        }
        if ebda_start > BIOS_START_MIN && ebda_start < bios_start {
            bios_start = ebda_start;
        }
        bios_start &= PAGE_MASK;

        // Match C's reverse traversal and signed zero-entry termination.
        // boot_params and its E820 entries are packed: use only raw pointers.
        let mut i = (*bp).e820_entries as i32 - 1;
        while i >= 0 {
            let entry = addr_of!((*bp).e820_table)
                .cast::<b::boot_e820_entry>()
                .add(i as usize);
            i -= 1;
            let start = read_unaligned(addr_of!((*entry).addr));
            if bios_start as u64 <= start {
                continue;
            }
            if read_unaligned(addr_of!((*entry).type_)) != b::E820_TYPE_RAM {
                continue;
            }

            let end = start.wrapping_add(read_unaligned(addr_of!((*entry).size)));
            let mut new = bios_start;
            if bios_start as u64 > end {
                new = end as usize;
            }
            new &= PAGE_MASK;

            // The subtraction in both original comparisons is unsigned.
            // Preserve wrapping and the original too-small/underflow order.
            let candidate = new.wrapping_sub(TRAMPOLINE_SIZE);
            if (candidate as u64) < start {
                continue;
            }
            if candidate > bios_start {
                break;
            }
            bios_start = new;
            break;
        }
        bios_start.wrapping_sub(TRAMPOLINE_SIZE)
    }
}

// native_cpuid_eax/ecx initialize ECX to zero and include a memory clobber.
// The Rust boot cpuid_count owner preserves BX and has that same native asm.
unsafe fn native_cpuid(op: u32) -> (u32, u32) {
    let mut eax = 0;
    let mut ecx = 0;
    let mut ignored = 0;
    unsafe {
        crate::cpu_bindings::cpuid_count(
            op,
            0,
            addr_of_mut!(eax),
            addr_of_mut!(ignored),
            addr_of_mut!(ecx),
            addr_of_mut!(ignored),
        );
    }
    (eax, ecx)
}

// PTE_PFN_MASK = PHYSICAL_PAGE_MASK = PAGE_MASK & __PHYSICAL_MASK.
// The dynamic mask must remain a runtime read, including its SME adjustment.
unsafe fn pte_pfn_mask() -> c_ulong {
    #[cfg(CONFIG_DYNAMIC_PHYSICAL_MASK)]
    let physical = unsafe { p::physical_mask };
    #[cfg(not(CONFIG_DYNAMIC_PHYSICAL_MASK))]
    let physical = p::LUPOS_BOOT_PGTABLE_PHYSICAL_MASK;
    (physical as c_ulong) & PAGE_MASK as c_ulong
}

unsafe fn native_read_cr3_pa() -> c_ulong {
    let value: c_ulong;
    unsafe {
        asm!("mov {}, cr3", out(reg) value, options(nomem, nostack, preserves_flags));
        let mask = pte_pfn_mask();
        // CR3_ADDR_MASK additionally removes the encryption bit, even before
        // initialize_identity_maps() narrows the dynamic physical mask.
        #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
        let mask = mask & !(p::sme_me_mask as c_ulong);
        value & mask
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn configure_5level_paging(
    bp: *mut b::boot_params,
    pgtable: *mut c_void,
) {
    unsafe {
        sanitize_boot_params(bp);
        b::boot_params_ptr = bp;

        // The original compressed C owner does not gate this on
        // CONFIG_X86_5LEVEL: retain its command-line/CPUID decision exactly.
        let l5_required = b::cmdline_find_option_bool(c"no5lvl".as_ptr()) == 0
            && native_cpuid(0).0 >= 7
            && native_cpuid(7).1 & (1 << 16) != 0;
        if l5_required {
            PGTABLE_L5_ENABLED = 1;
            PGDIR_SHIFT = 48;
            PTRS_PER_P4D = 512;
        }

        let cr4: c_ulong;
        asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack, preserves_flags));
        if l5_required == (cr4 & p::LUPOS_BOOT_PGTABLE_CR4_LA57 as c_ulong != 0) {
            return;
        }

        let trampoline = find_trampoline_placement() as *mut c_ulong;
        TRAMPOLINE_32BIT = trampoline;
        let saved = addr_of_mut!(TRAMPOLINE_SAVE).cast::<u8>();
        b::memcpy(saved.cast(), trampoline.cast(), TRAMPOLINE_SIZE);
        b::memset(trampoline.cast(), 0, TRAMPOLINE_SIZE);

        let code = trampoline.cast::<u8>().add(CODE_OFFSET);
        b::memcpy(
            code.cast(),
            p::trampoline_32bit_src as *const () as *const c_void,
            CODE_SIZE,
        );

        // The LJMP's immediate is a possibly unaligned 32-bit field in the
        // copied instruction stream. C truncates the adjusted address to u32.
        let immediate = code
            .add(p::trampoline_ljmp_imm_offset as usize)
            .cast::<u32>();
        write_unaligned(
            immediate,
            read_unaligned(immediate).wrapping_add(code as usize as u32),
        );

        if l5_required {
            // 4 -> 5: the sole top-level entry points to the current CR3.
            trampoline
                .write(native_read_cr3_pa() | p::LUPOS_BOOT_PGTABLE_PAGE_TABLE_NOENC as c_ulong);
        } else {
            // 5 -> 4: the first PGD entry's child can reside above 4 GiB,
            // so copy that table to the low-memory trampoline before switching.
            let pgdp = native_read_cr3_pa() as *const p::pgd_t;
            let pgd = read_unaligned(addr_of!((*pgdp).pgd))
                & p::LUPOS_BOOT_PGTABLE_PGD_ALLOWED_BITS as c_ulong;
            let new_cr3 = (pgd & pte_pfn_mask()) as *const c_void;
            b::memcpy(trampoline.cast(), new_cr3, PAGE_SIZE);
        }

        // la57toggle.S takes only RDI and toggles CR4.LA57. The copied code is
        // invoked with pgtable_64.c's one-argument callback ABI, regardless of
        // the two-argument prototype used solely to name the source template.
        let toggle_la57: unsafe extern "C" fn(*mut c_void) = transmute(code);
        toggle_la57(trampoline.cast());

        // Move the live table before restoring the low memory bytes. The CR3
        // write must retain native_write_cr3's compiler memory barrier.
        b::memcpy(pgtable, trampoline.cast(), PAGE_SIZE);
        asm!("mov cr3, {}", in(reg) pgtable as c_ulong, options(nostack, preserves_flags));
        b::memcpy(trampoline.cast(), saved.cast(), TRAMPOLINE_SIZE);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
