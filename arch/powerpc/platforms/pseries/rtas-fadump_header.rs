/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Firmware-Assisted Dump support on POWERVM platform.
 *
 * Copyright 2011, Mahesh Salgaonkar, IBM Corporation.
 * Copyright 2019, Hari Bathini, IBM Corporation.
 */

/*
 * On some Power systems where RMO is 128MB, it still requires minimum of
 * 256MB for kernel to boot successfully. When kdump infrastructure is
 * configured to save vmcore over network, we run into OOM issue while
 * loading modules related to network setup. Hence we need additional 64M
 * of memory to avoid OOM issue.
 */
pub const RTAS_FADUMP_MIN_BOOT_MEM: u64 = ((0x1_u64 << 28) + (0x1_u64 << 26));

/* Firmware provided dump sections */
pub const RTAS_FADUMP_CPU_STATE_DATA: u16 = 0x0001;
pub const RTAS_FADUMP_HPTE_REGION: u16 = 0x0002;
pub const RTAS_FADUMP_REAL_MODE_REGION: u16 = 0x0011;

/* OS defined sections */
pub const RTAS_FADUMP_PARAM_AREA: u16 = 0x0100;

/* Dump request flag */
pub const RTAS_FADUMP_REQUEST_FLAG: u32 = 0x00000001;

/* Dump status flag */
pub const RTAS_FADUMP_ERROR_FLAG: u16 = 0x2000;

/*
 * The Firmware Assisted Dump Memory structure supports a maximum of 10 sections
 * in the dump memory structure. Presently, three sections are used for
 * CPU state data, HPTE & Parameters area, while the remaining seven sections
 * can be used for boot memory regions.
 */
pub const MAX_SECTIONS: usize = 10;
pub const RTAS_FADUMP_MAX_BOOT_MEM_REGS: usize = 7;

/*
 * Maximum time to wait for firmware to respond to an
 * ibm,configure-kernel-dump RTAS call before giving up.
 */
pub const RTAS_FADUMP_MAX_WAIT_MS: u32 = 60000;

/* Kernel Dump section info */
#[repr(C)]
pub struct rtas_fadump_section {
    pub request_flag: __be32,
    pub source_data_type: __be16,
    pub error_flags: __be16,
    pub source_address: __be64,
    pub source_len: __be64,
    pub bytes_dumped: __be64,
    pub destination_address: __be64,
}

/* ibm,configure-kernel-dump header. */
#[repr(C)]
pub struct rtas_fadump_section_header {
    pub dump_format_version: __be32,
    pub dump_num_sections: __be16,
    pub dump_status_flag: __be16,
    pub offset_first_dump_section: __be32,

    /* Fields for disk dump option. */
    pub dd_block_size: __be32,
    pub dd_block_offset: __be64,
    pub dd_num_blocks: __be64,
    pub dd_offset_disk_path: __be32,

    /* Maximum time allowed to prevent an automatic dump-reboot. */
    pub max_time_auto: __be32,
}

/*
 * Firmware Assisted dump memory structure. This structure is required for
 * registering future kernel dump with power firmware through rtas call.
 *
 * In version 1, the platform permits one section header, dump-disk path
 * and ten sections.
 *
 * Note: No disk dump option. Hence disk dump path string section is not
 * included.
 */
#[repr(C)]
pub struct rtas_fadump_mem_struct {
    pub header: rtas_fadump_section_header,
    pub rgn: [rtas_fadump_section; MAX_SECTIONS],
}

/*
 * The firmware-assisted dump format.
 *
 * The register save area is an area in the partition's memory used to preserve
 * the register contents (CPU state data) for the active CPUs during a firmware
 * assisted dump. The dump format contains register save area header followed
 * by register entries. Each list of registers for a CPU starts with "CPUSTRT"
 * and ends with "CPUEND".
 */

/* Register save area header. */
#[repr(C)]
pub struct rtas_fadump_reg_save_area_header {
    pub magic_number: __be64,
    pub version: __be32,
    pub num_cpu_offset: __be32,
}

/* Register entry. */
#[repr(C)]
pub struct rtas_fadump_reg_entry {
    pub reg_id: __be64,
    pub reg_value: __be64,
}

/* Utility macro translated as an unsafe pointer-advancing function. */
pub unsafe fn RTAS_FADUMP_SKIP_TO_NEXT_CPU(mut reg_entry: *mut rtas_fadump_reg_entry) -> *mut rtas_fadump_reg_entry {
    while be64_to_cpu((*reg_entry).reg_id) != fadump_str_to_u64("CPUEND") {
        reg_entry = reg_entry.add(1);
    }
    reg_entry.add(1)
}

pub const RTAS_FADUMP_CPU_ID_MASK: u64 = (1_u64 << 32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
