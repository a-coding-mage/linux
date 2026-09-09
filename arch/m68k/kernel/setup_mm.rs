// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/kernel/setup.c
 *
 *  Copyright (C) 1995  Hamish Macdonald
 */

/* This file handles the architecture-dependent parts of system setup. */

// C headers and configuration-provided symbols are supplied by the surrounding
// translation unit/build environment.

pub static mut m68k_machtype: ::core::ffi::c_ulong = 0;
pub static mut m68k_cputype: ::core::ffi::c_ulong = 0;
pub static mut m68k_fputype: ::core::ffi::c_ulong = 0;
pub static mut m68k_mmutype: ::core::ffi::c_ulong = 0;
pub static mut m68k_is040or060: ::core::ffi::c_int = 0;
extern "C" {
    static mut availmem: ::core::ffi::c_ulong;
}
pub static mut m68k_num_memory: ::core::ffi::c_int = 0;
pub static mut m68k_realnum_memory: ::core::ffi::c_int = 0;
pub static mut m68k_memoffset: ::core::ffi::c_ulong = 0;
pub static mut m68k_memory: [m68k_mem_info; NUM_MEMINFO as usize] = [m68k_mem_info { addr: 0, size: 0 }; NUM_MEMINFO as usize];
static mut m68k_ramdisk: m68k_mem_info = m68k_mem_info { addr: 0, size: 0 };
static mut m68k_command_line: [::core::ffi::c_char; CL_SIZE as usize] = [0; CL_SIZE as usize];

pub static mut mach_sched_init: Option<unsafe extern "C" fn()> = None;
pub static mut mach_init_IRQ: Option<unsafe extern "C" fn()> = None;
pub static mut mach_get_model: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char)> = None;
pub static mut mach_get_hardware_list: Option<unsafe extern "C" fn(*mut seq_file)> = None;
pub static mut mach_reset: Option<unsafe extern "C" fn()> = None;
pub static mut mach_halt: Option<unsafe extern "C" fn()> = None;

const MASK_256K: u32 = 0xfffc0000;

unsafe fn m68k_parse_bootinfo(mut record: *const bi_record) {
    let first_record = record;
    loop {
        let tag = be16_to_cpu((*record).tag);
        if tag == BI_LAST { break; }
        let mut unknown = false;
        let data = (*record).data as *const u8;
        let size = be16_to_cpu((*record).size);
        match tag {
            BI_MACHTYPE | BI_CPUTYPE | BI_FPUTYPE | BI_MMUTYPE => {}
            BI_MEMCHUNK => {
                if m68k_num_memory < NUM_MEMINFO {
                    let m = data as *const mem_info;
                    m68k_memory[m68k_num_memory as usize].addr = be32_to_cpu((*m).addr);
                    m68k_memory[m68k_num_memory as usize].size = be32_to_cpu((*m).size);
                    m68k_num_memory += 1;
                } else { pr_warn!("{}: too many memory chunks\n", "m68k_parse_bootinfo"); }
            }
            BI_RAMDISK => {
                let m = data as *const mem_info;
                m68k_ramdisk.addr = be32_to_cpu((*m).addr);
                m68k_ramdisk.size = be32_to_cpu((*m).size);
            }
            BI_COMMAND_LINE => { strscpy(m68k_command_line.as_mut_ptr(), data); }
            BI_RNG_SEED => {
                let len = be16_to_cpup(data);
                add_bootloader_randomness(data.add(2), len);
                memzero_explicit(data as *mut ::core::ffi::c_void, len as usize + 2);
            }
            _ => {
                unknown = if MACH_IS_AMIGA { amiga_parse_bootinfo(record) != 0 }
                    else if MACH_IS_ATARI { atari_parse_bootinfo(record) != 0 }
                    else if MACH_IS_MAC { mac_parse_bootinfo(record) != 0 }
                    else if MACH_IS_Q40 { q40_parse_bootinfo(record) != 0 }
                    else if MACH_IS_BVME6000 { bvme6000_parse_bootinfo(record) != 0 }
                    else if MACH_IS_MVME16x { mvme16x_parse_bootinfo(record) != 0 }
                    else if MACH_IS_MVME147 { mvme147_parse_bootinfo(record) != 0 }
                    else if MACH_IS_HP300 { hp300_parse_bootinfo(record) != 0 }
                    else if MACH_IS_APOLLO { apollo_parse_bootinfo(record) != 0 }
                    else if MACH_IS_VIRT { virt_parse_bootinfo(record) != 0 }
                    else { true };
            }
        }
        if unknown { pr_warn!("m68k_parse_bootinfo: unknown tag 0x{:04x} ignored\n", tag); }
        record = ((record as usize) + size as usize) as *const bi_record;
    }
    save_bootinfo(first_record);
    m68k_realnum_memory = m68k_num_memory;
    // CONFIG_SINGLE_MEMORY_CHUNK: preserve the source's build-time condition.
    if m68k_num_memory > 1 { m68k_num_memory = 1; }
}

