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
struct MyAnnotation {
    id: Option<String>,
    label: Option<String>,
    #[serde(rename = "annotationType")]
    annotation_type: MyAnnotationType,
}

// annotate_snippets::Annotation has a lifetime parameter, so the only way to safely convert
// from our mirror struct is to keep a reference to the original struct around.
fn convert_annotation(annotation: &MyAnnotation) -> Annotation {
    Annotation {
        id: annotation.id.as_deref(),
        label: annotation.label.as_deref(),
        annotation_type: annotation.annotation_type.into(),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum MyAnnotationType {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "help")]
    Help,
}

impl From<MyAnnotationType> for AnnotationType {
    fn from(annotation_type: MyAnnotationType) -> Self {
        match annotation_type {
            MyAnnotationType::Error => AnnotationType::Error,
            MyAnnotationType::Warning => AnnotationType::Warning,
            MyAnnotationType::Info => AnnotationType::Info,
            MyAnnotationType::Note => AnnotationType::Note,
            MyAnnotationType::Help => AnnotationType::Help,
        }
    }
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
    let opt: FormatOptions = match serde_wasm_bindgen::from_value::<MyFormatOptions>(options) {
        Ok(config) => config.into(),
        Err(err) => {
            error(err.to_string());
            return String::from("Error");
        }
    };

    let parsed_title = match serde_wasm_bindgen::from_value::<Option<MyAnnotation>>(title) {
        Ok(annotation) => annotation,
        Err(err) => {
            error(err.to_string());
            return String::from("Error");
        }
    };
    let title = parsed_title.as_ref().map(convert_annotation);

    let snippet = Snippet {
        title,
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
        opt,
    };

    let dl = DisplayList::from(snippet);
    dl.to_string()
}
