<script>
    import { onMount } from "svelte";

    let repoDetails = null;
    let error = null;

    async function fetchRepoDetails() {
        try {
            console.log("A");
            repoDetails =
                await window.__TAURI__.core.invoke("get_repo_details");
            console.log(repoDetails);
        } catch (err) {
            console.log("B");
            error = err;
        }
    }

    onMount(() => {
        // Fetch repo details when the component mounts
        fetchRepoDetails();
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

<main>
    {#if error}
        <p>Error: {error.message}</p>
    {:else if repoDetails}
        <div class="section">
            <h2>Repository Details</h2>
            <ul>
                <li><strong>Name:</strong> {repoDetails.name}</li>
                <li><strong>Description:</strong> {repoDetails.description}</li>
                <li><strong>Remotes:</strong>
                    {#each repoDetails.remotes as remote}
                        <pre>{JSON.stringify(remote, null, 2)}</pre>
                        <strong>Name:</strong> {remote.name} <br>
                        <strong>URL:</strong> {remote.url} <br>
                        <strong>Push URL:</strong> {remote.push_url} <br>
                        <strong>Fetch:</strong> {remote.fetch} <br>
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
</style>
