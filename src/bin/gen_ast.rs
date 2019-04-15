use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        println!("Usage: generate_ast <output directory>");
        std::process::exit(1);
    }
    // let output_dir = args[0];
    // define_ast(output_dir, "Expr",
}

/*
fn define_ast(output_dir: String, base_name: String, types: Vec<String>) {
    let path = output_dir + "/" + base_name + ".rs";
    let f = File::create(path).expect("Unable to create file!");
    let mut writer = BufWriter::new(f);

    writer.write("use std;\n");

    for t in types {
        let t_split = t.split(":");
        assert!(t_split.len, 2);
        let struct_name = t_split[0].trim();
        let fields = t_split[1].trim();
        define_type(writer, base_name, struct_name, fields);
    }
}

fn define_type<W: Write>(
    writer: W, base_name: String,
    struct_name: String, fields: String) {
    writer.write_fmt("struct {} \{\n", struct_name);

}
*/
