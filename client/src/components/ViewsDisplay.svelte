<script lang="ts">
    import MdiEye from '~icons/mdi/eye';

    let {views}: {views: number} = $props();
</script>

<p><MdiEye style="margin-right: 5px"/> {views} views</p>

<style>
    p {
        display: flex;
        align-items: center;
        font-size: 1em;
        padding: 5px 0;
        color: var(--primary-colour);
    }
</style>