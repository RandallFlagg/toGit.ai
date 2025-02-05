<script>
  import { link } from 'svelte-spa-router';
</script>

<div id="app">
  <div class="container">
    <section>
      <h2>Start a working area</h2>
      <ul>
        <li>
          <a href="/testpage" use:link>Test Page</a> - The Revision Graph Tool
        </li>
        <li>
          <a href="/test" use:link>test.html</a> - The Revision Graph Tool
        </li>
        <li>
          <a href="/revision-graph" use:link>Revision Graph</a> - The Revision Graph Tool
        </li>
      </ul>
    </section>
    <h1>Common Git Commands</h1>
    <section>
      <h2>Start a working area</h2>
      <ul>
        <li>
          <a href="/clone" use:link>clone</a> - Clone a repository into a new directory
        </li>
        <li>
          <a href="/init" use:link>init</a> - Create an empty Git repository or reinitialize an existing one
        </li>
      </ul>
    </section>
    <section>
      <h2>Work on the current change</h2>
      <ul>
        <li>
          <a href="/add" use:link>add</a> - Add file contents to the index
        </li>
        <li>
          <a href="/mv" use:link>mv</a> - Move or rename a file, a directory, or a symlink
        </li>
        <li>
          <a href="/restore" use:link>restore</a> - Restore working tree files
        </li>
        <li>
          <a href="/rm" use:link>rm</a> - Remove files from the working tree and from the index
        </li>
      </ul>
    </section>
    <section>
      <h2>Examine the history and state</h2>
      <ul>
        <li>
          <a href="/bisect" use:link>bisect</a> - Use binary search to find the commit that introduced a bug
        </li>
        <li>
          <a href="/diff" use:link>diff</a> - Show changes between commits, commit and working tree, etc
        </li>
        <li>
          <a href="/grep" use:link>grep</a> - Print lines matching a pattern
        </li>
        <li>
          <a href="/log" use:link>log</a> - Show commit logs
        </li>
        <li>
          <a href="/show" use:link>show</a> - Show various types of objects
        </li>
        <li>
          <a href="/status" use:link>status</a> - Show the working tree status
        </li>
        <li>
          <a href="/clean" use:link>clean</a> - Remove untracked files from the working tree
        </li>
      </ul>
    </section>
    <section>
      <h2>Grow, mark and tweak your common history</h2>
      <ul>
        <li>
          <a href="/branch" use:link>branch</a> - List, create, or delete branches
        </li>
        <li>
          <a href="/commit" use:link>commit</a> - Record changes to the repository
        </li>
        <li>
          <a href="/merge" use:link>merge</a> - Join two or more development histories together
        </li>
        <li>
          <a href="/rebase" use:link>rebase</a> - Reapply commits on top of another base tip
        </li>
        <li>
          <a href="/reset" use:link>reset</a> - Reset current HEAD to the specified state
        </li>
        <li>
          <a href="/switch" use:link>switch</a> - Switch branches
        </li>
        <li>
          <a href="/tag" use:link>tag</a> - Create, list, delete or verify a tag object signed with GPG
        </li>
      </ul>
    </section>
    <section>
      <h2>Collaborate</h2>
      <ul>
        <li>
          <a href="/fetch" use:link>fetch</a> - Download objects and refs from another repository
        </li>
        <li>
          <a href="/pull" use:link>pull</a> - Fetch from and integrate with another repository or a local branch
        </li>
        <li>
          <a href="/push" use:link>push</a> - Update remote refs along with associated objects
        </li>
      </ul>
    </section>
  </div>
</div>

<style>
  .container {
    /* Add your CSS styles here */
  }
</style>