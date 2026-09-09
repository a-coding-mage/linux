/*
 *
 * Header for code common to all OMAP1 machines.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 675 Mass Ave, Cambridge, MA 02139, USA.
 */

// Dependencies supplied by the corresponding kernel headers and source files:
// linux/platform_data/i2c-omap.h, linux/reboot.h, asm/exception.h, irqs.h,
// soc.h, and i2c.h.

extern "C" {
    pub static omap16xx_mpu_gpio_swnode: software_node;
    pub static omap16xx_gpio1_swnode: software_node;

    #[cfg(CONFIG_OMAP_SERIAL_WAKE)]
    pub fn omap_serial_wakeup_init() -> i32;

    #[cfg(not(CONFIG_OMAP_SERIAL_WAKE))]
    #[inline]
    pub unsafe fn omap_serial_wakeup_init() -> i32 {
        0
    }

    pub fn omap1_map_io();
    pub fn omap1_init_early();
    pub fn omap1_init_irq();
    pub fn omap1_handle_irq(regs: *mut pt_regs);
    pub fn omap1_init_late();
    pub fn omap1_restart(mode: reboot_mode, cmd: *const core::ffi::c_char);

    pub fn omap_check_revision();

    pub fn omap1_nand_cmd_ctl(
        this: *mut nand_chip,
        cmd: i32,
        ctrl: u32,
    );

    pub fn omap1_timer_init();

    #[cfg(CONFIG_OMAP_32K_TIMER)]
    pub fn omap_32k_timer_init() -> i32;

    #[cfg(not(CONFIG_OMAP_32K_TIMER))]
    #[inline]
    pub unsafe fn omap_32k_timer_init() -> i32 {
        -19 // -ENODEV
    }

    #[cfg(CONFIG_ARCH_OMAP16XX)]
    pub fn ocpi_enable() -> i32;

    #[cfg(not(CONFIG_ARCH_OMAP16XX))]
    #[inline]
    pub unsafe fn ocpi_enable() -> i32 {
        0
    }

    pub fn omap1_get_reset_sources() -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
