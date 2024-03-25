<script lang="ts">
    import { type SelectionRange } from '@codemirror/state';

    import CodeMirrorEditor from "./editors/CodeMirrorEditor.svelte";
	import FileEditorHeader from './FileEditorHeader.svelte';

    let { language, value, filename, onChange, selection }: {language: string, value: string, filename: string, onChange?: (newContent: string, range: SelectionRange) => void, selection?: SelectionRange} = $props();

    let localValue = value;
    const copy = () => {
        navigator.clipboard.writeText(localValue);
    }

    const download = () => {
        const blob = new Blob([localValue], {type: "text/plain"});
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = filename;
        a.click();
        document.removeChild(a);
    }

    const raw = () => {
        const blob = new Blob([localValue], {type: "text/plain"});
        const fileURL = window.URL.createObjectURL(blob);
        window.open()!.location.href = fileURL;
    }

    const localOnChange = (value: string, selection: SelectionRange) => {
        localValue = value;
        onChange?.(value, selection);
    }
</script>

<section>
    <FileEditorHeader bind:language bind:filename onCopy={copy} onDownload={download} onRaw={raw} />
    <CodeMirrorEditor language={language} value={value} selection={selection} onChange={localOnChange} />
</section>

<style>
    textarea {
        min-height: 100px;
        width: 100%;
        outline: none;
        border: none;
        padding: 10px;
        color: var(--tertiary-colour);
        background-color: var(--secondary-bg-colour);
    }
</style>