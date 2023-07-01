import * as bindings from "../pkg";

export type AnnotationType = "error" | "warning" | "info" | "note" | "help";

export interface Annotation {
  id?: string;
  label?: string;
  annotationType: AnnotationType;
}

export interface Slice {
  source: string;
  lineStart: number;
  origin?: string;
  annotations: SourceAnnotation[];
  fold: boolean;
}

export interface SourceAnnotation {
  range: [number, number];
  label: string;
  annotationType: AnnotationType;
}

export interface FormatOptions {
  color: boolean;
  anonymizedLineNumbers: boolean;
  margin?: Margin;
}

export interface Margin {
  whitespaceLeft: number;
  spanLeft: number;
  spanRight: number;
  labelRight: number;
  columnWidth: number;
  maxLineLen: number;
}

export function annotateSnippet(
  title: Annotation | undefined,
  footer: Annotation[],
  slices: Slice[],
  options: FormatOptions,
) {
  return bindings.annotate_snippet(title, footer, slices, options);
}
