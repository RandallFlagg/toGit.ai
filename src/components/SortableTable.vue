<template>
  <div class="split-pane">
    <div class="table-pane">
      <table>
        <thead>
          <tr>
            <th><input type="checkbox"></th>
            <th @click="() => sortTable('fileName')">
              File Name
            </th>
            <th @click="() => sortTable('fileType')">
              File Type
            </th>
            <th @click="() => sortTable('status')">
              Status
            </th>
            <th @click="() => sortTable('size')">
              Size
            </th>
            <!-- <th @click="() => sortTable('full_file_path')">
              Full File Path
            </th> -->
            <th @click="() => sortTable('relative_file_path')">
              Relative File Path
            </th>
            <th @click="() => sortTable('fileExtension')">
              File Extension
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in sortedItems" :key="item.id" @click="() => selectItem(item)">
            <td>
              <input v-model="item.selected" type="checkbox" :checked="isChecked(item.file_status)">
            </td>
            <td>{{ item.file_name }}</td>
            <td>{{ item.file_type }}</td>
            <td>{{ statuses[item.file_status] }}</td>
            <td>{{ item.size }}</td>
            <!-- <td>{{ item.full_file_path }}</td> -->
            <td>{{ item.relative_file_path }}</td>
            <td>{{ item.file_extension }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div class="preview-pane">
      <div v-if="selectedItem">
        <h3>Preview: {{ selectedItem.file_name }}</h3>
        <p>{{ previewText }}</p><!--TODO: Do we still need this? remove-->
        <DiffViewer :diffString="diffString" />
      </div>
      <div v-else>
        <h3>Select an item to preview</h3>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import DiffViewer from './DiffViewer.vue';

const statuses = {
  "INDEX_NEW": "New",
  "WT_NEW": "Untracked",
  "INDEX_DELETED": "Deleted",
  "WT_DELETED": "Missing",
  "INDEX_MODIFIED": "Staged",
  "WT_MODIFIED": "Modified",
  "INDEX_RENAMED": "Renamed(Staged)",
  "WT_RENAMED": "Renamed",
  "INDEX_TYPECHANGE": "Type Changed(Staged)",
  "WT_TYPECHANGE": "Type Changed",
  "IGNORED": "Ignored",
  "CONFLICTED": "Conflicted",
};

const props = defineProps({
  items: {
    type: Array,
    required: true
  }
});

const sortColumn = ref('');
const sortOrder = ref('asc');
const selectedItem = ref(null);
const previewText = ref('');
const diffString = ref('');

const sortedItems = computed(() => {
  return [...props.items].sort((a, b) => {
    if (a[sortColumn.value] > b[sortColumn.value]) return sortOrder.value === 'asc' ? 1 : -1;
    if (a[sortColumn.value] < b[sortColumn.value]) return sortOrder.value === 'asc' ? -1 : 1;
    return 0;
  });
});

const sortTable = (column) => {
  if (sortColumn.value === column) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortColumn.value = column;
    sortOrder.value = 'asc';
  }
};

const selectItem = (item) => {
  selectedItem.value = item;
  getFileDiff(item.relative_file_path);
};

const getFileDiff = async (filePath) => {
  const fileContent = await window.__TAURI__.core.invoke('get_file_content', { repoPath: "../../TEST REPO", relativeFilePath: filePath });//TODO: Cahnge to get a diff instead of content
  // previewText.value = fileContent;
  diffString.value = fileContent.trim(); // TODO: Remove the trim and fix before being sent
};

const isChecked = (status) => {
  return status.startsWith("INDEX_");
};
</script>

<style scoped>
.split-pane {
  display: flex;
}

.table-pane {
  width: 50%;
  overflow-x: auto;
}

.preview-pane {
  flex-grow: 1;
  padding: 10px;
  border-left: 1px solid #ccc;
}
</style>
