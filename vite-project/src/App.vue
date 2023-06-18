<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
</script>

<template lang="pug">
div
    a(href="https://vitejs.dev" target="_blank")
        img.logo(src="/vite.svg" alt="Vite logo")
    a(href="https://vuejs.org/" target="_blank")
        img.logo.vue(src="./assets/vue.svg" alt="Vue logo")

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
