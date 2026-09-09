// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2011 Samsung Electronics Co.Ltd
// Author: Joonyoung Shim <jy0922.shim@samsung.com>

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

pub type u32 = ::core::ffi::c_uint;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum samsung_usb_phy_type {
    USB_PHY_TYPE_DEVICE,
    USB_PHY_TYPE_HOST,
}

extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn clk_get(dev: *mut core::ffi::c_void, id: *const core::ffi::c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> u64;
    fn clk_put(clk: *mut clk);
    fn mdelay(ms: u32);
    fn udelay(us: u32);
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
}

unsafe fn s3c_usb_otgphy_init(pdev: *mut platform_device) -> i32 {
    let mut xusbxti: *mut clk;
    let mut phyclk: u32;

    writel(
        readl(S3C64XX_OTHERS) | S3C64XX_OTHERS_USBMASK,
        S3C64XX_OTHERS,
    );

    /* set clock frequency for PLL */
    phyclk = readl(S3C_PHYCLK) & !S3C_PHYCLK_CLKSEL_MASK;

    xusbxti = clk_get(pdev as *mut core::ffi::c_void, c"xusbxti".as_ptr());
    if !is_err(xusbxti as *const core::ffi::c_void) {
        match clk_get_rate(xusbxti) {
            12 * MHZ as u64 => phyclk |= S3C_PHYCLK_CLKSEL_12M,
            24 * MHZ as u64 => phyclk |= S3C_PHYCLK_CLKSEL_24M,
            _ => {
                /* default reference clock */
            }
        }
        clk_put(xusbxti);
    }

    /* TODO: select external clock/oscillator */
    writel(phyclk | S3C_PHYCLK_CLK_FORCE, S3C_PHYCLK);

    /* set to normal OTG PHY */
    writel(readl(S3C_PHYPWR) & !S3C_PHYPWR_NORMAL_MASK, S3C_PHYPWR);
    mdelay(1);

    /* reset OTG PHY and Link */
    writel(
        S3C_RSTCON_PHY | S3C_RSTCON_HCLK | S3C_RSTCON_PHYCLK,
        S3C_RSTCON,
    );
    udelay(20); /* at-least 10uS */
    writel(0, S3C_RSTCON);

    0
}

unsafe fn s3c_usb_otgphy_exit(_pdev: *mut platform_device) -> i32 {
    writel(
        readl(S3C_PHYPWR) | S3C_PHYPWR_ANALOG_POWERDOWN | S3C_PHYPWR_OTG_DISABLE,
        S3C_PHYPWR,
    );

    writel(
        readl(S3C64XX_OTHERS) & !S3C64XX_OTHERS_USBMASK,
        S3C64XX_OTHERS,
    );

    0
}

pub unsafe fn s3c_usb_phy_init(pdev: *mut platform_device, phy_type: i32) -> i32 {
    if phy_type == USB_PHY_TYPE_DEVICE as i32 {
        return s3c_usb_otgphy_init(pdev);
    }

    -EINVAL
}

pub unsafe fn s3c_usb_phy_exit(pdev: *mut platform_device, phy_type: i32) -> i32 {
    if phy_type == USB_PHY_TYPE_DEVICE as i32 {
        return s3c_usb_otgphy_exit(pdev);
    }

    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
