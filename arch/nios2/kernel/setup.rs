/*
 * Nios2-specific parts of system setup
 *
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 * Copyright (C) 2001 Vic Phillips <vic@microtronix.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C dependencies: linux/export.h, linux/kernel.h, linux/mm.h, linux/sched.h,
// linux/sched/task.h, linux/console.h, linux/memblock.h, linux/initrd.h,
// linux/of_fdt.h, asm/mmu_context.h, asm/sections.h, asm/setup.h,
// asm/cpuinfo.h

pub static mut memory_start: usize = 0;
pub static mut memory_end: usize = 0;

static mut fake_regs: pt_regs = pt_regs { fields: [0; 22] };

#[repr(C)]
pub struct pt_regs {
    pub fields: [usize; 22],
}

extern "C" {
    static exception_handler_hook: unsafe extern "C" fn();
    static fast_handler: unsafe extern "C" fn();
    static fast_handler_end: unsafe extern "C" fn();
    static mut initrd_start: usize;
    static mut initrd_end: usize;
    static mut boot_command_line: [core::ffi::c_char; COMMAND_LINE_SIZE];
    static mut min_low_pfn: usize;
    static mut max_low_pfn: usize;
    static mut max_pfn: usize;
    static mut cpuinfo: cpuinfo;
    static mut init_task: task_struct;
    static _stext: u8;
    static _etext: u8;
    static _edata: u8;
    static _end: u8;

    fn early_init_devtree(fdt: *mut core::ffi::c_void);
    fn strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, size: usize) -> isize;
    fn parse_early_param();
    fn memblock_get_current_limit() -> u64;
    fn memblock_start_of_DRAM() -> u64;
    fn memblock_end_of_DRAM() -> u64;
    fn memblock_set_current_limit(limit: u64);
    fn console_verbose();
    fn setup_initial_init_mm(stext: *const u8, etext: *const u8, edata: *const u8, end: *const u8);
    fn memblock_reserve(start: u64, size: u64);
    fn virt_to_phys(addr: *const core::ffi::c_void) -> u64;
    fn early_init_fdt_reserve_self();
    fn early_init_fdt_scan_reserved_mem();
    fn unflatten_and_copy_device_tree();
    fn setup_cpuinfo();
    fn mmu_init();
    fn mmu_context_init();
    fn paging_init();
}

#[repr(C)]
pub struct cpuinfo { pub exception_addr: u32, pub fast_tlb_miss_exc_addr: u32 }
#[repr(C)]
pub struct thread_struct { pub kregs: *mut pt_regs }
#[repr(C)]
pub struct task_struct { pub thread: thread_struct }

const COMMAND_LINE_SIZE: usize = 2048;

/* Copy a short hook instruction sequence to the exception address */
unsafe fn copy_exception_handler(addr: u32) {
    let start = &exception_handler_hook as *const _ as usize as u32;
    let mut tmp: u32 = 0;
    if start == addr { return; }
    // Original Nios2 volatile assembly copies and flushes three instructions.
    core::arch::asm!(
        "ldw {2},0({0})", "stw {2},0({1})", "ldw {2},4({0})", "stw {2},4({1})",
        "ldw {2},8({0})", "stw {2},8({1})", "flushd 0({1})", "flushd 4({1})",
        "flushd 8({1})", "flushi {1}", "addi {1},{1},4", "flushi {1}",
        "addi {1},{1},4", "flushi {1}", "flushp",
        in(reg) start, inout(reg) addr, inout(reg) tmp, options(nostack)
    );
}

/* Copy the fast TLB miss handler */
unsafe fn copy_fast_tlb_miss_handler(addr: u32) {
    let mut start = &fast_handler as *const _ as usize as u32;
    let end = &fast_handler_end as *const _ as usize as u32;
    let mut tmp: u32 = 0;
    core::arch::asm!(
        "1:", "ldw {3},0({0})", "stw {3},0({1})", "flushd 0({1})", "flushi {1}",
        "flushp", "addi {0},{0},4", "addi {1},{1},4", "bne {0},{2},1b",
        inout(reg) start, inout(reg) addr, in(reg) end, inout(reg) tmp, options(nostack)
    );
}

pub unsafe extern "C" fn nios2_boot_init(r4: u32, r5: u32, r6: u32, r7: u32) {
    let mut dtb_passed: u32 = 0;
    let mut cmdline_passed = [0i8; COMMAND_LINE_SIZE];
    // CONFIG_NIOS2_PASS_CMDLINE and related options are build-time conditions.
    if r4 == 0x534f494e {
        if r5 != 0 { initrd_start = r5 as usize; initrd_end = r6 as usize; }
        dtb_passed = r6;
        if r7 != 0 { strscpy(cmdline_passed.as_mut_ptr(), r7 as *const i8, COMMAND_LINE_SIZE); }
    }
    early_init_devtree(dtb_passed as usize as *mut core::ffi::c_void);
    if cmdline_passed[0] != 0 { strscpy(boot_command_line.as_mut_ptr(), cmdline_passed.as_ptr(), COMMAND_LINE_SIZE); }
    parse_early_param();
}

unsafe fn find_limits(min: *mut usize, max_low: *mut usize, max_high: *mut usize) {
    *max_low = (memblock_get_current_limit() >> 12) as usize;
    *min = ((memblock_start_of_DRAM() + 4095) >> 12) as usize;
    *max_high = (memblock_end_of_DRAM() >> 12) as usize;
}

unsafe fn adjust_lowmem_bounds() { memblock_set_current_limit(memblock_end_of_DRAM()); }

pub unsafe extern "C" fn setup_arch(cmdline_p: *mut *mut core::ffi::c_char) {
    console_verbose();
    memory_start = memblock_start_of_DRAM() as usize;
    memory_end = memblock_end_of_DRAM() as usize;
    setup_initial_init_mm(&_stext, &_etext, &_edata, &_end);
    init_task.thread.kregs = &mut fake_regs;
    *cmdline_p = boot_command_line.as_mut_ptr();
    adjust_lowmem_bounds();
    find_limits(&mut min_low_pfn, &mut max_low_pfn, &mut max_pfn);
    memblock_reserve(&_stext as *const _ as usize as u64, (&_end as *const _ as usize - &_stext as *const _ as usize) as u64);
    if initrd_start != 0 { memblock_reserve(virt_to_phys(initrd_start as *const core::ffi::c_void), (initrd_end - initrd_start) as u64); }
    early_init_fdt_reserve_self();
    early_init_fdt_scan_reserved_mem();
    unflatten_and_copy_device_tree();
    setup_cpuinfo();
    copy_exception_handler(cpuinfo.exception_addr);
    mmu_init();
    copy_fast_tlb_miss_handler(cpuinfo.fast_tlb_miss_exc_addr);
    mmu_context_init();
    paging_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
