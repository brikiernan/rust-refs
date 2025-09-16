#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        let file = File { name };
        self.contents.push(file);
    }

    fn delete_file(&mut self, index: usize) -> Option<File> {
        if index < self.contents.len() {
            Some(self.contents.remove(index))
        } else {
            None
        }
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut folder = Folder::new("MyFolder".to_string());

    folder.create_file("file1.txt".to_string());
    folder.create_file("file2.txt".to_string());
    println!("Folder after creating files: {:?}", folder);

    if let Some(deleted_file) = folder.delete_file(0) {
        println!("Deleted file: {:?}", deleted_file);
    } else {
        println!("No file found to delete at the given index.");
    }
    println!("Folder after deleting a file: {:?}", folder);

    match folder.get_file(0) {
        Some(file) => println!("Retrieved file: {:?}", file),
        None => println!("There was no file"),
    }
}
