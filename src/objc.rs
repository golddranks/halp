use std::ffi::{CStr, c_void};
use std::mem;
use std::os::raw::{c_char};
use std::ptr::null;

pub const NIL: *const fn() = null();

#[allow(non_camel_case_types)]
pub type c_size_t = usize; // When std::c_size_t stabilizes, use it

pub type CGFloat = f64;
pub type NSPoint = CGPoint;
pub type NSRect = CGRect;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

#[repr(transparent)]
pub struct Imp(pub unsafe extern fn());

impl Imp {
    pub fn from_fn_1<R, S, A>(f: fn(S, Sel, A) -> R) -> Imp {
        Imp(unsafe { mem::transmute(f) })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Bool(c_char);

impl Bool {
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }

    pub fn assert_true(self) {
        assert!(self.as_bool())
    }
}

pub const FALSE: Bool = Bool(0);
pub const TRUE: Bool = Bool(1);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sel(*const c_void);

#[repr(transparent)]
pub struct NSInteger(pub i32);

#[repr(transparent)]
pub struct NSUInteger(pub u32);

#[repr(transparent)]
pub struct Protocol(objc_object);

#[repr(C)]
pub struct objc_object{
    isa: Class,
}

#[repr(C)]
pub struct objc_class{
    _private: [u8; 0],
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Class(*mut objc_class);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Id(*mut objc_object);

extern {
    pub fn sel_registerName(name: *const c_char) -> Sel;
    pub fn objc_getClass(name: *const c_char) -> Class;
    pub fn objc_allocateClassPair(supr: Class, name: *const c_char, extra_bytes: c_size_t) -> Class;
    pub fn objc_getProtocol(name: *const c_char) -> *mut Protocol;
    pub fn class_addProtocol(class: Class, protocol: *mut Protocol) -> Bool;
    pub fn class_addMethod(class: Class, name: Sel, imp: Imp, types: *const c_char) -> Bool;
    pub fn objc_msgSend();
    pub fn objc_msgSend_stret();
    pub static NSApp: Id;
    pub static NSDefaultRunLoopMode: Id;
}

impl Class {
    pub fn add_protocol(self, protocol: *mut Protocol) -> Bool {
        unsafe { class_addProtocol(self, protocol) }
    }

    pub fn add_method(self, name: Sel, imp: Imp, types: &CStr) -> Bool {
        unsafe { class_addMethod(self, name, imp, types.as_ptr()) }
    }
}

pub fn sel_register_name(name: &CStr) -> Sel {
    unsafe { sel_registerName(name.as_ptr()) }
}

pub fn get_class(name: &CStr) -> Class {
    unsafe { objc_getClass(name.as_ptr()) }
}

pub fn allocate_class_pair(supr: Class, name: &CStr, extra_bytes: c_size_t) -> Class {
    unsafe { objc_allocateClassPair(supr, name.as_ptr(), extra_bytes) }
}

pub fn get_protocol(name: &CStr) -> *mut Protocol {
    unsafe { objc_getProtocol(name.as_ptr()) }
}

#[cfg(target_arch = "x86_64")]
fn msg_send_trampoline_ptr<R>() -> unsafe extern fn() {
    if mem::size_of::<R>() <= 16 {
        objc_msgSend
    } else {
        objc_msgSend_stret
    }
}

#[cfg(target_arch = "aarch64")]
fn msg_send_trampoline_ptr<R>() -> unsafe extern fn() {
    objc_msgSend
}

pub fn msg_send_fn_0<R, S>() -> unsafe extern fn(S, Sel) -> R {
    unsafe { mem::transmute(msg_send_trampoline_ptr::<R>()) }
}

pub fn msg_send_fn_1<R, S, A>() -> unsafe extern fn(S, Sel, A) -> R {
    unsafe { mem::transmute(msg_send_trampoline_ptr::<R>()) }
}

pub fn msg_send_fn_2<R, S, A, B>() -> unsafe extern fn(S, Sel, A, B) -> R {
    unsafe { mem::transmute(msg_send_trampoline_ptr::<R>()) }
}

pub fn msg_send_fn_3<R, S, A, B, C>() -> unsafe extern fn(S, Sel, A, B, C) -> R {
    unsafe { mem::transmute(msg_send_trampoline_ptr::<R>()) }
}

pub fn msg_send_fn_4<R, S, A, B, C, D>() -> unsafe extern fn(S, Sel, A, B, C, D) -> R {
    unsafe { mem::transmute(msg_send_trampoline_ptr::<R>()) }
}

macro_rules! cstr {
    ($s:expr) => {
        ::std::ffi::CStr::from_bytes_with_nul(concat!($s, "\0").as_bytes()).unwrap()
    };
}

macro_rules! class {
    ($name:expr) => {
        $crate::objc::get_class(crate::objc::cstr!($name))
    };
}

macro_rules! sel {
    ($name:expr) => {
        $crate::objc::sel_register_name(crate::objc::cstr!($name))
    };
}

macro_rules! msg {
    ($obj:literal, $($tail:tt)*) => {
        {
            let cls = class!($obj);
            $crate::objc::msg!(cls, $($tail)*)
        }
    };
    ($obj:ident, $sel:literal $(, $tail:expr)*) => {
        {
            let sel = sel!($sel);
            $crate::objc::msg!($obj, sel $(, $tail)*)
        }
    };
    ($obj:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr) => {
        unsafe { $crate::objc::msg_send_fn_4()($obj, $sel, $a, $b, $c, $d) }
    };
    ($obj:ident, $sel:ident, $a:expr, $b:expr, $c:expr) => {
        unsafe { $crate::objc::msg_send_fn_3()($obj, $sel, $a, $b, $c) }
    };
    ($obj:ident, $sel:ident, $a:expr, $b:expr) => {
        unsafe { $crate::objc::msg_send_fn_2()($obj, $sel, $a, $b) }
    };
    ($obj:ident, $sel:ident, $a:expr) => {
        unsafe { $crate::objc::msg_send_fn_1()($obj, $sel, $a) }
    };
    ($obj:ident, $sel:ident) => {
        unsafe { $crate::objc::msg_send_fn_0()($obj, $sel) }
    };
}

macro_rules! msg_void {
    ($obj:tt, $sel:tt $(, $tail:expr)*) => {
        {
            let _: () = msg!($obj, $sel $(, $tail)*);
        }
    };
}

macro_rules! msg_id {
    ($obj:tt, $sel:tt $(, $tail:expr)*) => {
        {
            let id: $crate::objc::Id = msg!($obj, $sel $(, $tail)*);
            id
        }
    };
}

macro_rules! make {
    ($class:literal) => {
        {
            let alloc = msg_id!($class, "alloc");
            let init = msg_id!(alloc, "init");
            init
        }
    };
    ($class:ident) => {
        {
            let alloc = msg_id!($class, "alloc");
            let init = msg_id!(alloc, "init");
            msg_void!(init, "autorelease");
            init
        }
    };
}

pub(crate) use class;
pub(crate) use sel;
pub(crate) use cstr;
pub(crate) use msg;
pub(crate) use msg_void;
pub(crate) use msg_id;
pub(crate) use make;
