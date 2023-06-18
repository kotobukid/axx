<script setup lang="ts">
import axios, {type AxiosResponse} from "axios";
import {onMounted, ref} from "vue";

type User = {
    id: number,
    username: string
};

const users = ref<User[]>([]);

onMounted(() => {
    axios.get('/api/users').then((res: AxiosResponse<{ users: User[] }>): void => {
        users.value = res.data.users;
    });
});

</script>

<template lang="pug">
ul
    li(v-for="u in users" v-text="u.username")
</template>

<style scoped lang="less">
ul {
    list-style: none;
}

li {
    border: 1px solid grey;
    margin-bottom: 5px;

    &:last-child {
        margin-bottom: 0;
    }
}
</style>