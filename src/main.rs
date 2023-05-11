#[macro_use]
extern crate rocket;
use chrono;

use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::http::Status;
use rocket::{Request, Response};
use serde_json::Value;

use std::io::Write;
use std::fs::File;
use std::io::prelude::*;

mod csvstuff;
mod settings;
mod catchers;

// Just a silly little easter egg
#[get("/teapot")]
async fn teapot() -> Status {
    Status::ImATeapot
}

// A basic implementation of Catching Headers
struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

struct PassKey<'r>(&'r str);

// Request header checks
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

// Password header checks
#[rocket::async_trait]
impl<'r> FromRequest<'r> for PassKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        
        let passwords = ["ChangeMe!".to_string()];
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str, passwords: Vec<String>) -> bool {
            passwords.contains(&format!("{}", key))
        }

        match req.headers().get_one("x-pass-key") {
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
            Some(key) if is_valid(key, passwords.into()) => Outcome::Success(PassKey(key)),
            Some(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
        }
    }
}

#[get("/sensitive")]
fn sensitive(_key: ApiKey<'_>) -> &'static str {
    "Sensitive data."
}

pub struct CORS;

// Needed implementation of CORS headers
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

// Mainly so it doesn't keep returning 404 on any webpage that's pulled up on here
#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("favicon.ico").await.ok()
}

// Accepting POST requests from Meanscout
#[post("/scouting", format = "json", data = "<json>")]
async fn scouting_post(_key: PassKey<'_>, json: String) -> Status {

    let mut file = std::fs::OpenOptions::new()
      .append(true)
      .open("test.json")
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", json));

    let data: csvstuff::Data = serde_json::from_str(&json).unwrap();

    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    let mut hash_vec: Vec<(&String, &Value)> = data.data.iter().collect();

    hash_vec.sort_by(|a, b| a.1.as_object().unwrap().get_key_value("category").unwrap().1.as_str().unwrap().cmp(b.1.as_object().unwrap().get_key_value("category").unwrap().1.as_str().unwrap()));
    for i in hash_vec {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i.1.as_object().unwrap().get_key_value("content").expect("Failed to get content").1.to_string().replace(",", ""));
        // if String::from(i) == json.data.get_key_value("playstylesumm").to_string() {
        //     thing = format!("{}", i.replace(",", ""))
        // }
        owned_string.push_str(&thing)
    }
    match csvstuff::append_csv(&owned_string) {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error))
        }
    } // Adds the information to data.csv
    return Status::Accepted; // Returns accepted status when done
}

#[allow(unused)]
#[post("/scoutingnew", data = "<csv>")]
async fn scouting_post_test(csv: Json<csvstuff::FormData<'_>>) -> Status {
    // Array for storing the passwords
    let passwords = ["ChangeMe!".to_string()];

    if passwords.contains(&csv.password.to_string()) == false {
        return Status::Unauthorized;
    } // If the json interpreted doesn't have the right password, it's bad
    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    // Puts all of the data into a vector/array
    let data = [
        csv.team.to_string(), 
        csv.matchnum.to_string(), 
        csv.absent.to_string().to_uppercase(), 
        csv.name.to_string(), 
        csv.location.to_string(), 
        csv.teamleftcommu.to_string().to_uppercase(), 
        csv.teamcollected.to_string().to_uppercase(), 
        csv.autochargesta.to_string(),
        // format!("{:?}", csv.toggletesting),
        csv.toggletesting[0].value.to_string(),
        csv.toggletesting[1].value.to_string(),
        csv.toggletesting[2].value.to_string(),
        csv.toggletesting[3].value.to_string(),
        csv.toggletesting[4].value.to_string(),
        csv.toggletesting[5].value.to_string(),
        csv.toggletesting[6].value.to_string(),
        csv.toggletesting[7].value.to_string(),
        csv.toggletesting[8].value.to_string(),
        csv.toggletesting[9].value.to_string(),
        csv.toggletesting[10].value.to_string(),
        csv.toggletesting[11].value.to_string(),
        csv.toggletesting[12].value.to_string(),
        csv.toggletesting[13].value.to_string(),
        csv.toggletesting[14].value.to_string(),
        csv.toggletesting[15].value.to_string(),
        csv.toggletesting[16].value.to_string(),
        csv.toggletesting[17].value.to_string(),
        csv.toggletesting[18].value.to_string(),
        csv.toggletesting[19].value.to_string(),
        csv.toggletesting[20].value.to_string(),
        csv.toggletesting[21].value.to_string(),
        csv.toggletesting[22].value.to_string(),
        csv.toggletesting[23].value.to_string(),
        csv.toggletesting[24].value.to_string(),
        csv.toggletesting[25].value.to_string(),
        csv.toggletesting[26].value.to_string(),
        
        // csv.topcubes.to_string(), 
        // csv.bottomcubes.to_string(), 
        // csv.middlecubes.to_string(), 
        // csv.missedcubes.to_string(), 
        // csv.topcones.to_string(), 
        // csv.middlecones.to_string(), 
        // csv.bottomcones.to_string(), 
        // csv.missedcones.to_string(), 
        // csv.topcube.to_string(), 
        // csv.middlecube.to_string(), 
        // csv.bottomcube.to_string(), 
        // csv.missedcube.to_string(), 
        // csv.topcone.to_string(), 
        // csv.middlecone.to_string(), 
        // csv.bottomcone.to_string(), 
        // csv.missedcone.to_string(), 
        format!("{:.1}", csv.defenseplayti),
        csv.defensiverati.to_string(),
        csv.teamattemptsc.to_string().to_uppercase(),
        csv.chargestation.to_string().to_uppercase(), 
        // csv.links.to_string(),
        csv.anyrobotprobl.to_string(),
        csv.fouls.to_string(),
        csv.extranotes.to_string(),
        csv.driveteamrati.to_string(),
        csv.playstylesumm.to_string(),
    ];
    for i in data.iter() {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i.replace(",", ""));
        if String::from(i) == csv.playstylesumm.to_string() {
            thing = format!("{}", i.replace(",", ""))
        }
        owned_string.push_str(&thing)
    }
    match csvstuff::append_csv(&owned_string) {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error))
        }
    } // Adds the information to data.csv
    return Status::Accepted; // Returns accepted status when done
}

