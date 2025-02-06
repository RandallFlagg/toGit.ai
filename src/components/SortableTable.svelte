<script>
  //TODO: Updated status as the checkbox is being de/selected
  //TODO: Make the preview float to the right? Find a better solution for the preview pane
  //TODO: Implmeent the selection buttons for the differnt files types
  //TODO: What should be shown in the preview pane when a file is selected? Modofied, Staged, Untracked, Deleted, etc.
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { writable } from "svelte/store";
  import DiffViewer from "./DiffViewer.svelte";

  // Define writable stores
  //const diffString = writable('');
  let diffString;
  const explanationVisible = writable(false);
  const navEnabled = writable({
    untracked: false,
    tracked: false,
    added: false,
    deleted: false,
    modified: false,
    files: false,
    submodules: false
  });
  const previewText = writable('');
  // const selectedItem = writable(null);
  let selectedItem;
  // const sortColumn = writable('');
  let sortColumn;
  // const sortOrder = writable('asc');
  let sortOrder;// = "asc";
  let tableData = [];// = writable([]);

  const statuses = {
    INDEX_NEW: "New",
    WT_NEW: "Untracked",
    INDEX_DELETED: "Deleted",
    WT_DELETED: "Missing",
    INDEX_MODIFIED: "Staged",
    WT_MODIFIED: "Modified",
    INDEX_RENAMED: "Renamed(Staged)",
    WT_RENAMED: "Renamed",
    INDEX_TYPECHANGE: "Type Changed(Staged)",
    WT_TYPECHANGE: "Type Changed",
    IGNORED: "Ignored",
    CONFLICTED: "Conflicted",
  };

  const trackedStatuses = [
    "INDEX_NEW",
    "INDEX_DELETED",
    "WT_DELETED",
    "INDEX_MODIFIED",
    "WT_MODIFIED",
    "INDEX_RENAMED",
    "WT_RENAMED",
  ];

  onMount(() => {
    toggleNav("staged", "INDEX"); //All staged files
  });

  listen("file-watcher", async (event) => {
    console.log("Received event:", event.payload);
    await updateTable();
    // Handle the event payload (e.g., update UI)
  });

  //TODO: Fix the logic so it will work
  // Sorting logic

  $: sortedItems = tableData ? [...tableData].sort((a, b) => {
    const aValue = a[sortColumn];
    const bValue = b[sortColumn];
    
    // Handle undefined or null values by treating them as empty strings
    const aString =
      aValue !== undefined && aValue !== null
        ? aValue.toString().toLowerCase()
        : "";
    const bString =
      bValue !== undefined && bValue !== null
        ? bValue.toString().toLowerCase()
        : "";

    // Convert values to numbers if possible
    const aNumber = parseFloat(aString);
    const bNumber = parseFloat(bString);

    // Check if both values are numbers
    const aIsNumber = !isNaN(aNumber);
    const bIsNumber = !isNaN(bNumber);

    if (aIsNumber && bIsNumber) {
      // Compare as numbers
      return sortOrder === "asc" ? aNumber - bNumber : bNumber - aNumber;
    } else {
      // Compare as strings
      if (aString > bString) return sortOrder === "asc" ? 1 : -1;
      if (aString < bString) return sortOrder === "asc" ? -1 : 1;
      return 0;
    }
  }) : [];

  const determineAllChecked = () => {
    return (
      tableData.length > 0 &&
      tableData.every((item) => isChecked(item.file_status))
    );
  };

  // Function to fetch data (simulate fetching data from somewhere else)
  const getRepoStatus = async () => {
    debugLog("Get Repo Status Enter");
    const fetchedData = await window.__TAURI__.core.invoke(
      "get_repo_status",
      {},
    );
    debugLog(fetchedData);
    debugLog("Get Repo Status Exit");
    return fetchedData;
  };

  const updateTable = async () => {
    const fetchedDataAfter = await getRepoStatus();
    tableData = fetchedDataAfter;
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
      const command = commands.split(":");
      // relativeFilePath, command, newFilePath,
      changeStatusObject = {
        newFilesPath: [],
        command: event.target.checked ? command[0] : command[1],
      };
    }
    // tableData.value.forEach(item => {
    fetchedDataBefore.forEach((item) => {
      switch (section) {
        case "tracked":
          debugLog("1");
          if (trackedStatuses.includes(item.file_status)) {
            changeStatusObject.newFilesPath.push(item.relative_file_path);
          }
          //item.selected = trackedStatuses.includes(item.file_status);//TODO: Shouold we use statuses and remove the const?
          break;
        case "untracked":
          debugLog("2");
          item.selected = !trackedStatuses.includes(item.file_status);
          break;
        case "deleted":
          debugLog("3");
          item.selected = item.file_status === "DELETED";
          break;
        case "conflicted":
          debugLog("4");
          item.selected = item.file_status === "CONFLICTED";
          break;
        case "staged":
          debugLog("5");
          item.selected = item.file_status.startsWith(statuses);
          break;
        default:
          throw new Error("Unknown section: " + section);
      }
    });
    if (section !== "staged") {
      await changeStatus(changeStatusObject);
    }
    updateTable();
    navEnabled[section] = !navEnabled[section];
  };

  const sortTable = (column) => {
    //TODO: Some column are not sortable. Need to fix this
    if (sortColumn === column) {
      sortOrder = sortOrder === "asc" ? "desc" : "asc";
    } else {
      sortColumn = column;
      sortOrder = "asc";
    }
    //From here it continues to the sortedItem computed function
  };

  const selectItem = (item) => {
    selectedItem = item;
    getFileDiff(item.relative_file_path);
  };

  const getFileDiff = async (filePath) => {
    const fileContent = await window.__TAURI__.core.invoke("get_file_content", {
      repoPath: "../../TEST REPO",
      relativeFilePath: filePath,
    });

    // previewText.value = fileContent;
    diffString = fileContent.trim(); // TODO: Remove the trim and fix before being sent
  };

  const toggleCheckbox = async (item, event) => {
    const changeStatusObject = {
      relativeFilePath: item.relative_file_path,
      command: event.target.checked
        ? item.file_status.includes("DELETED")
          ? "Remove"
          : "Add"
        : "Unstage",
    };
    await changeStatus(changeStatusObject);
  };

  const changeStatus = async (changeStatusObject) => {
    console.log(changeStatusObject);
    debugLog(changeStatusObject);
    const result = await window.__TAURI__.core.invoke("change_file_status", {
      relativeFilePath: changeStatusObject.relativeFilePath || "",
      command: changeStatusObject.command,
      newFilePath: changeStatusObject.newFilePath || null,
      newFilesPath: changeStatusObject.newFilesPath || null,
    });
    debugLog(result);
    return result;
  };

  const isChecked = (status) => {
    return status.startsWith("INDEX_");
  };

  const toggleAllCheckboxes = async (event) => {
    debugger;
    // const isChecked = event.target.checked;
    // //TODO: Need to check a case of modified file after staged - Add more descritption
    // await changeStatus({
    //   relativeFilePath: "*",
    //   command: isChecked ? "Add All" : "Unstage All",
    // });
    // tableData.forEach((item) => {
    //   item.selected = isChecked;
    // });
  };

  // Reactive statement to update `allChecked` based on the function result
  $: allChecked = determineAllChecked();
