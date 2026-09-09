/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AHCI SATA platform driver
 *
 * Copyright 2004-2005  Red Hat, Inc.
 *   Jeff Garzik <jgarzik@pobox.com>
 * Copyright 2010  MontaVista Software, LLC.
 *   Anton Vorontsov <avorontsov@ru.mvista.com>
 */

// Translated from the C header; linux/compiler.h supplied declarations are
// intentionally left as external dependencies.

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ata_port_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ahci_host_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scsi_host_template {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ahci_platform_enable_phys(hpriv: *mut ahci_host_priv) -> i32;
    pub fn ahci_platform_disable_phys(hpriv: *mut ahci_host_priv);
    pub fn ahci_platform_find_clk(
        hpriv: *mut ahci_host_priv,
        con_id: *const std::ffi::c_char,
    ) -> *mut clk;
    pub fn ahci_platform_enable_clks(hpriv: *mut ahci_host_priv) -> i32;
    pub fn ahci_platform_disable_clks(hpriv: *mut ahci_host_priv);
    pub fn ahci_platform_deassert_rsts(hpriv: *mut ahci_host_priv) -> i32;
    pub fn ahci_platform_assert_rsts(hpriv: *mut ahci_host_priv) -> i32;
    pub fn ahci_platform_enable_regulators(hpriv: *mut ahci_host_priv) -> i32;
    pub fn ahci_platform_disable_regulators(hpriv: *mut ahci_host_priv);
    pub fn ahci_platform_enable_resources(hpriv: *mut ahci_host_priv) -> i32;
    pub fn ahci_platform_disable_resources(hpriv: *mut ahci_host_priv);
    pub fn ahci_platform_get_resources(
        pdev: *mut platform_device,
        flags: u32,
    ) -> *mut ahci_host_priv;
    pub fn ahci_platform_init_host(
        pdev: *mut platform_device,
        hpriv: *mut ahci_host_priv,
        pi_template: *const ata_port_info,
        sht: *const scsi_host_template,
    ) -> i32;

    pub fn ahci_platform_shutdown(pdev: *mut platform_device);

    pub fn ahci_platform_suspend_host(dev: *mut device) -> i32;
    pub fn ahci_platform_resume_host(dev: *mut device) -> i32;
    pub fn ahci_platform_suspend(dev: *mut device) -> i32;
    pub fn ahci_platform_resume(dev: *mut device) -> i32;
}

pub const AHCI_PLATFORM_GET_RESETS: u32 = 1u32 << 0;
pub const AHCI_PLATFORM_RST_TRIGGER: u32 = 1u32 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
