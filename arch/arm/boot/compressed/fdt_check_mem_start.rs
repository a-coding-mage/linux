// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_void};

type Fdt32 = u32;

const FDT_MAGIC: u32 = 0xd00dfeed;
const SZ_2M: u32 = 2 * 1024 * 1024;

unsafe extern "C" {
    fn fdt_path_offset(fdt: *const c_void, path: *const c_char) -> c_int;
    fn fdt_getprop(
        fdt: *const c_void,
        nodeoffset: c_int,
        name: *const c_char,
        lenp: *mut c_int,
    ) -> *const c_void;
    fn fdt_magic(fdt: *const c_void) -> u32;
    fn fdt32_ld(ptr: *const Fdt32) -> u32;
    fn fdt_next_node(fdt: *const c_void, offset: c_int, depth: *mut c_int) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn round_up(value: u32, alignment: u32) -> u32;
}

unsafe fn get_prop(
    fdt: *const c_void,
    node_path: *const c_char,
    property: *const c_char,
    minlen: c_int,
) -> *const c_void {
    let mut len: c_int = 0;
    let offset = unsafe { fdt_path_offset(fdt, node_path) };
    if offset < 0 {
        return core::ptr::null();
    }

    let prop = unsafe { fdt_getprop(fdt, offset, property, &mut len) };
    if prop.is_null() || len < minlen {
        return core::ptr::null();
    }

    prop
}

unsafe fn get_cells(fdt: *const c_void, name: *const c_char) -> u32 {
    let root = c"/".as_ptr();
    let prop = unsafe { get_prop(fdt, root, name, core::mem::size_of::<Fdt32>() as c_int) }
        as *const Fdt32;

    if prop.is_null() {
        /* default */
        return 1;
    }

    unsafe { fdt32_ld(prop) }
}

unsafe fn get_val(cells: *const Fdt32, ncells: u32) -> u64 {
    let mut r = unsafe { fdt32_ld(cells) } as u64;
    if ncells > 1 {
        r = (r << 32) | unsafe { fdt32_ld(cells.add(1)) } as u64;
    }

    r
}

/*
 * Check the start of physical memory
 *
 * Traditionally, the start address of physical memory is obtained by masking
 * the program counter.  However, this does require that this address is a
 * multiple of 128 MiB, precluding booting Linux on platforms where this
 * requirement is not fulfilled.
 * Hence validate the calculated address against the memory information in the
 * DTB, and, if out-of-range, replace it by the real start address.
 * To preserve backwards compatibility (systems reserving a block of memory
 * at the start of physical memory, kdump, ...), the traditional method is
 * used if it yields a valid address, unless the "linux,usable-memory-range"
 * property is present.
 *
 * Return value: start address of physical memory to use
 */
#[no_mangle]
pub unsafe extern "C" fn fdt_check_mem_start(mem_start: u32, fdt: *const c_void) -> u32 {
    let addr_cells: u32;
    let size_cells: u32;
    let mut usable_base: u32 = 0;
    let mut fdt_mem_start: u32 = 0xffff_ffff;
    let mut usable_end: u64 = 0;
    let mut usable: *const Fdt32;

    if fdt.is_null() || unsafe { fdt_magic(fdt) } != FDT_MAGIC {
        return mem_start;
    }

    /* There may be multiple cells on LPAE platforms */
    addr_cells = unsafe { get_cells(fdt, c"#address-cells".as_ptr()) };
    size_cells = unsafe { get_cells(fdt, c"#size-cells".as_ptr()) };
    if addr_cells > 2 || size_cells > 2 {
        return mem_start;
    }

    /*
     * Usable memory in case of a crash dump kernel
     * This property describes a limitation: memory within this range is
     * only valid when also described through another mechanism
     */
    usable = unsafe {
        get_prop(
            fdt,
            c"/chosen".as_ptr(),
            c"linux,usable-memory-range".as_ptr(),
            ((addr_cells + size_cells) * core::mem::size_of::<Fdt32>() as u32) as c_int,
        )
    } as *const Fdt32;
    if !usable.is_null() {
        let size = unsafe { get_val(usable.add(addr_cells as usize), size_cells) };
        if size == 0 {
            return mem_start;
        }
        if addr_cells > 1 && unsafe { fdt32_ld(usable) } != 0 {
            /* Outside 32-bit address space */
            return mem_start;
        }
        usable_base = unsafe { fdt32_ld(usable.add((addr_cells - 1) as usize)) };
        usable_end = usable_base as u64 + size;
    }

    /* Walk all memory nodes and regions */
    let mut offset = unsafe { fdt_next_node(fdt, -1, core::ptr::null_mut()) };
    while offset >= 0 {
        let type_ = unsafe { fdt_getprop(fdt, offset, c"device_type".as_ptr(), core::ptr::null_mut()) }
            as *const c_char;
        if !type_.is_null() && unsafe { strcmp(type_, c"memory".as_ptr()) } == 0 {
            let mut len: c_int = 0;
            let mut reg = unsafe { fdt_getprop(fdt, offset, c"linux,usable-memory".as_ptr(), &mut len) }
                as *const Fdt32;
            if reg.is_null() {
                reg = unsafe { fdt_getprop(fdt, offset, c"reg".as_ptr(), &mut len) } as *const Fdt32;
            }
            if !reg.is_null() {
                let endp = unsafe { reg.add((len as usize) / core::mem::size_of::<Fdt32>()) };
                while unsafe { endp.offset_from(reg) } >= (addr_cells + size_cells) as isize {
                    let size = unsafe { get_val(reg.add(addr_cells as usize), size_cells) };
                    if size != 0 && !(addr_cells > 1 && unsafe { fdt32_ld(reg) } != 0) {
                        let mut base = unsafe { fdt32_ld(reg.add((addr_cells - 1) as usize)) };
                        let mut end = base as u64 + size;
                        if !usable.is_null() {
                            if base < usable_base { base = usable_base; }
                            if end > usable_end { end = usable_end; }
                            if end > base as u64 && base < fdt_mem_start { fdt_mem_start = base; }
                        } else if (mem_start >= base) && (mem_start as u64 < end) {
                            return mem_start;
                        } else if base < fdt_mem_start { fdt_mem_start = base; }
                    }
                    reg = unsafe { reg.add((addr_cells + size_cells) as usize) };
                }
            }
        }
        offset = unsafe { fdt_next_node(fdt, offset, core::ptr::null_mut()) };
    }

    if fdt_mem_start == 0xffff_ffff {
        /* No usable memory found, falling back to default */
        return mem_start;
    }

    /* The calculated address is not usable, or was overridden by the property. */
    unsafe { round_up(fdt_mem_start, SZ_2M) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
