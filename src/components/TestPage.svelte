<template>
  START
  <main id="TEST">
    <button @click="sendDataToDiffViewer">Send Data to Diff Viewer</button>
    <DiffViewer :diffString="diffString" />
    <!-- <DiffViewer /> -->
    <!-- <SortableTable :items="tableData" /> -->
  </main>
  END
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import SortableTable from './SortableTable.vue';
import DiffViewer from './DiffViewer.vue';

// debugger;
// import * as mykey from '../assets/diff2html-ui.min.js';

// console.log(mykey.AAA)

// Define a reactive reference for table data
const tableData = ref([]);
const diffString = ref("");

function sendDataToDiffViewer() {
  const sampleString = '--- a/server/vendor/golang.org/x/sys/unix/zsyscall_linux_mipsle.go\n+++ b/server/vendor/golang.org/x/sys/unix/zsyscall_linux_mipsle.go\n@@ -1035,6 +1035,17 @@ func Prctl(option int, arg2 uintptr, arg3 uintptr, arg4 uintptr, arg5 uintptr) (\n \n // THIS FILE IS GENERATED BY THE COMMAND AT THE TOP; DO NOT EDIT\n \n+func Pselect(nfd int, r *FdSet, w *FdSet, e *FdSet, timeout *Timespec, sigmask *Sigset_t) (n int, err error) {\n+\tr0, _, e1 := Syscall6(SYS_PSELECT6, uintptr(nfd), uintptr(unsafe.Pointer(r)), uintptr(unsafe.Pointer(w)), uintptr(unsafe.Pointer(e)), uintptr(unsafe.Pointer(timeout)), uintptr(unsafe.Pointer(sigmask)))\n+\tn = int(r0)\n+\tif e1 != 0 {\n+\t\terr = errnoErr(e1)\n+\t}\n+\treturn\n+}\n+\n+// THIS FILE IS GENERATED BY THE COMMAND AT THE TOP; DO NOT EDIT\n+\n func read(fd int, p []byte) (n int, err error) {\n \tvar _p0 unsafe.Pointer\n \tif len(p) > 0 {\n';
  diffString.value = sampleString;
}

fetchData();

// Define a function to fetch data
function fetchData() {
  // Simulate fetching data from somewhere else
  const fetchedData = [
    {
      id: 1, fileName: 'file1.txt', fileType: 'Text', status: 'Added', size: '2 KB',
      createdBy: 'User A', createdAt: '2024-01-01', modifiedBy: 'User B', modifiedAt: '2024-01-05',
      comments: 'No comments', preview: 'This is the content of file1.txt', selected: false
    },
    {
      id: 2, fileName: 'file2.txt', fileType: 'Text', status: 'Committed', size: '3 KB',
      createdBy: 'User B', createdAt: '2024-01-02', modifiedBy: 'User C', modifiedAt: '2024-01-06',
      comments: 'Reviewed', preview: 'This is the content of file2.txt', selected: false
    },
    {
      id: 3, fileName: 'file3.txt', fileType: 'Text', status: 'Added', size: '4 KB',
      createdBy: 'User C', createdAt: '2024-01-03', modifiedBy: 'User D', modifiedAt: '2024-01-07',
      comments: 'Pending review', preview: 'This is the content of file3.txt', selected: false
    },
    {
      id: 4, fileName: 'file4.txt', fileType: 'Text', status: 'Committed', size: '5 KB',
      createdBy: 'User D', createdAt: '2024-01-04', modifiedBy: 'User E', modifiedAt: '2024-01-08',
      comments: 'Approved', preview: 'This is the content of file4.txt', selected: false
    },
    {
      id: 5, fileName: 'file5.txt', fileType: 'Text', status: 'Added', size: '6 KB',
      createdBy: 'User E', createdAt: '2024-01-05', modifiedBy: 'User F', modifiedAt: '2024-01-09',
      comments: 'Rejected', preview: 'This is the content of file5.txt', selected: false
    }
    // Other dummy data items...
  ];
  // tableData.value = fetchedData;//TODO: Uncomment this line to display the table
}

// Fetch data when the component is mounted
onMounted(() => {
  fetchData();
});
</script>
