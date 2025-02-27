use std::fmt;

pub struct Text {
    text : String,              // Text being edited
    line_lengths : Vec<usize>,  // Length of each line, not counting newlines
    dirty : bool                // True if text has been edited since line_length was calculated
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

    pub fn find_line_number(&self, index : usize) -> Result<usize,&str> {
        let mut line_count : usize = 1;
        let mut total_length : usize = 0;

        if self.text.chars().count() - 1 < index as usize {
            return Err("Index must be within string");
        }

        for length in &self.line_lengths {
            total_length += length + 1;
            if total_length > index {
                return Ok(line_count);
            }
            line_count += 1;
        }
        return Ok(line_count);
    }

    pub fn get_line_length(&self, line_no : usize) -> usize {
        return match self.line_lengths.get(line_no) {
            None => 0,
            Some(l) => *l,
        }
    }

    pub fn get_string_index(&self, line_no : usize, xoffset : usize) -> usize {
        let mut idx : usize = 0;
        for line_length in &self.line_lengths[0..line_no] {
            idx += line_length + 1;
        }
        idx += xoffset;
        idx
    }

    pub fn size(&self) -> usize {
        return self.text.chars().count();
    }
    
    pub fn line_count(&self) -> u16 {
        return self.line_lengths.len() as u16;
    }

    pub fn get_text(&self) -> &str {
        return &self.text
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
    fn test_get_line_count() {
        let t : Text = Text::new("Some text");
        let t2 : Text = Text::new("Some\ntext");
        let t3 : Text = Text::new("Some\ntext\n");
        assert_eq!(t.line_count(), 1);
        assert_eq!(t2.line_count(), 2);
        assert_eq!(t3.line_count(), 3);
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
        let t : Text = Text::new("This\nIs\nSome\nText.");

        assert_eq!(t.get_line_length(0),4);
        assert_eq!(t.get_line_length(1),2);
        assert_eq!(t.get_line_length(2),4);
        assert_eq!(t.get_line_length(3),5);
        assert_eq!(t.get_line_length(4),0);
    }

    #[test]
    fn test_check_line_count() {
        let t : Text = Text::new("This\nIs\nSome\nText.");

        assert_eq!(t.find_line_number(0),Ok(1));
        assert_eq!(t.find_line_number(1),Ok(1));
        assert_eq!(t.find_line_number(2),Ok(1));
        assert_eq!(t.find_line_number(3),Ok(1));
        assert_eq!(t.find_line_number(4),Ok(1));
        assert_eq!(t.find_line_number(5),Ok(2));
        assert_eq!(t.find_line_number(6),Ok(2));
        assert_eq!(t.find_line_number(7),Ok(2));
        assert_eq!(t.find_line_number(8),Ok(3));
        assert_eq!(t.find_line_number(9),Ok(3));
        assert_eq!(t.find_line_number(10),Ok(3));
        assert_eq!(t.find_line_number(11),Ok(3));
        assert_eq!(t.find_line_number(12),Ok(3));
        assert_eq!(t.find_line_number(13),Ok(4));
        assert_eq!(t.find_line_number(14),Ok(4));
        assert_eq!(t.find_line_number(15),Ok(4));
    }
    #[test]
    fn test_get_index_start_of_line() {
        let t : Text = Text::new("This\nIs\nSome\nText.");

        assert_eq!(t.get_string_index(0,0),0);
        assert_eq!(t.get_string_index(1,0),5);
        assert_eq!(t.get_string_index(2,0),8);
        assert_eq!(t.get_string_index(3,0),13);
    }
    #[test]
    fn test_get_index_middle_of_line() {
        let t : Text = Text::new("This\nIs\nSome\nText.");

        assert_eq!(t.get_string_index(0,1),1);
        assert_eq!(t.get_string_index(1,1),6);
        assert_eq!(t.get_string_index(2,2),10);
        assert_eq!(t.get_string_index(3,3),16);
    }
    #[test]
    fn test_get_index_end_of_line() {
        let t : Text = Text::new("This\nIs\nSome\nText.");

        assert_eq!(t.get_string_index(0,4),4);
        assert_eq!(t.get_string_index(1,3),8);
        assert_eq!(t.get_string_index(2,4),12);
        assert_eq!(t.get_string_index(3,5),18);
    }
}
