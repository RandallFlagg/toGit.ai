<template>
  <div class="container">
    <button @click="openFolderExplorer" class="explorer-button" :type="type">
      {{ displayText }}
    </button>
  </div>
</template>

<script setup>
import { open, save } from '@tauri-apps/plugin-dialog';
import { defineProps } from 'vue';

const props = defineProps({
  displayText: {
    type: String,
    default: 'Open Folder Explorer'
  },
  type: {
    type: String,
    default: 'button' // Default to 'button' if not specified
  }
});

const openFolderExplorer = async () => {
  try {
    const selectedFolder = await open({
      directory: true,
      multiple: false,
      title: 'Select a Folder',
    });

    if (selectedFolder) {
      console.log('Selected folder:', selectedFolder);
      // You can handle the selected folder path here
    }
  } catch (error) {
    console.error('Error opening folder explorer:', error);
  }
};
</script>

<style scoped>
.container {
  margin: 20px;
}

.explorer-button {
  background-color: #007bff;
  color: white;
  padding: 10px;
  border: none;
  cursor: pointer;
  border-radius: 4px;
}
</style>