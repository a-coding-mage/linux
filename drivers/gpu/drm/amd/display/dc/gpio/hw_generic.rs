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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// dm_services.h, gpio_interface.h, gpio_types.h, hw_gpio.h, hw_generic.h,
// reg_helper.h, and generic_regs.h.

// C macros preserved in intent:
// FN(reg_name, field_name) =>
//     gpio_reg_shift(generic->shifts->field_name), generic->masks->field_name
// CTX => generic->base.base.ctx
// REG(reg) => generic->regs->reg

struct gpio;

unsafe fn dal_hw_generic_destruct(pin: *mut hw_generic) {
    dal_hw_gpio_destruct(&mut (*pin).base);
}

unsafe fn dal_hw_generic_destroy(ptr: *mut *mut hw_gpio_pin) {
    let generic: *mut hw_generic = HW_GENERIC_FROM_BASE(*ptr);

    dal_hw_generic_destruct(generic);

    kfree(generic);

    *ptr = core::ptr::null_mut();
}

unsafe fn set_config(
    ptr: *mut hw_gpio_pin,
    config_data: *const gpio_config_data,
) -> gpio_result {
    let generic: *mut hw_generic = HW_GENERIC_FROM_BASE(ptr);

    if config_data.is_null() {
        return GPIO_RESULT_INVALID_DATA;
    }

    // Equivalent to REG_UPDATE_2(mux, GENERIC_EN, ..., GENERIC_SEL, ...).
    REG_UPDATE_2!(
        generic,
        mux,
        GENERIC_EN,
        (*config_data).config.generic_mux.enable_output_from_mux,
        GENERIC_SEL,
        (*config_data).config.generic_mux.mux_select
    );

    GPIO_RESULT_OK
}

static funcs: hw_gpio_pin_funcs = hw_gpio_pin_funcs {
    destroy: Some(dal_hw_generic_destroy),
    open: Some(dal_hw_gpio_open),
    get_value: Some(dal_hw_gpio_get_value),
    set_value: Some(dal_hw_gpio_set_value),
    set_config: Some(set_config),
    change_mode: Some(dal_hw_gpio_change_mode),
    close: Some(dal_hw_gpio_close),
};

unsafe fn dal_hw_generic_construct(
    pin: *mut hw_generic,
    id: gpio_id,
    en: u32,
    ctx: *mut dc_context,
) {
    dal_hw_gpio_construct(&mut (*pin).base, id, en, ctx);
    (*pin).base.base.funcs = &funcs;
}

unsafe fn dal_hw_generic_init(
    hw_generic: *mut *mut hw_generic,
    ctx: *mut dc_context,
    id: gpio_id,
    en: u32,
) {
    if en > GPIO_DDC_LINE_MAX {
        ASSERT_CRITICAL!(false);
        *hw_generic = core::ptr::null_mut();
    }

    *hw_generic = kzalloc_obj::<hw_generic>();
    if (*hw_generic).is_null() {
        ASSERT_CRITICAL!(false);
        return;
    }

    dal_hw_generic_construct(*hw_generic, id, en, ctx);
}

unsafe fn dal_hw_generic_get_pin(gpio: *mut gpio) -> *mut hw_gpio_pin {
    let hw_generic: *mut hw_generic = dal_gpio_get_generic(gpio);

    &mut (*hw_generic).base.base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
