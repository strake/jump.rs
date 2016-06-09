#![feature(fn_traits)]
#![feature(unboxed_closures)]

#![no_std]

extern crate void;

#[cfg(test)] #[macro_use] extern crate std;

use core::mem;
use core::ptr;
use void::Void;

#[cfg(target_arch = "x86_64")]
const n_saved_regs: usize = 8;

#[link(name = "jump", kind = "static")]
extern "C" {
    fn __mkjump(ptr: *mut [usize; n_saved_regs]);
    fn __jump(ptr: *const [usize; n_saved_regs]) -> !;
}

#[derive(Debug)]
pub struct Jump<T>([usize; n_saved_regs], *mut Option<T>);

impl<T> Jump<T> {
    #[inline]
    pub unsafe fn new() -> (Self, Option<T>) {
        let mut opt_x = None;
        let mut jump = Jump(mem::uninitialized(), &mut opt_x);
        __mkjump(&mut jump.0);
        (jump, opt_x)
    }
}

impl<T> FnOnce<(T,)> for Jump<T> {
    type Output = Void;

    #[inline]
    extern "rust-call" fn call_once(self, a: (T,)) -> Void { self.call(a) }
}

impl<T> FnMut<(T,)> for Jump<T> {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, a: (T,)) -> Void { self.call(a) }
}

impl<T> Fn<(T,)> for Jump<T> {
    #[inline]
    extern "rust-call" fn call(&self, (x,): (T,)) -> Void { unsafe {
        ptr::write(self.1, Some(x));
        __jump(&self.0)
    } }
}

#[inline]
pub fn call_cc<T, F: FnOnce(&Jump<T>) -> T>(f: F) -> T {
    match unsafe { Jump::new() } {
        (jump, None) => f(&jump),
        (_, Some(x)) => x,
    }
}

#[test]
fn test() {
    let mut opt_y = None;
    let (jump, opt_x) = unsafe { Jump::new() };
    assert_eq!(opt_y, opt_x);
    if let Some(()) = opt_x { return }
    else { assert_eq!(None, opt_y) };
    opt_y = Some(());
    jump(());
}
