/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021-2023 Alibaba Inc.
 * Copyright (C) 2025 Linaro Ltd.
 *
 * Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 */

// Dependency intent: symbols supplied by the Linux PCI ID definitions.

pub const PCI_VENDOR_ID_LECARC: u16 = 0x0720;

#[repr(C)]
pub struct dwc_pcie_vsec_id {
    pub vendor_id: u16,
    pub vsec_id: u16,
    pub vsec_rev: u8,
}

/*
 * VSEC IDs are allocated by the vendor, so a given ID may mean different
 * things to different vendors.  See PCIe r6.0, sec 7.9.5.2.
 */
pub static dwc_pcie_rasdes_vsec_ids: [dwc_pcie_vsec_id; 8] = [
    dwc_pcie_vsec_id {
        vendor_id: PCI_VENDOR_ID_ALIBABA,
        vsec_id: 0x02,
        vsec_rev: 0x4,
    },
    dwc_pcie_vsec_id {
        vendor_id: PCI_VENDOR_ID_AMPERE,
        vsec_id: 0x02,
        vsec_rev: 0x4,
    },
    dwc_pcie_vsec_id {
        vendor_id: PCI_VENDOR_ID_PICOHEART,
        vsec_id: 0x02,
        vsec_rev: 0x4,
    },
    dwc_pcie_vsec_id {
        vendor_id: PCI_VENDOR_ID_QCOM,
        vsec_id: 0x02,
        vsec_rev: 0x4,
    },
    dwc_pcie_vsec_id {
        vendor_id: PCI_VENDOR_ID_ROCKCHIP,
        vsec_id: 0x02,
        vsec_rev: 0x4,
    },
    dwc_pcie_vsec_id {
        vendor_id: PCI_VENDOR_ID_SAMSUNG,
        vsec_id: 0x02,
        vsec_rev: 0x4,
    },
    dwc_pcie_vsec_id {
        vendor_id: PCI_VENDOR_ID_LECARC,
        vsec_id: 0x02,
        vsec_rev: 0x4,
    },
    dwc_pcie_vsec_id {
        vendor_id: 0,
        vsec_id: 0,
        vsec_rev: 0,
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
