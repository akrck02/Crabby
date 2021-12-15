pub struct Class {
    indent: i32,
    code : String
}

impl Class {

    /**
     * Add to identation index
     */
    pub fn indent(&mut self) -> &mut Class{
        self.indent += 1;
        return self;
    }

    /**
     * set identation index back;
     */
    pub fn indent_back(&mut self) -> &mut Class {
        self.indent -= 1;
        return self;
    }

    /**
     * Write code depending on identation
     */
    pub fn writeln(&mut self, text : &str ) -> &mut Class{
        
        for _i in 0..self.indent {
            self.code.push_str("\t");
        }
        self.code.push_str(text);
        self.code.push_str("\n");
        return self;
    }

    /**
     * Get formatted code
     */
    pub fn to_string(&mut self) -> String { 
        return self.code.to_owned();
    }
}

    /**
     * Public constructor
     */
    pub fn instance() -> Class {
        return Class { indent: 0 , code: String::new() };
    }