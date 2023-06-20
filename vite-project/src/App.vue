<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import UserList from "./components/UserList.vue";
import TodoList from "./components/TodoList.vue";
</script>

<template lang="pug">
TodoList
UserList
HelloWorld(msg="Vite + Vue")
</template>

<style scoped lang="less">
.logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;

    &:hover {
        filter: drop-shadow(0 0 2em #646cffaa);
    }

    &.vue:hover {
        filter: drop-shadow(0 0 2em #42b883aa);
    }
}
</style>
