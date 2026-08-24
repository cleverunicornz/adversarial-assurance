//! Syntax-token enforcement for the constrained YAML 1.2 profile.
//!
//! Provenance: token-level rejection follows bedrock's public YAML scanner
//! approach because deserialization has already erased anchor and alias syntax.

#[allow(clippy::unsafe_removed_from_name)]
use unsafe_libyaml_norway as yaml;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Forbidden {
    Anchor { line: u32, name: String },
    Alias { line: u32, name: String },
    MergeKey { line: u32 },
    Tag { line: u32 },
    Scanner { line: u32 },
}

pub fn scan(input: &[u8]) -> Vec<Forbidden> {
    let storage = Box::new(std::mem::MaybeUninit::<yaml::yaml_parser_t>::uninit());
    let parser = storage.as_ptr().cast_mut();

    // SAFETY: `storage` pins parser memory for the complete scan, `input`
    // outlives the parser, and every initialized token is deleted once.
    unsafe {
        if yaml::yaml_parser_initialize(parser).fail {
            return vec![Forbidden::Scanner { line: 1 }];
        }
        yaml::yaml_parser_set_encoding(parser, yaml::YAML_UTF8_ENCODING);
        yaml::yaml_parser_set_input_string(parser, input.as_ptr(), input.len() as u64);
    }

    let mut violations = Vec::new();
    let mut mapping_key = false;
    loop {
        let mut slot = std::mem::MaybeUninit::<yaml::yaml_token_t>::uninit();
        // SAFETY: parser initialization succeeded and slot is writable.
        let scanned = unsafe { yaml::yaml_parser_scan(parser, slot.as_mut_ptr()) };
        if scanned.fail {
            // SAFETY: the initialized parser owns a valid problem mark.
            let line = unsafe { (&*parser).problem_mark.line as u32 + 1 };
            violations.push(Forbidden::Scanner { line });
            break;
        }
        // SAFETY: a successful scan initialized `slot`.
        let mut token = unsafe { slot.assume_init() };
        let line = token.start_mark.line as u32 + 1;
        let stream_end = token.type_ == yaml::YAML_STREAM_END_TOKEN;

        match token.type_ {
            yaml::YAML_KEY_TOKEN => mapping_key = true,
            yaml::YAML_VALUE_TOKEN
            | yaml::YAML_BLOCK_ENTRY_TOKEN
            | yaml::YAML_FLOW_ENTRY_TOKEN
            | yaml::YAML_BLOCK_END_TOKEN
            | yaml::YAML_FLOW_SEQUENCE_END_TOKEN
            | yaml::YAML_FLOW_MAPPING_END_TOKEN => mapping_key = false,
            yaml::YAML_ANCHOR_TOKEN => {
                // SAFETY: this token variant owns a NUL-terminated name.
                let name = unsafe { c_string(token.data.anchor.value) };
                violations.push(Forbidden::Anchor { line, name });
            }
            yaml::YAML_ALIAS_TOKEN => {
                // SAFETY: this token variant owns a NUL-terminated name.
                let name = unsafe { c_string(token.data.alias.value) };
                violations.push(Forbidden::Alias { line, name });
            }
            yaml::YAML_TAG_TOKEN => violations.push(Forbidden::Tag { line }),
            yaml::YAML_SCALAR_TOKEN => {
                // SAFETY: this token variant owns `length` scalar bytes.
                let value =
                    unsafe { bytes(token.data.scalar.value, token.data.scalar.length as usize) };
                if mapping_key && value == "<<" {
                    violations.push(Forbidden::MergeKey { line });
                }
                mapping_key = false;
            }
            _ => {}
        }

        // SAFETY: token was initialized by libyaml and is deleted exactly once.
        unsafe { yaml::yaml_token_delete(&mut token) };
        if stream_end {
            break;
        }
    }

    // SAFETY: no token or parser access follows deletion.
    unsafe { yaml::yaml_parser_delete(parser) };
    violations
}

unsafe fn c_string(pointer: *mut u8) -> String {
    if pointer.is_null() {
        return String::new();
    }
    let mut length = 0;
    // SAFETY: caller supplies a NUL-terminated token-owned buffer.
    while unsafe { *pointer.add(length) } != 0 {
        length += 1;
    }
    // SAFETY: the buffer has at least `length` readable bytes.
    let value = unsafe { std::slice::from_raw_parts(pointer, length) };
    String::from_utf8_lossy(value).into_owned()
}

unsafe fn bytes(pointer: *mut u8, length: usize) -> String {
    if pointer.is_null() {
        return String::new();
    }
    // SAFETY: caller supplies a token-owned buffer and its exact byte length.
    let value = unsafe { std::slice::from_raw_parts(pointer, length) };
    String::from_utf8_lossy(value).into_owned()
}
