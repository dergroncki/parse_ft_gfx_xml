extern crate clap;
extern crate quick_xml;

use clap::{Arg, App};

use std::ffi::OsStr;

use std::path::Path;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;

use std::str;

use quick_xml::reader::Reader;
use quick_xml::events::Event;

fn read_file_names(path: &Path) -> Vec<String> {
    let mut names:Vec<String> = Vec::new();

    let entries = fs::read_dir(path).unwrap();

    for entry in entries {

        let my_entry = entry.unwrap();

        //Determine the file extention
        let fullpath = my_entry.path().into_os_string().into_string().unwrap();
        let file_path = std::path::Path::new(&fullpath);
        let content_type = match file_path.extension().and_then(OsStr::to_str) {
                Some("xml") => "text/xml",
                _ => "",
        };
        
        //Save fullpath if the file extention is "xml"
        if content_type == "text/xml" {
            names.push(my_entry.path().into_os_string().into_string().unwrap()); 
        }
    }

    //Return filenames
    names
}

fn main(){

    //Clap
    let matches = App::new("extract tags from gfx-xml")
                        .version("0.1")
                        .about("Extract tags from factorytalk gfx-xml file!")
                        .author("Michael Groncki")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the input dir to use")
                            .required(true)
                            .index(1))
                        .get_matches();    

    let path = Path::new(matches.value_of("INPUT").unwrap());

    for entry in read_file_names(path) {

        let file_name_in = format!("{}", entry);
        println!(); println!("Using input file: {}", file_name_in); println!();
        let input = File::open(file_name_in.clone()).unwrap();

        //Output file
        let file_name_out = format!("{}{}", entry, ".txt");
        println!(); println!("Using output file: {}", file_name_out); println!();
        let mut output = File::create(file_name_out).unwrap();

        //Create xml reader
        let buffered = BufReader::new(input);
        let mut reader = Reader::from_reader(buffered);
        reader.trim_text(true);

        let mut buf = Vec::new();

        let mut tag_names:Vec<String> = Vec::new(); //save all tags found during search here

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf) {
                // Ok(Event::Start(ref e)) => {

                //     match e.name() {
                //         b"group" => {
                //             let my_result = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();

                //             for attrib in e.attributes() {

                //                 let my_attrib:quick_xml::events::attributes::Attribute = attrib.unwrap();

                //                 let my_key = str::from_utf8(my_attrib.key).unwrap();
                //                 let my_value = str::from_utf8(my_attrib.value).unwrap();
                                
                //                 if my_key == "isReferenceObject" {
                //                     if my_value == "true" {
                //                         // println!("The group is reference object!");
                //                     }
                //                 }
                //             }

                //             for rs in &my_result {
                //                 if str::from_utf8(rs.key).unwrap() == "isReferenceObject" {
                //                     // println!("{:?}", str::from_utf8(rs.key).unwrap());
                //                 }
                //             }
                //         }    

                //         _ => (),
                //     }
                // },
                Ok(Event::Empty(ref e)) => {

                    match e.name() {
                        b"connection" => {
                            let my_result = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                            // println!("attributes values: {:?} - {:?}", str::from_utf8(my_result[1].key).unwrap(), str::from_utf8(my_result[1].value).unwrap());
                            tag_names.push(String::from(str::from_utf8(my_result[1].value).unwrap()));
                        },

                        b"parameter" => {
                            let my_result = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                            if str::from_utf8(my_result[2].value).unwrap().contains("{") {
                                // println!("attributes values: {:?} - {:?}", str::from_utf8(my_result[2].key).unwrap(), str::from_utf8(my_result[2].value).unwrap());
                                tag_names.push(String::from(str::from_utf8(my_result[2].value).unwrap()));
                            }
                        },

                        _ => (),
                    }
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        println!("--- Tagnames ---");
        
        let mut tag_name_placeholder:Vec<String> = Vec::new(); //save tags which contains a placeholder here
        let mut final_tag_names:Vec<String> = Vec::new(); //the final tags

        for tag in tag_names {
            //Remove curly braces 
            let v: Vec<&str> = tag.split(|c| c == '{' || c == '}').collect();

            if tag.contains("#") {
                tag_name_placeholder.push(String::from(v[1])); 
            }

            if tag.contains("[PLC") {
                for placeholder in &tag_name_placeholder {
                    if placeholder.contains("#1") { final_tag_names.push(placeholder.replace("#1", &String::from(v[1]))); }
                    if placeholder.contains("#2") { final_tag_names.push(placeholder.replace("#2", &String::from(v[1]))); }
                    if placeholder.contains("#3") { final_tag_names.push(placeholder.replace("#3", &String::from(v[1]))); }
                    if placeholder.contains("#4") { final_tag_names.push(placeholder.replace("#4", &String::from(v[1]))); }
                    if placeholder.contains("#5") { final_tag_names.push(placeholder.replace("#5", &String::from(v[1]))); }
                    if placeholder.contains("#6") { final_tag_names.push(placeholder.replace("#6", &String::from(v[1]))); }
                    if placeholder.contains("#7") { final_tag_names.push(placeholder.replace("#7", &String::from(v[1]))); }
                    if placeholder.contains("#8") { final_tag_names.push(placeholder.replace("#8", &String::from(v[1]))); }
                    if placeholder.contains("#9") { final_tag_names.push(placeholder.replace("#9", &String::from(v[1]))); }
                }
                &tag_name_placeholder.clear();
            }
            
        }

        //print tag names to screen
        for name in final_tag_names {
            println!("{:?}", name);
            write!(output, "{}\r\n", name).unwrap();
        }

        println!("--- Done ---");

    }
}
