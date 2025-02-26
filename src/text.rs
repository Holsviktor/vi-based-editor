use std::fmt;

struct Text {
    text : String,              // Text being edited
    line_lengths : Vec<usize>,    // Length of each line, not counting newlines
    dirty : bool        // True if text has been edited since line_length was calculated
                                // last
}

impl Text {
    pub fn new(s : &str) -> Text{
        let mut t = Text {
            text : String::from(s),
            line_lengths : vec![0],
            dirty : true,
        };
        t.refresh_line_lengths();
        return t;
    }
    fn refresh_line_lengths(&mut self) {
        if self.dirty {
            let mut line_count : usize = 1;
            let mut line_length : usize = 0;

            for c in self.text.chars() {
                match c {
                    '\n' => {
                        if self.line_lengths.len() < line_count as usize {
                            self.line_lengths.push(0);
                        }
                        self.line_lengths[(line_count - 1) as usize] = line_length;
                        line_count += 1;
                        line_length = 0;
                    }
                    _ => line_length += 1, // Assume only normal chars for now
                }
            }
            if self.line_lengths.len() < line_count as usize {
                self.line_lengths.push(0);
            }
            self.line_lengths[(line_count - 1) as usize] = line_length;
            self.dirty = false;
        }

    }

    pub fn find_line_number(&mut self, index : usize) -> Result<usize,&str> {
        let mut line_count : usize = 1;
        let mut total_length : usize = 0;

        if self.text.chars().count() - 1 < index as usize {
            return Err("Index must be within string");
        }
        self.refresh_line_lengths();

        for length in &self.line_lengths {
            total_length += length + 1;
            eprintln!("l: {}  i: {}", total_length, index);
            if total_length > index {
                return Ok(line_count);
            }
            line_count += 1;
        }
        return Ok(line_count);
    }

    pub fn get_line_length(&mut self, line_no : usize) -> usize {
        self.refresh_line_lengths();
        return match self.line_lengths.get(line_no) {
            None => 0,
            Some(l) => *l,
        }
    }

    pub fn write_char<'a>(&mut self, c : &'a str, idx : usize) -> Result<&'a str, &'a str> {
        match c.chars().count() {
            1 => {
                self.text.insert_str(idx, &c);
                match c {
                    "\n" => {
                        let current_line = self.find_line_number(idx).ok().unwrap() - 1;
                        self.line_lengths.insert(current_line, 0);
                        self.dirty = true;
                        self.refresh_line_lengths();
                    }
                    _ => {
                        if !self.dirty {
                            // Optimization
                            let current_line = self.find_line_number(idx).ok().unwrap();
                            self.line_lengths[current_line - 1] += 1;
                        }
                    }
                }
                Ok(c)
            }
            0 => Err("Cannot push empty string."),
            _ => Err("Cannot push multiple chars."),
        }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let t : Text = Text::new("Some text");
        assert_eq!(format!("{}", t), "Some text");
    }

    #[test]
    fn test_append_character() {
        let mut t : Text = Text::new("Some text");

        match t.write_char(".", 9) {
            Ok(_) => print!(""),
            Err(e) => eprintln!("{}", e),
        }

        assert_eq!(format!("{}", t), "Some text.");
        assert_eq!(t.line_lengths[0], t.text.chars().count().try_into().unwrap());
    }

    #[test]
    fn test_insert_characters() {
        let mut t : Text = Text::new("ometxt.");

        match t.write_char("S", 0) {
            Ok(_) => print!(""),
            Err(e) => eprintln!("{}", e),
        }

        assert_eq!(format!("{}", t), "Sometxt.");
        assert_eq!(t.line_lengths[0], t.text.chars().count().try_into().unwrap());

        match t.write_char(" ", 4) {
            Ok(_) => print!(""),
            Err(e) => eprintln!("{}", e),
        }

        assert_eq!(format!("{}", t), "Some txt.");
        assert_eq!(t.line_lengths[0], t.text.chars().count().try_into().unwrap());

        match t.write_char("e", 6) {
            Ok(_) => print!(""),
            Err(e) => eprintln!("{}", e),
        }

        assert_eq!(format!("{}", t), "Some text.");
        assert_eq!(t.line_lengths[0], t.text.chars().count().try_into().unwrap());
    }

    #[test]
    fn test_line_length_getter() {
        let mut t : Text = Text::new("This\nIs\nSome\nText.");

        assert_eq!(t.get_line_length(0),4);
        assert_eq!(t.get_line_length(1),2);
        assert_eq!(t.get_line_length(2),4);
        assert_eq!(t.get_line_length(3),5);
        assert_eq!(t.get_line_length(4),0);
    }

    #[test]
    fn test_check_line_count() {
        let mut t : Text = Text::new("This\nIs\nSome\nText.");

        assert_eq!(t.find_line_number(1),Ok(1));
        assert_eq!(t.find_line_number(5),Ok(2));
        assert_eq!(t.find_line_number(8),Ok(3));
        assert_eq!(t.find_line_number(13),Ok(4));
    }
}
