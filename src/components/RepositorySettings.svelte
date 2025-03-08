<script>
    import { fork_stub_data } from "./stub.js";
    import { onMount } from "svelte";

    let repoDetails;
    let error;
    let forks = [];
    let selectedRemote = "origin";
    let selectedBranch = [];

    const selectRemote = (remote) => {
                // selectedRemote = repoDetails.remotes[0];
                // selectedBranch = selectedRemote.remote_branches[0];
                selectedRemote = remote;
                console.log("selectedRemote: ", selectedRemote);
                selectedBranch = remote.remote_branches[0];
                console.log("selectedBranch: ",selectedBranch);
    };

    const selectBranch = (branch) => {
        selectedBranch = branch;//.split(" ");
    };

    // Function to check if the URL starts with http, https, or ssh
    const testShowOpenButton = (url) => {
        return /^(https?|ssh):/.test(url);
    };

    const getPullStatus = (pullBranch) => {
        if (pullBranch.includes("rebases onto")) {
            return "Rebases onto remote branch";
        } else if (pullBranch.includes("merges with")) {
            return "Merges with remote branch";
        } else {
            return "Unknown status";
        }
    };

    const getPushStatus = (pushBranch) => {
        if (pushBranch.includes("up to date")) {
            return "up-to-date";
        } else if (
            pushBranch.includes("commits ahead") &&
            pushBranch.includes("commits behind")
        ) {
            return "divergent";
        } else if (pushBranch.includes("commits ahead")) {
            return "remote out of date";
        } else if (pushBranch.includes("commits behind")) {
            return "local out of date";
        } else {
            return "Unknown status";
        }
    };

    onMount(async () => {
        const fetchRepoDetails = async () => {
            let result;
            try {
                console.log("A");
                result =
                    await window.__TAURI__.core.invoke("get_repo_details");
                console.log(result);
            } catch (err) {
                console.log("B");
                error = err;
                result = null;
            }

            return result;
        };

        function parseGitUrl(url) {
            let owner, repo;

            if (url.startsWith("git@")) {
                // SSH URL
                const match = url.match(/git@github\.com:(.+?)\/(.+?)\.git/);
                if (match) {
                    owner = match[1];
                    repo = match[2];
                }
            } else if (url.startsWith("https://")) {
                // HTTPS URL
                const match = url.match(
                    /https:\/\/github\.com\/(.+?)\/(.+?)\.git/,
                );
                if (match) {
                    owner = match[1];
                    repo = match[2];
                }
            }

            return { owner, repo };
        }

        async function fetchForkSource(owner, repo) {
            const response = await fetch(
                `https://api.github.com/repos/${owner}/${repo}`,
            );
            const data = await response.json();
            return data.source
                ? { owner: data.source.owner.login, repo: data.source.name }
                : null;
        }

        // Fetch repo details when the component mounts
        repoDetails = await fetchRepoDetails();

        selectRemote(repoDetails.remotes[0]);

        for (const remote of repoDetails.remotes) {
            const info = parseGitUrl(remote.fetch_url);
            console.log(info);
            if (false) {
                //TODO: Need to autenticate to get higher API calls
                //Inorder to save aPI calls this is disabled and I am using a mock instead
                const source = await fetchForkSource(info.owner, info.repo);
                if (source) {
                    console.log(`Original repo owner: ${source.owner}`);
                    console.log(`Original repo name: ${source.repo}`);
                } else {
                    console.log(
                        "This repository is not a fork or does not have a source. Fetching...",
                    );
                    const forkResponse = await fetch(
                        `https://api.github.com/repos/${info.owner}/${info.repo}/forks`,
                    );

                    //forks.push(await ...(forkResponse.json()));
                    const forks_data = await forkResponse.json();
                    forks = [...forks_data];
                    console.log("Done.");
                }
            } else {
                const data = fork_stub_data;
                forks = [...forks, data];
            }
        }

        console.log(forks.length);
        console.log(forks);
    });

    export let accessPermissions = {
        collaborators: ["Alice", "Bob", "Charlie"],
        teams: ["Developers", "Admins"],
        branchProtection: ["main", "develop"],
    };

    export let branches = {
        defaultBranch: "main",
        otherBranches: ["develop", "feature-x"],
    };

    export let issuesAndPRs = {
        issueSettings: ["bug", "enhancement", "question"],
        prSettings: ["require reviews", "use templates"],
    };

    export let webhooksAndIntegrations = {
        webhooks: ["Webhook1", "Webhook2"],
        integrations: ["CI/CD", "Project Management"],
    };

    export let gitHooks = {
        clientSide: ["pre-commit", "commit-msg", "pre-push"],
        serverSide: ["pre-receive", "post-receive"],
    };

    export let gitConfig = {
        globalConfig: "Global config content here",
        localConfig: "Local config content here",
    };

    export let documentation = {
        readme: "README content here",
        contributing: "CONTRIBUTING guidelines here",
        codeOfConduct: "Code of conduct content here",
        license: "LICENSE content here",
    };

    export let security = {
        securityPolicy: "Security policy content here",
        dependabotAlerts: ["Alert1", "Alert2"],
    };

    export let environmentVariables = ["VAR1=value1", "VAR2=value2"];

    export let ciCdSettings = {
        buildConfig: "Build configuration here",
        deploymentConfig: "Deployment configuration here",
    };

    export let backupAndRestore = {
        backupProcedures: "Backup procedures here",
        restoreProcedures: "Restore procedures here",
    };

    export let contactInfo = {
        maintainers: ["Maintainer1", "Maintainer2"],
        supportChannels: ["email@example.com", "Slack Channel"],
    };

    export let misc = {
        customScripts: ["Script1", "Script2"],
        hooksAndAutomation: ["Hook1", "Automation Tool"],
    };
