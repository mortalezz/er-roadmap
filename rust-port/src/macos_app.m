#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>

extern char* rust_get_initial_html(void);
extern char* rust_handle_ipc(const char* action, const char* payload);
extern char* rust_export_json(void);
extern char* rust_import_json(const char* json_str);
extern void rust_free_string(char* ptr);

static NSString* js_escape_string(const char* raw_utf8) {
    if (!raw_utf8) return @"\"\"";
    NSString* str = [NSString stringWithUTF8String:raw_utf8];
    if (!str) return @"\"\"";
    NSData* jsonData = [NSJSONSerialization dataWithJSONObject:@[str] options:0 error:nil];
    NSString* jsonArr = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
    if (jsonArr.length >= 2) {
        return [jsonArr substringWithRange:NSMakeRange(1, jsonArr.length - 2)];
    }
    return @"\"\"";
}

@interface ERAppDelegate : NSObject <NSApplicationDelegate, WKScriptMessageHandler, NSWindowDelegate>
@property (nonatomic, strong) NSWindow* window;
@property (nonatomic, strong) WKWebView* webView;
@end

@implementation ERAppDelegate

- (void)setupMenuBar {
    NSMenu* mainMenu = [[NSMenu alloc] init];

    // App Menu
    NSMenuItem* appMenuItem = [[NSMenuItem alloc] init];
    [mainMenu addItem:appMenuItem];
    NSMenu* appMenu = [[NSMenu alloc] initWithTitle:@"ER Roadmap"];
    [appMenu addItemWithTitle:@"Quit ER Roadmap" action:@selector(terminate:) keyEquivalent:@"q"];
    [appMenuItem setSubmenu:appMenu];

    // Edit Menu (enables Cmd+C, Cmd+V, Cmd+A, Cmd+X inside WKWebView search bar)
    NSMenuItem* editMenuItem = [[NSMenuItem alloc] init];
    [mainMenu addItem:editMenuItem];
    NSMenu* editMenu = [[NSMenu alloc] initWithTitle:@"Edit"];
    [editMenu addItemWithTitle:@"Cut" action:@selector(cut:) keyEquivalent:@"x"];
    [editMenu addItemWithTitle:@"Copy" action:@selector(copy:) keyEquivalent:@"c"];
    [editMenu addItemWithTitle:@"Paste" action:@selector(paste:) keyEquivalent:@"v"];
    [editMenu addItemWithTitle:@"Select All" action:@selector(selectAll:) keyEquivalent:@"a"];
    [editMenuItem setSubmenu:editMenu];

    [NSApp setMainMenu:mainMenu];
}

- (void)applicationDidFinishLaunching:(NSNotification*)notification {
    [self setupMenuBar];

    NSRect frame = NSMakeRect(0, 0, 1280, 860);
    NSUInteger style = NSWindowStyleMaskTitled |
                       NSWindowStyleMaskClosable |
                       NSWindowStyleMaskMiniaturizable |
                       NSWindowStyleMaskResizable;

    self.window = [[NSWindow alloc] initWithContentRect:frame
                                              styleMask:style
                                                backing:NSBackingStoreBuffered
                                                  defer:NO];
    [self.window setTitle:@"Elden Ring Current Run Roadmap"];
    [self.window setMinSize:NSMakeSize(420, 560)];
    [self.window setAppearance:[NSAppearance appearanceNamed:NSAppearanceNameDarkAqua]];
    [self.window setBackgroundColor:[NSColor colorWithSRGBRed:0.035 green:0.035 blue:0.043 alpha:1.0]];
    [self.window setDelegate:self];
    [self.window center];

    WKWebViewConfiguration* config = [[WKWebViewConfiguration alloc] init];
    WKUserContentController* ucc = [[WKUserContentController alloc] init];
    [ucc addScriptMessageHandler:self name:@"erBridge"];
    config.userContentController = ucc;

    self.webView = [[WKWebView alloc] initWithFrame:[[self.window contentView] bounds]
                                      configuration:config];
    [self.webView setAutoresizingMask:(NSViewWidthSizable | NSViewHeightSizable)];
    [self.webView setValue:@(NO) forKey:@"drawsBackground"];

    [[self.window contentView] addSubview:self.webView];

    char* initial_html_ptr = rust_get_initial_html();
    if (initial_html_ptr) {
        NSString* htmlString = [NSString stringWithUTF8String:initial_html_ptr];
        rust_free_string(initial_html_ptr);
        [self.webView loadHTMLString:htmlString baseURL:[NSURL URLWithString:@"https://er-roadmap.local/"]];
    }

    [self.window makeKeyAndOrderFront:nil];
    [NSApp activateIgnoringOtherApps:YES];
}

