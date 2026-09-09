// SPDX-License-Identifier: GPL-2.0
// Rust translation of regmap-irq.c. Kernel types and APIs are supplied externally.

use core::ffi::c_void;

#[repr(C)] pub struct regmap_irq_chip_data {
    pub lock: mutex, pub lock_key: lock_class_key, pub irq_chip: irq_chip,
    pub map: *mut regmap, pub chip: *const regmap_irq_chip, pub irq_base: i32,
    pub domain: *mut irq_domain, pub irq: i32, pub wake_count: i32,
    pub status_reg_buf: *mut c_void, pub main_status_buf: *mut u32,
    pub status_buf: *mut u32, pub prev_status_buf: *mut u32,
    pub mask_buf: *mut u32, pub mask_buf_def: *mut u32, pub wake_buf: *mut u32,
    pub type_buf: *mut u32, pub type_buf_def: *mut u32,
    pub config_buf: *mut *mut u32, pub irq_reg_stride: u32,
    pub get_irq_reg: Option<unsafe extern "C" fn(*mut regmap_irq_chip_data,u32,i32)->u32>,
    pub clear_status: bool,
}

extern "C" { fn regmap_irq_get_irq_reg_linear(d:*mut regmap_irq_chip_data,b:u32,i:i32)->u32; }
unsafe fn irq_to_regmap_irq(d:*mut regmap_irq_chip_data, irq:i32)->*const regmap_irq { &(*(*d).chip).irqs.add(irq as usize) }
unsafe fn get_reg(d:*mut regmap_irq_chip_data,b:u32,i:i32)->u32 { ((*d).get_irq_reg.unwrap())(d,b,i) }

unsafe extern "C" fn regmap_irq_lock(data:*mut irq_data) { let d=irq_data_get_irq_chip_data(data); mutex_lock(&mut (*d).lock); }
unsafe extern "C" fn regmap_irq_sync_unlock(data:*mut irq_data) {
    let d=irq_data_get_irq_chip_data(data); let map=(*d).map; let chip=&*(*d).chip; let mut ret;
    if chip.runtime_pm { ret=pm_runtime_get_sync((*map).dev); if ret<0 { dev_err((*map).dev,"IRQ sync failed to resume: %d\n",ret); } }
    if (*d).clear_status { for i in 0..chip.num_regs { let mut v=0; ret=regmap_read(map,get_reg(d,chip.status_base,i as i32),&mut v); if ret!=0 {dev_err((*map).dev,"Failed to clear the interrupt status bits\n");} } (*d).clear_status=false; }
    for i in 0..chip.num_regs { if let Some(f)=chip.handle_mask_sync { f(i,(*d).mask_buf_def.add(i as usize).read(),(*d).mask_buf.add(i as usize).read(),chip.irq_drv_data); }
        if chip.mask_base!=0 && chip.handle_mask_sync.is_none() { let r=get_reg(d,chip.mask_base,i as i32); ret=regmap_update_bits(map,r,(*d).mask_buf_def.add(i as usize).read(),(*d).mask_buf.add(i as usize).read()); if ret!=0 {dev_err((*map).dev,"Failed to sync masks in %x\n",r);} }
        if chip.unmask_base!=0 && chip.handle_mask_sync.is_none() { let r=get_reg(d,chip.unmask_base,i as i32); ret=regmap_update_bits(map,r,(*d).mask_buf_def.add(i as usize).read(),!(*d).mask_buf.add(i as usize).read()); if ret!=0 {dev_err((*map).dev,"Failed to sync masks in %x\n",r);} }
        let r=get_reg(d,chip.wake_base,i as i32); if !(*d).wake_buf.is_null() { let v=if chip.wake_invert {!(*d).wake_buf.add(i as usize).read()} else {(*d).wake_buf.add(i as usize).read()}; ret=regmap_update_bits(map,r,(*d).mask_buf_def.add(i as usize).read(),v); if ret!=0 {dev_err((*map).dev,"Failed to sync wakes in %x: %d\n",r,ret);} }
        if chip.init_ack_masked && (*d).mask_buf.add(i as usize).read()!=0 && (chip.ack_base!=0||chip.use_ack) { let r=get_reg(d,chip.ack_base,i as i32); ret=regmap_write(map,r,if chip.ack_invert {!(*d).mask_buf.add(i as usize).read()} else {(*d).mask_buf.add(i as usize).read()}); if chip.clear_ack && ret==0 {ret=regmap_write(map,r,if chip.ack_invert {u32::MAX} else {0});} if ret!=0 {dev_err((*map).dev,"Failed to ack 0x%x: %d\n",r,ret);} }
    }
    for i in 0..chip.num_config_bases { for j in 0..chip.num_config_regs { let r=get_reg(d,chip.config_base.add(i as usize).read(),j as i32); ret=regmap_write(map,r,(*d).config_buf.add(i as usize).read().add(j as usize).read()); if ret!=0 {dev_err((*map).dev,"Failed to write config %x: %d\n",r,ret);} } }
    if chip.runtime_pm {pm_runtime_put((*map).dev);} if (*d).wake_count<0 {for _ in (*d).wake_count..0 {disable_irq_wake((*d).irq);}} else {for _ in 0..(*d).wake_count {enable_irq_wake((*d).irq);}} (*d).wake_count=0; mutex_unlock(&mut (*d).lock);
}

