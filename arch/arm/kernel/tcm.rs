// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008-2009 ST-Ericsson AB
 * TCM memory handling for ARM systems
 *
 * Author: Linus Walleij <linus.walleij@stericsson.com>
 * Author: Rickard Andersson <rickard.andersson@stericsson.com>
 */
// Linux and ARM header dependencies are supplied by the surrounding tree.

const TCMTR_FORMAT_MASK: u32 = 0xe0000000;

static mut tcm_pool: *mut gen_pool = core::ptr::null_mut();
static mut dtcm_present: bool = false;
static mut itcm_present: bool = false;

extern "C" {
    static mut __itcm_start: core::ffi::c_char;
    static mut __sitcm_text: core::ffi::c_char;
    static mut __eitcm_text: core::ffi::c_char;
    static mut __dtcm_start: core::ffi::c_char;
    static mut __sdtcm_data: core::ffi::c_char;
    static mut __edtcm_data: core::ffi::c_char;
}

static mut dtcm_end: u32 = DTCM_OFFSET;
static mut itcm_end: u32 = ITCM_OFFSET;

static mut dtcm_res: resource = resource {
    name: b"DTCM RAM\0".as_ptr() as *const i8,
    start: DTCM_OFFSET,
    end: DTCM_OFFSET,
    flags: IORESOURCE_MEM,
};

static mut itcm_res: resource = resource {
    name: b"ITCM RAM\0".as_ptr() as *const i8,
    start: ITCM_OFFSET,
    end: ITCM_OFFSET,
    flags: IORESOURCE_MEM,
};

static mut dtcm_iomap: [map_desc; 1] = [map_desc {
    virtual_: DTCM_OFFSET,
    pfn: __phys_to_pfn(DTCM_OFFSET),
    length: 0,
    type_: MT_MEMORY_RW_DTCM,
}];

static mut itcm_iomap: [map_desc; 1] = [map_desc {
    virtual_: ITCM_OFFSET,
    pfn: __phys_to_pfn(ITCM_OFFSET),
    length: 0,
    type_: MT_MEMORY_RWX_ITCM,
}];

