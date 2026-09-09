/*
 * am300epd.c -- Platform device for AM300 EPD kit
 *
 * Copyright (C) 2008, Jaya Kumar
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file COPYING in the main directory of this archive for
 * more details.
 *
 * This work was made possible by help and equipment support from E-Ink
 * Corporation. http://support.eink.com/community
 *
 * This driver is written to be used with the Broadsheet display controller.
 * on the AM300 EPD prototype kit/development kit with an E-Ink 800x600
 * Vizplex EPD on a Gumstix board using the Broadsheet interface board.
 *
 */

// C includes provide symbols supplied by other translation units.

static mut panel_type: u32 = 6;
static mut am300_device: *mut platform_device = core::ptr::null_mut();
static mut am300_board: broadsheet_board = broadsheet_board {
    owner: THIS_MODULE,
    init: Some(am300_init_board),
    cleanup: Some(am300_cleanup),
    set_hdb: Some(am300_set_hdb),
    get_hdb: Some(am300_get_hdb),
    set_ctl: Some(am300_set_ctl),
    wait_for_rdy: Some(am300_wait_event),
    get_panel_type: Some(am300_get_panel_type),
    setup_irq: Some(am300_setup_irq),
};

static mut am300_pin_config: [c_ulong; 26] = [
    GPIO16_GPIO, GPIO17_GPIO, GPIO32_GPIO, GPIO48_GPIO, GPIO49_GPIO,
    GPIO51_GPIO, GPIO74_GPIO, GPIO75_GPIO, GPIO76_GPIO, GPIO77_GPIO,
    GPIO58_GPIO, GPIO59_GPIO, GPIO60_GPIO, GPIO61_GPIO, GPIO62_GPIO,
    GPIO63_GPIO, GPIO64_GPIO, GPIO65_GPIO, GPIO66_GPIO, GPIO67_GPIO,
    GPIO68_GPIO, GPIO69_GPIO, GPIO70_GPIO, GPIO71_GPIO, GPIO72_GPIO,
    GPIO73_GPIO,
];

/* register offsets for gpio control */
const PWR_GPIO_PIN: c_int = 16;
const CFG_GPIO_PIN: c_int = 17;
const RDY_GPIO_PIN: c_int = 32;
const DC_GPIO_PIN: c_int = 48;
const RST_GPIO_PIN: c_int = 49;
const LED_GPIO_PIN: c_int = 51;
const RD_GPIO_PIN: c_int = 74;
const WR_GPIO_PIN: c_int = 75;
const CS_GPIO_PIN: c_int = 76;
const IRQ_GPIO_PIN: c_int = 77;

/* hdb bus */
const DB0_GPIO_PIN: c_int = 58;
const DB15_GPIO_PIN: c_int = 73;

static mut gpios: [c_int; 10] = [
    PWR_GPIO_PIN, CFG_GPIO_PIN, RDY_GPIO_PIN, DC_GPIO_PIN, RST_GPIO_PIN,
    RD_GPIO_PIN, WR_GPIO_PIN, CS_GPIO_PIN, IRQ_GPIO_PIN, LED_GPIO_PIN,
];
static mut gpio_names: [*mut c_char; 10] = [
    c"PWR".as_ptr() as *mut c_char, c"CFG".as_ptr() as *mut c_char,
    c"RDY".as_ptr() as *mut c_char, c"DC".as_ptr() as *mut c_char,
    c"RST".as_ptr() as *mut c_char, c"RD".as_ptr() as *mut c_char,
    c"WR".as_ptr() as *mut c_char, c"CS".as_ptr() as *mut c_char,
    c"IRQ".as_ptr() as *mut c_char, c"LED".as_ptr() as *mut c_char,
];

unsafe fn am300_wait_event(par: *mut broadsheetfb_par) -> c_int {
    /* todo: improve err recovery */
    wait_event!((*par).waitq, gpio_get_value(RDY_GPIO_PIN) != 0);
    0
}

