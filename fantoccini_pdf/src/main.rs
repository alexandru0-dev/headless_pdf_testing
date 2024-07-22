use axum::{
    body::Bytes, extract::{FromRef, State}, http::{Result, StatusCode}, response::{Response, IntoResponse}, routing::{get, post}, Json, Router
};
// use serde::{Deserialize, Serialize};
use serde_json::map;

use std::sync::Arc;
use tokio::{signal, sync::{RwLock, OnceCell}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use webdriver::command::{WebDriverCommand, PrintParameters};
use fantoccini::{client::NewWindowResponse, wd::WindowHandle, Client, ClientBuilder};

#[derive(Clone, Debug)]
struct AppState {
    webdriver_client: Arc<RwLock<Client>>,
}

static BLANK_TAB: OnceCell<WindowHandle> = OnceCell::const_new();

enum Browers {
    Firefox,
    Chrome
}

fn make_capabilities(browser: Browers) -> map::Map<String, serde_json::Value> {
    match browser {
        Browers::Firefox => {
            let mut caps = serde_json::map::Map::new();
            let opts = serde_json::json!({ "args": ["--headless"] });
            caps.insert("moz:firefoxOptions".to_string(), opts);
            caps
        }
        Browers::Chrome => {
            let mut caps = serde_json::map::Map::new();
            let opts = serde_json::json!({
                "args": [
                    "--headless",
                    "--disable-gpu",
                    "--disable-dev-shm-usage",
                ],
            });
            caps.insert("goog:chromeOptions".to_string(), opts);
            caps
        }
    }
}

async fn init_blank_tab(client: &Arc<RwLock<Client>>,) -> &WindowHandle {
    BLANK_TAB.get_or_init(|| async {
        let window: WindowHandle = {
            // lock client
            let wd = client.read().await;
            // get blank page
            wd.windows().await.unwrap().get(0).unwrap().clone()
        };
        window
    }).await
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "fantoccini_pdf=debug,tower_http=debug,axum=trace".into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
    
    let webdriver_client: Client = ClientBuilder::rustls()
    .expect("Failed to initialize rustls")
    .capabilities(make_capabilities(Browers::Firefox))
    .connect("http://localhost:4444")
    .await.expect("couldn't connect to the client builder");

    let state = AppState {
        webdriver_client: Arc::new(RwLock::new(webdriver_client))
    };

    init_blank_tab(&state.webdriver_client).await;

    // build our application with a route
    let app = Router::new()
        .route("/", get(generate_pdf))
        .route("/2", get(generate_pdf2))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    ()
}

async fn generate_pdf(State(state): State<AppState>) -> Response {
    let window = open_new_tab(&state.webdriver_client, "https://kozea.github.io/WeasyPerf/samples/columns/columns.html").await;
    print_window(&state.webdriver_client, window).await.into_response()
}

async fn generate_pdf2(State(state): State<AppState>) -> Response {
    let window = open_new_tab(&state.webdriver_client, "https://kozea.github.io/WeasyPerf/samples/odyssey/odyssey.html").await;
    print_window(&state.webdriver_client, window).await.into_response()
}

async fn open_new_tab(client: &Arc<RwLock<Client>>, url: &str) -> NewWindowResponse {
    let tab = {
        // lock webdriver
        let wd = client.write().await;

        // switch to window
        let tab = wd.new_window(true).await.expect("switch");

        // open url in new tab
        wd.goto(url).await.expect("close failed");
        tab
    };

    // return tab
    tab
}

async fn print_window(client: &Arc<RwLock<Client>>, window: NewWindowResponse) -> Bytes {
    let cmd = WebDriverCommand::Print(PrintParameters::default());
    let pdf = {
        // lock webdriver
        let wd = client.write().await;

        // switch to window
        wd.switch_to_window(window.handle).await.expect("switch");

        // print window
        let buff: String = wd.issue_cmd(cmd).await.expect("failed print pdf").to_string();

        // close window
        wd.close_window().await.expect("close failed");

        // switch to blank window
        wd.switch_to_window(BLANK_TAB.get().expect("blank_tab_not_initialized").clone()).await.expect("switch");
        buff
    };
    
    Bytes::from(pdf)
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
