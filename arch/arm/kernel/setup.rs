// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/setup.c
 *
 *  Copyright (C) 1995-2001 Russell King
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/* C kernel headers and configuration macros are supplied by the surrounding
 * translation unit; their declarations are intentionally not reimplemented. */

#[cfg(any(feature = "CONFIG_FPE_NWFPE", feature = "CONFIG_FPE_FASTFPE"))]
static mut fpe_type: [u8; 8] = [0; 8];

#[cfg(any(feature = "CONFIG_FPE_NWFPE", feature = "CONFIG_FPE_FASTFPE"))]
unsafe fn fpe_setup(line: *mut i8) -> i32 {
    core::ptr::copy_nonoverlapping(line as *const u8, fpe_type.as_mut_ptr(), 8);
    1
}

pub static mut processor_id: u32 = 0;
pub static mut __machine_arch_type: u32 = 0;
pub static mut cacheid: u32 = 0;
pub static mut __atags_pointer: u32 = 0;
pub static mut system_rev: u32 = 0;
pub static mut system_serial: *const i8 = core::ptr::null();
pub static mut system_serial_low: u32 = 0;
pub static mut system_serial_high: u32 = 0;
pub static mut elf_hwcap: u32 = 0;
pub static mut elf_hwcap2: u32 = 0;

#[cfg(feature = "MULTI_CPU")]
static mut processor: processor = unsafe { core::mem::zeroed() };
#[cfg(feature = "MULTI_TLB")]
static mut cpu_tlb: cpu_tlb_fns = unsafe { core::mem::zeroed() };
#[cfg(feature = "MULTI_USER")]
static mut cpu_user: cpu_user_fns = unsafe { core::mem::zeroed() };
#[cfg(feature = "MULTI_CACHE")]
static mut cpu_cache: cpu_cache_fns = unsafe { core::mem::zeroed() };
#[cfg(feature = "CONFIG_OUTER_CACHE")]
static mut outer_cache: outer_cache_fns = unsafe { core::mem::zeroed() };

pub static mut __cpu_architecture: i32 = CPU_ARCH_UNKNOWN;

#[repr(C)]
struct stack { irq: [u32; 4], abt: [u32; 4], und: [u32; 4], fiq: [u32; 4] }
#[cfg(not(feature = "CONFIG_CPU_V7M"))]
static mut stacks: [stack; NR_CPUS] = unsafe { core::mem::zeroed() };

pub static mut elf_platform: [i8; ELF_PLATFORM_SIZE] = [0; ELF_PLATFORM_SIZE];
static mut cpu_name: *const i8 = core::ptr::null();
static mut machine_name: *const i8 = core::ptr::null();
static mut cmd_line: [i8; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];
static mut machine_desc: *const machine_desc = core::ptr::null();

#[repr(C)]
union endian_test_union { c: [u8; 4], l: usize }
static mut endian_test: endian_test_union = endian_test_union { c: [b'l', b'?', b'?', b'b'] };

#[repr(C)]
struct stack_resource { name: *const i8, start: usize, end: usize, flags: usize }
static mut mem_res: [stack_resource; 3] = [
    stack_resource { name: b"Video RAM\0".as_ptr() as *const i8, start: 0, end: 0, flags: IORESOURCE_MEM },
    stack_resource { name: b"Kernel code\0".as_ptr() as *const i8, start: 0, end: 0, flags: IORESOURCE_SYSTEM_RAM },
    stack_resource { name: b"Kernel data\0".as_ptr() as *const i8, start: 0, end: 0, flags: IORESOURCE_SYSTEM_RAM },
];
static mut io_res: [stack_resource; 3] = [
    stack_resource { name: b"reserved\0".as_ptr() as *const i8, start: 0x3bc, end: 0x3be, flags: IORESOURCE_IO | IORESOURCE_BUSY },
    stack_resource { name: b"reserved\0".as_ptr() as *const i8, start: 0x378, end: 0x37f, flags: IORESOURCE_IO | IORESOURCE_BUSY },
    stack_resource { name: b"reserved\0".as_ptr() as *const i8, start: 0x278, end: 0x27f, flags: IORESOURCE_IO | IORESOURCE_BUSY },
];

static proc_arch: [&[u8]; 17] = [b"undefined/unknown", b"3", b"4", b"4T", b"5", b"5T", b"5TE", b"5TEJ", b"6TEJ", b"7", b"7M", b"?(12)", b"?(13)", b"?(14)", b"?(15)", b"?(16)", b"?(17)"];

