<script lang="ts">
    let {name}: {name: string} = $props();
</script>

<a href="/"><span>&lbrace;</span>{name}<span>&rbrace;</span></a>

<style>
    a {
        font-weight: 700;
        font-size: 2em;
        text-decoration: none;
        color: var(--primary-colour);
        transition: 0.3s ease all;
        width: fit-content;
    }

    a:hover {
        color: var(--tertiary-colour);
    }

    span {
        color: var(--secondary-colour);
        margin: 0 5px;
    }
</style>