unsafe extern "C" fn regmap_irq_enable(data:*mut irq_data) { let d=irq_data_get_irq_chip_data(data); let q=irq_to_regmap_irq(d,(*data).hwirq as i32); let r=(*q).reg_offset/(*(*d).map).reg_stride; let m=if (*(*d).chip).type_in_mask&&(*q).type_.types_supported!=0 {(*d).type_buf.add(r as usize).read()&(*q).mask} else {(*q).mask}; if (*(*d).chip).clear_on_unmask {(*d).clear_status=true;} (*d).mask_buf.add(r as usize).write((*d).mask_buf.add(r as usize).read()&!m); }
unsafe extern "C" fn regmap_irq_disable(data:*mut irq_data) {let d=irq_data_get_irq_chip_data(data);let q=irq_to_regmap_irq(d,(*data).hwirq as i32);let r=(*q).reg_offset/(*(*d).map).reg_stride;(*d).mask_buf.add(r as usize).write((*d).mask_buf.add(r as usize).read()|(*q).mask);}
unsafe extern "C" fn regmap_irq_set_type(data:*mut irq_data,ty:u32)->i32 {let d=irq_data_get_irq_chip_data(data);let q=irq_to_regmap_irq(d,(*data).hwirq as i32);let t=&(*q).type_;if t.types_supported&ty!=ty{return 0}let r=(t.type_reg_offset/(*(*d).map).reg_stride) as i32;if (*(*d).chip).type_in_mask {if let Some(f)=(*(*d).chip).set_type_config_simple {let x=f(&mut (*d).type_buf,ty,q,r,(*(*d).chip).irq_drv_data);if x!=0{return x}}}if let Some(f)=(*(*d).chip).set_type_config {return f((*d).config_buf,ty,q,r,(*(*d).chip).irq_drv_data)}0}
unsafe extern "C" fn regmap_irq_set_wake(data:*mut irq_data,on:u32)->i32 {let d=irq_data_get_irq_chip_data(data);let q=irq_to_regmap_irq(d,(*data).hwirq as i32);let r=((*q).reg_offset/(*(*d).map).reg_stride) as usize;if !(*d).wake_buf.is_null(){if on!=0{(*d).wake_buf.add(r).write((*d).wake_buf.add(r).read()&!(*q).mask)}else{(*d).wake_buf.add(r).write((*d).wake_buf.add(r).read()|(*q).mask)}}if on!=0{(*d).wake_count+=1}else{(*d).wake_count-=1}0}

#[no_mangle] pub unsafe extern "C" fn regmap_irq_get_irq_reg_linear_impl(d:*mut regmap_irq_chip_data,b:u32,i:i32)->u32 {b.wrapping_add((i as u32).wrapping_mul((*(*d).map).reg_stride).wrapping_mul((*d).irq_reg_stride))}
#[no_mangle] pub unsafe extern "C" fn regmap_irq_set_type_config_simple(buf:*mut *mut u32,ty:u32,q:*const regmap_irq,_idx:i32,_x:*mut c_void)->i32 {let t=&(*q).type_;let p=*buf;if t.type_reg_mask!=0{(*p).write((*p).read()&!t.type_reg_mask)}else{(*p).write((*p).read()&!(t.type_falling_val|t.type_rising_val|t.type_level_low_val|t.type_level_high_val));}let v=match ty{IRQ_TYPE_EDGE_FALLING=>t.type_falling_val,IRQ_TYPE_EDGE_RISING=>t.type_rising_val,IRQ_TYPE_EDGE_BOTH=>t.type_falling_val|t.type_rising_val,IRQ_TYPE_LEVEL_HIGH=>t.type_level_high_val,IRQ_TYPE_LEVEL_LOW=>t.type_level_low_val,_=>return -22};(*p).write((*p).read()|v);0}

// External kernel declarations and structures are intentionally referenced, not reimplemented.
extern "C" { fn regmap_add_irq_chip_fwnode(f:*mut fwnode_handle,m:*mut regmap,irq:i32,flags:i32,base:i32,c:*const regmap_irq_chip,d:*mut *mut regmap_irq_chip_data)->i32; fn regmap_del_irq_chip(irq:i32,d:*mut regmap_irq_chip_data); fn regmap_add_irq_chip(m:*mut regmap,irq:i32,flags:i32,base:i32,c:*const regmap_irq_chip,d:*mut *mut regmap_irq_chip_data)->i32; fn regmap_irq_get_virq(d:*mut regmap_irq_chip_data,irq:i32)->i32; fn regmap_irq_chip_get_base(d:*mut regmap_irq_chip_data)->i32; fn regmap_irq_get_domain(d:*mut regmap_irq_chip_data)->*mut irq_domain; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
