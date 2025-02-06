<script>
  // import ResizableSplitPane from './ResizableSplitPane.vue';
  import SortableTable from "./SortableTable.svelte";
  // import { writable } from "svelte/store";

  //TODO: Add incremental search
  //TODO: Create a seperator between staged/unstaged
  //TODO: Add partial staging
  //TODO: Add colors for different stages?
  //TODO: Add a way to select all staged/unstage
  //TODO: Add a new branch button
  // TODO: Add buttons to hide/show ignored files
  // TODO: Add buttons to hide/show untracked files

  // Reactive references for form data and table data
  // Define writable stores
  const form = {
    message: "",
    all: false,
    amend: false,
    amendOptions: {
      message: false,
      contents: false,
      author: false,
      date: false,
      files: false,
    },
    noEdit: false,
    signoff: false,
    gpgSign: "",
    quiet: false,
    dryRun: false,
    untrackedFiles: "",
    allowEmpty: false,
    interactivePatch: false,
    verbose: false,
    commitRef: "",
    fixup: "",
    file: "",
    allowEmptyMessage: false,
    noVerify: false,
    editMessage: false,
    author: "",
    date: "",
    cleanup: "",
    status: false,
    pathspecFile: "",
    pathspecFileNul: false,
    trailer: "",
  };

  //const commandOutput = writable("");
  let commandOutput;

  // Function to generate the git command
  const generateCommand = async () => {
    let command = "git commit";

    if (form.message) command += ` -m "${form.message}"`;
    if (form.all) command += " --all";
    if (form.amend) command += " --amend";
    if (form.noEdit) command += " --no-edit";
    if (form.signoff) command += " --signoff";
    if (form.gpgSign) command += ` --gpg-sign=${form.gpgSign}`;
    if (form.quiet) command += " --quiet";
    if (form.dryRun) command += " --dry-run";
    if (form.untrackedFiles)
      command += ` --untracked-files=${form.untrackedFiles}`;
    if (form.allowEmpty) command += " --allow-empty";
    if (form.interactivePatch) command += " --interactive --patch";
    if (form.verbose) command += " --verbose";
    if (form.commitRef) command += ` -C ${form.commitRef}`;
    if (form.fixup) command += ` --fixup=${form.fixup}`;
    if (form.file) command += ` -F ${form.file}`;
    if (form.allowEmptyMessage) command += " --allow-empty-message";
    if (form.noVerify) command += " --no-verify";
    if (form.editMessage) command += " -e";
    if (form.author) command += ` --author=${form.author}`;
    if (form.date) command += ` --date=${form.date}`;
    if (form.cleanup) command += ` --cleanup=${form.cleanup}`;
    if (form.status) command += " --status";
    if (form.pathspecFile)
      command += ` --pathspec-from-file=${form.pathspecFile}`;
    if (form.pathspecFileNul) command += " --pathspec-file-nul";
    if (form.trailer) command += ` --trailer=${form.trailer}`;

    commandOutput = "NOT COMMITTED: " + command;
    // const status = await window.__TAURI__.core.invoke('commit', { });
    //TODO: Make the error messages show on the frontend
  };
</script>

