// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI da8xx master peripheral priority driver
 *
 * Copyright (C) 2016 BayLibre SAS
 *
 * Author:
 *   Bartosz Golaszewski <bgolaszewski@baylibre.com>
 */

// Linux kernel dependencies supplied by other translation units.

/*
 * REVISIT: Linux doesn't have a good framework for the kind of performance
 * knobs this driver controls. We can't use device tree properties as it deals
 * with hardware configuration rather than description. We also don't want to
 * commit to maintaining some random sysfs attributes.
 *
 * For now we just hardcode the register values for the boards that need
 * some changes (as is the case for the LCD controller on da850-lcdk - the
 * first board we support here). When linux gets an appropriate framework,
 * we'll easily convert the driver to it.
 */

const DA8XX_MSTPRI0_OFFSET: i32 = 0;
const DA8XX_MSTPRI1_OFFSET: i32 = 4;
const DA8XX_MSTPRI2_OFFSET: i32 = 8;

const DA8XX_MSTPRI_ARM_I: usize = 0;
const DA8XX_MSTPRI_ARM_D: usize = 1;
const DA8XX_MSTPRI_UPP: usize = 2;
const DA8XX_MSTPRI_SATA: usize = 3;
const DA8XX_MSTPRI_PRU0: usize = 4;
const DA8XX_MSTPRI_PRU1: usize = 5;
const DA8XX_MSTPRI_EDMA30TC0: usize = 6;
const DA8XX_MSTPRI_EDMA30TC1: usize = 7;
const DA8XX_MSTPRI_EDMA31TC0: usize = 8;
const DA8XX_MSTPRI_VPIF_DMA_0: usize = 9;
const DA8XX_MSTPRI_VPIF_DMA_1: usize = 10;
const DA8XX_MSTPRI_EMAC: usize = 11;
const DA8XX_MSTPRI_USB0CFG: usize = 12;
const DA8XX_MSTPRI_USB0CDMA: usize = 13;
const DA8XX_MSTPRI_UHPI: usize = 14;
const DA8XX_MSTPRI_USB1: usize = 15;
const DA8XX_MSTPRI_LCDC: usize = 16;

#[repr(C)]
struct Da8xxMstpriDescr {
    reg: i32,
    shift: i32,
    mask: i32,
}

static DA8XX_MSTPRI_PRIORITY_LIST: [Da8xxMstpriDescr; 17] = [
    Da8xxMstpriDescr { reg: 0, shift: 0, mask: 0x0000000f },
    Da8xxMstpriDescr { reg: 0, shift: 4, mask: 0x000000f0 },
    Da8xxMstpriDescr { reg: 0, shift: 16, mask: 0x000f0000 },
    Da8xxMstpriDescr { reg: 0, shift: 20, mask: 0x00f00000 },
    Da8xxMstpriDescr { reg: 4, shift: 0, mask: 0x0000000f },
    Da8xxMstpriDescr { reg: 4, shift: 4, mask: 0x000000f0 },
    Da8xxMstpriDescr { reg: 4, shift: 8, mask: 0x00000f00 },
    Da8xxMstpriDescr { reg: 4, shift: 12, mask: 0x0000f000 },
    Da8xxMstpriDescr { reg: 4, shift: 16, mask: 0x000f0000 },
    Da8xxMstpriDescr { reg: 4, shift: 24, mask: 0x0f000000 },
    Da8xxMstpriDescr { reg: 4, shift: 28, mask: 0xf0000000u32 as i32 },
    Da8xxMstpriDescr { reg: 8, shift: 0, mask: 0x0000000f },
    Da8xxMstpriDescr { reg: 8, shift: 8, mask: 0x00000f00 },
    Da8xxMstpriDescr { reg: 8, shift: 12, mask: 0x0000f000 },
    Da8xxMstpriDescr { reg: 8, shift: 20, mask: 0x00f00000 },
    Da8xxMstpriDescr { reg: 8, shift: 24, mask: 0x0f000000 },
    Da8xxMstpriDescr { reg: 8, shift: 28, mask: 0xf0000000u32 as i32 },
];

#[repr(C)]
struct Da8xxMstpriPriority {
    which: usize,
    val: u32,
}

#[repr(C)]
struct Da8xxMstpriBoardPriorities {
    board: *const i8,
    priorities: *const Da8xxMstpriPriority,
    numprio: usize,
}

