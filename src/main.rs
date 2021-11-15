use std::{ops::Not, sync::atomic::{AtomicBool, AtomicU32, Ordering}};

mod gl;
mod objc;

use objc::{Id, NSUInteger, Sel, sel, class, msg, cstr, NSApp, NSInteger, Imp, NSRect, NSPoint, CGSize, FALSE, TRUE, NIL, NSDefaultRunLoopMode};


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

	//NSAutoreleasePool * pool = [[NSAutoreleasePool alloc] init];
	let NSAutoreleasePool = class!("NSAutoreleasePool");
	let pool_alloc: Id = unsafe { msg!(NSAutoreleasePool, alloc) };
	let pool: Id = unsafe { msg!(pool_alloc, init) };

	//[NSApplication sharedApplication];
	let NSApplication = class!("NSApplication");
	let shared_application = sel!("sharedApplication");
	let _: Id = unsafe { msg!(NSApplication, shared_application) };

	//[NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];
	let set_activation_policy = sel!("setActivationPolicy:");
	let _: () = unsafe { msg!(NSApp, set_activation_policy, NSInteger(0)) };

	//AppDelegate * dg = [[AppDelegate alloc] init];
	let NSObjectClass = class!("NSObject");
	let AppDelegateClass = objc::allocate_class_pair(NSObjectClass, cstr!("AppDelegate"), 0);
	let NSApplicationDelegate = objc::get_protocol(cstr!("NSApplicationDelegate"));
	let resultAddProtoc = AppDelegateClass.add_protocol(NSApplicationDelegate);
	assert!(resultAddProtoc.as_bool());
	let applicationShouldTerminate = sel!("applicationShouldTerminate:");
	let resultAddMethod = AppDelegateClass.add_method(applicationShouldTerminate, Imp::from_fn_1(application_should_terminate), cstr!("L@:@"));
	assert!(resultAddMethod.as_bool());
	let dg_alloc: Id = unsafe{ msg!(AppDelegateClass, alloc) };
	let dg: Id = unsafe{ msg!(dg_alloc, init) };

	let autorelease = sel!("autorelease");
	let _: () = unsafe{ msg!(dg, autorelease) };

	//[NSApp setDelegate:dg];
    let setDelegate = sel!("setDelegate:");
    let _: () = unsafe{ msg!(NSApp, setDelegate, dg) };

	// only needed if we don't use [NSApp run]
	//[NSApp finishLaunching];
	let finishLaunching = sel!("finishLaunching");
    let _: () = unsafe{ msg!(NSApp, finishLaunching) };

	//id menubar = [[NSMenu alloc] init];
    let NSMenuClass = class!("NSMenu");
    let menubarAlloc: Id = unsafe{ msg!(NSMenuClass, alloc) };
    let menubar: Id = unsafe{ msg!(menubarAlloc, init) };
    let _: () = unsafe{ msg!(menubar, autorelease) };

	//id appMenuItem = [[NSMenuItem alloc] init];
	let NSMenuItemClass = class!("NSMenuItem");
	let appMenuItemAlloc: Id = unsafe{ msg!(NSMenuItemClass, alloc) };
	let appMenuItem: Id = unsafe{ msg!(appMenuItemAlloc, init) };
    let _: () = unsafe{ msg!(appMenuItem, autorelease) };

	//[menubar addItem:appMenuItem];
	let addItem = sel!("addItem:");
    let _: () = unsafe{ msg!(menubar, addItem, appMenuItem) };

	//[NSApp setMainMenu:menubar];
	let setMainMenu = sel!("setMainMenu:");
	let _: Id = unsafe{ msg!(NSApp, setMainMenu, menubar) };

	//id appMenu = [[NSMenu alloc] init];
	let appMenuAlloc: Id = unsafe{ msg!(NSMenuClass, alloc) };
	let appMenu: Id = unsafe{ msg!(appMenuAlloc, init) };
    let _: () = unsafe{ msg!(appMenu, autorelease) };

	//id appName = [[NSProcessInfo processInfo] processName];
	let NSProcessInfoClass = class!("NSProcessInfo");
	let processInfo = sel!("processInfo");
	let processInfo: Id = unsafe{ msg!(NSProcessInfoClass, processInfo) };
	let processName = sel!("processName");
	let appName: Id = unsafe{ msg!(processInfo, processName) };

	//id quitTitle = [@"Quit " stringByAppendingString:appName];
	let NSStringClass = class!("NSString");
	let stringWithUTF8String = sel!("stringWithUTF8String:");
	let quitTitlePrefixString: Id = unsafe{ msg!(NSStringClass, stringWithUTF8String, cstr!("Quit ")) };
	let stringByAppendingString = sel!("stringByAppendingString:");
	let quitTitle: Id = unsafe { msg!(quitTitlePrefixString, stringByAppendingString, appName) };

	//id quitMenuItem = [[NSMenuItem alloc] initWithTitle:quitTitle action:@selector(terminate:) keyEquivalent:@"q"];
	let quitMenuItemKey: Id = unsafe { msg!(NSStringClass, stringWithUTF8String, cstr!("q")) };
	let quitMenuItemAlloc: Id = unsafe { msg!(NSMenuItemClass, alloc) };
	let initWithTitle = sel!("initWithTitle:action:keyEquivalent:");
	let terminate = sel!("terminate:");
	let quitMenuItem: Id = unsafe { msg!(quitMenuItemAlloc, initWithTitle, quitTitle, terminate, quitMenuItemKey) };
	let _: () = unsafe { msg!(quitMenuItem, autorelease) };

	//[appMenu addItem:quitMenuItem];
	let _: () = unsafe { msg!(appMenu, addItem, quitMenuItem) };

	//[appMenuItem setSubmenu:appMenu];
	let setSubmenu = sel!("setSubmenu:");
	let _: () = unsafe { msg!(appMenuItem, setSubmenu, appMenu) };

    
	//id window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 500, 500) styleMask:NSTitledWindowMask | NSClosableWindowMask | NSMiniaturizableWindowMask | NSResizableWindowMask backing:NSBackingStoreBuffered defer:NO];
	let rect = NSRect {
        origin: NSPoint {x: 0.0, y: 0.0},
        size: CGSize {width: 500.0, height: 500.0}
    };
	let NSWindowClass = class!("NSWindow");
	let windowAlloc: Id = unsafe { msg!(NSWindowClass, alloc) };
	let initWithContentRect = sel!("initWithContentRect:styleMask:backing:defer:");
	let window: Id = unsafe { msg!(windowAlloc, initWithContentRect, rect, NSUInteger(15), NSUInteger(2), FALSE) };
	let _: () = unsafe { msg!(window, autorelease) };

	// when we are not using ARC, than window will be added to autorelease pool
	// so if we close it by hand (pressing red button), we don't want it to be released for us
	// so it will be released by autorelease pool later
	//[window setReleasedWhenClosed:NO];
	let setReleasedWhenClosed = sel!("setReleasedWhenClosed:");
    let _: () = unsafe { msg!(window, setReleasedWhenClosed, FALSE) };

	WINDOW_COUNT.store(1, Ordering::Relaxed);

	//WindowDelegate * wdg = [[WindowDelegate alloc] init];
	let WindowDelegateClass = objc::allocate_class_pair(NSObjectClass, cstr!("WindowDelegate"), 0);
    let NSWindowDelegateProtocol = objc::get_protocol(cstr!("NSWindowDelegate"));
    let resultAddProtoc = WindowDelegateClass.add_protocol(NSWindowDelegateProtocol);
	assert!(resultAddProtoc.as_bool());
	let windowWillClose = sel!("windowWillClose:");
	let resultAddMethod = WindowDelegateClass.add_method(windowWillClose, Imp::from_fn_1(window_will_close),  cstr!("v@:@"));
	assert!(resultAddMethod.as_bool());
	let wdgAlloc: Id = unsafe { msg!(WindowDelegateClass, alloc) };
	let wdg: Id = unsafe { msg!(wdgAlloc, init) };
	let _: () = unsafe { msg!(wdg, autorelease) };

	//[window setDelegate:wdg];
	let _: () = unsafe { msg!(window, setDelegate, wdg) };

	//NSView * contentView = [window contentView];
	let contentViewSel = sel!("contentView");
	let contentView: Id = unsafe { msg!(window, contentViewSel) };

	// disable this if you don't want retina support
	//[contentView setWantsBestResolutionOpenGLSurface:YES];
	let setWantsBestResolutionOpenGLSurface = sel!("setWantsBestResolutionOpenGLSurface:");
	let _: () = unsafe { msg!(contentView, setWantsBestResolutionOpenGLSurface, TRUE) };


	//[window cascadeTopLeftFromPoint:NSMakePoint(20,20)];
	let point = NSPoint {x: 20.0, y: 20.0 };
	let cascadeTopLeftFromPoint = sel!("cascadeTopLeftFromPoint:");
	let _: () = unsafe { msg!(window, cascadeTopLeftFromPoint, point) };

	//[window setTitle:@"sup"];
	let titleString: Id = unsafe { msg!(NSStringClass, stringWithUTF8String, cstr!("sup from Rust")) };
	let setTitle = sel!("setTitle:");
	let _: () = unsafe { msg!(window, setTitle, titleString) };

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
	let glAttributes: [u32; 14] =
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

	//NSOpenGLPixelFormat * pixelFormat = [[NSOpenGLPixelFormat alloc] initWithAttributes:glAttributes];
	let NSOpenGLPixelFormatClass = class!("NSOpenGLPixelFormat");
	let pixelFormatAlloc: Id = unsafe { msg!(NSOpenGLPixelFormatClass, alloc) };
	let initWithAttributes = sel!("initWithAttributes:");
	let pixelFormat: Id = unsafe { msg!(pixelFormatAlloc, initWithAttributes, glAttributes.as_ptr()) };
	let _: () = unsafe { msg!(pixelFormat, autorelease) };

	//NSOpenGLContext * openGLContext = [[NSOpenGLContext alloc] initWithFormat:pixelFormat shareContext:nil];
	let NSOpenGLContextClass = class!("NSOpenGLContext");
	let openGLContextAlloc: Id = unsafe{ msg!(NSOpenGLContextClass, alloc) };
	let initWithFormat = sel!("initWithFormat:shareContext:");
	let openGLContext: Id = unsafe{msg!(openGLContextAlloc, initWithFormat, pixelFormat, NIL)};
	let _: () = unsafe { msg!(openGLContext, autorelease) };

	//[openGLContext setView:contentView];
	let setView = sel!("setView:");
	let _: () = unsafe { msg!(openGLContext, setView, contentView) };

	//[window makeKeyAndOrderFront:window];
	let makeKeyAndOrderFront = sel!("makeKeyAndOrderFront:");
	let _: () = unsafe { msg!(window, makeKeyAndOrderFront, window) };

	//[window setAcceptsMouseMovedEvents:YES];
	let setAcceptsMouseMovedEvents = sel!("setAcceptsMouseMovedEvents:");
	let _: () = unsafe { msg!(window, setAcceptsMouseMovedEvents, TRUE) };

	//[window setBackgroundColor:[NSColor blackColor]];
	let NSColorClass = class!("NSColor");
    let blackColorSel = sel!("blackColor");
	let blackColor: Id = unsafe { msg!(NSColorClass, blackColorSel) };
	let setBackgroundColor = sel!("setBackgroundColor:");
	let _: () = unsafe { msg!(window, setBackgroundColor, blackColor) };

	// TODO do we really need this?
	//[NSApp activateIgnoringOtherApps:YES];
	let activateIgnoringOtherApps = sel!("activateIgnoringOtherApps:");
	let _: () = unsafe { msg!(NSApp, activateIgnoringOtherApps, TRUE) };

	// explicit runloop
	println!("Entering runloop!");

	let NSDateClass = class!("NSDate");
	let distantPastSel = sel!("distantPast");
	let nextEventMatchingMaskSel = sel!("nextEventMatchingMask:untilDate:inMode:dequeue:");
	let frameSel = sel!("frame");
	let typeSel = sel!("type");
	let buttonNumberSel = sel!("buttonNumber");
	let keyCodeSel = sel!("keyCode");
	let keyWindowSel = sel!("keyWindow");
	let mouseLocationOutsideOfEventStreamSel = sel!("mouseLocationOutsideOfEventStream");
	let convertRectToBackingSel = sel!("convertRectToBacking:");
	let scrollingDeltaXSel = sel!("scrollingDeltaX");
	let scrollingDeltaYSel = sel!("scrollingDeltaY");
	let hasPreciseScrollingDeltasSel = sel!("hasPreciseScrollingDeltas");
	let modifierFlagsSel = sel!("modifierFlags");
	let charactersSel = sel!("characters");
	let UTF8StringSel = sel!("UTF8String");
	let sendEventSel = sel!("sendEvent:");
	let updateWindowsSel = sel!("updateWindows");
	let updateSel = sel!("update");
	let makeCurrentContextSel = sel!("makeCurrentContext");
	let flushBufferSel = sel!("flushBuffer");


    while TERMINATED.load(Ordering::Relaxed).not() {
        		//NSEvent * event = [NSApp nextEventMatchingMask:NSAnyEventMask untilDate:[NSDate distantPast] inMode:NSDefaultRunLoopMode dequeue:YES];
		let distantPast: Id = unsafe{ msg!(NSDateClass, distantPastSel) };
		let event: Id = unsafe { msg!(NSApp, nextEventMatchingMaskSel, NSUInteger(u32::MAX), distantPast, NSDefaultRunLoopMode, TRUE) };

        // EVENT HANDLING


		// do runloop stuff
		//[openGLContext update]; // probably we only need to do it when we resize the window
        let _: () = unsafe { msg!(openGLContext, updateSel) };

		//[openGLContext makeCurrentContext];
        let _: () = unsafe { msg!(openGLContext, makeCurrentContextSel) };

		//NSRect rect = [contentView frame];
		let rect: NSRect = unsafe { msg!(contentView, frameSel) };

		//rect = [contentView convertRectToBacking:rect];
		let rect: NSRect = unsafe { msg!(contentView, frameSel, rect) };

        unsafe { 
            gl::glViewport(0, 0, rect.size.width as i32, rect.size.height as i32);

            gl::glClearColor(0.0, 0.0, 1.0, 1.0);
            gl::glClear(gl::GL_COLOR_BUFFER_BIT);
            gl::glColor3f(1.0, 0.85, 0.35);
            gl::glBegin(gl::GL_TRIANGLES);
            {
                gl::glVertex3f(  0.0,  0.6, 0.0);
                gl::glVertex3f( -0.2, -0.3, 0.0);
                gl::glVertex3f(  0.2, -0.3 ,0.0);
            }
            gl::glEnd();
        };

		//[openGLContext flushBuffer];
        let _: () = unsafe { msg!(openGLContext, flushBufferSel) };

    }

	println!("Gracefully terminated.");

	//[pool drain];
    let drain = sel!("drain");
	let _: () = unsafe { msg!(pool, drain) };
}
