/*
 *  Definitions for timer registers
 *
 *  Copyright 2004 Philip Rischel <rischelp@idt.com>
 *  Copyright 2008 Florian Fainelli <florian@openwrt.org>
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

// Dependency supplied by <asm/mach-rc32434/rb.h>.

pub const TIMER0_BASE_ADDR: u32 = 0x18028000;
pub const TIMER_COUNT: usize = 3;

#[repr(C)]
pub struct timer_counter {
    pub count: u32,
    pub compare: u32,
    pub ctc: u32, // use CTC_
}

#[repr(C)]
pub struct timer {
    pub tim: [timer_counter; TIMER_COUNT],
    pub rcount: u32,   // use RCOUNT_
    pub rcompare: u32, // use RCOMPARE_
    pub rtc: u32,      // use RTC_
}

pub const RC32434_CTC_EN_BIT: u32 = 0;
pub const RC32434_CTC_TO_BIT: u32 = 1;

/* Real time clock registers */
#[macro_export]
macro_rules! RC32434_RTC_MSK {
    ($x:expr) => {
        BIT_TO_MASK($x)
    };
}
pub const RC32434_RTC_CE_BIT: u32 = 0;
pub const RC32434_RTC_TO_BIT: u32 = 1;
pub const RC32434_RTC_RQE_BIT: u32 = 2;

/* Counter registers */
pub const RC32434_RCOUNT_BIT: u32 = 0;
pub const RC32434_RCOUNT_MSK: u32 = 0x0000ffff;
pub const RC32434_RCOMP_BIT: u32 = 0;
pub const RC32434_RCOMP_MSK: u32 = 0x0000ffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
