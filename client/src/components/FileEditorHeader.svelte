<script lang="ts">
	import InputField from "./InputField.svelte";
	import LanguageSelector from "./LanguageSelector.svelte";

    let { language, filename, onCopy, onDownload, onRaw }: {language: string, filename: string, onCopy: () => void, onDownload: () => void, onRaw: () => void} = $props();
</script>

<div class="file-editor-header">
    <div class="left">
        <LanguageSelector bind:language />
        <InputField placeholder="Filename" bind:value={filename} fontWeight="500" fontSize="1.2em" colour="var(--primary-colour)" />
    </div>
    <div class="right">
        <button on:click={onCopy}>Copy</button>
        <button on:click={onDownload}>Download</button>
        <button on:click={onRaw}>Raw</button>
    </div>
</div>

<style>
    .file-editor-header {
        display: flex;
        justify-content: space-between;
        background-color: var(--secondary-bg-colour);
        flex-wrap: wrap;
    }

    .left,.right {
        display: flex;
        flex-wrap: wrap;
    }


    button {
        cursor: pointer;
        border: none;
        outline: none;
        padding: 5px 10px;
        height: 100%;
        color: var(--primary-colour);
        background-color: var(--secondary-bg-colour);
        transition: 0.3s ease all;
    }

    button:hover {
        background-color: rgba(255,255,255,0.1);
    }
</style>