// Accepting POST requests from Meanscout
#[post("/test", data = "<csv>")]
async fn test_post(csv: Json<csvstuff::FormData<'_>>) -> Status {
    // Array for storing the passwords
    let passwords = ["ChangeMe!".to_string()];

    if passwords.contains(&csv.password.to_string()) == false {
        return Status::Unauthorized;
    } // If the json interpreted doesn't have the right password, it's bad
    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    // Puts all of the data into a vector/array
    let data = [
        csv.team.to_string(), 
        csv.matchnum.to_string(), 
        csv.absent.to_string().to_uppercase(), 
        csv.name.to_string().replace(",", ""), 
        csv.location.to_string(), 
        csv.teamleftcommu.to_string().to_uppercase(), 
        csv.teamcollected.to_string().to_uppercase(), 
        csv.autochargesta.to_string(),
        // csv.topcubes.to_string(), 
        // csv.bottomcubes.to_string(), 
        // csv.middlecubes.to_string(), 
        // csv.missedcubes.to_string(), 
        // csv.topcones.to_string(), 
        // csv.middlecones.to_string(), 
        // csv.bottomcones.to_string(), 
        // csv.missedcones.to_string(), 
        // csv.topcube.to_string(), 
        // csv.middlecube.to_string(), 
        // csv.bottomcube.to_string(), 
        // csv.missedcube.to_string(), 
        // csv.topcone.to_string(), 
        // csv.middlecone.to_string(), 
        // csv.bottomcone.to_string(), 
        // csv.missedcone.to_string(), 
        format!("{:.1}", csv.defenseplayti),
        csv.defensiverati.to_string(),
        csv.teamattemptsc.to_string().to_uppercase(),
        csv.chargestation.to_string().to_uppercase(),
        // csv.links.to_string(),
        csv.anyrobotprobl.to_string(),
        csv.fouls.to_string().replace(",", ""),
        csv.extranotes.to_string().replace(",", ""),
        csv.driveteamrati.to_string().replace(",", ""),
        csv.playstylesumm.to_string().replace(",", ""),
    ];
    for i in data.iter() {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i);
        if String::from(i) == csv.playstylesumm.to_string() {
            thing = format!("{}", i)
        }
        owned_string.push_str(&thing)
    }
    match csvstuff::append_test(&owned_string) {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error))
        }
    } // Adds the information to tes.ctsv
    return Status::Accepted; // Returns accepted status when done
}

