// SPDX-License-Identifier: GPL-2.0

//! Miscellaneous devices.
//!
//! C header: [`include/linux/miscdevice.h`](../../../../include/linux/miscdevice.h)
//!
//! Reference: <https://www.kernel.org/doc/html/latest/driver-api/misc_devices.html>

use crate::bindings;
use crate::error::{Error, Result};
use crate::file_operations::{FileOpenAdapter, FileOpener, FileOperationsVtable};
use crate::str::CStr;
use alloc::boxed::Box;
use core::marker::PhantomPinned;
use core::pin::Pin;

use crate::c_types;
use crate::c_types::*;

// In the verification mock version of misc_register, we
// can only handle a small number of devices - but that is
// enough for the tests that we write
// Note that this mock is designed to be efficient for
// verification - it is not required to be efficient to execute.
const MAX_REGISTRATIONS: usize = 4;

pub struct Registrations<T> {
    list: [Option<T>; MAX_REGISTRATIONS],
    registered: usize,
}

impl<T: Copy> Registrations<T> {
    pub fn new() -> Self {
        Self {
            registered: 0,
            list: [None; MAX_REGISTRATIONS],
        }
    }

    pub fn add(self: &mut Self, r: T) -> c_types::c_int {
        assert!(self.registered < MAX_REGISTRATIONS);
        let i = self.registered;
        self.list[i] = Some(r);
        self.registered += 1;
        i as c_types::c_int
    }

    // todo: rearrange this so that it return an index and then use
    // to implement both a lookup and an unregister function
    pub fn find(self: &mut Self, p: fn(&T) -> bool) -> Option<&T> {
        // todo: if we really wanted to match the semantics of misc_register, entries
        // would be searched in reverse order so that later entries can override earlier ones.
        for i in 0..self.registered {
            if let Some(r) = &self.list[i] {
                if p(r) {
                    return Some(r);
                }
            }
        }
        None
    }
}

// static mut registrations: Registrations<&bindings::miscdevice> = Registrations::new();


/// A registration of a miscellaneous device.
pub struct Registration<T: Sync = ()> {
    registered: bool,
    mdev: bindings::miscdevice,
    _pin: PhantomPinned,

    /// Context initialised on construction and made available to all file instances on
    /// [`FileOpener::open`].
    pub context: T,
}

impl<T: Sync> Registration<T> {
    /// Creates a new [`Registration`] but does not register it yet.
    ///
    /// It is allowed to move.
    pub fn new(context: T) -> Self {
        Self {
            registered: false,
            mdev: bindings::miscdevice::default(),
            _pin: PhantomPinned,
            context,
        }
    }

    /// Registers a miscellaneous device.
    ///
    /// Returns a pinned heap-allocated representation of the registration.
    pub fn new_pinned<F: FileOpener<T>>(
        name: &'static CStr,
        minor: Option<i32>,
        context: T,
    ) -> Result<Pin<Box<Self>>> {
        let mut r = Pin::from(Box::try_new(Self::new(context))?);
        r.as_mut().register::<F>(name, minor)?;
        Ok(r)
    }

    /// Registers a miscellaneous device with the rest of the kernel.
    ///
    /// It must be pinned because the memory block that represents the registration is
    /// self-referential. If a minor is not given, the kernel allocates a new one if possible.
    pub fn register<F: FileOpener<T>>(
        self: Pin<&mut Self>,
        name: &'static CStr,
        minor: Option<i32>,
    ) -> Result {
        // SAFETY: We must ensure that we never move out of `this`.
        let this = unsafe { self.get_unchecked_mut() };
        if this.registered {
            // Already registered.
            return Err(Error::EINVAL);
        }

        // SAFETY: The adapter is compatible with `misc_register`.
        this.mdev.fops = unsafe { FileOperationsVtable::<Self, F>::build() };
        this.mdev.name = name.as_char_ptr();
        this.mdev.minor = minor.unwrap_or(bindings::MISC_DYNAMIC_MINOR as i32);

        // let ret = unsafe { bindings::misc_register(&mut this.mdev) };

        // SAFETY: stores &this.mdev into a 'static but the drop method removes it
        // again so it's all fine.
        // let mdev = unsafe { &this.mdev as &'static bindings::miscdevice };

        // todo: in the test environment, instead of keeping a registry of &this.mdev, would we be better registering &this
        // todo: in the test environment, do we want to access drivers through the existing
        // major/minor lookup mechanism or do we want to expose the Rust objects/types and access drivers
        // through Rust's type system?
        // todo: the following ignores MISC_DYNAMIC_MINOR - a problem for the rust_semaphore sample
        // and Android binder
        // let ret = registrations.add(mdev);
        let ret = 0;
        if ret < 0 {
            return Err(Error::from_kernel_errno(ret));
        }
        this.registered = true;
        Ok(())
    }
}

impl<T: Sync> FileOpenAdapter for Registration<T> {
    type Arg = T;

    unsafe fn convert(_inode: *mut bindings::inode, file: *mut bindings::file) -> *const Self::Arg {
        let reg = crate::container_of!((*file).private_data, Self, mdev);
        &(*reg).context
    }
}

// SAFETY: The only method is `register()`, which requires a (pinned) mutable `Registration`, so it
// is safe to pass `&Registration` to multiple threads because it offers no interior mutability,
// except maybe through `Registration::context`, but it is itself `Sync`.
unsafe impl<T: Sync> Sync for Registration<T> {}

// SAFETY: All functions work from any thread. So as long as the `Registration::context` is
// `Send`, so is `Registration<T>`. `T` needs to be `Sync` because it's a requirement of
// `Registration<T>`.
unsafe impl<T: Send + Sync> Send for Registration<T> {}

impl<T: Sync> Drop for Registration<T> {
    /// Removes the registration from the kernel if it has completed successfully before.
    fn drop(&mut self) {
        if self.registered {
            // unsafe { bindings::misc_deregister(&mut self.mdev) }
        }
    }
}
