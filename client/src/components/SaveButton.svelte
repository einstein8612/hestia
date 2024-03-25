<script lang="ts">
    const {onClick, disabled}: {onClick?: () => void, disabled?: boolean} = $props();
</script>

<button id="save-button" on:click={onClick} disabled={disabled}>Save</button>

<style>
    #save-button {
        border: none;
        border-radius: 5px;
        outline: none;
        color: var(--primary-colour);
        font-size: 1.5em;
        padding: 5px 15px;
        align-self: flex-end;
        background-color: var(--secondary-bg-colour);
        cursor: pointer;
        transition: 0.3s ease all;
    }

    #save-button:hover {
        color: var(--tertiary-colour);
    }

    #save-button:disabled {
        cursor: not-allowed;
    }
</style>