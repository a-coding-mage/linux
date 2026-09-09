// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-mmp/common.c
 *
 *  Code common to PXA168 processor lines
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct map_desc {
    pub pfn: usize,
    pub virtual_: u64,
    pub length: usize,
    pub type_: u32,
}

// Build-time/kernel symbols supplied externally.
extern "C" {
    fn iotable_init(io_desc: *mut map_desc, nr: usize);
    fn __raw_readl(addr: usize) -> u32;
}

// The following constants/macros are supplied by addr-map.h and the kernel
// headers: CIU_REG, __phys_to_pfn, APB_PHYS_BASE, APB_VIRT_BASE,
// APB_PHYS_SIZE, AXI_PHYS_BASE, AXI_VIRT_BASE, AXI_PHYS_SIZE,
// PGU_PHYS_BASE, PGU_VIRT_BASE, PGU_PHYS_SIZE, and MT_DEVICE.

const MMP_CHIPID: usize = CIU_REG(0x00);

pub static mut mmp_chip_id: u32 = 0;

static mut standard_io_desc: [map_desc; 2] = [
    map_desc {
        pfn: __phys_to_pfn(APB_PHYS_BASE),
        virtual_: APB_VIRT_BASE as u64,
        length: APB_PHYS_SIZE,
        type_: MT_DEVICE,
    },
    map_desc {
        pfn: __phys_to_pfn(AXI_PHYS_BASE),
        virtual_: AXI_VIRT_BASE as u64,
        length: AXI_PHYS_SIZE,
        type_: MT_DEVICE,
    },
];

static mut mmp2_io_desc: [map_desc; 1] = [map_desc {
    pfn: __phys_to_pfn(PGU_PHYS_BASE),
    virtual_: PGU_VIRT_BASE as u64,
    length: PGU_PHYS_SIZE,
    type_: MT_DEVICE,
}];

pub unsafe fn mmp_map_io() {
    iotable_init(standard_io_desc.as_mut_ptr(), standard_io_desc.len());

    /* this is early, initialize mmp_chip_id here */
    mmp_chip_id = __raw_readl(MMP_CHIPID);
}

pub unsafe fn mmp2_map_io() {
    mmp_map_io();
    iotable_init(mmp2_io_desc.as_mut_ptr(), mmp2_io_desc.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
