<script lang="ts">
	import { formatTimeSinceDate } from '$lib/format';
	import MdiCalendar from '~icons/mdi/calendar';

	let { date }: { date: Date } = $props();
</script>

<div class="tooltip">
	<p>
		<MdiCalendar style="margin-right: 5px" />{formatTimeSinceDate(date)}
	</p>
	<span class="tooltiptext">{date.toLocaleString()}</span>
</div>

<style>
	p {
		display: flex;
		align-items: center;
		font-size: 1em;
		padding: 5px 0;
		color: var(--primary-colour);
	}

	.tooltip {
		position: relative;
		display: inline-block;
	}

	.tooltip .tooltiptext {
        position: absolute;
		z-index: 1;
		visibility: hidden;
		width: 120px;
		background-color: var(--primary-bg-colour);
		color: #fff;
		text-align: center;
		border-radius: 6px;
		padding: 5px 0;
		bottom: 100%;
		left: 50%;
		margin-left: -60px;
        opacity: 0;
        transition: opacity 0.3s;
        font-size: 0.75em;
	}

	.tooltip .tooltiptext::after {
		content: '';
		position: absolute;
		top: 100%;
		left: 50%;
		margin-left: -5px;
		border-width: 5px;
		border-style: solid;
		border-color: var(--primary-bg-colour) transparent transparent transparent;
	}

	.tooltip:hover .tooltiptext {
        opacity: 1;
		visibility: visible;
	}
</style>
