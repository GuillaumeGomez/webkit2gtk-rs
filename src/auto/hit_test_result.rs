// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use std::fmt;
use webkit2_sys;

glib_wrapper! {
    pub struct HitTestResult(Object<webkit2_sys::WebKitHitTestResult, webkit2_sys::WebKitHitTestResultClass, HitTestResultClass>);

    match fn {
        get_type => || webkit2_sys::webkit_hit_test_result_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct HitTestResultBuilder {
    context: Option<u32>,
    image_uri: Option<String>,
    link_label: Option<String>,
    link_title: Option<String>,
    link_uri: Option<String>,
    media_uri: Option<String>,
}

impl HitTestResultBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> HitTestResult {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref context) = self.context {
            properties.push(("context", context));
        }
        if let Some(ref image_uri) = self.image_uri {
            properties.push(("image-uri", image_uri));
        }
        if let Some(ref link_label) = self.link_label {
            properties.push(("link-label", link_label));
        }
        if let Some(ref link_title) = self.link_title {
            properties.push(("link-title", link_title));
        }
        if let Some(ref link_uri) = self.link_uri {
            properties.push(("link-uri", link_uri));
        }
        if let Some(ref media_uri) = self.media_uri {
            properties.push(("media-uri", media_uri));
        }
        let ret = glib::Object::new(HitTestResult::static_type(), &properties)
            .expect("object new")
            .downcast::<HitTestResult>()
            .expect("downcast");
        ret
    }

    pub fn context(mut self, context: u32) -> Self {
        self.context = Some(context);
        self
    }

    pub fn image_uri(mut self, image_uri: &str) -> Self {
        self.image_uri = Some(image_uri.to_string());
        self
    }

    pub fn link_label(mut self, link_label: &str) -> Self {
        self.link_label = Some(link_label.to_string());
        self
    }

    pub fn link_title(mut self, link_title: &str) -> Self {
        self.link_title = Some(link_title.to_string());
        self
    }

    pub fn link_uri(mut self, link_uri: &str) -> Self {
        self.link_uri = Some(link_uri.to_string());
        self
    }

    pub fn media_uri(mut self, media_uri: &str) -> Self {
        self.media_uri = Some(media_uri.to_string());
        self
    }
}

pub const NONE_HIT_TEST_RESULT: Option<&HitTestResult> = None;

pub trait HitTestResultExt: 'static {
    fn context_is_editable(&self) -> bool;

    fn context_is_image(&self) -> bool;

    fn context_is_link(&self) -> bool;

    fn context_is_media(&self) -> bool;

    fn context_is_scrollbar(&self) -> bool;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn context_is_selection(&self) -> bool;

    fn get_context(&self) -> u32;

    fn get_image_uri(&self) -> Option<GString>;

    fn get_link_label(&self) -> Option<GString>;

    fn get_link_title(&self) -> Option<GString>;

    fn get_link_uri(&self) -> Option<GString>;

    fn get_media_uri(&self) -> Option<GString>;
}

impl<O: IsA<HitTestResult>> HitTestResultExt for O {
    fn context_is_editable(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_editable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context_is_image(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context_is_link(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_link(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context_is_media(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context_is_scrollbar(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_scrollbar(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn context_is_selection(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_selection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_context(&self) -> u32 {
        unsafe { webkit2_sys::webkit_hit_test_result_get_context(self.as_ref().to_glib_none().0) }
    }

    fn get_image_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_image_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_link_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_link_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_link_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_link_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_link_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_link_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_media_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_media_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for HitTestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HitTestResult")
    }
}
