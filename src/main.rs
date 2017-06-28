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
use std::collections::HashMap;

use quick_xml::reader::Reader;
use quick_xml::events::Event;

//Read all xml files in a dir 
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

        //Input file
        let file_name_in = format!("{}", entry);
        println!(); println!("Using input file: {}", file_name_in);
        let input = File::open(file_name_in.clone()).unwrap();

        //Output file
        let file_name_out = format!("{}{}", entry, ".txt");
        println!("Using output file: {}", file_name_out); println!();
        let mut output = File::create(file_name_out).unwrap();

        //Create xml reader
        let buffered = BufReader::new(input);
        let mut reader = Reader::from_reader(buffered);
        reader.trim_text(true);

        let mut parameters = HashMap::new(); //save all parameters found during search here
        let mut final_tag_names:Vec<String> = Vec::new(); //the final tags of a document

        let mut main_group:bool; //Helper for searching the groups of the document

        let mut buf = Vec::new(); //buffer of main reader
        loop { //Hint: No interator is implemented by the reader because of performance (see quick_xml)
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {

                    match e.name() {
                        b"group" => {
                            for attrib in e.attributes() {

                                let my_attrib:quick_xml::events::attributes::Attribute = attrib.unwrap();

                                let my_key = str::from_utf8(my_attrib.key).unwrap();
                                let my_value = str::from_utf8(my_attrib.value).unwrap();
                                
                                if my_key == "isReferenceObject" {
                                    if my_value == "true" {

                                        main_group = true;
                                        //println!("The group is reference object!");
                                        let mut names:Vec<String> = Vec::new(); //save all tags found during search here
                                        let mut buf_group = Vec::new(); //buffer of group reader
                                        loop { //check group for any tags
                                            match reader.read_event(&mut buf_group) {
                                                Ok(Event::Empty(ref e)) => {

                                                    match e.name() {
                                                        b"connection" => {
                                                            let my_result = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                                                            //println!("attributes values: {:?} - {:?}", str::from_utf8(my_result[1].key).unwrap(), str::from_utf8(my_result[1].value).unwrap());
                                                            let tag_helper: Vec<&str> = str::from_utf8(my_result[1].value).unwrap().split(|c| c == '{' || c == '}').collect();
                                                            if tag_helper.len() >= 2 {
                                                                names.push(String::from(tag_helper[1]));
                                                            }
                                                        },

                                                        b"parameter" => {
                                                            main_group = false;
                                                            let my_result = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                                                            if str::from_utf8(my_result[2].value).unwrap().contains("{") {                                                                
                                                                //println!("attributes values: {:?} - {:?}", str::from_utf8(my_result[2].key).unwrap(), str::from_utf8(my_result[2].value).unwrap());
                                                                let para_helper: Vec<&str> = str::from_utf8(my_result[2].value).unwrap().split(|c| c == '{' || c == '}').collect();
                                                                parameters.insert(String::from(str::from_utf8(my_result[0].value).unwrap()), String::from(para_helper[1]));
                                                            }
                                                        },

                                                        _ => (),
                                                    }
                                                },
                                                Ok(Event::End(ref e)) => {
                                                        match e.name() {
                                                            b"group" => {
                                                                if main_group == false {
                                                                    //println!("End of group reached!");
                                                                    break;
                                                                }
                                                            },
                                                            _ => (),
                                                        }
                                                },
                                                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                                                _ => (), // There are several other `Event`s we do not consider here
                                            }
                                        } //End of loop

                                        buf_group.clear();

                                        //Complete and save all tags of the group
                                        for my_tag in &names {
                                            //Remove curly braces 
                                            let tag_parts: Vec<&str> = my_tag.split(|c| c == '.').collect();
                                            //println!("parameter: {:?}", tag_parts[0]);
                                            if parameters.contains_key(tag_parts[0]) {
                                                final_tag_names.push(my_tag.replace(tag_parts[0], parameters.get_mut(tag_parts[0]).unwrap()));
                                                //println!("final tag: {:?}", my_tag.replace(tag_parts[0], parameters.get_mut(tag_parts[0]).unwrap()));
                                            }
                                        }
                                        //Delete the parameters of the group
                                        if parameters.len() > 0 {
                                            parameters.clear();
                                            names.clear();
                                        }

                                    }
                                }
                            }
                        }    
                        _ => (), //Nothing else to do
                    }
                }, //Start event

                Ok(Event::Empty(ref e)) => {

                    match e.name() {
                        b"connection" => {
                            let my_result = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                            // println!("attributes values: {:?} - {:?}", str::from_utf8(my_result[1].key).unwrap(), str::from_utf8(my_result[1].value).unwrap());
                            let my_name = String::from(str::from_utf8(my_result[1].value).unwrap()); 
                            if my_name.contains("#") {
                                println!("Error - no placeholder tags should be found here");
                            }
                            else {
                                //println!("{:?}", my_name);
                                if my_name.contains("{") & my_name.contains("}") {
                                    let v: Vec<&str> = my_name.split(|c| c == '{' || c == '}').collect();
                                    final_tag_names.push(String::from(v[1]));
                                }
                            }
                        },

                        b"parameter" => {
                            println!("Error - no parameters should be found here");
                        },

                        _ => (),
                    }
                }, // Empty event

                Ok(Event::Eof) => break, // exits the loop when reaching end of file

                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),

                _ => (), //Nothing to do
            }

            // clear buffer of reader
            buf.clear();
        }

        println!("--- Tagnames ---");
        
        //print tag names to screen
        for name in final_tag_names {
            println!("{:?}", name);
            write!(output, "{}\r\n", name).unwrap();
        }

        println!("--- Done ---");

    }
}
