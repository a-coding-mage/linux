// SPDX-License-Identifier: GPL-2.0
/*
 *  Nintendo 64 init.
 *
 *  Copyright (C) 2021\tLauri Kasanen
 */

// Linux and MIPS headers provide the types, constants, and functions referenced below.

const IO_MEM_RESOURCE_START: ::core::ffi::c_ulong = 0;
const IO_MEM_RESOURCE_END: ::core::ffi::c_ulong = 0x1fffffff;

/*
 * System-specifc irq names for clarity
 */
const MIPS_SOFTINT0_IRQ: ::core::ffi::c_int = MIPS_CPU_IRQ_BASE + 0;
const MIPS_SOFTINT1_IRQ: ::core::ffi::c_int = MIPS_CPU_IRQ_BASE + 1;
const RCP_IRQ: ::core::ffi::c_int = MIPS_CPU_IRQ_BASE + 2;
const CART_IRQ: ::core::ffi::c_int = MIPS_CPU_IRQ_BASE + 3;
const PRENMI_IRQ: ::core::ffi::c_int = MIPS_CPU_IRQ_BASE + 4;
const RDBR_IRQ: ::core::ffi::c_int = MIPS_CPU_IRQ_BASE + 5;
const RDBW_IRQ: ::core::ffi::c_int = MIPS_CPU_IRQ_BASE + 6;
const TIMER_IRQ: ::core::ffi::c_int = MIPS_CPU_IRQ_BASE + 7;

unsafe fn iomem_resource_init() {
    iomem_resource.start = IO_MEM_RESOURCE_START;
    iomem_resource.end = IO_MEM_RESOURCE_END;
}

#[no_mangle]
pub unsafe extern "C" fn get_system_type() -> *const ::core::ffi::c_char {
    b"Nintendo 64\0".as_ptr() as *const ::core::ffi::c_char
}

pub unsafe extern "C" fn prom_init() {
    fw_init_cmdline();
}

const W: usize = 320;
const H: usize = 240;

unsafe fn n64rdp_write_reg(reg: u8, value: u32) {
    let reg_base = CKSEG1ADDR(0x4400000) as *mut u32;
    ::core::ptr::write_volatile(reg_base.add(reg as usize), value);
}

static NTSC_320: [u32; 14] = [
    0x00013212, 0x00000000, 0x00000140, 0x00000200,
    0x00000000, 0x03e52239, 0x0000020d, 0x00000c15,
    0x0c150c15, 0x006c02ec, 0x002501ff, 0x000e0204,
    0x00000200, 0x00000400,
];

const MI_REG_BASE: usize = 0x4300000;
const NUM_MI_REGS: usize = 4;
const AI_REG_BASE: usize = 0x4500000;
const NUM_AI_REGS: usize = 6;
const PI_REG_BASE: usize = 0x4600000;
const NUM_PI_REGS: usize = 5;
const SI_REG_BASE: usize = 0x4800000;
const NUM_SI_REGS: usize = 7;

unsafe fn n64_platform_init() -> ::core::ffi::c_int {
    static SIMPLEFB_RESNAME: &[u8] = b"FB\0";
    let mode = simplefb_platform_data {
        width: W as _,
        height: H as _,
        stride: (W * 2) as _,
        format: b"r5g5b5a1\0".as_ptr() as *const ::core::ffi::c_char,
    };
    let mut res: [resource; 3] = ::core::mem::zeroed();

    res[0].flags = IORESOURCE_MEM;
    res[0].start = MI_REG_BASE as _;
    res[0].end = (MI_REG_BASE + NUM_MI_REGS * 4 - 1) as _;
    res[1].flags = IORESOURCE_MEM;
    res[1].start = AI_REG_BASE as _;
    res[1].end = (AI_REG_BASE + NUM_AI_REGS * 4 - 1) as _;
    res[2].flags = IORESOURCE_IRQ;
    res[2].start = RCP_IRQ as _;
    res[2].end = RCP_IRQ as _;
    platform_device_register_simple(b"n64audio\0".as_ptr() as _, -1, res.as_ptr(), 3);

    res[0] = ::core::mem::zeroed();
    res[0].flags = IORESOURCE_MEM;
    res[0].start = PI_REG_BASE as _;
    res[0].end = (PI_REG_BASE + NUM_PI_REGS * 4 - 1) as _;
    platform_device_register_simple(b"n64cart\0".as_ptr() as _, -1, res.as_ptr(), 1);

    res[0] = ::core::mem::zeroed();
    res[0].flags = IORESOURCE_MEM;
    res[0].start = SI_REG_BASE as _;
    res[0].end = (SI_REG_BASE + NUM_SI_REGS * 4 - 1) as _;
    platform_device_register_simple(b"n64joy\0".as_ptr() as _, -1, res.as_ptr(), 1);

    /* The framebuffer needs 64-byte alignment */
    let orig = kzalloc(W * H * 2 + 63, GFP_DMA | GFP_KERNEL);
    if orig.is_null() {
        return -ENOMEM;
    }
    let mut phys = virt_to_phys(orig);
    phys = (phys + 63) & !63;

    let mut i = 0;
    while i < NTSC_320.len() {
        if i == 1 {
            n64rdp_write_reg(i as u8, phys as u32);
        } else {
            n64rdp_write_reg(i as u8, NTSC_320[i]);
        }
        i += 1;
    }

    /* setup IORESOURCE_MEM as framebuffer memory */
    res[0] = ::core::mem::zeroed();
    res[0].flags = IORESOURCE_MEM;
    res[0].name = SIMPLEFB_RESNAME.as_ptr() as *const ::core::ffi::c_char;
    res[0].start = phys;
    res[0].end = phys + (W * H * 2 - 1) as _;
    platform_device_register_resndata(::core::ptr::null_mut(), b"simple-framebuffer\0".as_ptr() as _, 0, res.as_ptr(), 1, &mode as *const _, ::core::mem::size_of_val(&mode));
    0
}

pub unsafe extern "C" fn plat_mem_setup() {
    iomem_resource_init();
    memblock_add(0x0, 8 * 1024 * 1024); /* Bootloader blocks the 4mb config */
}

pub unsafe extern "C" fn plat_time_init() {
    /* 93.75 MHz cpu, count register runs at half rate */
    mips_hpt_frequency = 93750000 / 2;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
