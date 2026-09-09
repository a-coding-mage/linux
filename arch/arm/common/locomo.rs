// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of linux/arch/arm/common/locomo.c.

// C kernel dependencies are supplied by the surrounding translation unit.

const IRQ_LOCOMO_KEY: i32 = 0;
const IRQ_LOCOMO_GPIO: i32 = 1;
const IRQ_LOCOMO_LT: i32 = 2;
const IRQ_LOCOMO_SPI: i32 = 3;
const M62332_EVR_CH: i32 = 1;
const M62332_SLAVE_ADDR: u8 = 0x4e;
const M62332_W_BIT: u8 = 0;
const M62332_SUB_ADDR: u8 = 0;
const M62332_A_BIT: u8 = 0;
const DAC_BUS_FREE_TIME: u32 = 5;
const DAC_START_SETUP_TIME: u32 = 5;
const DAC_STOP_SETUP_TIME: u32 = 4;
const DAC_START_HOLD_TIME: u32 = 5;
const DAC_SCL_LOW_HOLD_TIME: u32 = 5;
const DAC_SCL_HIGH_HOLD_TIME: u32 = 4;
const DAC_DATA_SETUP_TIME: u32 = 1;
const DAC_DATA_HOLD_TIME: u32 = 1;
const DAC_LOW_SETUP_TIME: u32 = 1;
const DAC_HIGH_SETUP_TIME: u32 = 1;

