#[macro_use] extern crate rocket;
// use rocket::response::status;
use rocket::serde::json::Json;
mod csvstuff;
use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use std::io::Write;
use chrono::{Datelike, Timelike, Local};

pub struct CORS;

// Needed implementation of CORS headers
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
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

// Accepting POST requests from Meanscout
#[post("/scouting", data="<csv>")]
async fn scouting_post(csv: Json<csvstuff::FormData<'_>>) -> Status {
    // Array for storing the passwords
    let passwords = ["ChangeMe!".to_string()];
    
    if passwords.contains(&csv.password.to_string()) == false {return Status::Unauthorized}    // If the json interpreted doesn't have the right password, it's bad
    let mut owned_string: String = "".to_owned();   // String for later to append to
    let mut thing: String;      // Placeholder string

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
        csv.topcubes.to_string(), 
        csv.bottomcubes.to_string(), 
        csv.middlecubes.to_string(), 
        csv.missedcubes.to_string(), 
        csv.topcones.to_string(), 
        csv.middlecones.to_string(), 
        csv.bottomcones.to_string(), 
        csv.missedcones.to_string(), 
        csv.topcube.to_string(), 
        csv.middlecube.to_string(), 
        csv.bottomcube.to_string(), 
        csv.missedcube.to_string(), 
        csv.topcone.to_string(), 
        csv.middlecone.to_string(), 
        csv.bottomcone.to_string(), 
        csv.missedcone.to_string(), 
        format!("{:.1}", csv.defenceplayti),
        csv.defensiverati.to_string(),
        csv.teamattemptsc.to_string().to_uppercase(),
        csv.chargestation.to_string().to_uppercase(),  
        csv.anyrobotprobl.to_string(),
        csv.fouls.to_string().replace(",", ""),
        csv.extranotes.to_string().replace(",", ""),
        csv.driveteamrati.to_string().replace(",", ""),
        csv.playstylesumm.to_string().replace(",", ""),
    ];
    for i in data.iter() {   // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i);
        if String::from(i) == csv.playstylesumm.to_string() {
            thing = format!("{}", i)
        }
        owned_string.push_str(&thing)
    }
    csvstuff::append_csv(&owned_string);    // Adds the information to data.csv
    return Status::Accepted    // Returns accepted status when done
}

// Accepting POST requests from Meanscout
#[post("/pits", data="<csv>")]
async fn pits_post(csv: Json<csvstuff::PitData<'_>>) -> Status {
    // Array for storing the passwords
    let passwords = ["ChangeMe!".to_string()];
    
    if passwords.contains(&csv.password.to_string()) == false {return Status::Unauthorized}    // If the json interpreted doesn't have the right password, it's bad
    let mut owned_string: String = "".to_owned();   // String for later to append to
    let mut thing: String;      // Placeholder string

    // Puts all of the data into a vector/array
    let data = [
    csv.team.to_string(),
    csv.absent.to_string(),
    csv.name.to_string(),
    csv.location.to_string(), //
    csv.fullteamname.to_string(),
    csv.teamlocation.to_string(),
    csv.robotname.to_string(),
    csv.drivetraintype.to_string(),
    csv.motortype.to_string(),
    csv.abilitytomoveco.to_string(),
    csv.abilitytomovecu.to_string(),
    csv.averageconecycl.to_string(),
    csv.averagecubecycl.to_string(),
    csv.successfullgrab.to_string(),
    csv.robotweightlbs.to_string(),
    csv.maxheightcapabi.to_string(),
    csv.totalwheelsused.to_string(),

    csv.endgameabilitys.to_string(),
    csv.endgametraction.to_string(),
    csv.wherearepneumat.to_string(),
    csv.whereare3dprint.to_string(),

    csv.programmedautoc.to_string(),
    csv.limelightcapabi.to_string(),
    csv.apriltagsused.to_string(),
    csv.reflectivetapeu.to_string(),
    csv.extracamerasuse.to_string(),
    csv.automationviase.to_string(),

    csv.whatisyourfavor.to_string(),
    csv.arethereanyothe.to_string(),
    ];
    for i in data.iter() {   // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i);
        if String::from(i) == csv.arethereanyothe.to_string() {
            thing = format!("{}", i)
        }
        owned_string.push_str(&thing)
    }
    csvstuff::append_pits(&owned_string);    // Adds the information to data.csv
    return Status::Accepted    // Returns accepted status when done
}


// When you send a GET request or open it in a web browser it will send the file for data.csv
#[get("/scouting")]
async fn scouting_get() -> Option<NamedFile>{
    NamedFile::open("data.csv").await.ok()    // Returns the filename
}

// Function for accepting DELETE requests to delete data.csv
#[delete("/scouting")]
async fn scouting_delete() -> String {
    csvstuff::wipe_data();
    String::from("Wiped data.csv")
}

#[rocket::main]
async fn main() {
    let config = rocket::Config::figment()
    // The address is set to 0.0.0.0 so it sets the ip to whatever the public network ip is
    .merge(("address", "0.0.0.0"))
    .merge(("port", 8000))
    // Replace the file paths below with wherever your needed pem files are for the right certifications
    // Or comment it out if you want to live the dangerous life
    .merge(("tls.certs", "/etc/letsencrypt/live/data.team4198.org/fullchain.pem"))
    .merge(("tls.key", "/etc/letsencrypt/live/data.team4198.org/privkey.pem"));
    // .finalize();
    csvstuff::init_files();
    success!("Started API");
    let _ = rocket::custom(config)
        .mount("/", routes![index, scouting_post, scouting_get, scouting_delete, pits_post, all_options])  // Just put all of the routes in here
        .attach(CORS)
        .launch()
        .await;
}

#[macro_export]
macro_rules! error {
    ( $x:expr ) => {{    
        let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("logs/scouting.log")
        .unwrap();
        let now = Local::now();
        let (is_pm, hour) = now.hour12();
         let _ = writeln!(file, "[ ERROR ] [{}] - {}", format!("{:02}-{:02}-{} {:02}:{:02}:{:02} {}", now.day(), now.month(), now.year(), hour, now.minute(), now.second(), if is_pm { "PM" } else { "AM" }), format!("{}", $x));    
    }};
}

#[macro_export]
macro_rules! success {
    ( $x:expr ) => {{    
        let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("logs/scouting.log")
        .unwrap();
        let now = Local::now();
        let (is_pm, hour) = now.hour12();
         let _ = writeln!(file, "[ SUCCESS ] [{}] - {}", format!("{:02}-{:02}-{} {:02}:{:02}:{:02} {}", now.day(), now.month(), now.year(), hour, now.minute(), now.second(), if is_pm { "PM" } else { "AM" }), format!("{}", $x));    
    }};
}

#[macro_export]
macro_rules! warning {
    ( $x:expr ) => {{    
        let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("logs/scouting.log")
        .unwrap();
        let now = Local::now();
        let (is_pm, hour) = now.hour12();
         let _ = writeln!(file, "[ WARNING ] [{}] - {}", format!("{:02}-{:02}-{} {:02}:{:02}:{:02} {}", now.day(), now.month(), now.year(), hour, now.minute(), now.second(), if is_pm { "PM" } else { "AM" }), format!("{}", $x));    
    }};
}
