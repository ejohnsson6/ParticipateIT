const TEMPLATE: &str = 
"%FDF-1.2
%����
1 0 obj 
<<
/FDF 
<<
/Fields [
{{content}}
]
>>
>>
endobj 
trailer

<<
/Root 1 0 R
>>
%%EOF
";

pub struct FdfConverter {
    content: String,
}

impl FdfConverter {
    pub fn new() -> FdfConverter {
        FdfConverter {
            content: String::new(),
        }
    }

    pub fn add_data<'a>(&'a mut self, field: &str , text: &str) -> &'a mut FdfConverter {
        let to_push = format!("<< \n/V ({0})\n/T ({1})\n>>\n", text, field);
        self.content.push_str(to_push.as_str());
        self
    }

    pub fn finish(&self) -> String {
        TEMPLATE.replace("{{content}}", self.content.as_str())
    }

}