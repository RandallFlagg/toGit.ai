<template>
  <div class="container">
    <nav class="breadcrumb">
      <ul>
        <li><span>Select:</span></li>
        <li><a href="#" :class="{ disabled: !navEnabled.tracked }" @click="toggleNav('tracked')">Tracked</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.untracked }" @click="toggleNav('untracked')">Untracked</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.added }" @click="toggleNav('added')">Added</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.deleted }" @click="toggleNav('deleted')">Deleted</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.modified }" @click="toggleNav('modified')">Modified</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.files }" @click="toggleNav('files')">Files</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.submodules }" @click="toggleNav('submodules')">Submodules</a>
        </li>
      </ul>
    </nav>
    <div class="content">
      <div class="table-pane">
        <table>
          <thead>
            <tr>
              <th>
                <input type="checkbox" :checked="allChecked" @click="toggleAllCheckboxes($event)">
              </th>
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
                <input v-model="item.selected" type="checkbox" :checked="isChecked(item.file_status)"
                  @click.stop="changeStatus(item, $event)">
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
        <DiffViewer :diffString="diffString" />
        </div>
        <div v-else>
          <h3>Select an item to preview</h3>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
//TODO: Updated status as the checkbox is being de/selected
//TODO: Make the preview float to the right? Find a better solution for the preview pane
//TODO: Implmeent the selection buttons for the differnt files types
//TODO: What should be shown in the preview pane when a file is selected? Modofied, Staged, Untracked, Deleted, etc.

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
const navEnabled = ref({
  untracked: false,
  tracked: false,
  added: false,
  deleted: false,
  modified: false,
  files: false,
  submodules: false
});

const toggleNav = (section, statusPrefix) => {
  TAURI //TODO: Implement the Tauri API to get the status of the files
  navEnabled.value[section] = !navEnabled.value[section];
  props.items.forEach(item => {
    if (item.file_status.startsWith(statusPrefix)) {
      item.selected = true;
      changeStatus(item, { target: { checked: true } });
    }
  });
};

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
  const fileContent = await window.__TAURI__.core.invoke('get_file_content', { repoPath: "../../TEST REPO", relativeFilePath: filePath });
  // previewText.value = fileContent;
  diffString.value = fileContent.trim(); // TODO: Remove the trim and fix before being sent
};

const changeStatus = async (item, event) => {
  // debugger
  console.log(item);
  const status = await window.__TAURI__.core.invoke('change_file_status', { repoPath: "../../TEST REPO", relativeFilePath: item.relative_file_path, command: event.target.checked ? "Add" : "Unstage", newFilePath: null });
};

const isChecked = (status) => {
  return status.startsWith("INDEX_");
};

const toggleAllCheckboxes = async (event) => {
  const isChecked = event.target.checked;
  //TODO: Need to check a case of modified file after staged
  const status = await window.__TAURI__.core.invoke('change_file_status', { repoPath: "../../TEST REPO", relativeFilePath: "*", command: event.target.checked ? "Add All" : "Unstage All", newFilePath: null });//TODO: Find a better solution for the relative file path parameter. Maybe use Some?
  props.items.forEach(item => {
    item.selected = isChecked;
  });
};

const allChecked = computed(() => {
  return props.items.length > 0 && props.items.every(item => isChecked(item.file_status));
});
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
}

.content {
  display: flex;
  flex-direction: row;
  flex: 1;
}

/* Breadcrumb styles */
.breadcrumb {
  margin-bottom: 10px;
}

.breadcrumb ul {
  list-style: none;
  padding: 0;
  display: flex;
}

.breadcrumb li {
  margin-right: 5px;
}

.breadcrumb a {
  text-decoration: none;
  color: #007bff;
}

.breadcrumb a:hover {
  text-decoration: underline;
}

.breadcrumb a.disabled {
  /* pointer-events: none; */
  color: grey;
}

/*TODO: Fix styling for the table. Using element rules!*/
.table-pane {
  /* width: 50%; */
  /* overflow-x: auto; */
  flex: 1;
  overflow: auto;
}

.preview-pane {
  flex-grow: 1;
  padding: 10px;
  border-left: 1px solid #ccc;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 8px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

th {
  cursor: pointer;
}

th input {
  cursor: default;
}

input[type="checkbox"] {
  width: 20px;
  height: 20px;
}
</style>
