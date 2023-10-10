use axum::{
    http::{self, HeaderValue, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
    extract,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
mod csvstuff;
mod settings;
use serde_json::Value;
use std::io::Write;
use std::fs::File;

#[tokio::main]
async fn main() {
    let frontend = async {
        let app = Router::new().route("/", get(html));
        serve(app, 4000).await;
    };

    let backend = async {
        let app = Router::new().route("/scouting", post(scouting_post)).layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET])
                .allow_headers([http::header::CONTENT_TYPE]),
        );
        serve(app, 8000).await;
    };

    println!("running");
    tokio::join!(frontend, backend);
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn scouting_post(extract::Json(data): Json<csvstuff::Data>) -> StatusCode {
    println!("- Scouting data was posted to the server");
    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    let mut hash_vec: Vec<(&String, &Value)> = data.data.iter().collect();

    hash_vec.sort_by(|a, b| {
        a.1.as_object().expect("Failed to turn into object").get_key_value("category").expect("Failed to get category").1.as_str().expect("Failed to get content").cmp(
            b.1.as_object().expect("Failed to turn into object").get_key_value("category").expect("Failed to get category").1.as_str().expect("Failed to get content")
        )
    });

    if csvstuff::file_empty("data/data.csv".into()).unwrap() {
        let mut header: String = "".to_owned();
        println!("yeah");
        let mapped: Vec<String> = hash_vec.iter().map(|point| point.0.to_string()).collect();
        for val in mapped {header.push_str(format!("{},", val).as_str())}
        let _ = csvstuff::append_csv(&header);
    }

    for i in hash_vec {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i.1.as_object().unwrap().get_key_value("content").expect("Failed to get content").1.to_string().replace(",", ""));
        owned_string.push_str(&thing)
    }
    match csvstuff::append_csv(&owned_string) {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error));
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    } // Adds the information to data.csv
    return StatusCode::ACCEPTED; // Returns accepted status when done
}

async fn html() -> impl IntoResponse {
    Html(
        r#"
        hi
        <script>
            fetch('http://127.0.0.1:8000/scouting')
              .then(response => response.json())
              .then(data => console.log(data));
        </script>
        "#,
    )
}

async fn json() -> impl IntoResponse {
    Json(vec!["one", "two", "three"])
}

#[macro_export]
macro_rules! success {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().unwrap().logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ SUCCESS ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}

#[macro_export]
macro_rules! warning {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().unwrap().logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ WARNING ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}

#[macro_export]
macro_rules! error {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().unwrap().logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ ERROR ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}