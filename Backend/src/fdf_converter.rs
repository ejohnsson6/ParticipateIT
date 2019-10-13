use std::fs;

pub struct FdfConverter {
    template: String,
}

impl FdfConverter {
    pub fn new() -> Result<FdfConverter, std::io::Error> {
        
        let template = fs::read_to_string("./resources/fdftempl.txt")?;

        Ok(FdfConverter {
            template: template,
        })
    }

}