<script setup lang="ts">
import { ref, onMounted } from 'vue';
// import ResizableSplitPane from './ResizableSplitPane.vue';
import SortableTable from './SortableTable.vue';

// Reactive references for form data and table data
const form = ref({
  message: '',
  all: false,
  amend: false,
  noEdit: false,
  signoff: false,
  gpgSign: '',
  quiet: false,
  dryRun: false,
  untrackedFiles: '',
  allowEmpty: false,
  interactivePatch: false,
  verbose: false,
  commitRef: '',
  fixup: '',
  file: '',
  allowEmptyMessage: false,
  noVerify: false,
  editMessage: false,
  author: '',
  date: '',
  cleanup: '',
  status: false,
  pathspecFile: '',
  pathspecFileNul: false,
  trailer: ''
});

const tableData = ref([]);
const selectedItem = ref(null);
const commandOutput = ref('');

// Function to generate the git command
const generateCommand = () => {
  let command = 'git commit';

  if (form.value.message) command += ` -m "${form.value.message}"`;
  if (form.value.all) command += ' --all';
  if (form.value.amend) command += ' --amend';
  if (form.value.noEdit) command += ' --no-edit';
  if (form.value.signoff) command += ' --signoff';
  if (form.value.gpgSign) command += ` --gpg-sign=${form.value.gpgSign}`;
  if (form.value.quiet) command += ' --quiet';
  if (form.value.dryRun) command += ' --dry-run';
  if (form.value.untrackedFiles) command += ` --untracked-files=${form.value.untrackedFiles}`;
  if (form.value.allowEmpty) command += ' --allow-empty';
  if (form.value.interactivePatch) command += ' --interactive --patch';
  if (form.value.verbose) command += ' --verbose';
  if (form.value.commitRef) command += ` -C ${form.value.commitRef}`;
  if (form.value.fixup) command += ` --fixup=${form.value.fixup}`;
  if (form.value.file) command += ` -F ${form.value.file}`;
  if (form.value.allowEmptyMessage) command += ' --allow-empty-message';
  if (form.value.noVerify) command += ' --no-verify';
  if (form.value.editMessage) command += ' -e';
  if (form.value.author) command += ` --author=${form.value.author}`;
  if (form.value.date) command += ` --date=${form.value.date}`;
  if (form.value.cleanup) command += ` --cleanup=${form.value.cleanup}`;
  if (form.value.status) command += ' --status';
  if (form.value.pathspecFile) command += ` --pathspec-from-file=${form.value.pathspecFile}`;
  if (form.value.pathspecFileNul) command += ' --pathspec-file-nul';
  if (form.value.trailer) command += ` --trailer=${form.value.trailer}`;

  commandOutput.value = command;
};

