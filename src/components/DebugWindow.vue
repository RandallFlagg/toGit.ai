<template>
    <div v-if="isVisible" class="debug-window">
        <button @click="toggleVisibility">Close</button>
        <button @click="clearLogs">Clear</button>
        <div v-for="(message, index) in logs" :key="index" :style="{ color: message.color }">
            <span>{{ message.message }}</span>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue';

const isVisible = ref(false);
const logs = ref([]);

const toggleVisibility = () => {
    isVisible.value = !isVisible.value;
};

const addLog = (message, color = 'white') => {
    logs.value.push({ message, color });
};

const clearLogs = () => {
    logs.value = [];
};

const show = () => {
    isVisible.value = true;
};

const hide = () => {
    isVisible.value = false;
};

const handleKeydown = (event) => {
    if (event.ctrlKey && event.key === 'd') {
        toggleVisibility();
    }
};

onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
});

onBeforeUnmount(() => {
    window.removeEventListener('keydown', handleKeydown);
});

// Expose methods to parent component
defineExpose({
    toggleVisibility,
    addLog,
    clearLogs
});

// Expose methods globally
window.debugLog = addLog;
window.toggleDebugWindow = toggleVisibility;
</script>

<style scoped>
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