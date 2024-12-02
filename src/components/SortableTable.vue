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
            <th @click="sortTable('createdBy')">
              Created By
            </th>
            <th @click="sortTable('createdAt')">
              Created At
            </th>
            <th @click="sortTable('modifiedBy')">
              Modified By
            </th>
            <th @click="sortTable('modifiedAt')">
              Modified At
            </th>
            <th @click="sortTable('comments')">
              Comments
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
            <td>{{ item.fileName }}</td>
            <td>{{ item.fileType }}</td>
            <td>{{ item.status }}</td>
            <td>{{ item.size }}</td>
            <td>{{ item.createdBy }}</td>
            <td>{{ item.createdAt }}</td>
            <td>{{ item.modifiedBy }}</td>
            <td>{{ item.modifiedAt }}</td>
            <td>{{ item.comments }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div class="preview-pane">
      <div v-if="selectedItem">
        <h3>Preview: {{ selectedItem.fileName }}</h3>
        <p>{{ selectedItem.preview }}</p>
      </div>
      <div v-else>
        <h3>Select an item to preview</h3>
      </div>
    </div>
  </div>
</template>

<script>
console.log("CCC")
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
      selectedItem: null
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