#[no_mangle]
pub unsafe extern "C" fn tcm_alloc(len: usize) -> *mut core::ffi::c_void {
    if tcm_pool.is_null() { return core::ptr::null_mut(); }
    let vaddr = gen_pool_alloc(tcm_pool, len);
    if vaddr == 0 { return core::ptr::null_mut(); }
    vaddr as *mut core::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn tcm_free(addr: *mut core::ffi::c_void, len: usize) {
    gen_pool_free(tcm_pool, addr as usize as u64, len);
}

#[no_mangle]
pub unsafe extern "C" fn tcm_dtcm_present() -> bool { dtcm_present }

#[no_mangle]
pub unsafe extern "C" fn tcm_itcm_present() -> bool { itcm_present }

unsafe fn setup_tcm_bank(type_: u8, bank: u8, banks: u8, offset: *mut u32) -> i32 {
    let tcm_sizes: [i32; 16] = [0, -1, -1, 4, 8, 16, 32, 64, 128, 256, 512, 1024, -1, -1, -1, -1];
    let mut tcm_region: u32;
    let tcm_size: i32;

    if banks > 1 {
        // Original ARM coprocessor instruction: mcr p15, 0, bank, c9, c2, 0
        core::arch::asm!("mcr p15, 0, {0}, c9, c2, 0", in(reg) bank);
    }
    if type_ == 0 {
        core::arch::asm!("mrc p15, 0, {0}, c9, c1, 0", out(reg) tcm_region);
    } else {
        core::arch::asm!("mrc p15, 0, {0}, c9, c1, 1", out(reg) tcm_region);
    }
    tcm_size = tcm_sizes[((tcm_region >> 2) & 0x0f) as usize];
    if tcm_size < 0 {
        pr_err!("CPU: {}TCM{} of unknown size\n", if type_ != 0 { "I" } else { "D" }, bank);
        return -EINVAL;
    } else if tcm_size > 32 {
        pr_err!("CPU: {}TCM{} larger than 32k found\n", if type_ != 0 { "I" } else { "D" }, bank);
        return -EINVAL;
    } else {
        pr_info!("CPU: found {}TCM{} {}k @ {:08x}, {}enabled\n", if type_ != 0 { "I" } else { "D" }, bank, tcm_size, tcm_region & 0xfffff000, if tcm_region & 1 != 0 { "" } else { "not " });
    }
    if tcm_size == 0 { return 0; }
    tcm_region = *offset | (tcm_region & 0x00000ffe) | 1;
    if type_ == 0 {
        core::arch::asm!("mcr p15, 0, {0}, c9, c1, 0", in(reg) tcm_region);
    } else {
        core::arch::asm!("mcr p15, 0, {0}, c9, c1, 1", in(reg) tcm_region);
    }
    *offset += (tcm_size << 10) as u32;
    pr_info!("CPU: moved {}TCM{} {}k to {:08x}, enabled\n", if type_ != 0 { "I" } else { "D" }, bank, tcm_size, tcm_region & 0xfffff000);
    0
}

const TCM_REGION_READ_MASK: u32 = 0xffff0fdf;
const TCM_REGION_READ_INSTR: u32 = 0xee190f11;
const DEST_REG_SHIFT: u32 = 12;
const DEST_REG_MASK: u32 = 0xf;

unsafe fn tcm_handler(regs: *mut pt_regs, instr: u32) -> i32 {
    (*regs).uregs[((instr >> DEST_REG_SHIFT) & DEST_REG_MASK) as usize] = 0;
    (*regs).ARM_pc += 4;
    0
}

static mut tcm_hook: undef_hook = undef_hook {
    instr_mask: TCM_REGION_READ_MASK,
    instr_val: TCM_REGION_READ_INSTR,
    cpsr_mask: MODE_MASK,
    cpsr_val: SVC_MODE,
    fn_: Some(tcm_handler),
};

pub unsafe extern "C" fn tcm_init() {
    let dtcm_code_sz = (&__edtcm_data as *const _ as usize).wrapping_sub(&__sdtcm_data as *const _ as usize);
    let itcm_code_sz = (&__eitcm_text as *const _ as usize).wrapping_sub(&__sitcm_text as *const _ as usize);
    if cpu_architecture() < CPU_ARCH_ARMv5 {
        if dtcm_code_sz != 0 || itcm_code_sz != 0 { pr_info!("CPU TCM: {} bytes of DTCM and {} bytes of ITCM code compiled in, but no TCM present in pre-v5 CPU\n", dtcm_code_sz, itcm_code_sz); }
        return;
    }
    let tcm_status = read_cpuid_tcmstatus();
    if tcm_status & TCMTR_FORMAT_MASK != 0 { return; }
    let mut dtcm_banks = ((tcm_status >> 16) & 0x03) as u8;
    let mut itcm_banks = (tcm_status & 0x03) as u8;
    register_undef_hook(&mut tcm_hook);
    if dtcm_banks > 2 { dtcm_banks = 0; }
    if itcm_banks > 2 { itcm_banks = 0; }
    if dtcm_banks > 0 {
        for i in 0..dtcm_banks { if setup_tcm_bank(0, i, dtcm_banks, &mut dtcm_end) != 0 { unregister_undef_hook(&mut tcm_hook); return; } }
        if dtcm_code_sz > (dtcm_end - DTCM_OFFSET) as usize || dtcm_end - DTCM_OFFSET == 0 { if dtcm_code_sz > (dtcm_end - DTCM_OFFSET) as usize { pr_info!("CPU DTCM: {} bytes of code compiled to DTCM but only {} bytes of DTCM present\n", dtcm_code_sz, dtcm_end - DTCM_OFFSET); } } else {
            dtcm_res.end = dtcm_end - 1; request_resource(&mut iomem_resource, &mut dtcm_res); dtcm_iomap[0].length = dtcm_end - DTCM_OFFSET; iotable_init(dtcm_iomap.as_mut_ptr(), 1); core::ptr::copy_nonoverlapping(&__dtcm_start, &mut __sdtcm_data, dtcm_code_sz); dtcm_present = true;
        }
    } else if dtcm_code_sz != 0 { pr_info!("CPU DTCM: {} bytes of code compiled to DTCM but no DTCM banks present in CPU\n", dtcm_code_sz); }
    if itcm_banks > 0 {
        for i in 0..itcm_banks { if setup_tcm_bank(1, i, itcm_banks, &mut itcm_end) != 0 { unregister_undef_hook(&mut tcm_hook); return; } }
        if itcm_code_sz > (itcm_end - ITCM_OFFSET) as usize || itcm_end - ITCM_OFFSET == 0 { if itcm_code_sz > (itcm_end - ITCM_OFFSET) as usize { pr_info!("CPU ITCM: {} bytes of code compiled to ITCM but only {} bytes of ITCM present\n", itcm_code_sz, itcm_end - ITCM_OFFSET); } } else {
            itcm_res.end = itcm_end - 1; request_resource(&mut iomem_resource, &mut itcm_res); itcm_iomap[0].length = itcm_end - ITCM_OFFSET; iotable_init(itcm_iomap.as_mut_ptr(), 1); core::ptr::copy_nonoverlapping(&__itcm_start, &mut __sitcm_text, itcm_code_sz); itcm_present = true;
        }
    } else if itcm_code_sz != 0 { pr_info!("CPU ITCM: {} bytes of code compiled to ITCM but no ITCM banks present in CPU\n", itcm_code_sz); }
    unregister_undef_hook(&mut tcm_hook);
}

unsafe fn setup_tcm_pool() -> i32 {
    let dtcm_pool_start = &__edtcm_data as *const _ as u32;
    let itcm_pool_start = &__eitcm_text as *const _ as u32;
    tcm_pool = gen_pool_create(2, -1);
    pr_debug!("Setting up TCM memory pool\n");
    if dtcm_present && dtcm_pool_start < dtcm_end { let ret = gen_pool_add(tcm_pool, dtcm_pool_start as u64, (dtcm_end - dtcm_pool_start) as usize, -1); if ret != 0 { pr_err!("CPU DTCM: could not add DTCM remainder to pool!\n"); return ret; } }
    if itcm_present && itcm_pool_start < itcm_end { let ret = gen_pool_add(tcm_pool, itcm_pool_start as u64, (itcm_end - itcm_pool_start) as usize, -1); if ret != 0 { pr_err!("CPU ITCM: could not add ITCM remainder to pool!\n"); return ret; } }
    0
}

core_initcall!(setup_tcm_pool);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
