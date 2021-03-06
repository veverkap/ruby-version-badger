use iron::prelude::*;
use iron::{Url, status};
use iron::modifiers::Redirect;
use router::Router;
use https;

use regex::Regex;

pub fn github(req: &mut Request) -> IronResult<Response> {
    
    // Get user and repo
    let ref user = req.extensions.get::<Router>().unwrap().find("user").unwrap_or("");
    let ref repo = req.extensions.get::<Router>().unwrap().find("repo").unwrap_or("");
        
    // Get ruby version from Gemfile
    let url = String::from(format!("https://raw.githubusercontent.com/{}/{}/master/Gemfile", user, repo));
    let gemfile = https::get(url);
    let mut version = parse_gemfile(gemfile);
    println!("version from Gemfile: '{}'", version);
    
    // fall back to .ruby-version
    if version == "" {
        // Get a file
        let url = String::from(format!("https://raw.githubusercontent.com/{}/{}/master/.ruby-version", user, repo));
        version = String::from(https::get(url).trim());
        println!("version from .ruby-version: '{}'", version);
    }
    
    // Check version and set colour
    println!("version being checked: '{}'", version);
    let mut colour = "red";
    if version == "2.4.1" {
        // current
        colour = "brightgreen";
    }
    else if version == "2.3.4" {
        // previous but in lifetime
        colour = "yellow";
    }
    else if version == "2.2.7" {
        // approaching EOL
        colour = "orange";
    }
    else if version == "" || version == "404: Not Found" {
        // unknown
        version = String::from("unknown");
        colour = "lightgray";
    }

    // Create URL (without dashes in the version)
    let badge = format!("https://img.shields.io/badge/ruby-{}-{}.svg", version.replace("-", ""), colour);
    let badge_url = Url::parse(&badge).unwrap();
    
    // Send response
    Ok(Response::with((status::Found, Redirect(badge_url))))
}

fn parse_gemfile(gemfile: String) -> String {
    let re = Regex::new("^\\s*ruby\\s*[\"'](.*?)[\"']").unwrap();
    let mut s;
    match re.captures(&gemfile) {
        Some(caps) => s = caps.get(1).map_or("", |m| m.as_str()),
        None => s = ""
    }    
    String::from(s)
}
