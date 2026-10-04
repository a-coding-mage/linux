// SPDX-License-Identifier: GPL-2.0
//! Compressed-boot RSDP discovery and SRAT immovable-memory parsing.
//! Keep the unchanged acpi.c and native ACPI headers as the behavioral source.

use crate::bindings as b;
use crate::boot_acpi_bindings as a;
#[cfg(CONFIG_EFI)]
use crate::boot_efi_bindings as e;
#[cfg(any(CONFIG_EFI, all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE)))]
use core::ffi::c_char;
#[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE))]
use core::ffi::{c_int, c_ulong};
#[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE))]
use core::mem::size_of;
#[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE))]
use core::ptr::addr_of_mut;
use core::ptr::{addr_of, read_unaligned};

// The C definition exists whenever acpi.c is selected, even if the SRAT parser
// is disabled. Its configured capacity comes from the original NUMA headers.
const IMMOVABLE_MEM_CAPACITY: usize = a::LUPOS_BOOT_ACPI_MAX_NUMNODES as usize * 2;
#[no_mangle]
pub(crate) static mut immovable_mem: [b::mem_vector; IMMOVABLE_MEM_CAPACITY] =
    [b::mem_vector { start: 0, size: 0 }; IMMOVABLE_MEM_CAPACITY];

#[inline]
#[cfg(all(
    CONFIG_X86_VERBOSE_BOOTUP,
    any(CONFIG_EFI, all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE))
))]
unsafe fn debug_putstr(message: *const c_char) {
    // SAFETY: callers supply the original terminated boot-console strings.
    unsafe { b::__putstr(message) }
}

#[inline]
#[cfg(all(
    not(CONFIG_X86_VERBOSE_BOOTUP),
    any(CONFIG_EFI, all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE))
))]
unsafe fn debug_putstr(_message: *const c_char) {}

#[cfg(CONFIG_EFI)]
unsafe fn __efi_get_rsdp_addr(
    cfg_tbl_pa: core::ffi::c_ulong,
    cfg_tbl_len: core::ffi::c_uint,
) -> b::acpi_physical_address {
    // SAFETY: EFI's native helpers validate the configured table interface;
    // boot_params_ptr and table storage have the same preconditions as acpi.c.
    unsafe {
        let rsdp_addr = e::efi_find_vendor_table(
            b::boot_params_ptr,
            cfg_tbl_pa,
            cfg_tbl_len,
            e::ACPI_20_TABLE_GUID,
        );
        if rsdp_addr != 0 {
            return rsdp_addr as b::acpi_physical_address;
        }

        let rsdp_addr = e::efi_find_vendor_table(
            b::boot_params_ptr,
            cfg_tbl_pa,
            cfg_tbl_len,
            e::ACPI_TABLE_GUID,
        );
        if rsdp_addr != 0 {
            return rsdp_addr as b::acpi_physical_address;
        }

        debug_putstr(c"Error getting RSDP address.\n".as_ptr());
        0
    }
}

#[cfg(CONFIG_EFI)]
unsafe fn efi_get_rsdp_addr() -> b::acpi_physical_address {
    // SAFETY: the early boot parameters and EFI helper ABI match acpi.c.
    unsafe {
        let mut cfg_tbl_pa = 0;
        let mut cfg_tbl_len = 0;
        if e::efi_get_type(b::boot_params_ptr) == e::EFI_TYPE_NONE {
            return 0;
        }

        if e::efi_get_system_table(b::boot_params_ptr) == 0 {
            b::error(
                c"EFI support advertised, but unable to locate system table."
                    .as_ptr()
                    .cast_mut(),
            );
        }

        let ret = e::efi_get_conf_table(b::boot_params_ptr, &mut cfg_tbl_pa, &mut cfg_tbl_len);
        if ret != 0 || cfg_tbl_pa == 0 {
            b::error(c"EFI config table not found.".as_ptr().cast_mut());
        }

        __efi_get_rsdp_addr(cfg_tbl_pa, cfg_tbl_len)
    }
}

#[cfg(not(CONFIG_EFI))]
unsafe fn efi_get_rsdp_addr() -> b::acpi_physical_address {
    0
}

unsafe fn compute_checksum(mut buffer: *const u8, length: u32) -> u8 {
    // SAFETY: the physical buffer is readable for the original checksum span.
    // Use byte loads and wrapping arithmetic for the C unsigned-byte sum.
    unsafe {
        let end = buffer.wrapping_add(length as usize);
        let mut sum = 0u8;
        while buffer < end {
            sum = sum.wrapping_add(*buffer);
            buffer = buffer.wrapping_add(1);
        }
        sum
    }
}

