// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

#[cfg(any(feature = "v2_6", feature = "dox"))]
use NavigationType;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use URIRequest;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct NavigationAction(Boxed<ffi::WebKitNavigationAction>);

    match fn {
        copy => |ptr| ffi::webkit_navigation_action_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_navigation_action_free(ptr),
        get_type => || ffi::webkit_navigation_action_get_type(),
    }
}

impl NavigationAction {
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn get_modifiers(&self) -> u32 {
        unsafe {
            ffi::webkit_navigation_action_get_modifiers(mut_override(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn get_mouse_button(&self) -> u32 {
        unsafe {
            ffi::webkit_navigation_action_get_mouse_button(mut_override(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn get_navigation_type(&self) -> NavigationType {
        unsafe {
            from_glib(ffi::webkit_navigation_action_get_navigation_type(mut_override(self.to_glib_none().0)))
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn get_request(&self) -> Option<URIRequest> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_action_get_request(mut_override(self.to_glib_none().0)))
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn is_user_gesture(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_navigation_action_is_user_gesture(mut_override(self.to_glib_none().0)))
        }
    }
}