pub unsafe extern "C" fn setup_arch(cmdline_p: *mut *mut ::core::ffi::c_char) {
    if !CPU_IS_COLDFIRE { m68k_parse_bootinfo(_end as *const bi_record); }
    if CPU_IS_040 { m68k_is040or060 = 4; } else if CPU_IS_060 { m68k_is040or060 = 6; }
    setup_initial_init_mm(PAGE_OFFSET as *mut _, _etext, _edata, _end);
    process_uboot_commandline(m68k_command_line.as_mut_ptr(), CL_SIZE);
    *cmdline_p = m68k_command_line.as_mut_ptr();
    memcpy(boot_command_line, *cmdline_p, CL_SIZE);
    jump_label_init();
    parse_early_param();
    match m68k_machtype {
        MACH_AMIGA => config_amiga(), MACH_ATARI => config_atari(), MACH_MAC => config_mac(),
        MACH_SUN3 => config_sun3(), MACH_APOLLO => config_apollo(), MACH_MVME147 => config_mvme147(),
        MACH_MVME16x => config_mvme16x(), MACH_BVME6000 => config_bvme6000(), MACH_HP300 => config_hp300(),
        MACH_Q40 => config_q40(), MACH_SUN3X => config_sun3x(), MACH_VIRT => config_virt(),
        MACH_M54XX | MACH_M5441X => { cf_bootmem_alloc(); cf_mmu_context_init(); config_BSP(core::ptr::null_mut(), 0); }
        _ => panic!("No configuration setup"),
    }
    if m68k_ramdisk.size != 0 { memblock_reserve(m68k_ramdisk.addr, m68k_ramdisk.size); }
    paging_init();
    if m68k_ramdisk.size != 0 {
        initrd_start = phys_to_virt(m68k_ramdisk.addr) as _;
        initrd_end = initrd_start + m68k_ramdisk.size;
        pr_info!("initrd: {:08x} - {:08x}\n", initrd_start, initrd_end);
    }
    nf_init();
    if MACH_IS_ATARI { atari_stram_reserve_pages(availmem as *mut _); }
    if MACH_IS_SUN3X { dvma_init(); }
}

unsafe fn show_cpuinfo(m: *mut seq_file, _v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let (cpu, factor) = if CPU_IS_020 { ("68020", 8) } else if CPU_IS_030 { ("68030", 8) }
        else if CPU_IS_040 { ("68040", 3) } else if CPU_IS_060 { ("68060", 1) }
        else if CPU_IS_COLDFIRE { ("ColdFire", 2) } else { ("680x0", 0) };
    let fpu = if m68k_fputype & FPU_68881 != 0 { "68881" } else if m68k_fputype & FPU_68882 != 0 { "68882" }
        else if m68k_fputype & FPU_68040 != 0 { "68040" } else if m68k_fputype & FPU_68060 != 0 { "68060" }
        else if m68k_fputype & FPU_SUNFPA != 0 { "Sun FPA" } else if m68k_fputype & FPU_COLDFIRE != 0 { "ColdFire" } else { "none" };
    let mmu = if m68k_mmutype & MMU_68851 != 0 { "68851" } else if m68k_mmutype & MMU_68030 != 0 { "68030" }
        else if m68k_mmutype & MMU_68040 != 0 { "68040" } else if m68k_mmutype & MMU_68060 != 0 { "68060" }
        else if m68k_mmutype & MMU_SUN3 != 0 { "Sun-3" } else if m68k_mmutype & MMU_APOLLO != 0 { "Apollo" }
        else if m68k_mmutype & MMU_COLDFIRE != 0 { "ColdFire" } else { "unknown" };
    seq_printf!(m, "CPU:\t\t{}\nMMU:\t\t{}\nFPU:\t\t{}\n", cpu, mmu, fpu);
    let _ = factor; 0
}

