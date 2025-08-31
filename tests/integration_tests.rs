use pretty_simple_display::{DebugPretty, DebugSimple, DisplayPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DebugPretty)]
struct TestStructDebugPretty {
    id: u64,
    name: String,
    active: bool,
}

#[derive(Serialize, Deserialize, DisplayPretty)]
struct TestStructDisplayPretty {
    id: u64,
    name: String,
    active: bool,
}

#[derive(Serialize, Deserialize, DebugSimple)]
struct TestStructDebugSimple {
    id: u64,
    name: String,
    active: bool,
}

#[derive(Serialize, Deserialize, DisplaySimple)]
struct TestStructDisplaySimple {
    id: u64,
    name: String,
    active: bool,
}

#[derive(Serialize, Deserialize, DebugPretty, DisplayPretty)]
enum TestEnum {
    Variant1,
    Variant2 { value: String },
}

#[derive(Serialize, Deserialize, DebugSimple, DisplaySimple)]
struct NestedStruct {
    inner: InnerStruct,
    values: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
struct InnerStruct {
    data: String,
}

#[test]
fn test_debug_pretty_basic_struct() {
    let test_struct = TestStructDebugPretty {
        id: 42,
        name: "test".to_string(),
        active: true,
    };

    let debug_output = format!("{:?}", test_struct);

    // Should contain pretty-printed JSON
    assert!(debug_output.contains("{\n"));
    assert!(debug_output.contains("  \"id\": 42"));
    assert!(debug_output.contains("  \"name\": \"test\""));
    assert!(debug_output.contains("  \"active\": true"));
}

#[test]
fn test_display_pretty_basic_struct() {
    let test_struct = TestStructDisplayPretty {
        id: 42,
        name: "test".to_string(),
        active: true,
    };

    let display_output = format!("{}", test_struct);

    // Should contain pretty-printed JSON
    assert!(display_output.contains("{\n"));
    assert!(display_output.contains("  \"id\": 42"));
    assert!(display_output.contains("  \"name\": \"test\""));
    assert!(display_output.contains("  \"active\": true"));
}

#[test]
fn test_debug_simple_basic_struct() {
    let test_struct = TestStructDebugSimple {
        id: 42,
        name: "test".to_string(),
        active: true,
    };

    let debug_output = format!("{:?}", test_struct);

    // Should contain compact JSON (no newlines or extra spaces)
    assert!(!debug_output.contains("\n"));
    assert!(debug_output.contains("{\"id\":42"));
    assert!(debug_output.contains("\"name\":\"test\""));
    assert!(debug_output.contains("\"active\":true}"));
}

#[test]
fn test_display_simple_basic_struct() {
    let test_struct = TestStructDisplaySimple {
        id: 42,
        name: "test".to_string(),
        active: true,
    };

    let display_output = format!("{}", test_struct);

    // Should contain compact JSON (no newlines or extra spaces)
    assert!(!display_output.contains("\n"));
    assert!(display_output.contains("{\"id\":42"));
    assert!(display_output.contains("\"name\":\"test\""));
    assert!(display_output.contains("\"active\":true}"));
}

#[test]
fn test_enum_debug_pretty() {
    let enum_variant1 = TestEnum::Variant1;
    let enum_variant2 = TestEnum::Variant2 {
        value: "hello".to_string(),
    };

    let output1 = format!("{:?}", enum_variant1);
    let output2 = format!("{:?}", enum_variant2);

    // Simple variant should be just the string
    assert_eq!(output1, "\"Variant1\"");

    // Struct variant should be pretty-printed
    assert!(output2.contains("{\n"));
    assert!(output2.contains("\"Variant2\""));
}

#[test]
fn test_enum_display_pretty() {
    let enum_variant2 = TestEnum::Variant2 {
        value: "hello".to_string(),
    };

    let output = format!("{}", enum_variant2);

    // Should be pretty-printed JSON
    assert!(output.contains("{\n"));
    assert!(output.contains("\"Variant2\""));
    assert!(output.contains("\"value\": \"hello\""));
}

#[test]
fn test_nested_struct_debug_simple() {
    let nested = NestedStruct {
        inner: InnerStruct {
            data: "nested_data".to_string(),
        },
        values: vec![1, 2, 3],
    };

    let debug_output = format!("{:?}", nested);

    // Should be compact JSON
    assert!(!debug_output.contains("\n"));
    assert!(debug_output.contains("\"inner\":{\"data\":\"nested_data\"}"));
    assert!(debug_output.contains("\"values\":[1,2,3]"));
}

#[test]
fn test_nested_struct_display_simple() {
    let nested = NestedStruct {
        inner: InnerStruct {
            data: "nested_data".to_string(),
        },
        values: vec![1, 2, 3],
    };

    let display_output = format!("{}", nested);

    // Should be compact JSON
    assert!(!display_output.contains("\n"));
    assert!(display_output.contains("\"inner\":{\"data\":\"nested_data\"}"));
    assert!(display_output.contains("\"values\":[1,2,3]"));
}

#[test]
fn test_multiple_derives_same_struct() {
    #[derive(Serialize, DebugPretty, DisplaySimple)]
    struct MultiDeriveStruct {
        id: u32,
        name: String,
    }

    let test = MultiDeriveStruct {
        id: 123,
        name: "test".to_string(),
    };

    let debug_output = format!("{:?}", test);
    let display_output = format!("{}", test);

    // Debug should be pretty
    assert!(debug_output.contains("{\n"));
    assert!(debug_output.contains("  \"id\": 123"));

    // Display should be simple
    assert!(!display_output.contains("\n"));
    assert!(display_output.contains("{\"id\":123"));
}

#[test]
fn test_special_characters() {
    #[derive(Serialize, DebugSimple)]
    struct SpecialCharsStruct {
        text: String,
    }

    let special = SpecialCharsStruct {
        text: "Hello \"world\" with\nnewlines and\ttabs".to_string(),
    };

    let debug_output = format!("{:?}", special);

    // Should properly escape special characters
    assert!(debug_output.contains("\\\""));
    assert!(debug_output.contains("\\n"));
    assert!(debug_output.contains("\\t"));
}

#[test]
fn test_option_types() {
    #[derive(Serialize, DebugPretty)]
    struct OptionStruct {
        some_value: Option<String>,
        none_value: Option<i32>,
    }

    let with_options = OptionStruct {
        some_value: Some("present".to_string()),
        none_value: None,
    };

    let debug_output = format!("{:?}", with_options);

    assert!(debug_output.contains("\"some_value\": \"present\""));
    assert!(debug_output.contains("\"none_value\": null"));
}
