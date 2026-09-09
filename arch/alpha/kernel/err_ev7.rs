// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/err_ev7.c
 *
 * Error handling code supporting Alpha systems
 */

// Kernel and architecture dependencies supplied externally.

pub unsafe fn ev7_collect_logout_frame_subpackets(
    mut el_ptr: *mut el_subpacket,
    lf_subpackets: *mut ev7_lf_subpackets,
) -> *mut ev7_lf_subpackets {
    let mut subpacket: *mut el_subpacket;
    let mut i: i32;

    if (*el_ptr).class != EL_CLASS__HEADER || (*el_ptr).type_ != EL_TYPE__HEADER__LOGOUT_FRAME {
        return core::ptr::null_mut();
    }

    el_ptr = ((el_ptr as usize) + (*el_ptr).length as usize) as *mut el_subpacket;

    if (*el_ptr).class != EL_CLASS__PAL || (*el_ptr).type_ != EL_TYPE__PAL__LOGOUT_FRAME {
        return core::ptr::null_mut();
    }

    (*lf_subpackets).logout = (*el_ptr).by_type.raw.data_start as *mut ev7_pal_logout_subpacket;

    subpacket = ((el_ptr as usize) + (*el_ptr).length as usize) as *mut el_subpacket;
    i = 0;
    while !subpacket.is_null() && i < (*(*lf_subpackets).logout).subpacket_count {
        if (*subpacket).class != EL_CLASS__PAL {
            printk(
                c"%s**UNEXPECTED SUBPACKET CLASS %d IN LOGOUT FRAME (packet %d\n",
                err_print_prefix,
                (*subpacket).class,
                i,
            );
            return core::ptr::null_mut();
        }

        match (*subpacket).type_ {
            EL_TYPE__PAL__EV7_PROCESSOR => {
                (*lf_subpackets).ev7 = (*subpacket).by_type.raw.data_start as *mut ev7_pal_processor_subpacket;
            }
            EL_TYPE__PAL__EV7_RBOX => {
                (*lf_subpackets).rbox = (*subpacket).by_type.raw.data_start as *mut ev7_pal_rbox_subpacket;
            }
            EL_TYPE__PAL__EV7_ZBOX => {
                (*lf_subpackets).zbox = (*subpacket).by_type.raw.data_start as *mut ev7_pal_zbox_subpacket;
            }
            EL_TYPE__PAL__EV7_IO => {
                (*lf_subpackets).io = (*subpacket).by_type.raw.data_start as *mut ev7_pal_io_subpacket;
            }
            EL_TYPE__PAL__ENV__AMBIENT_TEMPERATURE
            | EL_TYPE__PAL__ENV__AIRMOVER_FAN
            | EL_TYPE__PAL__ENV__VOLTAGE
            | EL_TYPE__PAL__ENV__INTRUSION
            | EL_TYPE__PAL__ENV__POWER_SUPPLY
            | EL_TYPE__PAL__ENV__LAN
            | EL_TYPE__PAL__ENV__HOT_PLUG => {
                (*lf_subpackets).env[ev7_lf_env_index((*subpacket).type_) as usize] =
                    (*subpacket).by_type.raw.data_start as *mut ev7_pal_environmental_subpacket;
            }
            _ => return core::ptr::null_mut(),
        }

        subpacket = ((subpacket as usize) + (*subpacket).length as usize) as *mut el_subpacket;
        i += 1;
    }

    lf_subpackets
}

pub unsafe fn ev7_machine_check(vector: c_ulong, la_ptr: c_ulong) {
    let el_ptr = la_ptr as *mut el_subpacket;
    let saved_err_prefix = err_print_prefix;

    mb();
    draina();

    err_print_prefix = KERN_CRIT;
    printk(
        c"%s*CPU %s Error (Vector 0x%x) reported on CPU %d\n",
        err_print_prefix,
        if vector == SCB_Q_PROCERR { c"Correctable" } else { c"Uncorrectable" },
        vector as c_uint,
        smp_processor_id() as c_int,
    );
    el_process_subpacket(el_ptr);
    err_print_prefix = saved_err_prefix;

    wrmces(0x7);
    mb();
}

