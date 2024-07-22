use axum::{
    body::Bytes, extract::{FromRef, State}, http::{Result, StatusCode}, response::{Response, IntoResponse}, routing::{get, post}, Json, Router
};
// use serde::{Deserialize, Serialize};
use serde_json::map;

use std::sync::Arc;
use tokio::{signal, sync::{RwLock, OnceCell}};
use tokio::time::{sleep, Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use webdriver::command::{WebDriverCommand, PrintParameters};
use thirtyfour::prelude::*;
//
// #[derive(Clone, Debug)]
// struct AppState {
//     webdriver_client: Arc<RwLock<Client>>,
// }
//
// static BLANK_TAB: OnceCell<WindowHandle> = OnceCell::const_new();
//
// async fn init_blank_tab(client: &Arc<RwLock<Client>>,) -> &WindowHandle {
//     BLANK_TAB.get_or_init(|| async {
//         let window: WindowHandle = {
//             // lock client
//             let wd = client.read().await;
//             // get blank page
//             wd.windows().await.unwrap().get(0).unwrap().clone()
//         };
//         window
//     }).await
// }

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
    let driver = WebDriver::new("http://localhost:4444", caps).await.expect("Cannot open connection with webdriver, check if webdriver proxy is running and check if address and port are matching");
    
    driver.
    //
    //
    // // Navigate to https://wikipedia.org.
    // driver.goto("https://wikipedia.org").await?;
    // let elem_form = driver.find(By::Id("search-form")).await?;
    //
    // // Find element from element.
    // let elem_text = elem_form.find(By::Id("searchInput")).await?;
    //
    // // Type in the search terms.
    // elem_text.send_keys("selenium").await?;
    //
    // // Click the search button.
    // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    // elem_button.click().await?;
    //
    // // Look for header to implicitly wait for the page to load.
    // driver.find(By::ClassName("firstHeading")).await?;
    // assert_eq!(driver.title().await?, "Selenium - Wikipedia");
    //
    // // Always explicitly close the browser.
    sleep(Duration::from_millis(10000)).await;
    driver.quit().await;
    //
    // Ok(())
    //
    //
    // let webdriver_client: Client = ClientBuilder::rustls()
    // .expect("Failed to initialize rustls")
    // .capabilities(make_capabilities(Browers::Firefox))
    // .connect("http://localhost:4444")
    // .await.expect("couldn't connect to the client builder");
    //
    // let state = AppState {
    //     webdriver_client: Arc::new(RwLock::new(webdriver_client))
    // };
    //
    // init_blank_tab(&state.webdriver_client).await;
    //
    // // build our application with a route
    // let app = Router::new()
    //     .route("/", get(generate_pdf))
    //     .route("/2", get(generate_pdf2))
    //     .with_state(state);
    //
    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    // 
    // ()
}

// async fn generate_pdf(State(state): State<AppState>) -> Response {
//     let window = open_new_tab(&state.webdriver_client, "https://kozea.github.io/WeasyPerf/samples/columns/columns.html").await;
//     print_window(&state.webdriver_client, window).await.into_response()
// }
//
// async fn generate_pdf2(State(state): State<AppState>) -> Response {
//     let window = open_new_tab(&state.webdriver_client, "https://kozea.github.io/WeasyPerf/samples/odyssey/odyssey.html").await;
//     print_window(&state.webdriver_client, window).await.into_response()
// }
//
// async fn open_new_tab(client: &Arc<RwLock<Client>>, url: &str) -> NewWindowResponse {
//     let tab = {
//         // lock webdriver
//         let wd = client.write().await;
//
//         // switch to window
//         let tab = wd.new_window(true).await.expect("switch");
//
//         // open url in new tab
//         wd.goto(url).await.expect("close failed");
//         tab
//     };
//
//     // return tab
//     tab
// }
//
// async fn print_window(client: &Arc<RwLock<Client>>, window: NewWindowResponse) -> Bytes {
//     let cmd = WebDriverCommand::Print(PrintParameters::default());
//     let pdf = {
//         // lock webdriver
//         let wd = client.write().await;
//
//         // switch to window
//         wd.switch_to_window(window.handle).await.expect("switch");
//
//         // print window
//         let buff: String = wd.issue_cmd(cmd).await.expect("failed print pdf").to_string();
//
//         // close window
//         wd.close_window().await.expect("close failed");
//
//         // switch to blank window
//         wd.switch_to_window(BLANK_TAB.get().expect("blank_tab_not_initialized").clone()).await.expect("switch");
//         buff
//     };
//     
//     Bytes::from(pdf)
// }

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
