<script setup>
// import HelloWorld from './components/HelloWorld.vue'
// import TheWelcome from './components/TheWelcome.vue'
import { ref, onMounted, onBeforeUnmount } from 'vue';
import DebugWindow from './components/DebugWindow.vue';

const debugWindowRef = ref(null);

onMounted(() => {
  // debugger;

  //TODO: DEL?
  // Ensure the debugWindowRef is available globally after mounting
  // window.debugWindowRef = debugWindowRef;
  window.addEventListener('keydown', (event) => {
    if (event.ctrlKey && event.key === 'D') {
      debugWindowRef.value.toggleVisibility();
    }
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