static mut el_ev7_processor_subpacket_annotation: [*mut c_char; 26] = [
    c"Subpacket Header".as_ptr() as *mut c_char, c"I_STAT".as_ptr() as *mut c_char,
    c"DC_STAT".as_ptr() as *mut c_char, c"C_ADDR".as_ptr() as *mut c_char,
    c"C_SYNDROME_1".as_ptr() as *mut c_char, c"C_SYNDROME_0".as_ptr() as *mut c_char,
    c"C_STAT".as_ptr() as *mut c_char, c"C_STS".as_ptr() as *mut c_char,
    c"MM_STAT".as_ptr() as *mut c_char, c"EXC_ADDR".as_ptr() as *mut c_char,
    c"IER_CM".as_ptr() as *mut c_char, c"ISUM".as_ptr() as *mut c_char,
    c"PAL_BASE".as_ptr() as *mut c_char, c"I_CTL".as_ptr() as *mut c_char,
    c"PROCESS_CONTEXT".as_ptr() as *mut c_char, c"CBOX_CTL".as_ptr() as *mut c_char,
    c"CBOX_STP_CTL".as_ptr() as *mut c_char, c"CBOX_ACC_CTL".as_ptr() as *mut c_char,
    c"CBOX_LCL_SET".as_ptr() as *mut c_char, c"CBOX_GLB_SET".as_ptr() as *mut c_char,
    c"BBOX_CTL".as_ptr() as *mut c_char, c"BBOX_ERR_STS".as_ptr() as *mut c_char,
    c"BBOX_ERR_IDX".as_ptr() as *mut c_char, c"CBOX_DDP_ERR_STS".as_ptr() as *mut c_char,
    c"BBOX_DAT_RMP".as_ptr() as *mut c_char, core::ptr::null_mut(),
];

static mut el_ev7_zbox_subpacket_annotation: [*mut c_char; 18] = [
    c"Subpacket Header".as_ptr() as *mut c_char,
    c"ZBOX(0): DRAM_ERR_STATUS_2 / DRAM_ERR_STATUS_1".as_ptr() as *mut c_char,
    c"ZBOX(0): DRAM_ERROR_CTL    / DRAM_ERR_STATUS_3".as_ptr() as *mut c_char,
    c"ZBOX(0): DIFT_TIMEOUT      / DRAM_ERR_ADR".as_ptr() as *mut c_char,
    c"ZBOX(0): FRC_ERR_ADR       / DRAM_MAPPER_CTL".as_ptr() as *mut c_char,
    c"ZBOX(0): reserved          / DIFT_ERR_STATUS".as_ptr() as *mut c_char,
    c"ZBOX(1): DRAM_ERR_STATUS_2 / DRAM_ERR_STATUS_1".as_ptr() as *mut c_char,
    c"ZBOX(1): DRAM_ERROR_CTL    / DRAM_ERR_STATUS_3".as_ptr() as *mut c_char,
    c"ZBOX(1): DIFT_TIMEOUT      / DRAM_ERR_ADR".as_ptr() as *mut c_char,
    c"ZBOX(1): FRC_ERR_ADR       / DRAM_MAPPER_CTL".as_ptr() as *mut c_char,
    c"ZBOX(1): reserved          / DIFT_ERR_STATUS".as_ptr() as *mut c_char,
    c"CBOX_CTL".as_ptr() as *mut c_char, c"CBOX_STP_CTL".as_ptr() as *mut c_char,
    c"ZBOX(0)_ERROR_PA".as_ptr() as *mut c_char, c"ZBOX(1)_ERROR_PA".as_ptr() as *mut c_char,
    c"ZBOX(0)_ORED_SYNDROME".as_ptr() as *mut c_char,
    c"ZBOX(1)_ORED_SYNDROME".as_ptr() as *mut c_char, core::ptr::null_mut(),
];

