## This file was taken from [Hackspire], and is a checklist of features.

*libndls* is a set of macros and functions available as a [static
library](https://en.wikipedia.org/wiki/Static_library) when building
with Ndless. The library is automatically linked by `nspire-gcc` and
`nspire-g++` without "-nostdlib".

These definitions are available in the latest version of the Ndless SDK
and should work on every Ndless version.

## Ndless

- [x] `void assert_ndless_rev(unsigned required_rev)`: Since v3.1 r617.
    Displays a popup asking to update Ndless if the Ndless revision on
    the calculator is less than *required\_rev*, and exits the program.
    Does nothing if the revision is greater or equal than
    *required\_rev*. You should call this function at the beginning of
    your program if it is using syscalls recently added to Ndless, or
    libndls functions which depend on recent syscalls. See Checking
    Ndless
    version
    for more info. Note that this function works without v3.1 r617 or
    higher.

## LCD API

- [x] `scr_type_t lcd_type()`: Returns the native LCD type (see the list
    below). Using this is always the fastest way to display a frame.
- [ ] `bool lcd_init(scr_type_t type)`: Set the LCD mode. You need to call
    this before you can use lcd\_blit with the same `scr_type`. You also
    need to call `lcd_init(SCR_TYPE_INVALID)` before using any of the
    functions in the UI section and before exiting the program.
- [ ] `void lcd_blit(void* buffer, scr_type_t type)`: Blit the buffer to
    the screen.

Available screen types (as of r2004):

- [x] SCR\_320x240\_4: 4bit grayscale. Native on classic calcs.
- [x] SCR\_320x240\_8: 8bit paletted mode.
- [x] SCR\_320x240\_16: RGB444
- [x] SCR\_320x240\_565: RGB565. Native on CX before HW-W
- [x] SCR\_240x320\_565: RGB565. Native on CX HW-W

## UI

- [x] `void show_msgbox(const char *title, const char *msg)`: show a
    message box, with a single button OK"
- [x] `unsigned int show_msgbox_2b(const char *title, const char *msg,
    const char *button1, const char *button2)`: since v3.1. show a
    message box with two buttons with custom labels. Return the number
    of the button pressed (1 for the first button).
- [x] `unsigned int show_msgbox_3b(const char *title, const char *msg,
    const char *button1, const char *button2, const char *button3)`:
    since v3.1. show a message box with three buttons with custom
    labels. Return the number of the button pressed (1 for the first
    button).
- [x] `int show_msg_user_input(const char * title, const char * msg, char * defaultvalue, char ** value_ref)`:
    since v3.1 r607. Request popup.
    Usage: `char * value; show_msg_user_input("title", "msg", "default", &value)`.
    `value` must be freed with `free()` once used. Returns the
    length of the value, or -1 if an empty text was entered or escape
    was pressed. Some issues fixed in r634 with the new String API.
- [x] `int show_1numeric_input(const char * title, const char * subtitle,
    const char * msg, int * value_ref, int min_value, int max_value)`:
    since v3.1 r607. Request popup for one numeric input. Caution,
    values like -1 or 0 for *min\_value* will cancel the popup. Returns
    1 if OK, 0 if cancelled.
- [x] `int show_2numeric_input(const char * title, const char * subtitle,
    const char * msg1, int * value1_ref, int min_value1, int max_value1,
    const char * msg2, int * value2_ref, int min_value2, int
    max_value2)`: since v3.1 r607. Request popup for two numeric inputs.
    Caution, values like -1 or 0 for *min\_value* will cancel the popup.
    Returns 1 if OK, 0 if cancelled.
- [x] `void refresh_osscr(void)`: since v3.1. Must be called at the end of
    a program that creates or deletes files, to update the OS document
    browser.

## Keyboard

- [x] `BOOL any_key_pressed(void)`: non-blocking key press test. Return
    `TRUE` if one or more keys are being pressed.
- [x] `BOOL isKeyPressed(key)`: non-blocking key press test. `key` must be
    one of the `KEY_NSPIRE_*` constants defined in keys.h.
- [x] `BOOL on_key_pressed(void)`: since v3.1. Non-blocking ON key press
    test. Caution, key scanning is time consuming and may hurt the
    performance of programs which needs high reactivity. You should skip
    key scanning regularly in the main loop of a game.
- [x] `void wait_key_pressed(void)`: block until a key is pressed.
    Changing the timer frequency have effects on the latency of this
    function.
- [x] `void wait_no_key_pressed(void)`: block until all the keys are
    released. Changing the timer frequency have effects on the latency
    of this function.
- [x] `touchpad_info_t *touchpad_getinfo(void)`: return information on the
    Touchpad area such as its dimension. Return `NULL` if not a
    TI-Nspire Touchpad. See `include/libndls.h` for the definition of
    `touchpad_info_t`.
- [x] `int touchpad_scan(touchpad_report_t *report)`: check user
    interactions with the Touchpad area and writes to `report`. See
    `include/libndls.h` for the definition of `touchpad_report_t`.
    `report->contact` and `report->pressed` are always `FALSE` on
    TI-Nspire Clickpad. See `src/arm/tests/ndless_tpad.c` for an example
    of use.
- [ ] `int get_event(struct s_ns_event*)`: since r721. Poll for an OS
    event. See `struct s_ns_event` in nucleus.h.
- [ ] `void send_key_event(struct s_ns_event* eventbuf, unsigned short
    keycode_asciicode, BOOL is_key_up, BOOL unknown)`: since r721.
    Simulate a key event.
- [ ] `void send_click_event(struct s_ns_event* eventbuf, unsigned short
    keycode_asciicode, BOOL is_key_up, BOOL unknown)`: since r750.
    Simulate a click event. keycode\_asciicode=0xFB00: single click,
    keycode\_asciicode=0xAC00: drag.
- [ ] `void send_pad_event(struct s_ns_event* eventbuf, unsigned short
    keycode_asciicode, BOOL is_key_up, BOOL unknown)`: since r750.
    Simulate a cursor move. Set the cursor coordinates in eventbuf, and
    keycode\_asciicode to 0x7F00.

## Filesystem

- [x] `int enable_relative_paths(char **argv)`: since r820. Call before using
`fopen()` and other file-related functions with paths relative to the
current program. `argv` should be the `argv` parameter of the `main()`
function. Returns -1 on error, 0 if success.
- [ ] `fstat`

## CPU

- [x] `void clear_cache(void)`: flush the data cache and invalidate the
    instruction and data caches of the processor. Should be called
    before loading code dynamically, after a code patch or with
    self-modifying code.

## Hardware

- [x] `BOOL is_classic`: since v3.1. `TRUE` on classic TI-Nspire. This is
    the preferred way to check CX/CM-specific features: *if
    (is\_classic) classic\_code; else cx\_code;*
- [x] `BOOL is_cm`: since v3.1 r863. `TRUE` on TI-Nspire CM/CM-C.
- [x] `BOOL has_colors`: since v3.1. `TRUE` if the device has a screen in
    color.
- [x] `BOOL is_touchpad`: `TRUE` on a TI-Nspire Touchpad or on a TI-Nspire
    CX.
- [x] `unsigned hwtype()`: 0 on classic TI-Nspire, 1 on TI-Nspire CX.
- [ ] `IO()`: select an I/O port whose mapping depends on the hardware
    type. Fo example `IO(0xDC00000C, 0xDC0000010)` will return
    0xDC00000C on classic TI-Nspire, 0xDC0000010 on CX. Returns a
    *volatile unsigned\**.

## Time

- [x] `void idle(void)`: switch to low-power state until the next
    interrupt occurs. The use of this function is encouraged when
    waiting in loops for an event to save the batteries. Changing the
    timer frequency have effects on the latency of this function.
- [x] `void msleep(unsigned ms)`: delay for a specified amount of time in
    ms. The CPU is regularly switched to low-power state while blocking.
    Note that the `sleep` function has been removed.

## Configuration

- [ ] `void cfg_register_fileext(const char *ext, const char *prgm)`:
    (since v3.1 r797) associate for Ndless the file extension `ext`
    (without leading '.') to the program name `prgm`. Does nothing if
    the extension is already registered.

## Debugging

- [x] `void bkpt()`: software breakpoint. Make the emulator halt and open
    the debugger. Remove before transferring to a calculator, as it will
    crash if executed.

## Builtin functions

Ndless exposes internal features and states throw the nl\_\*()
functions.

- [x] `BOOL nl_isstartup(void)`: (since v3.1 r540) returns TRUE if the
    program is currently being run at OS startup. See the [User
    Guide](http://ndlessly.wordpress.com/ndless-user-guide/#startup).
- [ ] `int nl_osvalue(const int values[], unsigned size)`: returns the
    value of `values` corresponding to the OS version. `size` is the
    number of values. values\[0\] corresponds to non-CAS 3.1,
    values\[1\] to CAS 3.1, values\[2\] to non-CAS CX 3.1, values\[3\]
    to CAS CX 3.1, values\[4\] to CM-C 3.1, values\[5\] to CAS CM-C 3.1,
    values\[6\] to non-CAS 3.6, values\[7\] to CAS 3.6, values\[8\] to
    non-CAS CX 3.6, values\[9\] to CAS CX 3.6.
- [x] `void nl_set_resident(void)`: (since v3.1 r553) see
      [Resident programs]
- [ ] `void nl_no_scr_redraw(void)`: (since v3.1 r756) don't restore the
    screen on program exit
- [x] `BOOL nl_loaded_by_3rd_party_loader(void)`: (since v3.1 r791) return
    TRUE if a third-party Launcher was used to boot the OS, such as
    nLaunch/nLaunchy
- [x] `int nl_exec(const char *prgm_path, int argsn, char *args[])`:
    (since v3.1 r877) run a program. `prgm_path` is its full path with
    .tns extension, `argsn` is `args[]` size, `args[]` sets the program
    additional arguments, passed through the main function's `argv[]`
    parameter. `args[]` must not include the program name (`argv[0]`)
    nor the terminating NULL argument. If the the program is run through
    a file association, `argv[2]` will be set to `args[0]`. `argsn` and
    `args[]` may be respectively 0 and NULL.

## Deprecated

### Old screen API

The following section is only valid if using the OLD\_SCREEN\_API
define.

- [ ] `SCREEN_BASE_ADDRESS`: address of the screen buffer. Read from the
    LCD controller, caching it is recommended.
- [ ] `SCREEN_BYTES_SIZE`: size of the screen buffer. Calculated depending
    on the color mode advertised by the LCD controller, caching it is
    recommended as long as the mode isn't changed.
- [ ] `SCREEN_WIDTH`: screen width in pixels
- [ ] `SCREEN_HEIGHT`: screen height in pixels
- [ ] `void clrscr(void)`: clear the screen
- [ ] `BOOL lcd_isincolor(void)`: since v3.1. Check the current LCD mode.
- [ ] `void lcd_incolor(void)`: since v3.1. Switch to color LCD mode.
- [ ] `void lcd_ingray(void)`: since v3.1. Switch to grayscale LCD mode.
- [ ] `volatile unsigned *IO_LCD_CONTROL`: since v3.1. LCD control
    register of the LCD controller

[Hackspire]: https://hackspire.org/index.php/Libndls
[Resident programs]: https://hackspire.org/index.php/Ndless_features_and_limitations#Resident_programs
