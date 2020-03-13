`fn create_widgets(widgets: &mut WatchFaceWidgets) -> ` <br>
&nbsp;&nbsp;`    LvglResult<()> {` <br>
&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>
&nbsp;&nbsp;`    let scr = widgets.screen;` <br>
&nbsp;&nbsp;`    let label1 = label::create(scr, ptr::null()) ? ;` <br>
&nbsp;&nbsp;`    label::set_long_mode(label1, label::LV_LABEL_LONG_BREAK) ? ;` <br>
&nbsp;&nbsp;`    label::set_text(label1, strn!("00:00")) ? ;` <br>
&nbsp;&nbsp;`    obj::set_width(label1, 240) ? ;` <br>
&nbsp;&nbsp;`    obj::set_height(label1, 200) ? ;` <br>
&nbsp;&nbsp;`    label::set_align(label1, label::LV_LABEL_ALIGN_CENTER) ? ;` <br>
&nbsp;&nbsp;`    obj::align(label1, scr, obj::LV_ALIGN_CENTER, 0, -30) ? ;` <br>
&nbsp;&nbsp;`    obj::set_style(label1, unsafe { &style_time }) ? ;` <br>
&nbsp;&nbsp;`    widgets.time_label = label1;` <br>
&nbsp;&nbsp;`    ...` <br>
&nbsp;&nbsp;`    //  Return OK` <br>
&nbsp;&nbsp;`    Ok(())` <br>
`}` <br>