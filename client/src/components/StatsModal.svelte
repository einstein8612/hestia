<script lang="ts">
    import MdiCounter from '~icons/mdi/counter';
    import MdiPaperText from '~icons/mdi/paper-text';
    import MdiDatabase from '~icons/mdi/database';
    import MdiEye from '~icons/mdi/eye';
	import { getStatistics, type Statistics } from '$lib/api';
	import { formatLargeNumber, formatStorageNumber } from '$lib/format';

    let {open}: {open: boolean} = $props();

    let dialogEl: HTMLDialogElement | undefined = $state();
    let stats: Statistics | undefined = $state();

    const close = () => {
        open = false;
    }

    const updateStatus = () => {
        getStatistics().then((data) => {
            stats = data;
        });
    }

    $effect(() => {
        if (open) {
            updateStatus()
            dialogEl?.showModal()
        } else {
            dialogEl?.close()
        }
    });
</script>

<dialog bind:this={dialogEl} on:close={close} >
    <h2>Stats</h2>
    <div class="stats-inner">
        <div class="stats-row">
            <div class="statistic">
                <h3>
                    <MdiCounter font-size="1.25em" /> 
                    Total Bins
                </h3>
                <p>{formatLargeNumber(stats?.total_bins)}</p>
            </div>
            <div class="statistic">
                <h3>
                    <MdiPaperText font-size="1.25em" /> 
                    Total Lines
                </h3>
                <p>{formatLargeNumber(stats?.total_lines)}</p>
            </div>
        </div>
        <div class="stats-row">
            <div class="statistic">
                <h3>
                    <MdiDatabase font-size="1.25em" /> 
                    Total Size
                </h3>
                <p>{formatStorageNumber(stats?.total_size_mb)}</p>
            </div>
            <div class="statistic">
                <h3>
                    <MdiEye font-size="1.25em" /> 
                    Views
                </h3>
                <p>{formatLargeNumber(stats?.total_views)}</p>
            </div>
        </div>
    </div>
</dialog>

<style>
    dialog {
        border: none;
        margin: auto;
        text-align: center;
        max-height: 100vh;
        width: 100%;
        max-width: 400px;
        padding: 30px;
        color: var(--primary-colour);
        background-color: var(--primary-bg-colour);
    }

    ::backdrop {
        backdrop-filter: blur(8px);
    }

    .stats-inner {
        display: flex;
        flex-direction: column;
        gap: 25px;
        margin-top: 1.5em;
    }

    .stats-row {
        display: flex;
        justify-content: space-between;
    }

    .statistic {
        display: flex;
        flex-direction: column;
        width: 150px;
    }

    .statistic h3 {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 5px;
    }

    .statistic p {
        color: var(--secondary-colour);
    }
</style>