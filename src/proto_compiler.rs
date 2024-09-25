use std::error::Error;
use std::path::Path;
use std::process::Command;

pub struct ProtoParser;

impl ProtoParser {
    pub fn parse(proto_path: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let proto_file = Path::new(proto_path);
        let out_dir = Path::new(output);

        if !proto_file.exists() {
            return Err("File not found!".into())
        }

        // Check if protoc is available
        let protoc_check = Command::new("protoc").arg("--version").output();
        match protoc_check {
            Ok(output) => println!("protoc version: {}", String::from_utf8_lossy(&output.stdout)),
            Err(e) => return Err(format!("Failed to execute protoc: {}", e).into()),
        }

        // Use protoc command directly for more control and error information
        let output = Command::new("protoc")
            .arg(format!("--proto_path={}", proto_file.parent().unwrap().to_str().unwrap()))
            .arg(format!("--rust_out=kernel=cpp:{}", out_dir.to_str().unwrap()))
            .arg("--rust_opt=experimental-codegen=enabled")  // Add this line to enable experimental codegen
            .arg(proto_file.to_str().unwrap())
            .output()?;

        if !output.status.success() {
            return Err(format!("protoc command failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        println!("Successfully Generated The Rust Code!");

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = "D:\\RUST\\loadbalancer\\helloworld.proto";
        let output = "D:\\";

        ProtoParser::parse(input,output).unwrap();
    }
}
