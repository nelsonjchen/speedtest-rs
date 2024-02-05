macro_rules! info {
    // info!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // info!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => {};

    // info!("a {} event", "log")
    ($($arg:tt)+) => {};
}

pub(crate) use info;
