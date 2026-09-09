/* SPDX-License-Identifier: GPL-2.0 */

// #define INIT_HWRPB ((struct hwrpb_struct *) 0x10000000)
pub const INIT_HWRPB: *mut hwrpb_struct = 0x10000000 as *mut hwrpb_struct;

/*
 * DEC processor types for Alpha systems.  Found in HWRPB.
 * These values are architected.
 */
pub const EV3_CPU: i32 = 1;
pub const EV4_CPU: i32 = 2;
pub const LCA4_CPU: i32 = 4;
pub const EV5_CPU: i32 = 5;
pub const EV45_CPU: i32 = 6;
pub const EV56_CPU: i32 = 7;
pub const EV6_CPU: i32 = 8;
pub const PCA56_CPU: i32 = 9;
pub const PCA57_CPU: i32 = 10;
pub const EV67_CPU: i32 = 11;
pub const EV68CB_CPU: i32 = 12;
pub const EV68AL_CPU: i32 = 13;
pub const EV68CX_CPU: i32 = 14;
pub const EV7_CPU: i32 = 15;
pub const EV79_CPU: i32 = 16;
pub const EV69_CPU: i32 = 17;

/*
 * DEC system types for Alpha systems.  Found in HWRPB.
 * These values are architected.
 */
pub const ST_ADU: i32 = 1;
pub const ST_DEC_4000: i32 = 2;
pub const ST_DEC_7000: i32 = 3;
pub const ST_DEC_3000_500: i32 = 4;
pub const ST_DEC_2000_300: i32 = 6;
pub const ST_DEC_3000_300: i32 = 7;
pub const ST_DEC_2100_A500: i32 = 9;
pub const ST_DEC_AXPVME_64: i32 = 10;
pub const ST_DEC_AXPPCI_33: i32 = 11;
pub const ST_DEC_TLASER: i32 = 12;
pub const ST_DEC_2100_A50: i32 = 13;
pub const ST_DEC_MUSTANG: i32 = 14;
pub const ST_DEC_ALCOR: i32 = 15;
pub const ST_DEC_1000: i32 = 17;
pub const ST_DEC_EB64: i32 = 18;
pub const ST_DEC_EB66: i32 = 19;
pub const ST_DEC_EB64P: i32 = 20;
pub const ST_DEC_BURNS: i32 = 21;
pub const ST_DEC_RAWHIDE: i32 = 22;
pub const ST_DEC_K2: i32 = 23;
pub const ST_DEC_LYNX: i32 = 24;
pub const ST_DEC_XL: i32 = 25;
pub const ST_DEC_EB164: i32 = 26;
pub const ST_DEC_NORITAKE: i32 = 27;
pub const ST_DEC_CORTEX: i32 = 28;
pub const ST_DEC_MIATA: i32 = 30;
pub const ST_DEC_XXM: i32 = 31;
pub const ST_DEC_TAKARA: i32 = 32;
pub const ST_DEC_YUKON: i32 = 33;
pub const ST_DEC_TSUNAMI: i32 = 34;
pub const ST_DEC_WILDFIRE: i32 = 35;
pub const ST_DEC_CUSCO: i32 = 36;
pub const ST_DEC_EIGER: i32 = 37;
pub const ST_DEC_TITAN: i32 = 38;
pub const ST_DEC_MARVEL: i32 = 39;

/* UNOFFICIAL!!! */
pub const ST_UNOFFICIAL_BIAS: i32 = 100;
pub const ST_DTI_RUFFIAN: i32 = 101;

/* Alpha Processor, Inc. systems */
pub const ST_API_BIAS: i32 = 200;
pub const ST_API_NAUTILUS: i32 = 201;

#[repr(C)]
pub struct pcb_struct { pub ksp: usize, pub usp: usize, pub ptbr: usize, pub pcc: u32, pub asn: u32, pub unique: usize, pub flags: usize, pub res1: usize, pub res2: usize }

