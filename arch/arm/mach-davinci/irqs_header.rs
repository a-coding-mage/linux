/*
 * DaVinci interrupt controller definitions
 *
 *  Copyright (C) 2006 Texas Instruments.
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under  the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation;  either version 2 of the  License, or (at your
 *  option) any later version.
 *
 *  THIS  SOFTWARE  IS PROVIDED   ``AS  IS'' AND   ANY  EXPRESS OR IMPLIED
 *  WARRANTIES,   INCLUDING, BUT NOT  LIMITED  TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN
 *  NO  EVENT  SHALL   THE AUTHOR  BE    LIABLE FOR ANY   DIRECT, INDIRECT,
 *  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *  NOT LIMITED   TO, PROCUREMENT OF  SUBSTITUTE GOODS  OR SERVICES; LOSS OF
 *  USE, DATA,  OR PROFITS; OR  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 *  ANY THEORY OF LIABILITY, WHETHER IN  CONTRACT, STRICT LIABILITY, OR TORT
 *  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 *  THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  You should have received a copy of the  GNU General Public License along
 *  with this program; if not, write  to the Free Software Foundation, Inc.,
 *  675 Mass Ave, Cambridge, MA 02139, USA.
 */

/* DA8XX interrupts */
pub const IRQ_DA8XX_COMMTX: i32 = 0;
pub const IRQ_DA8XX_COMMRX: i32 = 1;
pub const IRQ_DA8XX_NINT: i32 = 2;
pub const IRQ_DA8XX_EVTOUT0: i32 = 3;
pub const IRQ_DA8XX_EVTOUT1: i32 = 4;
pub const IRQ_DA8XX_EVTOUT2: i32 = 5;
pub const IRQ_DA8XX_EVTOUT3: i32 = 6;
pub const IRQ_DA8XX_EVTOUT4: i32 = 7;
pub const IRQ_DA8XX_EVTOUT5: i32 = 8;
pub const IRQ_DA8XX_EVTOUT6: i32 = 9;
pub const IRQ_DA8XX_EVTOUT7: i32 = 10;
pub const IRQ_DA8XX_CCINT0: i32 = 11;
pub const IRQ_DA8XX_CCERRINT: i32 = 12;
pub const IRQ_DA8XX_TCERRINT0: i32 = 13;
pub const IRQ_DA8XX_AEMIFINT: i32 = 14;
pub const IRQ_DA8XX_I2CINT0: i32 = 15;
pub const IRQ_DA8XX_MMCSDINT0: i32 = 16;
pub const IRQ_DA8XX_MMCSDINT1: i32 = 17;
pub const IRQ_DA8XX_ALLINT0: i32 = 18;
pub const IRQ_DA8XX_RTC: i32 = 19;
pub const IRQ_DA8XX_SPINT0: i32 = 20;
pub const IRQ_DA8XX_TINT12_0: i32 = 21;
pub const IRQ_DA8XX_TINT34_0: i32 = 22;
pub const IRQ_DA8XX_TINT12_1: i32 = 23;
pub const IRQ_DA8XX_TINT34_1: i32 = 24;
pub const IRQ_DA8XX_UARTINT0: i32 = 25;
pub const IRQ_DA8XX_KEYMGRINT: i32 = 26;
pub const IRQ_DA8XX_SECINT: i32 = 26;
pub const IRQ_DA8XX_SECKEYERR: i32 = 26;
pub const IRQ_DA8XX_CHIPINT0: i32 = 28;
pub const IRQ_DA8XX_CHIPINT1: i32 = 29;
pub const IRQ_DA8XX_CHIPINT2: i32 = 30;
pub const IRQ_DA8XX_CHIPINT3: i32 = 31;
pub const IRQ_DA8XX_TCERRINT1: i32 = 32;
pub const IRQ_DA8XX_C0_RX_THRESH_PULSE: i32 = 33;
pub const IRQ_DA8XX_C0_RX_PULSE: i32 = 34;
pub const IRQ_DA8XX_C0_TX_PULSE: i32 = 35;
pub const IRQ_DA8XX_C0_MISC_PULSE: i32 = 36;
pub const IRQ_DA8XX_C1_RX_THRESH_PULSE: i32 = 37;
pub const IRQ_DA8XX_C1_RX_PULSE: i32 = 38;
pub const IRQ_DA8XX_C1_TX_PULSE: i32 = 39;
pub const IRQ_DA8XX_C1_MISC_PULSE: i32 = 40;
pub const IRQ_DA8XX_MEMERR: i32 = 41;
pub const IRQ_DA8XX_GPIO0: i32 = 42;
pub const IRQ_DA8XX_GPIO1: i32 = 43;
pub const IRQ_DA8XX_GPIO2: i32 = 44;
pub const IRQ_DA8XX_GPIO3: i32 = 45;
pub const IRQ_DA8XX_GPIO4: i32 = 46;
pub const IRQ_DA8XX_GPIO5: i32 = 47;
pub const IRQ_DA8XX_GPIO6: i32 = 48;
pub const IRQ_DA8XX_GPIO7: i32 = 49;
pub const IRQ_DA8XX_GPIO8: i32 = 50;
pub const IRQ_DA8XX_I2CINT1: i32 = 51;
pub const IRQ_DA8XX_LCDINT: i32 = 52;
pub const IRQ_DA8XX_UARTINT1: i32 = 53;
pub const IRQ_DA8XX_MCASPINT: i32 = 54;
pub const IRQ_DA8XX_ALLINT1: i32 = 55;
pub const IRQ_DA8XX_SPINT1: i32 = 56;
pub const IRQ_DA8XX_UHPI_INT1: i32 = 57;
pub const IRQ_DA8XX_USB_INT: i32 = 58;
pub const IRQ_DA8XX_IRQN: i32 = 59;
pub const IRQ_DA8XX_RWAKEUP: i32 = 60;
pub const IRQ_DA8XX_UARTINT2: i32 = 61;
pub const IRQ_DA8XX_DFTSSINT: i32 = 62;
pub const IRQ_DA8XX_EHRPWM0: i32 = 63;
pub const IRQ_DA8XX_EHRPWM0TZ: i32 = 64;
pub const IRQ_DA8XX_EHRPWM1: i32 = 65;
pub const IRQ_DA8XX_EHRPWM1TZ: i32 = 66;
pub const IRQ_DA8XX_ECAP0: i32 = 69;
pub const IRQ_DA8XX_ECAP1: i32 = 70;
pub const IRQ_DA8XX_ECAP2: i32 = 71;
pub const IRQ_DA8XX_ARMCLKSTOPREQ: i32 = 90;

