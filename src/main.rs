use file_signature::FileSignature;

mod file_signature;

const JPG: FileSignature = FileSignature::new("JPEG File Interchange Format", "jpg", &[0xff, 0xd8]);

fn main() {
    println!("{:?}", JPG);
}