unsafe fn am300_init_gpio_regs(par: *mut broadsheetfb_par) -> c_int {
    let mut i: c_int;
    let mut err: c_int = 0;
    let mut dbname: [c_char; 8] = [0; 8];

    i = 0;
    while i < gpios.len() as c_int {
        err = gpio_request(gpios[i as usize], gpio_names[i as usize]);
        if err != 0 {
            dev_err(&(*am300_device).dev, "failed requesting gpio %s, err=%d\n", gpio_names[i as usize], err);
            goto! err_req_gpio;
        }
        i += 1;
    }

    /* we also need to take care of the hdb bus */
    i = DB0_GPIO_PIN;
    while i <= DB15_GPIO_PIN {
        sprintf(dbname.as_mut_ptr(), c"DB%d".as_ptr(), i);
        err = gpio_request(i, dbname.as_mut_ptr());
        if err != 0 {
            dev_err(&(*am300_device).dev, "failed requesting gpio %d, err=%d\n", i, err);
            goto! err_req_gpio2;
        }
        i += 1;
    }

    /* setup the outputs and init values */
    gpio_direction_output(PWR_GPIO_PIN, 0);
    gpio_direction_output(CFG_GPIO_PIN, 1);
    gpio_direction_output(DC_GPIO_PIN, 0);
    gpio_direction_output(RD_GPIO_PIN, 1);
    gpio_direction_output(WR_GPIO_PIN, 1);
    gpio_direction_output(CS_GPIO_PIN, 1);
    gpio_direction_output(RST_GPIO_PIN, 0);

    /* setup the inputs */
    gpio_direction_input(RDY_GPIO_PIN);
    gpio_direction_input(IRQ_GPIO_PIN);

    /* start the hdb bus as an input */
    i = DB0_GPIO_PIN;
    while i <= DB15_GPIO_PIN {
        gpio_direction_output(i, 0);
        i += 1;
    }

    /* go into command mode */
    gpio_set_value(CFG_GPIO_PIN, 1);
    gpio_set_value(RST_GPIO_PIN, 0);
    msleep(10);
    gpio_set_value(RST_GPIO_PIN, 1);
    msleep(10);
    am300_wait_event(par);
    return 0;

err_req_gpio2:
    while i > DB0_GPIO_PIN {
        i -= 1;
        gpio_free(i);
    }
    i = gpios.len() as c_int;
err_req_gpio:
    while i > 0 {
        i -= 1;
        gpio_free(gpios[i as usize]);
    }
    err
}

unsafe fn am300_init_board(par: *mut broadsheetfb_par) -> c_int { am300_init_gpio_regs(par) }

unsafe fn am300_cleanup(par: *mut broadsheetfb_par) {
    free_irq(PXA_GPIO_TO_IRQ(RDY_GPIO_PIN), par as *mut c_void);
    for i in 0..gpios.len() { gpio_free(gpios[i]); }
    for i in DB0_GPIO_PIN..=DB15_GPIO_PIN { gpio_free(i); }
}

unsafe fn am300_get_hdb(_par: *mut broadsheetfb_par) -> u16 {
    let mut res: u16 = 0;
    for i in 0..=(DB15_GPIO_PIN - DB0_GPIO_PIN) {
        res |= if gpio_get_value(DB0_GPIO_PIN + i) != 0 { 1u16 << i } else { 0 };
    }
    res
}

unsafe fn am300_set_hdb(_par: *mut broadsheetfb_par, data: u16) {
    for i in 0..=(DB15_GPIO_PIN - DB0_GPIO_PIN) {
        gpio_set_value(DB0_GPIO_PIN + i, ((data >> i) & 0x01) as c_int);
    }
}

unsafe fn am300_set_ctl(_par: *mut broadsheetfb_par, bit: c_uchar, state: u8) {
    match bit {
        BS_CS => gpio_set_value(CS_GPIO_PIN, state as c_int),
        BS_DC => gpio_set_value(DC_GPIO_PIN, state as c_int),
        BS_WR => gpio_set_value(WR_GPIO_PIN, state as c_int),
        _ => (),
    }
}

unsafe fn am300_get_panel_type() -> c_int { panel_type as c_int }

unsafe fn am300_handle_irq(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let par = dev_id as *mut broadsheetfb_par;
    wake_up(&mut (*par).waitq);
    IRQ_HANDLED
}

unsafe fn am300_setup_irq(info: *mut fb_info) -> c_int {
    let par = (*info).par as *mut broadsheetfb_par;
    let ret = request_irq(PXA_GPIO_TO_IRQ(RDY_GPIO_PIN), Some(am300_handle_irq), IRQF_TRIGGER_RISING, c"AM300".as_ptr(), par as *mut c_void);
    if ret != 0 { dev_err(&(*am300_device).dev, "request_irq failed: %d\n", ret); }
    ret
}

unsafe fn am300_init() -> c_int {
    pxa2xx_mfp_config(am300_pin_config.as_mut_ptr(), am300_pin_config.len());
    request_module(c"broadsheetfb".as_ptr());
    am300_device = platform_device_alloc(c"broadsheetfb".as_ptr(), -1);
    if am300_device.is_null() { return -ENOMEM; }
    /* the am300_board that will be seen by broadsheetfb is a copy */
    platform_device_add_data(am300_device, &am300_board as *const _ as *const c_void, core::mem::size_of::<broadsheet_board>());
    let ret = platform_device_add(am300_device);
    if ret != 0 { platform_device_put(am300_device); return ret; }
    0
}

module_param!(panel_type, uint, 0);
module_param_desc!(panel_type, "Select the panel type: 37, 6, 97");
module_description!("board driver for am300 epd kit");
module_author!("Jaya Kumar");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
