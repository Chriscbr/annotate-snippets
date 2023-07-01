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
export declare function annotateSnippet(title: Annotation | undefined, footer: Annotation[], slices: Slice[], options: FormatOptions): string;
