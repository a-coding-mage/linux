// SPDX-License-Identifier: GPL-2.0
#include <linux/init.h>
#include <string.h>

int main(void)
{
#ifndef MODULE
    unsigned int setup = 0, early = 0, obsolete = 0;
    char argument = 0x80;
    for (const struct obs_kernel_param *p = __setup_start; p < __setup_end; ++p) {
        int (*volatile callback)(char *) = p->setup_func;
        if (!strcmp(p->str, "init=")) {
            if (p->early || callback(&argument) != 129) return 1;
            ++setup;
        } else if (!strcmp(p->str, "debug")) {
            if (p->early != 1 || callback(&argument) != 130) return 2;
            ++early;
        } else if (!strcmp(p->str, "obsolete")) {
            if (p->early || callback) return 5;
            ++obsolete;
        } else {
            return 3;
        }
    }
    if (setup != 1 || early != 2 || obsolete != 1) return 4;
#endif
    return 0;
}
