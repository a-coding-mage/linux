// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Translated from the Linux kernel C implementation. Kernel-provided
// declarations and configuration constants are expected from other files.

const ROCIT_REG_BASE: usize = 0x1f403000;
const ROCIT_CONFIG_GEN1: usize = ROCIT_REG_BASE + 0x04;
const ROCIT_CONFIG_GEN1_MEMMAP_SHIFT: u32 = 8;
const ROCIT_CONFIG_GEN1_MEMMAP_MASK: u32 = 0xf << 8;

#[repr(align(8))]
static mut FDT_BUF: [u8; 16 << 10] = [0; 16 << 10];

extern "C" {
    static mut physical_memsize: libc::c_ulong;
    static mut arcs_cmdline: *mut libc::c_char;

    fn fw_getenv(name: *const libc::c_char) -> *mut libc::c_char;
    fn mips_cm_probe() -> libc::c_int;
    fn read_gcr_gic_status() -> u32;
    fn readl(addr: *const libc::c_void) -> u32;
    fn ioremap(addr: usize, size: usize) -> *mut libc::c_void;
    fn __raw_readl(addr: *const libc::c_void) -> u32;
    fn __raw_writel(value: u32, addr: *mut libc::c_void);
    fn fdt_check_header(fdt: *const libc::c_void) -> libc::c_int;
    fn fdt_open_into(fdt: *const libc::c_void, buf: *mut libc::c_void, size: libc::c_int) -> libc::c_int;
    fn fdt_path_offset(fdt: *const libc::c_void, path: *const libc::c_char) -> libc::c_int;
    fn fdt_getprop(fdt: *const libc::c_void, node: libc::c_int, name: *const libc::c_char, len: *mut libc::c_int) -> *const libc::c_char;
    fn fdt_add_subnode(fdt: *mut libc::c_void, parent: libc::c_int, name: *const libc::c_char) -> libc::c_int;
    fn fdt_setprop_string(fdt: *mut libc::c_void, node: libc::c_int, name: *const libc::c_char, value: *const libc::c_char) -> libc::c_int;
    fn fdt_setprop(fdt: *mut libc::c_void, node: libc::c_int, name: *const libc::c_char, value: *const libc::c_void, len: libc::c_int) -> libc::c_int;
    fn fdt_node_offset_by_compatible(fdt: *const libc::c_void, start: libc::c_int, compatible: *const libc::c_char) -> libc::c_int;
    fn fdt_nop_node(fdt: *mut libc::c_void, node: libc::c_int) -> libc::c_int;
    fn fdt_get_phandle(fdt: *const libc::c_void, node: libc::c_int) -> u32;
    fn fdt_setprop_u32(fdt: *mut libc::c_void, node: libc::c_int, name: *const libc::c_char, value: u32) -> libc::c_int;
    fn fdt_pack(fdt: *mut libc::c_void) -> libc::c_int;
    fn kstrtoul(s: *const libc::c_char, base: u32, result: *mut libc::c_ulong) -> libc::c_int;
    fn memparse(s: *const libc::c_char, retptr: *mut *mut libc::c_char) -> libc::c_ulong;
    fn snprintf(s: *mut libc::c_char, n: usize, format: *const libc::c_char, ...) -> libc::c_int;
    fn strlen(s: *const libc::c_char) -> usize;
    fn strstr(haystack: *const libc::c_char, needle: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(a: *const libc::c_char, b: *const libc::c_char, n: usize) -> libc::c_int;
    fn panic(message: *const libc::c_char, ... ) -> !;
}

#[repr(u32)]
enum MemMap { V1 = 0, V2 }

const MAX_MEM_ARRAY_ENTRIES: usize = 2;

unsafe fn malta_scon() -> libc::c_int {
    let scon = MIPS_REVISION_SCONID;
    if scon != MIPS_REVISION_SCON_OTHER { return scon; }
    match MIPS_REVISION_CORID {
        MIPS_REVISION_CORID_QED_RM5261 | MIPS_REVISION_CORID_CORE_LV |
        MIPS_REVISION_CORID_CORE_FPGA | MIPS_REVISION_CORID_CORE_FPGAR2 => MIPS_REVISION_SCON_GT64120,
        MIPS_REVISION_CORID_CORE_EMUL_BON | MIPS_REVISION_CORID_BONITO64 |
        MIPS_REVISION_CORID_CORE_20K => MIPS_REVISION_SCON_BONITO,
        MIPS_REVISION_CORID_CORE_MSC | MIPS_REVISION_CORID_CORE_FPGA2 |
        MIPS_REVISION_CORID_CORE_24K => MIPS_REVISION_SCON_SOCIT,
        _ => MIPS_REVISION_SCON_ROCIT,
    }
}

unsafe fn gen_fdt_mem_array(mem_array: *mut u32, mut size: libc::c_ulong, map: MemMap) -> u32 {
    let mut entries = 1u32;
    *mem_array = cpu_to_be32(PHYS_OFFSET);
    if IS_ENABLED_CONFIG_EVA {
        *mem_array.add(1) = cpu_to_be32(size as u32);
        return entries;
    }
    let size_preio = core::cmp::min(size, SZ_256M as libc::c_ulong);
    *mem_array.add(1) = cpu_to_be32(size_preio as u32);
    size -= size_preio;
    if size == 0 { return entries; }
    if map as u32 == MemMap::V2 as u32 {
        if size <= SZ_256M as libc::c_ulong { return entries; }
        size -= SZ_256M as libc::c_ulong;
        entries += 1;
        *mem_array.add(2) = cpu_to_be32((PHYS_OFFSET + SZ_512M) as u32);
        *mem_array.add(3) = cpu_to_be32(size as u32);
    } else {
        entries += 1;
        *mem_array.add(2) = cpu_to_be32((PHYS_OFFSET + SZ_2G + SZ_256M) as u32);
        *mem_array.add(3) = cpu_to_be32(size as u32);
    }
    BUG_ON(entries as usize > MAX_MEM_ARRAY_ENTRIES);
    entries
}

unsafe fn append_memory(fdt: *mut libc::c_void, root_off: libc::c_int) {
    let mut mem_array = [0u32; 2 * MAX_MEM_ARRAY_ENTRIES];
    let mut memsize: libc::c_ulong;
    let mut mem_entries: u32;
    let mut mem_off = fdt_path_offset(fdt, b"/memory\0".as_ptr() as _);
    if mem_off >= 0 { return; }
    let names = [b"ememsize\0".as_ptr(), b"memsize\0".as_ptr()];
    for name in names {
        let var = fw_getenv(name as _);
        if var.is_null() { continue; }
        if kstrtoul(var, 0, &mut physical_memsize) == 0 { break; }
    }
    if physical_memsize == 0 { physical_memsize = 32 << 20; }
    if IS_ENABLED_CONFIG_CPU_BIG_ENDIAN { physical_memsize -= PAGE_SIZE; }
    memsize = physical_memsize;
    for name in names {
        let mut param_name = [0i8; 10];
        snprintf(param_name.as_mut_ptr(), param_name.len(), b"%s=\0".as_ptr() as _, name);
        let var = strstr(arcs_cmdline, param_name.as_ptr());
        if !var.is_null() { memsize = memparse(var.add(strlen(param_name.as_ptr())), core::ptr::null_mut()); }
    }
    physical_memsize = core::cmp::max(physical_memsize, memsize);
    let mem_map = if malta_scon() == MIPS_REVISION_SCON_ROCIT {
        let config = readl(CKSEG1ADDR(ROCIT_CONFIG_GEN1) as *const _);
        MemMap::from_u32((config & ROCIT_CONFIG_GEN1_MEMMAP_MASK) >> ROCIT_CONFIG_GEN1_MEMMAP_SHIFT)
    } else { MemMap::V1 };
    mem_off = fdt_add_subnode(fdt, root_off, b"memory\0".as_ptr() as _);
    if mem_off < 0 { panic(b"Unable to add memory node to DT: %d\0".as_ptr() as _, mem_off); }
    let err = fdt_setprop_string(fdt, mem_off, b"device_type\0".as_ptr() as _, b"memory\0".as_ptr() as _);
    if err != 0 { panic(b"Unable to set memory node device_type: %d\0".as_ptr() as _, err); }
    mem_entries = gen_fdt_mem_array(mem_array.as_mut_ptr(), physical_memsize, mem_map);
    let err = fdt_setprop(fdt, mem_off, b"reg\0".as_ptr() as _, mem_array.as_ptr() as _, (mem_entries * 2 * 4) as _);
    if err != 0 { panic(b"Unable to set memory regs property: %d\0".as_ptr() as _, err); }
    mem_entries = gen_fdt_mem_array(mem_array.as_mut_ptr(), memsize, mem_map);
    let err = fdt_setprop(fdt, mem_off, b"linux,usable-memory\0".as_ptr() as _, mem_array.as_ptr() as _, (mem_entries * 2 * 4) as _);
    if err != 0 { panic(b"Unable to set linux,usable-memory property: %d\0".as_ptr() as _, err); }
}

unsafe fn remove_gic(fdt: *mut libc::c_void) {
    let err = mips_cm_probe();
    if err == 0 && (read_gcr_gic_status() & CM_GCR_GIC_STATUS_EX) != 0 { return; }
    if malta_scon() == MIPS_REVISION_SCON_ROCIT {
        let biu_base = ioremap(MSC01_BIU_REG_BASE, MSC01_BIU_ADDRSPACE_SZ);
        let mut sc_cfg = __raw_readl((biu_base as *mut u8).add(MSC01_SC_CFG_OFS) as _);
        if sc_cfg & MSC01_SC_CFG_GICPRES_MSK != 0 {
            sc_cfg |= BIT(MSC01_SC_CFG_GICENA_SHF);
            __raw_writel(sc_cfg, (biu_base as *mut u8).add(MSC01_SC_CFG_OFS) as _);
            return;
        }
    }
    let gic_off = fdt_node_offset_by_compatible(fdt, -1, b"mti,gic\0".as_ptr() as _);
    if gic_off < 0 { return; }
    fdt_nop_node(fdt, gic_off);
    let i8259_off = fdt_node_offset_by_compatible(fdt, -1, b"intel,i8259\0".as_ptr() as _);
    if i8259_off < 0 { return; }
    let cpu_off = fdt_node_offset_by_compatible(fdt, -1, b"mti,cpu-interrupt-controller\0".as_ptr() as _);
    if cpu_off < 0 { return; }
    let cpu_phandle = fdt_get_phandle(fdt, cpu_off);
    if cpu_phandle == 0 { return; }
    if fdt_setprop_u32(fdt, i8259_off, b"interrupt-parent\0".as_ptr() as _, cpu_phandle) != 0 { return; }
    fdt_setprop_u32(fdt, i8259_off, b"interrupts\0".as_ptr() as _, 2);
}

unsafe fn malta_dt_shim(fdt: *mut libc::c_void) -> *mut libc::c_void {
    if fdt_check_header(fdt) != 0 { panic(b"Corrupt DT\0".as_ptr() as _); }
    if fdt_open_into(fdt, FDT_BUF.as_mut_ptr() as _, FDT_BUF.len() as _) != 0 { panic(b"Unable to open FDT\0".as_ptr() as _); }
    let root_off = fdt_path_offset(FDT_BUF.as_ptr() as _, b"/\0".as_ptr() as _);
    if root_off < 0 { panic(b"No / node in DT\0".as_ptr() as _); }
    let mut len = 0;
    let compat = fdt_getprop(FDT_BUF.as_ptr() as _, root_off, b"compatible\0".as_ptr() as _, &mut len);
    if compat.is_null() { panic(b"No root compatible property in DT: %d\0".as_ptr() as _, len); }
    if strncmp(compat, b"mti,malta\0".as_ptr() as _, len as usize) != 0 { return fdt; }
    append_memory(FDT_BUF.as_mut_ptr() as _, root_off);
    remove_gic(FDT_BUF.as_mut_ptr() as _);
    if fdt_pack(FDT_BUF.as_mut_ptr() as _) != 0 { panic(b"Unable to pack FDT: %d\n\0".as_ptr() as _); }
    FDT_BUF.as_mut_ptr() as _
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
