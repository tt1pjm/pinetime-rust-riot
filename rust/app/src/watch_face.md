# watch_face.rs: Porting PineTime Watch Face from C to Rust on RIOT OS with LittlevGL

_This article is presented in CINEMASCOPE... Rotate your phone to view the C and Rust source code side by side... Or better yet, read this article on a desktop computer_

We'll learn step by step to convert this Embedded C code (based on LittlevGL) to Embedded Rust on RIOT OS...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
| `lv_obj_t *screen_time_create(home_time_widget_t *ht) {` <br><br>&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>&nbsp;&nbsp;`    lv_obj_t *scr = lv_obj_create(NULL, NULL);` <br>&nbsp;&nbsp;`    lv_obj_t *label1 = lv_label_create(scr, NULL);` <br><br>&nbsp;&nbsp;`    lv_label_set_text(label1, "00:00");` <br>&nbsp;&nbsp;`    lv_obj_set_width(label1, 240);` <br>&nbsp;&nbsp;`    lv_obj_set_height(label1, 200);` <br>&nbsp;&nbsp;`    ht->lv_time = label1;` <br>&nbsp;&nbsp;`    ...` <br>&nbsp;&nbsp;`    return scr;` <br>`}` <br> | `fn create_widgets(widgets: &mut WatchFaceWidgets) -> ` <br>&nbsp;&nbsp;`    LvglResult<()> {` <br><br>&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>&nbsp;&nbsp;`    let scr = widgets.screen;` <br>&nbsp;&nbsp;`    let label1 = label::create(scr, ptr::null()) ? ;` <br><br>&nbsp;&nbsp;`    label::set_text(label1, strn!("00:00")) ? ;` <br>&nbsp;&nbsp;`    obj::set_width(label1, 240) ? ;` <br>&nbsp;&nbsp;`    obj::set_height(label1, 200) ? ;` <br>&nbsp;&nbsp;`    widgets.time_label = label1;` <br>&nbsp;&nbsp;`    ...` <br>&nbsp;&nbsp;`    Ok(())` <br>`}` <br> |

We'll also learn how Rust handles memory safety when calling C functions...

| __Original C Code__ | __Converted Rust Code__ |
| :--- | :--- |
|
`int set_time_label(home_time_widget_t *ht) {` <br><br>&nbsp;&nbsp;`    //  Create a string buffer on stack` <br>&nbsp;&nbsp;`    char time[6];` <br><br>&nbsp;&nbsp;`    //  Format the time` <br>&nbsp;&nbsp;`    int res = snprintf(time, ` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        sizeof(time), ` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        "%02u:%02u", ` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        ht->time.hour,` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        ht->time.minute);` <br><br>&nbsp;&nbsp;`if (res != sizeof(time) - 1) {` <br>&nbsp;&nbsp;&nbsp;&nbsp;`LOG_ERROR("overflow");` <br>&nbsp;&nbsp;&nbsp;&nbsp;`return -1;` <br>&nbsp;&nbsp;`}` <br><br>&nbsp;&nbsp;`//  Set the label` <br>&nbsp;&nbsp;`lv_label_set_text(ht->lv_time, time);` <br><br>&nbsp;&nbsp;`//  Return OK` <br>&nbsp;&nbsp;`return 0;` <br>`}` <br>|`fn set_time_label(` <br>&nbsp;&nbsp;`    widgets: &WatchFaceWidgets, ` <br>&nbsp;&nbsp;`    state: &WatchFaceState) -> ` <br>&nbsp;&nbsp;`    LvglResult<()> {` <br><br>&nbsp;&nbsp;`    //  Create a static string buffer` <br>&nbsp;&nbsp;`    static mut TIME_BUF: HString::<U6> =`<br>&nbsp;&nbsp;&nbsp;&nbsp;` HString(IString::new());` <br><br>&nbsp;&nbsp;`    unsafe {` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        //  Format the time` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        TIME_BUF.clear();` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        write!(&mut TIME_BUF, ` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            "{:02}:{:02}\0",` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            state.time.hour,` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            state.time.minute)` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            .expect("overflow");` <br><br>&nbsp;&nbsp;&nbsp;&nbsp;`        //  Set the label` <br>&nbsp;&nbsp;&nbsp;&nbsp;`        label::set_text(widgets.time_label, ` <br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            &Strn::from_str(&TIME_BUF) ? ;` <br>&nbsp;&nbsp;`    }` <br><br>&nbsp;&nbsp;`    //  Return OK` <br>&nbsp;&nbsp;`    Ok(())` <br>`}` <br>
|

# Function Declaration

Here's a C function that calls the [LittlevGL](https://littlevgl.com/) library to create a Label Widget.  The Label Widget displays the time of the day (like `23:59`).  This code was taken from the [bosmoment /
PineTime-apps](https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c) port of [RIOT OS](https://www.riot-os.org/) to the [PineTime Smart Watch](https://wiki.pine64.org/index.php/PineTime).

```C
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

```C
lv_obj_t *screen_time_create(home_time_widget_t *ht) { ...
```

This function accepts a pointer and returns another pointer. In Rust, functions are defined with the `fn` keyword...

```Rust
fn screen_time_create( ...
```

The return type `lv_obj_t` goes to the end of the function declaration, marked by `->`...

```Rust
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

# Variable Declaration

Now let's convert this variable declaration from C to Rust...

```C
lv_obj_t *scr = lv_obj_create( ... ); 
```

`scr` is a pointer to a C Struct `lv_obj_t`. `scr` is set to the value returned by the C function LittlevGL `lv_obj_create` (which creates a LittlevGL Screen).

In Rust, variables are declared with the `let` keyword, followed by the variable name and type...

```Rust
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
<br>

The parameters are missing from the above code... Let's learn to convert `NULL` to Rust.

# Null Pointers

`NULL` is an unfortunate fact of life for C coders.

```C
lv_obj_t *scr = lv_obj_create(NULL, NULL); 
```

# Import C Functions

# Error Handling

```Rust
let scr: *mut lv_obj_t = lv_obj_create(ptr::null_mut(), ptr::null());
```

```Rust
let scr = lv_obj_create(ptr::null_mut(), ptr::null());
```

```Rust
let screen = obj::create(ptr::null_mut(), ptr::null())
    .expect("create screen obj fail");
```

```C
lv_obj_t * lv_obj_create(lv_obj_t *parent, const lv_obj_t *copy);
```
_From https://github.com/littlevgl/lvgl/blob/master/src/lv_core/lv_obj.h_

```Rust
#[lvgl_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Create a basic object"]
    #[doc = " - __`parent`__: pointer to a parent object."]
    #[doc = "                  If NULL then a screen will be created"]
    #[doc = " - __`copy`__: pointer to a base object, if not NULL then the new object will be copied from it"]
    #[doc = " Return: pointer to the new object"]
    pub fn lv_obj_create(parent: *mut lv_obj_t, copy: *const lv_obj_t) -> *mut lv_obj_t;
}
```
_From https://github.com/lupyuen/PineTime-apps/blob/master/rust/lvgl/src/core/obj.rs_

TODO

# Update LittlevGL Widget

TODO

From https://github.com/bosmoment/PineTime-apps/blob/master/widgets/home_time/screen_time.c

From https://github.com/lupyuen/PineTime-apps/blob/master/rust/app/src/watch_face.rs

# bindgen

TODO