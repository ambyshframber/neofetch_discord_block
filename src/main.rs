use std::io::{stdin, Read};
use regex::Regex;

fn main() {
    // get the neofetch output from stdin
    let mut stdin = stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap(); // just fucking panic i dont care any more
    //println!("{}", buf);
    

    // reformat to remove cursor movement
    let mut output: Vec<String> = Vec::new(); // gotta be string cuz you gotta mutate individual entries
    
    let re = Regex::new(r"\x1b\[\d+A\x1b\[\d+D\x1b\[(\d+)C").unwrap(); // match cursor movement
    let mut pad: usize = 0;
    let mut returned = false;
    let mut index = 0usize;
    let colour_grab = Regex::new(r"\x1b\[\d\dm(\x1b\[\dm)?").unwrap();
    let mut last_colour_code = ""; // grab last colour code and prepend it to the next line
    for (_i, mut line) in buf.split('\n').enumerate() {
        //println!("{}", count_non_escape_characters(line));
        if !returned {
            //print!("line {}: ", i);
            let mut caps = re.captures_iter(line);
            let colour_caps = colour_grab.find_iter(line);
            match colour_caps.last() {
                Some(c) => {
                    last_colour_code = c.as_str();
                    //println!("got colour {}", last_colour_code.replace('\x1b', "E"))
                }
                None => {
                    //println!("got no colour")
                }
            }
            if let Some(cap) = caps.next() { // you've hit cursor movement
                line = &line[cap[0].len() - 5..]; // chop movement escapes off the front
                pad = cap[1].parse().unwrap(); // get padding value
                returned = true; // "we have hit movement"
                //println!("{}", fmt_vec_with(&output, '\n'))
            }
            else {
                let mut new_line = String::from(last_colour_code);
                new_line.push_str(line);
                output.push(new_line.to_string());
                continue
            }
        }
        if line.len() == 0 { // bail out once you hit the empty line before colour blocks
            break
        }
        if output.len() <= index { // make sure you have a line to push to
            output.push(String::new())
        }
        let line_len = count_non_escape_characters(&output[index]); // get display length
        let mut new_line = String::new();
        for _ in 0..(pad - line_len) { // pad
            new_line.push(' ')
        }
        if line.len() > 3 && &line[..3] != "\x1b[?" {
            new_line.push_str(&line[5..]); // push current line
        }
        else {
            new_line.push_str(line)
        }
        output[index].push_str(&new_line);
        index += 1;
    }

    let output_string = fmt_vec_with(&output, '\n');
    let output_string = output_string.replace("\x1b[?25l\x1b[?7l", "");
    let output_string = output_string.replace("\x1b[?25h\x1b[?7h", "");

    println!("{}", output_string)
}

fn count_non_escape_characters(s: &str) -> usize {
    let re = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap(); // thank the lord for stack overflow
    let matches = re.find_iter(s);
    let mut ret = s.len();
    for mat in matches {
        let len = mat.end() - mat.start();
        ret -= len
    }
    ret
}
fn fmt_vec_with(v: &Vec<String>, c: char) -> String {
    let mut ret = String::new();
    for i in v {
        ret.push_str(&i);
        ret.push(c)
    }
    ret
}