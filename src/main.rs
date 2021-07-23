// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

mod r {
    use {
        std::env::{var},
        uptic::{Uptic}
    };

    fn process(data: &Uptic) -> String {
        let (d, h, m) =
            (if data.days != 0 { format!("\x1b[1;93m{}\x1b[0;97m\x1b[0;97m, ", data.days)
            } else { "".to_string() },
            if data.hours != 0 { format!("\x1b[1;94m{}\x1b[0;97m\x1b[0;97m, ", data.hours)
            } else { "".to_string() },
            if data.minutes != 0 { format!("\x1b[1;95m{}\x1b[0;97mm", data.minutes)
            } else { "".to_string() });

        format!("{}{}{}", d, h, m)
    }

    pub fn init(data: &Uptic) {
        let (user, mut lang, disp, term, desk) =
            (var("USER").unwrap(),
             var("LANG").unwrap(),
             var("DISPLAY").unwrap(),
             var("TERM").unwrap(),
             var("DESKTOP_SESSION").unwrap());

        let tty = disp.clone();

        if user.len() != 0 {
            print!("\x1b[1;94m{}\x1b[0m ", user);
        }

        if disp.len() != 0 {
            print!("\x1b[0;97m-> \x1b[1;91m{} ", tty);
            print!("\x1b[0;97md(\x1b[0;95m{}\x1b[0;97m) ", disp);
        }

        if lang.len() == 0 {
            lang = var("LANGUAGE").unwrap();
        } print!("l(\x1b[1;97m{}\x1b[0;97m) ", lang);

        print!("u({})\x1b[0;97m)", process(data));

        if term.len() != 0 {
            print!(" \x1b[1;93m{}\x1b[0;97m ", term);
        }

        if desk.len() != 0 {
            println!(" d(\x1b[1;90m{}\x1b[0;97m)", desk);
        }
    }
}

fn main() {
    r::init(&uptic::Uptic::default());
}
