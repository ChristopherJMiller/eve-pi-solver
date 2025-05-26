pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Initialize tracing for the appropriate environment
pub fn init_tracing() {
    #[cfg(target_arch = "wasm32")]
    {
        // For WASM, use tracing-wasm
        tracing_wasm::set_as_global_default();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native environments, use a simple subscriber
        use tracing_subscriber::{fmt, EnvFilter};

        // Only initialize if not already initialized
        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(
                EnvFilter::from_default_env().add_directive("eve_pi=debug".parse().unwrap()),
            )
            .finish();

        if tracing::subscriber::set_global_default(subscriber).is_ok() {
            tracing::info!("Tracing initialized for native environment");
        }
    }
}