unsafe fn __get_cpu_architecture() -> i32 {
    #[cfg(feature = "CONFIG_CPU_V7M")]
    { return CPU_ARCH_ARMv7M; }
    #[cfg(not(feature = "CONFIG_CPU_V7M"))]
    {
        let id = read_cpuid_id();
        if id & 0x0008f000 == 0 { CPU_ARCH_UNKNOWN }
        else if id & 0x0008f000 == 0x00007000 { if id & (1 << 23) != 0 { CPU_ARCH_ARMv4T } else { CPU_ARCH_ARMv3 } }
        else if id & 0x00080000 == 0 { let a = (id >> 16) & 7; if a != 0 { a as i32 + CPU_ARCH_ARMv3 } else { 0 } }
        else if id & 0x000f0000 == 0x000f0000 { let m = read_cpuid_ext(CPUID_EXT_MMFR0); if m & 0xf >= 3 || m & 0xf0 >= 0x30 { CPU_ARCH_ARMv7 } else if m & 0xf == 2 || m & 0xf0 == 0x20 { CPU_ARCH_ARMv6 } else { CPU_ARCH_UNKNOWN } }
        else { CPU_ARCH_UNKNOWN }
    }
}

unsafe fn cpu_architecture() -> i32 { BUG_ON(__cpu_architecture == CPU_ARCH_UNKNOWN); __cpu_architecture }

unsafe fn cpu_has_aliasing_icache(arch: u32) -> i32 {
    if icache_is_pipt() { return 0; }
    match arch as i32 {
        CPU_ARCH_ARMv7 => { set_csselr(CSSELR_ICACHE | CSSELR_L1); isb(); let id = read_ccsidr(); let line = 4 << ((id & 7) + 2); let sets = ((id >> 13) & 0x7fff) + 1; ((line * sets) > PAGE_SIZE) as i32 }
        CPU_ARCH_ARMv6 => (read_cpuid_cachetype() & (1 << 11)) as i32,
        _ => 0,
    }
}

unsafe fn cacheid_init() {
    let mut arch = cpu_architecture();
    if arch >= CPU_ARCH_ARMv6 {
        let ct = read_cpuid_cachetype();
        if arch == CPU_ARCH_ARMv7M && ct & 0xf000f == 0 { cacheid = 0; }
        else if ct & (7 << 29) == 4 << 29 { arch = CPU_ARCH_ARMv7; cacheid = CACHEID_VIPT_NONALIASING; match ct & (3 << 14) { 1 << 14 => cacheid |= CACHEID_ASID_TAGGED, 3 << 14 => cacheid |= CACHEID_PIPT, _ => {} } }
        else { arch = CPU_ARCH_ARMv6; cacheid = if ct & (1 << 23) != 0 { CACHEID_VIPT_ALIASING } else { CACHEID_VIPT_NONALIASING }; }
        if cpu_has_aliasing_icache(arch as u32) != 0 { cacheid |= CACHEID_VIPT_I_ALIASING; }
    } else { cacheid = CACHEID_VIVT; }
}

unsafe fn cpuid_init_hwcaps() {
    if cpu_architecture() < CPU_ARCH_ARMv7 { return; }
    let mut b = cpuid_feature_extract(CPUID_EXT_ISAR0, 24); if b >= 2 { elf_hwcap |= HWCAP_IDIVA; } if b >= 1 { elf_hwcap |= HWCAP_IDIVT; }
    b = cpuid_feature_extract(CPUID_EXT_MMFR0, 0); if b >= 5 { elf_hwcap |= HWCAP_LPAE; }
    let a = read_cpuid_ext(CPUID_EXT_ISAR5); b = cpuid_feature_extract_field(a, 4); if b >= 2 { elf_hwcap2 |= HWCAP2_PMULL; } if b >= 1 { elf_hwcap2 |= HWCAP2_AES; }
    b = cpuid_feature_extract_field(a, 8); if b >= 1 { elf_hwcap2 |= HWCAP2_SHA1; }
    b = cpuid_feature_extract_field(a, 12); if b >= 1 { elf_hwcap2 |= HWCAP2_SHA2; }
    b = cpuid_feature_extract_field(a, 16); if b >= 1 { elf_hwcap2 |= HWCAP2_CRC32; }
    b = cpuid_feature_extract_field(read_cpuid_ext(CPUID_EXT_ISAR6), 12); if b >= 1 { elf_hwcap2 |= HWCAP2_SB; }
    b = cpuid_feature_extract_field(read_cpuid_ext(CPUID_EXT_PFR2), 4); if b >= 1 { elf_hwcap2 |= HWCAP2_SSBS; }
}

