<script>
  import { onMount, onDestroy } from "svelte";
  import { writable } from "svelte/store";
  import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";
  import { link } from "svelte-spa-router";
  import { path } from "@tauri-apps/api"; // /path
  import { open, save } from "@tauri-apps/plugin-dialog";
  // import FolderExplorerButton from './FolderExplorerButton.svelte';

  let commandOutput = "";

  let form = {
    verbose: false,
    quiet: false,
    progress: false,
    rejectShallow: false,
    noCheckout: false,
    bare: false,
    mirror: false,
    local: false,
    noHardlinks: false,
    shared: false,
    recurseSubmodules: false,
    jobs: "",
    template: "",
    reference: "",
    dissociate: false,
    origin: "",
    branch: "",
    uploadPack: "",
    depth: "",
    shallowSince: "",
    shallowExclude: "",
    singleBranch: false,
    noTags: false,
    shallowSubmodules: false,
    separateGitDir: "",
    refFormat: "files",
    config: "",
    serverOption: "",
    ipv4: false,
    ipv6: false,
    filter: "",
    alsoFilterSubmodules: false,
    remoteSubmodules: false,
    sparse: false,
    bundleUri: "",
    repo: "",
    dir: "",
  };

  //TODO: Take this out to util file
  const getCWD = async (trailingSlash) => {
    const result =
      (await __TAURI__.core.invoke("get_launch_path")) +
      (trailingSlash ? path.sep() : "");
    return result;
  };

  //TODO: If a path was passed from the command line or the file explorer use it in form.dir instead of the CWD
  onMount(async () => {
    console.log(Date.now() + ": entering clone onMount");
    let isFocusHandled = false;
    const clipboardToRepoInput = async () => {
      if (isFocusHandled) return;
      isFocusHandled = true;
      window.removeEventListener("focus", clipboardToRepoInput);
      console.log(Date.now() + ": Window has focus");

      try {
        // Use Tauri's clipboard API to read text
        const clipboardText = await readText();

        if (
          (clipboardText.startsWith("https") ||
            clipboardText.startsWith("git@")) &&
          clipboardText.endsWith(".git")
        ) {
          form.repo = clipboardText;
        }
      } catch (err) {
        debugger;
        console.error("Failed to read clipboard contents:", err);
      }

      setTimeout(() => {
        console.log("readdig");
        window.addEventListener("focus", clipboardToRepoInput);
        isFocusHandled = false;
      }, 1000);
    };

    // Add the focus event listener when the component mounts
    window.addEventListener("focus", clipboardToRepoInput);

    // Cleanup the event listener when the component is destroyed
    onDestroy(() => {
      console.log("Removing focus event listener");
      window.removeEventListener("focus", clipboardToRepoInput);
    });

    // const currentDir = await __TAURI__.path.executableDir();
    // console.log('Current Working Directory:', currentDir);
    form.dir = await getCWD(true);
    debugLog("Launch Path:", form.dir);

    console.log("exiting clone onMount");
  });

  $: if (form.repo) {
    if (form.repo.endsWith(".git")) {
      const repoName = getRepoName(form.repo);
      if (!form.dir.endsWith(repoName)) {
        form.dir = `${form.dir.replace(/\/[^/]*$/, "")}/${repoName}`;
      }
    }
  }

  const getRepoName = (url) => {
    const parts = url.split("/");
    const repoWithGit = parts[parts.length - 1];
    const repoName1 = repoWithGit.replace(".git", "");

    console.log(repoName1);

    const regex = /([^/]+)\.git$/;
    const match = url.match(regex);

    if (match) {
      const repoName2 = match[1];
      console.log(repoName2);
    } else {
      console.log("No match found");
    }

    return repoName1; //TODO: Decide which method we want to use and remove the unnecessary code
  };

  const openFolderExplorer = async () => {
    try {
      const selectedFolder = await open({
        directory: true,
        multiple: false,
        title: "Select a Folder",
      });

      if (selectedFolder) {
        form.dir = selectedFolder + path.sep();
        console.log("Selected folder:", selectedFolder);
        // You can handle the selected folder path here
      }
    } catch (error) {
      console.error("Error opening folder explorer:", error);
    }
  };

  const generateCommand = () => {
    let command = "git clone";

    if (form.verbose) command += " --verbose";
    if (form.quiet) command += " --quiet";
    if (form.progress) command += " --progress";
    if (form.rejectShallow) command += " --reject-shallow";
    if (form.noCheckout) command += " --no-checkout";
    if (form.bare) command += " --bare";
    if (form.mirror) command += " --mirror";
    if (form.local) command += " --local";
    if (form.noHardlinks) command += " --no-hardlinks";
    if (form.shared) command += " --shared";
    if (form.recurseSubmodules) command += " --recurse-submodules";

    if (form.jobs) command += ` --jobs=${form.jobs}`;
    if (form.template) command += ` --template=${form.template}`;
    if (form.reference) command += ` --reference=${form.reference}`;
    if (form.dissociate) command += " --dissociate";
    if (form.origin) command += ` --origin=${form.origin}`;
    if (form.branch) command += ` --branch=${form.branch}`;
    if (form.uploadPack) command += ` --upload-pack=${form.uploadPack}`;
    if (form.depth) command += ` --depth=${form.depth}`;
    if (form.shallowSince) command += ` --shallow-since=${form.shallowSince}`;
    if (form.shallowExclude)
      command += ` --shallow-exclude=${form.shallowExclude}`;
    if (form.singleBranch) command += " --single-branch";
    if (form.noTags) command += " --no-tags";
    if (form.shallowSubmodules) command += " --shallow-submodules";
    if (form.separateGitDir)
      command += ` --separate-git-dir=${form.separateGitDir}`;
    if (form.refFormat) command += ` --ref-format=${form.refFormat}`;
    if (form.config) command += ` --config=${form.config}`;
    if (form.serverOption) command += ` --server-option=${form.serverOption}`;
    if (form.ipv4) command += " --ipv4";
    if (form.ipv6) command += " --ipv6";
    if (form.filter) command += ` --filter=${form.filter}`;
    if (form.alsoFilterSubmodules) command += " --also-filter-submodules";
    if (form.remoteSubmodules) command += " --remote-submodules";
    if (form.sparse) command += " --sparse";
    if (form.bundleUri) command += ` --bundle-uri=${form.bundleUri}`;
    if (form.repo) command += ` ${form.repo}`;
    if (form.dir) command += ` ${form.dir}`;

    commandOutput = command;
  };
