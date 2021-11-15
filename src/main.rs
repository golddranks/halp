use std::{ops::Not, sync::atomic::{AtomicBool, AtomicU32, Ordering}};

mod gl;
mod objc;

use objc::{Id, NSUInteger, Sel, sel, class, msg, cstr, NSApp, NSInteger, Imp, NSRect, NSPoint, CGSize, FALSE, TRUE, NIL, NSDefaultRunLoopMode};

use crate::objc::{make, msg_id, msg_void};


static TERMINATED: AtomicBool = AtomicBool::new(false);
static WINDOW_COUNT: AtomicU32 = AtomicU32::new(0);

fn application_should_terminate(_: Id, _: Sel, _: Id) -> NSUInteger {
	println!("Requested to terminate.");
	TERMINATED.store(true, Ordering::Relaxed);
	NSUInteger(0)
}

fn window_will_close(_: Id, _: Sel, _: Id) {
	println!("Window will close");
    if WINDOW_COUNT.fetch_sub(1, Ordering::Relaxed) == 0 {
        TERMINATED.store(true, Ordering::Relaxed);
    }
}

fn main() {

	let alloc = sel!("alloc");
	let init = sel!("init");
	let autorelease = sel!("autorelease");
    let pool_alloc = msg_id!("NSAutoreleasePool", alloc);
    let pool = msg_id!(pool_alloc, init);
	msg_id!("NSApplication", "sharedApplication");
	msg_void!(NSApp, "setActivationPolicy:", NSInteger(0));

    #[allow(non_snake_case)]
	let AppDelegate = objc::allocate_class_pair(class!("NSObject"), cstr!("AppDelegate"), 0);
	AppDelegate.add_protocol(
        objc::get_protocol(cstr!("NSApplicationDelegate"))
    ).assert_true();
	AppDelegate.add_method(
        sel!("applicationShouldTerminate:"),
        Imp::from_fn_1(application_should_terminate),
        cstr!("L@:@")
    ).assert_true();

    let dg = make!(AppDelegate);
    msg_void!(NSApp, "setDelegate:", dg);

	// only needed if we don't use [NSApp run]
    msg_void!(NSApp, "finishLaunching");
    let menubar = make!("NSMenu");
	let app_menu_item = make!("NSMenuItem");
    msg_void!(menubar, "addItem:", app_menu_item);
	msg_id!(NSApp, "setMainMenu:", menubar);
    let app_menu = make!("NSMenu");
	let process_info = msg_id!("NSProcessInfo", "processInfo");
	let app_name = msg_id!(process_info, "processName");
	let quit_title_prefix_string = msg_id!("NSString", "stringWithUTF8String:", cstr!("Quit "));
	let quit_title = msg_id!(quit_title_prefix_string, "stringByAppendingString:", app_name);
	let quit_menu_item_key = msg_id!("NSString", "stringWithUTF8String:", cstr!("q"));
	let quit_menu_item_alloc = msg_id!("NSMenuItem", alloc);
	let init_with_title = sel!("initWithTitle:action:keyEquivalent:");
	let terminate = sel!("terminate:");
	let quit_menu_item = msg_id!(quit_menu_item_alloc, init_with_title, quit_title, terminate, quit_menu_item_key);
	msg_void!(quit_menu_item, autorelease);
	msg_void!(app_menu, "addItem:", quit_menu_item);
	msg_void!(app_menu_item, "setSubmenu:", app_menu);

    
	//id window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 500, 500) styleMask:NSTitledWindowMask | NSClosableWindowMask | NSMiniaturizableWindowMask | NSResizableWindowMask backing:NSBackingStoreBuffered defer:NO];
	let rect = NSRect {
        origin: NSPoint {x: 0.0, y: 0.0},
        size: CGSize {width: 500.0, height: 500.0}
    };
	let window_alloc = msg_id!("NSWindow", alloc);
	let init_with_content_rect = sel!("initWithContentRect:styleMask:backing:defer:");
	let window = msg_id!(window_alloc, init_with_content_rect, rect, NSUInteger(15), NSUInteger(2), FALSE);
	msg_void!(window, autorelease);

	// when we are not using ARC, than window will be added to autorelease pool
	// so if we close it by hand (pressing red button), we don't want it to be released for us
	// so it will be released by autorelease pool later
	//[window setReleasedWhenClosed:NO];
    msg_void!(window, "setReleasedWhenClosed:", FALSE);
	WINDOW_COUNT.store(1, Ordering::Relaxed);

    #[allow(non_snake_case)]
	let WindowDelegateClass = objc::allocate_class_pair(class!("NSObject"), cstr!("WindowDelegate"), 0);
    WindowDelegateClass.add_protocol(objc::get_protocol(cstr!("NSWindowDelegate"))).assert_true();
	WindowDelegateClass.add_method(sel!("windowWillClose:"), Imp::from_fn_1(window_will_close),  cstr!("v@:@")).assert_true();
	let wdg = make!(WindowDelegateClass);
	msg_void!(window, "setDelegate:", wdg);
	let content_view = msg_id!(window, "contentView");
	// disable this if you don't want retina support
	msg_void!(content_view, "setWantsBestResolutionOpenGLSurface:", TRUE);
	let point = NSPoint {x: 20.0, y: 20.0 };
	msg_void!(window, "cascadeTopLeftFromPoint:", point);
	let title_string = msg_id!("NSString", "stringWithUTF8String:", cstr!("sup from Rust"));
	msg_void!(window, "setTitle:", title_string);

	//NSOpenGLPixelFormatAttribute glAttributes[] =
	//{
	//	NSOpenGLPFAColorSize, 24,
	//	NSOpenGLPFAAlphaSize, 8,
	//	NSOpenGLPFADoubleBuffer,
	//	NSOpenGLPFAAccelerated,
	//	NSOpenGLPFANoRecovery,
	//	NSOpenGLPFASampleBuffers, 1,
	//	NSOpenGLPFASamples, 4,
	//	NSOpenGLPFAOpenGLProfile, NSOpenGLProfileVersionLegacy, // or NSOpenGLProfileVersion3_2Core
	//	0
	//};
	let gl_attributes: [u32; 14] =
        [
            8, 24,
            11, 8,
            5,
            73,
            72,
            55, 1,
            56, 4,
            99, 0x1000, // or 0x3200
            0
        ];

	let pixel_dormat_alloc = msg_id!("NSOpenGLPixelFormat", alloc);
	let pixel_dormat = msg_id!(pixel_dormat_alloc, "initWithAttributes:", gl_attributes.as_ptr());
	msg_void!(pixel_dormat, autorelease);

	let opengl_context_alloc = msg_id!("NSOpenGLContext", alloc);
	let opengl_context = msg_id!(opengl_context_alloc, "initWithFormat:shareContext:", pixel_dormat, NIL);
	msg_void!(opengl_context, autorelease);

	msg_void!(opengl_context, "setView:", content_view);
	msg_void!(window, "makeKeyAndOrderFront:", window);

	msg_void!(window, "setAcceptsMouseMovedEvents:", TRUE);

	let black_color = msg_id!("NSColor", "blackColor");
	msg_void!(window, "setBackgroundColor:", black_color);

	// TODO do we really need this?
	msg_void!(NSApp, "activateIgnoringOtherApps:", TRUE);

	// explicit runloop
	println!("Entering runloop!");

    #[allow(non_snake_case)]
	let NSDateClass = class!("NSDate");
	let distant_past_sel= sel!("distantPast");
	let next_event_matching_mask_sel= sel!("nextEventMatchingMask:untilDate:inMode:dequeue:");
	let frame_sel = sel!("frame");
    /*
	let type_sel= sel!("type");
	let button_number_sel= sel!("buttonNumber");
	let key_code_sel= sel!("keyCode");
	let key_window_sel= sel!("keyWindow");
	let mouse_location_outside_of_event_stream_sel= sel!("mouseLocationOutsideOfEventStream");
	let convert_rect_to_backing_sel= sel!("convertRectToBacking:");
	let scrolling_delta_x_sel= sel!("scrollingDeltaX");
	let scrolling_delta_y_sel= sel!("scrollingDeltaY");
	let hasPreciseScrollingDeltas_sel= sel!("hasPreciseScrollingDeltas");
	let modifier_flags_sel= sel!("modifierFlags");
	let characters_sel= sel!("characters");
	let utf8_string_sel= sel!("UTF8String");
	let send_event_sel= sel!("sendEvent:");
	let update_windows_sel= sel!("updateWindows");
    */
	let update_sel= sel!("update");
	let make_current_context_sel= sel!("makeCurrentContext");
	let flush_buffer_sel= sel!("flushBuffer");

    let mut a = 0.0;

    while TERMINATED.load(Ordering::Relaxed).not() {
        		//NSEvent * event = [NSApp nextEventMatchingMask:NSAnyEventMask untilDate:[NSDate distantPast] inMode:NSDefaultRunLoopMode dequeue:YES];
		let distantPast = msg_id!(NSDateClass, distant_past_sel);
		let event = msg_id!(NSApp, next_event_matching_mask_sel, NSUInteger(u32::MAX), distantPast, NSDefaultRunLoopMode, TRUE);

        // EVENT HANDLING


		// do runloop stuff
		//[openGLContext update]; // probably we only need to do it when we resize the window
        msg_void!(opengl_context, update_sel);

		//[openGLContext makeCurrentContext];
        msg_void!(opengl_context, make_current_context_sel);

		//NSRect rect = [contentView frame];
		let rect: NSRect = msg!(content_view, frame_sel);

		//rect = [contentView convertRectToBacking:rect];
		let rect: NSRect = msg!(content_view, frame_sel, rect);

        unsafe { 
            gl::glViewport(0, 0, rect.size.width as i32, rect.size.height as i32);

            gl::glClearColor(0.0, 0.0, 1.0, 1.0);
            gl::glClear(gl::GL_COLOR_BUFFER_BIT);
            gl::glColor3f(1.0, 0.85, 0.35);
            gl::glBegin(gl::GL_TRIANGLES);
            {
                gl::glVertex3f(  a,  0.6, 0.0);
                gl::glVertex3f( -0.2, -0.3, 0.0);
                gl::glVertex3f(  0.2, -0.3-a/2. ,0.0);
            }
            gl::glEnd();
        };

        a += 0.01;

		//[openGLContext flushBuffer];
        msg_void!(opengl_context, flush_buffer_sel);

    }

	println!("Gracefully terminated.");

	msg_void!(pool, "drain");
}
