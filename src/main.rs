use std::{
    fs::File,
    io::{LineWriter, Result, Write},
};

struct FileDetail {
    entity_name: String,
    file_extention: String,
    create_package: bool,
}

impl FileDetail {
    fn new(entity_name: String) -> FileDetail {
        FileDetail {
            entity_name,
            file_extention: String::from(".java"),
            create_package: false,
        }
    }
}

pub trait ControllerCreator {
    fn file_name(&self) -> String;
    fn class_name(&self) -> String;
    fn create(&self) -> Result<()>;
}

impl ControllerCreator for FileDetail {
    fn file_name(&self) -> String {
        format!("{}Controller.{}", self.entity_name, self.file_extention)
    }

    fn class_name(&self) -> String {
        format!("{}Controller", self.entity_name)
    }

    fn create(&self) -> Result<()>{
        let detail : Vec<String> = vec![
            format!("public class {} {{", self.class_name()),
            format!("\t@GetMapping\n\tpublic ResponseEntity<List<{entity_name}>>  all{entity_name}s () {{ \n\t}}", entity_name=self.entity_name),
            format!("}}"),
        ];

        let file = File::create(self.file_name()).expect("Can not create file");
        let mut file = LineWriter::new(file);

        for line in detail {
            file.write(line.as_bytes())?;
            file.write(b"\n")?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let file_detail: FileDetail = FileDetail::new(String::from("User"));
    ControllerCreator::create(&file_detail)?;
    Ok(())
}