unsafe fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut ::core::ffi::c_void { if *pos < 1 { 1 as *mut _ } else { core::ptr::null_mut() } }
unsafe fn c_next(_m: *mut seq_file, _v: *mut ::core::ffi::c_void, pos: *mut loff_t) -> *mut ::core::ffi::c_void { *pos += 1; core::ptr::null_mut() }
unsafe fn c_stop(_m: *mut seq_file, _v: *mut ::core::ffi::c_void) {}

#[cfg(CONFIG_PROC_HARDWARE)]
unsafe fn hardware_proc_show(m: *mut seq_file, _v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut model = [0 as ::core::ffi::c_char; 80];
    if let Some(f) = mach_get_model { f(model.as_mut_ptr()); } else { strscpy(model.as_mut_ptr(), b"Unknown m68k\0".as_ptr() as *const _); }
    seq_printf!(m, "Model:\t\t{}\n", model.as_ptr());
    let mut mem: usize = 0;
    let mut i = 0;
    while i < m68k_num_memory { mem += m68k_memory[i as usize].size as usize; i += 1; }
    seq_printf!(m, "System Memory:\t{}K\n", mem >> 10);
    if let Some(f) = mach_get_hardware_list { f(m); }
    0
}

#[cfg(CONFIG_NVRAM)]
unsafe fn m68k_nvram_get_size() -> isize {
    if MACH_IS_ATARI { atari_nvram_get_size() } else if MACH_IS_MAC { mac_pram_get_size() } else { -ENODEV as isize }
}

#[cfg(CONFIG_MAC)]
unsafe fn m68k_nvram_read_byte(addr: ::core::ffi::c_int) -> u8 { if MACH_IS_MAC { mac_pram_read_byte(addr) } else { 0xff } }
#[cfg(CONFIG_MAC)]
unsafe fn m68k_nvram_write_byte(val: u8, addr: ::core::ffi::c_int) { if MACH_IS_MAC { mac_pram_write_byte(val, addr); } }

#[cfg(CONFIG_ATARI)]
unsafe fn m68k_nvram_read(buf: *mut ::core::ffi::c_char, count: usize, pos: *mut loff_t) -> isize {
    if MACH_IS_ATARI { atari_nvram_read(buf, count, pos) } else if MACH_IS_MAC { nvram_read_bytes(buf, count, pos) } else { -EINVAL as isize }
}
#[cfg(CONFIG_ATARI)]
unsafe fn m68k_nvram_write(buf: *mut ::core::ffi::c_char, count: usize, pos: *mut loff_t) -> isize {
    if MACH_IS_ATARI { atari_nvram_write(buf, count, pos) } else if MACH_IS_MAC { nvram_write_bytes(buf, count, pos) } else { -EINVAL as isize }
}
#[cfg(CONFIG_ATARI)]
unsafe fn m68k_nvram_set_checksum() -> ::core::ffi::c_long { if MACH_IS_ATARI { atari_nvram_set_checksum() } else { -EINVAL as _ } }
#[cfg(CONFIG_ATARI)]
unsafe fn m68k_nvram_initialize() -> ::core::ffi::c_long { if MACH_IS_ATARI { atari_nvram_initialize() } else { -EINVAL as _ } }

pub unsafe extern "C" fn arch_cpu_finalize_init() {
    if m68k_fputype == 0 { pr_emerg!("*** YOU DO NOT HAVE A FLOATING POINT UNIT, WHICH IS REQUIRED BY LINUX/M68K ***\n"); panic!("no FPU"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
