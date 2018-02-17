// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use ffi;
use glib;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct EditorState(Object<ffi::WebKitEditorState, ffi::WebKitEditorStateClass>);

    match fn {
        get_type => || ffi::webkit_editor_state_get_type(),
    }
}

pub trait EditorStateExt {
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_typing_attributes(&self) -> u32;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_typing_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EditorState> + IsA<glib::object::Object>> EditorStateExt for O {
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_typing_attributes(&self) -> u32 {
        unsafe {
            ffi::webkit_editor_state_get_typing_attributes(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_typing_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::typing-attributes",
                transmute(notify_typing_attributes_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_typing_attributes_trampoline<P>(this: *mut ffi::WebKitEditorState, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EditorState> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EditorState::from_glib_borrow(this).downcast_unchecked())
}
