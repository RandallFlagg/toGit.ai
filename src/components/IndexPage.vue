<template>
  <div id="app">
    <div class="container">
      <section>
        <h2>Start a working area</h2>
        <ul>
          <li>
            <router-link to="/testpage">
              Test Page
            </router-link> - The Revision Graph Tool
          </li>
          <li>
            <router-link to="/test">
              test.html
            </router-link> - The Revision Graph Tool
          </li>
          <li>
            <router-link to="/revision-graph">
              Revision Graph
            </router-link> - The Revision Graph Tool
          </li>
        </ul>
      </section>
      <h1>Common Git Commands</h1>
      <section>
        <h2>Start a working area</h2>
        <ul>
          <li>
            <router-link to="/clone">
              clone
            </router-link> - Clone a repository into a new directory
          </li>
          <li>
            <router-link to="/init">
              init
            </router-link> - Create an empty Git repository or reinitialize an existing one
          </li>
        </ul>
      </section>
      <section>
        <h2>Work on the current change</h2>
        <ul>
          <li>
            <router-link to="/add">
              add
            </router-link> - Add file contents to the index
          </li>
          <li>
            <router-link to="/mv">
              mv
            </router-link> - Move or rename a file, a directory, or a symlink
          </li>
          <li>
            <router-link to="/restore">
              restore
            </router-link> - Restore working tree files
          </li>
          <li>
            <router-link to="/rm">
              rm
            </router-link> - Remove files from the working tree and from the index
          </li>
        </ul>
      </section>
      <section>
        <h2>Examine the history and state</h2>
        <ul>
          <li>
            <router-link to="/bisect">
              bisect
            </router-link> - Use binary search to find the commit that introduced a bug
          </li>
          <li>
            <router-link to="/diff">
              diff
            </router-link> - Show changes between commits, commit and working tree, etc
          </li>
          <li>
            <router-link to="/grep">
              grep
            </router-link> - Print lines matching a pattern
          </li>
          <li>
            <router-link to="/log">
              log
            </router-link> - Show commit logs
          </li>
          <li>
            <router-link to="/show">
              show
            </router-link> - Show various types of objects
          </li>
          <li>
            <router-link to="/status">
              status
            </router-link> - Show the working tree status
          </li>
        </ul>
      </section>
      <section>
        <h2>Grow, mark and tweak your common history</h2>
        <ul>
          <li>
            <router-link to="/branch">
              branch
            </router-link> - List, create, or delete branches
          </li>
          <li>
            <router-link to="/commit">
              commit
            </router-link> - Record changes to the repository
          </li>
          <li>
            <router-link to="/merge">
              merge
            </router-link> - Join two or more development histories together
          </li>
          <li>
            <router-link to="/rebase">
              rebase
            </router-link> - Reapply commits on top of another base tip
          </li>
          <li>
            <router-link to="/reset">
              reset
            </router-link> - Reset current HEAD to the specified state
          </li>
          <li>
            <router-link to="/switch">
              switch
            </router-link> - Switch branches
          </li>
          <li>
            <router-link to="/tag">
              tag
            </router-link> - Create, list, delete or verify a tag object signed with GPG
          </li>
        </ul>
      </section>
      <section>
        <h2>Collaborate</h2>
        <ul>
          <li>
            <router-link to="/fetch">
              fetch
            </router-link> - Download objects and refs from another repository
          </li>
          <li>
            <router-link to="/pull">
              pull
            </router-link> - Fetch from and integrate with another repository or a local branch
          </li>
          <li>
            <router-link to="/push">
              push
            </router-link> - Update remote refs along with associated objects
          </li>
        </ul>
      </section>
    </div>
  </div>
</template>

<script>
export default {
  name: 'IndexPage'
}
</script>

<style scoped>
.container {
  /* Add your CSS styles here */
}
</style>

<style scoped src="../assets/styles.css"></style>