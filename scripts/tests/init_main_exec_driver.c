/* SPDX-License-Identifier: GPL-2.0-only */
/* Observable services for the unchanged C and staged Rust execution bodies.
 * They record calls; this fixture does not execute userspace programs or claim
 * to model printk consoles, panic shutdown, or the kernel_init lifecycle.
 */
#include <linux/init.h>
#include <linux/string.h>
#include <linux/binfmts.h>
#include <linux/printk.h>
#include <linux/dynamic_debug.h>
#include <linux/panic.h>
#include <linux/errno.h>
#include <linux/stdarg.h>

extern int printf(const char *, ...);
extern void exit(int) __attribute__((noreturn));
extern int vsnprintf(char *, size_t, const char *, va_list);

static unsigned int attempt;
static int scenario;
static const char **expected_argv, **expected_envp;

void prepare_exec_case(int, const char **, const char **, char **, char **);
int exercise_exec(int, int (*)(int));

/* Real kernel callers may provide only eight-byte stack alignment. Align
 * these observation boundaries before entering host libc's variadic ABI. */
__attribute__((force_align_arg_pointer))
void prepare_exec_case(int which, const char **argv, const char **envp,
                       char **execute, char **ramdisk)
{
    for (int i = 0; i < CONFIG_INIT_ENV_ARG_LIMIT + 2; ++i) {
        argv[i] = NULL;
        envp[i] = NULL;
    }
    argv[0] = "old-init";
    argv[1] = "-s";
    argv[2] = "flag=\x80";
    envp[0] = "HOME=/";
    envp[1] = "TERM=linux";
    envp[2] = "X=\xff";
    if (which == 17) {
        argv[1] = NULL;
        envp[0] = NULL;
    }
    *execute = which == 5 || which == 6 ? "/chosen" : NULL;
    *ramdisk = which >= 7 ? NULL : "/init";
    expected_argv = argv;
    expected_envp = envp;
}

__attribute__((force_align_arg_pointer))
int kernel_execve(const char *name, const char *const *argv, const char *const *envp)
{
    ++attempt;
    if (argv != expected_argv || envp != expected_envp || argv[0] != name)
        exit(90);
    printf("EXEC %s", name);
    for (const char *const *p = argv; *p; ++p) printf(" A[%s]", *p);
    for (const char *const *p = envp; *p; ++p) printf(" E[%s]", *p);
    printf("\n");
    if (scenario == 0 || (scenario >= 15 && scenario <= 18)) return 37;
    if (scenario == 1 || scenario == 14) return -ENOENT;
    if (scenario == 2 || scenario == 6) return -EACCES;
    if (scenario == 19) return (-2147483647 - 1);
    if (scenario == 20) return 2147483647;
    if (scenario == 3 || scenario == 4 || scenario == 7) return 0;
    if (scenario == 5) return attempt == 1 ? -EIO : 0;
    if (scenario == 8) return attempt < 3 ? -ENOENT : 0;
    return attempt == (unsigned int)(scenario - 8) ? 0 : -ENOENT;
}

#ifdef CONFIG_PRINTK
__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
    char message[1024];
    va_list args;
    va_start(args, format);
    int count = vsnprintf(message, sizeof(message), format, args);
    va_end(args);
    printf("PRINT %s", message);
    if (scenario == 18 && strstr(message, "Run ")) expected_argv[1] = "changed-by-printk";
    return count;
}
#endif

#ifdef CONFIG_DYNAMIC_DEBUG
extern struct _ddebug __start___dyndbg[], __stop___dyndbg[];
__attribute__((force_align_arg_pointer))
void __dynamic_pr_debug(struct _ddebug *descriptor, const char *format, ...)
{
    char message[1024];
    va_list args;
    if (strcmp(descriptor->function, "run_init_process") ||
        strcmp(descriptor->modname, "main") ||
        strcmp(descriptor->format, format) ||
        descriptor->class_id != _DPRINTK_CLASS_DFLT)
        exit(91);
    va_start(args, format);
    vsnprintf(message, sizeof(message), format, args);
    va_end(args);
    printf("DEBUG %s", message);
}
__attribute__((force_align_arg_pointer))
void dump_stack(void) { printf("STACK\n"); }
#endif

__attribute__((force_align_arg_pointer))
void panic(const char *format, ...)
{
    char message[1024];
    va_list args;
    va_start(args, format);
    vsnprintf(message, sizeof(message), format, args);
    va_end(args);
    printf("PANIC %s\n", message);
    exit(42);
}

__attribute__((force_align_arg_pointer))
int exercise_exec(int which, int (*callback)(int))
{
    scenario = which;
    attempt = 0;
#ifdef CONFIG_DYNAMIC_DEBUG
    for (struct _ddebug *p = __start___dyndbg; p < __stop___dyndbg; ++p) {
        if (p->flags != _DPRINTK_FLAGS_PRINT || p->class_id != _DPRINTK_CLASS_DFLT)
            return 92;
        if (which == 15) p->flags = 0;
        if (which == 16) p->flags |= _DPRINTK_FLAGS_INCL_STACK;
    }
#endif
    int result = callback(which);
    printf("RETURN %d attempts=%u\n", result, attempt);
    return 0;
}
