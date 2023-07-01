# annotate-snippets

An unnofficial set of Node.js (TypeScript) bindings for [annotate-snippets-rs](https://crates.io/crates/annotate-snippets) - a Rust library for annotation of programming code slices.

## Usage

```typescript
import { annotateSnippet } from "annotate-snippets";

console.log(annotateSnippet({
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
```

Output (with color):

```
error[E0308]: mismatched types
  --> src/format.rs:51:6
   |
51 |   ) -> Option<String> {
   |        -------------- expected `Option<String>` because of return type
52 |       for ann in annotations {
   |  _____^
53 | |         match (ann.range.0, ann.range.1) {
54 | |             (None, None) => continue,
55 | |             (Some(start), Some(end)) if start > end_index => continue,
56 | |             (Some(start), Some(end)) if start >= start_index => {
57 | |                 let label = if let Some(ref label) = ann.label {
58 | |                     format!(" {}", label)
59 | |                 } else {
60 | |                     String::from("")
61 | |                 };
62 | |
63 | |                 return Some(format!(
64 | |                     "{}{}{}",
65 | |                     " ".repeat(start - start_index),
66 | |                     "^".repeat(end - start),
67 | |                     label
68 | |                 ));
69 | |             }
70 | |             _ => continue,
71 | |         }
72 | |     }
   | |____^ expected enum `std::option::Option`
   |
```

## Contributing

Pull requests are welcome.