unsafe fn scan_mem_for_rsdp(start: *const u8, length: u32) -> *const u8 {
    // SAFETY: boot-time identity mapping makes the BIOS scan window readable.
    // As in C, candidate checks may extend beyond the last scan-step start.
    unsafe {
        let end = start.wrapping_add(length as usize);
        let mut address = start;
        while address < end {
            let rsdp = address.cast::<a::acpi_table_rsdp>();
            // ACPI_VALIDATE_RSDP_SIG receives the native eight-byte array.
            if b::strncmp(
                addr_of!((*rsdp).signature).cast(),
                a::ACPI_SIG_RSDP.as_ptr().cast(),
                8,
            ) == 0
                && compute_checksum(address, a::ACPI_RSDP_CHECKSUM_LENGTH as u32) == 0
                && (read_unaligned(addr_of!((*rsdp).revision)) < 2
                    || compute_checksum(address, a::ACPI_RSDP_XCHECKSUM_LENGTH as u32) == 0)
            {
                return address;
            }
            address = address.wrapping_add(a::ACPI_RSDP_SCAN_STEP as usize);
        }
        core::ptr::null()
    }
}

unsafe fn bios_get_rsdp_addr() -> b::acpi_physical_address {
    // SAFETY: preserve the BIOS-data-area read and identity-mapped BIOS/EBDA
    // windows used by the original early-boot routine.
    unsafe {
        let address = (read_unaligned(a::ACPI_EBDA_PTR_LOCATION as *const u16) as usize) << 4;
        if address > 0x400 {
            let rsdp = scan_mem_for_rsdp(address as *const u8, a::ACPI_EBDA_WINDOW_SIZE as u32);
            if !rsdp.is_null() {
                return rsdp as usize as b::acpi_physical_address;
            }
        }

        let rsdp = scan_mem_for_rsdp(
            a::ACPI_HI_RSDP_WINDOW_BASE as *const u8,
            a::ACPI_HI_RSDP_WINDOW_SIZE as u32,
        );
        if !rsdp.is_null() {
            return rsdp as usize as b::acpi_physical_address;
        }
        0
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn get_rsdp_addr() -> b::acpi_physical_address {
    // SAFETY: the boot owner has initialized boot_params_ptr. Firmware physical
    // addresses remain identity mapped, exactly as required by the C routine.
    unsafe {
        let mut pa = read_unaligned(addr_of!((*b::boot_params_ptr).acpi_rsdp_addr))
            as b::acpi_physical_address;
        if pa == 0 {
            pa = efi_get_rsdp_addr();
        }
        if pa == 0 {
            pa = bios_get_rsdp_addr();
        }
        pa
    }
}

#[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE, CONFIG_KEXEC_CORE))]
unsafe fn get_cmdline_acpi_rsdp() -> c_ulong {
    // SAFETY: the native cmdline/parser functions receive a live local buffer;
    // their ABI, returned full length, and parse-failure behavior match C.
    unsafe {
        // "0x" + sixteen hexadecimal digits + terminating NUL.
        const MAX_ADDR_LEN: usize = 19;
        let mut addr = 0;
        let mut val = [0 as c_char; MAX_ADDR_LEN];
        let ret =
            b::cmdline_find_option(c"acpi_rsdp".as_ptr(), val.as_mut_ptr(), val.len() as c_int);
        if ret < 0 {
            return 0;
        }
        if ret as usize >= val.len() {
            b::warn(c"acpi_rsdp= value too long; ignoring".as_ptr());
            return 0;
        }
        if b::boot_kstrtoul(val.as_ptr(), 16, &mut addr) != 0 {
            return 0;
        }
        addr
    }
}

#[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE, not(CONFIG_KEXEC_CORE)))]
unsafe fn get_cmdline_acpi_rsdp() -> c_ulong {
    0
}

#[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE))]
unsafe fn get_acpi_srat_table() -> c_ulong {
    // SAFETY: firmware supplies readable native packed tables under the same
    // assumptions as acpi.c. Unaligned loads avoid creating packed references.
    unsafe {
        let mut rsdp = get_cmdline_acpi_rsdp() as *const a::acpi_table_rsdp;
        if rsdp.is_null() {
            rsdp = read_unaligned(addr_of!((*b::boot_params_ptr).acpi_rsdp_addr)) as c_ulong
                as *const a::acpi_table_rsdp;
        }
        if rsdp.is_null() {
            return 0;
        }

        let mut arg = [0 as c_char; 10];
        let (root_table, entry_size) =
            if !(b::cmdline_find_option(c"acpi".as_ptr(), arg.as_mut_ptr(), arg.len() as c_int)
                == 4
                && b::strncmp(arg.as_ptr(), c"rsdt".as_ptr(), 4) == 0)
                && read_unaligned(addr_of!((*rsdp).xsdt_physical_address)) != 0
                && read_unaligned(addr_of!((*rsdp).revision)) > 1
            {
                (
                    read_unaligned(addr_of!((*rsdp).xsdt_physical_address)) as c_ulong,
                    a::LUPOS_BOOT_ACPI_XSDT_ENTRY_SIZE as u32,
                )
            } else {
                (
                    read_unaligned(addr_of!((*rsdp).rsdt_physical_address)) as c_ulong,
                    a::LUPOS_BOOT_ACPI_RSDT_ENTRY_SIZE as u32,
                )
            };
        if root_table == 0 {
            return 0;
        }

        let header = root_table as *const a::acpi_table_header;
        let len = read_unaligned(addr_of!((*header).length));
        if (len as usize) < size_of::<a::acpi_table_header>() + entry_size as usize {
            return 0;
        }

        let mut num_entries =
            ((len as usize - size_of::<a::acpi_table_header>()) / entry_size as usize) as u32;
        let mut entry =
            root_table.wrapping_add(size_of::<a::acpi_table_header>() as c_ulong) as *const u8;
        while num_entries != 0 {
            num_entries = num_entries.wrapping_sub(1);
            let acpi_table = if entry_size == a::LUPOS_BOOT_ACPI_RSDT_ENTRY_SIZE as u32 {
                read_unaligned(entry.cast::<u32>()) as c_ulong
            } else {
                read_unaligned(entry.cast::<u64>()) as c_ulong
            };
            if acpi_table != 0 {
                let header = acpi_table as *const a::acpi_table_header;
                // Native x86 ACPI_COMPARE_NAMESEG compares four bytes as u32.
                if read_unaligned(addr_of!((*header).signature).cast::<u32>())
                    == read_unaligned(a::ACPI_SIG_SRAT.as_ptr().cast::<u32>())
                {
                    return acpi_table;
                }
            }
            entry = entry.wrapping_add(entry_size as usize);
        }
        0
    }
}

#[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE))]
#[no_mangle]
pub(crate) unsafe extern "C" fn count_immovable_mem_regions() -> c_int {
    // SAFETY: the caller has exclusive access to early-boot state and readable
    // firmware tables. Preserve C's length checks, acceptance rules, and writes.
    unsafe {
        // Longest acpi= argument is "copy_dsdt", plus terminating NUL.
        const MAX_ACPI_ARG_LENGTH: usize = 10;
        let mut arg = [0 as c_char; MAX_ACPI_ARG_LENGTH];
        if b::cmdline_find_option(c"acpi".as_ptr(), arg.as_mut_ptr(), arg.len() as c_int) == 3
            && b::strncmp(arg.as_ptr(), c"off".as_ptr(), 3) == 0
        {
            return 0;
        }

        let table_addr = get_acpi_srat_table();
        if table_addr == 0 {
            return 0;
        }
        let table_header = table_addr as *const a::acpi_table_header;
        let table_end =
            table_addr.wrapping_add(read_unaligned(addr_of!((*table_header).length)) as c_ulong);
        let mut table = table_addr.wrapping_add(size_of::<a::acpi_table_srat>() as c_ulong);
        let mut num: c_int = 0;
        while table.wrapping_add(size_of::<a::acpi_subtable_header>() as c_ulong) < table_end {
            let sub_table = table as *const a::acpi_subtable_header;
            if read_unaligned(addr_of!((*sub_table).length)) == 0 {
                debug_putstr(c"Invalid zero length SRAT subtable.\n".as_ptr());
                return 0;
            }

            if read_unaligned(addr_of!((*sub_table).type_))
                == a::ACPI_SRAT_TYPE_MEMORY_AFFINITY as u8
            {
                let ma = sub_table.cast::<a::acpi_srat_mem_affinity>();
                if read_unaligned(addr_of!((*ma).flags)) & a::ACPI_SRAT_MEM_HOT_PLUGGABLE as u32
                    == 0
                    && read_unaligned(addr_of!((*ma).length)) != 0
                {
                    let region = addr_of_mut!(immovable_mem)
                        .cast::<b::mem_vector>()
                        .add(num as usize);
                    (*region).start = read_unaligned(addr_of!((*ma).base_address));
                    (*region).size = read_unaligned(addr_of!((*ma).length));
                    num += 1;
                }

                // C rejects at capacity after recording the last region, and
                // leaves previously recorded state intact on every failure.
                if num as usize >= IMMOVABLE_MEM_CAPACITY {
                    debug_putstr(c"Too many immovable memory regions, aborting.\n".as_ptr());
                    return 0;
                }
            }
            table = table.wrapping_add(read_unaligned(addr_of!((*sub_table).length)) as c_ulong);
        }
        num
    }
}
