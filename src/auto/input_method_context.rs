// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::InputHints;
use crate::InputPurpose;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct InputMethodContext(Object<ffi::WebKitInputMethodContext, ffi::WebKitInputMethodContextClass>);

    match fn {
        type_ => || ffi::webkit_input_method_context_get_type(),
    }
}

pub const NONE_INPUT_METHOD_CONTEXT: Option<&InputMethodContext> = None;

pub trait InputMethodContextExt: 'static {
    #[doc(alias = "webkit_input_method_context_filter_key_event")]
    fn filter_key_event(&self, key_event: &mut gdk::EventKey) -> bool;

    #[doc(alias = "webkit_input_method_context_get_input_hints")]
    fn input_hints(&self) -> InputHints;

    #[doc(alias = "webkit_input_method_context_get_input_purpose")]
    fn input_purpose(&self) -> InputPurpose;

    //#[doc(alias = "webkit_input_method_context_get_preedit")]
    //fn preedit(&self, underlines: /*Unimplemented*/Vec<InputMethodUnderline>) -> (Option<glib::GString>, u32);

    #[doc(alias = "webkit_input_method_context_notify_cursor_area")]
    fn notify_cursor_area(&self, x: i32, y: i32, width: i32, height: i32);

    #[doc(alias = "webkit_input_method_context_notify_focus_in")]
    fn notify_focus_in(&self);

    #[doc(alias = "webkit_input_method_context_notify_focus_out")]
    fn notify_focus_out(&self);

    #[doc(alias = "webkit_input_method_context_notify_surrounding")]
    fn notify_surrounding(&self, text: &str, cursor_index: u32, selection_index: u32);

    #[doc(alias = "webkit_input_method_context_reset")]
    fn reset(&self);

    #[doc(alias = "webkit_input_method_context_set_enable_preedit")]
    fn set_enable_preedit(&self, enabled: bool);

    #[doc(alias = "webkit_input_method_context_set_input_hints")]
    fn set_input_hints(&self, hints: InputHints);

    #[doc(alias = "webkit_input_method_context_set_input_purpose")]
    fn set_input_purpose(&self, purpose: InputPurpose);

    fn get_property_input_hints(&self) -> InputHints;

    fn set_property_input_hints(&self, input_hints: InputHints);

    fn get_property_input_purpose(&self) -> InputPurpose;

    fn set_property_input_purpose(&self, input_purpose: InputPurpose);

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_committed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_delete_surrounding<F: Fn(&Self, i32, u32) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_preedit_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_preedit_started<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_input_purpose_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<InputMethodContext>> InputMethodContextExt for O {
    fn filter_key_event(&self, key_event: &mut gdk::EventKey) -> bool {
        unsafe {
            from_glib(ffi::webkit_input_method_context_filter_key_event(
                self.as_ref().to_glib_none().0,
                key_event.to_glib_none_mut().0,
            ))
        }
    }

    fn input_hints(&self) -> InputHints {
        unsafe {
            from_glib(ffi::webkit_input_method_context_get_input_hints(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn input_purpose(&self) -> InputPurpose {
        unsafe {
            from_glib(ffi::webkit_input_method_context_get_input_purpose(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn preedit(&self, underlines: /*Unimplemented*/Vec<InputMethodUnderline>) -> (Option<glib::GString>, u32) {
    //    unsafe { TODO: call ffi:webkit_input_method_context_get_preedit() }
    //}

    fn notify_cursor_area(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::webkit_input_method_context_notify_cursor_area(
                self.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    fn notify_focus_in(&self) {
        unsafe {
            ffi::webkit_input_method_context_notify_focus_in(self.as_ref().to_glib_none().0);
        }
    }

    fn notify_focus_out(&self) {
        unsafe {
            ffi::webkit_input_method_context_notify_focus_out(self.as_ref().to_glib_none().0);
        }
    }

    fn notify_surrounding(&self, text: &str, cursor_index: u32, selection_index: u32) {
        let length = text.len() as i32;
        unsafe {
            ffi::webkit_input_method_context_notify_surrounding(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
                length,
                cursor_index,
                selection_index,
            );
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::webkit_input_method_context_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_enable_preedit(&self, enabled: bool) {
        unsafe {
            ffi::webkit_input_method_context_set_enable_preedit(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_input_hints(&self, hints: InputHints) {
        unsafe {
            ffi::webkit_input_method_context_set_input_hints(
                self.as_ref().to_glib_none().0,
                hints.to_glib(),
            );
        }
    }

    fn set_input_purpose(&self, purpose: InputPurpose) {
        unsafe {
            ffi::webkit_input_method_context_set_input_purpose(
                self.as_ref().to_glib_none().0,
                purpose.to_glib(),
            );
        }
    }

    fn get_property_input_hints(&self) -> InputHints {
        unsafe {
            let mut value = glib::Value::from_type(<InputHints as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"input-hints\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `input-hints` getter")
                .unwrap()
        }
    }

    fn set_property_input_hints(&self, input_hints: InputHints) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"input-hints\0".as_ptr() as *const _,
                glib::Value::from(&input_hints).to_glib_none().0,
            );
        }
    }

    fn get_property_input_purpose(&self) -> InputPurpose {
        unsafe {
            let mut value = glib::Value::from_type(<InputPurpose as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"input-purpose\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `input-purpose` getter")
                .unwrap()
        }
    }

    fn set_property_input_purpose(&self, input_purpose: InputPurpose) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"input-purpose\0".as_ptr() as *const _,
                glib::Value::from(&input_purpose).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_committed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn committed_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::WebKitInputMethodContext,
            text: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<InputMethodContext>,
        {
            let f: &F = &*(f as *const F);
            f(
                &InputMethodContext::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(text),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"committed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    committed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_delete_surrounding<F: Fn(&Self, i32, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn delete_surrounding_trampoline<P, F: Fn(&P, i32, u32) + 'static>(
            this: *mut ffi::WebKitInputMethodContext,
            offset: libc::c_int,
            n_chars: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<InputMethodContext>,
        {
            let f: &F = &*(f as *const F);
            f(
                &InputMethodContext::from_glib_borrow(this).unsafe_cast_ref(),
                offset,
                n_chars,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"delete-surrounding\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    delete_surrounding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitInputMethodContext,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<InputMethodContext>,
        {
            let f: &F = &*(f as *const F);
            f(&InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    preedit_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_preedit_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_finished_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitInputMethodContext,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<InputMethodContext>,
        {
            let f: &F = &*(f as *const F);
            f(&InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    preedit_finished_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn connect_preedit_started<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_started_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitInputMethodContext,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<InputMethodContext>,
        {
            let f: &F = &*(f as *const F);
            f(&InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-started\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    preedit_started_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_hints_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitInputMethodContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<InputMethodContext>,
        {
            let f: &F = &*(f as *const F);
            f(&InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-hints\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_hints_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_input_purpose_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_purpose_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitInputMethodContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<InputMethodContext>,
        {
            let f: &F = &*(f as *const F);
            f(&InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-purpose\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_purpose_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for InputMethodContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InputMethodContext")
    }
}