<div class="container">
  <a href="/" class="back-link"> Back to Index </a>
  <h1>Git Commit Command Generator</h1>
  <form class="main-form" on:submit|preventDefault={generateCommand}>
    <!-- Commit Message -->
    <label title="-m, --message <message>" class="commit-message-container">
      Commit Message <!-- <span class="required">*</span> -->
      <span class="help" title="Use the given <message> as the commit message."
        >?</span
      >
      <textarea
        bind:value={form.message}
        class="commit-message commit-message-box"
        name="message"
        placeholder="<message>"
        required
      ></textarea>
    </label>
    <details>
      <summary>Options not in use</summary>
      <span>-q --quiet</span>
      <br />
      <span>--no-edit</span>
      <br />
      <span>--allow-empty</span>
      <br />
      <span>--allow-empty-message</span>
      <span>AAa</span>
      <span>AAA</span>
      <span>AAA</span>
      <br />
      <label title="--dry-run">
        Dry Run
        <input bind:checked={form.dryRun} type="checkbox" name="dryRun" />
        <span
          class="help"
          title="Show what would be committed without actually committing."
          >?</span
        >
      </label>
    </details>
    <details>
      <summary>Additional Options</summary>

      <!-- Checkbox Inputs -->
      <fieldset class="flex-row">
        <legend>Checkbox Options</legend>

        <label title="--amend">
          Amend
          <input bind:checked={form.amend} type="checkbox" name="amend" />
          <span class="help" title="Amend the tip of the current branch."
            >?</span
          >
        </label>

        <label title="-s, --signoff">
          Sign Off
          <input bind:checked={form.signoff} type="checkbox" name="signoff" />
          <span
            class="help"
            title="Add Signed-off-by line at the end of the commit message."
            >?</span
          >
        </label>

        <label title="-a, --interactive, --patch">
          Interactive/Patch
          <input
            bind:checked={form.interactivePatch}
            type="checkbox"
            name="interactivePatch"
          />
          <span class="help" title="Stage changes interactively.">?</span>
        </label>

        <label title="-v, --verbose">
          Verbose
          <input bind:checked={form.verbose} type="checkbox" name="verbose" />
          <span class="help" title="Show unified diff of all file changes."
            >?</span
          >
        </label>

        <label title="--no-verify">
          No Verify
          <input bind:checked={form.noVerify} type="checkbox" name="noVerify" />
          <span class="help" title="Bypass pre-commit and commit-msg hooks."
            >?</span
          >
        </label>

        <label title="-e --edit">
          Edit Message
          <input
            bind:checked={form.editMessage}
            type="checkbox"
            name="editMessage"
          />
          <span
            class="help"
            title="The message taken from file with -F, command line with -m, and from commit object with -C are usually used as the commit log message unmodified. This option lets you further edit the message taken from these sources."
            >?</span
          >
        </label>

        <label title="--status">
          Status
          <input bind:checked={form.status} type="checkbox" name="status" />
          <span
            class="help"
            title="Include the output of git-status in the commit message template."
            >?</span
          >
        </label>

        <label title="--pathspec-file-nul">
          Pathspec File Null
          <input
            bind:checked={form.pathspecFileNul}
            type="checkbox"
            name="pathspecFileNul"
          />
          <span
            class="help"
            title="Only meaningful with --pathspec-from-file. Pathspec elements are separated with NUL character and all other characters are taken literally (including newlines and quotes)."
            >?</span
          >
        </label>
      </fieldset>
      <!-- Amend Options -->
      {#if form.amend}
        <fieldset class="flex-row">
          <legend>Amend Options</legend>

          <!-- Commit Message -->
          <div>
            <input
              type="checkbox"
              id="amendMessage"
              bind:checked={form.amendOptions.message}
            />
            <label for="amendMessage">Change Commit Message</label>
            <span title="Command: git commit --amend -m 'New commit message'"
              >❓</span
            >
            <p>
              Change the commit message to better describe the changes made.
            </p>
          </div>

          <!-- Commit Contents -->
          <div>
            <input
              type="checkbox"
              id="amendContents"
              bind:checked={form.amendOptions.contents}
            />
            <label for="amendContents">Modify Commit Contents</label>
            <span title="Command: git add <file> && git commit --amend">❓</span
            >
            <p>
              Modify the files included in the commit by adding changes to the
              staging area and then amending the commit.
            </p>
          </div>

          <!-- Author Information -->
          <div>
            <input
              type="checkbox"
              id="amendAuthor"
              bind:checked={form.amendOptions.author}
            />
            <label for="amendAuthor">Change Author Information</label>
            <span
              title="Command: git commit --amend --author='New Author <new.author@example.com>'"
              >❓</span
            >
            <p>Change the author and committer information of the commit.</p>
          </div>

          <!-- Commit Date and Time -->
          <div>
            <input
              type="checkbox"
              id="amendDate"
              bind:checked={form.amendOptions.date}
            />
            <label for="amendDate">Change Commit Date and Time</label>
            <span
              title="Command: GIT_COMMITTER_DATE='YYYY-MM-DD HH:MM:SS' git commit --amend --no-edit"
              >❓</span
            >
            <p>
              Change the date and time of the commit using environment
              variables.
            </p>
          </div>

          <!-- Add or Remove Files -->
          <div>
            <input
              type="checkbox"
              id="amendFiles"
              bind:checked={form.amendOptions.files}
            />
            <label for="amendFiles">Add or Remove Files</label>
            <span
              title="Command: git rm <file> && git add <new-file> && git commit --amend"
              >❓
            </span>
            <p>
              Add or remove files from the commit by staging the necessary
              changes before amending.
            </p>
          </div>
        </fieldset>
      {/if}

      <!-- Text Inputs -->
      <fieldset>
        <legend>Text Inputs</legend>

        <label title="-S, --gpg-sign">
          GPG Sign
          <input
            bind:value={form.gpgSign}
            type="text"
            name="gpgSign"
            placeholder="<key-id>"
          />
          <span class="help" title="GPG-sign commit using the given key."
            >?</span
          >
        </label>

        <label title="-u, --untracked-files">
          Untracked Files
          <input
            bind:value={form.untrackedFiles}
            type="text"
            name="untrackedFiles"
            placeholder="<mode>"
          />
          <span class="help" title="Include untracked files.">?</span>
        </label>

        <label title="--(c | -C | --squash) <commit>">
          Commit Reference
          <input
            bind:value={form.commitRef}
            type="text"
            name="commitRef"
            placeholder="<commit>"
          />
          <span
            class="help"
            title="Reuse existing commit message or squash commit.">?</span
          >
        </label>

        <label title="--fixup [(amend|reword):]<commit>">
          Fixup
          <input
            bind:value={form.fixup}
            type="text"
            name="fixup"
            placeholder="[(amend|reword):]<commit>"
          />
          <span class="help" title="Create a fixup commit.">?</span>
        </label>

        <label title="-F <file> --file=<file>">
          File
          <input
            bind:value={form.file}
            type="text"
            name="file"
            placeholder="<file>"
          />
          <span class="help" title="Take commit message from file.">?</span>
        </label>

        <label title="--author=<author>">
          Author
          <input
            bind:value={form.author}
            type="text"
            name="author"
            placeholder="<author>"
          />
          <span
            class="help"
            title="Override the author name used in the commit.">?</span
          >
        </label>

        <label title="--date=<date>">
          Date
          <input
            bind:value={form.date}
            type="text"
            name="date"
            placeholder="<date>"
          />
          <span
            class="help"
            title="Override the author date used in the commit.">?</span
          >
        </label>

        <label title="--cleanup=<mode>">
          Cleanup Mode
          <input
            bind:value={form.cleanup}
            type="text"
            name="cleanup"
            placeholder="<mode>"
          />
          <span class="help" title="Specify how to clean up the commit message."
            >?</span
          >
        </label>

        <label title="--pathspec-from-file=<file>">
          Pathspec from File
          <input
            bind:value={form.pathspecFile}
            type="text"
            name="pathspecFile"
            placeholder="<file>"
          />
          <span class="help" title="Read pathspec from a file.">?</span>
        </label>

        <label title="--trailer <token>[(=|:)<value>]">
          Trailer
          <input
            bind:value={form.trailer}
            type="text"
            name="trailer"
            placeholder="<token>[(=|:)<value>]"
          />
          <span class="help" title="Add trailers to the commit message.">?</span
          >
        </label>
      </fieldset>
    </details>
    <details id="fix">
      <summary>Commit Amend Details</summary>
      <pre><code>
Certainly! Here are the things you can change in a Git commit:

1. **Commit Message:**
   - You can change the commit message to better describe the changes made.
   &#96;&#96;&#96;bash
   git commit --amend -m "New commit message"
   &#96;&#96;&#96;

2. **Commit Contents:**
   - You can modify the files included in the commit by adding changes to the staging area and then amending the commit.
   &#96;&#96;&#96;
   git add &lt;file&gt;
   git commit --amend
   &#96;&#96;&#96;

3. **Author Information:**
   - You can change the author and committer information of the commit.
   &#96;&#96;&#96;
   git commit --amend --author="New Author &lt;new.author@example.com&gt;"
   &#96;&#96;&#96;

4. **Commit Date and Time:**
   - You can change the date and time of the commit using the `GIT_COMMITTER_DATE` and `GIT_AUTHOR_DATE` environment variables.
   &#96;&#96;&#96;
   GIT_COMMITTER_DATE="YYYY-MM-DD HH:MM:SS" git commit --amend --no-edit
   &#96;&#96;&#96;

5. **Add or Remove Files:**
   - You can add or remove files from the commit by staging the necessary changes before amending.
   &#96;&#96;&#96;
   git rm &lt;file&gt;
   git add &lt;new-file&gt;
   git commit --amend
   &#96;&#96;&#96;

If you need to add more items in future queries, we'll continue numbering from here. If you have any specific changes in mind or need further details, just let me know! 😊
</code></pre>
    </details>

    <SortableTable />
    <!-- <ResizableSplitPane>
        <template #left>
          <SortableTable items="{tableData}" />
        </template>
<template #right>
          <div class="preview-content"-->
    <!-- Preview content here -->
    <!--
      <p>{selectedItem ? selectedItem.preview : "Select file to preview"}</p>
          </div>
        </template>
</ResizableSplitPane> -->

    <button type="submit" class="submit-button" disabled={!form.message}>
      Generate Command
      <!--TODO: Fix the button style to handle a case it is disabled(currently on hover the color changes and the pointer is changing to a hand which is not relevant when disabled) -->
    </button>
  </form>
  <pre id="command-output">{commandOutput}</pre>
</div>

<style>
  .commit-message-container {
    display: block;
    width: 100%;
    margin-bottom: 8px;
    font-weight: bold;
  }

  .commit-message-box {
    width: 100%;
    height: 75px;
    /* Adjust height as needed */
    resize: vertical;
    /* Allows vertical resizing */
    padding: 10px;
    box-sizing: border-box;
    /* Ensures padding is included in the width */
    margin-top: 8px;
    /* Adds space between label text and textarea */
  }
</style>
