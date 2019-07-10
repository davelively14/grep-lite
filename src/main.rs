extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Trainer functions:");
    trainer();

    println!("No input grep:");
    no_input_grep();

    println!("Reading from files trainer:");
    read_from_file("README.md");

    println!("");
    println!("###################");
    println!("# Actual CLI grep #");
    println!("###################");
    grep();
}

fn trainer() {
    basic_grep();
    arrays();
    vector_grep();
}

fn grep() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    println!("Input: {}", pattern);
    println!("");

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn read_from_file(filename: &str) {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    for local_line in reader.lines() {
        let line = local_line.unwrap();
        println!("{} // ({} bytes long)", line, line.len());
    }
}

fn no_input_grep() {
    // "unwraps" a result, or panics if an error occurs (MTF)
    let re = Regex::new("picture").unwrap();

    let quote = "Every face, every shop, bedroom window, public-house and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn basic_grep() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for (idx, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = idx + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

// Items in arrays are accessed directly on the stack instead of a pointer,
// like in C. The compiler knows the size of an array's members and calculates
// memory offsets itself, so no pointer math needed.
fn arrays() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    // &arrays reference a slice of contiguous memory, which can be iterated on
    // without calling iter()
    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }

        print!("\t(Σ{:?} = {})", a, sum);
        println!("");
    }
}

fn vector_grep() {
    // PARAMS
    let context_lines = 2;
    let needle = "book";
    let haystack = "Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    // INITIALIZATION

    // tags contain the index for the line where the matches exist
    let mut tags: Vec<usize> = Vec::new();

    // ctx contains a vector per match to hold the context line numbers and the
    // line String for each match
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    // PASS 1
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            // Add line number to end of tags
            tags.push(i);

            // Reserves spaces for all context_lines plus the line that matches
            // Vec<T> will perform best when you provide it with a size hint
            let v = Vec::with_capacity(2 * context_lines + 1);
            ctx.push(v);
        }
    }

    if tags.len() == 0 {
        return;
    }

    // PASS 2
    // For each tag, at every line, check to see if a match is nearby, and if so
    // add to ctx
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            // saturating_sub is subtraction that returns 0 on integer overflow
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    // OUTPUT
    for local_ctx in ctx.iter() {
        // ref borrows line
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }

    print!("Tags:\t[");
    for (i, local_tag) in tags.iter().enumerate() {
        if i + 1 == tags.len() {
            print!("{}", local_tag);
        } else {
            print!("{}, ", local_tag);
        }
    }
    println!("]");
}
