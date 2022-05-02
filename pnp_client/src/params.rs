use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

lazy_static! {
    pub static ref PROGRAM_PARAMETERS: Vec<KeyValuePair> = {
        #[cfg(target_arch = "wasm32")]
        {
            use sapp_jsutils::JsObject;

            extern "C" {
                fn miniquad_parameters_param_count() -> i32;
                fn miniquad_parameters_get_key(pos: i32) -> JsObject;
                fn miniquad_parameters_get_value(pos: i32) -> JsObject;
            }

            let count = unsafe { miniquad_parameters_param_count() };
            let mut result: Vec<KeyValuePair> = Vec::new();
            for i in 0..count {
                let mut key = String::new();
                unsafe {
                    miniquad_parameters_get_key(i).to_string(&mut key);
                }

                let mut value = String::new();
                unsafe {
                    miniquad_parameters_get_value(i).to_string(&mut value);
                }
                result.push(KeyValuePair { key, value });
            }
            result
        }

        #[cfg(not(target_arch = "wasm32"))]
        { // it's not planned, but who knows...
            use std::env;
            let mut result: Vec<KeyValuePair> = Vec::new();
            for kvp in env::args() {
                let kvp = kvp.replace("-", "");
                let mut i = 0;
                let mut key = "".to_string();
                let mut value = "".to_string();
                for spl in kvp.split("=") {
                    if i == 0 {
                        key = spl.to_string();
                    } else if i == 1 {
                        value = spl.to_string();
                    } else {
                        value = value + &("=".to_string() + spl);
                    }
                    i += 1;
                }
                result.push(KeyValuePair { key, value });
            }
            result
        }
    };
}
