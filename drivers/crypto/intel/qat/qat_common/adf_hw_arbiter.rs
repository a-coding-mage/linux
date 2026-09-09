// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependencies supplied by the surrounding driver translation unit:
// adf_accel_devices.h, adf_common_drv.h, and adf_transport_internal.h.

const ADF_ARB_NUM: usize = 4;
const ADF_ARB_REG_SIZE: usize = 0x4;

#[inline]
unsafe fn write_csr_arb_sarconfig(
    csr_addr: *mut core::ffi::c_void,
    arb_offset: u32,
    index: usize,
    value: u32,
) {
    ADF_CSR_WR!(
        csr_addr,
        arb_offset + (ADF_ARB_REG_SIZE as u32 * index as u32),
        value
    );
}

#[inline]
unsafe fn write_csr_arb_wt2sam(
    csr_addr: *mut core::ffi::c_void,
    arb_offset: u32,
    wt_offset: u32,
    index: usize,
    value: u32,
) {
    ADF_CSR_WR!(
        csr_addr,
        (arb_offset + wt_offset) + (ADF_ARB_REG_SIZE as u32 * index as u32),
        value
    );
}

pub unsafe fn adf_init_arb(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_data = (*accel_dev).hw_device;
    let csr = (*(*accel_dev).transport).banks[0].csr_addr;
    let ae_mask: libc::c_ulong = (*hw_data).ae_mask;
    let (mut arb_off, mut wt_off, mut arb_cfg): (u32, u32, u32);
    let thd_2_arb_cfg: *const u32;
    let mut info = arb_info::default();

    ((*hw_data).get_arb_info)(&mut info);
    arb_cfg = info.arb_cfg;
    arb_off = info.arb_offset;
    wt_off = info.wt2sam_offset;

    /* Service arb configured for 32 bytes responses and
     * ring flow control check enabled. */
    for arb in 0..ADF_ARB_NUM {
        write_csr_arb_sarconfig(csr, arb_off, arb, arb_cfg);
    }

    /* Map worker threads to service arbiters */
    thd_2_arb_cfg = ((*hw_data).get_arb_mapping)(accel_dev);

    for i in 0..(*hw_data).num_engines as usize {
        if (ae_mask & ((1 as libc::c_ulong) << i)) != 0 {
            write_csr_arb_wt2sam(csr, arb_off, wt_off, i, *thd_2_arb_cfg.add(i));
        }
    }

    0
}

pub unsafe fn adf_update_ring_arb(ring: *mut adf_etr_ring_data) {
    let accel_dev = (*(*ring).bank).accel_dev;
    let hw_data = (*accel_dev).hw_device;
    let csr_ops = GET_CSR_OPS!(accel_dev);
    let tx_ring_mask: u32 = (*hw_data).tx_rings_mask;
    let shift: u32 = (*hw_data).tx_rx_gap;
    let rx_ring_mask: u32;
    let arben_tx: u32;
    let arben_rx: u32;
    let arben: u32;

    /*
     * Enable arbitration on a ring only if the TX half of the ring mask
     * matches the RX part. This results in writes to CSR on both TX and
     * RX update - only one is necessary, but both are done for
     * simplicity.
     */
    rx_ring_mask = tx_ring_mask << shift;
    arben_tx = ((*(*ring).bank).ring_mask & tx_ring_mask) >> 0;
    arben_rx = ((*(*ring).bank).ring_mask & rx_ring_mask) >> shift;
    arben = arben_tx & arben_rx;

    ((*csr_ops).write_csr_ring_srv_arb_en)(
        (*(*ring).bank).csr_addr,
        (*(*ring).bank).bank_number,
        arben,
    );
}

pub unsafe fn adf_exit_arb(accel_dev: *mut adf_accel_dev) {
    let hw_data = (*accel_dev).hw_device;
    let csr_ops = GET_CSR_OPS!(accel_dev);
    let (mut arb_off, mut wt_off): (u32, u32);
    let mut info = arb_info::default();
    let csr: *mut core::ffi::c_void;

    ((*hw_data).get_arb_info)(&mut info);
    arb_off = info.arb_offset;
    wt_off = info.wt2sam_offset;

    if (*accel_dev).transport.is_null() {
        return;
    }

    csr = (*(*accel_dev).transport).banks[0].csr_addr;

    ((*hw_data).get_arb_info)(&mut info);

    /* Unmap worker threads to service arbiters */
    for i in 0..(*hw_data).num_engines as usize {
        write_csr_arb_wt2sam(csr, arb_off, wt_off, i, 0);
    }

    /* Disable arbitration on all rings */
    for i in 0..GET_MAX_BANKS!(accel_dev) as usize {
        ((*csr_ops).write_csr_ring_srv_arb_en)(csr, i, 0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
