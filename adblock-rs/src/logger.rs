use lazy_static::lazy_static;
use std::sync::Once;

#[cfg(target_os = "android")]
use {android_logger::Config, log::LevelFilter};

#[cfg(not(target_os = "android"))]
use {env_logger::Builder, log::LevelFilter};

lazy_static! {
    static ref _LOGGER: Once = {
        let init = Once::new();

        #[cfg(target_os = "android")]
        init.call_once(|| {
            android_logger::init_once(
                Config::default()
                    .with_tag("Tag_RustAdvtBlocker")
                    .with_max_level(LevelFilter::max()),
            );
        });

        #[cfg(not(target_os = "android"))]
        init.call_once(|| {
            let mut builder = Builder::new();
            builder.filter_level(LevelFilter::Debug);
            builder.init();
        });

        init
    };
}
