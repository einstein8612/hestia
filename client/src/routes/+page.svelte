<svelte:options runes={true} />

<script lang="ts">
    import { page } from "$app/stores";

    import Logo from "../components/Logo.svelte";
    import FileHeader from "../components/FileHeader.svelte";
	import SaveButton from "../components/SaveButton.svelte";
	import { createBin } from "$lib/api";
	import { goto } from "$app/navigation";
	import FileEditor from "../components/FileEditor.svelte";

    const pageState: any = $page.state;
    const binContent: string = pageState.binContent ? pageState.binContent : "";

    let name: string = $state("");
    let description: string = $state("");
    let filename: string = $state("");
    let newBinContent: string = binContent;
    let language: string = $state("None");

    const save = () => {
        if (!newBinContent) {
            alert("You can't save an empty bin");
            return;
        }
        createBin(name, description, filename, newBinContent, language).then((bin) => {
            goto(`/${bin.key}`)
        });
    }
</script>

<svelte:head>
    <title>Hestia</title> 
</svelte:head>
<main>
    <FileHeader bind:name bind:description date={new Date()} views={0} />
    <FileEditor bind:filename bind:language value={binContent} selection={pageState.selection} onChange={(newContent) => newBinContent = newContent} />
    <SaveButton onClick={save} />
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