#[repr(C)]
struct locomo { dev: *mut device, phys: c_ulong, irq: c_uint, irq_base: c_int, lock: spinlock_t, base: *mut c_void, #[cfg(CONFIG_PM)] saved_state: *mut c_void }

#[repr(C)]
struct locomo_dev_info { offset: c_ulong, length: c_ulong, devid: c_uint, irq: [c_uint; 1], name: *const c_char }

static mut locomo_devices: [locomo_dev_info; 7] = [
    locomo_dev_info { devid: LOCOMO_DEVID_KEYBOARD, irq: [IRQ_LOCOMO_KEY as c_uint], name: b"locomo-keyboard\0".as_ptr() as _, offset: LOCOMO_KEYBOARD, length: 16 },
    locomo_dev_info { devid: LOCOMO_DEVID_FRONTLIGHT, irq: [0], name: b"locomo-frontlight\0".as_ptr() as _, offset: LOCOMO_FRONTLIGHT, length: 8 },
    locomo_dev_info { devid: LOCOMO_DEVID_BACKLIGHT, irq: [0], name: b"locomo-backlight\0".as_ptr() as _, offset: LOCOMO_BACKLIGHT, length: 8 },
    locomo_dev_info { devid: LOCOMO_DEVID_AUDIO, irq: [0], name: b"locomo-audio\0".as_ptr() as _, offset: LOCOMO_AUDIO, length: 4 },
    locomo_dev_info { devid: LOCOMO_DEVID_LED, irq: [0], name: b"locomo-led\0".as_ptr() as _, offset: LOCOMO_LED, length: 8 },
    locomo_dev_info { devid: LOCOMO_DEVID_UART, irq: [0], name: b"locomo-uart\0".as_ptr() as _, offset: 0, length: 0 },
    locomo_dev_info { devid: LOCOMO_DEVID_SPI, irq: [0], name: b"locomo-spi\0".as_ptr() as _, offset: LOCOMO_SPI, length: 0x30 },
];

unsafe fn locomo_handler(desc: *mut irq_desc) { let lchip = irq_desc_get_handler_data(desc) as *mut locomo; (*(*desc).irq_data.chip).irq_ack.unwrap()(&mut (*desc).irq_data); let req = locomo_readl((*lchip).base.add(LOCOMO_ICR as usize)) & 0x0f00; if req != 0 { let mut irq = (*lchip).irq_base; for i in 0..=3 { if req & (0x0100 << i) != 0 { generic_handle_irq(irq); } irq += 1; } } }
unsafe fn locomo_ack_irq(_d: *mut irq_data) {}
unsafe fn locomo_mask_irq(d: *mut irq_data) { let l = irq_data_get_irq_chip_data(d) as *mut locomo; let mut r = locomo_readl((*l).base.add(LOCOMO_ICR as usize)); r &= !(0x0010 << ((*d).irq - (*l).irq_base)); locomo_writel(r, (*l).base.add(LOCOMO_ICR as usize)); }
unsafe fn locomo_unmask_irq(d: *mut irq_data) { let l = irq_data_get_irq_chip_data(d) as *mut locomo; let mut r = locomo_readl((*l).base.add(LOCOMO_ICR as usize)); r |= 0x0010 << ((*d).irq - (*l).irq_base); locomo_writel(r, (*l).base.add(LOCOMO_ICR as usize)); }

unsafe fn locomo_setup_irq(l: *mut locomo) { irq_set_irq_type((*l).irq, IRQ_TYPE_EDGE_FALLING); irq_set_chained_handler_and_data((*l).irq, Some(locomo_handler), l as _); let mut irq = (*l).irq_base; while irq <= (*l).irq_base + 3 { irq_set_chip_and_handler(irq, &mut locomo_chip, handle_level_irq); irq_set_chip_data(irq, l as _); irq_clear_status_flags(irq, IRQ_NOREQUEST | IRQ_NOPROBE); irq += 1; } }
unsafe fn locomo_dev_release(dev: *mut device) { kfree(LOCOMO_DEV(dev) as _); }

unsafe fn locomo_init_one_child(l: *mut locomo, info: *mut locomo_dev_info) -> c_int {
    let dev = kzalloc_obj::<locomo_dev>(); if dev.is_null() { return -ENOMEM; }
    if !(*(*l).dev).dma_mask.is_null() { (*dev).dma_mask = *(*(*l).dev).dma_mask; (*dev).dev.dma_mask = &mut (*dev).dma_mask; }
    dev_set_name(&mut (*dev).dev, (*info).name); (*dev).devid = (*info).devid; (*dev).dev.parent = (*l).dev; (*dev).dev.bus = &locomo_bus_type; (*dev).dev.release = Some(locomo_dev_release); (*dev).dev.coherent_dma_mask = (*(*l).dev).coherent_dma_mask; (*dev).mapbase = if (*info).offset != 0 { (*l).base.add((*info).offset as usize) } else { core::ptr::null_mut() }; (*dev).length = (*info).length; (*dev).irq[0] = if (*l).irq_base == NO_IRQ { NO_IRQ } else { (*l).irq_base + (*info).irq[0] as i32 } as _;
    let ret = device_register(&mut (*dev).dev); if ret != 0 { kfree(dev as _); } ret
}

unsafe fn __locomo_probe(me: *mut device, mem: *mut resource, irq: c_int) -> c_int {
    let pdata = (*me).platform_data as *mut locomo_platform_data; let l = kzalloc_obj::<locomo>(); if l.is_null() { return -ENOMEM; } spin_lock_init(&mut (*l).lock); (*l).dev=me; dev_set_drvdata(me,l as _); (*l).phys=(*mem).start; (*l).irq=irq as _; (*l).irq_base=if !pdata.is_null(){(*pdata).irq_base}else{NO_IRQ}; (*l).base=ioremap((*mem).start,PAGE_SIZE); if (*l).base.is_null(){ kfree(l as _); return -ENOMEM; }
    locomo_writel(0,(*l).base.add(LOCOMO_ICR as usize)); locomo_writel(0,(*l).base.add((LOCOMO_KEYBOARD+LOCOMO_KIC) as usize)); locomo_writel(0,(*l).base.add(LOCOMO_GPO as usize)); let gp=LOCOMO_GPIO(1)|LOCOMO_GPIO(2)|LOCOMO_GPIO(13)|LOCOMO_GPIO(14); locomo_writel(gp,(*l).base.add(LOCOMO_GPE as usize)); locomo_writel(gp,(*l).base.add(LOCOMO_GPD as usize)); locomo_writel(0,(*l).base.add(LOCOMO_GIE as usize)); locomo_writel(0,(*l).base.add((LOCOMO_FRONTLIGHT+LOCOMO_ALS) as usize)); locomo_writel(0,(*l).base.add((LOCOMO_FRONTLIGHT+LOCOMO_ALD) as usize)); locomo_writel(0,(*l).base.add(LOCOMO_LTINT as usize)); locomo_writel(0,(*l).base.add((LOCOMO_SPI+LOCOMO_SPIIE) as usize)); locomo_writel(354,(*l).base.add(LOCOMO_ASD as usize)); let mut r=locomo_readl((*l).base.add(LOCOMO_ASD as usize))|0x8000; locomo_writel(r,(*l).base.add(LOCOMO_ASD as usize)); locomo_writel(230,(*l).base.add(LOCOMO_HSD as usize)); r=locomo_readl((*l).base.add(LOCOMO_HSD as usize))|0x8000; locomo_writel(r,(*l).base.add(LOCOMO_HSD as usize)); locomo_writel(16,(*l).base.add(LOCOMO_HSC as usize)); locomo_writel(0x80,(*l).base.add(LOCOMO_TADC as usize)); udelay(1000); r=locomo_readl((*l).base.add(LOCOMO_TADC as usize))|0x10; locomo_writel(r,(*l).base.add(LOCOMO_TADC as usize)); udelay(100); r=locomo_readl((*l).base.add(LOCOMO_DAC as usize))|LOCOMO_DAC_SCLOEB|LOCOMO_DAC_SDAOEB; locomo_writel(r,(*l).base.add(LOCOMO_DAC as usize)); r=locomo_readl((*l).base.add(LOCOMO_VER as usize)); printk(KERN_INFO,b"LoCoMo Chip: %lu%lu\n\0".as_ptr() as _,r>>8,r&0xff); if (*l).irq!=NO_IRQ as _ && (*l).irq_base!=NO_IRQ {locomo_setup_irq(l);} for i in 0..7 {locomo_init_one_child(l,&mut locomo_devices[i]);} 0
}

unsafe fn locomo_remove_child(dev:*mut device,_:*mut c_void)->c_int{device_unregister(dev);0}
unsafe fn __locomo_remove(l:*mut locomo){device_for_each_child((*l).dev,core::ptr::null_mut(),Some(locomo_remove_child));if (*l).irq!=NO_IRQ as _{irq_set_chained_handler_and_data((*l).irq,None,core::ptr::null_mut())}iounmap((*l).base);kfree(l as _);}
unsafe fn locomo_probe(dev:*mut platform_device)->c_int{let mem=platform_get_resource(dev,IORESOURCE_MEM,0);if mem.is_null(){return -EINVAL}let irq=platform_get_irq(dev,0);if irq<0{return -ENXIO}__locomo_probe(&mut (*dev).dev,mem,irq)}
unsafe fn locomo_remove(dev:*mut platform_device){let l=platform_get_drvdata(dev) as *mut locomo;if !l.is_null(){__locomo_remove(l);platform_set_drvdata(dev,core::ptr::null_mut());}}

unsafe fn locomo_chip_driver(ldev:*mut locomo_dev)->*mut locomo{dev_get_drvdata((*(*ldev).dev.parent).as_mut().unwrap()) as _}
pub unsafe fn locomo_gpio_set_dir(dev:*mut device,bits:c_uint,dir:c_uint){let l=dev_get_drvdata(dev) as *mut locomo;if l.is_null(){return}let mut r=locomo_readl((*l).base.add(LOCOMO_GPD as usize));r=if dir!=0{r|bits}else{r&!bits};locomo_writel(r,(*l).base.add(LOCOMO_GPD as usize));r=locomo_readl((*l).base.add(LOCOMO_GPE as usize));r=if dir!=0{r|bits}else{r&!bits};locomo_writel(r,(*l).base.add(LOCOMO_GPE as usize));}
pub unsafe fn locomo_gpio_read_level(dev:*mut device,bits:c_uint)->c_int{let l=dev_get_drvdata(dev)as*mut locomo;if l.is_null(){return -ENODEV} (locomo_readl((*l).base.add(LOCOMO_GPL as usize))&bits) as c_int}
pub unsafe fn locomo_gpio_read_output(dev:*mut device,bits:c_uint)->c_int{let l=dev_get_drvdata(dev)as*mut locomo;if l.is_null(){return -ENODEV}(locomo_readl((*l).base.add(LOCOMO_GPO as usize))&bits)as c_int}
pub unsafe fn locomo_gpio_write(dev:*mut device,bits:c_uint,set:c_uint){let l=dev_get_drvdata(dev)as*mut locomo;if l.is_null(){return}let mut r=locomo_readl((*l).base.add(LOCOMO_GPO as usize));r=if set!=0{r|bits}else{r&!bits};locomo_writel(r,(*l).base.add(LOCOMO_GPO as usize));}

unsafe fn locomo_m62332_sendbit(map:*mut c_void,bit:c_int){let mut r=locomo_readl(map.add(LOCOMO_DAC as usize))&!LOCOMO_DAC_SCLOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_LOW_SETUP_TIME);udelay(DAC_DATA_HOLD_TIME);r=locomo_readl(map.add(LOCOMO_DAC as usize))&!LOCOMO_DAC_SCLOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_LOW_SETUP_TIME);udelay(DAC_SCL_LOW_HOLD_TIME);r=locomo_readl(map.add(LOCOMO_DAC as usize));r=if bit&1!=0{r|LOCOMO_DAC_SDAOEB}else{r&!LOCOMO_DAC_SDAOEB};locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(if bit&1!=0{DAC_HIGH_SETUP_TIME}else{DAC_LOW_SETUP_TIME});udelay(DAC_DATA_SETUP_TIME);r=locomo_readl(map.add(LOCOMO_DAC as usize))|LOCOMO_DAC_SCLOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_HIGH_SETUP_TIME);udelay(DAC_SCL_HIGH_HOLD_TIME);}

