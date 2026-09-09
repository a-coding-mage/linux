// SPDX-License-Identifier: GPL-2.0-only
/*
 * tboot.c: main implementation of helper functions used by kernel for
 *          runtime support of Intel(R) Trusted Execution Technology
 *
 * Copyright (c) 2006-2009, Intel Corporation
 */

// C header dependencies are supplied by the surrounding kernel translation unit.

static mut TBOOT: *mut tboot = core::ptr::null_mut();
const AP_WAIT_TIMEOUT: u64 = 1;
static mut TBOOT_UUID: [u8; 16] = TBOOT_UUID_VALUE;

pub unsafe fn tboot_enabled() -> bool { !TBOOT.is_null() }

unsafe fn check_tboot_version() -> bool {
    if core::slice::from_raw_parts(TBOOT_UUID.as_ptr(), 16)
        != core::slice::from_raw_parts((*TBOOT).uuid.as_ptr(), 16) {
        pr_warn!("tboot at 0x%llx is invalid\n", boot_params.tboot_addr);
        return false;
    }
    if (*TBOOT).version < 5 {
        pr_warn!("tboot version is invalid: %u\n", (*TBOOT).version);
        return false;
    }
    pr_info!("found shared page at phys addr 0x%llx:\n", boot_params.tboot_addr);
    pr_debug!("version: %d\n", (*TBOOT).version);
    pr_debug!("log_addr: 0x%08x\n", (*TBOOT).log_addr);
    pr_debug!("shutdown_entry: 0x%x\n", (*TBOOT).shutdown_entry);
    pr_debug!("tboot_base: 0x%08x\n", (*TBOOT).tboot_base);
    pr_debug!("tboot_size: 0x%x\n", (*TBOOT).tboot_size);
    true
}

pub unsafe fn tboot_probe() {
    if boot_params.tboot_addr == 0 { return; }
    if !e820__mapped_any(boot_params.tboot_addr, boot_params.tboot_addr, E820_TYPE_RESERVED) {
        pr_warn!("non-0 tboot_addr but it is not of type E820_TYPE_RESERVED\n");
        return;
    }
    set_fixmap(FIX_TBOOT_BASE, boot_params.tboot_addr);
    TBOOT = fix_to_virt(FIX_TBOOT_BASE) as *mut tboot;
    if !check_tboot_version() { TBOOT = core::ptr::null_mut(); }
}

static mut TBOOT_PG_DIR: *mut pgd_t = core::ptr::null_mut();
static mut TBOOT_MM: mm_struct = mm_struct { /* initialized from init_mm in C */ };

unsafe fn switch_to_tboot_pt() { write_cr3(virt_to_phys(TBOOT_PG_DIR)); }

unsafe fn map_tboot_page(mut vaddr: usize, pfn: usize, prot: pgprot_t) -> i32 {
    let pgd = pgd_offset(&mut TBOOT_MM, vaddr);
    let p4d = p4d_alloc(&mut TBOOT_MM, pgd, vaddr);
    if p4d.is_null() { return -1; }
    let pud = pud_alloc(&mut TBOOT_MM, p4d, vaddr);
    if pud.is_null() { return -1; }
    let pmd = pmd_alloc(&mut TBOOT_MM, pud, vaddr);
    if pmd.is_null() { return -1; }
    let pte = pte_alloc_map(&mut TBOOT_MM, pmd, vaddr);
    if pte.is_null() { return -1; }
    set_pte_at(&mut TBOOT_MM, vaddr, pte, pfn_pte(pfn, prot));
    pte_unmap(pte);
    (*pgd).pgd &= !_PAGE_NX;
    0
}

unsafe fn map_tboot_pages(mut vaddr: usize, mut start_pfn: usize, mut nr: usize) -> i32 {
    TBOOT_PG_DIR = pgd_alloc(&mut TBOOT_MM);
    if TBOOT_PG_DIR.is_null() { return -1; }
    while nr > 0 {
        if map_tboot_page(vaddr, start_pfn, PAGE_KERNEL_EXEC) != 0 { return -1; }
        nr -= 1; vaddr += PAGE_SIZE; start_pfn += 1;
    }
    0
}

