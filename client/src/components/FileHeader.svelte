<script lang="ts">
	import DateDisplay from "./DateDisplay.svelte";
	import InputField from "./InputField.svelte";
	import ViewsDisplay from "./ViewsDisplay.svelte";

    let {name, description, date, views}: {name: string, description: string, date: Date, views: number} = $props();
</script>

<div class="file-header">
    <InputField placeholder="Name" fontSize="1.5em" fontWeight="700" bind:value={name} />
    <InputField placeholder="Description" fontSize="1em" fontWeight="600" colour="var(--primary-colour)" bind:value={description} />

    <div class="display-wrapper">
        <DateDisplay date={date} />
        <ViewsDisplay views={views} />
    </div>
</div>

<style>
    .file-header {
        display: flex;
        flex-direction: column;
        padding: 10px;
        background-color: var(--secondary-bg-colour);
    }

    .display-wrapper {
        display: flex;
        flex-wrap: wrap;
        gap: 20px;
        padding: 0px 10px;
    }
</style>