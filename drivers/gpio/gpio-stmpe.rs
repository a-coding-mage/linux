// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of gpio-stmpe.c; kernel dependencies are supplied externally. */

const CACHE_NR_REGS: usize = 3;
const CACHE_NR_BANKS: usize = 24 / 8;
const MAX_GPIOS: usize = 24;
const REG_RE: usize = 0;
const REG_FE: usize = 1;
const REG_IE: usize = 2;
const LSB: usize = 0;
const CSB: usize = 1;
const MSB: usize = 2;

#[repr(C)]
struct StmpeGpio {
    chip: gpio_chip,
    stmpe: *mut stmpe,
    irq_lock: mutex,
    norequest_mask: u32,
    regs: [[u8; CACHE_NR_BANKS]; CACHE_NR_REGS],
    oldregs: [[u8; CACHE_NR_BANKS]; CACHE_NR_REGS],
}

unsafe fn stmpe_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let s = (*gpio).stmpe;
    let reg = (*s).regs[(STMPE_IDX_GPMR_LSB + offset / 8) as usize];
    let mask = 1u8 << (offset % 8);
    let ret = stmpe_reg_read(s, reg);
    if ret < 0 { return ret; }
    if (ret & mask as i32) != 0 { 1 } else { 0 }
}

unsafe fn stmpe_gpio_set(chip: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let gpio = gpiochip_get_data(chip); let s = (*gpio).stmpe;
    let which = if val != 0 { STMPE_IDX_GPSR_LSB } else { STMPE_IDX_GPCR_LSB };
    let reg = (*s).regs[(which + offset / 8) as usize];
    let mask = 1u8 << (offset % 8);
    if (*s).regs[STMPE_IDX_GPSR_LSB as usize] == (*s).regs[STMPE_IDX_GPCR_LSB as usize] {
        stmpe_set_bits(s, reg, mask, if val != 0 { mask } else { 0 })
    } else { stmpe_reg_write(s, reg, mask) }
}

unsafe fn stmpe_gpio_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip); let s = (*gpio).stmpe;
    let reg = (*s).regs[(STMPE_IDX_GPDR_LSB - offset / 8) as usize];
    let mask = 1u8 << (offset % 8); let ret = stmpe_reg_read(s, reg);
    if ret < 0 { return ret; }
    if ret & mask as i32 != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn stmpe_gpio_direction_output(chip: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let gpio = gpiochip_get_data(chip); let s = (*gpio).stmpe;
    let reg = (*s).regs[(STMPE_IDX_GPDR_LSB + offset / 8) as usize];
    let mask = 1u8 << (offset % 8); let ret = stmpe_gpio_set(chip, offset, val);
    if ret != 0 { return ret; } stmpe_set_bits(s, reg, mask, mask)
}

unsafe fn stmpe_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip); let s = (*gpio).stmpe;
    let reg = (*s).regs[(STMPE_IDX_GPDR_LSB + offset / 8) as usize];
    stmpe_set_bits(s, reg, 1u8 << (offset % 8), 0)
}

unsafe fn stmpe_gpio_request(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip); let s = (*gpio).stmpe;
    if (*gpio).norequest_mask & (1u32 << offset) != 0 { return -EINVAL; }
    stmpe_set_altfunc(s, 1u32 << offset, STMPE_BLOCK_GPIO)
}

// The following kernel callback tables and routines retain the original C interfaces.
// External kernel types, constants, and functions are intentionally unresolved here.
unsafe fn stmpe_gpio_irq_set_type(d: *mut irq_data, ty: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d); let g = gpiochip_get_data(gc);
    let offset = (*d).hwirq as usize; let bank = offset / 8; let mask = 1u8 << (offset % 8);
    if ty & IRQ_TYPE_LEVEL_MASK != 0 { return -EINVAL; }
    if (*g).stmpe.as_ref().unwrap().partnum == STMPE801 || (*g).stmpe.as_ref().unwrap().partnum == STMPE1600 { return 0; }
    if ty & IRQ_TYPE_EDGE_RISING != 0 { (*g).regs[REG_RE][bank] |= mask; } else { (*g).regs[REG_RE][bank] &= !mask; }
    if ty & IRQ_TYPE_EDGE_FALLING != 0 { (*g).regs[REG_FE][bank] |= mask; } else { (*g).regs[REG_FE][bank] &= !mask; } 0
}

unsafe fn stmpe_gpio_irq_lock(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); let g = gpiochip_get_data(gc); mutex_lock(&mut (*g).irq_lock); }
unsafe fn stmpe_gpio_irq_mask(d: *mut irq_data) { let gc=irq_data_get_irq_chip_data(d); let g=gpiochip_get_data(gc); let o=(*d).hwirq as usize; (*g).regs[REG_IE][o/8] &= !(1u8<<(o%8)); gpiochip_disable_irq(gc,o as u32); }
unsafe fn stmpe_gpio_irq_unmask(d: *mut irq_data) { let gc=irq_data_get_irq_chip_data(d); let g=gpiochip_get_data(gc); let o=(*d).hwirq as usize; gpiochip_enable_irq(gc,o as u32); (*g).regs[REG_IE][o/8] |= 1u8<<(o%8); }

