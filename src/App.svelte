<script>
  import Router from 'svelte-spa-router';
  import routes from './routes'
  import DebugWindow from './components/DebugWindow.svelte';
</script>

<Router {routes}/>
<DebugWindow/>