pub unsafe fn locomo_m62332_senddata(ldev:*mut locomo_dev,dac_data:c_uint,channel:c_int){let l=locomo_chip_driver(ldev);let map=(*l).base;let mut r=locomo_readl(map.add(LOCOMO_DAC as usize))|LOCOMO_DAC_SCLOEB|LOCOMO_DAC_SDAOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_HIGH_SETUP_TIME);udelay(DAC_SCL_HIGH_HOLD_TIME);r=locomo_readl(map.add(LOCOMO_DAC as usize))&!LOCOMO_DAC_SDAOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_START_HOLD_TIME);udelay(DAC_DATA_HOLD_TIME);for i in 1..=8{locomo_m62332_sendbit(map,((M62332_SLAVE_ADDR<<1|M62332_W_BIT)>>(8-i))as c_int)}for data in [M62332_SUB_ADDR.wrapping_add(channel as u8),dac_data as u8]{for i in 1..=8{locomo_m62332_sendbit(map,(data>>(8-i))as c_int)}r=locomo_readl(map.add(LOCOMO_DAC as usize))&!LOCOMO_DAC_SCLOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_LOW_SETUP_TIME);udelay(DAC_SCL_LOW_HOLD_TIME);r=locomo_readl(map.add(LOCOMO_DAC as usize))&!LOCOMO_DAC_SDAOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_LOW_SETUP_TIME);r=locomo_readl(map.add(LOCOMO_DAC as usize))|LOCOMO_DAC_SCLOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_HIGH_SETUP_TIME);udelay(DAC_SCL_HIGH_HOLD_TIME);}r=locomo_readl(map.add(LOCOMO_DAC as usize))&!LOCOMO_DAC_SCLOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_LOW_SETUP_TIME);udelay(DAC_SCL_LOW_HOLD_TIME);r=locomo_readl(map.add(LOCOMO_DAC as usize))|LOCOMO_DAC_SCLOEB|LOCOMO_DAC_SDAOEB;locomo_writel(r,map.add(LOCOMO_DAC as usize));udelay(DAC_HIGH_SETUP_TIME);udelay(DAC_SCL_HIGH_HOLD_TIME);}