// Function to fetch data (simulate fetching data from somewhere else)
const fetchData = async () => {
  // const repo_path = "~/Ohad/Projects/OIV/OIV/"; // TODO: Adjust as needed
  const repo_path = "../TEST REPO"; // TODO: Adjust as needed
  const fetchedData = await window.__TAURI__.core.invoke('get_repo_changes', { repoPath: repo_path }); //TODO:From where should we get the repo path?
  const fetchedData2 = [
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
  tableData.value = fetchedData;
  debugger;
};

onMounted(() => {
  fetchData();
});
</script>
<template>
  <div class="container">
    <router-link to="/" class="back-link">
      Back to Index
    </router-link>
    <h1>Git Commit Command Generator</h1>
    <form class="main-form" @submit.prevent="generateCommand">
      <!-- Commit Message -->
      <label title="-m, --message <message>">
        Commit Message <!-- <span class="required">*</span> -->
        <span class="help" title="Use the given <message> as the commit message.">?</span>
        <textarea v-model="form.message" class="commit-message" name="message" placeholder="<message>" required />
      </label>
      <details>
        <summary>Additional Options</summary>

        <!-- Checkbox Inputs -->
        <fieldset class="flex-row">
          <legend>Checkbox Options</legend>

          <label title="-a, --all">
            All
            <input v-model="form.all" type="checkbox" name="all">
            <span class="help" title="Stage all modified and deleted paths.">?</span>
          </label>

          <label title="--amend">
            Amend
            <input v-model="form.amend" type="checkbox" name="amend">
            <span class="help" title="Amend the tip of the current branch.">?</span>
          </label>

          <label title="--no-edit">
            No Edit
            <input v-model="form.noEdit" type="checkbox" name="noEdit">
            <span class="help" title="Do not open an editor to modify the commit message.">?</span>
          </label>

          <label title="-s, --signoff">
            Sign Off
            <input v-model="form.signoff" type="checkbox" name="signoff">
            <span class="help" title="Add Signed-off-by line at the end of the commit message.">?</span>
          </label>

          <label title="-q, --quiet">
            Quiet
            <input v-model="form.quiet" type="checkbox" name="quiet">
            <span class="help" title="Suppress commit summary message.">?</span>
          </label>

          <label title="--dry-run">
            Dry Run
            <input v-model="form.dryRun" type="checkbox" name="dryRun">
            <span class="help" title="Show what would be committed without actually committing.">?</span>
          </label>

          <label title="--allow-empty">
            Allow Empty
            <input v-model="form.allowEmpty" type="checkbox" name="allowEmpty" disabled>
            <span class="help" title="Create a commit even if there are no changes.">?</span>
          </label>

          <label title="-a, --interactive, --patch">
            Interactive/Patch
            <input v-model="form.interactivePatch" type="checkbox" name="interactivePatch">
            <span class="help" title="Stage changes interactively.">?</span>
          </label>

          <label title="-v, --verbose">
            Verbose
            <input v-model="form.verbose" type="checkbox" name="verbose">
            <span class="help" title="Show unified diff of all file changes.">?</span>
          </label>

          <label title="--allow-empty-message">
            Allow Empty Message
            <input v-model="form.allowEmptyMessage" type="checkbox" name="allowEmptyMessage">
            <span class="help" title="Create a commit with an empty commit message.">?</span>
          </label>

          <label title="--no-verify">
            No Verify
            <input v-model="form.noVerify" type="checkbox" name="noVerify">
            <span class="help" title="Bypass pre-commit and commit-msg hooks.">?</span>
          </label>

          <label title="-e">
            Edit Message
            <input v-model="form.editMessage" type="checkbox" name="editMessage">
            <span class="help" title="Edit the commit message before committing.">?</span>
          </label>

          <label title="--status">
            Status
            <input v-model="form.status" type="checkbox" name="status">
            <span class="help" title="Include the output of git-status in the commit message template.">?</span>
          </label>

          <label title="--pathspec-file-nul">
            Pathspec File Null
            <input v-model="form.pathspecFileNul" type="checkbox" name="pathspecFileNul">
            <span class="help" title="Pathspec elements are separated with NUL character.">?</span>
          </label>
        </fieldset>

        <!-- Text Inputs -->
        <fieldset>
          <legend>Text Inputs</legend>

          <label title="-S, --gpg-sign">
            GPG Sign
            <input v-model="form.gpgSign" type="text" name="gpgSign" placeholder="<key-id>">
            <span class="help" title="GPG-sign commit using the given key.">?</span>
          </label>

          <label title="-u, --untracked-files">
            Untracked Files
            <input v-model="form.untrackedFiles" type="text" name="untrackedFiles" placeholder="<mode>">
            <span class="help" title="Include untracked files.">?</span>
          </label>

          <label title="--(c | -C | --squash) <commit>">
            Commit Reference
            <input v-model="form.commitRef" type="text" name="commitRef" placeholder="<commit>">
            <span class="help" title="Reuse existing commit message or squash commit.">?</span>
          </label>

          <label title="--fixup [(amend|reword):]<commit>">
            Fixup
            <input v-model="form.fixup" type="text" name="fixup" placeholder="[(amend|reword):]<commit>">
            <span class="help" title="Create a fixup commit.">?</span>
          </label>

          <label title="-F <file>">
            File
            <input v-model="form.file" type="text" name="file" placeholder="<file>">
            <span class="help" title="Take commit message from file.">?</span>
          </label>

          <label title="--author=<author>">
            Author
            <input v-model="form.author" type="text" name="author" placeholder="<author>">
            <span class="help" title="Override the author name used in the commit.">?</span>
          </label>

          <label title="--date=<date>">
            Date
            <input v-model="form.date" type="text" name="date" placeholder="<date>">
            <span class="help" title="Override the author date used in the commit.">?</span>
          </label>

          <label title="--cleanup=<mode>">
            Cleanup Mode
            <input v-model="form.cleanup" type="text" name="cleanup" placeholder="<mode>">
            <span class="help" title="Specify how to clean up the commit message.">?</span>
          </label>

          <label title="--pathspec-from-file=<file>">
            Pathspec from File
            <input v-model="form.pathspecFile" type="text" name="pathspecFile" placeholder="<file>">
            <span class="help" title="Read pathspec from a file.">?</span>
          </label>

          <label title="--trailer <token>[(=|:)<value>]">
            Trailer
            <input v-model="form.trailer" type="text" name="trailer" placeholder="<token>[(=|:)<value>]">
            <span class="help" title="Add trailers to the commit message.">?</span>
          </label>
        </fieldset>

      </details>

      <SortableTable :items="tableData" />
      <!-- <ResizableSplitPane>
        <template #left>
          <SortableTable :items="tableData" />
        </template>
<template #right>
          <div class="preview-content"-->
      <!-- Preview content here -->
      <!--p v-if="selectedItem">
              {{ selectedItem.preview }}
            </p>
            <p v-else>Select file to preview</p>
          </div>
        </template>
</ResizableSplitPane> -->

      <button type="submit" class="submit-button" :disabled="!form.message">
        Generate Command
      </button>
    </form>
    <pre id="command-output">{{ commandOutput }}</pre>
  </div>
</template>