unsafe fn tboot_create_trampoline() {
    let map_base = PFN_DOWN((*TBOOT).tboot_base);
    let map_size = PFN_UP((*TBOOT).tboot_size);
    if map_tboot_pages(map_base << PAGE_SHIFT, map_base, map_size) != 0 {
        panic!("tboot: Error mapping tboot pages (mfns) @ 0x%x, 0x%x\n", map_base, map_size);
    }
}

unsafe fn add_mac_region(start: phys_addr_t, size: usize) {
    if (*TBOOT).num_mac_regions >= MAX_TB_MAC_REGIONS { panic!("tboot: Too many MAC regions\n"); }
    if start != 0 && size != 0 {
        let n = (*TBOOT).num_mac_regions as usize;
        (*TBOOT).num_mac_regions += 1;
        (*TBOOT).mac_regions[n].start = round_down(start, PAGE_SIZE);
        (*TBOOT).mac_regions[n].size = round_up(start + size, PAGE_SIZE) - (*TBOOT).mac_regions[n].start;
    }
}

unsafe fn tboot_setup_sleep() -> i32 {
    (*TBOOT).num_mac_regions = 0;
    for i in 0..e820_table.nr_entries {
        if e820_table.entries[i].type != E820_TYPE_RAM { continue; }
        add_mac_region(e820_table.entries[i].addr, e820_table.entries[i].size);
    }
    (*TBOOT).acpi_sinfo.kernel_s3_resume_vector = real_mode_header.wakeup_start;
    0
}

pub unsafe fn tboot_shutdown(shutdown_type: u32) {
    if !tboot_enabled() || TBOOT_PG_DIR.is_null() { return; }
    if shutdown_type == TB_SHUTDOWN_S3 && tboot_setup_sleep() != 0 { return; }
    (*TBOOT).shutdown_type = shutdown_type;
    switch_to_tboot_pt();
    let shutdown: extern "C" fn() = core::mem::transmute((*TBOOT).shutdown_entry as usize);
    shutdown();
    loop { halt(); }
}

unsafe fn tboot_copy_fadt(fadt: *const acpi_table_fadt) {
    macro_rules! copy_gas { ($t:expr, $g:expr) => {{
        $t.space_id = $g.space_id; $t.bit_width = $g.bit_width;
        $t.bit_offset = $g.bit_offset; $t.access_width = $g.access_width;
        $t.address = $g.address;
    }}; }
    copy_gas!((*TBOOT).acpi_sinfo.pm1a_cnt_blk, (*fadt).xpm1a_control_block);
    copy_gas!((*TBOOT).acpi_sinfo.pm1b_cnt_blk, (*fadt).xpm1b_control_block);
    copy_gas!((*TBOOT).acpi_sinfo.pm1a_evt_blk, (*fadt).xpm1a_event_block);
    copy_gas!((*TBOOT).acpi_sinfo.pm1b_evt_blk, (*fadt).xpm1b_event_block);
    (*TBOOT).acpi_sinfo.wakeup_vector = (*fadt).facs + core::mem::offset_of!(acpi_table_facs, firmware_waking_vector) as u64;
}

unsafe fn tboot_sleep(sleep_state: u8, pm1a_control: u32, pm1b_control: u32) -> i32 {
    const MAP: [i32; ACPI_S_STATE_COUNT] = [-1, -1, -1, TB_SHUTDOWN_S3 as i32, TB_SHUTDOWN_S4 as i32, TB_SHUTDOWN_S5 as i32];
    if !tboot_enabled() { return 0; }
    tboot_copy_fadt(&acpi_gbl_FADT);
    (*TBOOT).acpi_sinfo.pm1a_cnt_val = pm1a_control;
    (*TBOOT).acpi_sinfo.pm1b_cnt_val = pm1b_control;
    (*TBOOT).acpi_sinfo.vector_width = 32;
    if sleep_state as usize >= ACPI_S_STATE_COUNT || MAP[sleep_state as usize] == -1 {
        pr_warn!("unsupported sleep state 0x%x\n", sleep_state); return -1;
    }
    tboot_shutdown(MAP[sleep_state as usize] as u32); 0
}

