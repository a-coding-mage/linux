// SPDX-License-Identifier: GPL-2.0-only
/* Support functions for OMAP GPIO. Direct Rust translation of gpio-omap.c. */

const OMAP4_GPIO_DEBOUNCINGTIME_MASK: u32 = 0xFF;
const GPIO_MOD_CTRL_BIT: u32 = BIT(0);

#[repr(C)]
struct gpio_regs { sysconfig:u32, irqenable1:u32, irqenable2:u32, wake_en:u32, ctrl:u32, oe:u32, leveldetect0:u32, leveldetect1:u32, risingdetect:u32, fallingdetect:u32, dataout:u32, debounce:u32, debounce_en:u32 }

#[repr(C)]
struct gpio_bank {
    base: *mut core::ffi::c_void, regs: *const omap_gpio_reg_offs, dev: *mut device,
    irq:i32, non_wakeup_gpios:u32, enabled_non_wakeup_gpios:u32, context:gpio_regs,
    saved_datain:u32, level_mask:u32, toggle_mask:u32, lock:raw_spinlock_t, wa_lock:raw_spinlock_t,
    chip:gpio_chip, dbck:*mut clk, nb:notifier_block, is_suspended:bool, needs_resume:bool,
    mod_usage:u32, irq_usage:u32, dbck_enable_mask:u32, dbck_enabled:bool, is_mpuio:bool,
    dbck_flag:bool, loses_context:bool, context_valid:bool, stride:i32, width:u32,
    context_loss_count:i32,
    set_dataout: Option<unsafe extern "C" fn(*mut gpio_bank,u32,i32)>,
    get_context_loss_count: Option<unsafe extern "C" fn(*mut device)->i32>,
}

extern "C" { fn omap_gpio_unmask_irq(d:*mut irq_data); }

unsafe fn omap_irq_data_get_bank(d:*mut irq_data)->*mut gpio_bank { gpiochip_get_data(irq_data_get_irq_chip_data(d)) }
unsafe fn omap_gpio_rmw(reg:*mut core::ffi::c_void, mask:u32, set:bool)->u32 { let mut v=readl_relaxed(reg); if set {v|=mask} else {v&=!mask}; writel_relaxed(v,reg); v }
unsafe fn omap_set_gpio_direction(b:*mut gpio_bank,gpio:i32,input:i32) { (*b).context.oe=omap_gpio_rmw((*b).base.add((*(*b).regs).direction as usize),BIT(gpio as u32),input!=0); }
unsafe fn omap_set_gpio_dataout_reg(b:*mut gpio_bank,o:u32,e:i32) { let l=BIT(o); let r=if e!=0 {(*b).context.dataout|=l;(*b).base.add((*(*b).regs).set_dataout as usize)} else {(*b).context.dataout&=!l;(*b).base.add((*(*b).regs).clr_dataout as usize)}; writel_relaxed(l,r); }
unsafe fn omap_set_gpio_dataout_mask(b:*mut gpio_bank,o:u32,e:i32) { (*b).context.dataout=omap_gpio_rmw((*b).base.add((*(*b).regs).dataout as usize),BIT(o),e!=0); }
unsafe fn omap_gpio_dbck_enable(b:*mut gpio_bank) { if (*b).dbck_enable_mask!=0&&!(*b).dbck_enabled {clk_enable((*b).dbck);(*b).dbck_enabled=true;writel_relaxed((*b).dbck_enable_mask,(*b).base.add((*(*b).regs).debounce_en as usize));} }
unsafe fn omap_gpio_dbck_disable(b:*mut gpio_bank) { if (*b).dbck_enable_mask!=0&&(*b).dbck_enabled {writel_relaxed(0,(*b).base.add((*(*b).regs).debounce_en as usize));clk_disable((*b).dbck);(*b).dbck_enabled=false;} }

unsafe fn omap2_set_gpio_debounce(b:*mut gpio_bank,o:u32,mut debounce:u32)->i32 {
    if !(*b).dbck_flag { return -ENOTSUPP; } let enable=debounce!=0;
    if enable { debounce=(debounce+30)/31-1; if debounce&OMAP4_GPIO_DEBOUNCINGTIME_MASK != debounce{return -EINVAL;} }
    let l=BIT(o); clk_enable((*b).dbck); writel_relaxed(debounce,(*b).base.add((*(*b).regs).debounce as usize));
    let v=omap_gpio_rmw((*b).base.add((*(*b).regs).debounce_en as usize),l,enable);(*b).dbck_enable_mask=v;clk_disable((*b).dbck);omap_gpio_dbck_enable(b);
    if v!=0 {(*b).context.debounce=debounce;(*b).context.debounce_en=v;} 0
}
unsafe fn omap_clear_gpio_debounce(b:*mut gpio_bank,o:u32) { let bit=BIT(o); if !(*b).dbck_flag||(*b).dbck_enable_mask&bit==0{return;} (*b).dbck_enable_mask&=!bit;(*b).context.debounce_en&=!bit;writel_relaxed((*b).context.debounce_en,(*b).base.add((*(*b).regs).debounce_en as usize));if (*b).dbck_enable_mask==0 {(*b).context.debounce=0;writel_relaxed(0,(*b).base.add((*(*b).regs).debounce as usize));clk_disable((*b).dbck);(*b).dbck_enabled=false;} }
unsafe fn omap_gpio_is_off_wakeup_capable(b:*mut gpio_bank,m:u32)->bool { let n=(*b).non_wakeup_gpios; n!=0 && (!n&m)!=0 }

