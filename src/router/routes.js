// import Home from './routes/Home.svelte'
// import Author from './routes/Author.svelte'
// import Book from './routes/Book.svelte'
// import NotFound from './routes/NotFound.svelte'
// import SortableTable from './components/SortableTable.svelte';
import IndexPage from '../components/IndexPage.svelte';
import Clone from '../components/Clone.svelte';
import Commit from '../components/Commit.svelte';
// import DiffViewer from './components/DiffViewer.svelte';
// import MyCounter from './components/MyCounter.svelte';
// import DebugWindow from './components/DebugWindow.svelte';
// import About from './routes/About.svelte';

const routes = {
    // Exact path
    '/': Clone,
    "/commit" : Commit,
    "/clone": Clone,
    '/index': IndexPage,
    // '/counter': MyCounter
    // Using named parameters, with last being optional
    // '/author/:first/:last?': Author,

    // Wildcard parameter
    // '/book/*': Book,

    // Catch-all
    // This is optional, but if present it must be the last
    // '*': NotFound,
}

export default routes;