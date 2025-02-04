<script setup>
import { computed, onMounted, ref } from 'vue';
// import FolderExplorerButton from './FolderExplorerButton.vue';

const commandOutput = ref('');
const form = ref({
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
  jobs: '',
  template: '',
  reference: '',
  dissociate: false,
  origin: '',
  branch: '',
  uploadPack: '',
  depth: '',
  shallowSince: '',
  shallowExclude: '',
  singleBranch: false,
  noTags: false,
  shallowSubmodules: false,
  separateGitDir: '',
  refFormat: 'files',
  config: '',
  serverOption: '',
  ipv4: false,
  ipv6: false,
  filter: '',
  alsoFilterSubmodules: false,
  remoteSubmodules: false,
  sparse: false,
  bundleUri: '',
  repo: '',
  dir: ''
});

onMounted(() => {

});

const sortedItems = computed(() => {
  // getRepoName();
});

const getRepoName = (url) => {
  const parts = url.split('/');
  const repoWithGit = parts[parts.length - 1];
  const repoName1 = repoWithGit.replace('.git', '');

  console.log(repoName1); // Output: myrepo

  const regex = /([^/]+)\.git$/;
  const match = url.match(regex);

  if (match) {
    const repoName2 = match[1];
    console.log(repoName2); // Output: myrepo
  } else {
    console.log("No match found");
  }

  return repoName1;//TODO: Decide which method we want to use and remove the unneccessery code
}

const openFolderExplorer = async () => {
  try {
    const selectedFolder = await open({
      directory: true,
      multiple: false,
      title: 'Select a Folder',
    });

    if (selectedFolder) {
      console.log('Selected folder:', selectedFolder);
      // You can handle the selected folder path here
    }
  } catch (error) {
    console.error('Error opening folder explorer:', error);
  }
};

const handleRepoInput = (event) => {
  const value = event.target.value;
  if (value.endsWith('.git')) {
    const result = getRepoName(value);
    // if (!form.value.repo.includes(result)) {
    form.value.dir = form.value.dir + result;
    // }
  }
};

const generateCommand = () => {
  let command = 'git clone';

  if (form.value.verbose) command += ' --verbose';
  if (form.value.quiet) command += ' --quiet';
  if (form.value.progress) command += ' --progress';
  if (form.value.rejectShallow) command += ' --reject-shallow';
  if (form.value.noCheckout) command += ' --no-checkout';
  if (form.value.bare) command += ' --bare';
  if (form.value.mirror) command += ' --mirror';
  if (form.value.local) command += ' --local';
  if (form.value.noHardlinks) command += ' --no-hardlinks';
  if (form.value.shared) command += ' --shared';
  if (form.value.recurseSubmodules) command += ' --recurse-submodules';

  if (form.value.jobs) command += ` --jobs=${form.value.jobs}`;
  if (form.value.template) command += ` --template=${form.value.template}`;
  if (form.value.reference) command += ` --reference=${form.value.reference}`;
  if (form.value.dissociate) command += ' --dissociate';
  if (form.value.origin) command += ` --origin=${form.value.origin}`;
  if (form.value.branch) command += ` --branch=${form.value.branch}`;
  if (form.value.uploadPack) command += ` --upload-pack=${form.value.uploadPack}`;
  if (form.value.depth) command += ` --depth=${form.value.depth}`;
  if (form.value.shallowSince) command += ` --shallow-since=${form.value.shallowSince}`;
  if (form.value.shallowExclude) command += ` --shallow-exclude=${form.value.shallowExclude}`;
  if (form.value.singleBranch) command += ' --single-branch';
  if (form.value.noTags) command += ' --no-tags';
  if (form.value.shallowSubmodules) command += ' --shallow-submodules';
  if (form.value.separateGitDir) command += ` --separate-git-dir=${form.value.separateGitDir}`;
  if (form.value.refFormat) command += ` --ref-format=${form.value.refFormat}`;
  if (form.value.config) command += ` --config=${form.value.config}`;
  if (form.value.serverOption) command += ` --server-option=${form.value.serverOption}`;
  if (form.value.ipv4) command += ' --ipv4';
  if (form.value.ipv6) command += ' --ipv6';
  if (form.value.filter) command += ` --filter=${form.value.filter}`;
  if (form.value.alsoFilterSubmodules) command += ' --also-filter-submodules';
  if (form.value.remoteSubmodules) command += ' --remote-submodules';
  if (form.value.sparse) command += ' --sparse';
  if (form.value.bundleUri) command += ` --bundle-uri=${form.value.bundleUri}`;
  if (form.value.repo) command += ` ${form.value.repo}`;
  if (form.value.dir) command += ` ${form.value.dir}`;

  commandOutput.value = command;
};
</script>

