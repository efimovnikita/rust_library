use std::path::Path;
use uuid::Uuid;

pub fn is_valid_guid(guid: String) -> Result<(), String> {
    // Check if the provided string is a valid GUID using the
    // `uuid` crate and return an error message if it is not.
    if !Uuid::parse_str(&guid).is_ok() {
        return Err(format!("'{}' is not a valid GUID", guid));
    }
    Ok(())
}

pub fn is_valid_path(path: String) -> Result<(), String> {
    // Check if the provided path exists and return an error
    // message if it does not.
    let path = Path::new(&path);
    if !path.exists() {
        return Err(format!("'{}' does not exist", path.display()));
    }

    if path.is_dir() == false {
        return Err(format!("'{}' should be directory", path.display()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;

    #[test]
    fn test_is_valid_path() {
        // Test that a valid directory path returns Ok
        let temp_dir = tempdir::TempDir::new("test_dir").unwrap();
        let path = temp_dir.path().to_str().unwrap().to_string();
        let result = is_valid_path(path);
        assert!(result.is_ok());

        // Test that an invalid path returns an error
        let path = String::from("/some/invalid/path");
        let result = is_valid_path(path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "'/some/invalid/path' does not exist");

        // Test that a file path returns an error
        let temp_dir = tempdir::TempDir::new("test_file").unwrap();
        let path = temp_dir.path().join("test_file.txt").to_str().unwrap().to_string();
        let _file = File::create(path.clone()).unwrap();
        let result = is_valid_path(path.clone());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), format!("'{}' should be directory", path));
    }
}