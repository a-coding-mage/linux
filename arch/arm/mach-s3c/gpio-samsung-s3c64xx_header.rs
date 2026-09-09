/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C6400 - GPIO lib support
 */

/* CONFIG_GPIO_SAMSUNG */

/* GPIO bank sizes */
pub const S3C64XX_GPIO_A_NR: i32 = 8;
pub const S3C64XX_GPIO_B_NR: i32 = 7;
pub const S3C64XX_GPIO_C_NR: i32 = 8;
pub const S3C64XX_GPIO_D_NR: i32 = 5;
pub const S3C64XX_GPIO_E_NR: i32 = 5;
pub const S3C64XX_GPIO_F_NR: i32 = 16;
pub const S3C64XX_GPIO_G_NR: i32 = 7;
pub const S3C64XX_GPIO_H_NR: i32 = 10;
pub const S3C64XX_GPIO_I_NR: i32 = 16;
pub const S3C64XX_GPIO_J_NR: i32 = 12;
pub const S3C64XX_GPIO_K_NR: i32 = 16;
pub const S3C64XX_GPIO_L_NR: i32 = 15;
pub const S3C64XX_GPIO_M_NR: i32 = 6;
pub const S3C64XX_GPIO_N_NR: i32 = 16;
pub const S3C64XX_GPIO_O_NR: i32 = 16;
pub const S3C64XX_GPIO_P_NR: i32 = 15;
pub const S3C64XX_GPIO_Q_NR: i32 = 9;

/* GPIO bank numbers */
/* CONFIG_S3C_GPIO_SPACE allows the user to select extra space for debugging. */

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum s3c_gpio_number {
    S3C64XX_GPIO_A_START = 0,
    S3C64XX_GPIO_B_START = S3C64XX_GPIO_A_START as i32 + S3C64XX_GPIO_A_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_C_START = S3C64XX_GPIO_B_START as i32 + S3C64XX_GPIO_B_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_D_START = S3C64XX_GPIO_C_START as i32 + S3C64XX_GPIO_C_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_E_START = S3C64XX_GPIO_D_START as i32 + S3C64XX_GPIO_D_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_F_START = S3C64XX_GPIO_E_START as i32 + S3C64XX_GPIO_E_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_G_START = S3C64XX_GPIO_F_START as i32 + S3C64XX_GPIO_F_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_H_START = S3C64XX_GPIO_G_START as i32 + S3C64XX_GPIO_G_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_I_START = S3C64XX_GPIO_H_START as i32 + S3C64XX_GPIO_H_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_J_START = S3C64XX_GPIO_I_START as i32 + S3C64XX_GPIO_I_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_K_START = S3C64XX_GPIO_J_START as i32 + S3C64XX_GPIO_J_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_L_START = S3C64XX_GPIO_K_START as i32 + S3C64XX_GPIO_K_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_M_START = S3C64XX_GPIO_L_START as i32 + S3C64XX_GPIO_L_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_N_START = S3C64XX_GPIO_M_START as i32 + S3C64XX_GPIO_M_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_O_START = S3C64XX_GPIO_N_START as i32 + S3C64XX_GPIO_N_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_P_START = S3C64XX_GPIO_O_START as i32 + S3C64XX_GPIO_O_NR + CONFIG_S3C_GPIO_SPACE + 1,
    S3C64XX_GPIO_Q_START = S3C64XX_GPIO_P_START as i32 + S3C64XX_GPIO_P_NR + CONFIG_S3C_GPIO_SPACE + 1,
}

/* S3C64XX GPIO number definitions. */
pub const fn S3C64XX_GPA(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_A_START as i32 + nr }
pub const fn S3C64XX_GPB(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_B_START as i32 + nr }
pub const fn S3C64XX_GPC(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_C_START as i32 + nr }
pub const fn S3C64XX_GPD(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_D_START as i32 + nr }
pub const fn S3C64XX_GPE(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_E_START as i32 + nr }
pub const fn S3C64XX_GPF(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_F_START as i32 + nr }
pub const fn S3C64XX_GPG(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_G_START as i32 + nr }
pub const fn S3C64XX_GPH(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_H_START as i32 + nr }
pub const fn S3C64XX_GPI(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_I_START as i32 + nr }
pub const fn S3C64XX_GPJ(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_J_START as i32 + nr }
pub const fn S3C64XX_GPK(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_K_START as i32 + nr }
pub const fn S3C64XX_GPL(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_L_START as i32 + nr }
pub const fn S3C64XX_GPM(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_M_START as i32 + nr }
pub const fn S3C64XX_GPN(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_N_START as i32 + nr }
pub const fn S3C64XX_GPO(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_O_START as i32 + nr }
pub const fn S3C64XX_GPP(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_P_START as i32 + nr }
pub const fn S3C64XX_GPQ(nr: i32) -> i32 { s3c_gpio_number::S3C64XX_GPIO_Q_START as i32 + nr }

/* the end of the S3C64XX specific gpios */
pub const S3C64XX_GPIO_END: i32 = S3C64XX_GPQ(S3C64XX_GPIO_Q_NR) + 1;
pub const S3C_GPIO_END: i32 = S3C64XX_GPIO_END;
pub const GPIO_BOARD_START: i32 = S3C64XX_GPQ(S3C64XX_GPIO_Q_NR) + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
