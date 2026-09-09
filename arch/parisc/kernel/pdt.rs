// SPDX-License-Identifier: GPL-2.0
/*
 *    Page Deallocation Table (PDT) support
 *
 *    The Page Deallocation Table (PDT) is maintained by firmware and holds a
 *    list of memory addresses in which memory errors were detected.
 *    The list contains both single-bit (correctable) and double-bit
 *    (uncorrectable) errors.
 *
 *    Copyright 2017 by Helge Deller <deller@gmx.de>
 *
 *    possible future enhancements:
 *    - add userspace interface via procfs or sysfs to clear PDT
 */

enum pdt_access_type {
    PDT_NONE,
    PDT_PDC,
    PDT_PAT_NEW,
    PDT_PAT_CELL,
}

static mut pdt_type: pdt_access_type = pdt_access_type::PDT_NONE;

/* PDT poll interval: 1 minute if errors, 5 minutes if everything OK. */
const PDT_POLL_INTERVAL_DEFAULT: usize = 5 * 60 * HZ;
const PDT_POLL_INTERVAL_SHORT: usize = 1 * 60 * HZ;
static mut pdt_poll_interval: usize = PDT_POLL_INTERVAL_DEFAULT;

/* global PDT status information */
static mut pdt_status: pdc_mem_retinfo = unsafe { core::mem::zeroed() };

const MAX_PDT_TABLE_SIZE: usize = PAGE_SIZE;
const MAX_PDT_ENTRIES: usize = MAX_PDT_TABLE_SIZE / core::mem::size_of::<usize>();
static mut pdt_entry: [usize; MAX_PDT_ENTRIES] = [0; MAX_PDT_ENTRIES];

/*
 * Constants for the pdt_entry format:
 * A pdt_entry holds the physical address in bits 0-57, bits 58-61 are
 * reserved, bit 62 is the perm bit and bit 63 is the error_type bit.
 * The perm bit indicates whether the error have been verified as a permanent
 * error (value of 1) or has not been verified, and may be transient (value
 * of 0). The error_type bit indicates whether the error is a single bit error
 * (value of 1) or a multiple bit error.
 * On non-PAT machines phys_addr is encoded in bits 0-59 and error_type in bit
 * 63. Those machines don't provide the perm bit.
 */

const PDT_ADDR_PERM_ERR: usize = 2;
const PDT_ADDR_SINGLE_ERR: usize = 1;

#[cfg(CONFIG_PROC_FS)]
/* report PDT entries via /proc/meminfo */
pub unsafe fn arch_report_meminfo(m: *mut seq_file) {
    if pdt_type == pdt_access_type::PDT_NONE {
        return;
    }

    seq_printf!(m, "PDT_max_entries: {:7}\n", pdt_status.pdt_size);
    seq_printf!(m, "PDT_cur_entries: {:7}\n", pdt_status.pdt_entries);
}

unsafe fn get_info_pat_new() -> i32 {
    let mut pat_rinfo: pdc_pat_mem_retinfo = core::mem::zeroed();
    let ret: i32;

    /* newer PAT machines like C8000 report info for all cells */
    if is_pdc_pat() {
        ret = pdc_pat_mem_pdt_info(&mut pat_rinfo);
    } else {
        return PDC_BAD_PROC;
    }

    pdt_status.pdt_size = pat_rinfo.max_pdt_entries;
    pdt_status.pdt_entries = pat_rinfo.current_pdt_entries;
    pdt_status.pdt_status = 0;
    pdt_status.first_dbe_loc = pat_rinfo.first_dbe_loc;
    pdt_status.good_mem = pat_rinfo.good_mem;

    ret
}

unsafe fn get_info_pat_cell() -> i32 {
    let mut cell_rinfo: pdc_pat_mem_cell_pdt_retinfo = core::mem::zeroed();
    let ret: i32;

    /* older PAT machines like rp5470 report cell info only */
    if is_pdc_pat() {
        ret = pdc_pat_mem_pdt_cell_info(&mut cell_rinfo, parisc_cell_num);
    } else {
        return PDC_BAD_PROC;
    }

    pdt_status.pdt_size = cell_rinfo.max_pdt_entries;
    pdt_status.pdt_entries = cell_rinfo.current_pdt_entries;
    pdt_status.pdt_status = 0;
    pdt_status.first_dbe_loc = cell_rinfo.first_dbe_loc;
    pdt_status.good_mem = cell_rinfo.good_mem;

    ret
}

