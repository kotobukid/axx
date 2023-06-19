<script setup lang="ts">
import {ref} from 'vue'

defineProps<{ msg: string }>()

const count = ref<number>(0)

const increment = (delta: number) => {
    count.value = count.value + delta;
}
const reset = () => {
    if (count.value > 0) {
        let next: number = count.value;
        let delta: number = 1;
        if (next > 60) {
            delta = 60;
        }

        const interval = 380 / delta;

        const timer = setInterval(() => {
            next = next - delta;
            if (next < 1) {
                count.value = 0;
                clearInterval(timer);
            } else {
                count.value = count.value - delta;
            }
        }, interval);
    }
}
</script>

<template lang="pug">
h1 {{ msg }}

.card
    button(type="button" @click="increment(1)") count is {{ count }}
    button(type="button" @click="increment(100)") count is {{ count }}
    a(href="#" @click.prevent="reset") RESET
    p
        span Edit
        code components/HelloWorld.vue
            span to test HMR
p
    span Check out
    a(href="https://vuejs.org/guide/quick-start.html#local" target="_blank") create-vue
    span , the official Vue + Vite starter
p
    span Install
    a(href="https://github.com/vuejs/language-tools" target="_blank") Volar
    span in your IDE for a better DX
p.read-the-docs Click on the Vite and Vue logos to learn more
</template>

<style scoped lang="less">
.read-the-docs {
    color: #888;
}

button {
    color: red;
    border: 1px solid red;
    background-color: pink;
}
</style>
