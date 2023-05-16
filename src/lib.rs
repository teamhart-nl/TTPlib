use std::os::raw::c_char;
use std::ffi::{CString, CStr};
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[no_mangle]
pub extern "C" fn word_to_phonemes(word: *const c_char) -> *const c_char {
    let word = unsafe { CStr::from_ptr(word) };
    let word = word.to_str().unwrap();
    let phonemes;
    match KEYWORDS.get(word) {
        Some(result) => phonemes = result.to_string(),
        None => phonemes = "NONE".to_string(),
    };
    let phonemes = CString::new(phonemes).unwrap();
    phonemes.into_raw()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_exists() { 
        let word = "hello";
        let phonemes = word_to_phonemes(CString::new(word).unwrap().as_ptr());
        println!("{}: {:?}", word, phonemes);
        let phonemes = unsafe { CStr::from_ptr(phonemes) };
        let phonemes = String::from_utf8_lossy(phonemes.to_bytes()).to_string();
        assert_eq!(phonemes, "HH AH L OW");
    }
    #[test]
    fn work_does_not_exist() { 

        let word = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let phonemes = word_to_phonemes(CString::new(word).unwrap().as_ptr());
        println!("{}: {:?}", word, phonemes);
        let phonemes = unsafe { CStr::from_ptr(phonemes) };
        let phonemes = String::from_utf8_lossy(phonemes.to_bytes()).to_string();
        assert_eq!(phonemes, "NONE");
    }
    
}
