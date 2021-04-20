// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ApplicationInfo;
use crate::WebView;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct AutomationSession(Object<ffi::WebKitAutomationSession, ffi::WebKitAutomationSessionClass>);

    match fn {
        type_ => || ffi::webkit_automation_session_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct AutomationSessionBuilder {
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    id: Option<String>,
}

impl AutomationSessionBuilder {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn build(self) -> AutomationSession {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_18", feature = "dox"))]
if let Some(ref id) = self.id {
                properties.push(("id", id));
            }
        let ret = glib::Object::new::<AutomationSession>(&properties)
            .expect("object new");
    ret
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
}

pub const NONE_AUTOMATION_SESSION: Option<&AutomationSession> = None;

pub trait AutomationSessionExt: 'static {
    #[doc(alias = "webkit_automation_session_get_application_info")]
    fn application_info(&self) -> Option<ApplicationInfo>;

    #[doc(alias = "webkit_automation_session_get_id")]
    fn id(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_automation_session_set_application_info")]
    fn set_application_info(&self, info: &ApplicationInfo);

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn connect_create_web_view<F: Fn(&Self) -> WebView + 'static>(&self, detail: Option<&str>, f: F) -> SignalHandlerId;
}

impl<O: IsA<AutomationSession>> AutomationSessionExt for O {
    fn application_info(&self) -> Option<ApplicationInfo> {
        unsafe {
            from_glib_none(ffi::webkit_automation_session_get_application_info(self.as_ref().to_glib_none().0))
        }
    }

    fn id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_automation_session_get_id(self.as_ref().to_glib_none().0))
        }
    }

    fn set_application_info(&self, info: &ApplicationInfo) {
        unsafe {
            ffi::webkit_automation_session_set_application_info(self.as_ref().to_glib_none().0, info.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn connect_create_web_view<F: Fn(&Self) -> WebView + 'static>(&self, detail: Option<&str>, f: F) -> SignalHandlerId {
        unsafe extern "C" fn create_web_view_trampoline<P, F: Fn(&P) -> WebView + 'static>(this: *mut ffi::WebKitAutomationSession, f: glib::ffi::gpointer) -> *mut ffi::WebKitWebView
            where P: IsA<AutomationSession>
        {
            let f: &F = &*(f as *const F);
            f(&AutomationSession::from_glib_borrow(this).unsafe_cast_ref())/*Not checked*/.to_glib_none().0
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| { format!("create-web-view::{}\0", name) });
            let signal_name: &[u8] = detailed_signal_name.as_ref().map_or(&b"create-web-view\0"[..], |n| n.as_bytes());
            connect_raw(self.as_ptr() as *mut _, signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(create_web_view_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for AutomationSession {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AutomationSession")
    }
}
