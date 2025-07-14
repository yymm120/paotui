use time::OffsetDateTime;

fn is_timestamp_convertible(timestamp: i64) -> bool {
  OffsetDateTime::from_unix_timestamp(timestamp).is_ok()
}

fn is_valid_timestamp_time_crate(timestamp: i64) -> bool {
  if let Ok(dt) = OffsetDateTime::from_unix_timestamp(timestamp) {
    // 检查是否在合理范围内（1970 ~ 当前时间 + 100 年）
    let now = OffsetDateTime::now_utc();
    let min_time = OffsetDateTime::from_unix_timestamp(0).unwrap();
    let max_time = now.replace_year(now.year() + 100).unwrap();

    dt >= min_time && dt <= max_time
  } else {
    false // 转换失败（无效时间戳）
  }
}
