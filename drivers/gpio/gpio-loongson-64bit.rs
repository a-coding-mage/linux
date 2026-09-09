// SPDX-License-Identifier: GPL-2.0+
/* Loongson GPIO Support */

// C kernel dependencies are supplied by the surrounding kernel Rust bindings.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum loongson_gpio_mode { BIT_CTRL_MODE, BYTE_CTRL_MODE }

#[repr(C)]
struct loongson_gpio_chip_data {
    label: *const core::ffi::c_char,
    mode: loongson_gpio_mode,
    conf_offset: u32, out_offset: u32, in_offset: u32, inten_offset: u32,
    intpol_offset: u32, intedge_offset: u32, intclr_offset: u32,
    intsts_offset: u32, intdual_offset: u32, intr_num: u32,
    irq_handler: Option<unsafe extern "C" fn(*mut irq_desc)>,
    girqchip: *const irq_chip,
}

#[repr(C)]
struct loongson_gpio_chip { chip: gpio_generic_chip, lock: spinlock_t,
    reg_base: *mut u8, chip_data: *const loongson_gpio_chip_data }

unsafe fn to_loongson_gpio_chip(chip: *mut gpio_chip) -> *mut loongson_gpio_chip {
    container_of(to_gpio_generic_chip(chip), core::ptr::addr_of_mut!((*core::ptr::null_mut::<loongson_gpio_chip>()).chip),)
}

unsafe fn loongson_commit_direction(lgpio: *mut loongson_gpio_chip, pin: u32, input: i32) {
    writeb(if input != 0 { 1 } else { 0 }, (*lgpio).reg_base.add((*(*lgpio).chip_data).conf_offset as usize + pin as usize));
}
unsafe fn loongson_commit_level(lgpio: *mut loongson_gpio_chip, pin: u32, high: i32) {
    writeb(if high != 0 { 1 } else { 0 }, (*lgpio).reg_base.add((*(*lgpio).chip_data).out_offset as usize + pin as usize));
}
unsafe extern "C" fn loongson_gpio_direction_input(chip: *mut gpio_chip, pin: u32) -> i32 {
    let lgpio = to_loongson_gpio_chip(chip); let mut flags = 0; spin_lock_irqsave(&mut (*lgpio).lock, &mut flags);
    loongson_commit_direction(lgpio, pin, 1); spin_unlock_irqrestore(&mut (*lgpio).lock, flags); 0
}
unsafe extern "C" fn loongson_gpio_direction_output(chip: *mut gpio_chip, pin: u32, value: i32) -> i32 {
    let lgpio = to_loongson_gpio_chip(chip); let mut flags = 0; spin_lock_irqsave(&mut (*lgpio).lock, &mut flags);
    loongson_commit_level(lgpio, pin, value); loongson_commit_direction(lgpio, pin, 0); spin_unlock_irqrestore(&mut (*lgpio).lock, flags); 0
}
unsafe extern "C" fn loongson_gpio_get(chip: *mut gpio_chip, pin: u32) -> i32 { let lgpio=to_loongson_gpio_chip(chip); (readb((*lgpio).reg_base.add((*(*lgpio).chip_data).in_offset as usize+pin as usize)) & 1) as i32 }
unsafe extern "C" fn loongson_gpio_get_direction(chip: *mut gpio_chip, pin: u32) -> i32 { let lgpio=to_loongson_gpio_chip(chip); if readb((*lgpio).reg_base.add((*(*lgpio).chip_data).conf_offset as usize+pin as usize)) & 1 != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT } }
unsafe extern "C" fn loongson_gpio_set(chip: *mut gpio_chip, pin: u32, value: i32) -> i32 { let lgpio=to_loongson_gpio_chip(chip); let mut flags=0; spin_lock_irqsave(&mut (*lgpio).lock,&mut flags); loongson_commit_level(lgpio,pin,value); spin_unlock_irqrestore(&mut (*lgpio).lock,flags); 0 }

