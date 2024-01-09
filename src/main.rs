use lopdf::Document;
use std::{path::Path, fs};


fn split_pdf(input_path: &Path, output_prefix: &Path) {
    let doc: Document = Document::load(input_path).unwrap();
    let file_prefix: &str = input_path.file_stem().unwrap().to_str().unwrap();
    
    let pages: Vec<u32> = (0..=doc.get_pages().len() as u32).collect();

    if output_prefix.exists() == false {
        // Create new dir
        let _ = fs::create_dir_all(output_prefix);
    } else {
        // Clear dir and create new
        let _ = fs::remove_dir_all(output_prefix);
        let _ = fs::create_dir_all(output_prefix);
    }

    for page in &pages {
        let new_pages: Vec<u32> = pages.clone();
        let mut new_doc: Document = doc.clone();

        let output_file = format!("{}/{}-{}.pdf", output_prefix.to_string_lossy(), file_prefix, &page);

        let before: Vec<u32> = new_pages.iter().take(*page as usize).copied().collect();
        let after: Vec<u32> = new_pages.iter().skip((*page + 1) as usize).copied().collect();
        println!("output_file: {}, before: {:?}, after: {:?}", output_file, before, after);
        
        let mut joined = before.clone();
        joined.extend(after);
        println!("joined: {:?}", joined);
        
        new_doc.delete_pages(&joined[..]);
    
        new_doc.save(output_file).unwrap();

    }




}

fn main() {
    let input_path = Path::new("Split.pdf");
    let output_prefix = Path::new("out");
    split_pdf(input_path, output_prefix);
}