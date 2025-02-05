<script>
    import { onMount, onDestroy } from "svelte";
    import { writable } from "svelte/store";
    import "diff2html/bundles/css/diff2html.min.css";
    import * as Diff2Html from "diff2html"; //TODO: Find a way to use local resources without NPM if possible?
    // const Diff2Html = {};
    // Diff2Html.html = ()=>{console.log("Diff2Html: The lib is missing. Need to check.");}
    // Import necessary modules
    // import '../assets/diff2html.min.css';
    // import * as Diff2Html from '../assets/diff2html.min.js';

    // Define component props
    export let diffString;

    // Reactive variable to store the rendered diff HTML
    let renderedDiff = "";

    // Function to render the diff
    const showDiff = () => {
        const configuration = {
            drawFileList: true,
            fileListToggle: false,
            fileListStartVisible: false,
            fileContentToggle: false,
            matching: "lines",
            outputFormat: "side-by-side",
            synchronisedScroll: true,
            highlight: true,
            renderNothingWhenEmpty: false,
        };

        // debugger;
        const diff = $diffString;

        renderedDiff = diff
            ? Diff2Html.html(diff, configuration)
            : "";
    };

    // Reactive statement to handle updates
    $: {
        // debugger;
        const diff = $diffString;
        if (diff === "Binary file") {
            renderedDiff = `<h3>Preview: ${diff}</h3>`;
        } else if (diff) {
            showDiff();
        } else {
            renderedDiff = "<h3>Preview: No Changes</h3>";
        }
    }
</script>

<!-- Render the diff HTML -->
<div>
    {@html renderedDiff}
</div>

<style>
    /* TODO: This is a hack to hide the title of the diff viewer. This should be fixed. */
    :global(.d2h-file-list-wrapper) {
        display: none;
    }
</style>
