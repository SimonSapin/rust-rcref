use std::ops::Deref;
use std::rc::Rc;

/// Something that holds a `std::rc::Rc` strong reference,
/// but derefs to some component inside of it.
pub struct RcRef<T, U> {
    _keep_alive: Rc<T>,
    _reference: *const U
}

impl<T, U> RcRef<T, U> {
    /// Create a `RcRef` for a component in a given `Rc`.
    pub fn new<F>(rc: Rc<T>, f: F) -> RcRef<T, U> where F: FnOnce(&T) -> &U {
        RcRef {
            _reference: f(&*rc),
            _keep_alive: rc,
        }
    }

    /// Create a `RcRef` for and a component that may or may not be in a given `Rc`.
    pub fn new_opt<F>(rc: Rc<T>, f: F) -> Option<RcRef<T, U>> where F: FnOnce(&T) -> Option<&U> {
        f(&*rc).map(|r| r as *const U).map(move |r| RcRef {
            _reference: r,
            _keep_alive: rc,
        })
    }
}

impl<T, U> Deref for RcRef<T, U> {
    type Target = U;
    fn deref(&self) -> &U { unsafe { &*self._reference } }
}

#[test]
fn it_works() {
    let a = Rc::new(Some(5));
    let b = RcRef::new(a, |opt| opt.as_ref().unwrap());
    assert_eq!(*b, 5);

    let a = Rc::new(Some(5));
    let b = RcRef::new_opt(a, |opt| opt.as_ref()).unwrap();
    assert_eq!(*b, 5);
}
