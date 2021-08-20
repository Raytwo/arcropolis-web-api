#[macro_use]
extern crate rocket;

use semver::Version;

#[get("/blacklist/<version>")]
fn index(version: &str) -> Option<String> {
    let semversion= Version::parse(version).unwrap();

    if std::path::Path::new("./version/").join(version).exists() || semversion.pre.as_str().contains("alpha") {
        Some(format!("Version {} is blacklisted", semversion))
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}