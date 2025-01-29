<template>
  <div class="container">
    <!-- TODO: Fix position -->
    <button @click="myFunc" class="command-button">Update Table</button>
    <nav class="breadcrumb">
      <ul>
        <li><span>Select:</span></li>
        <li>
          <a href="#" :class="[{ disabled: !navEnabled.tracked }, 'help']"
            @click="toggleNav('tracked', trackedStatuses, 'Add Files:Unstage Files', $event)"
            explanation="Stage all modified and deleted paths. (-a, --all)">
            Tracked
          </a>
        </li>
        <li><a href="#" :class="{ disabled: !navEnabled.untracked }" @click="toggleNav('untracked', 'WT')">Untracked</a>
        </li>
        <li><a href="#" :class="{ disabled: !navEnabled.added }" @click="toggleNav('added', 'INDEX')">Added</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.deleted }" @click="toggleNav('deleted')">Deleted</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.modified }" @click="toggleNav('modified', 'INDEX')">Modified</a>
        </li>
        <li><a href="#" :class="{ disabled: !navEnabled.files }" @click="toggleNav('files')">Files</a></li>
        <li><a href="#" :class="{ disabled: !navEnabled.submodules }" @click="toggleNav('submodules')">Submodules</a>
        </li>
      </ul>
    </nav>
    <div class="content">
      <div class="table-pane">
        <table class="table">
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
                  @click.stop="toggleCheckbox(item, $event)">
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

import { ref, computed, onMounted } from 'vue';
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
const trackedStatuses = ["INDEX_NEW", "INDEX_DELETED", "WT_DELETED", "INDEX_MODIFIED", "WT_MODIFIED", "INDEX_RENAMED", "WT_RENAMED"];

const diffString = ref('');
const explanationVisible = ref(false);
const navEnabled = ref({
  untracked: false,
  tracked: false,
  added: false,
  deleted: false,
  modified: false,
  files: false,
  submodules: false
});
const previewText = ref('');
const selectedItem = ref(null);
const sortColumn = ref('');
const sortOrder = ref('asc');
const tableData = ref([]);

onMounted(() => {
  //getRepoStatus();
  toggleNav('staged', 'INDEX'); //All staged files
});

const sortedItems = computed(() => {
  return [...tableData.value].sort((a, b) => {
    if (a[sortColumn.value] > b[sortColumn.value]) return sortOrder.value === 'asc' ? 1 : -1;
    if (a[sortColumn.value] < b[sortColumn.value]) return sortOrder.value === 'asc' ? -1 : 1;
    return 0;
  });
});

const allChecked = computed(() => {
  return tableData.value.length > 0 && tableData.value.every(item => isChecked(item.file_status));
});

// Function to fetch data (simulate fetching data from somewhere else)
const getRepoStatus = async () => {
  debugLog("Get Repo Status Enter");
  const fetchedData = await window.__TAURI__.core.invoke('get_repo_status', {});
  // tableData.value = fetchedData;
  debugLog(fetchedData);
  debugLog("Get Repo Status Exit");
  return fetchedData;
};

const toggleNav = async (section, statuses, commands, event) => {
  //TODO: Not loading because there is a deleted file. Need to fix this
  const fetchedDataBefore = await getRepoStatus();
  //TODO: add here reference to section.
  //TODO: Use AI to solve this problem
  //TODO: What happens if we delete a file?
  //TODO: Need to check the following scenario: Delete a staged file and load the UI. It get stuck. Why?
  let changeStatusObject;
  if (commands) {
    const command = commands.split(':');
    // relativeFilePath, command, newFilePath, 
    changeStatusObject = { newFilesPath: [], command: event.target.checked ? command[0] : command[1] };
  }
  // tableData.value.forEach(item => {
  fetchedDataBefore.forEach(item => {
    switch (section) {
      case 'tracked':
        debugLog("1");
        if (trackedStatuses.includes(item.file_status)) {
          changeStatusObject.newFilesPath.push(item.relative_file_path);
        }
        //item.selected = trackedStatuses.includes(item.file_status);//TODO: Shouold we use statuses and remove the const?
        break;
      case 'untracked':
        debugLog("2");
        item.selected = !trackedStatuses.includes(item.file_status);
        break;
      case 'deleted':
        debugLog("3");
        item.selected = item.file_status === 'DELETED';
        break;
      case 'conflicted':
        debugLog("4");
        item.selected = item.file_status === 'CONFLICTED';
        break;
      case 'staged':
        debugLog("5");
        item.selected = item.file_status.startsWith(statuses);
        break;
      default:
        throw "Unknown section: " + section;
    }
  });
  if (section !== "staged") {
    await changeStatus(changeStatusObject);
  }
  const fetchedDataAfter = await getRepoStatus();
  tableData.value = fetchedDataAfter;
  navEnabled.value[section] = !navEnabled.value[section];
};

const sortTable = (column) => {
  debugger;
  //TODO: Some column are not sortable. Need to fix this
  if (sortColumn.value === column) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortColumn.value = column;
    sortOrder.value = 'asc';
  }
};

const selectItem = (item) => {
  debugger;
  selectedItem.value = item;
  getFileDiff(item.relative_file_path);
};

const getFileDiff = async (filePath) => {
  const fileContent = await window.__TAURI__.core.invoke('get_file_content', { repoPath: "../../TEST REPO", relativeFilePath: filePath });
  // previewText.value = fileContent;
  diffString.value = fileContent.trim(); // TODO: Remove the trim and fix before being sent
};

const toggleCheckbox = async (item, event) => {
  debugger;
  const changeStatusObject = { relativeFilePath: item.relative_file_path, command: event.target.checked ? (item.file_status.includes("DELETED") ? "Remove" : "Add") : "Unstage" };
  await changeStatus(changeStatusObject);
}

const changeStatus = async (changeStatusObject) => {
  console.log(changeStatusObject);
  debugLog(changeStatusObject);
  const result = await window.__TAURI__.core.invoke('change_file_status', {
    relativeFilePath: changeStatusObject.relativeFilePath || "",
    command: changeStatusObject.command,
    newFilePath: changeStatusObject.newFilePath || null,
    newFilesPath: changeStatusObject.newFilesPath || null
  });
  debugLog(result);
  return result;
};

const isChecked = (status) => {
  return status.startsWith("INDEX_");
};

const toggleAllCheckboxes = async (event) => {
  const isChecked = event.target.checked;
  //TODO: Need to check a case of modified file after staged - Add more descritption
  await changeStatus({ relativeFilePath: "*", command: event.target.checked ? "Add All" : "Unstage All" });
  tableData.value.forEach(item => {
    item.selected = isChecked;
  });
};
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

.table {
  width: 100%;
  border-collapse: collapse;
}

.table th,
.table td {
  padding: 8px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

.table th {
  cursor: pointer;
}

.table th>input {
  /*TODO: Change to class */
  cursor: default;
}

.table th>input[type="checkbox"],
.table td>input[type="checkbox"] {
  /*TODO: Change to class */
  width: 20px;
  height: 20px;
}
</style>
