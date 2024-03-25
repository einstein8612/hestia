<script lang="ts">
    let {value, placeholder, fontSize, fontWeight, colour}: {value: string, placeholder: string, fontSize?: string, fontWeight?: string, colour?: string} = $props();
</script>

<input type="text" bind:value placeholder="{placeholder}" style="font-size: {fontSize}; font-weight: {fontWeight}; color: {colour};" />

<style>
    input {
        border: none;
        outline: none;
        font-size: 1.5em;
        font-weight: 600;
        padding: 5px 10px;
        color: var(--tertiary-colour);
        background-color: transparent;
    }

    input::placeholder {
        font-style: italic;
    }
</style>