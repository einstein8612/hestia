import { HighlightStyle, syntaxHighlighting } from "@codemirror/language"
import { EditorView } from '@codemirror/view';
import { tags as t } from "@lezer/highlight"

const chalky = "#e5c07b",
        coral = "#e06c75",
        cyan = "#56b6c2",
        invalid = "#ffffff",
        ivory = "#abb2bf",
        stone = "#7d8799",
        malibu = "#61afef",
        sage = "#98c379",
        whiskey = "#d19a66",
        violet = "#c678dd";

export const oneDarkTheme = EditorView.theme({
        "&": {
            color: "var(--tertiary-colour)",
            backgroundColor: "var(--tertiary-bg-colour)"
        },
        ".cm-content": {
            minHeight: "400px"
        },
        "&.cm-focused .cm-cursor": {
            borderLeftColor: "var(--secondary-colour)"
        },
        "&.cm-focused .cm-selectionBackground, ::selection": {
            backgroundColor: "rgba(255,255,255,0.25) !important"
        },
        ".cm-activeLineGutter, .cm-activeLine": {
            backgroundColor: "rgba(255,255,255,0.1)"
        },
        ".cm-gutters": {
            backgroundColor: "var(--tertiary-bg-colour)",
            color: "var(--tertiary-colour)",
            border: "none"
        },
        ".cm-lineNumbers .cm-gutterElement": {
            color: "var(--primary-colour)",
            padding: "0 3px 0 25px"
        },
        ".cm-line": {
            padding: "0px"
        },
        ".cm-lineNumbers": {
            minWidth: "30px"
        }
});

export const oneDarkHighlightStyle = syntaxHighlighting(HighlightStyle.define([
    {tag: t.keyword,
    color: violet},
    {tag: [t.name, t.deleted, t.character, t.propertyName, t.macroName],
    color: coral},
    {tag: [t.function(t.variableName), t.labelName],
    color: malibu},
    {tag: [t.color, t.constant(t.name), t.standard(t.name)],
    color: whiskey},
    {tag: [t.definition(t.name), t.separator],
    color: ivory},
    {tag: [t.typeName, t.className, t.number, t.changed, t.annotation, t.modifier, t.self, t.namespace],
    color: chalky},
    {tag: [t.operator, t.operatorKeyword, t.url, t.escape, t.regexp, t.link, t.special(t.string)],
    color: cyan},
    {tag: [t.meta, t.comment],
    color: stone},
    {tag: t.strong,
    fontWeight: "bold"},
    {tag: t.emphasis,
    fontStyle: "italic"},
    {tag: t.strikethrough,
    textDecoration: "line-through"},
    {tag: t.link,
    color: stone,
    textDecoration: "underline"},
    {tag: t.heading,
    fontWeight: "bold",
    color: coral},
    {tag: [t.atom, t.bool, t.special(t.variableName)],
    color: whiskey },
    {tag: [t.processingInstruction, t.string, t.inserted],
    color: sage},
    {tag: t.invalid,
    color: invalid},
]));