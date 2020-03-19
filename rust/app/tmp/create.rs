`fn create_widgets(widgets: &mut WatchFaceWidgets) -> ` <br>
<br>
&nbsp;&nbsp;`    LvglResult<()> {` <br>
&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>
&nbsp;&nbsp;`    let scr = widgets.screen;` <br>
&nbsp;&nbsp;`    let label1 = label::create(scr, ptr::null()) ? ;` <br>
<br>
&nbsp;&nbsp;`    label::set_text(label1, strn!("00:00")) ? ;` <br>
&nbsp;&nbsp;`    obj::set_width(label1, 240) ? ;` <br>
&nbsp;&nbsp;`    obj::set_height(label1, 200) ? ;` <br>
&nbsp;&nbsp;`    widgets.time_label = label1;` <br>
&nbsp;&nbsp;`    ...` <br>
&nbsp;&nbsp;`    Ok(())` <br>
`}` <br>

unsafe { lv_label_set_text(
    b"abc\0".as_ptr(), 
    [b'a', b'b', b'c', 0].as_ptr()) }; ////
}

extern "C" {
fn lv_label_set_text(label: *const u8, text: *const u8); ////
}