</script>

<div class="container">
  <button type="button" on:click={updateTable} class="command-button"
    >Update Table</button
  >
  <nav class="breadcrumb">
    <ul>
      <li><span>Select:</span></li>
      <li>
        <a
          href="#"
          class:disabled={!navEnabled.tracked}
          class="help"
          on:click={(e) =>
            toggleNav("tracked", trackedStatuses, "Add Files:Unstage Files", e)}
          title="Stage all modified and deleted paths. (-a, --all)"
        >
          Tracked
        </a>
      </li>
      <li>
        <a
          href="#"
          class:disabled={!navEnabled.untracked}
          on:click={(e) => toggleNav("untracked", "WT", null, e)}>Untracked</a
        >
      </li>
      <li>
        <a
          href="#"
          class:disabled={!navEnabled.added}
          on:click={(e) => toggleNav("added", "INDEX", null, e)}>Added</a
        >
      </li>
      <li>
        <a
          href="#"
          class:disabled={!navEnabled.deleted}
          on:click={(e) => toggleNav("deleted", null, null, e)}>Deleted</a
        >
      </li>
      <li>
        <a
          href="#"
          class:disabled={!navEnabled.modified}
          on:click={(e) => toggleNav("modified", "INDEX", null, e)}>Modified</a
        >
      </li>
      <li>
        <a
          href="#"
          class:disabled={!navEnabled.files}
          on:click={(e) => toggleNav("files", null, null, e)}>Files</a
        >
      </li>
      <li>
        <a
          href="#"
          class:disabled={!navEnabled.submodules}
          on:click={(e) => toggleNav("submodules", null, null, e)}>Submodules</a
        >
      </li>
    </ul>
  </nav>
  <div class="content">
    <div class="table-pane">
      <table class="table">
        <thead>
          <tr>
            <th>
              <input
                type="checkbox"
                bind:checked={allChecked}
                on:click={toggleAllCheckboxes}
              />
            </th>
            <th on:click={() => sortTable("file_name")}>File Name</th>
            <th on:click={() => sortTable("file_type")}>File Type</th>
            <th on:click={() => sortTable("file_status")}>Status</th>
            <th on:click={() => sortTable("size")}>Size</th>
            <!-- <th on:click={() => sortTable('full_file_path')}>Full File Path</th> -->
            <th on:click={() => sortTable("relative_file_path")}
              >Relative File Path</th
            >
            <th on:click={() => sortTable("file_extension")}>File Extension</th>
          </tr>
        </thead>
        <tbody>
          {#each sortedItems as item (item.id)}
            <tr on:click={() => selectItem(item)}>
              <td>
                <input
                  type="checkbox"
                  checked={isChecked(item.file_status)}
                  on:click={(e) => toggleCheckbox(item, e)}
                />
              </td>
              <td>{item.file_name}</td>
              <td>{item.file_type}</td>
              <td>{statuses[item.file_status]}</td>
              <td>{item.size}</td>
              <!-- <td>{item.full_file_path}</td> -->
              <td>{item.relative_file_path}</td>
              <td>{item.file_extension}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <div class="preview-pane">
      {#if selectedItem}
        <!-- TODO: Use vue-diff instead of diff2html? -->
        <DiffViewer {diffString} />
      {:else}
        <h3>Select an item to preview</h3>
      {/if}
    </div>
  </div>
</div>

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

  .table th > input {
    /*TODO: Change to class */
    cursor: default;
  }

  .table th > input[type="checkbox"],
  .table td > input[type="checkbox"] {
    /*TODO: Change to class */
    width: 20px;
    height: 20px;
  }
</style>
