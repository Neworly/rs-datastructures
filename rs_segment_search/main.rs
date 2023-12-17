// Segment-search, Search-file, read-context

/*
    prefix -> 'P next_segments'
    Segment search: we will search for an entity using strings
    each string will be stored onto a colletion.
    * back_one: depending on how many '.' we will go back to Nst children;
    * start_dir: our segment-search will start from: '/'
*/

/* 
    prefix -> 'S filename'
    Search file: input a file name and his format, search into every folder starting from '/' until matches.
*/

use core::panic;
use std::{collections::{HashMap, HashSet}, ops::Add, cell::RefCell};

mod cml;
mod input;

use cml::CommandAction;

#[derive(Debug)] enum Errors {
    DuplicatePrefix,
}


fn segment_search(default_path: String) -> Result<cml::Ok, cml::Err> {
    let track = RefCell::new(String::from(default_path));

    'main: loop {
        let next_segment = "Users/pioca/loam";
        let chunks = next_segment.split("/").collect::<Vec<&str>>();
        println!("chunks: {:?}", chunks);
        for chunk in chunks {
            let predict = track.borrow_mut().clone().add(chunk);
            if let Err(_) = std::fs::read_dir(predict.clone()) {
                println!("warning: `{chunk}` in `{}` not found.", predict);
                break 'main;
            }
            track.borrow_mut().push_str(&chunk.to_string().add("/"));
            println!("updated: {:?}", track);            
            continue;
        }
    }
    println!("{}", "-".repeat(10));

    let borrow = track.borrow_mut().clone();

    if let Ok(parent) =  std::fs::read_dir(borrow.clone()) {
        for child in parent {
            println!("parent: [{borrow}], child = [{:?}]", child.unwrap());
        }
    }
    return Ok(cml::Ok::Passed)
}

fn search_file(name: String) -> Result<cml::Ok, cml::Err> {
    return Ok(cml::Ok::Passed)
}

fn build_commands() -> Result<Vec<CommandAction<cml::FPointer>>, Errors> {
    let mut hash = HashSet::new();

    let commands: Vec<CommandAction<cml::FPointer>> = vec![ 
        CommandAction::new("P", Box::new(|path: String| {
            return segment_search(path)
        })), CommandAction::new("S", Box::new(|name: String| {
            return search_file(name)
        }))
    ];

    for command in commands.iter() {
        if hash.get(&command.prefix()).is_some() {
            return Err(Errors::DuplicatePrefix)
        }
        hash.insert(command.prefix());
    }

    return Ok(commands)
}

fn main() {
    cml::use_command("P", &build_commands().unwrap(), "/");
}