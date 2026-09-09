/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding translation unit and headers.

pub unsafe fn dal_gpio_service_create(
    dce_version: dce_version,
    dce_environment: dce_environment,
    ctx: *mut dc_context,
) -> *mut gpio_service {
    let service = kzalloc_obj::<gpio_service>();
    if service.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }

    if !dal_hw_translate_init(&mut (*service).translate, dce_version, dce_environment) {
        BREAK_TO_DEBUGGER!(); goto_failure_1(service);
    }
    if !dal_hw_factory_init(&mut (*service).factory, dce_version, dce_environment) {
        BREAK_TO_DEBUGGER!(); goto_failure_1(service);
    }

    let mut index_of_id: i32 = 0;
    (*service).ctx = ctx;
    loop {
        let number_of_bits = (*service).factory.number_of_pins[index_of_id as usize];
        let mut i: u32 = 0;
        if number_of_bits != 0 {
            (*service).busyness[index_of_id as usize] = kcalloc(number_of_bits, core::mem::size_of::<u8>(), GFP_KERNEL);
            if (*service).busyness[index_of_id as usize].is_null() { BREAK_TO_DEBUGGER!(); goto_failure_2(service, index_of_id); }
            while i < number_of_bits { *(*service).busyness[index_of_id as usize].add(i as usize) = 0; i += 1; }
        } else { (*service).busyness[index_of_id as usize] = core::ptr::null_mut(); }
        index_of_id += 1;
        if index_of_id >= GPIO_ID_COUNT { break; }
    }
    return service;
}

unsafe fn goto_failure_2(service: *mut gpio_service, mut index_of_id: i32) -> ! {
    while index_of_id > 0 { index_of_id -= 1; kfree((*service).busyness[index_of_id as usize]); }
    goto_failure_1(service)
}
unsafe fn goto_failure_1(service: *mut gpio_service) -> ! { kfree(service); core::ptr::null_mut() }

pub unsafe fn dal_gpio_service_create_irq(service: *mut gpio_service, offset: u32, mask: u32) -> *mut gpio {
    let mut id = GPIO_ID_UNKNOWN; let mut en = 0;
    if !(*(*service).translate.funcs).offset_to_id(offset, mask, &mut id, &mut en) { ASSERT_CRITICAL!(false); return core::ptr::null_mut(); }
    dal_gpio_create_irq(service, id, en)
}

pub unsafe fn dal_gpio_service_create_generic_mux(service: *mut gpio_service, offset: u32, mask: u32) -> *mut gpio {
    let mut id = GPIO_ID_UNKNOWN; let mut en = 0;
    if !(*(*service).translate.funcs).offset_to_id(offset, mask, &mut id, &mut en) { ASSERT_CRITICAL!(false); return core::ptr::null_mut(); }
    dal_gpio_create(service, id, en, GPIO_PIN_OUTPUT_STATE_DEFAULT)
}

pub unsafe fn dal_gpio_destroy_generic_mux(mux: *mut *mut gpio) { if mux.is_null() || (*mux).is_null() { ASSERT_CRITICAL!(false); return; } dal_gpio_destroy(mux); kfree(*mux); *mux = core::ptr::null_mut(); }

pub unsafe fn dal_gpio_get_generic_pin_info(service: *mut gpio_service, id: gpio_id, en: u32) -> gpio_pin_info {
    let mut pin: gpio_pin_info = core::mem::zeroed();
    if !(*(*service).translate.funcs).id_to_offset.is_none() { (*(*service).translate.funcs).id_to_offset.unwrap()(id, en, &mut pin); }
    else { pin.mask = 0xffff_ffff; pin.offset = 0xffff_ffff; }
    pin
}

pub unsafe fn dal_gpio_service_destroy(ptr: *mut *mut gpio_service) {
    if ptr.is_null() || (*ptr).is_null() { BREAK_TO_DEBUGGER!(); return; }
    let mut i = 0; while i < GPIO_ID_COUNT { kfree((**ptr).busyness[i as usize]); i += 1; }
    kfree(*ptr); *ptr = core::ptr::null_mut();
}

pub unsafe fn dal_mux_setup_config(mux: *mut gpio, config: *mut gpio_generic_mux_config) -> gpio_result {
    if config.is_null() { return GPIO_RESULT_INVALID_DATA; }
    let mut data: gpio_config_data = core::mem::zeroed(); data.config.generic_mux = *config; data.type_ = GPIO_CONFIG_TYPE_GENERIC_MUX; dal_gpio_set_config(mux, &mut data)
}

unsafe fn is_pin_busy(service: *const gpio_service, id: gpio_id, en: u32) -> bool { if id == GPIO_ID_UNKNOWN { false } else { *(*service).busyness[id as usize].add(en as usize) != 0 } }
unsafe fn set_pin_busy(service: *mut gpio_service, id: gpio_id, en: u32) { if id != GPIO_ID_UNKNOWN { *(*service).busyness[id as usize].add(en as usize) = 1; } }
unsafe fn set_pin_free(service: *mut gpio_service, id: gpio_id, en: u32) { if id != GPIO_ID_UNKNOWN { *(*service).busyness[id as usize].add(en as usize) = 0; } }