/*
 * Default memory settings of da850 do not meet the throughput/latency
 * requirements of tilcdc. This results in the image displayed being
 * incorrect and the following warning being displayed by the LCDC
 * drm driver:
 *
 *   tilcdc da8xx_lcdc.0: tilcdc_crtc_irq(0x00000020): FIFO underfow
 */
static DA850_LCDK_PRIORITIES: [Da8xxMstpriPriority; 3] = [
    Da8xxMstpriPriority { which: DA8XX_MSTPRI_LCDC, val: 0 },
    Da8xxMstpriPriority { which: DA8XX_MSTPRI_EDMA30TC1, val: 0 },
    Da8xxMstpriPriority { which: DA8XX_MSTPRI_EDMA30TC0, val: 1 },
];

static DA8XX_MSTPRI_BOARD_CONFS: [Da8xxMstpriBoardPriorities; 1] = [
    Da8xxMstpriBoardPriorities {
        board: b"ti,da850-lcdk\0" as *const [u8] as *const i8,
        priorities: DA850_LCDK_PRIORITIES.as_ptr(),
        numprio: DA850_LCDK_PRIORITIES.len(),
    },
];

extern "C" {
    fn of_machine_is_compatible(compat: *const i8) -> bool;
}

unsafe fn da8xx_mstpri_get_board_prio() -> *const Da8xxMstpriBoardPriorities {
    let mut i = 0usize;
    while i < DA8XX_MSTPRI_BOARD_CONFS.len() {
        let board_prio = &DA8XX_MSTPRI_BOARD_CONFS[i];
        if of_machine_is_compatible(board_prio.board) {
            return board_prio as *const _;
        }
        i += 1;
    }
    core::ptr::null()
}

// The kernel platform-device/resource APIs and logging helpers are supplied externally.
extern "C" {
    fn platform_get_resource(pdev: *mut core::ffi::c_void, typ: i32, num: i32) -> *mut core::ffi::c_void;
    fn devm_ioremap_resource(dev: *mut core::ffi::c_void, res: *mut core::ffi::c_void) -> *mut u8;
    fn resource_size(res: *mut core::ffi::c_void) -> usize;
    fn readl(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn dev_err(dev: *mut core::ffi::c_void, message: *const i8);
    fn dev_warn(dev: *mut core::ffi::c_void, message: *const i8);
    fn ptr_err(ptr: *mut u8) -> i32;
}

unsafe fn da8xx_mstpri_probe(pdev: *mut core::ffi::c_void) -> i32 {
    let dev = pdev;
    let res = platform_get_resource(pdev, 0, 0); // IORESOURCE_MEM
    let mstpri = devm_ioremap_resource(dev, res);
    if mstpri as isize < 0 {
        dev_err(dev, b"unable to map MSTPRI registers\n\0" as *const [u8] as *const i8);
        return ptr_err(mstpri);
    }

    let prio_list = da8xx_mstpri_get_board_prio();
    if prio_list.is_null() {
        dev_err(dev, b"no master priorities defined for this board\n\0" as *const [u8] as *const i8);
        return -22; // -EINVAL
    }

    let list = &*prio_list;
    let mut i = 0usize;
    while i < list.numprio {
        let prio = &*list.priorities.add(i);
        let prio_descr = &DA8XX_MSTPRI_PRIORITY_LIST[prio.which];

        if prio_descr.reg as usize + core::mem::size_of::<u32>() > resource_size(res) {
            dev_warn(dev, b"register offset out of range\n\0" as *const [u8] as *const i8);
            i += 1;
            continue;
        }

        let addr = mstpri.add(prio_descr.reg as usize);
        let mut reg = readl(addr);
        reg &= !(prio_descr.mask as u32);
        reg |= prio.val << prio_descr.shift;
        writel(reg, addr);
        i += 1;
    }
    0
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const i8,
}

static DA8XX_MSTPRI_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"ti,da850-mstpri\0" as *const [u8] as *const i8 },
    OfDeviceId { compatible: core::ptr::null() },
];

// struct platform_driver da8xx_mstpri_driver and module_platform_driver are kernel declarations/macros.
// MODULE_AUTHOR, MODULE_DESCRIPTION, and MODULE_LICENSE metadata:
// "Bartosz Golaszewski <bgolaszewski@baylibre.com>",
// "TI da8xx master peripheral priority driver", "GPL v2".

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
