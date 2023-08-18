use dotenv::dotenv;

pub fn init() {
    dotenv().ok();

    // ログ出力を設定する
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .compact();

    tracing_subscriber::fmt().event_format(format).init();
}
