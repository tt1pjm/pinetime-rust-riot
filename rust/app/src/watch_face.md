# watch_face.rs: Porting PineTime Watch Face from C to Rust on RIOT OS with LittlevGL

_This article is presented in CINEMASCOPE... Rotate your phone to view the C and Rust source code side by side... Or better yet, read this article on a desktop computer_

We'll learn step by step to convert this [Embedded C code](https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c) (based on LittlevGL) to [Embedded Rust](https://github.com/lupyuen/PineTime-apps/blob/master/rust/app/src/watch_face.rs) on RIOT OS...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
| `lv_obj_t *screen_time_create(home_time_widget_t *ht) {` <br><br>&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>&nbsp;&nbsp;`    lv_obj_t *scr = lv_obj_create(NULL, NULL);` <br>&nbsp;&nbsp;`    lv_obj_t *label1 = lv_label_create(scr, NULL);` <br><br>&nbsp;&nbsp;`    lv_label_set_text(label1, "00:00");` <br>&nbsp;&nbsp;`    lv_obj_set_width(label1, 240);` <br>&nbsp;&nbsp;`    lv_obj_set_height(label1, 200);` <br>&nbsp;&nbsp;`    ht->lv_time = label1;` <br>&nbsp;&nbsp;`    ...` <br>&nbsp;&nbsp;`    return scr;` <br>`}` <br> | `fn create_widgets(widgets: &mut WatchFaceWidgets) -> ` <br>&nbsp;&nbsp;`    LvglResult<()> {` <br><br>&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>&nbsp;&nbsp;`    let scr = widgets.screen;` <br>&nbsp;&nbsp;`    let label1 = label::create(scr, ptr::null()) ? ;` <br><br>&nbsp;&nbsp;`    label::set_text(label1, strn!("00:00")) ? ;` <br>&nbsp;&nbsp;`    obj::set_width(label1, 240) ? ;` <br>&nbsp;&nbsp;`    obj::set_height(label1, 200) ? ;` <br>&nbsp;&nbsp;`    widgets.time_label = label1;` <br>&nbsp;&nbsp;`    ...` <br>&nbsp;&nbsp;`    Ok(())` <br>`}` <br> |
| _From https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c_ | _From https://github.com/lupyuen/PineTime-apps/blob/master/rust/app/src/watch_face.rs_ |

We'll also learn how Rust handles memory safety when calling C functions...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
|
`int set_time_label(home_time_widget_t *ht) {` <br><br>&nbsp;&nbsp;`    //  Create a string buffer on stack` <br>&nbsp;&nbsp;`    char time[6];` <br><br>&nbsp;&nbsp;`    //  Format the time` <br>&nbsp;&nbsp;`    int res = snprintf(time, ` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        sizeof(time), ` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        "%02u:%02u", ` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        ht->time.hour,` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        ht->time.minute);` <br><br>&nbsp;&nbsp;`if (res != sizeof(time) - 1) {` <br>&nbsp;&nbsp;&nbsp;&nbsp;`LOG_ERROR("overflow");` <br>&nbsp;&nbsp;&nbsp;&nbsp;`return -1;` <br>&nbsp;&nbsp;`}` <br><br>&nbsp;&nbsp;`//  Set the label` <br>&nbsp;&nbsp;`lv_label_set_text(ht->lv_time, time);` <br><br>&nbsp;&nbsp;`//  Return OK` <br>&nbsp;&nbsp;`return 0;` <br>`}` <br>|`fn set_time_label(` <br>&nbsp;&nbsp;`    widgets: &WatchFaceWidgets, ` <br>&nbsp;&nbsp;`    state: &WatchFaceState) -> ` <br>&nbsp;&nbsp;`    LvglResult<()> {` <br><br>&nbsp;&nbsp;`    //  Create a static string buffer` <br>&nbsp;&nbsp;`    static mut TIME_BUF: HString::<U6> =`<br>&nbsp;&nbsp;&nbsp;&nbsp;` HString(IString::new());` <br><br>&nbsp;&nbsp;`    unsafe {` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        //  Format the time` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        TIME_BUF.clear();` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        write!(&mut TIME_BUF, ` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            "{:02}:{:02}\0",` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            state.time.hour,` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            state.time.minute)` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            .expect("overflow");` <br><br>&nbsp;&nbsp;&nbsp;&nbsp;`        //  Set the label` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        label::set_text(widgets.time_label, ` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            &Strn::from_str(&TIME_BUF) ? ;` <br>&nbsp;&nbsp;`    }` <br><br>&nbsp;&nbsp;`    //  Return OK` <br>&nbsp;&nbsp;`    Ok(())` <br>`}` <br>
|
| _From https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c_ | _From https://github.com/lupyuen/PineTime-apps/blob/master/rust/app/src/watch_face.rs_ |