unsafe extern "C" fn loongson_gpio_to_irq(chip: *mut gpio_chip, offset: u32) -> i32 {
    let pdev=to_platform_device((*chip).parent); let lgpio=to_loongson_gpio_chip(chip); let d=(*lgpio).chip_data;
    if (*d).mode == loongson_gpio_mode::BIT_CTRL_MODE { let p=(*lgpio).reg_base.add((*d).inten_offset as usize+(offset/32*4) as usize); let u=readl(p)| (1u32 << (offset%32)); writel(u,p); } else { writeb(1,(*lgpio).reg_base.add((*d).inten_offset as usize+offset as usize)); } platform_get_irq(pdev,offset)
}
unsafe extern "C" fn loongson_gpio_irq_ack(data:*mut irq_data){let c=irq_data_get_irq_chip_data(data);let l=to_loongson_gpio_chip(c);writeb(1,(*l).reg_base.add((*(*l).chip_data).intclr_offset as usize+irqd_to_hwirq(data) as usize));}
unsafe extern "C" fn loongson_gpio_irq_mask(data:*mut irq_data){let c=irq_data_get_irq_chip_data(data);let l=to_loongson_gpio_chip(c);writeb(0,(*l).reg_base.add((*(*l).chip_data).inten_offset as usize+irqd_to_hwirq(data) as usize));}
unsafe extern "C" fn loongson_gpio_irq_unmask(data:*mut irq_data){let c=irq_data_get_irq_chip_data(data);let l=to_loongson_gpio_chip(c);writeb(1,(*l).reg_base.add((*(*l).chip_data).inten_offset as usize+irqd_to_hwirq(data) as usize));}

const fn cstr(s: &[u8]) -> *const core::ffi::c_char { s.as_ptr() as *const _ }
static L2K:&[u8]=b"ls2k_gpio\0"; static L0300:&[u8]=b"ls2k0300_gpio\0";
static L0500:&[u8]=b"ls2k0500_gpio\0"; static L2000:&[u8]=b"ls2k2000_gpio\0";
static L3A5:&[u8]=b"ls3a5000_gpio\0"; static L7A:&[u8]=b"ls7a_gpio\0";
static L7A2:&[u8]=b"ls7a2000_gpio\0"; static L3A6:&[u8]=b"ls3a6000_gpio\0";

static loongson_gpio_ls2k_data: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L2K),mode:loongson_gpio_mode::BIT_CTRL_MODE,conf_offset:0,in_offset:0x20,out_offset:0x10,inten_offset:0x30,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls2k0300_data: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L0300),mode:loongson_gpio_mode::BYTE_CTRL_MODE,conf_offset:0x800,in_offset:0xa00,out_offset:0x900,inten_offset:0xb00,intpol_offset:0xc00,intedge_offset:0xd00,intclr_offset:0xe00,intsts_offset:0xf00,intdual_offset:0xf80,intr_num:7,irq_handler:Some(loongson_gpio_ls2k0300_irq_handler),girqchip:core::ptr::null(),..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls2k0500_data0: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L0500),mode:loongson_gpio_mode::BIT_CTRL_MODE,conf_offset:0,in_offset:8,out_offset:0x10,inten_offset:0xb0,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls2k0500_data1: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L0500),mode:loongson_gpio_mode::BIT_CTRL_MODE,conf_offset:0,in_offset:8,out_offset:0x10,inten_offset:0x98,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls2k2000_data0: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L2000),mode:loongson_gpio_mode::BIT_CTRL_MODE,conf_offset:0,in_offset:0xc,out_offset:8,inten_offset:0x14,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls2k2000_data1: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L2000),mode:loongson_gpio_mode::BYTE_CTRL_MODE,conf_offset:0x800,in_offset:0xa00,out_offset:0x900,inten_offset:0xb00,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls2k2000_data2: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L2000),mode:loongson_gpio_mode::BIT_CTRL_MODE,conf_offset:4,in_offset:8,out_offset:0,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls3a5000_data: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L3A5),mode:loongson_gpio_mode::BIT_CTRL_MODE,conf_offset:0,in_offset:0xc,out_offset:8,inten_offset:0x14,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls7a_data: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L7A),mode:loongson_gpio_mode::BYTE_CTRL_MODE,conf_offset:0x800,in_offset:0xa00,out_offset:0x900,inten_offset:0xb00,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls7a2000_data0: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L7A2),mode:loongson_gpio_mode::BYTE_CTRL_MODE,conf_offset:0x800,in_offset:0xa00,out_offset:0x900,inten_offset:0xb00,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls7a2000_data1: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L7A2),mode:loongson_gpio_mode::BIT_CTRL_MODE,conf_offset:4,in_offset:8,out_offset:0,..unsafe{core::mem::zeroed()}};
static loongson_gpio_ls3a6000_data: loongson_gpio_chip_data=loongson_gpio_chip_data{label:cstr(L3A6),mode:loongson_gpio_mode::BIT_CTRL_MODE,conf_offset:0,in_offset:0xc,out_offset:8,inten_offset:0x14,..unsafe{core::mem::zeroed()}};

unsafe extern "C" fn loongson_gpio_ls2k0300_irq_handler(_: *mut irq_desc) {}
unsafe extern "C" fn loongson_gpio_probe(_: *mut platform_device)->i32 { 0 }
unsafe extern "C" fn loongson_gpio_setup()->i32 { platform_driver_register(core::ptr::null_mut()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
