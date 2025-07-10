extern crate skeptic;

fn main() {
    // Generate tests for README.md
    skeptic::generate_doc_tests(&["README.md"]);
}
