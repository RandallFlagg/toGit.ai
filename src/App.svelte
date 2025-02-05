<script>
  import Router from 'svelte-spa-router';
  import routes from './router/routes'
  import DebugWindow from './components/DebugWindow.svelte';
</script>

<Router {routes}/>
<DebugWindow/>