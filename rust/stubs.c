#include <linux/kernel.h>
// #include <linux/compiler_types.h>

// asmlinkage __visible
int printk(const char *fmt, ...) {
    // The following implementation looks promising but, because it just
    // prints the format string, all you see is something like "6%s: %.*s:0"
    // which is not as useful as I had hoped.
    extern void klee_print_expr(const char *msg, int _dummy);
    klee_print_expr(fmt, 0);
    return 0;
}

int alloc_chrdev_region(dev_t *dev, unsigned baseminor, unsigned count,
			const char *name)
{
	return 0;
}

// note: don't really need to have a stub for this since default behaviour would be fine
void unregister_chrdev_region(dev_t from, unsigned count)
{
}

void cdev_init(struct cdev *cdev, const struct file_operations *fops)
{
}

// todo: Should we be checking whether we have
// exceeded the alloc_chrdev_region count (or something like that)?
int cdev_add(struct cdev *p, dev_t dev, unsigned count)
{
  // todo: also consider returning -EBUSY or some other negative error value
  // to test whether Rust framework can handle that gracefully
  // return -EBUSY;
  // return -42;
	return 0;
}

void cdev_del(struct cdev *p)
{
}
