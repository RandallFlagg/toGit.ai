<template>
    <div v-html="renderedDiff" />
</template>

<script setup>
// Import necessary modules
// import '../assets/diff2html.min.css';
// import * as Diff2Html from '../assets/diff2html.min.js';
import { ref, watch, defineProps } from 'vue';
import 'diff2html/bundles/css/diff2html.min.css';
import * as Diff2Html from 'diff2html'; //TODO: Find a way to use local resources without NPM if possible?

// Define component props
const props = defineProps({
    diffString: String
});

// Ref to store rendered diff HTML
const renderedDiff = ref('');

// Define a method to render the diff
const showDiff = () => {
    const configuration = {
        drawFileList: true,
        fileListToggle: false,
        fileListStartVisible: false,
        fileContentToggle: false,
        matching: 'lines',
        outputFormat: 'side-by-side',
        synchronisedScroll: true,
        highlight: true,
        renderNothingWhenEmpty: false,
    };

    renderedDiff.value = props.diffString ? Diff2Html.html(props.diffString, configuration) : '';
};

// Watch for changes to diffString and call showDiff when it changes
watch(() => props.diffString, (newVal) => {
  if (newVal) {
        showDiff();
    }
}, { immediate: true });
</script>


<style scoped></style>