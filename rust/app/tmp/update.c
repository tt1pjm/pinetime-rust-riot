static int _home_time_set_time_label(home_time_widget_t *ht)
{
    char time[6];
    int res = snprintf(time, sizeof(time), "%02u:%02u", ht->time.hour,
                       ht->time.minute);
    if (res != sizeof(time) - 1) {
        LOG_ERROR("[home_time]: error formatting time string %*s\n", res, time);
        return -1;
    }
    lv_label_set_text(ht->lv_time, time);
    return 0;
}