// Accepting POST requests from Meanscout
#[post("/pits", data = "<csv>")]
async fn pits_post(csv: Json<csvstuff::PitData<'_>>) -> Status {
    // Array for storing the passwords
    let passwords = ["ChangeMe!".to_string()];

    if passwords.contains(&csv.password.to_string()) == false {
        return Status::Unauthorized;
    } // If the json interpreted doesn't have the right password, it's bad
    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    // Puts all of the data into a vector/array
    let data = [
    csv.team.to_string(),
    csv.absent.to_string(),
    csv.name.to_string(),
    csv.location.to_string(), //
    csv.fullteamname.to_string().replace(",", ""),
    csv.teamlocation.to_string().replace(",", ""),
    csv.robotname.to_string().replace(",", ""),
    csv.drivetraintype.to_string().replace(",", ""),
    csv.motortype.to_string().replace(",", ""),
    csv.abilitytomoveco.to_string().replace(",", ""),
    csv.abilitytomovecu.to_string().replace(",", ""),
    csv.averageconecycl.to_string().replace(",", ""),
    csv.averagecubecycl.to_string().replace(",", ""),
    csv.successfullgrab.to_string().replace(",", ""),
    csv.robotweightlbs.to_string().replace(",", ""),
    csv.maxheightcapabi.to_string().replace(",", ""),
    csv.totalwheelsused.to_string().replace(",", ""),

    
    // csv.endgametraction.to_string(),
    csv.wherearepneumat.to_string().replace(",", ""),
    csv.whereare3dprint.to_string().replace(",", ""),

    csv.programmedautoc.to_string().replace(",", ""),
    // csv.limelightcapabi.to_string(),
    csv.apriltagsused.to_string().replace(",", ""),
    csv.reflectivetapeu.to_string().replace(",", ""),
    csv.extracamerasuse.to_string().replace(",", ""),
    csv.automationviase.to_string().replace(",", ""),

    csv.endgameabilitys.to_string().replace(",", ""),
    csv.whatisyourfavor.to_string().replace(",", ""),
    csv.drivestationsum.to_string().replace(",", ""),
    csv.arethereanyothe.to_string().replace(",", ""),
    ];
    for i in data.iter() {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i);
        if String::from(i) == csv.arethereanyothe.to_string() {
            thing = format!("{}", i)
        }
        owned_string.push_str(&thing)
    }
    match csvstuff::append_pits(&owned_string) {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error))
        }
    } // Adds the information to data.csv
    return Status::Accepted; // Returns accepted status when done
}


// When you send a GET request or open it in a web browser it will send the file for data.csv
#[get("/scouting")]
async fn scouting_get() -> Option<NamedFile> {
    NamedFile::open("data.csv").await.ok() // Returns the filename
}

// Function for accepting DELETE requests to delete data.csv
#[delete("/scouting")]
async fn scouting_delete() -> String {
    csvstuff::wipe_data();
    String::from("Wiped data.csv")
}

// Accessing Logs
#[get("/logs")]
async fn logs() -> String {
    let mut file = File::open("logs/scouting.log").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}

#[rocket::main]
async fn main() {
    if cfg!(debug_assertions) {
        // let settings = config::Settings::new().unwrap();
        // println!("{:?}", settings.thing.thin);
        // config::convert_to_mean();
    }


    let config = rocket::Config::figment()
    // The address is set to 0.0.0.0 so it sets the ip to whatever the public network ip is
    .merge(("address", "0.0.0.0"))
    .merge(("port", 8000))
    // Replace the file paths below with wherever your needed pem files are for the right certifications
    // Or comment it out if you want to live the dangerous life
    .merge(("tls.certs", "/etc/letsencrypt/live/data.team4198.org/fullchain.pem"))
    .merge(("tls.key", "/etc/letsencrypt/live/data.team4198.org/privkey.pem"));
    // .finalize();
    match csvstuff::init_files() {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error))
        }
    }
    success!("Started API");
    let _ = rocket::custom(config)
        .mount(
            "/",
            routes![
                favicon,
                index,
                scouting_post,
                test_post,
                scouting_get,
                logs,
                scouting_delete,
                pits_post,
                all_options,
                sensitive,
                teapot
            ],
        ) // Just put all of the routes in here
        .register(
            "/",
            catchers![
                catchers::default_catcher, // All of the status code catchers put in catchers.rs goes here
                catchers::not_found,
                catchers::im_a_teapot,
                catchers::bad_request,
                catchers::content_too_large,
                catchers::forbidden,
            ],
        )
        .attach(CORS)
        .launch()
        .await;
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