</script>

<div class="container">
  <a href="/index" class="back-link" use:link>Back to Index</a>
  <h1>Git Clone Commands</h1>

  <form id="git-clone-form" on:submit|preventDefault={generateCommand}>
    <div class="form-row">
      <label title="<repo>">
        <div class="editable-dropdown">
          Repository
          <input
            bind:value={form.repo}
            type="text"
            name="repo"
            placeholder="<repo>"
            list="repo-options"
          />
          <datalist id="repo-options">
            <!-- TODO: The values here should be populated onMount by a js call to the backend and load the history file -->
            <option value="R 1"></option>
            <option value="Re 2"></option>
            <option value="Rep 3"></option>
            <option value="Repo 4"></option>
            <option value="Repos 5"></option>
            <option value="Reposi 6"></option>
            <option value="Reposit 7"></option>
            <option value="Reposito 8"></option>
            <option value="Repositor 9"></option>
            <option value="Repository 10"></option>
          </datalist>
          <span class="help" title="The repository to clone.">?</span>
        </div>
      </label>
      <button
        on:click={openFolderExplorer}
        class="explorer-button"
        type="button"
      >
        Browse
      </button>
    </div>

    <div class="form-row">
      <label title="<dir>">
        Directory
        <input
          bind:value={form.dir}
          type="text"
          name="dir"
          placeholder="<dir>"
        />
        <span class="help" title="The directory to clone into.">?</span>
      </label>
      <button
        on:click={openFolderExplorer}
        class="explorer-button"
        type="button"
      >
        Browse
      </button>
      <!-- <FolderExplorerButton display-text="Browse" type="button" /> -->
    </div>

    <details open class="details">
      <summary>Shallow Clone</summary>
      <label title="--depth <depth>">
        Depth
        <input
          bind:value={form.depth}
          type="number"
          name="depth"
          placeholder="<depth>"
          disabled={!!form.bundleUri}
        />
        <span class="help" title="Create a shallow clone of that depth.">?</span
        >
      </label>
      <label title="--shallow-submodules">
        Shallow Submodules
        <input
          bind:checked={form.shallowSubmodules}
          type="checkbox"
          name="shallow-submodules"
        />
        <span
          class="help"
          title="All submodules which are cloned will be shallow with a depth of 1."
          >?</span
        >
      </label>

      <label title="--reject-shallow">
        Reject Shallow
        <input
          bind:checked={form.rejectShallow}
          type="checkbox"
          name="reject-shallow"
        />
        <span class="help" title="Don't clone shallow repository.">?</span>
      </label>

      <label title="--shallow-since <time>">
        Shallow Since
        <input
          bind:value={form.shallowSince}
          type="date"
          name="shallow-since"
          placeholder="<time>"
          disabled={!!form.bundleUri}
        />
        <span class="help" title="Create a shallow clone since a specific time."
          >?</span
        >
      </label>

      <label title="--shallow-exclude <revision>">
        Shallow Exclude
        <input
          bind:value={form.shallowExclude}
          type="text"
          name="shallow-exclude"
          placeholder="<revision>"
          disabled={!!form.bundleUri}
        />
        <span
          class="help"
          title="Deepen history of shallow clone, excluding rev.">?</span
        >
      </label>
    </details>

    <details open class="details">
      <summary>Checkouts</summary>
      <label title="--bundle-uri <uri>">
        Bundle URI
        <input
          bind:value={form.bundleUri}
          type="text"
          name="bundle-uri"
          placeholder="<uri>"
          disabled={form.depth || form.shallowExclude || !!form.shallowSince}
        />
        <span
          class="help"
          title="A URI for downloading bundles before fetching from origin remote."
          >?</span
        >
      </label>

      <label title="-n, --no-checkout">
        No Checkout
        <input
          bind:checked={form.noCheckout}
          type="checkbox"
          name="no-checkout"
        />
        <span class="help" title="Don't create a checkout.">?</span>
      </label>

      <label title="--bare">
        Bare Repository
        <input bind:checked={form.bare} type="checkbox" name="bare" />
        <span class="help" title="Create a bare repository.">?</span>
      </label>

      <label title="--mirror">
        Mirror Repository
        <input bind:checked={form.mirror} type="checkbox" name="mirror" />
        <span class="help" title="Create a mirror repository (implies --bare)."
          >?</span
        >
      </label>

      <label title="--sparse">
        Sparse
        <input bind:checked={form.sparse} type="checkbox" name="sparse" />
        <span
          class="help"
          title="Initialize sparse-checkout file to include only files at root."
          >?</span
        >
      </label>
    </details>

    <details open class="details">
      <summary>Locals</summary>
      <label title="-l, --local">
        Local
        <input bind:checked={form.local} type="checkbox" name="local" />
        <span class="help" title="To clone from a local repository.">?</span>
      </label>

      <label title="--no-hardlinks">
        No Hardlinks
        <input
          bind:checked={form.noHardlinks}
          type="checkbox"
          name="no-hardlinks"
        />
        <span class="help" title="Don't use local hardlinks, always copy."
          >?</span
        >
      </label>

      <label title="-s, --shared">
        Shared
        <input bind:checked={form.shared} type="checkbox" name="shared" />
        <span class="help" title="Setup as shared repository.">?</span>
      </label>

      <label title="--reference <repo>">
        Reference Repository
        <input
          bind:value={form.reference}
          type="text"
          name="reference"
          placeholder="<repo>"
        />
        <span class="help" title="Reference repository.">?</span>
      </label>
    </details>

    <details open class="details">
      <summary>Post Clone</summary>
      <label title="--also-filter-submodules">
        Also Filter Submodules
        <input
          bind:checked={form.alsoFilterSubmodules}
          type="checkbox"
          name="also-filter-submodules"
        />
        <span class="help" title="Apply partial clone filters to submodules."
          >?</span
        >
      </label>
      <label title="--recurse-submodules">
        Recurse Submodules
        <input
          bind:checked={form.recurseSubmodules}
          type="checkbox"
          name="recurse-submodules"
        />
        <span class="help" title="Initialize submodules in the clone.">?</span>
      </label>
      <label title="--filter <args>">
        Filter
        <input
          bind:value={form.filter}
          type="text"
          name="filter"
          placeholder="<args>"
        />
        <span class="help" title="Object filtering.">?</span>
      </label>
    </details>

    <details open class="details">
      <summary>Point to</summary>
      <label title="-o, --origin <name>">
        Origin Name
        <input
          bind:value={form.origin}
          type="text"
          name="origin"
          placeholder="<name>"
        />
        <span
          class="help"
          title="Use <name> instead of 'origin' to track upstream.">?</span
        >
      </label>

      <label title="-b, --branch <branch>">
        Branch Name
        <input
          bind:value={form.branch}
          type="text"
          name="branch"
          placeholder="<branch>"
        />
        <span
          class="help"
          title="Checkout <branch> instead of the remote's HEAD.">?</span
        >
      </label>
    </details>

    <details open class="details">
      <summary>General</summary>
      <label title="--ref-format <format>">
        Ref Format
        <select bind:value={form.refFormat} name="ref-format">
          <option value="files">files</option>
          <option value="reftable">reftable</option>
        </select>
        <span class="help" title="Specify the reference format to use.">?</span>
      </label><br />
    </details>
    <button type="submit" class="form-row" style="margin-top: 20px;">
      Generate Command
    </button>
  </form>
  <pre id="command-output">{commandOutput}</pre>

  <details>
    <summary>Options</summary>
    <label title="-v, --verbose">
      Verbose
      <input bind:checked={form.verbose} type="checkbox" name="verbose" />
      <span class="help" title="Be more verbose.">?</span>
    </label><br />

    <label title="-q, --quiet">
      Quiet
      <input bind:checked={form.quiet} type="checkbox" name="quiet" />
      <span class="help" title="Be more quiet.">?</span>
    </label><br />

    <label title="--progress">
      Progress
      <input bind:checked={form.progress} type="checkbox" name="progress" />
      <span class="help" title="Force progress reporting.">?</span>
    </label><br />

    <label title="-j, --jobs <n>">
      Jobs
      <input bind:value={form.jobs} type="text" name="jobs" placeholder="<n>" />
      <span class="help" title="Number of submodules cloned in parallel."
        >?</span
      >
    </label><br />

    <label title="--template <template-directory>">
      Template Directory
      <input
        bind:value={form.template}
        type="text"
        name="template"
        placeholder="<template-directory>"
      />
      <span class="help" title="Directory from which templates will be used."
        >?</span
      >
    </label><br />

    <label title="--dissociate">
      Dissociate
      <input bind:checked={form.dissociate} type="checkbox" name="dissociate" />
      <span class="help" title="Use --reference only while cloning.">?</span>
    </label><br />

    <label title="-u, --upload-pack <path>">
      Upload Pack
      <input
        bind:value={form.uploadPack}
        type="text"
        name="upload-pack"
        placeholder="<path>"
      />
      <span class="help" title="Path to git-upload-pack on the remote.">?</span>
    </label><br />

    <label title="--single-branch">
      Single Branch
      <input
        bind:checked={form.singleBranch}
        type="checkbox"
        name="single-branch"
      />
      <span class="help" title="Clone only one branch, HEAD or --branch."
        >?</span
      >
    </label><br />

    <label title="--no-tags">
      No Tags
      <input bind:checked={form.noTags} type="checkbox" name="no-tags" />
      <span
        class="help"
        title="Don't clone any tags, and make later fetches not to follow them."
        >?</span
      >
    </label><br />

    <label title="--separate-git-dir <gitdir>">
      Separate Git Directory
      <input
        bind:value={form.separateGitDir}
        type="text"
        name="separate-git-dir"
        placeholder="<gitdir>"
      />
      <span class="help" title="Separate git dir from working tree.">?</span>
    </label><br />

    <label title="-c, --config <key=value>">
      Config
      <input
        bind:value={form.config}
        type="text"
        name="config"
        placeholder="<key=value>"
      />
      <span class="help" title="Set config inside the new repository.">?</span>
    </label><br />

    <label title="--server-option <server-specific>">
      Server Option
      <input
        bind:value={form.serverOption}
        type="text"
        name="server-option"
        placeholder="<server-specific>"
      />
      <span class="help" title="Option to transmit.">?</span>
    </label><br />

    <label title="-4, --ipv4">
      IPv4
      <input bind:checked={form.ipv4} type="checkbox" name="ipv4" />
      <span class="help" title="Use IPv4 addresses only.">?</span>
    </label><br />

    <label title="-6, --ipv6">
      IPv6
      <input bind:checked={form.ipv6} type="checkbox" name="ipv6" />
      <span class="help" title="Use IPv6 addresses only.">?</span>
    </label><br />

    <label title="--remote-submodules">
      Remote Submodules
      <input
        bind:checked={form.remoteSubmodules}
        type="checkbox"
        name="remote-submodules"
      />
      <span
        class="help"
        title="Any cloned submodules will use their remote-tracking branch."
        >?</span
      >
    </label><br />
  </details>
</div>

<style>
  .container {
    /* Add your CSS styles here */
  }

  .back-link {
    text-decoration: none;
    color: blue;
  }

  .help {
    cursor: help;
    font-weight: bold;
  }

  .form-row {
    display: flex;
    align-items: center;
    gap: 10px;
    /* Adjust the gap between elements within the same row */
    margin-bottom: 0px;
    /* Add space between rows */
  }

  .form-row:last-child {
    justify-content: flex-start;
    /* Align the submit button to the start */
  }

  .form-row button {
    margin: 0;
  }

  .details {
    margin-top: 20px;
  }
</style>
