<script setup lang="ts">
import {onMounted, ref} from "vue";
import axios, {type AxiosResponse} from "axios";

type Todo = {
    id: number,
    completed: boolean,
    text: number
}

const todos = ref<Todo[]>([]);

onMounted(() => {
    axios.get('/api/todos').then((res: AxiosResponse<Todo[]>) => {
        todos.value = res.data;
    });
});

const new_text = ref<string>('');

const submit = () => {
    axios.post('/api/todos', {text: new_text.value}).then((res: AxiosResponse<Todo>) => {
        todos.value = [res.data, ...todos.value];
        new_text.value = '';
    });
}
</script>

<template lang="pug">
.todoList
    form(@submit.prevent="submit")
        input(type="text" v-model="new_text")
        button(type="submit") add todo

ul
    li(v-for="todo in todos" v-text="todo.text")


</template>

<style scoped lang="less">
ul {
    list-style: none;
}

li {
    text-decoration: underline;
    border: 1px solid red;
    margin: 0 0 5px 0;

    &:last-child {
        margin-bottom: 0;
    }
}
</style>