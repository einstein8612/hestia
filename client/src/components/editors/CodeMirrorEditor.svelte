<script lang="ts">
	import { basicSetup } from 'codemirror';
    import { indentWithTab } from '@codemirror/commands';
    import { languages } from "@codemirror/language-data";
	import { type EditorViewConfig, EditorView, keymap } from '@codemirror/view';
    import { type SelectionRange, Compartment } from '@codemirror/state';
    
	import { oneDarkHighlightStyle, oneDarkTheme } from './codemirrortheme';

    let { language, value, onChange, selection }: {language: string, value: string, onChange?: (newContent: string, range: SelectionRange) => void, selection?: SelectionRange} = $props();
	let editorEl: Element | undefined = $state();

    const langHolder = new Compartment()
    let view: EditorView | undefined;

    $effect(() => {
        if (!editorEl) return;

        const viewConfig: EditorViewConfig = {
            doc: value,
            extensions: [basicSetup, oneDarkTheme, oneDarkHighlightStyle, keymap.of([indentWithTab]), langHolder.of([])],
            parent: editorEl,
        }
        if (onChange) {
            viewConfig.dispatchTransactions = (trs, view) => {
                view.update(trs);
                if (trs.some(tr => tr.docChanged)) {
                    onChange?.(view.state.doc.toString(), view.state.selection.main);
                }
            }
        }

        view?.destroy();
        view = new EditorView(viewConfig);
        if (selection) {
            view.focus();
            view.dispatch({
                selection: {
                    anchor: selection.anchor,
                    head: selection.head
                }
            });
        }
	});

    $effect(() => {
        let lang = languages.filter(l => l.name === language)[0];
        if (!lang) {
            view?.dispatch({
                effects: langHolder.reconfigure([]),
            })
            return;
        }
        lang.load().then(langSupport => {
            view?.dispatch({
                effects: langHolder.reconfigure(langSupport.language),
            });
        });
    });
</script>

<div id="editor" bind:this={editorEl}></div>