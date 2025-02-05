<script>
    import { onMount, onDestroy } from "svelte";
    import { writable } from "svelte/store";

    let isVisible = writable(false);
    let logs = writable([]);

    const toggleVisibility = () => {
        isVisible.update(value => !value);
    };

    const addLog = (message, color = "white") => {
        logs.update(currentLogs => [...currentLogs, { message, color }]);
    };

    const clearLogs = () => {
        logs.set([]);
    };

    const show = () => {
        isVisible.set(true);
    };

    const hide = () => {
        isVisible.set(false);
    };

    const handleKeydown = event => {
        if (event.ctrlKey && event.key === "d") {
            toggleVisibility();
        }
    };

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
    });

    // Expose methods globally
    window.debugLog = addLog;
    window.toggleDebugWindow = toggleVisibility;
</script>

<style>
    .debug-window-check {
        position: fixed;
        bottom: 10px;
        right: 10px;
        background-color: rgba(0, 0, 0, 0.8);
        color: white;
        padding: 10px;
        border-radius: 5px;
        z-index: 1000;
    }

    .debug-window {
        position: fixed;
        bottom: 0;
        left: 0;
        width: 100%;
        max-height: 200px;
        overflow-y: auto;
        background: rgba(0, 0, 0, 0.8);
        color: white;
        padding: 10px;
        z-index: 1000;
    }
</style>

{#if $isVisible}
    <div class="debug-window">
        <button on:click={toggleVisibility}>Close</button>
        <button on:click={clearLogs}>Clear</button>
        {#each $logs as { message, color }, index (index)}
            <div style="color: {color}">
                <span>{message}</span>
            </div>
        {/each}
    </div>
{/if}
