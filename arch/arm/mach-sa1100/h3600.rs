// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support for Compaq iPAQ H3600 handheld computer
 *
 * Copyright (c) 2000,1 Compaq Computer Corporation. (Author: Jamey Hicks)
 * Copyright (c) 2009 Dmitry Artamonow <mad_soft@inbox.ru>
 */

// Linux kernel and machine-specific dependencies are supplied by other files.

unsafe extern "C" {
    fn gpio_request(gpio: i32, label: *const core::ffi::c_char) -> i32;
    fn gpio_direction_output(gpio: i32, value: i32) -> i32;
    fn gpio_free(gpio: i32);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn h3xxx_map_io();
    fn h3xxx_mach_init();
    fn sa11x0_register_lcd(info: *mut sa1100fb_mach_info);
    fn sa1100_init_irq();
    fn sa1100_timer_init();
    fn sa11x0_init_late();
    fn sa11x0_restart();
}

// Constants and types are defined by the included kernel headers.
const RGB_16: usize = 0;

#[repr(C)]
struct sa1100fb_rgb_component {
    offset: u32,
    length: u32,
}

#[repr(C)]
struct sa1100fb_rgb {
    red: sa1100fb_rgb_component,
    green: sa1100fb_rgb_component,
    blue: sa1100fb_rgb_component,
    transp: sa1100fb_rgb_component,
}

#[repr(C)]
struct sa1100fb_mach_info {
    pixclock: u32,
    bpp: u32,
    xres: u32,
    yres: u32,
    hsync_len: u32,
    vsync_len: u32,
    left_margin: u32,
    upper_margin: u32,
    right_margin: u32,
    lower_margin: u32,
    cmap_static: u32,
    lccr0: u32,
    lccr3: u32,
    rgb: [*const sa1100fb_rgb; 3],
    lcd_power: Option<unsafe extern "C" fn(i32)>,
}

static mut H3600_LCD_OK: bool = false;

unsafe extern "C" fn h3600_lcd_request() -> bool {
    let mut rc: i32 = 0;

    if H3600_LCD_OK {
        return true;
    }

    rc = gpio_request(H3XXX_EGPIO_LCD_ON, c"LCD power".as_ptr());
    if rc != 0 { goto_out_free_on!(); }
    rc = gpio_direction_output(H3XXX_EGPIO_LCD_ON, 0);
    if rc != 0 { goto_out_free_on!(); }
    rc = gpio_request(H3600_EGPIO_LCD_PCI, c"LCD control".as_ptr());
    if rc != 0 { goto_out_free_on!(); }
    rc = gpio_direction_output(H3600_EGPIO_LCD_PCI, 0);
    if rc != 0 { goto_out_free_pci!(); }
    rc = gpio_request(H3600_EGPIO_LCD_5V_ON, c"LCD 5v".as_ptr());
    if rc != 0 { goto_out_free_pci!(); }
    rc = gpio_direction_output(H3600_EGPIO_LCD_5V_ON, 0);
    if rc != 0 { goto_out_free_5v_on!(); }
    rc = gpio_request(H3600_EGPIO_LVDD_ON, c"LCD 9v/-6.5v".as_ptr());
    if rc != 0 { goto_out_free_5v_on!(); }
    rc = gpio_direction_output(H3600_EGPIO_LVDD_ON, 0);
    if rc != 0 { goto_out_free_lvdd_on!(); }

    if rc != 0 {
        pr_err(c"%s: can't request GPIOs\n".as_ptr(), c"h3600_lcd_request".as_ptr());
    } else {
        H3600_LCD_OK = true;
    }
    return H3600_LCD_OK;

    macro_rules! goto_out_free_lvdd_on { () => { gpio_free(H3600_EGPIO_LVDD_ON); goto_out_free_5v_on!(); }; }
    macro_rules! goto_out_free_5v_on { () => { gpio_free(H3600_EGPIO_LCD_5V_ON); goto_out_free_pci!(); }; }
    macro_rules! goto_out_free_pci { () => { gpio_free(H3600_EGPIO_LCD_PCI); goto_out_free_on!(); }; }
    macro_rules! goto_out_free_on { () => {{ gpio_free(H3XXX_EGPIO_LCD_ON); if rc != 0 { pr_err(c"%s: can't request GPIOs\n".as_ptr(), c"h3600_lcd_request".as_ptr()); } else { H3600_LCD_OK = true; } return H3600_LCD_OK; }}; }
}

unsafe extern "C" fn h3600_lcd_power(enable: i32) {
    if !h3600_lcd_request() { return; }
    gpio_direction_output(H3XXX_EGPIO_LCD_ON, enable);
    gpio_direction_output(H3600_EGPIO_LCD_PCI, enable);
    gpio_direction_output(H3600_EGPIO_LCD_5V_ON, enable);
    gpio_direction_output(H3600_EGPIO_LVDD_ON, enable);
}

static H3600_RGB_16: sa1100fb_rgb = sa1100fb_rgb {
    red: sa1100fb_rgb_component { offset: 12, length: 4 },
    green: sa1100fb_rgb_component { offset: 7, length: 4 },
    blue: sa1100fb_rgb_component { offset: 1, length: 4 },
    transp: sa1100fb_rgb_component { offset: 0, length: 0 },
};

static mut H3600_LCD_INFO: sa1100fb_mach_info = sa1100fb_mach_info {
    pixclock: 174757, bpp: 16, xres: 320, yres: 240,
    hsync_len: 3, vsync_len: 3, left_margin: 12, upper_margin: 10,
    right_margin: 17, lower_margin: 1, cmap_static: 1,
    lccr0: LCCR0_Color | LCCR0_Sngl | LCCR0_Act,
    lccr3: LCCR3_OutEnH | LCCR3_PixRsEdg | LCCR3_ACBsDiv(2),
    rgb: [core::ptr::null(), &H3600_RGB_16, core::ptr::null()],
    lcd_power: Some(h3600_lcd_power),
};

unsafe extern "C" fn h3600_map_io() { h3xxx_map_io(); }

unsafe extern "C" fn h3600_mach_init() {
    h3xxx_mach_init();
    sa11x0_register_lcd(&raw mut H3600_LCD_INFO);
}

// MACHINE_START(H3600, "Compaq iPAQ H3600")
// .atag_offset = 0x100, .map_io = h3600_map_io, .nr_irqs = SA1100_NR_IRQS,
// .init_irq = sa1100_init_irq, .init_time = sa1100_timer_init,
// .init_machine = h3600_mach_init, .init_late = sa11x0_init_late,
// .restart = sa11x0_restart, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