- (BOOL)applicationShouldTerminateAfterLastWindowClosed:(NSApplication*)sender {
    return YES;
}

- (void)userContentController:(WKUserContentController*)userContentController
      didReceiveScriptMessage:(WKScriptMessage*)message {
    if (![message.body isKindOfClass:[NSDictionary class]]) return;
    NSDictionary* body = (NSDictionary*)message.body;
    NSString* action = body[@"action"] ?: @"";
    NSString* payload = body[@"payload"] ?: @"";

    if ([action isEqualToString:@"/export"]) {
        NSSavePanel* savePanel = [NSSavePanel savePanel];
        [savePanel setNameFieldStringValue:@"elden-ring-roadmap.json"];
        [savePanel beginSheetModalForWindow:self.window completionHandler:^(NSModalResponse result) {
            if (result == NSModalResponseOK && savePanel.URL) {
                char* json_ptr = rust_export_json();
                if (json_ptr) {
                    NSString* jsonStr = [NSString stringWithUTF8String:json_ptr];
                    rust_free_string(json_ptr);
                    [jsonStr writeToURL:savePanel.URL atomically:YES encoding:NSUTF8StringEncoding error:nil];
                }
            }
        }];
        return;
    }

    if ([action isEqualToString:@"/import"]) {
        NSOpenPanel* openPanel = [NSOpenPanel openPanel];
        [openPanel setCanChooseFiles:YES];
        [openPanel setCanChooseDirectories:NO];
        [openPanel setAllowsMultipleSelection:NO];
        [openPanel beginSheetModalForWindow:self.window completionHandler:^(NSModalResponse result) {
            if (result == NSModalResponseOK && openPanel.URL) {
                NSString* content = [NSString stringWithContentsOfURL:openPanel.URL encoding:NSUTF8StringEncoding error:nil];
                if (content) {
                    char* ws_html_ptr = rust_import_json([content UTF8String]);
                    if (ws_html_ptr) {
                        NSString* escaped = js_escape_string(ws_html_ptr);
                        rust_free_string(ws_html_ptr);
                        NSString* js = [NSString stringWithFormat:@"window.__erUpdateWorkspace(%@);", escaped];
                        [self.webView evaluateJavaScript:js completionHandler:nil];
                    }
                }
            }
        }];
        return;
    }

    char* html_ptr = rust_handle_ipc([action UTF8String], [payload UTF8String]);
    if (html_ptr) {
        NSString* escaped = js_escape_string(html_ptr);
        rust_free_string(html_ptr);
        NSString* js = nil;
        if ([action isEqualToString:@"/search"]) {
            js = [NSString stringWithFormat:@"window.__erUpdateSearch(%@);", escaped];
        } else {
            js = [NSString stringWithFormat:@"window.__erUpdateWorkspace(%@);", escaped];
        }
        [self.webView evaluateJavaScript:js completionHandler:nil];
    }
}

@end

void launch_native_macos_app(void) {
    @autoreleasepool {
        NSApplication* app = [NSApplication sharedApplication];
        [app setActivationPolicy:NSApplicationActivationPolicyRegular];
        ERAppDelegate* delegate = [[ERAppDelegate alloc] init];
        [app setDelegate:delegate];
        [app run];
    }
}
