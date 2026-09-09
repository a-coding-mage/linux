/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct ata_port_info {
    _private: [u8; 0],
}

/* pata_sis.c */
unsafe extern "C" {
    pub static sis_info133_for_sata: ata_port_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
