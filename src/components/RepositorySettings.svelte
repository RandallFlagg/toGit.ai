<script>
    import { onMount } from "svelte";

    let repoDetails; // = null;
    let error; // = null;
    let forks = [];

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

    onMount(async () => {
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
        await fetchRepoDetails();
        for (const remote of repoDetails.remotes) {
            const info = parseGitUrl(remote.fetch_url);
            console.log(info);
            if (false) {
                //TODO: Need to autenticate to get higher API calls
                //Inorder to save aPI calls this is disabled
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

                    //forks.push(await forkResponse.json());
                    const data = await forkResponse.json();
                    forks = [...data];
                    console.log("Done.");
                }
            } else {
                const data = [
                    {
                        id: 677639753,
                        node_id: "R_kgDOKGP2SQ",
                        name: "OpenImageViewer",
                        full_name: "mfkiwl/OpenImageViewer",
                        private: false,
                        owner: {
                            login: "mfkiwl",
                            id: 3277000,
                            node_id: "MDQ6VXNlcjMyNzcwMDA=",
                            avatar_url:
                                "https://avatars.githubusercontent.com/u/3277000?v=4",
                            gravatar_id: "",
                            url: "https://api.github.com/users/mfkiwl",
                            html_url: "https://github.com/mfkiwl",
                            followers_url:
                                "https://api.github.com/users/mfkiwl/followers",
                            following_url:
                                "https://api.github.com/users/mfkiwl/following{/other_user}",
                            gists_url:
                                "https://api.github.com/users/mfkiwl/gists{/gist_id}",
                            starred_url:
                                "https://api.github.com/users/mfkiwl/starred{/owner}{/repo}",
                            subscriptions_url:
                                "https://api.github.com/users/mfkiwl/subscriptions",
                            organizations_url:
                                "https://api.github.com/users/mfkiwl/orgs",
                            repos_url:
                                "https://api.github.com/users/mfkiwl/repos",
                            events_url:
                                "https://api.github.com/users/mfkiwl/events{/privacy}",
                            received_events_url:
                                "https://api.github.com/users/mfkiwl/received_events",
                            type: "User",
                            user_view_type: "public",
                            site_admin: false,
                        },
                        html_url: "https://github.com/mfkiwl/OpenImageViewer",
                        description:
                            "Open image viewer is a hardware accelerated open code c++20 compliant cross platform 'C' library and application for viewing and manipulating images.",
                        fork: true,
                        url: "https://api.github.com/repos/mfkiwl/OpenImageViewer",
                        forks_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/forks",
                        keys_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/keys{/key_id}",
                        collaborators_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/collaborators{/collaborator}",
                        teams_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/teams",
                        hooks_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/hooks",
                        issue_events_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/issues/events{/number}",
                        events_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/events",
                        assignees_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/assignees{/user}",
                        branches_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/branches{/branch}",
                        tags_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/tags",
                        blobs_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/git/blobs{/sha}",
                        git_tags_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/git/tags{/sha}",
                        git_refs_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/git/refs{/sha}",
                        trees_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/git/trees{/sha}",
                        statuses_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/statuses/{sha}",
                        languages_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/languages",
                        stargazers_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/stargazers",
                        contributors_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/contributors",
                        subscribers_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/subscribers",
                        subscription_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/subscription",
                        commits_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/commits{/sha}",
                        git_commits_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/git/commits{/sha}",
                        comments_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/comments{/number}",
                        issue_comment_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/issues/comments{/number}",
                        contents_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/contents/{+path}",
                        compare_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/compare/{base}...{head}",
                        merges_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/merges",
                        archive_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/{archive_format}{/ref}",
                        downloads_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/downloads",
                        issues_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/issues{/number}",
                        pulls_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/pulls{/number}",
                        milestones_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/milestones{/number}",
                        notifications_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/notifications{?since,all,participating}",
                        labels_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/labels{/name}",
                        releases_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/releases{/id}",
                        deployments_url:
                            "https://api.github.com/repos/mfkiwl/OpenImageViewer/deployments",
                        created_at: "2023-08-12T06:14:21Z",
                        updated_at: "2023-08-12T06:14:36Z",
                        pushed_at: "2023-08-12T00:26:14Z",
                        git_url: "git://github.com/mfkiwl/OpenImageViewer.git",
                        ssh_url: "git@github.com:mfkiwl/OpenImageViewer.git",
                        clone_url:
                            "https://github.com/mfkiwl/OpenImageViewer.git",
                        svn_url: "https://github.com/mfkiwl/OpenImageViewer",
                        homepage: "",
                        size: 1941,
                        stargazers_count: 1,
                        watchers_count: 1,
                        language: null,
                        has_issues: false,
                        has_projects: true,
                        has_downloads: true,
                        has_wiki: true,
                        has_pages: false,
                        has_discussions: false,
                        forks_count: 0,
                        mirror_url: null,
                        archived: false,
                        disabled: false,
                        open_issues_count: 0,
                        license: {
                            key: "other",
                            name: "Other",
                            spdx_id: "NOASSERTION",
                            url: null,
                            node_id: "MDc6TGljZW5zZTA=",
                        },
                        allow_forking: true,
                        is_template: false,
                        web_commit_signoff_required: false,
                        topics: [],
                        visibility: "public",
                        forks: 0,
                        open_issues: 0,
                        watchers: 1,
                        default_branch: "master",
                    },
                    {
                        id: 675764169,
                        node_id: "R_kgDOKEdXyQ",
                        name: "OpenImageViewer",
                        full_name: "webstorage119/OpenImageViewer",
                        private: false,
                        owner: {
                            login: "webstorage119",
                            id: 44592032,
                            node_id: "MDQ6VXNlcjQ0NTkyMDMy",
                            avatar_url:
                                "https://avatars.githubusercontent.com/u/44592032?v=4",
                            gravatar_id: "",
                            url: "https://api.github.com/users/webstorage119",
                            html_url: "https://github.com/webstorage119",
                            followers_url:
                                "https://api.github.com/users/webstorage119/followers",
                            following_url:
                                "https://api.github.com/users/webstorage119/following{/other_user}",
                            gists_url:
                                "https://api.github.com/users/webstorage119/gists{/gist_id}",
                            starred_url:
                                "https://api.github.com/users/webstorage119/starred{/owner}{/repo}",
                            subscriptions_url:
                                "https://api.github.com/users/webstorage119/subscriptions",
                            organizations_url:
                                "https://api.github.com/users/webstorage119/orgs",
                            repos_url:
                                "https://api.github.com/users/webstorage119/repos",
                            events_url:
                                "https://api.github.com/users/webstorage119/events{/privacy}",
                            received_events_url:
                                "https://api.github.com/users/webstorage119/received_events",
                            type: "User",
                            user_view_type: "public",
                            site_admin: false,
                        },
                        html_url:
                            "https://github.com/webstorage119/OpenImageViewer",
                        description:
                            "Open image viewer is a hardware accelerated open code c++20 compliant cross platform 'C' library and application for viewing and manipulating images.",
                        fork: true,
                        url: "https://api.github.com/repos/webstorage119/OpenImageViewer",
                        forks_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/forks",
                        keys_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/keys{/key_id}",
                        collaborators_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/collaborators{/collaborator}",
                        teams_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/teams",
                        hooks_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/hooks",
                        issue_events_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/issues/events{/number}",
                        events_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/events",
                        assignees_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/assignees{/user}",
                        branches_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/branches{/branch}",
                        tags_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/tags",
                        blobs_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/git/blobs{/sha}",
                        git_tags_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/git/tags{/sha}",
                        git_refs_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/git/refs{/sha}",
                        trees_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/git/trees{/sha}",
                        statuses_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/statuses/{sha}",
                        languages_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/languages",
                        stargazers_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/stargazers",
                        contributors_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/contributors",
                        subscribers_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/subscribers",
                        subscription_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/subscription",
                        commits_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/commits{/sha}",
                        git_commits_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/git/commits{/sha}",
                        comments_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/comments{/number}",
                        issue_comment_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/issues/comments{/number}",
                        contents_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/contents/{+path}",
                        compare_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/compare/{base}...{head}",
                        merges_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/merges",
                        archive_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/{archive_format}{/ref}",
                        downloads_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/downloads",
                        issues_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/issues{/number}",
                        pulls_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/pulls{/number}",
                        milestones_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/milestones{/number}",
                        notifications_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/notifications{?since,all,participating}",
                        labels_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/labels{/name}",
                        releases_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/releases{/id}",
                        deployments_url:
                            "https://api.github.com/repos/webstorage119/OpenImageViewer/deployments",
                        created_at: "2023-08-07T17:02:43Z",
                        updated_at: "2023-08-07T17:02:43Z",
                        pushed_at: "2022-07-06T03:42:53Z",
                        git_url:
                            "git://github.com/webstorage119/OpenImageViewer.git",
                        ssh_url:
                            "git@github.com:webstorage119/OpenImageViewer.git",
                        clone_url:
                            "https://github.com/webstorage119/OpenImageViewer.git",
                        svn_url:
                            "https://github.com/webstorage119/OpenImageViewer",
                        homepage: "",
                        size: 2041,
                        stargazers_count: 0,
                        watchers_count: 0,
                        language: null,
                        has_issues: false,
                        has_projects: true,
                        has_downloads: true,
                        has_wiki: true,
                        has_pages: false,
                        has_discussions: false,
                        forks_count: 0,
                        mirror_url: null,
                        archived: false,
                        disabled: false,
                        open_issues_count: 0,
                        license: {
                            key: "other",
                            name: "Other",
                            spdx_id: "NOASSERTION",
                            url: null,
                            node_id: "MDc6TGljZW5zZTA=",
                        },
                        allow_forking: true,
                        is_template: false,
                        web_commit_signoff_required: false,
                        topics: [],
                        visibility: "public",
                        forks: 0,
                        open_issues: 0,
                        watchers: 0,
                        default_branch: "master",
                    },
                    {
                        id: 539337115,
                        node_id: "R_kgDOICWhmw",
                        name: "OpenImageViewer",
                        full_name: "WilliamQf-AI/OpenImageViewer",
                        private: false,
                        owner: {
                            login: "WilliamQf-AI",
                            id: 40328063,
                            node_id: "MDQ6VXNlcjQwMzI4MDYz",
                            avatar_url:
                                "https://avatars.githubusercontent.com/u/40328063?v=4",
                            gravatar_id: "",
                            url: "https://api.github.com/users/WilliamQf-AI",
                            html_url: "https://github.com/WilliamQf-AI",
                            followers_url:
                                "https://api.github.com/users/WilliamQf-AI/followers",
                            following_url:
                                "https://api.github.com/users/WilliamQf-AI/following{/other_user}",
                            gists_url:
                                "https://api.github.com/users/WilliamQf-AI/gists{/gist_id}",
                            starred_url:
                                "https://api.github.com/users/WilliamQf-AI/starred{/owner}{/repo}",
                            subscriptions_url:
                                "https://api.github.com/users/WilliamQf-AI/subscriptions",
                            organizations_url:
                                "https://api.github.com/users/WilliamQf-AI/orgs",
                            repos_url:
                                "https://api.github.com/users/WilliamQf-AI/repos",
                            events_url:
                                "https://api.github.com/users/WilliamQf-AI/events{/privacy}",
                            received_events_url:
                                "https://api.github.com/users/WilliamQf-AI/received_events",
                            type: "User",
                            user_view_type: "public",
                            site_admin: false,
                        },
                        html_url:
                            "https://github.com/WilliamQf-AI/OpenImageViewer",
                        description:
                            "Open image viewer is a hardware accelerated open code c++20 compliant cross platform 'C' library and application for viewing and manipulating images.",
                        fork: true,
                        url: "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer",
                        forks_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/forks",
                        keys_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/keys{/key_id}",
                        collaborators_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/collaborators{/collaborator}",
                        teams_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/teams",
                        hooks_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/hooks",
                        issue_events_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/issues/events{/number}",
                        events_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/events",
                        assignees_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/assignees{/user}",
                        branches_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/branches{/branch}",
                        tags_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/tags",
                        blobs_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/git/blobs{/sha}",
                        git_tags_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/git/tags{/sha}",
                        git_refs_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/git/refs{/sha}",
                        trees_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/git/trees{/sha}",
                        statuses_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/statuses/{sha}",
                        languages_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/languages",
                        stargazers_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/stargazers",
                        contributors_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/contributors",
                        subscribers_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/subscribers",
                        subscription_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/subscription",
                        commits_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/commits{/sha}",
                        git_commits_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/git/commits{/sha}",
                        comments_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/comments{/number}",
                        issue_comment_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/issues/comments{/number}",
                        contents_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/contents/{+path}",
                        compare_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/compare/{base}...{head}",
                        merges_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/merges",
                        archive_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/{archive_format}{/ref}",
                        downloads_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/downloads",
                        issues_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/issues{/number}",
                        pulls_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/pulls{/number}",
                        milestones_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/milestones{/number}",
                        notifications_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/notifications{?since,all,participating}",
                        labels_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/labels{/name}",
                        releases_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/releases{/id}",
                        deployments_url:
                            "https://api.github.com/repos/WilliamQf-AI/OpenImageViewer/deployments",
                        created_at: "2022-09-21T06:21:39Z",
                        updated_at: "2024-03-05T02:10:19Z",
                        pushed_at: "2024-03-05T02:10:12Z",
                        git_url:
                            "git://github.com/WilliamQf-AI/OpenImageViewer.git",
                        ssh_url:
                            "git@github.com:WilliamQf-AI/OpenImageViewer.git",
                        clone_url:
                            "https://github.com/WilliamQf-AI/OpenImageViewer.git",
                        svn_url:
                            "https://github.com/WilliamQf-AI/OpenImageViewer",
                        homepage: "",
                        size: 2269,
                        stargazers_count: 0,
                        watchers_count: 0,
                        language: "C++",
                        has_issues: false,
                        has_projects: true,
                        has_downloads: true,
                        has_wiki: true,
                        has_pages: false,
                        has_discussions: false,
                        forks_count: 0,
                        mirror_url: null,
                        archived: false,
                        disabled: false,
                        open_issues_count: 0,
                        license: {
                            key: "other",
                            name: "Other",
                            spdx_id: "NOASSERTION",
                            url: null,
                            node_id: "MDc6TGljZW5zZTA=",
                        },
                        allow_forking: true,
                        is_template: false,
                        web_commit_signoff_required: false,
                        topics: [],
                        visibility: "public",
                        forks: 0,
                        open_issues: 0,
                        watchers: 0,
                        default_branch: "master",
                    },
                    {
                        id: 262526381,
                        node_id: "MDEwOlJlcG9zaXRvcnkyNjI1MjYzODE=",
                        name: "OpenImageViewer",
                        full_name: "sharny/OpenImageViewer",
                        private: false,
                        owner: {
                            login: "sharny",
                            id: 12163323,
                            node_id: "MDQ6VXNlcjEyMTYzMzIz",
                            avatar_url:
                                "https://avatars.githubusercontent.com/u/12163323?v=4",
                            gravatar_id: "",
                            url: "https://api.github.com/users/sharny",
                            html_url: "https://github.com/sharny",
                            followers_url:
                                "https://api.github.com/users/sharny/followers",
                            following_url:
                                "https://api.github.com/users/sharny/following{/other_user}",
                            gists_url:
                                "https://api.github.com/users/sharny/gists{/gist_id}",
                            starred_url:
                                "https://api.github.com/users/sharny/starred{/owner}{/repo}",
                            subscriptions_url:
                                "https://api.github.com/users/sharny/subscriptions",
                            organizations_url:
                                "https://api.github.com/users/sharny/orgs",
                            repos_url:
                                "https://api.github.com/users/sharny/repos",
                            events_url:
                                "https://api.github.com/users/sharny/events{/privacy}",
                            received_events_url:
                                "https://api.github.com/users/sharny/received_events",
                            type: "User",
                            user_view_type: "public",
                            site_admin: false,
                        },
                        html_url: "https://github.com/sharny/OpenImageViewer",
                        description:
                            "Open image viewer is an hardware accelerated open code c++17 compliant cross platform 'C' library and application for viewing and manipulating images.",
                        fork: true,
                        url: "https://api.github.com/repos/sharny/OpenImageViewer",
                        forks_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/forks",
                        keys_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/keys{/key_id}",
                        collaborators_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/collaborators{/collaborator}",
                        teams_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/teams",
                        hooks_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/hooks",
                        issue_events_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/issues/events{/number}",
                        events_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/events",
                        assignees_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/assignees{/user}",
                        branches_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/branches{/branch}",
                        tags_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/tags",
                        blobs_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/git/blobs{/sha}",
                        git_tags_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/git/tags{/sha}",
                        git_refs_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/git/refs{/sha}",
                        trees_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/git/trees{/sha}",
                        statuses_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/statuses/{sha}",
                        languages_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/languages",
                        stargazers_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/stargazers",
                        contributors_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/contributors",
                        subscribers_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/subscribers",
                        subscription_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/subscription",
                        commits_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/commits{/sha}",
                        git_commits_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/git/commits{/sha}",
                        comments_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/comments{/number}",
                        issue_comment_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/issues/comments{/number}",
                        contents_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/contents/{+path}",
                        compare_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/compare/{base}...{head}",
                        merges_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/merges",
                        archive_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/{archive_format}{/ref}",
                        downloads_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/downloads",
                        issues_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/issues{/number}",
                        pulls_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/pulls{/number}",
                        milestones_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/milestones{/number}",
                        notifications_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/notifications{?since,all,participating}",
                        labels_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/labels{/name}",
                        releases_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/releases{/id}",
                        deployments_url:
                            "https://api.github.com/repos/sharny/OpenImageViewer/deployments",
                        created_at: "2020-05-09T08:36:27Z",
                        updated_at: "2020-05-09T08:36:29Z",
                        pushed_at: "2020-02-15T16:11:10Z",
                        git_url: "git://github.com/sharny/OpenImageViewer.git",
                        ssh_url: "git@github.com:sharny/OpenImageViewer.git",
                        clone_url:
                            "https://github.com/sharny/OpenImageViewer.git",
                        svn_url: "https://github.com/sharny/OpenImageViewer",
                        homepage: "",
                        size: 1533,
                        stargazers_count: 0,
                        watchers_count: 0,
                        language: null,
                        has_issues: false,
                        has_projects: true,
                        has_downloads: true,
                        has_wiki: true,
                        has_pages: false,
                        has_discussions: false,
                        forks_count: 0,
                        mirror_url: null,
                        archived: false,
                        disabled: false,
                        open_issues_count: 0,
                        license: {
                            key: "other",
                            name: "Other",
                            spdx_id: "NOASSERTION",
                            url: null,
                            node_id: "MDc6TGljZW5zZTA=",
                        },
                        allow_forking: true,
                        is_template: false,
                        web_commit_signoff_required: false,
                        topics: [],
                        visibility: "public",
                        forks: 0,
                        open_issues: 0,
                        watchers: 0,
                        default_branch: "master",
                    },
                    {
                        id: 107843349,
                        node_id: "MDEwOlJlcG9zaXRvcnkxMDc4NDMzNDk=",
                        name: "OpenImageViewerFork",
                        full_name: "TheNicker/OpenImageViewerFork",
                        private: false,
                        owner: {
                            login: "TheNicker",
                            id: 4753283,
                            node_id: "MDQ6VXNlcjQ3NTMyODM=",
                            avatar_url:
                                "https://avatars.githubusercontent.com/u/4753283?v=4",
                            gravatar_id: "",
                            url: "https://api.github.com/users/TheNicker",
                            html_url: "https://github.com/TheNicker",
                            followers_url:
                                "https://api.github.com/users/TheNicker/followers",
                            following_url:
                                "https://api.github.com/users/TheNicker/following{/other_user}",
                            gists_url:
                                "https://api.github.com/users/TheNicker/gists{/gist_id}",
                            starred_url:
                                "https://api.github.com/users/TheNicker/starred{/owner}{/repo}",
                            subscriptions_url:
                                "https://api.github.com/users/TheNicker/subscriptions",
                            organizations_url:
                                "https://api.github.com/users/TheNicker/orgs",
                            repos_url:
                                "https://api.github.com/users/TheNicker/repos",
                            events_url:
                                "https://api.github.com/users/TheNicker/events{/privacy}",
                            received_events_url:
                                "https://api.github.com/users/TheNicker/received_events",
                            type: "User",
                            user_view_type: "public",
                            site_admin: false,
                        },
                        html_url:
                            "https://github.com/TheNicker/OpenImageViewerFork",
                        description:
                            "OIV, Open image viewer is an hardware accelerated open code c++20 compliant cross platform 'C' library and application for viewing and manipulating images.",
                        fork: true,
                        url: "https://api.github.com/repos/TheNicker/OpenImageViewerFork",
                        forks_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/forks",
                        keys_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/keys{/key_id}",
                        collaborators_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/collaborators{/collaborator}",
                        teams_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/teams",
                        hooks_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/hooks",
                        issue_events_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/issues/events{/number}",
                        events_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/events",
                        assignees_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/assignees{/user}",
                        branches_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/branches{/branch}",
                        tags_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/tags",
                        blobs_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/git/blobs{/sha}",
                        git_tags_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/git/tags{/sha}",
                        git_refs_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/git/refs{/sha}",
                        trees_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/git/trees{/sha}",
                        statuses_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/statuses/{sha}",
                        languages_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/languages",
                        stargazers_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/stargazers",
                        contributors_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/contributors",
                        subscribers_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/subscribers",
                        subscription_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/subscription",
                        commits_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/commits{/sha}",
                        git_commits_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/git/commits{/sha}",
                        comments_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/comments{/number}",
                        issue_comment_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/issues/comments{/number}",
                        contents_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/contents/{+path}",
                        compare_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/compare/{base}...{head}",
                        merges_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/merges",
                        archive_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/{archive_format}{/ref}",
                        downloads_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/downloads",
                        issues_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/issues{/number}",
                        pulls_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/pulls{/number}",
                        milestones_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/milestones{/number}",
                        notifications_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/notifications{?since,all,participating}",
                        labels_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/labels{/name}",
                        releases_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/releases{/id}",
                        deployments_url:
                            "https://api.github.com/repos/TheNicker/OpenImageViewerFork/deployments",
                        created_at: "2017-10-22T06:41:04Z",
                        updated_at: "2025-02-21T09:13:27Z",
                        pushed_at: "2025-02-22T17:40:30Z",
                        git_url:
                            "git://github.com/TheNicker/OpenImageViewerFork.git",
                        ssh_url:
                            "git@github.com:TheNicker/OpenImageViewerFork.git",
                        clone_url:
                            "https://github.com/TheNicker/OpenImageViewerFork.git",
                        svn_url:
                            "https://github.com/TheNicker/OpenImageViewerFork",
                        homepage: "",
                        size: 2411,
                        stargazers_count: 0,
                        watchers_count: 0,
                        language: "C++",
                        has_issues: false,
                        has_projects: true,
                        has_downloads: true,
                        has_wiki: true,
                        has_pages: false,
                        has_discussions: false,
                        forks_count: 0,
                        mirror_url: null,
                        archived: false,
                        disabled: false,
                        open_issues_count: 0,
                        license: {
                            key: "other",
                            name: "Other",
                            spdx_id: "NOASSERTION",
                            url: null,
                            node_id: "MDc6TGljZW5zZTA=",
                        },
                        allow_forking: true,
                        is_template: false,
                        web_commit_signoff_required: false,
                        topics: [],
                        visibility: "public",
                        forks: 0,
                        open_issues: 0,
                        watchers: 0,
                        default_branch: "master",
                    },
                    {
                        id: 98785234,
                        node_id: "MDEwOlJlcG9zaXRvcnk5ODc4NTIzNA==",
                        name: "OIV",
                        full_name: "RandallFlagg/OIV",
                        private: false,
                        owner: {
                            login: "RandallFlagg",
                            id: 434681,
                            node_id: "MDQ6VXNlcjQzNDY4MQ==",
                            avatar_url:
                                "https://avatars.githubusercontent.com/u/434681?v=4",
                            gravatar_id: "",
                            url: "https://api.github.com/users/RandallFlagg",
                            html_url: "https://github.com/RandallFlagg",
                            followers_url:
                                "https://api.github.com/users/RandallFlagg/followers",
                            following_url:
                                "https://api.github.com/users/RandallFlagg/following{/other_user}",
                            gists_url:
                                "https://api.github.com/users/RandallFlagg/gists{/gist_id}",
                            starred_url:
                                "https://api.github.com/users/RandallFlagg/starred{/owner}{/repo}",
                            subscriptions_url:
                                "https://api.github.com/users/RandallFlagg/subscriptions",
                            organizations_url:
                                "https://api.github.com/users/RandallFlagg/orgs",
                            repos_url:
                                "https://api.github.com/users/RandallFlagg/repos",
                            events_url:
                                "https://api.github.com/users/RandallFlagg/events{/privacy}",
                            received_events_url:
                                "https://api.github.com/users/RandallFlagg/received_events",
                            type: "User",
                            user_view_type: "public",
                            site_admin: false,
                        },
                        html_url: "https://github.com/RandallFlagg/OIV",
                        description:
                            "OIV, Open image viewer is an hardware accelerated open code c++17 compliant cross platform 'C' library and application for viewing and manipulating images.",
                        fork: true,
                        url: "https://api.github.com/repos/RandallFlagg/OIV",
                        forks_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/forks",
                        keys_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/keys{/key_id}",
                        collaborators_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/collaborators{/collaborator}",
                        teams_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/teams",
                        hooks_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/hooks",
                        issue_events_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/issues/events{/number}",
                        events_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/events",
                        assignees_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/assignees{/user}",
                        branches_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/branches{/branch}",
                        tags_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/tags",
                        blobs_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/git/blobs{/sha}",
                        git_tags_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/git/tags{/sha}",
                        git_refs_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/git/refs{/sha}",
                        trees_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/git/trees{/sha}",
                        statuses_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/statuses/{sha}",
                        languages_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/languages",
                        stargazers_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/stargazers",
                        contributors_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/contributors",
                        subscribers_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/subscribers",
                        subscription_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/subscription",
                        commits_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/commits{/sha}",
                        git_commits_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/git/commits{/sha}",
                        comments_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/comments{/number}",
                        issue_comment_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/issues/comments{/number}",
                        contents_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/contents/{+path}",
                        compare_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/compare/{base}...{head}",
                        merges_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/merges",
                        archive_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/{archive_format}{/ref}",
                        downloads_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/downloads",
                        issues_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/issues{/number}",
                        pulls_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/pulls{/number}",
                        milestones_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/milestones{/number}",
                        notifications_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/notifications{?since,all,participating}",
                        labels_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/labels{/name}",
                        releases_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/releases{/id}",
                        deployments_url:
                            "https://api.github.com/repos/RandallFlagg/OIV/deployments",
                        created_at: "2017-07-30T08:36:07Z",
                        updated_at: "2023-08-01T12:36:37Z",
                        pushed_at: "2024-11-24T12:17:16Z",
                        git_url: "git://github.com/RandallFlagg/OIV.git",
                        ssh_url: "git@github.com:RandallFlagg/OIV.git",
                        clone_url: "https://github.com/RandallFlagg/OIV.git",
                        svn_url: "https://github.com/RandallFlagg/OIV",
                        homepage: null,
                        size: 3027,
                        stargazers_count: 0,
                        watchers_count: 0,
                        language: "C++",
                        has_issues: false,
                        has_projects: true,
                        has_downloads: true,
                        has_wiki: true,
                        has_pages: false,
                        has_discussions: false,
                        forks_count: 0,
                        mirror_url: null,
                        archived: false,
                        disabled: false,
                        open_issues_count: 0,
                        license: {
                            key: "other",
                            name: "Other",
                            spdx_id: "NOASSERTION",
                            url: null,
                            node_id: "MDc6TGljZW5zZTA=",
                        },
                        allow_forking: true,
                        is_template: false,
                        web_commit_signoff_required: false,
                        topics: [],
                        visibility: "public",
                        forks: 0,
                        open_issues: 0,
                        watchers: 0,
                        default_branch: "master",
                    },
                ];
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
</style>
