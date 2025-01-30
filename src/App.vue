<script setup>
// import HelloWorld from './components/HelloWorld.vue'
// import TheWelcome from './components/TheWelcome.vue'
import { ref, onMounted, onBeforeUnmount } from 'vue';
import DebugWindow from './components/DebugWindow.vue';

const debugWindowRef = ref(null);

const toggleDebugWindow = () => {
  // if (debugWindowRef.value) {
  debugWindowRef.value.toggleVisibility();
  // }
};

const handleKeyDown = (event) => {
  if (event.ctrlKey && event.key === 'D') {
    toggleDebugWindow();
  }
};

onMounted(() => {
  // debugger;
  window.addEventListener('keydown', handleKeyDown);
  // Ensure the debugWindowRef is available globally after mounting
  // window.debugWindowRef = debugWindowRef;
  //TODO: This is the start of file watcher - do we need this?
  window.addEventListener("custom-event", (event) => {
    console.log(event.detail); // Output: Hello from Rust!
  });
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeyDown);
});
</script>

<template>
  <header>
    <!-- <img alt="Vue logo" class="logo" src="./assets/logo.svg" width="125" height="125" />

    <div class="wrapper">
      <HelloWorld msg="You did it!" />
    </div> -->
  </header>

  <main id="app">
    <!-- <button @click="toggleDebugWindow">Toggle Debug Window</button> -->
    <DebugWindow ref="debugWindowRef" />
    <router-view />
  </main>

</template>

<style scoped>
/* header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
} */
/* #app {
  text-align: center;
} */
</style>