static mut el_ev7_rbox_subpacket_annotation: [*mut c_char; 17] = [
    c"Subpacket Header".as_ptr() as *mut c_char, c"RBOX_CFG".as_ptr() as *mut c_char,
    c"RBOX_N_CFG".as_ptr() as *mut c_char, c"RBOX_S_CFG".as_ptr() as *mut c_char,
    c"RBOX_E_CFG".as_ptr() as *mut c_char, c"RBOX_W_CFG".as_ptr() as *mut c_char,
    c"RBOX_N_ERR".as_ptr() as *mut c_char, c"RBOX_S_ERR".as_ptr() as *mut c_char,
    c"RBOX_E_ERR".as_ptr() as *mut c_char, c"RBOX_W_ERR".as_ptr() as *mut c_char,
    c"RBOX_IO_CFG".as_ptr() as *mut c_char, c"RBOX_IO_ERR".as_ptr() as *mut c_char,
    c"RBOX_L_ERR".as_ptr() as *mut c_char, c"RBOX_WHOAMI".as_ptr() as *mut c_char,
    c"RBOX_IMASL".as_ptr() as *mut c_char, c"RBOX_INTQ".as_ptr() as *mut c_char,
    c"RBOX_INT".as_ptr() as *mut c_char,
];

static mut el_ev7_io_subpacket_annotation: [*mut c_char; 52] = [
    c"Subpacket Header".as_ptr() as *mut c_char, c"IO_ASIC_REV".as_ptr() as *mut c_char,
    c"IO_SYS_REV".as_ptr() as *mut c_char, c"IO7_UPH".as_ptr() as *mut c_char,
    c"HPI_CTL".as_ptr() as *mut c_char, c"CRD_CTL".as_ptr() as *mut c_char,
    c"HEI_CTL".as_ptr() as *mut c_char, c"PO7_ERROR_SUM".as_ptr() as *mut c_char,
    c"PO7_UNCRR_SYM".as_ptr() as *mut c_char, c"PO7_CRRCT_SYM".as_ptr() as *mut c_char,
    c"PO7_UGBGE_SYM".as_ptr() as *mut c_char, c"PO7_ERR_PKT0".as_ptr() as *mut c_char,
    c"PO7_ERR_PKT1".as_ptr() as *mut c_char, c"reserved".as_ptr() as *mut c_char,
    c"reserved".as_ptr() as *mut c_char, c"PO0_ERR_SUM".as_ptr() as *mut c_char,
    c"PO0_TLB_ERR".as_ptr() as *mut c_char, c"PO0_SPL_COMPLT".as_ptr() as *mut c_char,
    c"PO0_TRANS_SUM".as_ptr() as *mut c_char, c"PO0_FIRST_ERR".as_ptr() as *mut c_char,
    c"PO0_MULT_ERR".as_ptr() as *mut c_char, c"DM CSR PH".as_ptr() as *mut c_char,
    c"DM CSR PH".as_ptr() as *mut c_char, c"DM CSR PH".as_ptr() as *mut c_char,
    c"DM CSR PH".as_ptr() as *mut c_char, c"reserved".as_ptr() as *mut c_char,
    c"PO1_ERR_SUM".as_ptr() as *mut c_char, c"PO1_TLB_ERR".as_ptr() as *mut c_char,
    c"PO1_SPL_COMPLT".as_ptr() as *mut c_char, c"PO1_TRANS_SUM".as_ptr() as *mut c_char,
    c"PO1_FIRST_ERR".as_ptr() as *mut c_char, c"PO1_MULT_ERR".as_ptr() as *mut c_char,
    c"DM CSR PH".as_ptr() as *mut c_char, c"DM CSR PH".as_ptr() as *mut c_char,
    c"DM CSR PH".as_ptr() as *mut c_char, c"DM CSR PH".as_ptr() as *mut c_char,
    c"reserved".as_ptr() as *mut c_char, c"PO2_ERR_SUM".as_ptr() as *mut c_char,
    c"PO2_TLB_ERR".as_ptr() as *mut c_char, c"PO2_SPL_COMPLT".as_ptr() as *mut c_char,
    c"PO2_TRANS_SUM".as_ptr() as *mut c_char, c"PO2_FIRST_ERR".as_ptr() as *mut c_char,
    c"PO2_MULT_ERR".as_ptr() as *mut c_char, c"DM CSR PH".as_ptr() as *mut c_char,
    c"DM CSR PH".as_ptr() as *mut c_char, c"DM CSR PH".as_ptr() as *mut c_char,
    c"DM CSR PH".as_ptr() as *mut c_char, c"reserved".as_ptr() as *mut c_char,
    c"PO3_ERR_SUM".as_ptr() as *mut c_char, c"PO3_TLB_ERR".as_ptr() as *mut c_char,
    c"PO3_SPL_COMPLT".as_ptr() as *mut c_char, c"PO3_TRANS_SUM".as_ptr() as *mut c_char,
    c"PO3_FIRST_ERR".as_ptr() as *mut c_char, c"PO3_MULT_ERR".as_ptr() as *mut c_char,
    c"DM CSR PH".as_ptr() as *mut c_char, c"DM CSR PH".as_ptr() as *mut c_char,
    c"DM CSR PH".as_ptr() as *mut c_char, c"DM CSR PH".as_ptr() as *mut c_char,
    c"reserved".as_ptr() as *mut c_char,
];

