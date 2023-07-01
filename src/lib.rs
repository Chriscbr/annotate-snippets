use annotate_snippets::{
    display_list::{DisplayList, FormatOptions, Margin},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "exports.error = function(s) { throw new Error(s) }")]
extern "C" {
    fn error(s: String);
}

#[derive(Serialize, Deserialize, Debug)]
struct MyFormatOptions {
    color: bool,
    #[serde(rename = "anonymizedLineNumbers")]
    anonymized_line_numbers: bool,
    margin: Option<MyMargin>,
}

impl From<MyFormatOptions> for FormatOptions {
    fn from(options: MyFormatOptions) -> Self {
        FormatOptions {
            color: options.color,
            anonymized_line_numbers: options.anonymized_line_numbers,
            margin: options.margin.map(|m| m.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MyMargin {
    #[serde(rename = "whitespaceLeft")]
    whitespace_left: usize,
    #[serde(rename = "spanLeft")]
    span_left: usize,
    #[serde(rename = "spanRight")]
    span_right: usize,
    #[serde(rename = "labelRight")]
    label_right: usize,
    #[serde(rename = "columnWidth")]
    column_width: usize,
    #[serde(rename = "maxLineLen")]
    max_line_len: usize,
}

impl From<MyMargin> for Margin {
    fn from(margin: MyMargin) -> Self {
        Margin::new(
            margin.whitespace_left,
            margin.span_left,
            margin.span_right,
            margin.label_right,
            margin.column_width,
            margin.max_line_len,
        )
    }
}

#[wasm_bindgen]
pub fn annotate_snippet(
    title: JsValue,
    footer: JsValue,
    slices: JsValue,
    options: JsValue,
) -> String {
    let options: FormatOptions = match serde_wasm_bindgen::from_value::<MyFormatOptions>(options) {
        Ok(config) => config.into(),
        Err(err) => {
            error(err.to_string());
            return String::from("Error");
        }
    };

    let snippet = Snippet {
        title: Some(Annotation {
            label: Some("mismatched types"),
            id: Some("E0308"),
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![Annotation {
            label: Some(
                "expected type: `snippet::Annotation`\n   found type: `__&__snippet::Annotation`",
            ),
            id: None,
            annotation_type: AnnotationType::Note,
        }],
        slices: vec![Slice {
            source: "        slices: vec![\"A\",",
            line_start: 13,
            origin: Some("src/multislice.rs"),
            fold: false,
            annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference",
                range: (21, 24),
                annotation_type: AnnotationType::Error,
            }],
        }],
        opt: options,
    };

    let dl = DisplayList::from(snippet);
    dl.to_string()
}
