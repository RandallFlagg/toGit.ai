<template>
  <div class="container">
    <h1>Git Clone Commands</h1>
    <ul>
      <!-- existing list items -->
    </ul>
    <router-link
      to="/"
      class="back-link"
    >
      Back to Index
    </router-link>
  </div>
  <div class="container">
    <h1>Git Clone Commands</h1>
    <form
      id="git-clone-form"
      @submit.prevent="generateCommand"
    >
      <label title="-v, --verbose">
        Verbose
        <input
          v-model="form.verbose"
          type="checkbox"
          name="verbose"
        >
        <span
          class="help"
          title="Be more verbose."
        >?</span>
      </label><br>

      <label title="-q, --quiet">
        Quiet
        <input
          v-model="form.quiet"
          type="checkbox"
          name="quiet"
        >
        <span
          class="help"
          title="Be more quiet."
        >?</span>
      </label><br>

      <label title="--progress">
        Progress
        <input
          v-model="form.progress"
          type="checkbox"
          name="progress"
        >
        <span
          class="help"
          title="Force progress reporting."
        >?</span>
      </label><br>

      <label title="--reject-shallow">
        Reject Shallow
        <input
          v-model="form.rejectShallow"
          type="checkbox"
          name="reject-shallow"
        >
        <span
          class="help"
          title="Don't clone shallow repository."
        >?</span>
      </label><br>

      <label title="-n, --no-checkout">
        No Checkout
        <input
          v-model="form.noCheckout"
          type="checkbox"
          name="no-checkout"
        >
        <span
          class="help"
          title="Don't create a checkout."
        >?</span>
      </label><br>

      <label title="--bare">
        Bare Repository
        <input
          v-model="form.bare"
          type="checkbox"
          name="bare"
        >
        <span
          class="help"
          title="Create a bare repository."
        >?</span>
      </label><br>

      <label title="--mirror">
        Mirror Repository
        <input
          v-model="form.mirror"
          type="checkbox"
          name="mirror"
        >
        <span
          class="help"
          title="Create a mirror repository (implies --bare)."
        >?</span>
      </label><br>

      <label title="-l, --local">
        Local
        <input
          v-model="form.local"
          type="checkbox"
          name="local"
        >
        <span
          class="help"
          title="To clone from a local repository."
        >?</span>
      </label><br>

      <label title="--no-hardlinks">
        No Hardlinks
        <input
          v-model="form.noHardlinks"
          type="checkbox"
          name="no-hardlinks"
        >
        <span
          class="help"
          title="Don't use local hardlinks, always copy."
        >?</span>
      </label><br>

      <label title="-s, --shared">
        Shared
        <input
          v-model="form.shared"
          type="checkbox"
          name="shared"
        >
        <span
          class="help"
          title="Setup as shared repository."
        >?</span>
      </label><br>

      <label title="--recurse-submodules">
        Recurse Submodules
        <input
          v-model="form.recurseSubmodules"
          type="checkbox"
          name="recurse-submodules"
        >
        <span
          class="help"
          title="Initialize submodules in the clone."
        >?</span>
      </label><br>

      <label title="-j, --jobs <n>">
        Jobs
        <input
          v-model="form.jobs"
          type="text"
          name="jobs"
          placeholder="<n>"
        >
        <span
          class="help"
          title="Number of submodules cloned in parallel."
        >?</span>
      </label><br>

      <label title="--template <template-directory>">
        Template Directory
        <input
          v-model="form.template"
          type="text"
          name="template"
          placeholder="<template-directory>"
        >
        <span
          class="help"
          title="Directory from which templates will be used."
        >?</span>
      </label><br>

      <label title="--reference <repo>">
        Reference Repository
        <input
          v-model="form.reference"
          type="text"
          name="reference"
          placeholder="<repo>"
        >
        <span
          class="help"
          title="Reference repository."
        >?</span>
      </label><br>

      <label title="--dissociate">
        Dissociate
        <input
          v-model="form.dissociate"
          type="checkbox"
          name="dissociate"
        >
        <span
          class="help"
          title="Use --reference only while cloning."
        >?</span>
      </label><br>

      <label title="-o, --origin <name>">
        Origin Name
        <input
          v-model="form.origin"
          type="text"
          name="origin"
          placeholder="<name>"
        >
        <span
          class="help"
          title="Use <name> instead of 'origin' to track upstream."
        >?</span>
      </label><br>

      <label title="-b, --branch <branch>">
        Branch Name
        <input
          v-model="form.branch"
          type="text"
          name="branch"
          placeholder="<branch>"
        >
        <span
          class="help"
          title="Checkout <branch> instead of the remote's HEAD."
        >?</span>
      </label><br>

      <label title="-u, --upload-pack <path>">
        Upload Pack
        <input
          v-model="form.uploadPack"
          type="text"
          name="upload-pack"
          placeholder="<path>"
        >
        <span
          class="help"
          title="Path to git-upload-pack on the remote."
        >?</span>
      </label><br>

      <label title="--depth <depth>">
        Depth
        <input
          v-model="form.depth"
          type="text"
          name="depth"
          placeholder="<depth>"
        >
        <span
          class="help"
          title="Create a shallow clone of that depth."
        >?</span>
      </label><br>

      <label title="--shallow-since <time>">
        Shallow Since
        <input
          v-model="form.shallowSince"
          type="text"
          name="shallow-since"
          placeholder="<time>"
        >
        <span
          class="help"
          title="Create a shallow clone since a specific time."
        >?</span>
      </label><br>

      <label title="--shallow-exclude <revision>">
        Shallow Exclude
        <input
          v-model="form.shallowExclude"
          type="text"
          name="shallow-exclude"
          placeholder="<revision>"
        >
        <span
          class="help"
          title="Deepen history of shallow clone, excluding rev."
        >?</span>
      </label><br>

      <label title="--single-branch">
        Single Branch
        <input
          v-model="form.singleBranch"
          type="checkbox"
          name="single-branch"
        >
        <span
          class="help"
          title="Clone only one branch, HEAD or --branch."
        >?</span>
      </label><br>

      <label title="--no-tags">
        No Tags
        <input
          v-model="form.noTags"
          type="checkbox"
          name="no-tags"
        >
        <span
          class="help"
          title="Don't clone any tags, and make later fetches not to follow them."
        >?</span>
      </label><br>

      <label title="--shallow-submodules">
        Shallow Submodules
        <input
          v-model="form.shallowSubmodules"
          type="checkbox"
          name="shallow-submodules"
        >
        <span
          class="help"
          title="Any cloned submodules will be shallow."
        >?</span>
      </label><br>

      <label title="--separate-git-dir <gitdir>">
        Separate Git Directory
        <input
          v-model="form.separateGitDir"
          type="text"
          name="separate-git-dir"
          placeholder="<gitdir>"
        >
        <span
          class="help"
          title="Separate git dir from working tree."
        >?</span>
      </label><br>

      <label title="--ref-format <format>">
        Ref Format
        <select
          v-model="form.refFormat"
          name="ref-format"
        >
          <option value="files">files</option>
          <option value="reftable">reftable</option>
        </select>
        <span
          class="help"
          title="Specify the reference format to use."
        >?</span>
      </label><br>

      <label title="-c, --config <key=value>">
        Config
        <input
          v-model="form.config"
          type="text"
          name="config"
          placeholder="<key=value>"
        >
        <span
          class="help"
          title="Set config inside the new repository."
        >?</span>
      </label><br>

      <label title="--server-option <server-specific>">
        Server Option
        <input
          v-model="form.serverOption"
          type="text"
          name="server-option"
          placeholder="<server-specific>"
        >
        <span
          class="help"
          title="Option to transmit."
        >?</span>
      </label><br>

      <label title="-4, --ipv4">
        IPv4
        <input
          v-model="form.ipv4"
          type="checkbox"
          name="ipv4"
        >
        <span
          class="help"
          title="Use IPv4 addresses only."
        >?</span>
      </label><br>

      <label title="-6, --ipv6">
        IPv6
        <input
          v-model="form.ipv6"
          type="checkbox"
          name="ipv6"
        >
        <span
          class="help"
          title="Use IPv6 addresses only."
        >?</span>
      </label><br>

      <label title="--filter <args>">
        Filter
        <input
          v-model="form.filter"
          type="text"
          name="filter"
          placeholder="<args>"
        >
        <span
          class="help"
          title="Object filtering."
        >?</span>
      </label><br>

      <label title="--also-filter-submodules">
        Also Filter Submodules
        <input
          v-model="form.alsoFilterSubmodules"
          type="checkbox"
          name="also-filter-submodules"
        >
        <span
          class="help"
          title="Apply partial clone filters to submodules."
        >?</span>
      </label><br>

      <label title="--remote-submodules">
        Remote Submodules
        <input
          v-model="form.remoteSubmodules"
          type="checkbox"
          name="remote-submodules"
        >
        <span
          class="help"
          title="Any cloned submodules will use their remote-tracking branch."
        >?</span>
      </label><br>

      <label title="--sparse">
        Sparse
        <input
          v-model="form.sparse"
          type="checkbox"
          name="sparse"
        >
        <span
          class="help"
          title="Initialize sparse-checkout file to include only files at root."
        >?</span>
      </label><br>

      <label title="--bundle-uri <uri>">
        Bundle URI
        <input
          v-model="form.bundleUri"
          type="text"
          name="bundle-uri"
          placeholder="<uri>"
        >
        <span
          class="help"
          title="A URI for downloading bundles before fetching from origin remote."
        >?</span>
      </label><br>

      <label title="<repo>">
        Repository
        <input
          v-model="form.repo"
          type="text"
          name="repo"
          placeholder="<repo>"
        >
        <span
          class="help"
          title="The repository to clone."
        >?</span>
      </label><br>

      <label title="<dir>">
        Directory
        <input
          v-model="form.dir"
          type="text"
          name="dir"
          placeholder="<dir>"
        >
        <span
          class="help"
          title="The directory to clone into."
        >?</span>
      </label><br>

      <button type="submit">
        Generate Command
      </button>
    </form>
    <pre id="command-output">{{ commandOutput }}</pre>
  </div>
