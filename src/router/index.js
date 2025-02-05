import { createRouter, createWebHistory } from 'vue-router';
import IndexPage from '../components/IndexPage.vue';
import Clone from '../components/Clone.vue';
import TestPage from '../components/TestPage.vue';
// import Init from '../views/Init.vue';
// import Add from '../views/Add.vue';
// import Mv from '../views/Mv.vue';
// import Restore from '../views/Restore.vue';
// import Rm from '../views/Rm.vue';
// import Bisect from '../views/Bisect.vue';
// import Diff from '../views/Diff.vue';
// import Grep from '../views/Grep.vue';
// import Log from '../views/Log.vue';
// import Show from '../views/Show.vue';
// import Status from '../views/Status.vue';
// import Branch from '../views/Branch.vue';
import Commit from '../components/Commit.vue';
// import Merge from '../views/Merge.vue';
// import Rebase from '../views/Rebase.vue';
// import Reset from '../views/Reset.vue';
// import Switch from '../views/Switch.vue';
// import Tag from '../views/Tag.vue';
// import Fetch from '../views/Fetch.vue';
// import Pull from '../views/Pull.vue';
// import Push from '../views/Push.vue';
// import Clean from '../views/Clean.vue';

const routes = [
  {
    path: '/testpage',
    name: 'TestPage',
    component: TestPage
  },
  {
    path: '/test',
    name: 'tets',
    component: { template: '<div></div>', beforeRouteEnter(to, from, next) { window.location.href = 'test.html'; } }
  },
  {
    path: '/revision-graph',
    name: 'RevisionGraphHTML',
    component: { template: '<div></div>', beforeRouteEnter(to, from, next) { window.location.href = '/pages/graph.html'; } }
  },
  {
    path: '/',
    name: 'IndexPage',
    // component: IndexPage
    // component: Commit
    component: Clone
  },
  {
    path: '/clone',
    name: 'Clone',
    component: Clone
  },
  //   {
  //     path: '/init',
  //     name: 'Init',
  //     component: Init
  //   },
  {
    path: '/init',
    name: 'InitHTML',
    component: { template: '<div></div>', beforeRouteEnter(to, from, next) { window.location.href = '/pages/init.html'; } }
  },
  //   {
  //     path: '/add',
  //     name: 'Add',
  //     component: Add
  //   },
  //   {
  //     path: '/mv',
  //     name: 'Mv',
  //     component: Mv
  //   },
  //   {
  //     path: '/restore',
  //     name: 'Restore',
  //     component: Restore
  //   },
  //   {
  //     path: '/rm',
  //     name: 'Rm',
  //     component: Rm
  //   },
  //   {
  //     path: '/bisect',
  //     name: 'Bisect',
  //     component: Bisect
  //   },
  //   {
  //     path: '/diff',
  //     name: 'Diff',
  //     component: Diff
  //   },
  //   {
  //     path: '/grep',
  //     name: 'Grep',
  //     component: Grep
  //   },
  //   {
  //     path: '/log',
  //     name: 'Log',
  //     component: Log
  //   },
  //   {
  //     path: '/show',
  //     name: 'Show',
  //     component: Show
  //   },
  //   {
  //     path: '/status',
  //     name: 'Status',
  //     component: Status
  //   },
  //   {
  //     path: '/branch',
  //     name: 'Branch',
  //     component: Branch
  //   },
    {
      path: '/commit',
      name: 'Commit',
      component: Commit
    },
  // {
  //   path: '/commit',
  //   name: 'CommitHTML',
  //   component: { template: '<div></div>', beforeRouteEnter(to, from, next) { window.location.href = '/pages/commit.html'; } }
  // },
  //   {
  //     path: '/merge',
  //     name: 'Merge',
  //     component: Merge
  //   },
  //   {
  //     path: '/rebase',
  //     name: 'Rebase',
  //     component: Rebase
  //   },
  //   {
  //     path: '/reset',
  //     name: 'Reset',
  //     component: Reset
  //   },
  //   {
  //     path: '/switch',
  //     name: 'Switch',
  //     component: Switch
  //   },
  //   {
  //     path: '/tag',
  //     name: 'Tag',
  //     component: Tag
  //   },
  //   {
  //     path: '/fetch',
  //     name: 'Fetch',
  //     component: Fetch
  //   },
  //   {
  //     path: '/pull',
  //     name: 'Pull',
  //     component: Pull
  //   },
  //   {
  //     path: '/push',
  //     name: 'Push',
  //     component: Pushr
  //   }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
