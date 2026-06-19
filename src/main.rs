#![recursion_limit = "512"]

#[cfg(feature = "trailing_telemetry")]
fn init_tracing() {
    use opentelemetry::global;
    use opentelemetry::KeyValue;
    use opentelemetry_otlp::{SpanExporter, WithExportConfig};
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use opentelemetry_sdk::Resource;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "open_diy=debug,info,axum_tracing_opentelemetry=error".into());

    let fmt_layer = tracing_subscriber::fmt::layer();

    if let Ok(otlp_endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        let exporter = SpanExporter::builder()
            .with_http()
            .with_endpoint(otlp_endpoint)
            .build()
            .expect("Failed to create OTLP exporter");

        let provider = SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(
                Resource::builder_empty()
                    .with_attributes(vec![KeyValue::new("service.name", "open-diy")])
                    .build()
            )
            .build();

        global::set_tracer_provider(provider.clone());
        let tracer = global::tracer("open-diy");

        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .with(telemetry)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    #[cfg(feature = "trailing_telemetry")]
    init_tracing();

    // Initialize Pyroscope if endpoint is configured
    if let Ok(pyroscope_endpoint) = std::env::var("PYROSCOPE_ENDPOINT") {
        use pyroscope::pyroscope::PyroscopeAgentBuilder;
        use pyroscope::backend::{pprof_backend, PprofConfig, BackendConfig};
        
        leptos::logging::log!("Initializing Pyroscope with endpoint: {}", pyroscope_endpoint);
        let agent = PyroscopeAgentBuilder::new(
            &pyroscope_endpoint,
            "open-diy-backend",
            100, // 100Hz
            "pyroscope-rs",
            env!("CARGO_PKG_VERSION"),
            pprof_backend(PprofConfig::default(), BackendConfig::default()),
        )
        .build();

        match agent {
            Ok(agent) => match agent.start() {
                Ok(running_agent) => {
                    Box::leak(Box::new(running_agent));
                    leptos::logging::log!("Pyroscope agent started successfully");
                }
                Err(e) => leptos::logging::log!("Failed to start Pyroscope agent: {:?}", e),
            },
            Err(e) => leptos::logging::log!("Failed to build Pyroscope agent: {:?}", e),
        }
    }

    use axum::{http::header, response::IntoResponse, routing::get, Router};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use open_diy::app::*;
    use open_diy::seo::{robots_txt, sitemap_xml};

    async fn robots_txt_handler() -> impl IntoResponse {
        (
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            robots_txt(),
        )
    }

    async fn sitemap_xml_handler() -> impl IntoResponse {
        (
            [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
            sitemap_xml(),
        )
    }

    async fn otel_test_handler() -> impl IntoResponse {
        (
            [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
            r#"{"status":"ok","message":"OpenTelemetry test request successful"}"#,
        )
    }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/robots.txt", get(robots_txt_handler))
        .route("/sitemap.xml", get(sitemap_xml_handler))
        .route("/api/otel-test", get(otel_test_handler))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell));

    #[cfg(feature = "trailing_telemetry")]
    let app = app.layer(axum_tracing_opentelemetry::middleware::OtelAxumLayer::default());

    let app = app.with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
