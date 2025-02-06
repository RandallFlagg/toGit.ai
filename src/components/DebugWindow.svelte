<script>
    import { onMount, onDestroy } from "svelte";

    let isVisible;
    let logs = [];

    const toggleVisibility = () => {
        isVisible = !isVisible;
    };

    const addLog = (message, color = "white") => {
        console.log(message);
        logs.push({ message, color });
    };

    const clearLogs = () => {
        logs = [];
    };

    const show = () => {
        isVisible = true;
    };

    const hide = () => {
        isVisible = false;
    };

    const handleKeydown = (event) => {
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

{#if isVisible}
    <div class="debug-window">
        <button on:click={toggleVisibility}>Close</button>
        <button on:click={clearLogs}>Clear</button>
        {#each logs as { message, color }, index (index)}
            <div style="color: {color}">
                <span>{message}</span>
            </div>
        {/each}
    </div>
{/if}

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