unsafe fn report_mem_err(pde: usize) {
    let mut loc: pdc_pat_mem_phys_mem_location = core::mem::zeroed();
    let addr: usize;
    let mut dimm_txt = [0i8; 32];

    addr = pde & if pdt_type != pdt_access_type::PDT_PDC { !0x3f } else { !0x0f };

    /* show DIMM slot description on PAT machines */
    if is_pdc_pat() {
        pdc_pat_mem_get_dimm_phys_location(&mut loc, addr);
        sprintf!(dimm_txt.as_mut_ptr(), "DIMM slot {:02x}, ", loc.dimm_slot);
    } else {
        dimm_txt[0] = 0;
    }

    pr_warn!(
        "PDT: BAD MEMORY at 0x{:08lx}, {}{}{}-bit error.\n",
        addr,
        dimm_txt.as_ptr(),
        if pde & if pdt_type != pdt_access_type::PDT_PDC { 2 } else { 0 } != 0 { "permanent " } else { "" },
        if pde & PDT_ADDR_SINGLE_ERR != 0 { "single" } else { "multi" }
    );
}

/*
 * pdc_pdt_init()
 *
 * Initialize kernel PDT structures, read initial PDT table from firmware,
 * report all current PDT entries and mark bad memory with memblock_reserve()
 * to avoid that the kernel will use broken memory areas.
 *
 */
pub unsafe fn pdc_pdt_init() {
    let mut ret: i32;
    let mut i: i32;
    let mut entries: usize;
    let mut pdt_read_ret: pdc_mem_read_pdt = core::mem::zeroed();

    pdt_type = pdt_access_type::PDT_PAT_NEW;
    ret = get_info_pat_new();

    if ret != PDC_OK {
        pdt_type = pdt_access_type::PDT_PAT_CELL;
        ret = get_info_pat_cell();
    }

    if ret != PDC_OK {
        pdt_type = pdt_access_type::PDT_PDC;
        /* non-PAT machines provide the standard PDC call */
        ret = pdc_mem_pdt_info(&mut pdt_status);
    }

    if ret != PDC_OK {
        pdt_type = pdt_access_type::PDT_NONE;
        pr_info!("PDT: Firmware does not provide any page deallocation information.\n");
        return;
    }

    entries = pdt_status.pdt_entries;
    if WARN_ON!(entries > MAX_PDT_ENTRIES) {
        entries = MAX_PDT_ENTRIES;
        pdt_status.pdt_entries = MAX_PDT_ENTRIES;
    }

    pr_info!(
        "PDT: type {}, size {}, entries {}, status {}, dbe_loc 0x{:x}, good_mem {} MB\n",
        if pdt_type == pdt_access_type::PDT_PDC { "PDT_PDC" } else if pdt_type == pdt_access_type::PDT_PAT_CELL { "PDT_PAT_CELL" } else { "PDT_PAT_NEW" },
        pdt_status.pdt_size, pdt_status.pdt_entries, pdt_status.pdt_status,
        pdt_status.first_dbe_loc, pdt_status.good_mem / 1024 / 1024
    );

    if entries == 0 {
        pr_info!("PDT: Firmware reports all memory OK.\n");
        return;
    }

    if pdt_status.first_dbe_loc != 0 && pdt_status.first_dbe_loc <= __pa((&_end as *const _ as usize)) {
        pr_crit!("CRITICAL: Bad memory inside kernel image memory area!\n");
    }

    pr_warn!("PDT: Firmware reports {} entries of faulty memory:\n", entries);

    if pdt_type == pdt_access_type::PDT_PDC {
        ret = pdc_mem_pdt_read_entries(&mut pdt_read_ret, pdt_entry.as_mut_ptr());
    } else {
        #[cfg(CONFIG_64BIT)]
        {
            let mut pat_pret: pdc_pat_mem_read_pd_retinfo = core::mem::zeroed();
            if pdt_type == pdt_access_type::PDT_PAT_CELL {
                ret = pdc_pat_mem_read_cell_pdt(&mut pat_pret, pdt_entry.as_mut_ptr(), MAX_PDT_ENTRIES);
            } else {
                ret = pdc_pat_mem_read_pd_pdt(&mut pat_pret, pdt_entry.as_mut_ptr(), MAX_PDT_TABLE_SIZE, 0);
            }
        }
        #[cfg(not(CONFIG_64BIT))]
        {
            ret = PDC_BAD_PROC;
        }
    }

    if ret != PDC_OK {
        pdt_type = pdt_access_type::PDT_NONE;
        pr_warn!("PDT: Get PDT entries failed with {}\n", ret);
        return;
    }

    i = 0;
    while i < pdt_status.pdt_entries as i32 {
        let entry = pdt_entry[i as usize];
        report_mem_err(entry);

        let addr = entry & if pdt_type != pdt_access_type::PDT_PDC { !0x3f } else { !0x0f };
        if IS_ENABLED!(CONFIG_BLK_DEV_INITRD) && addr >= initrd_start && addr < initrd_end {
            pr_crit!("CRITICAL: initrd possibly broken due to bad memory!\n");
        }

        /* mark memory page bad */
        memblock_reserve(entry & PAGE_MASK, PAGE_SIZE);
        num_poisoned_pages_inc(addr >> PAGE_SHIFT);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
