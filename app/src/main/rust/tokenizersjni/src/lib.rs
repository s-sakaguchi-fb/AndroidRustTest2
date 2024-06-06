#[macro_use]
extern crate log;
extern crate tokenizers;
extern crate jni;
extern crate android_logger;
use log::LevelFilter;
use android_logger::Config;
    
use self::jni::JNIEnv;
use self::jni::objects::{JClass, JString};
use self::jni::sys::{jstring, jint};
use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_char};
use tokenizers::models::wordpiece::WordPiece;
use tokenizers::tokenizer::{AddedToken, Tokenizer};

// Android JNI
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_freebit_rusttest_Tokenizers_serialization(env: JNIEnv, _: JClass, j_filename: JString) {
        android_logger::init_once(
            Config::default().with_max_level(LevelFilter::Trace),
        );
        debug!("Java_com_freebit_rusttest_Tokenizers_serialization called");
        let filename: String = env.get_string(j_filename).unwrap().into();
        let start = std::time::Instant::now();
        let mut tokenizer = Tokenizer::new(WordPiece::default());
        debug!("Export file path: {}", filename);
    
        // Mix special and not special
        // You can make sure ids are in order, and special status is correct.
        let tokens: Vec<_> = (0..12_000)  // 元は120000だが処理が長すぎるので変更
            .map(|i| AddedToken::from(format!("[SPECIAL_{}]", i), i % 2 == 0))
            .collect();
        debug!("Save start");
        tokenizer.add_tokens(&tokens);
        debug!("Token took {:?}", start.elapsed());
        let result = tokenizer.save(filename.clone(), true);
        debug!("Save took {:?}", start.elapsed());
        let _ = result.map_err(|e| debug!("Save error: {}", e));
        let start = std::time::Instant::now();
        let _tok = Tokenizer::from_file(filename).unwrap();
        debug!("Took {:?}", start.elapsed());
        //std::fs::remove_file("_tok.json").unwrap();
    }
}