static mut el_ev7_pal_annotations: [el_subpacket_annotation; 4] = [
    SUBPACKET_ANNOTATION!(EL_CLASS__PAL, EL_TYPE__PAL__EV7_PROCESSOR, 1, c"EV7 Processor Subpacket", el_ev7_processor_subpacket_annotation),
    SUBPACKET_ANNOTATION!(EL_CLASS__PAL, EL_TYPE__PAL__EV7_ZBOX, 1, c"EV7 ZBOX Subpacket", el_ev7_zbox_subpacket_annotation),
    SUBPACKET_ANNOTATION!(EL_CLASS__PAL, EL_TYPE__PAL__EV7_RBOX, 1, c"EV7 RBOX Subpacket", el_ev7_rbox_subpacket_annotation),
    SUBPACKET_ANNOTATION!(EL_CLASS__PAL, EL_TYPE__PAL__EV7_IO, 1, c"EV7 IO Subpacket", el_ev7_io_subpacket_annotation),
];

pub unsafe fn ev7_process_pal_subpacket(header: *mut el_subpacket) -> *mut el_subpacket {
    if (*header).class != EL_CLASS__PAL {
        printk(c"%s  ** Unexpected header CLASS %d TYPE %d, aborting\n", err_print_prefix, (*header).class, (*header).type_);
        return core::ptr::null_mut();
    }

    let packet = (*header).by_type.raw.data_start as *mut ev7_pal_subpacket;
    match (*header).type_ {
        EL_TYPE__PAL__LOGOUT_FRAME => {
            printk(c"%s*** MCHK occurred on LPID %lld (RBOX %llx)\n", err_print_prefix, (*packet).by_type.logout.whami, (*packet).by_type.logout.rbox_whami);
            el_print_timestamp(&(*packet).by_type.logout.timestamp);
            printk(c"%s  EXC_ADDR: %016llx\n  HALT_CODE: %llx\n", err_print_prefix, (*packet).by_type.logout.exc_addr, (*packet).by_type.logout.halt_code);
            el_process_subpackets(header, (*packet).by_type.logout.subpacket_count);
        }
        _ => {
            printk(c"%s  ** PAL TYPE %d SUBPACKET\n", err_print_prefix, (*header).type_);
            el_annotate_subpacket(header);
        }
    }

    ((header as usize) + (*header).length as usize) as *mut el_subpacket
}

pub static mut ev7_pal_subpacket_handler: el_subpacket_handler =
    SUBPACKET_HANDLER_INIT!(EL_CLASS__PAL, ev7_process_pal_subpacket);

pub unsafe fn ev7_register_error_handlers() {
    let mut i = 0usize;
    while i < core::mem::size_of_val(&el_ev7_pal_annotations) / core::mem::size_of::<el_subpacket_annotation>() {
        cdl_register_subpacket_annotation(&mut el_ev7_pal_annotations[i]);
        i += 1;
    }
    cdl_register_subpacket_handler(&mut ev7_pal_subpacket_handler);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
