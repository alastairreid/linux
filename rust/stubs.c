#include <linux/kernel.h>
// #include <linux/compiler_types.h>

// note that the following cannot be written in C because of the
// varargs arguments
int printk(const char *fmt, ...) {
    // The following implementation looks promising but, because it just
    // prints the format string, all you see is something like "6%s: %.*s:0"
    // which is not as useful as I had hoped.
    extern void klee_print_expr(const char *msg, int _dummy);
    klee_print_expr(fmt, 0);
    return 0;
}