</script>

<main class="container">
    {#if error}
        <p>Error: {error.message}</p>
    {:else if repoDetails}
        <h1 class="header">Git Repository Information</h1>
        <section class="section">
            <h2 class="section-title">Remotes</h2>
            <!-- <div class="remote-list">
                {#each repoDetails.remotes as remote}
                    <div
                        class="remote-item {selectedRemote === remote
                            ? 'active'
                            : ''}"
                        on:click={() => selectRemote(remote)}
                    >
                        <h3 class="remote-title">Remote: {remote.name}</h3>
                    </div>
                {/each}
            </div> -->
            <nav aria-label="Breadcrumb">
                <div class="breadcrumb">
                    {#each repoDetails.remotes as remote}
                        <button
                            class="breadcrumb-item {selectedRemote === remote
                                ? 'active'
                                : ''}"
                            on:click={() => selectRemote(remote)}
                        >
                            {remote.name}
                        </button>
                    {/each}
                </div>
            </nav>

            <div class="remote">
                <!-- <h3 class="remote-title">
                    Remote: {selectedRemote.name}
                </h3> -->
                <div class="paragraph">
                    <label for="fetchUrl">Fetch URL</label>
                    <input
                        id="fetchUrl"
                        class="textbox"
                        type="text"
                        bind:value={selectedRemote.fetch_url}
                    />
                    <button
                        class="button {testShowOpenButton(
                            selectedRemote.fetch_url,
                        )
                            ? 'show'
                            : ''}"
                        on:click={() =>
                            window.open(selectedRemote.fetch_url, "_blank")}
                    >
                        Open URL
                    </button>
                </div>
                <div class="paragraph">
                    <label for="pushUrl">Push URL</label>
                    <input
                        id="pushUrl"
                        class="textbox"
                        type="text"
                        bind:value={selectedRemote.push_url}
                    />
                    <button
                        class="button {testShowOpenButton(
                            selectedRemote.push_url,
                        )
                            ? 'show'
                            : ''}"
                        on:click={() =>
                            window.open(selectedRemote.push_url, "_blank")}
                    >
                        Open URL
                    </button>
                </div>
                <p class="paragraph">
                    Fetch Refspecs: - This should be taken from a different
                    place. This should b removed. leaving here only until dev is
                    done.
                    {#each selectedRemote.fetch_refspecs as fetch_refspec}
                        <li>{fetch_refspec}</li>
                    {/each}
                </p>

                <h4 class="branch-title">
                    Local Branches Configured for Git Pull
                </h4>
                <ul class="list">
                    {#each selectedRemote.local_branches_configured_for_git_pull as pullBranch}
                        <li class="list-item">
                            <span class="branch-name">
                                {pullBranch.split(" (")[0]}
                            </span>
                            <span class="branch-status">
                                {getPullStatus(pullBranch)}
                            </span>
                        </li>
                    {/each}
                </ul>

                <h4 class="branch-title">
                    Local Branches Configured for Git Push
                </h4>
                <ul class="list">
                    {#each selectedRemote.local_branches_configured_for_git_push as pushBranch}
                        <li class="list-item">
                            <span class="branch-name">
                                {pushBranch.split(" (")[0]}
                            </span>
                            <span class="branch-status">
                                {#if pushBranch.includes("up to date")}
                                    <span class="branch-status up-to-date"
                                        >Up to date</span
                                    >
                                {:else if pushBranch.includes("commits ahead") && pushBranch.includes("commits behind")}
                                    <span class="arrow-up">⬆️</span>
                                    {pushBranch.match(/(\d+) commits ahead/)[1]}
                                    commits ahead,
                                    <span class="arrow-down">⬇️</span>
                                    {pushBranch.match(
                                        /(\d+) commits behind/,
                                    )[1]} commits behind (divergent)
                                {:else if pushBranch.includes("commits ahead")}
                                    <span class="arrow-up">⬆️</span>
                                    {pushBranch.match(/(\d+) commits ahead/)[1]}
                                    commits ahead (remote out of date)
                                {:else if pushBranch.includes("commits behind")}
                                    <span class="arrow-down">⬇️</span>
                                    {pushBranch.match(
                                        /(\d+) commits behind/,
                                    )[1]} commits behind (local out of date)
                                {:else}
                                    Unknown status
                                {/if}
                            </span>
                        </li>
                    {/each}
                </ul>

                <h4 class="branch-title">Remote Branches</h4>
                <div class="branch-list">
                    {#each selectedRemote.remote_branches as branch}
                        <div
                            class="branch-item {selectedBranch === branch
                                ? 'active'
                                : ''}"
                            on:click={() => selectBranch(branch)}
                        >
                            <li class="list-item">
                                <span class="branch-name">{branch.name}</span>
                            </li>
                        </div>

                        {#if selectedBranch === branch}
                            <h4 class="branch-title">Branch Information</h4>
                            <div class="branch-status">
                                Last updated {branch.commit_age}
                                {#if branch.tracking_branch}
                                    , tracked by {branch.tracking_branch}
                                {/if}
                                <br />
                                Last commit: {branch.last_commit_message}
                                <br />
                                Author: {branch.last_commit_author}
                                <br />
                                Merged: {branch.is_merged}
                                <br />
                                Tags: {#each branch.tags as tag}
                                    {tag.name} ({tag.message || "No message"})
                                {/each}
                            </div>
                        {/if}
                    {/each}
                </div>
            </div>
        </section>

        <section class="section">
            <h2 class="section-title">Tags</h2>
            <ul class="list">
                {#each repoDetails.tags as tag}
                    <li class="list-item">{tag}</li>
                {/each}
            </ul>
        </section>

        <section class="section">
            <h2 class="section-title">Default Branch</h2>
            <p class="paragraph">Name: {repoDetails.default_branch_name}</p>
            <p class="paragraph">
                Full Name: {repoDetails.default_full_branch_name}
            </p>
            <p class="paragraph">
                Push Remote: {repoDetails.default_push_remote}
            </p>
            <p class="paragraph">
                Pull Remote: {repoDetails.default_pull_remote}
            </p>
        </section>

        <section class="section">
            <h2 class="section-title">Miscellaneous</h2>
            <p class="paragraph">
                Contributors: {repoDetails.contributors.length
                    ? repoDetails.contributors.join(", ")
                    : "None"}
            </p>
            <p class="paragraph">Forks: {repoDetails.forks}</p>
            <p class="paragraph">Stars: {repoDetails.stars}</p>
            <p class="paragraph">Language: {repoDetails.language}</p>
            <p class="paragraph">Size: {repoDetails.size}</p>
            <p class="paragraph">Created At: {repoDetails.created_at}</p>
            <p class="paragraph">Updated At: {repoDetails.updated_at}</p>
        </section>

        <div class="section">
            <h2 class="title">Repository Details</h2>
            <ul>
                <li>
                    <pre>{JSON.stringify(repoDetails, null, 2)}</pre>
                </li>
                <li><strong>Name:</strong> {repoDetails.name}</li>
                <li><strong>Description:</strong> {repoDetails.description}</li>
                <li>
                    <strong>Remotes:</strong>
                    {#each repoDetails.remotes as remote}
                        <strong>Name:</strong>
                        {remote.name} <br />
                        <strong>URL:</strong>
                        {remote.url} <br />
                        <strong>Push URL:</strong>
                        {remote.push_url} <br />
                        <strong>Fetch:</strong>
                        {remote.fetch} <br />
                    {/each}
                </li>
                <li>
                    <strong>URL:</strong>
                    <a href={repoDetails.url} target="_blank"
                        >{repoDetails.url}</a
                    >
                </li>
            </ul>
        </div>

        <h2 class="title">Forks of the Repository</h2>
        <ul class="fork-list">
            <li>
                <span>Array Length: {forks.length}</span>
            </li>
            {#each forks as fork}
                <li>
                    <span>Fork Size: {Object.keys(fork).length}</span>
                    <pre>{JSON.stringify(fork, null, 2)}</pre>
                </li>
            {/each}
            {#each forks as fork}
                <li class="fork-item">
                    <strong class="fork-name">{fork.full_name}</strong><br />
                    <a class="fork-link" href={fork.html_url} target="_blank"
                        >View on GitHub</a
                    >
                </li>
            {/each}
        </ul>

        <div class="section">
            <h2>Access Permissions</h2>
            <ul>
                <li>
                    <strong>Collaborators:</strong>
                    {#each accessPermissions.collaborators as collaborator}{collaborator},
                    {/each}
                </li>
                <li>
                    <strong>Teams:</strong>
                    {#each accessPermissions.teams as team}{team},
                    {/each}
                </li>
                <li>
                    <strong>Branch Protection:</strong>
                    {#each accessPermissions.branchProtection as branch}{branch},
                    {/each}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Branches</h2>
            <ul>
                <li>
                    <strong>Default Branch:</strong>
                    {branches.defaultBranch}
                </li>
                <li>
                    <strong>Other Branches:</strong>
                    {#each branches.otherBranches as branch}{branch},
                    {/each}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Issues and Pull Requests</h2>
            <ul>
                <li>
                    <strong>Issue Settings:</strong>
                    {#each issuesAndPRs.issueSettings as setting}{setting},
                    {/each}
                </li>
                <li>
                    <strong>Pull Request Settings:</strong>
                    {#each issuesAndPRs.prSettings as setting}{setting},
                    {/each}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Webhooks and Integrations</h2>
            <ul>
                <li>
                    <strong>Webhooks:</strong>
                    {#each webhooksAndIntegrations.webhooks as webhook}{webhook},
                    {/each}
                </li>
                <li>
                    <strong>Integrations:</strong>
                    {#each webhooksAndIntegrations.integrations as integration}{integration},
                    {/each}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Git Hooks</h2>
            <ul>
                <li>
                    <strong>Client-Side Hooks:</strong>
                    {#each gitHooks.clientSide as hook}{hook},
                    {/each}
                </li>
                <li>
                    <strong>Server-Side Hooks:</strong>
                    {#each gitHooks.serverSide as hook}{hook},
                    {/each}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Git Configuration</h2>
            <ul>
                <li>
                    <strong>Global Configuration:</strong>
                    {gitConfig.globalConfig}
                </li>
                <li>
                    <strong>Local Configuration:</strong>
                    {gitConfig.localConfig}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Documentation</h2>
            <ul>
                <li><strong>README:</strong> {documentation.readme}</li>
                <li>
                    <strong>CONTRIBUTING:</strong>
                    {documentation.contributing}
                </li>
                <li>
                    <strong>Code of Conduct:</strong>
                    {documentation.codeOfConduct}
                </li>
                <li><strong>LICENSE:</strong> {documentation.license}</li>
            </ul>
        </div>

        <div class="section">
            <h2>Security</h2>
            <ul>
                <li>
                    <strong>Security Policy:</strong>
                    {security.securityPolicy}
                </li>
                <li>
                    <strong>Dependabot Alerts:</strong>
                    {#each security.dependabotAlerts as alert}{alert},
                    {/each}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Environment Variables</h2>
            <ul>
                {#each environmentVariables as envVar}
                    <li>{envVar}</li>
                {/each}
            </ul>
        </div>

        <div class="section">
            <h2>CI/CD Settings</h2>
            <ul>
                <li>
                    <strong>Build Configuration:</strong>
                    {ciCdSettings.buildConfig}
                </li>
                <li>
                    <strong>Deployment Configuration:</strong>
                    {ciCdSettings.deploymentConfig}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Backup and Restore</h2>
            <ul>
                <li>
                    <strong>Backup Procedures:</strong>
                    {backupAndRestore.backupProcedures}
                </li>
                <li>
                    <strong>Restore Procedures:</strong>
                    {backupAndRestore.restoreProcedures}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Contact Information</h2>
            <ul>
                <li>
                    <strong>Maintainers:</strong>
                    {#each contactInfo.maintainers as maintainer}{maintainer},
                    {/each}
                </li>
                <li>
                    <strong>Support Channels:</strong>
                    {#each contactInfo.supportChannels as channel}{channel},
                    {/each}
                </li>
            </ul>
        </div>

        <div class="section">
            <h2>Miscellaneous</h2>
            <ul>
                <li>
                    <strong>Custom Scripts:</strong>
                    {#each misc.customScripts as script}{script},
                    {/each}
                </li>
                <li>
                    <strong>Hooks and Automation:</strong>
                    {#each misc.hooksAndAutomation as item}{item},
                    {/each}
                </li>
            </ul>
        </div>
    {:else}
        <p>Loading...</p>
    {/if}
</main>

<style>
    main {
        font-family: Arial, sans-serif;
        padding: 1rem;
    }
    .section {
        margin-bottom: 2rem;
    }
    .section h2 {
        margin-bottom: 1rem;
    }
    .section ul {
        list-style-type: none;
        padding-left: 0;
    }
    .section ul li {
        margin-bottom: 0.5rem;
    }

    .container {
        font-family: Arial, sans-serif;
        padding: 20px;
    }
    .title {
        color: #333;
    }
    .fork-list {
        list-style-type: none;
        padding: 0;
    }
    .fork-item {
        background: #f9f9f9;
        margin: 10px 0;
        padding: 10px;
        border: 1px solid #ddd;
        border-radius: 5px;
    }
    .fork-link {
        color: #007bff;
        text-decoration: none;
    }
    .fork-link:hover {
        text-decoration: underline;
    }

    .container {
        width: 80%;
        margin: 0 auto;
        padding: 20px;
        background-color: #fff;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        border-radius: 8px;
    }

    .header,
    .section-title,
    .remote-title,
    .branch-title {
        color: #0056b3;
    }

    .link {
        color: #0056b3;
        text-decoration: none;
    }

    .link:hover {
        text-decoration: underline;
    }

    .list {
        list-style-type: none;
        padding: 0;
    }

    .list-item {
        margin-bottom: 10px;
    }
    .remote,
    .branch-details,
    .section {
        margin-bottom: 20px;
    }

    .branch-details .list .list-item {
        margin-left: 20px;
        list-style-type: disc;
    }

    .remote-list,
    .branch-list {
        display: flex;
        flex-direction: column;
    }

    .remote-item,
    .branch-item {
        cursor: pointer;
        padding: 10px;
        border-bottom: 1px solid #eee;
    }

    .remote-item:hover,
    .branch-item:hover {
        background-color: #f9f9f9;
    }

    .active {
        background-color: #e0e0e0;
    }

    .container {
        width: 80%;
        margin: 0 auto;
        padding: 20px;
        background-color: #fff;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        border-radius: 8px;
    }

    .header,
    .section-title,
    .remote-title,
    .branch-title {
        color: #0056b3;
    }

    .link {
        color: #0056b3;
        text-decoration: none;
    }

    .link:hover {
        text-decoration: underline;
    }

    .breadcrumb {
        display: flex;
        list-style: none;
        padding: 0;
    }

    .breadcrumb-item {
        margin-right: 10px;
        cursor: pointer;
    }

    /* .breadcrumb-item::after {
        content: ">";
        margin-left: 10px;
        color: #999;
    } */

    .breadcrumb-item:last-child::after {
        content: "";
    }

    .list {
        list-style-type: none;
        padding: 0;
    }

    .list-item {
        margin-bottom: 10px;
    }
    .remote,
    .branch-details,
    .section {
        margin-bottom: 20px;
    }

    .branch-details .list .list-item {
        margin-left: 20px;
        list-style-type: disc;
    }

    .branch-list {
        display: flex;
        flex-direction: column;
    }

    .branch-item {
        cursor: pointer;
        padding: 10px;
        border-bottom: 1px solid #eee;
    }

    .branch-item:hover {
        background-color: #f9f9f9;
    }

    .active {
        background-color: #e0e0e0;
    }

    .breadcrumb {
        display: flex;
        justify-content: space-around; /* Ensures even distribution across the available space */
        align-items: center;
        padding: 0;
        margin: 0;
        background-color: #f9f9f9;
        border-radius: 8px;
        box-shadow: 0 0 5px rgba(0, 0, 0, 0.1);
    }

    .breadcrumb-item {
        flex: 1; /* Each item takes an equal share of the available space */
        text-align: center; /* Center the content within each item */
        padding: 10px;
        cursor: pointer;
        background-color: #f5f5f5;
        border-right: 1px solid #ddd;
    }

    .breadcrumb-item:last-child {
        border-right: none;
    }

    .breadcrumb-item.active {
        background-color: #e0e0e0;
        font-weight: bold;
    }

    .breadcrumb-item:hover {
        background-color: #f0f0f0;
    }

    .remote-details {
        margin-top: 20px;
        padding: 20px;
        background-color: #f9f9f9;
        border-radius: 8px;
        box-shadow: 0 0 5px rgba(0, 0, 0, 0.1);
    }

    .remote-details p {
        margin: 10px 0;
    }

    .remote-details a {
        color: #0056b3;
        text-decoration: none;
    }

    .remote-details a:hover {
        text-decoration: underline;
    }

    .paragraph {
        /* font-family: Arial, sans-serif;
        margin-bottom: 1rem; */
        /* margin: 5px 0; */
        display: flex;
        align-items: center;
        font-family: Arial, sans-serif;
        margin-bottom: 1rem;
    }

    .textbox {
        width: 500px; /* Adjust width as needed */
        padding: 0.5rem;
        font-size: 1rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        margin-right: 1rem;
    }

    .button {
        padding: 0.5rem 1rem;
        font-size: 1rem;
        color: #fff;
        background-color: #3498db;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        display: none; /* Default hidden */
    }

    .button:hover {
        background-color: #2980b9;
    }

    .button.show {
        display: inline-block; /* Show button when condition is met */
    }

    .branch-title {
        font-family: "Arial", sans-serif;
        font-size: 1.2rem;
        color: #333;
        margin-bottom: 0.5rem;
        border-bottom: 2px solid #3498db;
        padding-bottom: 0.2rem;
    }

    .list {
        list-style-type: none;
        padding-left: 0;
        margin-bottom: 1rem;
    }

    .list-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        margin-bottom: 0.5rem;
        transition: background-color 0.3s ease;
    }

    .list-item:hover {
        background-color: #f1f1f1;
    }

    .branch-name {
        font-weight: bold;
        color: #2c3e50;
    }

    .branch-status {
        font-style: italic;
        color: #7f8c8d;
    }

    .branch-status.up-to-date {
        color: #27ae60;
    }

    .branch-status.local-out-of-date {
        color: #e74c3c;
    }

    .branch-status.remote-out-of-date {
        color: #e67e22;
    }

    .branch-status.divergent {
        color: #f39c12;
    }

    .arrow-up {
        color: #3498db; /* Blue arrow */
        font-size: 1.75rem; /* Adjust the size as needed */
    }

    .arrow-down {
        color: #e74c3c; /* Red arrow */
        font-size: 1.75rem; /* Adjust the size as needed */
    }

    .branch-title {
        font-family: "Arial", sans-serif;
        font-size: 1.2rem;
        color: #333;
        margin-bottom: 0.5rem;
        border-bottom: 2px solid #3498db;
        padding-bottom: 0.2rem;
    }

    .branch-list {
        list-style-type: none;
        padding-left: 0;
        margin-bottom: 1rem;
    }

    .branch-item {
        display: flex;
        flex-direction: column;
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        margin-bottom: 0.5rem;
        transition: background-color 0.3s ease;
    }

    .branch-item:hover {
        background-color: #f1f1f1;
    }

    .branch-name {
        font-weight: bold;
        color: #2c3e50;
    }

    .branch-status {
        font-style: italic;
        color: #7f8c8d;
    }

    .branch-status.up-to-date {
        color: #27ae60;
    }

    .branch-status.local-out-of-date {
        color: #e74c3c;
    }

    .branch-status.remote-out-of-date {
        color: #e67e22;
    }

    .branch-status.divergent {
        color: #f39c12;
    }

    .arrow-up {
        color: #3498db; /* Blue arrow */
        font-size: 1.5rem; /* Adjust the size as needed */
    }

    .arrow-down {
        color: #e74c3c; /* Red arrow */
        font-size: 1.5rem; /* Adjust the size as needed */
    }

    .active {
        background-color: #e0f7fa;
    }
</style>
