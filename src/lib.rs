use annotate_snippets::{
    display_list::{DisplayList, FormatOptions, Margin},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "exports.error = function(s) { throw new Error(s) }")]
extern "C" {
    fn error(s: String);
}

#[derive(Deserialize, Debug)]
struct MyAnnotation {
    id: Option<String>,
    label: Option<String>,
    #[serde(rename = "annotationType")]
    annotation_type: MyAnnotationType,
}

fn convert_annotation(annotation: &MyAnnotation) -> Annotation {
    Annotation {
        id: annotation.id.as_deref(),
        label: annotation.label.as_deref(),
        annotation_type: annotation.annotation_type.into(),
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
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

#[derive(Deserialize, Debug)]
struct MySlice {
    source: String,
    #[serde(rename = "lineStart")]
    line_start: usize,
    origin: Option<String>,
    annotations: Vec<MySourceAnnotation>,
    fold: bool,
}

fn convert_slice(slice: &MySlice) -> Slice {
    Slice {
        source: &slice.source,
        line_start: slice.line_start,
        origin: slice.origin.as_deref(),
        annotations: slice
            .annotations
            .iter()
            .map(convert_source_annotation)
            .collect(),
        fold: slice.fold,
    }
}

#[derive(Deserialize, Debug)]
struct MySourceAnnotation {
    range: (usize, usize),
    label: String,
    #[serde(rename = "annotationType")]
    annotation_type: MyAnnotationType,
}

fn convert_source_annotation(source_annotation: &MySourceAnnotation) -> SourceAnnotation {
    SourceAnnotation {
        range: source_annotation.range,
        label: &source_annotation.label,
        annotation_type: source_annotation.annotation_type.into(),
    }
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

    let parsed_footer = match serde_wasm_bindgen::from_value::<Vec<MyAnnotation>>(footer) {
        Ok(annotation) => annotation,
        Err(err) => {
            error(err.to_string());
            return String::from("Error");
        }
    };
    let footer: Vec<Annotation> = parsed_footer.iter().map(convert_annotation).collect();

    let parsed_slices = match serde_wasm_bindgen::from_value::<Vec<MySlice>>(slices) {
        Ok(slices) => slices,
        Err(err) => {
            error(err.to_string());
            return String::from("Error");
        }
    };
    let slices: Vec<Slice> = parsed_slices.iter().map(convert_slice).collect();

    let snippet = Snippet {
        title,
        footer,
        slices,
        opt,
    };

    let dl = DisplayList::from(snippet);
    dl.to_string()
}
