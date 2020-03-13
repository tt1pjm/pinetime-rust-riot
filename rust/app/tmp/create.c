`lv_obj_t *screen_time_create(home_time_widget_t *ht) {` <br>
&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>
&nbsp;&nbsp;`    lv_obj_t *scr = lv_obj_create(NULL, NULL);` <br>
&nbsp;&nbsp;`    lv_obj_t * label1 = lv_label_create(scr, NULL);` <br>
&nbsp;&nbsp;`    lv_label_set_long_mode(label1, LV_LABEL_LONG_BREAK);` <br>
&nbsp;&nbsp;`    lv_label_set_text(label1, "00:00");` <br>
&nbsp;&nbsp;`    lv_obj_set_width(label1, 240);` <br>
&nbsp;&nbsp;`    lv_obj_set_height(label1, 200);` <br>
&nbsp;&nbsp;`    lv_label_set_align(label1, LV_LABEL_ALIGN_CENTER);` <br>
&nbsp;&nbsp;`    lv_obj_align(label1, scr, LV_ALIGN_CENTER, 0, -30);` <br>
&nbsp;&nbsp;`    lv_label_set_style(label1, LV_LABEL_STYLE_MAIN, &style_time);` <br>
&nbsp;&nbsp;`    ht->lv_time = label1;` <br>
&nbsp;&nbsp;`    ...` <br>
&nbsp;&nbsp;`    //  Set the touch callback` <br>
&nbsp;&nbsp;`    lv_obj_set_event_cb(scr, _screen_time_pressed);` <br>
&nbsp;&nbsp;`    //  Update the widgets` <br>
&nbsp;&nbsp;`    _screen_time_update_screen(&ht->widget);` <br>
&nbsp;&nbsp;`    //  Return the screen` <br>
&nbsp;&nbsp;`    return scr;` <br>
`}` <br>