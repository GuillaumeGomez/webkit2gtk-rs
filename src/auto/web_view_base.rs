// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct WebViewBase(Object<ffi::WebKitWebViewBase, ffi::WebKitWebViewBaseClass>): [
        gtk::Container => gtk_ffi::GtkContainer,
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::webkit_web_view_base_get_type(),
    }
}

impl WebViewBase {}