unsafe fn elf_hwcap_fixup() { let id = read_cpuid_id(); if read_cpuid_part() == ARM_CPU_PART_ARM1136 && ((id >> 20) & 3) == 0 { elf_hwcap &= !HWCAP_TLS; return; } if id & 0x000f0000 != 0x000f0000 { return; } let x = cpuid_feature_extract(CPUID_EXT_ISAR3, 12); if x > 1 || x == 1 && cpuid_feature_extract(CPUID_EXT_ISAR4, 20) >= 3 { elf_hwcap &= !HWCAP_SWP; } }

unsafe fn smp_setup_processor_id() { let mpidr = if is_smp() { read_cpuid_mpidr() & MPIDR_HWID_BITMASK } else { 0 }; let cpu = MPIDR_AFFINITY_LEVEL(mpidr, 0); cpu_logical_map(0) = cpu; for i in 1..nr_cpu_ids { cpu_logical_map(i) = if i == cpu { 0 } else { i }; } set_my_cpu_offset(0); pr_info!("Booting Linux on physical CPU 0x%x\n", mpidr); }

unsafe fn arm_add_memory(mut start: u64, mut size: u64) -> i32 { let aligned = PAGE_ALIGN(start); if aligned > start + size { size = 0; } else { size -= aligned - start; } if aligned < PHYS_OFFSET { if aligned + size <= PHYS_OFFSET { return -EINVAL; } size -= PHYS_OFFSET - aligned; start = PHYS_OFFSET; } else { start = aligned; } size &= !(PAGE_SIZE as u64 - 1); if size == 0 { return -EINVAL; } memblock_add(start, size); 0 }

unsafe fn hyp_mode_check() { #[cfg(feature = "CONFIG_ARM_VIRT_EXT")] { sync_boot_mode(); if is_hyp_mode_available() { pr_info!("CPU: All CPU(s) started in HYP mode.\n"); pr_info!("CPU: Virtualization extensions available.\n"); } else if is_hyp_mode_mismatched() { pr_warn!("CPU: WARNING: CPU(s) started in wrong/inconsistent modes\n"); } else { pr_info!("CPU: All CPU(s) started in SVC mode.\n"); } } }

unsafe fn customize_machine() -> i32 { if !machine_desc.is_null() && (*machine_desc).init_machine.is_some() { ((*machine_desc).init_machine.unwrap())(); } 0 }
unsafe fn init_machine_late() -> i32 { if !machine_desc.is_null() && (*machine_desc).init_late.is_some() { ((*machine_desc).init_late.unwrap())(); } 0 }

unsafe fn setup_arch(cmdline_p: *mut *mut i8) {
    let mut mdesc: *const machine_desc = core::ptr::null();
    let atags = if __atags_pointer != 0 { FDT_VIRT_BASE(__atags_pointer) } else { core::ptr::null_mut() };
    setup_processor();
    if !atags.is_null() { mdesc = setup_machine_fdt(atags); if !mdesc.is_null() { memblock_reserve(__atags_pointer as u64, fdt_totalsize(atags)); } }
    if mdesc.is_null() { mdesc = setup_machine_tags(atags, __machine_arch_type); }
    if mdesc.is_null() { early_print!("\nError: invalid dtb and unrecognized/unsupported machine ID\n"); dump_machine_table(); }
    machine_desc = mdesc; machine_name = (*mdesc).name; strscpy(cmd_line.as_mut_ptr(), boot_command_line, COMMAND_LINE_SIZE); *cmdline_p = cmd_line.as_mut_ptr();
    early_fixmap_init(); early_ioremap_init(); parse_early_param(); setup_dma_zone(mdesc); xen_early_init(); arm_efi_init(); adjust_lowmem_bounds(); arm_memblock_init(mdesc); adjust_lowmem_bounds(); early_ioremap_reset(); paging_init(mdesc); kasan_init(); request_standard_resources(mdesc); unflatten_device_tree(); arm_dt_init_cpu_maps(); psci_dt_init(); if !is_smp() { hyp_mode_check(); } reserve_crashkernel(); if (*mdesc).init_early.is_some() { ((*mdesc).init_early.unwrap())(); }
}

unsafe fn arch_cpu_is_hotpluggable(num: i32) -> bool { platform_can_hotplug_cpu(num) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
