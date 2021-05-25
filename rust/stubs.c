#include <linux/kernel.h>
#include <linux/sched/signal.h>
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

unsigned long rust_helper_copy_from_user(void *to, const void __user *from, unsigned long n)
{
	return 0;
}

unsigned long rust_helper_copy_to_user(void __user *to, const void *from, unsigned long n)
{
	return 0;
}

void rust_helper_init_wait(struct wait_queue_entry *wq_entry)
{
}

void rust_helper_kunmap(struct page *page)
{
}

int rust_helper_signal_pending(void)
{
    return 0;
}
