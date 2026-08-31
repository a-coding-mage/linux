// SPDX-License-Identifier: GPL-2.0
/*
 * Kselftest for PCI Endpoint Subsystem
 *
 * Copyright (c) 2022 Samsung Electronics Co., Ltd.
 *             https://www.samsung.com
 * Author: Aman Gupta <aman1.gupta@samsung.com>
 *
 * Copyright (c) 2024, Linaro Ltd.
 * Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 */

/* Translated from C includes:
 * <errno.h>, <fcntl.h>, <stdbool.h>, <stdio.h>, <stdlib.h>,
 * <sys/ioctl.h>, <unistd.h>,
 * "../../../../include/uapi/linux/pcitest.h",
 * "kselftest_harness.h"
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
}

extern "C" {
    static PCITEST_BAR: c_ulong;
    static PCITEST_BAR_SUBRANGE: c_ulong;
    static PCITEST_BARS: c_ulong;
    static PCITEST_SET_IRQTYPE: c_ulong;
    static PCITEST_GET_IRQTYPE: c_ulong;
    static PCITEST_LEGACY_IRQ: c_ulong;
    static PCITEST_MSI: c_ulong;
    static PCITEST_MSIX: c_ulong;
    static PCITEST_READ: c_ulong;
    static PCITEST_WRITE: c_ulong;
    static PCITEST_COPY: c_ulong;
    static PCITEST_DOORBELL: c_ulong;
}

extern "C" {
    static PCITEST_IRQ_TYPE_AUTO: c_int;
    static PCITEST_IRQ_TYPE_INTX: c_int;
    static PCITEST_IRQ_TYPE_MSI: c_int;
    static PCITEST_IRQ_TYPE_MSIX: c_int;
    static PCITEST_FLAGS_USE_DMA: c_ulong;
}

const O_RDWR: c_int = 0o2;
const ENODATA: c_int = 61;
const ENOBUFS: c_int = 105;
const EBUSY: c_int = 16;
const EOPNOTSUPP: c_int = 95;
const ENOSPC: c_int = 28;
const EINVAL: c_int = 22;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_endpoint_test_xfer_param {
    pub size: c_ulong,
    pub flags: c_ulong,
}

impl Default for pci_endpoint_test_xfer_param {
    fn default() -> Self {
        Self { size: 0, flags: 0 }
    }
}

static TEST_DEVICE: &[u8] = b"/dev/pci-endpoint-test.0\0";
static TEST_SIZE: [c_ulong; 5] = [1, 1024, 1025, 1024000, 1024001];

unsafe fn pci_ep_ioctl(fd: c_int, cmd: c_ulong, arg: c_ulong) -> c_int {
    let ret = ioctl(fd, cmd, arg);
    if ret < 0 {
        -*__errno_location()
    } else {
        ret
    }
}

unsafe fn pci_ep_ioctl_ptr(fd: c_int, cmd: c_ulong, arg: *mut c_void) -> c_int {
    let ret = ioctl(fd, cmd, arg);
    if ret < 0 {
        -*__errno_location()
    } else {
        ret
    }
}

#[repr(C)]
pub struct pci_ep_bar {
    pub fd: c_int,
}

unsafe fn pci_ep_bar_setup(self_: *mut pci_ep_bar) {
    (*self_).fd = open(TEST_DEVICE.as_ptr() as *const c_char, O_RDWR);

    ASSERT_NE!(-1, (*self_).fd, "Can't open PCI Endpoint Test device");
}

unsafe fn pci_ep_bar_teardown(self_: *mut pci_ep_bar) {
    close((*self_).fd);
}

#[repr(C)]
pub struct pci_ep_bar_variant {
    pub barno: c_int,
}

static PCI_EP_BAR_BAR0: pci_ep_bar_variant = pci_ep_bar_variant { barno: 0 };
static PCI_EP_BAR_BAR1: pci_ep_bar_variant = pci_ep_bar_variant { barno: 1 };
static PCI_EP_BAR_BAR2: pci_ep_bar_variant = pci_ep_bar_variant { barno: 2 };
static PCI_EP_BAR_BAR3: pci_ep_bar_variant = pci_ep_bar_variant { barno: 3 };
static PCI_EP_BAR_BAR4: pci_ep_bar_variant = pci_ep_bar_variant { barno: 4 };
static PCI_EP_BAR_BAR5: pci_ep_bar_variant = pci_ep_bar_variant { barno: 5 };

unsafe fn pci_ep_bar_BAR_TEST(self_: *mut pci_ep_bar, variant: *const pci_ep_bar_variant) {
    let mut ret: c_int;

    ret = pci_ep_ioctl((*self_).fd, PCITEST_BAR, (*variant).barno as c_ulong);
    if ret == -ENODATA {
        SKIP!("BAR is disabled");
        return;
    }
    if ret == -ENOBUFS {
        SKIP!("BAR is reserved");
        return;
    }
    EXPECT_FALSE!(ret, "Test failed for BAR{}", (*variant).barno);
}

unsafe fn pci_ep_bar_BAR_SUBRANGE_TEST(self_: *mut pci_ep_bar, variant: *const pci_ep_bar_variant) {
    let mut ret: c_int;

    ret = pci_ep_ioctl((*self_).fd, PCITEST_SET_IRQTYPE, PCITEST_IRQ_TYPE_AUTO as c_ulong);
    ASSERT_EQ!(0, ret, "Can't set AUTO IRQ type");

    ret = pci_ep_ioctl((*self_).fd, PCITEST_BAR_SUBRANGE, (*variant).barno as c_ulong);
    if ret == -ENODATA {
        SKIP!("BAR is disabled");
        return;
    }
    if ret == -EBUSY {
        SKIP!("BAR is test register space");
        return;
    }
    if ret == -EOPNOTSUPP {
        SKIP!("Subrange map is not supported");
        return;
    }
    if ret == -ENOBUFS {
        SKIP!("BAR is reserved");
        return;
    }
    if ret == -ENOSPC {
        SKIP!("Not enough inbound windows");
        return;
    }
    EXPECT_FALSE!(ret, "Test failed for BAR{}", (*variant).barno);
}

#[repr(C)]
pub struct pci_ep_basic {
    pub fd: c_int,
}

unsafe fn pci_ep_basic_setup(self_: *mut pci_ep_basic) {
    (*self_).fd = open(TEST_DEVICE.as_ptr() as *const c_char, O_RDWR);

    ASSERT_NE!(-1, (*self_).fd, "Can't open PCI Endpoint Test device");
}

unsafe fn pci_ep_basic_teardown(self_: *mut pci_ep_basic) {
    close((*self_).fd);
}

unsafe fn pci_ep_basic_CONSECUTIVE_BAR_TEST(self_: *mut pci_ep_basic) {
    let mut ret: c_int;

    ret = pci_ep_ioctl((*self_).fd, PCITEST_BARS, 0);
    EXPECT_FALSE!(ret, "Consecutive BAR test failed");
}

unsafe fn pci_ep_basic_LEGACY_IRQ_TEST(self_: *mut pci_ep_basic) {
    let mut ret: c_int;

    ret = pci_ep_ioctl((*self_).fd, PCITEST_SET_IRQTYPE, PCITEST_IRQ_TYPE_INTX as c_ulong);
    ASSERT_EQ!(0, ret, "Can't set Legacy IRQ type");

    ret = pci_ep_ioctl((*self_).fd, PCITEST_GET_IRQTYPE, 0);
    ASSERT_EQ!(PCITEST_IRQ_TYPE_INTX, ret, "Can't get Legacy IRQ type");

    ret = pci_ep_ioctl((*self_).fd, PCITEST_LEGACY_IRQ, 0);
    EXPECT_FALSE!(ret, "Test failed for Legacy IRQ");
}

unsafe fn pci_ep_basic_MSI_TEST(self_: *mut pci_ep_basic) {
    let mut ret: c_int;
    let mut i: c_int;

    ret = pci_ep_ioctl((*self_).fd, PCITEST_SET_IRQTYPE, PCITEST_IRQ_TYPE_MSI as c_ulong);
    ASSERT_EQ!(0, ret, "Can't set MSI IRQ type");

    ret = pci_ep_ioctl((*self_).fd, PCITEST_GET_IRQTYPE, 0);
    ASSERT_EQ!(PCITEST_IRQ_TYPE_MSI, ret, "Can't get MSI IRQ type");

    i = 1;
    while i <= 32 {
        ret = pci_ep_ioctl((*self_).fd, PCITEST_MSI, i as c_ulong);
        if ret == -EINVAL {
            SKIP!("MSI{} is disabled", i);
            return;
        }
        EXPECT_FALSE!(ret, "Test failed for MSI{}", i);
        i += 1;
    }
}

unsafe fn pci_ep_basic_MSIX_TEST(self_: *mut pci_ep_basic) {
    let mut ret: c_int;
    let mut i: c_int;

    ret = pci_ep_ioctl((*self_).fd, PCITEST_SET_IRQTYPE, PCITEST_IRQ_TYPE_MSIX as c_ulong);
    ASSERT_EQ!(0, ret, "Can't set MSI-X IRQ type");

    ret = pci_ep_ioctl((*self_).fd, PCITEST_GET_IRQTYPE, 0);
    ASSERT_EQ!(PCITEST_IRQ_TYPE_MSIX, ret, "Can't get MSI-X IRQ type");

    i = 1;
    while i <= 2048 {
        ret = pci_ep_ioctl((*self_).fd, PCITEST_MSIX, i as c_ulong);
        if ret == -EINVAL {
            SKIP!("MSI-X{} is disabled", i);
            return;
        }
        EXPECT_FALSE!(ret, "Test failed for MSI-X{}", i);
        i += 1;
    }
}

#[repr(C)]
pub struct pci_ep_data_transfer {
    pub fd: c_int,
}

unsafe fn pci_ep_data_transfer_setup(self_: *mut pci_ep_data_transfer) {
    (*self_).fd = open(TEST_DEVICE.as_ptr() as *const c_char, O_RDWR);

    ASSERT_NE!(-1, (*self_).fd, "Can't open PCI Endpoint Test device");
}

unsafe fn pci_ep_data_transfer_teardown(self_: *mut pci_ep_data_transfer) {
    close((*self_).fd);
}

#[repr(C)]
pub struct pci_ep_data_transfer_variant {
    pub use_dma: bool,
}

static PCI_EP_DATA_TRANSFER_MEMCPY: pci_ep_data_transfer_variant =
    pci_ep_data_transfer_variant { use_dma: false };

static PCI_EP_DATA_TRANSFER_DMA: pci_ep_data_transfer_variant =
    pci_ep_data_transfer_variant { use_dma: true };

unsafe fn pci_ep_data_transfer_READ_TEST(
    self_: *mut pci_ep_data_transfer,
    variant: *const pci_ep_data_transfer_variant,
) {
    let mut param: pci_endpoint_test_xfer_param = Default::default();
    let mut ret: c_int;
    let mut i: usize;

    if (*variant).use_dma {
        param.flags = PCITEST_FLAGS_USE_DMA;
    }

    ret = pci_ep_ioctl((*self_).fd, PCITEST_SET_IRQTYPE, PCITEST_IRQ_TYPE_AUTO as c_ulong);
    ASSERT_EQ!(0, ret, "Can't set AUTO IRQ type");

    i = 0;
    while i < TEST_SIZE.len() {
        param.size = TEST_SIZE[i];
        ret = pci_ep_ioctl_ptr(
            (*self_).fd,
            PCITEST_READ,
            &mut param as *mut pci_endpoint_test_xfer_param as *mut c_void,
        );
        EXPECT_FALSE!(ret, "Test failed for size ({})", TEST_SIZE[i]);
        i += 1;
    }
}

unsafe fn pci_ep_data_transfer_WRITE_TEST(
    self_: *mut pci_ep_data_transfer,
    variant: *const pci_ep_data_transfer_variant,
) {
    let mut param: pci_endpoint_test_xfer_param = Default::default();
    let mut ret: c_int;
    let mut i: usize;

    if (*variant).use_dma {
        param.flags = PCITEST_FLAGS_USE_DMA;
    }

    ret = pci_ep_ioctl((*self_).fd, PCITEST_SET_IRQTYPE, PCITEST_IRQ_TYPE_AUTO as c_ulong);
    ASSERT_EQ!(0, ret, "Can't set AUTO IRQ type");

    i = 0;
    while i < TEST_SIZE.len() {
        param.size = TEST_SIZE[i];
        ret = pci_ep_ioctl_ptr(
            (*self_).fd,
            PCITEST_WRITE,
            &mut param as *mut pci_endpoint_test_xfer_param as *mut c_void,
        );
        EXPECT_FALSE!(ret, "Test failed for size ({})", TEST_SIZE[i]);
        i += 1;
    }
}

unsafe fn pci_ep_data_transfer_COPY_TEST(
    self_: *mut pci_ep_data_transfer,
    variant: *const pci_ep_data_transfer_variant,
) {
    let mut param: pci_endpoint_test_xfer_param = Default::default();
    let mut ret: c_int;
    let mut i: usize;

    if (*variant).use_dma {
        param.flags = PCITEST_FLAGS_USE_DMA;
    }

    ret = pci_ep_ioctl((*self_).fd, PCITEST_SET_IRQTYPE, PCITEST_IRQ_TYPE_AUTO as c_ulong);
    ASSERT_EQ!(0, ret, "Can't set AUTO IRQ type");

    i = 0;
    while i < TEST_SIZE.len() {
        param.size = TEST_SIZE[i];
        ret = pci_ep_ioctl_ptr(
            (*self_).fd,
            PCITEST_COPY,
            &mut param as *mut pci_endpoint_test_xfer_param as *mut c_void,
        );
        EXPECT_FALSE!(ret, "Test failed for size ({})", TEST_SIZE[i]);
        i += 1;
    }
}

#[repr(C)]
pub struct pcie_ep_doorbell {
    pub fd: c_int,
}

unsafe fn pcie_ep_doorbell_setup(self_: *mut pcie_ep_doorbell) {
    (*self_).fd = open(TEST_DEVICE.as_ptr() as *const c_char, O_RDWR);

    ASSERT_NE!(-1, (*self_).fd, "Can't open PCI Endpoint Test device");
}

unsafe fn pcie_ep_doorbell_teardown(self_: *mut pcie_ep_doorbell) {
    close((*self_).fd);
}

unsafe fn pcie_ep_doorbell_DOORBELL_TEST(self_: *mut pcie_ep_doorbell) {
    let mut ret: c_int;

    ret = pci_ep_ioctl((*self_).fd, PCITEST_SET_IRQTYPE, PCITEST_IRQ_TYPE_AUTO as c_ulong);
    ASSERT_EQ!(0, ret, "Can't set AUTO IRQ type");

    ret = pci_ep_ioctl((*self_).fd, PCITEST_DOORBELL, 0);
    if ret == -EOPNOTSUPP {
        SKIP!("Doorbell test is not supported");
        return;
    }
    EXPECT_FALSE!(ret, "Test failed for Doorbell\n");
}

TEST_HARNESS_MAIN!();