unsafe fn tboot_extended_sleep(_sleep_state: u8, _val_a: u32, _val_b: u32) -> i32 {
    if !tboot_enabled() { return 0; }
    pr_warn!("tboot is not able to suspend on platforms with reduced hardware sleep (ACPIv5)");
    -ENODEV
}

static mut AP_WFS_COUNT: atomic_t = atomic_t::new(0);
unsafe fn tboot_wait_for_aps(num_aps: i32) -> i32 {
    let mut timeout = AP_WAIT_TIMEOUT * HZ;
    while atomic_read(&(*TBOOT).num_in_wfs) != num_aps && timeout != 0 { mdelay(1); timeout -= 1; }
    if timeout != 0 { pr_warn!("tboot wait for APs timeout\n"); }
    (atomic_read(&(*TBOOT).num_in_wfs) != num_aps) as i32
}
unsafe fn tboot_dying_cpu(_cpu: u32) -> i32 {
    atomic_inc(&mut AP_WFS_COUNT);
    if num_online_cpus() == 1 && tboot_wait_for_aps(atomic_read(&AP_WFS_COUNT)) != 0 { return -EBUSY; }
    0
}

const TXT_PUB_CONFIG_REGS_BASE: usize = 0xfed30000;
const TXT_PRIV_CONFIG_REGS_BASE: usize = 0xfed20000;
const NR_TXT_CONFIG_PAGES: usize = (TXT_PUB_CONFIG_REGS_BASE - TXT_PRIV_CONFIG_REGS_BASE) >> PAGE_SHIFT;
const TXTCR_HEAP_BASE: usize = 0x0300;
const TXTCR_HEAP_SIZE: usize = 0x0308;
const SHA1_SIZE: usize = 20;

#[repr(C)] pub struct sha1_hash { pub hash: [u8; SHA1_SIZE] }
#[repr(C, packed)] pub struct sinit_mle_data {
    pub version: u32, pub bios_acm_id: sha1_hash, pub edx_senter_flags: u32,
    pub mseg_valid: u64, pub sinit_hash: sha1_hash, pub mle_hash: sha1_hash,
    pub stm_hash: sha1_hash, pub lcp_policy_hash: sha1_hash, pub lcp_policy_control: u32,
    pub rlp_wakeup_addr: u32, pub reserved: u32, pub num_mdrs: u32, pub mdrs_off: u32,
    pub num_vtd_dmars: u32, pub vtd_dmars_off: u32,
}

pub unsafe fn tboot_get_dmar_table(mut dmar_tbl: *mut acpi_table_header) -> *mut acpi_table_header {
    if !tboot_enabled() { return dmar_tbl; }
    let config = ioremap(TXT_PUB_CONFIG_REGS_BASE, NR_TXT_CONFIG_PAGES * PAGE_SIZE);
    if config.is_null() { return core::ptr::null_mut(); }
    let heap_base = ioremap(*(config.add(TXTCR_HEAP_BASE) as *const u64), *(config.add(TXTCR_HEAP_SIZE) as *const u64));
    iounmap(config);
    if heap_base.is_null() { return core::ptr::null_mut(); }
    let mut heap_ptr = heap_base.add(*(heap_base as *const u64) as usize);
    heap_ptr = heap_ptr.add(*(heap_ptr as *const u64) as usize);
    heap_ptr = heap_ptr.add(*(heap_ptr as *const u64) as usize);
    heap_ptr = heap_ptr.add(core::mem::size_of::<u64>());
    dmar_tbl = heap_ptr.add((*(heap_ptr.add(core::mem::offset_of!(sinit_mle_data, vtd_dmars_off)) as *const u32) as usize) - core::mem::size_of::<u64>()) as *mut acpi_table_header;
    dmar_tbl
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
