pub struct ThingBuilder {
    value_one: u8,
    value_two: i128,
    value_three: Option<String>,
}

impl Default for ThingBuilder {
    fn default() -> ThingBuilder{
        ThingBuilder{
            value_one: 1,
            value_two: -125143,
            value_three: None,
        }
    }
}

impl ThingBuilder {
    pub fn set_one(mut self, v: u8) -> Self {
        self.value_one = v;
        self
    }
    pub fn set_two(mut self, v: i128) -> Self {
        self.value_two = v;
        self
    }
    pub fn set_three(mut self, v: String) -> Self {
        self.value_three = Some(v);
        self
    }
    pub fn build(self) -> Thing {
        Thing{
            value_one: self.value_one,
            value_two: self.value_two,
            value_three: self.value_three,
        }
    }
}

#[derive(Debug)]
pub struct Thing {
    pub value_one: u8,
    pub value_two: i128,
    pub value_three: Option<String>,
}