pub unsafe fn dal_gpio_service_lock(service: *mut gpio_service, id: gpio_id, en: u32) -> gpio_result { if id != GPIO_ID_UNKNOWN && (*service).busyness[id as usize].is_null() { ASSERT_CRITICAL!(false); return GPIO_RESULT_OPEN_FAILED; } set_pin_busy(service,id,en); GPIO_RESULT_OK }
pub unsafe fn dal_gpio_service_unlock(service: *mut gpio_service, id: gpio_id, en: u32) -> gpio_result { if id != GPIO_ID_UNKNOWN && (*service).busyness[id as usize].is_null() { ASSERT_CRITICAL!(false); return GPIO_RESULT_OPEN_FAILED; } set_pin_free(service,id,en); GPIO_RESULT_OK }

pub unsafe fn dal_gpio_service_open(gpio: *mut gpio) -> gpio_result {
    let service=(*gpio).service; let id=(*gpio).id; let en=(*gpio).en; let mode=(*gpio).mode; let pin=&mut (*gpio).pin;
    if (*service).busyness[id as usize].is_null() { ASSERT_CRITICAL!(false); return GPIO_RESULT_OPEN_FAILED; }
    if is_pin_busy(service,id,en) { ASSERT_CRITICAL!(false); return GPIO_RESULT_DEVICE_BUSY; }
    match id { GPIO_ID_DDC_DATA|GPIO_ID_DDC_CLOCK => { *pin=(*(*service).factory.funcs).get_ddc_pin(gpio); (*(*service).factory.funcs).define_ddc_registers(*pin,en); }, GPIO_ID_GENERIC => { *pin=(*(*service).factory.funcs).get_generic_pin(gpio); (*(*service).factory.funcs).define_generic_registers(*pin,en); }, GPIO_ID_HPD => { *pin=(*(*service).factory.funcs).get_hpd_pin(gpio); (*(*service).factory.funcs).define_hpd_registers(*pin,en); }, GPIO_ID_SYNC|GPIO_ID_GSL => {}, _ => { ASSERT_CRITICAL!(false); return GPIO_RESULT_NON_SPECIFIC_ERROR; } }
    if (*pin).is_null() { ASSERT_CRITICAL!(false); return GPIO_RESULT_NON_SPECIFIC_ERROR; }
    if !(*(*pin).funcs).open(*pin,mode) { ASSERT_CRITICAL!(false); dal_gpio_service_close(service,pin); return GPIO_RESULT_OPEN_FAILED; }
    set_pin_busy(service,id,en); GPIO_RESULT_OK
}

pub unsafe fn dal_gpio_service_close(service:*mut gpio_service, ptr:*mut *mut hw_gpio_pin) { if ptr.is_null() { ASSERT_CRITICAL!(false); return; } let pin=*ptr; if !pin.is_null() { set_pin_free(service,(*pin).id,(*pin).en); (*(*pin).funcs).close(pin); *ptr=core::ptr::null_mut(); } }

pub unsafe fn dal_irq_get_source(irq:*const gpio)->dc_irq_source { match dal_gpio_get_id(irq) { GPIO_ID_HPD => (DC_IRQ_SOURCE_HPD1 as u32 + dal_gpio_get_enum(irq)) as dc_irq_source, GPIO_ID_GPIO_PAD => (DC_IRQ_SOURCE_GPIOPAD0 as u32 + dal_gpio_get_enum(irq)) as dc_irq_source, _=>DC_IRQ_SOURCE_INVALID } }
pub unsafe fn dal_irq_get_rx_source(irq:*const gpio)->dc_irq_source { match dal_gpio_get_id(irq) { GPIO_ID_HPD => (DC_IRQ_SOURCE_HPD1RX as u32 + dal_gpio_get_enum(irq)) as dc_irq_source, _=>DC_IRQ_SOURCE_INVALID } }
pub unsafe fn dal_irq_get_read_request(irq:*const gpio)->dc_irq_source { match dal_gpio_get_id(irq) { GPIO_ID_HPD => (DC_IRQ_SOURCE_DCI2C_RR_DDC1 as u32 + dal_gpio_get_enum(irq)) as dc_irq_source, _=>DC_IRQ_SOURCE_INVALID } }

pub unsafe fn dal_irq_setup_hpd_filter(irq:*mut gpio, config:*mut gpio_hpd_config)->gpio_result { if config.is_null(){return GPIO_RESULT_INVALID_DATA;} let mut d:gpio_config_data=core::mem::zeroed(); d.type_=GPIO_CONFIG_TYPE_HPD; d.config.hpd=*config; dal_gpio_set_config(irq,&mut d) }

pub unsafe fn dal_gpio_create_irq(service:*mut gpio_service,id:gpio_id,en:u32)->*mut gpio { match id { GPIO_ID_HPD|GPIO_ID_GPIO_PAD=>{}, _=>{ASSERT_CRITICAL!(false);return core::ptr::null_mut();} } let irq=dal_gpio_create(service,id,en,GPIO_PIN_OUTPUT_STATE_DEFAULT); if !irq.is_null(){irq}else{ASSERT_CRITICAL!(false);core::ptr::null_mut()} }
pub unsafe fn dal_gpio_destroy_irq(irq:*mut *mut gpio){if irq.is_null()||(*irq).is_null(){ASSERT_CRITICAL!(false);return;}dal_gpio_destroy(irq);kfree(*irq);*irq=core::ptr::null_mut();}

