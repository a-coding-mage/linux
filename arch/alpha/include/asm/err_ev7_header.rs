/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Data for el packet class PAL (14), type LOGOUT_FRAME (1)
 */
#[repr(C)]
pub struct ev7_pal_logout_subpacket {
    pub mchk_code: u32,
    pub subpacket_count: u32,
    pub whami: u64,
    pub rbox_whami: u64,
    pub rbox_int: u64,
    pub exc_addr: u64,
    pub timestamp: el_timestamp,
    pub halt_code: u64,
    pub reserved: u64,
}

/*
 * Data for el packet class PAL (14), type EV7_PROCESSOR (4)
 */
#[repr(C)]
pub struct ev7_pal_processor_subpacket {
    pub i_stat: u64,
    pub dc_stat: u64,
    pub c_addr: u64,
    pub c_syndrome_1: u64,
    pub c_syndrome_0: u64,
    pub c_stat: u64,
    pub c_sts: u64,
    pub mm_stat: u64,
    pub exc_addr: u64,
    pub ier_cm: u64,
    pub isum: u64,
    pub pal_base: u64,
    pub i_ctl: u64,
    pub process_context: u64,
    pub cbox_ctl: u64,
    pub cbox_stp_ctl: u64,
    pub cbox_acc_ctl: u64,
    pub cbox_lcl_set: u64,
    pub cbox_gbl_set: u64,
    pub bbox_ctl: u64,
    pub bbox_err_sts: u64,
    pub bbox_err_idx: u64,
    pub cbox_ddp_err_sts: u64,
    pub bbox_dat_rmp: u64,
    pub reserved: [u64; 2],
}

/* Data for el packet class PAL (14), type EV7_ZBOX (5) */
#[repr(C)]
pub struct ev7_pal_zbox_subpacket {
    pub zbox0_dram_err_status_1: u32,
    pub zbox0_dram_err_status_2: u32,
    pub zbox0_dram_err_status_3: u32,
    pub zbox0_dram_err_ctl: u32,
    pub zbox0_dram_err_adr: u32,
    pub zbox0_dift_timeout: u32,
    pub zbox0_dram_mapper_ctl: u32,
    pub zbox0_frc_err_adr: u32,
    pub zbox0_dift_err_status: u32,
    pub reserved1: u32,
    pub zbox1_dram_err_status_1: u32,
    pub zbox1_dram_err_status_2: u32,
    pub zbox1_dram_err_status_3: u32,
    pub zbox1_dram_err_ctl: u32,
    pub zbox1_dram_err_adr: u32,
    pub zbox1_dift_timeout: u32,
    pub zbox1_dram_mapper_ctl: u32,
    pub zbox1_frc_err_adr: u32,
    pub zbox1_dift_err_status: u32,
    pub reserved2: u32,
    pub cbox_ctl: u64,
    pub cbox_stp_ctl: u64,
    pub zbox0_error_pa: u64,
    pub zbox1_error_pa: u64,
    pub zbox0_ored_syndrome: u64,
    pub zbox1_ored_syndrome: u64,
    pub reserved3: [u64; 2],
}

/* Data for el packet class PAL (14), type EV7_RBOX (6) */
#[repr(C)]
pub struct ev7_pal_rbox_subpacket {
    pub rbox_cfg: u64, pub rbox_n_cfg: u64, pub rbox_s_cfg: u64,
    pub rbox_e_cfg: u64, pub rbox_w_cfg: u64, pub rbox_n_err: u64,
    pub rbox_s_err: u64, pub rbox_e_err: u64, pub rbox_w_err: u64,
    pub rbox_io_cfg: u64, pub rbox_io_err: u64, pub rbox_l_err: u64,
    pub rbox_whoami: u64, pub rbox_imask: u64, pub rbox_intq: u64,
    pub rbox_int: u64, pub reserved: [u64; 2],
}

/* Data for el packet class PAL (14), type EV7_IO (7) */
#[repr(C)]
pub struct ev7_pal_io_one_port {
    pub pox_err_sum: u64, pub pox_tlb_err: u64, pub pox_spl_cmplt: u64,
    pub pox_trans_sum: u64, pub pox_first_err: u64, pub pox_mult_err: u64,
    pub pox_dm_source: u64, pub pox_dm_dest: u64, pub pox_dm_size: u64,
    pub pox_dm_ctrl: u64, pub reserved: u64,
}

#[repr(C)]
pub struct ev7_pal_io_subpacket {
    pub io_asic_rev: u64, pub io_sys_rev: u64, pub io7_uph: u64,
    pub hpi_ctl: u64, pub crd_ctl: u64, pub hei_ctl: u64,
    pub po7_error_sum: u64, pub po7_uncrr_sym: u64, pub po7_crrct_sym: u64,
    pub po7_ugbge_sym: u64, pub po7_err_pkt0: u64, pub po7_err_pkt1: u64,
    pub reserved: [u64; 2],
    pub ports: [ev7_pal_io_one_port; 4],
}

/* Environmental subpacket. Data used for el packets:
 * class PAL (14), type AMBIENT_TEMPERATURE (10)
 * class PAL (14), type AIRMOVER_FAN (11)
 * class PAL (14), type VOLTAGE (12)
 * class PAL (14), type INTRUSION (13)
 * class PAL (14), type POWER_SUPPLY (14)
 * class PAL (14), type LAN (15)
 * class PAL (14), type HOT_PLUG (16)
 */
#[repr(C)]
pub struct ev7_pal_environmental_subpacket {
    pub cabinet: u16, pub drawer: u16, pub reserved1: [u16; 2],
    pub module_type: u8,
    pub unit_id: u8, /* unit reporting condition */
    pub reserved2: u8,
    pub condition: u8, /* condition reported */
}

/* Convert environmental type to index */
#[inline]
pub unsafe fn ev7_lf_env_index(type_: i32) -> i32 {
    BUG_ON!((type_ < EL_TYPE__PAL__ENV__AMBIENT_TEMPERATURE)
        || (type_ > EL_TYPE__PAL__ENV__HOT_PLUG));
    type_ - EL_TYPE__PAL__ENV__AMBIENT_TEMPERATURE
}

/* Data for generic el packet class PAL. */
#[repr(C)]
pub union ev7_pal_subpacket_by_type {
    pub logout: core::mem::ManuallyDrop<ev7_pal_logout_subpacket>,
    pub ev7: core::mem::ManuallyDrop<ev7_pal_processor_subpacket>,
    pub zbox: core::mem::ManuallyDrop<ev7_pal_zbox_subpacket>,
    pub rbox: core::mem::ManuallyDrop<ev7_pal_rbox_subpacket>,
    pub io: core::mem::ManuallyDrop<ev7_pal_io_subpacket>,
    pub env: core::mem::ManuallyDrop<ev7_pal_environmental_subpacket>,
    pub as_quad: [u64; 1],
}

#[repr(C)]
pub struct ev7_pal_subpacket {
    pub by_type: ev7_pal_subpacket_by_type,
}

/* Struct to contain collected logout from subpackets. */
#[repr(C)]
pub struct ev7_lf_subpackets {
    pub logout: *mut ev7_pal_logout_subpacket,
    pub ev7: *mut ev7_pal_processor_subpacket,
    pub zbox: *mut ev7_pal_zbox_subpacket,
    pub rbox: *mut ev7_pal_rbox_subpacket,
    pub io: *mut ev7_pal_io_subpacket,
    pub env: [*mut ev7_pal_environmental_subpacket; 7],
    pub io_pid: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