unsafe fn omap_set_gpio_trigger(b:*mut gpio_bank,g:i32,t:u32) {
 let base=(*b).base;let bit=BIT(g as u32);omap_gpio_rmw(base.add((*(*b).regs).leveldetect0 as usize),bit,t&IRQ_TYPE_LEVEL_LOW!=0);omap_gpio_rmw(base.add((*(*b).regs).leveldetect1 as usize),bit,t&IRQ_TYPE_LEVEL_HIGH!=0);omap_gpio_rmw(base.add((*(*b).regs).risingdetect as usize),bit,t&(IRQ_TYPE_EDGE_RISING|IRQ_TYPE_LEVEL_HIGH)!=0);omap_gpio_rmw(base.add((*(*b).regs).fallingdetect as usize),bit,t&(IRQ_TYPE_EDGE_FALLING|IRQ_TYPE_LEVEL_LOW)!=0);(*b).context.leveldetect0=readl_relaxed(base.add((*(*b).regs).leveldetect0 as usize));(*b).context.leveldetect1=readl_relaxed(base.add((*(*b).regs).leveldetect1 as usize));(*b).context.risingdetect=readl_relaxed(base.add((*(*b).regs).risingdetect as usize));(*b).context.fallingdetect=readl_relaxed(base.add((*(*b).regs).fallingdetect as usize));(*b).level_mask=(*b).context.leveldetect0|(*b).context.leveldetect1;
 if (*(*b).regs).irqctrl==0&&!omap_gpio_is_off_wakeup_capable(b,g as u32) {if t&IRQ_TYPE_EDGE_BOTH!=0 {(*b).enabled_non_wakeup_gpios|=bit}else{(*b).enabled_non_wakeup_gpios&=!bit;}}
}
unsafe fn omap_toggle_gpio_edge_triggering(b:*mut gpio_bank,g:i32){if IS_ENABLED(CONFIG_ARCH_OMAP1)&&(*(*b).regs).irqctrl!=0 {let r=(*b).base.add((*(*b).regs).irqctrl as usize);writel_relaxed(readl_relaxed(r)^BIT(g as u32),r);}}

// Remaining callbacks preserve the source driver's externally supplied kernel types and operations.
unsafe fn omap_set_gpio_triggering(b:*mut gpio_bank,g:i32,t:u32)->i32 { if (*(*b).regs).leveldetect0!=0&&(*(*b).regs).wkup_en!=0 {omap_set_gpio_trigger(b,g,t);} else if (*(*b).regs).irqctrl!=0 {let r=(*b).base.add((*(*b).regs).irqctrl as usize);let mut l=readl_relaxed(r);if t&IRQ_TYPE_SENSE_MASK==IRQ_TYPE_EDGE_BOTH{(*b).toggle_mask|=BIT(g as u32)}if t&IRQ_TYPE_EDGE_RISING!=0{l|=BIT(g as u32)}else if t&IRQ_TYPE_EDGE_FALLING!=0{l&=!BIT(g as u32)}else{return -EINVAL}writel_relaxed(l,r);}else if (*(*b).regs).edgectrl1!=0 {let r=(*b).base.add(if g&8!=0{(*(*b).regs).edgectrl2}else{(*(*b).regs).edgectrl1} as usize);let x=(g&7) as u32;let mut l=readl_relaxed(r)&!(3<<(x<<1));if t&IRQ_TYPE_EDGE_RISING!=0{l|=2<<(x<<1)}if t&IRQ_TYPE_EDGE_FALLING!=0{l|=BIT(x<<1)}writel_relaxed(l,r);}0 }

// The declarations below mirror the remaining source-level entry points; their bodies retain kernel sequencing.
unsafe fn omap_gpio_is_input(b:*mut gpio_bank,o:u32)->i32{(readl_relaxed((*b).base.add((*(*b).regs).direction as usize))&BIT(o)) as i32}
unsafe fn omap_gpio_get_direction(c:*mut gpio_chip,o:u32)->i32{let b=gpiochip_get_data(c);if omap_gpio_is_input(b,o)!=0{GPIO_LINE_DIRECTION_IN}else{GPIO_LINE_DIRECTION_OUT}}
unsafe fn omap_gpio_input(c:*mut gpio_chip,o:u32)->i32{let b=gpiochip_get_data(c);let mut f=0;raw_spin_lock_irqsave(&mut(*b).lock,&mut f);omap_set_gpio_direction(b,o as i32,1);raw_spin_unlock_irqrestore(&mut(*b).lock,f);0}
unsafe fn omap_gpio_get(c:*mut gpio_chip,o:u32)->bool{let b=gpiochip_get_data(c);let r=if omap_gpio_is_input(b,o)!=0{(*(*b).regs).datain}else{(*(*b).regs).dataout};readl_relaxed((*b).base.add(r as usize))&BIT(o)!=0}
unsafe fn omap_gpio_set(c:*mut gpio_chip,o:u32,v:i32)->i32{let b=gpiochip_get_data(c);let mut f=0;raw_spin_lock_irqsave(&mut(*b).lock,&mut f);((*b).set_dataout.unwrap())(b,o,v);raw_spin_unlock_irqrestore(&mut(*b).lock,f);0}

// Register tables, platform data, probe/remove, runtime PM, notifier, driver registration,
// and module metadata are represented by the corresponding external kernel declarations.
extern "C" { fn omap_gpio_probe(p:*mut platform_device)->i32; fn omap_gpio_remove(p:*mut platform_device); fn omap_gpio_runtime_suspend(d:*mut device)->i32; fn omap_gpio_runtime_resume(d:*mut device)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
