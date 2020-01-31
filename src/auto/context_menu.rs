// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use webkit2_sys;
use ContextMenuItem;

glib_wrapper! {
    pub struct ContextMenu(Object<webkit2_sys::WebKitContextMenu, webkit2_sys::WebKitContextMenuClass, ContextMenuClass>);

    match fn {
        get_type => || webkit2_sys::webkit_context_menu_get_type(),
    }
}

impl ContextMenu {
    pub fn new() -> ContextMenu {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_context_menu_new())
        }
    }

    pub fn new_with_items(items: &[ContextMenuItem]) -> ContextMenu {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_context_menu_new_with_items(items.to_glib_none().0))
        }
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CONTEXT_MENU: Option<&ContextMenu> = None;

pub trait ContextMenuExt: 'static {
    fn append<P: IsA<ContextMenuItem>>(&self, item: &P);

    fn first(&self) -> Option<ContextMenuItem>;

    fn get_item_at_position(&self, position: u32) -> Option<ContextMenuItem>;

    fn get_items(&self) -> Vec<ContextMenuItem>;

    fn get_n_items(&self) -> u32;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_user_data(&self) -> Option<glib::Variant>;

    fn insert<P: IsA<ContextMenuItem>>(&self, item: &P, position: i32);

    fn last(&self) -> Option<ContextMenuItem>;

    fn move_item<P: IsA<ContextMenuItem>>(&self, item: &P, position: i32);

    fn prepend<P: IsA<ContextMenuItem>>(&self, item: &P);

    fn remove<P: IsA<ContextMenuItem>>(&self, item: &P);

    fn remove_all(&self);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn set_user_data(&self, user_data: &glib::Variant);
}

impl<O: IsA<ContextMenu>> ContextMenuExt for O {
    fn append<P: IsA<ContextMenuItem>>(&self, item: &P) {
        unsafe {
            webkit2_sys::webkit_context_menu_append(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0);
        }
    }

    fn first(&self) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_context_menu_first(self.as_ref().to_glib_none().0))
        }
    }

    fn get_item_at_position(&self, position: u32) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_context_menu_get_item_at_position(self.as_ref().to_glib_none().0, position))
        }
    }

    fn get_items(&self) -> Vec<ContextMenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(webkit2_sys::webkit_context_menu_get_items(self.as_ref().to_glib_none().0))
        }
    }

    fn get_n_items(&self) -> u32 {
        unsafe {
            webkit2_sys::webkit_context_menu_get_n_items(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_user_data(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_context_menu_get_user_data(self.as_ref().to_glib_none().0))
        }
    }

    fn insert<P: IsA<ContextMenuItem>>(&self, item: &P, position: i32) {
        unsafe {
            webkit2_sys::webkit_context_menu_insert(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0, position);
        }
    }

    fn last(&self) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_context_menu_last(self.as_ref().to_glib_none().0))
        }
    }

    fn move_item<P: IsA<ContextMenuItem>>(&self, item: &P, position: i32) {
        unsafe {
            webkit2_sys::webkit_context_menu_move_item(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0, position);
        }
    }

    fn prepend<P: IsA<ContextMenuItem>>(&self, item: &P) {
        unsafe {
            webkit2_sys::webkit_context_menu_prepend(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0);
        }
    }

    fn remove<P: IsA<ContextMenuItem>>(&self, item: &P) {
        unsafe {
            webkit2_sys::webkit_context_menu_remove(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0);
        }
    }

    fn remove_all(&self) {
        unsafe {
            webkit2_sys::webkit_context_menu_remove_all(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn set_user_data(&self, user_data: &glib::Variant) {
        unsafe {
            webkit2_sys::webkit_context_menu_set_user_data(self.as_ref().to_glib_none().0, user_data.to_glib_none().0);
        }
    }
}

impl fmt::Display for ContextMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContextMenu")
    }
}
