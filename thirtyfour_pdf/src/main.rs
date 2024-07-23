use axum::{
    extract::{State},
    response::{IntoResponse, Response},
    routing::{get}, Router,
};
// use serde::{Deserialize, Serialize};

use std::sync::Arc;
use thirtyfour::prelude::*;
use tokio::{
    sync::{RwLock},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use webdriver::command::PrintParameters;
//
#[derive(Clone, Debug)]
struct AppState {
    webdriver_client: Arc<RwLock<WebDriver>>,
    webdriver_client2: Arc<RwLock<WebDriver>>,
}

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug,axum=trace", PKG_NAME).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let caps = DesiredCapabilities::chrome();

    let driver = WebDriver::new("http://localhost:4444", caps.clone()).await.expect("Cannot open connection with webdriver, check if webdriver proxy is running and check if address and port are matching");

    let driver2 = WebDriver::new("http://localhost:4444", caps.clone()).await.expect("Cannot open connection with webdriver, check if webdriver proxy is running and check if address and port are matching");

    let state = AppState {
        webdriver_client: Arc::new(RwLock::new(driver.clone())),
        webdriver_client2: Arc::new(RwLock::new(driver2.clone())),
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(generate_pdf))
        .route("/2", get(generate_pdf2))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // Always explicitly close the browser.
    //sleep(Duration::from_millis(10000)).await;
    //driver.quit().await;
}

const ODYSSEY: &str = include_str!("base64_other.txt");

async fn generate_pdf(State(state): State<AppState>) -> Response {
    let tmp = ODYSSEY.trim();
    //tracing::info!("{tmp:?}");

    tracing::info!("tab create");
    let window = {
        let driver = state.webdriver_client.read().await;
        driver.new_tab().await
    }
    .unwrap();
    tracing::info!("tab ended {window:?}");

    tracing::info!("window switch");
    {
        let driver = state.webdriver_client.write().await;
        driver.switch_to_window(window.clone()).await.unwrap();
        driver.goto(tmp).await.unwrap();
    }
    tracing::info!("window switch ended");

    tracing::info!("pdf started");

    let pdf = {
        let driver = state.webdriver_client.write().await;
        driver.switch_to_window(window).await.unwrap();
        driver.print_page_base64(PrintParameters::default()).await
    };

    let pdf = pdf.unwrap();
    tracing::info!("pdf ended");

    pdf.into_response()
}

async fn generate_pdf2(State(state): State<AppState>) -> Response {
    let tmp = ODYSSEY.trim();
    //tracing::info!("{tmp:?}");

    tracing::info!("tab create");
    let window = {
        let driver = state.webdriver_client2.read().await;
        driver.new_tab().await
    }
    .unwrap();
    tracing::info!("tab ended {window:?}");

    tracing::info!("window switch");
    {
        let driver = state.webdriver_client2.write().await;
        driver.switch_to_window(window.clone()).await.unwrap();
        driver.goto(tmp).await.unwrap();
    }
    tracing::info!("window switch ended");

    tracing::info!("pdf started");

    let pdf = {
        let driver = state.webdriver_client2.write().await;
        driver.switch_to_window(window).await.unwrap();
        driver.print_page_base64(PrintParameters::default()).await
    };

    let pdf = pdf.unwrap();
    tracing::info!("pdf ended");

    pdf.into_response()
}

// async fn shutdown_signal() {
//     let ctrl_c = async {
//         signal::ctrl_c()
//             .await
//             .expect("failed to install Ctrl+C handler");
//     };

//     #[cfg(unix)]
//     let terminate = async {
//         signal::unix::signal(signal::unix::SignalKind::terminate())
//             .expect("failed to install signal handler")
//             .recv()
//             .await;
//     };

//     #[cfg(not(unix))]
//     let terminate = std::future::pending::<()>();

//     tokio::select! {
//         _ = ctrl_c => {},
//         _ = terminate => {},
//     }
// }
