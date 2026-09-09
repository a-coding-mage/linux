/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by nitrox_dev.h in the C source.
#[repr(C)]
pub struct nitrox_device {
    _private: [u8; 0],
}

// C enum vf_mode; its concrete definition is supplied by the dependent code.
pub type vf_mode = core::ffi::c_int;

unsafe extern "C" {
    pub fn nitrox_config_aqm_rings(ndev: *mut nitrox_device);
    pub fn nitrox_config_aqm_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_emu_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_pkt_input_rings(ndev: *mut nitrox_device);
    pub fn nitrox_config_pkt_solicit_ports(ndev: *mut nitrox_device);
    pub fn nitrox_config_nps_core_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_nps_pkt_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_pom_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_rand_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_efl_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_bmi_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_bmo_unit(ndev: *mut nitrox_device);
    pub fn nitrox_config_lbc_unit(ndev: *mut nitrox_device);
    pub fn invalidate_lbc(ndev: *mut nitrox_device);
    pub fn enable_aqm_ring(ndev: *mut nitrox_device, qno: core::ffi::c_int);
    pub fn enable_pkt_input_ring(ndev: *mut nitrox_device, ring: core::ffi::c_int);
    pub fn enable_pkt_solicit_port(ndev: *mut nitrox_device, port: core::ffi::c_int);
    pub fn config_nps_core_vfcfg_mode(ndev: *mut nitrox_device, mode: vf_mode);
    pub fn nitrox_get_hwinfo(ndev: *mut nitrox_device);
    pub fn enable_pf2vf_mbox_interrupts(ndev: *mut nitrox_device);
    pub fn disable_pf2vf_mbox_interrupts(ndev: *mut nitrox_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
