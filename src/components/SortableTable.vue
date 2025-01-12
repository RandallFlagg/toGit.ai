<template>
  <div class="split-pane">
    <div class="table-pane">
      <table>
        <thead>
          <tr>
            <th><input type="checkbox"></th>
            <th @click="sortTable('fileName')">
              File Name
            </th>
            <th @click="sortTable('fileType')">
              File Type
            </th>
            <th @click="sortTable('status')">
              Status
            </th>
            <th @click="sortTable('size')">
              Size
            </th>
            <th @click="sortTable('full_file_path')">
              Full File Path
            </th>
            <th @click="sortTable('relative_file_path')">
              Relative File Path
            </th>
            <th @click="sortTable('fileExtension')">
              File Extension
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="item in sortedItems"
            :key="item.id"
            @click="selectItem(item)"
          >
            <td>
              <input
                v-model="item.selected"
                type="checkbox"
              >
            </td>
            <td>{{ item.file_name }}</td>
            <td>{{ item.file_type }}</td>
            <td>{{ item.file_status }}</td>
            <td>{{ item.size }}</td>
            <td>{{ item.full_file_path }}</td>
            <td>{{ item.relative_file_path }}</td>
            <td>{{ item.file_extension }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div class="preview-pane">
      <div v-if="selectedItem">
        <h3>Preview: {{ selectedItem.file_name }}</h3>
        <p>{{ previewText }}</p>
      </div>
      <div v-else>
        <h3>Select an item to preview</h3>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'SortableTable',
  props: {
    items: {
      type: Array,
      required: true
    }
  },
  data() {
    return {
      sortColumn: '',
      sortOrder: 'asc',
      selectedItem: null,
      previewText: ''
    };
  },
  computed: {
    sortedItems() {
      return [...this.items].sort((a, b) => {
        if (a[this.sortColumn] > b[this.sortColumn]) return this.sortOrder === 'asc' ? 1 : -1;
        if (a[this.sortColumn] < b[this.sortColumn]) return this.sortOrder === 'asc' ? -1 : 1;
        return 0;
      });
    }
  },
  methods: {
    sortTable(column) {
      if (this.sortColumn === column) {
        this.sortOrder = this.sortOrder === 'asc' ? 'desc' : 'asc';
      } else {
        this.sortColumn = column;
        this.sortOrder = 'asc';
      }
    },
    selectItem(item) {
      this.selectedItem = item;
      this.getFileContent(item.full_file_path);
    },
    async getFileContent(fullFilePath) {
      const fileContent = await window.__TAURI__.core.invoke('get_file_content', { fullFilePath: fullFilePath });
      this.previewText = fileContent;
    }
  }
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
