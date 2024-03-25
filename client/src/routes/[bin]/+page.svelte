<script lang="ts">
    import { goto } from '$app/navigation';
    import type {Bin} from "$lib/api"

    import { type SelectionRange } from '@codemirror/state';

    import FileHeader from "../../components/FileHeader.svelte";
	import SaveButton from "../../components/SaveButton.svelte";
	import FileEditor from '../../components/FileEditor.svelte';

    const {data}: {data: Bin} = $props();
    const contentAsString = new TextDecoder().decode(new Uint8Array(data.content).buffer);

    const changedContent = (newContent: string, selection: SelectionRange) => {
        goto("/", {state: {binContent: newContent, selection}})
    }
</script>

<svelte:head>
    <title>Hestia | {data.name ? data.name : "Untitled"}</title> 

    <meta property="og:title" content="{data.name}" />
    <meta property="og:description" content={data.description} />
    <meta property="og:type" content="website" />
</svelte:head>
<main>
    <FileHeader name={data.name} description={data.description} date={new Date(data.created_at)} views={data.views} />
    <FileEditor filename={data.filename} language={data.language} value={contentAsString} onChange={changedContent} />
    <SaveButton disabled/>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        row-gap: 1em;
        background-color: var(--primary-bg-colour);
        width: 100%;
    }
</style>