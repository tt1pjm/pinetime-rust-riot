`fn create_widgets(widgets: &mut WatchFaceWidgets) -> ` <br>
`    LvglResult<()> {` <br>
`    //  Create a label for time (00:00)` <br>
`    let scr = widgets.screen;` <br>
`    let label1 = label::create(scr, ptr::null()) ? ;` <br>
`    label::set_long_mode(label1, label::LV_LABEL_LONG_BREAK) ? ;` <br>
`    label::set_text(label1, strn!("00:00")) ? ;` <br>
`    obj::set_width(label1, 240) ? ;` <br>
`    obj::set_height(label1, 200) ? ;` <br>
`    label::set_align(label1, label::LV_LABEL_ALIGN_CENTER) ? ;` <br>
`    obj::align(label1, scr, obj::LV_ALIGN_CENTER, 0, -30) ? ;` <br>
`    obj::set_style(label1, unsafe { &style_time }) ? ;` <br>
`    widgets.time_label = label1;` <br>
`    ...` <br>
`    //  Return OK` <br>
`    Ok(())` <br>
`}` <br>