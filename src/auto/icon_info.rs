// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "futures")]
use futures::future;
use gdk;
use gdk_pixbuf;
use gio;
use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::ptr;
use Error;
use IconTheme;
use StyleContext;

glib_wrapper! {
    pub struct IconInfo(Object<gtk_sys::GtkIconInfo, gtk_sys::GtkIconInfoClass, IconInfoClass>);

    match fn {
        get_type => || gtk_sys::gtk_icon_info_get_type(),
    }
}

impl IconInfo {
    pub fn new_for_pixbuf<P: IsA<IconTheme>>(
        icon_theme: &P,
        pixbuf: &gdk_pixbuf::Pixbuf,
    ) -> IconInfo {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_info_new_for_pixbuf(
                icon_theme.as_ref().to_glib_none().0,
                pixbuf.to_glib_none().0,
            ))
        }
    }

    pub fn get_base_scale(&self) -> i32 {
        unsafe { gtk_sys::gtk_icon_info_get_base_scale(self.to_glib_none().0) }
    }

    pub fn get_base_size(&self) -> i32 {
        unsafe { gtk_sys::gtk_icon_info_get_base_size(self.to_glib_none().0) }
    }

    pub fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe { from_glib_none(gtk_sys::gtk_icon_info_get_filename(self.to_glib_none().0)) }
    }

    pub fn is_symbolic(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_icon_info_is_symbolic(self.to_glib_none().0)) }
    }

    pub fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_icon(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn load_icon_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn load_icon_async_trampoline<
            Q: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                gtk_sys::gtk_icon_info_load_icon_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_icon_async_trampoline::<Q>;
        unsafe {
            gtk_sys::gtk_icon_info_load_icon_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    pub fn load_icon_async_future(
        &self,
    ) -> Box_<dyn future::Future<Output = Result<gdk_pixbuf::Pixbuf, Error>> + std::marker::Unpin>
    {
        use fragile::Fragile;
        use gio::GioFuture;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.load_icon_async(Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    pub fn load_symbolic(
        &self,
        fg: &gdk::RGBA,
        success_color: Option<&gdk::RGBA>,
        warning_color: Option<&gdk::RGBA>,
        error_color: Option<&gdk::RGBA>,
    ) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        unsafe {
            let mut was_symbolic = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_symbolic(
                self.to_glib_none().0,
                fg.to_glib_none().0,
                success_color.to_glib_none().0,
                warning_color.to_glib_none().0,
                error_color.to_glib_none().0,
                was_symbolic.as_mut_ptr(),
                &mut error,
            );
            let was_symbolic = was_symbolic.assume_init();
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib(was_symbolic)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn load_symbolic_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static,
    >(
        &self,
        fg: &gdk::RGBA,
        success_color: Option<&gdk::RGBA>,
        warning_color: Option<&gdk::RGBA>,
        error_color: Option<&gdk::RGBA>,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn load_symbolic_async_trampoline<
            Q: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut was_symbolic = mem::MaybeUninit::uninit();
            let ret = gtk_sys::gtk_icon_info_load_symbolic_finish(
                _source_object as *mut _,
                res,
                was_symbolic.as_mut_ptr(),
                &mut error,
            );
            let was_symbolic = was_symbolic.assume_init();
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib(was_symbolic)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_symbolic_async_trampoline::<Q>;
        unsafe {
            gtk_sys::gtk_icon_info_load_symbolic_async(
                self.to_glib_none().0,
                fg.to_glib_none().0,
                success_color.to_glib_none().0,
                warning_color.to_glib_none().0,
                error_color.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    pub fn load_symbolic_async_future(
        &self,
        fg: &gdk::RGBA,
        success_color: Option<&gdk::RGBA>,
        warning_color: Option<&gdk::RGBA>,
        error_color: Option<&gdk::RGBA>,
    ) -> Box_<
        dyn future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), Error>> + std::marker::Unpin,
    > {
        use fragile::Fragile;
        use gio::GioFuture;

        let fg = fg.clone();
        let success_color = success_color.map(ToOwned::to_owned);
        let warning_color = warning_color.map(ToOwned::to_owned);
        let error_color = error_color.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.load_symbolic_async(
                &fg,
                success_color.as_ref().map(::std::borrow::Borrow::borrow),
                warning_color.as_ref().map(::std::borrow::Borrow::borrow),
                error_color.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    pub fn load_symbolic_for_context<P: IsA<StyleContext>>(
        &self,
        context: &P,
    ) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        unsafe {
            let mut was_symbolic = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_symbolic_for_context(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                was_symbolic.as_mut_ptr(),
                &mut error,
            );
            let was_symbolic = was_symbolic.assume_init();
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib(was_symbolic)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn load_symbolic_for_context_async<
        P: IsA<StyleContext>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static,
    >(
        &self,
        context: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn load_symbolic_for_context_async_trampoline<
            R: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut was_symbolic = mem::MaybeUninit::uninit();
            let ret = gtk_sys::gtk_icon_info_load_symbolic_for_context_finish(
                _source_object as *mut _,
                res,
                was_symbolic.as_mut_ptr(),
                &mut error,
            );
            let was_symbolic = was_symbolic.assume_init();
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib(was_symbolic)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_symbolic_for_context_async_trampoline::<R>;
        unsafe {
            gtk_sys::gtk_icon_info_load_symbolic_for_context_async(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    pub fn load_symbolic_for_context_async_future<P: IsA<StyleContext> + Clone + 'static>(
        &self,
        context: &P,
    ) -> Box_<
        dyn future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), Error>> + std::marker::Unpin,
    > {
        use fragile::Fragile;
        use gio::GioFuture;

        let context = context.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.load_symbolic_for_context_async(&context, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    pub fn load_texture(&self) -> Option<gdk::Texture> {
        unsafe { from_glib_full(gtk_sys::gtk_icon_info_load_texture(self.to_glib_none().0)) }
    }
}

impl fmt::Display for IconInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconInfo")
    }
}
