"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const pkg_1 = require("../pkg");
console.log((0, pkg_1.annotate_snippet)({
    label: "mismatched types",
    id: "E0308",
    annotationType: "error",
}, [], [{
        source: `) -> Option<String> {
    for ann in annotations {
        match (ann.range.0, ann.range.1) {
            (None, None) => continue,
            (Some(start), Some(end)) if start > end_index => continue,
            (Some(start), Some(end)) if start >= start_index => {
                let label = if let Some(ref label) = ann.label {
                    format!(" {}", label)
                } else {
                    String::from("")
                };

                return Some(format!(
                    "{}{}{}",
                    " ".repeat(start - start_index),
                    "^".repeat(end - start),
                    label
                ));
            }
            _ => continue,
        }
    }`,
        lineStart: 51,
        origin: "src/format.rs",
        fold: false,
        annotations: [
            {
                label: "expected `Option<String>` because of return type",
                annotationType: "warning",
                range: [5, 19],
            },
            {
                label: "expected enum `std::option::Option`",
                annotationType: "error",
                range: [26, 724],
            }
        ]
    }], {
    color: true,
    anonymizedLineNumbers: false,
}));