pub unsafe fn locomo_frontlight_set(dev:*mut locomo_dev,duty:c_int,vr:c_int,bpwf:c_int){locomo_gpio_write((*(*dev).dev).parent,LOCOMO_GPIO_FL_VR,if vr!=0{1}else{0});let l=locomo_chip_driver(dev);locomo_writel(bpwf as _,(*l).base.add((LOCOMO_FRONTLIGHT+LOCOMO_ALS)as usize));udelay(100);locomo_writel(duty as _,(*l).base.add((LOCOMO_FRONTLIGHT+LOCOMO_ALD)as usize));locomo_writel((bpwf as u32)|LOCOMO_ALC_EN,(*l).base.add((LOCOMO_FRONTLIGHT+LOCOMO_ALS)as usize));}

unsafe fn locomo_match(dev:*mut device,drv:*const device_driver)->c_int{( (*LOCOMO_DEV(dev)).devid==(*LOCOMO_DRV(drv)).devid)as c_int}
unsafe fn locomo_bus_probe(dev:*mut device)->c_int{let d=LOCOMO_DEV(dev);let drv=LOCOMO_DRV((*dev).driver);if !(*drv).probe.is_none(){return ((*drv).probe.unwrap())(d)}-ENODEV}
unsafe fn locomo_bus_remove(dev:*mut device){let d=LOCOMO_DEV(dev);let drv=LOCOMO_DRV((*dev).driver);if !(*drv).remove.is_none(){((*drv).remove.unwrap())(d);}}
pub unsafe fn locomo_driver_register(driver:*mut locomo_driver)->c_int{(*driver).drv.bus=&locomo_bus_type;driver_register(&mut (*driver).drv)}
pub unsafe fn locomo_driver_unregister(driver:*mut locomo_driver){driver_unregister(&mut (*driver).drv)}

// The following items retain the original driver's registration and bus layout.
static mut locomo_chip: irq_chip = irq_chip { name: b"LOCOMO\0".as_ptr() as _, irq_ack: Some(locomo_ack_irq), irq_mask: Some(locomo_mask_irq), irq_unmask: Some(locomo_unmask_irq) };
static mut locomo_bus_type: bus_type = bus_type { name: b"locomo-bus\0".as_ptr() as _, match_: Some(locomo_match), probe: Some(locomo_bus_probe), remove: Some(locomo_bus_remove) };

