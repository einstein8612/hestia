<script lang="ts">
    import MdiGithub from '~icons/mdi/github';
    import MdiGraphBox from '~icons/mdi/graph-box';
	import StatsModal from './StatsModal.svelte';

    let statsOpen: boolean = false;

    const github = () => {
        window.open("https://github.com/einstein8612/hestia", "_blank");
    }

    const stats = () => {
        statsOpen = !statsOpen;
    }
</script>

<div class="footer">
    <div class="footer-buttons">
        <button id="github" on:click={github}><MdiGithub font-size="1.75em" /> Github</button>
        <div class="separator"></div>
        <button id="stats" on:click={stats}><MdiGraphBox font-size="1.75em" /> Stats</button>
    </div>
    <p>Powered by Hestia</p>
</div>
<StatsModal bind:open={statsOpen} />

<style>
    .footer {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .footer-buttons {
        display: flex;
        justify-content: center;
        gap: 10px;
    }

    button {
        border: none;
        outline: none;
        display: flex;
        gap: 5px;
        align-items: center;
        color: var(--primary-colour);
        background-color: transparent;
        transition: 0.3s ease all;
        cursor: pointer;
    }

    button:hover {
        color: var(--tertiary-colour);
    }

    .separator {
        border-right: 1px solid var(--primary-colour);
    }

    p {
        font-size: 0.75em;
        text-align: center;
        color: var(--primary-colour);
    }
</style>