pub unsafe fn dal_gpio_create_ddc(service:*mut gpio_service,offset:u32,mask:u32,info:*mut gpio_ddc_hw_info)->*mut ddc { let mut id=GPIO_ID_UNKNOWN;let mut en=0;if !(*(*service).translate.funcs).offset_to_id(offset,mask,&mut id,&mut en){return core::ptr::null_mut();}let ddc=kzalloc_obj::<ddc>();if ddc.is_null(){BREAK_TO_DEBUGGER!();return core::ptr::null_mut();}(*ddc).pin_data=dal_gpio_create(service,GPIO_ID_DDC_DATA,en,GPIO_PIN_OUTPUT_STATE_DEFAULT);if (*ddc).pin_data.is_null(){BREAK_TO_DEBUGGER!();kfree(ddc);return core::ptr::null_mut();}(*ddc).pin_clock=dal_gpio_create(service,GPIO_ID_DDC_CLOCK,en,GPIO_PIN_OUTPUT_STATE_DEFAULT);if (*ddc).pin_clock.is_null(){BREAK_TO_DEBUGGER!();dal_gpio_destroy(&mut (*ddc).pin_data);kfree(ddc);return core::ptr::null_mut();}(*ddc).hw_info=*info;(*ddc).ctx=(*service).ctx;ddc}
pub unsafe fn dal_gpio_destroy_ddc(ddc:*mut *mut ddc){if ddc.is_null()||(*ddc).is_null(){BREAK_TO_DEBUGGER!();return;}dal_ddc_close(*ddc);dal_gpio_destroy(&mut(**ddc).pin_data);dal_gpio_destroy(&mut(**ddc).pin_clock);kfree(*ddc);*ddc=core::ptr::null_mut();}

pub unsafe fn dal_ddc_open(ddc:*mut ddc,mode:gpio_mode,config_type:gpio_ddc_config_type)->gpio_result { let mut r=dal_gpio_open_ex((*ddc).pin_data,mode);if r!=GPIO_RESULT_OK{BREAK_TO_DEBUGGER!();return r;}r=dal_gpio_open_ex((*ddc).pin_clock,mode);if r!=GPIO_RESULT_OK{BREAK_TO_DEBUGGER!();dal_gpio_close((*ddc).pin_data);return r;}let mut d:gpio_config_data=core::mem::zeroed();d.type_=if mode==GPIO_MODE_INPUT{GPIO_CONFIG_TYPE_I2C_AUX_DUAL_MODE}else{GPIO_CONFIG_TYPE_DDC};d.config.ddc.type_=config_type;let hd=FROM_HW_GPIO_PIN((*(*ddc).pin_data).pin);let hc=FROM_HW_GPIO_PIN((*(*ddc).pin_clock).pin);d.config.ddc.data_en_bit_present=(*hd).store.en!=0;d.config.ddc.clock_en_bit_present=(*hc).store.en!=0;r=dal_gpio_set_config((*ddc).pin_data,&mut d);if r==GPIO_RESULT_OK{return r;}BREAK_TO_DEBUGGER!();dal_gpio_close((*ddc).pin_clock);dal_gpio_close((*ddc).pin_data);r}
pub unsafe fn dal_ddc_change_mode(ddc:*mut ddc,mode:gpio_mode)->gpio_result {let original=dal_gpio_get_mode((*ddc).pin_data);let mut r=dal_gpio_change_mode((*ddc).pin_data,mode);if r!=GPIO_RESULT_OK{dal_gpio_change_mode((*ddc).pin_data,original);return r;}r=dal_gpio_change_mode((*ddc).pin_clock,mode);if r!=GPIO_RESULT_OK{dal_gpio_change_mode((*ddc).pin_clock,original);dal_gpio_change_mode((*ddc).pin_data,original);}r}
pub unsafe fn dal_ddc_get_line(ddc:*const ddc)->gpio_ddc_line{if ddc.is_null(){GPIO_DDC_LINE_UNKNOWN}else{dal_gpio_get_enum((*ddc).pin_data) as gpio_ddc_line}}
pub unsafe fn dal_ddc_set_config(ddc:*mut ddc,config_type:gpio_ddc_config_type)->gpio_result{let mut d:gpio_config_data=core::mem::zeroed();d.type_=GPIO_CONFIG_TYPE_DDC;d.config.ddc.type_=config_type;d.config.ddc.data_en_bit_present=false;d.config.ddc.clock_en_bit_present=false;dal_gpio_set_config((*ddc).pin_data,&mut d)}
pub unsafe fn dal_ddc_close(ddc:*mut ddc){if !ddc.is_null(){dal_gpio_close((*ddc).pin_clock);dal_gpio_close((*ddc).pin_data);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