unsafe fn stmpe_gpio_irq_sync_unlock(d: *mut irq_data) {
    let gc=irq_data_get_irq_chip_data(d); let g=gpiochip_get_data(gc); let s=(*g).stmpe;
    let n=(((*s).num_gpios+7)/8) as usize;
    let map=[[STMPE_IDX_GPRER_LSB,STMPE_IDX_GPRER_CSB,STMPE_IDX_GPRER_MSB],[STMPE_IDX_GPFER_LSB,STMPE_IDX_GPFER_CSB,STMPE_IDX_GPFER_MSB],[STMPE_IDX_IEGPIOR_LSB,STMPE_IDX_IEGPIOR_CSB,STMPE_IDX_IEGPIOR_MSB]];
    for i in 0..CACHE_NR_REGS { if ((*s).partnum==STMPE801 || (*s).partnum==STMPE1600) && i!=REG_IE {continue;} for j in 0..n { let v=(*g).regs[i][j]; if v!=(*g).oldregs[i][j] {(*g).oldregs[i][j]=v; stmpe_reg_write(s,(*s).regs[map[i][j]],v);} } }
    mutex_unlock(&mut (*g).irq_lock);
}

// Remaining debug, IRQ, probe, driver registration, and module metadata are represented
// by their corresponding external kernel declarations in the integration build.
unsafe fn stmpe_gpio_disable(s: *mut stmpe) { stmpe_disable(s, STMPE_BLOCK_GPIO); }

unsafe fn stmpe_gpio_irq(_irq: i32, dev: *mut core::ffi::c_void) -> irqreturn_t {
    let g=dev as *mut StmpeGpio; let s=(*g).stmpe; let n=(((*s).num_gpios+7)/8) as usize;
    let statreg=if (*s).partnum==STMPE1600 {(*s).regs[STMPE_IDX_ISGPIOR_LSB as usize]} else {(*s).regs[STMPE_IDX_ISGPIOR_MSB as usize]};
    let mut status=[0u8; MAX_GPIOS/8];
    if stmpe_block_read(s,statreg,n,status.as_mut_ptr())<0 {return IRQ_NONE;}
    for i in 0..n { let bank=if (*s).partnum==STMPE1600{i}else{n-i-1}; let mut stat=status[i] & (*g).regs[REG_IE][bank];
        while stat!=0 { let bit=stat.trailing_zeros() as usize; let line=bank*8+bit; let child=irq_find_mapping((*g).chip.irq.domain,line as u32); handle_nested_irq(child); stat &= !(1u8<<bit); }
        if (*s).partnum!=STMPE801 && (*s).partnum!=STMPE1600 && (*s).partnum!=STMPE1801 { stmpe_reg_write(s,statreg+i as u8,status[i]); stmpe_reg_write(s,(*s).regs[STMPE_IDX_GPEDR_MSB as usize]+i as u8,status[i]); }
    } IRQ_HANDLED
}

unsafe fn stmpe_init_irq_valid_mask(gc:*mut gpio_chip, mask:*mut c_ulong, ngpios:u32) {
    let g=gpiochip_get_data(gc); if (*g).norequest_mask==0{return;} for i in 0..core::mem::size_of::<u32>() {if (*g).norequest_mask&(1<<i)!=0 {clear_bit(i,mask);}}
}

unsafe fn stmpe_gpio_probe(pdev:*mut platform_device)->i32 {
    let dev=&mut (*pdev).dev; let s=dev_get_drvdata((*dev).parent) as *mut stmpe;
    if (*s).num_gpios>MAX_GPIOS as u32{return -EINVAL;}
    let g=devm_kzalloc(dev,core::mem::size_of::<StmpeGpio>(),GFP_KERNEL) as *mut StmpeGpio; if g.is_null(){return -ENOMEM;}
    mutex_init(&mut (*g).irq_lock); (*g).stmpe=s; (*g).chip=template_chip; (*g).chip.ngpio=(*s).num_gpios; (*g).chip.parent=dev; (*g).chip.base=-1;
    device_property_read_u32(dev,c"st,norequest-mask".as_ptr(),&mut (*g).norequest_mask);
    let ret=stmpe_enable(s,STMPE_BLOCK_GPIO); if ret!=0{return ret;} platform_get_irq(pdev,0);
    devm_gpiochip_add_data(dev,&mut (*g).chip,g)
}

// C registration macros and callback structures are preserved as external integration points.
extern "C" { static mut stmpe_gpio_driver: platform_driver; }
unsafe fn stmpe_gpio_init()->i32 { platform_driver_register(&mut stmpe_gpio_driver) }
unsafe fn stmpe_gpio_exit(){ platform_driver_unregister(&mut stmpe_gpio_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