/* DA850 speicific interrupts */
pub const IRQ_DA850_MPUADDRERR0: i32 = 27;
pub const IRQ_DA850_MPUPROTERR0: i32 = 27;
pub const IRQ_DA850_IOPUADDRERR0: i32 = 27;
pub const IRQ_DA850_IOPUPROTERR0: i32 = 27;
pub const IRQ_DA850_IOPUADDRERR1: i32 = 27;
pub const IRQ_DA850_IOPUPROTERR1: i32 = 27;
pub const IRQ_DA850_IOPUADDRERR2: i32 = 27;
pub const IRQ_DA850_IOPUPROTERR2: i32 = 27;
pub const IRQ_DA850_BOOTCFG_ADDR_ERR: i32 = 27;
pub const IRQ_DA850_BOOTCFG_PROT_ERR: i32 = 27;
pub const IRQ_DA850_MPUADDRERR1: i32 = 27;
pub const IRQ_DA850_MPUPROTERR1: i32 = 27;
pub const IRQ_DA850_IOPUADDRERR3: i32 = 27;
pub const IRQ_DA850_IOPUPROTERR3: i32 = 27;
pub const IRQ_DA850_IOPUADDRERR4: i32 = 27;
pub const IRQ_DA850_IOPUPROTERR4: i32 = 27;
pub const IRQ_DA850_IOPUADDRERR5: i32 = 27;
pub const IRQ_DA850_IOPUPROTERR5: i32 = 27;
pub const IRQ_DA850_MIOPU_BOOTCFG_ERR: i32 = 27;
pub const IRQ_DA850_SATAINT: i32 = 67;
pub const IRQ_DA850_TINT12_2: i32 = 68;
pub const IRQ_DA850_TINT34_2: i32 = 68;
pub const IRQ_DA850_TINTALL_2: i32 = 68;
pub const IRQ_DA850_MMCSDINT0_1: i32 = 72;
pub const IRQ_DA850_MMCSDINT1_1: i32 = 73;
pub const IRQ_DA850_T12CMPINT0_2: i32 = 74;
pub const IRQ_DA850_T12CMPINT1_2: i32 = 75;
pub const IRQ_DA850_T12CMPINT2_2: i32 = 76;
pub const IRQ_DA850_T12CMPINT3_2: i32 = 77;
pub const IRQ_DA850_T12CMPINT4_2: i32 = 78;
pub const IRQ_DA850_T12CMPINT5_2: i32 = 79;
pub const IRQ_DA850_T12CMPINT6_2: i32 = 80;
pub const IRQ_DA850_T12CMPINT7_2: i32 = 81;
pub const IRQ_DA850_T12CMPINT0_3: i32 = 82;
pub const IRQ_DA850_T12CMPINT1_3: i32 = 83;
pub const IRQ_DA850_T12CMPINT2_3: i32 = 84;
pub const IRQ_DA850_T12CMPINT3_3: i32 = 85;
pub const IRQ_DA850_T12CMPINT4_3: i32 = 86;
pub const IRQ_DA850_T12CMPINT5_3: i32 = 87;
pub const IRQ_DA850_T12CMPINT6_3: i32 = 88;
pub const IRQ_DA850_T12CMPINT7_3: i32 = 89;
pub const IRQ_DA850_RPIINT: i32 = 91;
pub const IRQ_DA850_VPIFINT: i32 = 92;
pub const IRQ_DA850_CCINT1: i32 = 93;
pub const IRQ_DA850_CCERRINT1: i32 = 94;
pub const IRQ_DA850_TCERRINT2: i32 = 95;
pub const IRQ_DA850_TINT12_3: i32 = 96;
pub const IRQ_DA850_TINT34_3: i32 = 96;
pub const IRQ_DA850_TINTALL_3: i32 = 96;
pub const IRQ_DA850_MCBSP0RINT: i32 = 97;
pub const IRQ_DA850_MCBSP0XINT: i32 = 98;
pub const IRQ_DA850_MCBSP1RINT: i32 = 99;
pub const IRQ_DA850_MCBSP1XINT: i32 = 100;

pub const DA850_N_CP_INTC_IRQ: i32 = 101;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