<template>
  <div class="container">
    <router-link to="/" class="back-link">
      Back to Index
    </router-link>
    <h1>Git Clone Commands</h1>

    <form id="git-clone-form" @submit.prevent="generateCommand">
      <div class="form-row">
        <label title="<repo>">
          Repository
          <input v-model="form.repo" type="text" name="repo" placeholder="<repo>" @input="handleRepoInput">
          <span class="help" title="The repository to clone.">?</span>
        </label>
        <button @click="openFolderExplorer" class="explorer-button" type="button">
          Browse
        </button>
        <!-- <FolderExplorerButton display-text="Browse" type="button" /> -->
      </div>

      <div class="form-row">
        <label title="<dir>">
          Directory
          <input v-model="form.dir" type="text" name="dir" placeholder="<dir>">
          <span class="help" title="The directory to clone into.">?</span>
        </label>
        <button @click="openFolderExplorer" class="explorer-button" type="button">
          Browse
        </button>
        <!-- <FolderExplorerButton display-text="Browse" type="button" /> -->
      </div>

      <details open="open" class="details">
        <summary>Shallow Clone</summary>
        <label title="--depth <depth>">
          Depth
          <input v-model="form.depth" type="number" name="depth" placeholder="<depth>" :disabled="!!form.bundleUri">
          <span class="help" title="Create a shallow clone of that depth.">?</span>
        </label>

        <label title="--shallow-submodules">
          Shallow Submodules
          <input v-model="form.shallowSubmodules" type="checkbox" name="shallow-submodules" />
          <span class="help" title="All submodules which are cloned will be shallow with a depth of 1.">?</span>
        </label>

        <label title="--reject-shallow">
          Reject Shallow
          <input v-model="form.rejectShallow" type="checkbox" name="reject-shallow">
          <span class="help" title="Don't clone shallow repository.">?</span>
        </label>

        <label title="--shallow-since <time>">
          Shallow Since
          <input v-model="form.shallowSince" type="date" name="shallow-since" placeholder="<time>"
            :disabled="!!form.bundleUri">
          <span class="help" title="Create a shallow clone since a specific time.">?</span>
        </label>

        <label title="--shallow-exclude <revision>">
          Shallow Exclude
          <input v-model="form.shallowExclude" type="text" name="shallow-exclude" placeholder="<revision>"
            :disabled="!!form.bundleUri" />
          <span class="help" title="Deepen history of shallow clone, excluding rev.">?</span>
        </label>

      </details>

      <details open="open" class="details">
        <summary>Checkouts</summary>
        <label title="--bundle-uri <uri>">
          Bundle URI
          <input v-model="form.bundleUri" type="text" name="bundle-uri" placeholder="<uri>"
            :disabled="form.depth || form.shallowExclude || !!form.shallowSince">
          <span class="help" title="A URI for downloading bundles before fetching from origin remote.">?</span>
        </label>

        <label title="-n, --no-checkout">
          No Checkout
          <input v-model="form.noCheckout" type="checkbox" name="no-checkout">
          <span class="help" title="Don't create a checkout.">?</span>
        </label>

        <label title="--bare">
          Bare Repository
          <input v-model="form.bare" type="checkbox" name="bare">
          <span class="help" title="Create a bare repository.">?</span>
        </label>

        <label title="--mirror">
          Mirror Repository
          <input v-model="form.mirror" type="checkbox" name="mirror">
          <span class="help" title="Create a mirror repository (implies --bare).">?</span>
        </label>

        <label title="--sparse">
          Sparse
          <input v-model="form.sparse" type="checkbox" name="sparse">
          <span class="help" title="Initialize sparse-checkout file to include only files at root.">?</span>
        </label>
      </details>

      <details open="open" class="details">
        <summary>Locals</summary>
        <label title="-l, --local">
          Local
          <input v-model="form.local" type="checkbox" name="local">
          <span class="help" title="To clone from a local repository.">?</span>
        </label>

        <label title="--no-hardlinks">
          No Hardlinks
          <input v-model="form.noHardlinks" type="checkbox" name="no-hardlinks">
          <span class="help" title="Don't use local hardlinks, always copy.">?</span>
        </label>

        <label title="-s, --shared">
          Shared
          <input v-model="form.shared" type="checkbox" name="shared">
          <span class="help" title="Setup as shared repository.">?</span>
        </label>

        <label title="--reference <repo>">
          Reference Repository
          <input v-model="form.reference" type="text" name="reference" placeholder="<repo>">
          <span class="help" title="Reference repository.">?</span>
        </label>
      </details>

      <details open="open" class="details">
        <summary>Post Clone</summary>
        <label title="--also-filter-submodules">
          Also Filter Submodules
          <input v-model="form.alsoFilterSubmodules" type="checkbox" name="also-filter-submodules">
          <span class="help" title="Apply partial clone filters to submodules.">?</span>
        </label>
        <label title="--recurse-submodules">
          Recurse Submodules
          <input v-model="form.recurseSubmodules" type="checkbox" name="recurse-submodules">
          <span class="help" title="Initialize submodules in the clone.">?</span>
        </label>
        <label title="--filter <args>">
          Filter
          <input v-model="form.filter" type="text" name="filter" placeholder="<args>">
          <span class="help" title="Object filtering.">?</span>
        </label>
      </details>

      <details open="open" class="details">
        <summary>Point to</summary>
        <label title="-o, --origin <name>">
          Origin Name
          <input v-model="form.origin" type="text" name="origin" placeholder="<name>">
          <span class="help" title="Use <name> instead of 'origin' to track upstream.">?</span>
        </label>

        <label title="-b, --branch <branch>">
          Branch Name
          <input v-model="form.branch" type="text" name="branch" placeholder="<branch>">
          <span class="help" title="Checkout <branch> instead of the remote's HEAD.">?</span>
        </label>
      </details>
      <button type="submit" class="form-row" style="margin-top: 20px;">
        Generate Command
      </button>
    </form>
    <pre id="command-output">{{ commandOutput }}</pre>

    <details>
      <summary>Options</summary>
      <label title="-v, --verbose">
        Verbose
        <input v-model="form.verbose" type="checkbox" name="verbose">
        <span class="help" title="Be more verbose.">?</span>
      </label><br>

      <label title="-q, --quiet">
        Quiet
        <input v-model="form.quiet" type="checkbox" name="quiet">
        <span class="help" title="Be more quiet.">?</span>
      </label><br>

      <label title="--progress">
        Progress
        <input v-model="form.progress" type="checkbox" name="progress">
        <span class="help" title="Force progress reporting.">?</span>
      </label><br>


      <label title="-j, --jobs <n>">
        Jobs
        <input v-model="form.jobs" type="text" name="jobs" placeholder="<n>">
        <span class="help" title="Number of submodules cloned in parallel.">?</span>
      </label><br>

      <label title="--template <template-directory>">
        Template Directory
        <input v-model="form.template" type="text" name="template" placeholder="<template-directory>">
        <span class="help" title="Directory from which templates will be used.">?</span>
      </label><br>

      <label title="--dissociate">
        Dissociate
        <input v-model="form.dissociate" type="checkbox" name="dissociate">
        <span class="help" title="Use --reference only while cloning.">?</span>
      </label><br>



      <label title="-u, --upload-pack <path>">
        Upload Pack
        <input v-model="form.uploadPack" type="text" name="upload-pack" placeholder="<path>">
        <span class="help" title="Path to git-upload-pack on the remote.">?</span>
      </label><br>

      <label title="--single-branch">
        Single Branch
        <input v-model="form.singleBranch" type="checkbox" name="single-branch">
        <span class="help" title="Clone only one branch, HEAD or --branch.">?</span>
      </label><br>

      <label title="--no-tags">
        No Tags
        <input v-model="form.noTags" type="checkbox" name="no-tags">
        <span class="help" title="Don't clone any tags, and make later fetches not to follow them.">?</span>
      </label><br>

      <label title="--separate-git-dir <gitdir>">
        Separate Git Directory
        <input v-model="form.separateGitDir" type="text" name="separate-git-dir" placeholder="<gitdir>">
        <span class="help" title="Separate git dir from working tree.">?</span>
      </label><br>

      <label title="--ref-format <format>">
        Ref Format
        <select v-model="form.refFormat" name="ref-format">
          <option value="files">files</option>
          <option value="reftable">reftable</option>
        </select>
        <span class="help" title="Specify the reference format to use.">?</span>
      </label><br>

      <label title="-c, --config <key=value>">
        Config
        <input v-model="form.config" type="text" name="config" placeholder="<key=value>">
        <span class="help" title="Set config inside the new repository.">?</span>
      </label><br>

      <label title="--server-option <server-specific>">
        Server Option
        <input v-model="form.serverOption" type="text" name="server-option" placeholder="<server-specific>">
        <span class="help" title="Option to transmit.">?</span>
      </label><br>

      <label title="-4, --ipv4">
        IPv4
        <input v-model="form.ipv4" type="checkbox" name="ipv4">
        <span class="help" title="Use IPv4 addresses only.">?</span>
      </label><br>

      <label title="-6, --ipv6">
        IPv6
        <input v-model="form.ipv6" type="checkbox" name="ipv6">
        <span class="help" title="Use IPv6 addresses only.">?</span>
      </label><br>

      <label title="--remote-submodules">
        Remote Submodules
        <input v-model="form.remoteSubmodules" type="checkbox" name="remote-submodules">
        <span class="help" title="Any cloned submodules will use their remote-tracking branch.">?</span>
      </label><br>

    </details>
  </div>
</template>

<style scoped>
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