</template>

<script>
export default {
    name: 'ClonePage',
    data() {
        return {
            form: {
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
            },
            commandOutput: ''
        };
    },
    methods: {
        generateCommand() {
            let command = 'git clone';

            if (this.form.verbose) command += ' --verbose';
            if (this.form.quiet) command += ' --quiet';
            if (this.form.progress) command += ' --progress';
            if (this.form.rejectShallow) command += ' --reject-shallow';
            if (this.form.noCheckout) command += ' --no-checkout';
            if (this.form.bare) command += ' --bare';
            if (this.form.mirror) command += ' --mirror';
            if (this.form.local) command += ' --local';
            if (this.form.noHardlinks) command += ' --no-hardlinks';
            if (this.form.shared) command += ' --shared';
            if (this.form.recurseSubmodules) command += ' --recurse-submodules';

            if (this.form.jobs) command += ` --jobs=${this.form.jobs}`;
            if (this.form.template) command += ` --template=${this.form.template}`;
            if (this.form.reference) command += ` --reference=${this.form.reference}`;
            if (this.form.dissociate) command += ' --dissociate';
            if (this.form.origin) command += ` --origin=${this.form.origin}`;
            if (this.form.branch) command += ` --branch=${this.form.branch}`;
            if (this.form.uploadPack) command += ` --upload-pack=${this.form.uploadPack}`;
            if (this.form.depth) command += ` --depth=${this.form.depth}`;
            if (this.form.shallowSince) command += ` --shallow-since=${this.form.shallowSince}`;
            if (this.form.shallowExclude) command += ` --shallow-exclude=${this.form.shallowExclude}`;
            if (this.form.singleBranch) command += ' --single-branch';
            if (this.form.noTags) command += ' --no-tags';
            if (this.form.shallowSubmodules) command += ' --shallow-submodules';
            if (this.form.separateGitDir) command += ` --separate-git-dir=${this.form.separateGitDir}`;
            command += ` --ref-format=${this.form.refFormat}`;
            if (this.form.config) command += ` --config=${this.form.config}`;
            if (this.form.serverOption) command += ` --server-option=${this.form.serverOption}`;
            if (this.form.ipv4) command += ' --ipv4';
            if (this.form.ipv6) command += ' --ipv6';
            if (this.form.filter) command += ` --filter=${this.form.filter}`;
            if (this.form.alsoFilterSubmodules) command += ' --also-filter-submodules';
            if (this.form.remoteSubmodules) command += ' --remote-submodules';
            if (this.form.sparse) command += ' --sparse';
            if (this.form.bundleUri) command += ` --bundle-uri=${this.form.bundleUri}`;
            if (this.form.repo) command += ` ${this.form.repo}`;
            if (this.form.dir) command += ` ${this.form.dir}`;

            this.commandOutput = command;
        }
    }
};
</script>

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
</style>
