import "./assets/styles.css"
// import './app.css'

import { mount } from 'svelte'
import SvelteApp from './App.svelte'

// import { createApp } from 'vue';
// import VueApp from './App.vue';
// import router from './router';

const svelte = true;
const vue = false;

if (svelte && vue) {
  throw new Error("Fix configuration")
} else {
  console.log(`svelte: ${svelte}, vue: ${vue}`);
}

if (svelte) {
  const app = mount(SvelteApp, {
    target: document.getElementById('app'),
  })
}
else if (vue) {
  const app = createApp(VueApp);
  app.use(router);
  app.mount('#app');
}
else {
  throw new Error("How did we get here?");
}

// export default app