#[repr(C)]
pub struct percpu_struct {
    pub hwpcb: [usize; 16], pub flags: usize, pub pal_mem_size: usize, pub pal_scratch_size: usize,
    pub pal_mem_pa: usize, pub pal_scratch_pa: usize, pub pal_revision: usize, pub r#type: usize,
    pub variation: usize, pub revision: usize, pub serial_no: [usize; 2], pub logout_area_pa: usize,
    pub logout_area_len: usize, pub halt_PCBB: usize, pub halt_PC: usize, pub halt_PS: usize,
    pub halt_arg: usize, pub halt_ra: usize, pub halt_pv: usize, pub halt_reason: usize, pub res: usize,
    pub ipc_buffer: [usize; 21], pub palcode_avail: [usize; 16], pub compatibility: usize,
    pub console_data_log_pa: usize, pub console_data_log_length: usize, pub bcache_info: usize,
}

#[repr(C)] pub struct procdesc_struct { pub weird_vms_stuff: usize, pub address: usize }
#[repr(C)] pub struct vf_map_struct { pub va: usize, pub pa: usize, pub count: usize }
#[repr(C)] pub struct crb_struct { pub dispatch_va: *mut procdesc_struct, pub dispatch_pa: *mut procdesc_struct, pub fixup_va: *mut procdesc_struct, pub fixup_pa: *mut procdesc_struct, pub map_entries: usize, pub map_pages: usize, pub map: [vf_map_struct; 0] }
#[repr(C)] pub struct memclust_struct { pub start_pfn: usize, pub numpages: usize, pub numtested: usize, pub bitmap_va: usize, pub bitmap_pa: usize, pub bitmap_chksum: usize, pub usage: usize }
#[repr(C)] pub struct memdesc_struct { pub chksum: usize, pub optional_pa: usize, pub numclusters: usize, pub cluster: [memclust_struct; 0] }
#[repr(C)] pub struct dsr_struct { pub smm: isize, pub lurt_off: usize, pub sysname_off: usize }

#[repr(C)]
pub struct hwrpb_struct {
    pub phys_addr: usize, pub id: usize, pub revision: usize, pub size: usize, pub cpuid: usize,
    pub pagesize: usize, pub pa_bits: usize, pub max_asn: usize, pub ssn: [u8; 16], pub sys_type: usize,
    pub sys_variation: usize, pub sys_revision: usize, pub intr_freq: usize, pub cycle_freq: usize,
    pub vptb: usize, pub res1: usize, pub tbhb_offset: usize, pub nr_processors: usize,
    pub processor_size: usize, pub processor_offset: usize, pub ctb_nr: usize, pub ctb_size: usize,
    pub ctbt_offset: usize, pub crb_offset: usize, pub mddt_offset: usize, pub cdb_offset: usize,
    pub frut_offset: usize, pub save_terminal: Option<unsafe extern "C" fn(usize)>,
    pub save_terminal_data: usize, pub restore_terminal: Option<unsafe extern "C" fn(usize)>,
    pub restore_terminal_data: usize, pub CPU_restart: Option<unsafe extern "C" fn(usize)>,
    pub CPU_restart_data: usize, pub res2: usize, pub res3: usize, pub chksum: usize,
    pub rxrdy: usize, pub txrdy: usize, pub dsr_offset: usize,
}

// __KERNEL__ declarations and definitions
pub unsafe extern "C" { pub static mut hwrpb: *mut hwrpb_struct; }

#[inline]
pub unsafe fn hwrpb_update_checksum(h: *mut hwrpb_struct) {
    let mut sum: usize = 0;
    let mut l = h as *mut usize;
    let end = unsafe { core::ptr::addr_of_mut!((*h).chksum) };
    while l < end {
        sum = sum.wrapping_add(unsafe { *l });
        l = unsafe { l.add(1) };
    }
    unsafe { (*h).chksum = sum; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
