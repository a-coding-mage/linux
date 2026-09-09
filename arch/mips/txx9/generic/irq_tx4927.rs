/*
 * Common tx4927 irq handler
 *
 * Author: MontaVista Software, Inc.
 *	   source@mvista.com
 *
 *  under the terms of the GNU General Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 *  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 *  IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 *  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *  BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS
 *  OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 *  ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 *  TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
 *  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  You should have received a copy of the GNU General Public License along
 *  with this program; if not, write to the Free Software Foundation, Inc.,
 *  675 Mass Ave, Cambridge, MA 02139, USA.
 */

// External Linux/MIPS declarations and TX4927 constants are supplied by the
// corresponding platform dependencies.
extern "C" {
    fn mips_cpu_irq_init();
    fn txx9_irq_init(base: u64);
    fn irq_set_chained_handler(irq: i32, handler: *const core::ffi::c_void);
    fn handle_simple_irq();
    fn txx9_irq_set_pri(irq: i32, priority: i32);
}

pub unsafe fn tx4927_irq_init() {
    let mut i: i32;

    mips_cpu_irq_init();
    txx9_irq_init((TX4927_IRC_REG as u64) & 0xfffffffff_u64);
    irq_set_chained_handler(
        MIPS_CPU_IRQ_BASE + TX4927_IRC_INT,
        handle_simple_irq as *const core::ffi::c_void,
    );
    /* raise priority for errors, timers, SIO */
    txx9_irq_set_pri(TX4927_IR_ECCERR, 7);
    txx9_irq_set_pri(TX4927_IR_WTOERR, 7);
    txx9_irq_set_pri(TX4927_IR_PCIERR, 7);
    txx9_irq_set_pri(TX4927_IR_PCIPME, 7);
    i = 0;
    while i < TX4927_NUM_IR_TMR {
        txx9_irq_set_pri(TX4927_IR_TMR(i), 6);
        i += 1;
    }
    i = 0;
    while i < TX4927_NUM_IR_SIO {
        txx9_irq_set_pri(TX4927_IR_SIO(i), 5);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