# Function Declaration

Here's a C function that calls the [LittlevGL](https://littlevgl.com/) library to create a Label Widget.  The Label Widget displays the time of the day (like `23:59`).  This code was taken from the [bosmoment /
PineTime-apps](https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c) port of [RIOT OS](https://www.riot-os.org/) to the [PineTime Smart Watch](https://wiki.pine64.org/index.php/PineTime).

```c
lv_obj_t *screen_time_create(home_time_widget_t *ht) {
    //  Create a label for time (00:00)
    lv_obj_t *scr = lv_obj_create(NULL, NULL);
    lv_obj_t *label1 = lv_label_create(scr, NULL);

    lv_label_set_text(label1, "00:00");
    lv_obj_set_width(label1, 240);
    lv_obj_set_height(label1, 200);
    ht->lv_time = label1;
    return scr;
}
```
_From https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c_

Functions whose names start with `lv_` (like `lv_obj_create`) are defined in the LittlevGL library. `lv_obj_t` is a C Struct exposed by the LittlevGL library. `home_time_widget_t` is a custom C Struct defined by the RIOT OS application.

Let's start by converting this function declaration from C to Rust...

```c
lv_obj_t *screen_time_create(home_time_widget_t *ht) { ...
```

This function accepts a pointer and returns another pointer. In Rust, functions are defined with the `fn` keyword...

```rust
fn screen_time_create( ...
```

The return type `lv_obj_t` goes to the end of the function declaration, marked by `->`...

```rust
fn screen_time_create(ht: *mut home_time_widget_t) 
    -> *mut lv_obj_t { ...
```

Note that the names and types have been flipped, also for pointers...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
| `lv_obj_t *` | `*mut lv_obj_t` |
| `home_time_widget_t *ht` | `ht: *mut home_time_widget_t` |
| `lv_obj_t *screen_time_create(...)` | `fn screen_time_create(...)` <br> &nbsp;&nbsp;`-> *mut lv_obj_t` |

As we convert code from C to Rust, we'll find ourselves doing a lot of this Name/Type Flipping.

Rust is strict about Mutability of variables (whether a variable's value may be modified). `*mut` declares that the pointer refers to an object that is Mutable (i.e. may be modified). For objects that may not be modified, we write `*const` (similar to C).

Here's the C function declaration converted to Rust...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
| `lv_obj_t *screen_time_create(` <br> &nbsp;&nbsp;`home_time_widget_t *ht)` | `fn screen_time_create(` <br> &nbsp;&nbsp;`ht: *mut home_time_widget_t)` <br> &nbsp;&nbsp;`-> *mut lv_obj_t` |
| _From https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c_ | _From https://github.com/lupyuen/PineTime-apps/blob/master/rust/app/src/watch_face.rs_ |

# Variable Declaration

Now let's convert this variable declaration from C to Rust...

```c
lv_obj_t *scr = lv_obj_create( ... ); 
```

`scr` is a pointer to a C Struct `lv_obj_t`. `scr` is set to the value returned by the C function LittlevGL `lv_obj_create` (which creates a LittlevGL Screen).

In Rust, variables are declared with the `let` keyword, followed by the variable name and type...

```rust
let scr: *mut lv_obj_t = lv_obj_create( ... );
```

_(Yep we did the Name/Type Flipping again)_

Here's a really cool thing about Rust... Types are optional in variable declarations!

We may drop the type `*mut lv_obj_t`, resulting in this perfectly valid Rust declaration...

```Rust
let scr = lv_obj_create( ... );
```

_What is this type dropping magic? Won't Rust complain about the missing type?_

If we think about it... `lv_obj_create` is a C function already declared somewhere. The Rust Compiler already knows that `lv_obj_create` returns a value of type `*mut lv_obj_t`.

Thus the Rust Compiler uses __Type Inference__ to deduce that `scr` must have type `*mut lv_obj_t`!

This saves us a lot of rewriting when we convert C code to Rust.

Here's how it looks when we convert to Rust the two variable declarations from our C function...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
| `lv_obj_t *screen_time_create(` <br> &nbsp;&nbsp;`home_time_widget_t *ht) {` | `fn screen_time_create(` <br> &nbsp;&nbsp;`ht: *mut home_time_widget_t)` <br> &nbsp;&nbsp;`-> *mut lv_obj_t {` <br> |
| &nbsp;&nbsp;`//  Create a label for time (00:00)` | &nbsp;&nbsp;`//  Create a label for time (00:00)` |
| &nbsp;&nbsp;`lv_obj_t *scr = lv_obj_create( ... );` | &nbsp;&nbsp;`let scr = lv_obj_create( ... );` |
| &nbsp;&nbsp;`lv_obj_t *label1 = lv_label_create(scr, ... );` | &nbsp;&nbsp;`let label1 = lv_label_create(scr, ... );` |
| _From https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c_ | _From https://github.com/lupyuen/PineTime-apps/blob/master/rust/app/src/watch_face.rs_ |
<br>

The parameters are missing from the above code... Let's learn to convert `NULL` to Rust.

# Null Pointers

`NULL` is an unfortunate fact of life for C coders. In our C code we pass two `NULL` pointers to `lv_obj_create`...

```c
//  In C: Call lv_obj_create passing 2 NULL pointers
lv_obj_t *scr = lv_obj_create(NULL, NULL); 
```

Both `NULL`s look the same to C... But not to Rust! Let's look at the function declaration in C...

```c
//  In C: Function declaration for lv_obj_create
lv_obj_t * lv_obj_create(lv_obj_t *parent, const lv_obj_t *copy);
```
_From https://github.com/littlevgl/lvgl/blob/master/src/lv_core/lv_obj.h_

See the difference? The first parameter is a non-`const` pointer (i.e. it's Mutable), whereas the second parameter is a `const` pointer.

Here's how we pass the two `NULL` pointers in Rust...

```rust
//  In Rust: Call lv_obj_create passing 2 NULL pointers: 1 mutable, 1 const
let scr = lv_obj_create(ptr::null_mut(), ptr::null());
```

`null_mut` creates a `NULL` Mutable pointer, `null` creates a Non-Mutable `const NULL` pointer.

`ptr` references the Rust Core Library, which we import like this...

```rust
//  In Rust: Import the Rust Core Library for pointer handling
use core::ptr;
```

When we insert the `NULL` parameters into the converted Rust code, we get this...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
| `lv_obj_t *screen_time_create(` <br> &nbsp;&nbsp;`home_time_widget_t *ht) {` | `fn screen_time_create(` <br> &nbsp;&nbsp;`ht: *mut home_time_widget_t)` <br> &nbsp;&nbsp;`-> *mut lv_obj_t {` <br> |
| &nbsp;&nbsp;`//  Create a label for time (00:00)` | &nbsp;&nbsp;`//  Create a label for time (00:00)` |
| &nbsp;&nbsp;`lv_obj_t *scr = lv_obj_create(` | &nbsp;&nbsp;`let scr = lv_obj_create(` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;__`NULL,`__ | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;__`ptr::null_mut(),`__ |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;__`NULL`__ | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;__`ptr::null()`__ |
| &nbsp;&nbsp;`);` | &nbsp;&nbsp;`);` |
| &nbsp;&nbsp;`lv_obj_t *label1 = lv_label_create(`&nbsp;&nbsp;&nbsp;&nbsp; | &nbsp;&nbsp;`let label1 = lv_label_create(` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`scr,` | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`scr,` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;__`NULL`__ | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;__`ptr::null()`__ |
| &nbsp;&nbsp;`);` | &nbsp;&nbsp;`);` |
| _From https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c_ | _From https://github.com/lupyuen/PineTime-apps/blob/master/rust/app/src/watch_face.rs_ |
<br>

# Import C Functions into Rust

Let's look back at the C code that we're convering to Rust...

```c
//  In C: Create a label for time (00:00)
lv_obj_t *scr = lv_obj_create(NULL, NULL);
lv_obj_t *label1 = lv_label_create(scr, NULL);

//  Set the text, width and height of the label
lv_label_set_text(label1, "00:00");
lv_obj_set_width(label1, 240);
lv_obj_set_height(label1, 200);
```

The `lv_...` functions called above come from the LittlevGL library. Here are the function declarations in C...

```c
//  In C: LittlevGL Function Declarations
lv_obj_t * lv_obj_create(lv_obj_t *parent, const lv_obj_t *copy);
lv_obj_t * lv_label_create(lv_obj_t *par, const lv_obj_t *copy);
void lv_label_set_text(lv_obj_t *label, const char *text);
void lv_obj_set_width(lv_obj_t *obj, int16_t w);
void lv_obj_set_height(lv_obj_t *obj, int16_t h);
```
_From https://github.com/littlevgl/lvgl/blob/master/src/lv_core/lv_obj.h, https://github.com/littlevgl/lvgl/blob/master/src/lv_objx/lv_label.h_

To call these C functions from Rust, we need to import them with `extern "C"` like this...

```rust
//  In Rust: Import LittlevGL Functions
extern "C" {
    fn lv_obj_create(parent: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
    fn lv_label_create(par: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
    fn lv_label_set_text(label: *mut lv_obj_t, text: *const u8);
    fn lv_obj_set_width(obj: *mut lv_obj_t, w: i16);
    fn lv_obj_set_height(obj: *mut lv_obj_t, h: i16);
}
```
_From https://github.com/lupyuen/PineTime-apps/blob/master/rust/lvgl/src/core/obj.rs, https://github.com/lupyuen/PineTime-apps/blob/master/rust/lvgl/src/objx/label.rs_

_See the Name/Type Flipping? We did it again!_

Take note of the `*mut` and `*const` pointers... Rust is very picky about Mutability!

What's `*const u8`? It's complicated... We'll talk about strings in a while.

Once the C functions have been imported, we may call them in Rust like this...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
| `lv_obj_t *screen_time_create(` <br> &nbsp;&nbsp;`home_time_widget_t *ht) {` | `fn screen_time_create(` <br> &nbsp;&nbsp;`ht: *mut home_time_widget_t)` <br> &nbsp;&nbsp;`-> *mut lv_obj_t {` <br> |
| &nbsp;&nbsp;`//  Create a label for time (00:00)` | &nbsp;&nbsp;`//  Create a label for time (00:00)` |
| &nbsp;&nbsp;`lv_obj_t *scr = lv_obj_create(` | &nbsp;&nbsp;`let scr = lv_obj_create(` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`NULL, NULL` | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`ptr::null_mut(), ptr::null()` |
| &nbsp;&nbsp;`);` | &nbsp;&nbsp;`);` |
| &nbsp;&nbsp;`lv_obj_t *label1 = lv_label_create(`&nbsp;&nbsp;&nbsp;&nbsp; | &nbsp;&nbsp;`let label1 = lv_label_create(` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`scr, NULL` | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`scr, ptr::null()` |
| &nbsp;&nbsp;`);` | &nbsp;&nbsp;`);` |
| &nbsp;&nbsp;`//  Set the text, width and height` | &nbsp;&nbsp;`//  Set the text, width and height` |
| &nbsp;&nbsp;`lv_label_set_text(` | &nbsp;&nbsp;`lv_label_set_text(` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`label1, "00:00"` | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`label1, //  TODO` |
| &nbsp;&nbsp;`);` | &nbsp;&nbsp;`);` |
| &nbsp;&nbsp;`lv_obj_set_width(` | &nbsp;&nbsp;`lv_obj_set_width(` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`label1, 240` | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`label1, 240` |
| &nbsp;&nbsp;`);` | &nbsp;&nbsp;`);` |
| &nbsp;&nbsp;`lv_obj_set_height(` | &nbsp;&nbsp;`lv_obj_set_height(` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`label1, 200` | &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`label1, 200` |
| &nbsp;&nbsp;`);` | &nbsp;&nbsp;`);` |
| _From https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c_ | _From https://github.com/lupyuen/PineTime-apps/blob/master/rust/app/src/watch_face.rs_ |
<br>

# Numeric Types

Something interesting happened when we took this C function declaration...

```c
//  In C: Function declaration for lv_obj_set_width
void lv_obj_set_width(lv_obj_t *obj, int16_t w);
```

And imported it into Rust...

```rust
//  In Rust: Import lv_obj_set_width function from C
extern "C" {
    fn lv_obj_set_width(obj: *mut lv_obj_t, w: i16);
}
```

_Look at the second parameter... How did `int16_t` in C (16-bit signed integer) become `i16` in Rust?_

You might have guessed... Numeric Types in Rust have no-nonsense, super-compact names!

So `int16_t` gets shortened to `i16`. `uint16_t` (unsigned 16-bit integer) gets shortened to `u16`.

Numeric Types are such a joy to write!  And there's no need to `#include <stdint.h>`

| __C Numeric Type__ &nbsp;&nbsp; | __Rust Numeric Type__ |
| :--- | :---: |
| `int8_t` | `i8` |
| `uint8_t` | `u8` |
| `int16_t` | `i16` |
| `uint16_t` | `u16` |
| `int32_t` | `i32` |
| `uint32_t` | `u32` |
| `int64_t` | `i64` |
| `uint64_t` | `u64` |
| `float` | `f32` |
| `double` | `f64` |
<br>

In Rust we use `u8` to refer to a byte.

# Pass Strings from Rust to C

Rust has a powerful `String` type for manipulating strings (stored in heap memory)... But we'll look at a simpler way to pass strings from Rust to C.

This is our original C code...

```c
//  In C: Declare function lv_label_set_text
void lv_label_set_text(lv_obj_t *label, const char *text);
...
//  Set the text of the label to "00:00"
lv_label_set_text(label1, "00:00");
```

Here's how we pass the string `"00:00"` from Rust to C...

```rust
//  In Rust: Import function lv_label_set_text from C
extern "C" {
    fn lv_label_set_text(label: *mut lv_obj_t, text: *const u8);
}
...
//  Set the text of the label to "00:00"
lv_label_set_text(
    label1,
    b"00:00\0".as_ptr()
);
```

Remember that `u8` in Rust means unsigned byte, so `*const u8` in Rust is similar to `const char *` in C.

Let's compare the C string and its Rust equivalent...

| __C String__ &nbsp;&nbsp; | __Rust Equivalent__ |
| :--- | :--- |
| `"00:00"`&nbsp;&nbsp;&nbsp;&nbsp; | `b"00:00\0".as_ptr()` |
<br>

The `b"`...`"` notation creates a Rust [Byte String](https://doc.rust-lang.org/reference/tokens.html#byte-string-literals). A Byte String is an array of bytes, similar to strings in C.

Unlike C, strings in Rust don't have a terminating null. So we manually added the null: `\0`

In C, arrays and pointers are interchangeable, so `char *` behaves like `char[]`... But not in Rust! 

Rust arrays have an internal counter that remembers the length of the array. Which explains why Rust strings don't have a terminating null... Rust internally tracks the length of each string.

To convert a Rust array to a pointer, we use `as_ptr()` as shown above.

_What happens if we forget to add the terminating null `\0`? Catastrophe!_

The C function `lv_label_set_text` will get very confused without the terminating null. So the above Byte String notation `b"`...`"` is prone to problems.

Later we'll see an easier, safer way to write strings... With a Rust Macro.

```rust
//  In Rust: Set the label text with a macro
lv_label_set_text(
    label1,
    strn!("00:00")
);
```
# Pointer Dereferencing

In C we write `->` to dereference a pointer and access a field...

```c
//  In C: Dereference the pointer ht and set the lv_time field
ht->lv_time = label1;
```

Rust doesn't have an operator for pointer dereferencing and field access. Instead, we use the `*` and `.` operators, which are used the same way as in C...

```rust
//  In Rust: Dereference the pointer ht and set the lv_time field
(*ht).lv_time = label1;
```

# Return Value

In C we use the `return` keyword to set the return value of the current function...

```c
lv_obj_t *screen_time_create(home_time_widget_t *ht) {
    ...
    //  In C: Return scr as the value of the function
    return scr;
}
```

In Rust the `return` keyword works the same way...

```rust
fn screen_time_create(ht: *mut home_time_widget_t) -> *mut lv_obj_t { 
    ...
    //  In Rust: Return scr as the value of the function
    return scr;
}   //  End of function
```

Rust also allows us to set the return value of the current function by writing the value as the last expression of the function...

```rust
fn screen_time_create(ht: *mut home_time_widget_t) -> *mut lv_obj_t { 
    ...
    //  In Rust: Return scr as the value of the function. Note: No semicolon ";" at the end
    scr
}   //  End of function
```

If we use this convention, the last expression of the function should not end with a semicolon.

# Import C Types

```rust
#[repr(C)]
struct home_time_widget_t {
    //  TODO
    screen:      *mut obj::lv_obj_t,
    time_label:  *mut obj::lv_obj_t,
    date_label:  *mut obj::lv_obj_t,
    ble_label:   *mut obj::lv_obj_t,
    power_label: *mut obj::lv_obj_t,
}
```

TODO

# Unsafe

TODO

# No Mangle

# Expose Rust functions to C

# Error Handling

TODO

```Rust
let screen = obj::create(ptr::null_mut(), ptr::null())
    .expect("create screen obj fail");
```

TODO

# Lifetime

TODO

# Generate Rust Bindings Automatically with bindgen

TODO

```Rust
#[lvgl_macros::safe_wrap(attr)]
extern "C" {
    pub fn lv_obj_create(parent: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
    pub fn lv_label_create(par: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
    pub fn lv_label_set_text(label: *mut lv_obj_t, text: *const ::cty::c_char);
    pub fn lv_obj_set_width(obj: *mut lv_obj_t, w: i16);
    pub fn lv_obj_set_height(obj: *mut lv_obj_t, h: i16);
}
```
_From https://github.com/lupyuen/PineTime-apps/blob/master/rust/lvgl/src/core/obj.rs, https://github.com/lupyuen/PineTime-apps/blob/master/rust/lvgl/src/objx/label.rs_

# RIOT OS Bindings

TODO

# Build and link with RIOT OS

TODO

# VSCode Development

TODO

# VSCode Debugging

TODO
