/// Populate the Time and Date Labels with the time and date. Called by screen_time_update_screen() above.
fn set_time_label(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    //  Create a string buffer with max size 6 to format the time
    static mut TIME_BUF: heapless::String::<heapless::consts::U6> = heapless::String(heapless::i::String::new());
    //  Format the time and set the label
    unsafe {
        TIME_BUF.clear();
        write!(&mut TIME_BUF, "{:02}:{:02}\0",  //  Must terminate Rust strings with null
            state.time.hour,
            state.time.minute)
            .expect("time fail");
        label::set_text(widgets.time_label, &Strn::new(TIME_BUF.as_bytes())) ? ;  //  TODO: Simplify
    }
    //  Return OK
    Ok(())
}