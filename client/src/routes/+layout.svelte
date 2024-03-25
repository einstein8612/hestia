<script>
    import Logo from "../components/Logo.svelte";
	import Footer from "../components/Footer.svelte";
</script>


<div>
    <Logo name="Hestia" />
    <slot />
    <Footer />
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }
</style>