unsafe fn locomo_init() -> c_int { let ret=bus_register(&mut locomo_bus_type); if ret==0 { platform_driver_register(&mut locomo_device_driver) } else { ret } }
unsafe fn locomo_exit(){platform_driver_unregister(&mut locomo_device_driver);bus_unregister(&mut locomo_bus_type);}

static mut locomo_device_driver: platform_driver = platform_driver {
    probe: Some(locomo_probe), remove: Some(locomo_remove),
    #[cfg(CONFIG_PM)] suspend: Some(locomo_suspend),
    #[cfg(CONFIG_PM)] resume: Some(locomo_resume),
    driver: driver { name: b"locomo\0".as_ptr() as _, ..driver_zeroed() },
};

#[cfg(CONFIG_PM)]
#[repr(C)] struct locomo_save_data { LCM_GPO:u16, LCM_SPICT:u16, LCM_GPE:u16, LCM_ASD:u16, LCM_SPIMD:u16 }
#[cfg(CONFIG_PM)]
unsafe fn locomo_suspend(dev:*mut platform_device,_state:pm_message_t)->c_int { let l=platform_get_drvdata(dev)as*mut locomo;let save=kmalloc_obj::<locomo_save_data>();if save.is_null(){return -ENOMEM}(*l).saved_state=save;(*save).LCM_GPO=locomo_readl((*l).base.add(LOCOMO_GPO as usize))as _;locomo_writel(0,(*l).base.add(LOCOMO_GPO as usize));(*save).LCM_SPICT=locomo_readl((*l).base.add((LOCOMO_SPI+LOCOMO_SPICT)as usize))as _;locomo_writel(0x40,(*l).base.add((LOCOMO_SPI+LOCOMO_SPICT)as usize));(*save).LCM_GPE=locomo_readl((*l).base.add(LOCOMO_GPE as usize))as _;locomo_writel(0,(*l).base.add(LOCOMO_GPE as usize));(*save).LCM_ASD=locomo_readl((*l).base.add(LOCOMO_ASD as usize))as _;locomo_writel(0,(*l).base.add(LOCOMO_ASD as usize));(*save).LCM_SPIMD=locomo_readl((*l).base.add((LOCOMO_SPI+LOCOMO_SPIMD)as usize))as _;locomo_writel(0x3c14,(*l).base.add((LOCOMO_SPI+LOCOMO_SPIMD)as usize));locomo_writel(0,(*l).base.add(LOCOMO_PAIF as usize));locomo_writel(0,(*l).base.add(LOCOMO_DAC as usize));locomo_writel(0,(*l).base.add((LOCOMO_BACKLIGHT+LOCOMO_TC)as usize));locomo_writel(0,(*l).base.add(LOCOMO_TADC as usize));locomo_writel(0,(*l).base.add((LOCOMO_AUDIO+LOCOMO_ACC)as usize));locomo_writel(0,(*l).base.add((LOCOMO_FRONTLIGHT+LOCOMO_ALS)as usize));0 }
#[cfg(CONFIG_PM)]
unsafe fn locomo_resume(dev:*mut platform_device)->c_int { let l=platform_get_drvdata(dev)as*mut locomo;let s=(*l).saved_state as*mut locomo_save_data;if s.is_null(){return 0}locomo_writel((*s).LCM_GPO as _,(*l).base.add(LOCOMO_GPO as usize));locomo_writel((*s).LCM_SPICT as _,(*l).base.add((LOCOMO_SPI+LOCOMO_SPICT)as usize));locomo_writel((*s).LCM_GPE as _,(*l).base.add(LOCOMO_GPE as usize));locomo_writel((*s).LCM_ASD as _,(*l).base.add(LOCOMO_ASD as usize));locomo_writel((*s).LCM_SPIMD as _,(*l).base.add((LOCOMO_SPI+LOCOMO_SPIMD)as usize));locomo_writel(0,(*l).base.add(LOCOMO_C32K as usize));locomo_writel(0x90,(*l).base.add(LOCOMO_TADC as usize));locomo_writel(0,(*l).base.add((LOCOMO_KEYBOARD+LOCOMO_KSC)as usize));let r=locomo_readl((*l).base.add((LOCOMO_KEYBOARD+LOCOMO_KIC)as usize)&0xfeff;locomo_writel(r,(*l).base.add((LOCOMO_KEYBOARD+LOCOMO_KIC)as usize));locomo_writel(1,(*l).base.add((LOCOMO_KEYBOARD+LOCOMO_KCMD)as usize));(*l).saved_state=core::ptr::null_mut();kfree(s as _);0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
